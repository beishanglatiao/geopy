use std::collections::{HashMap, BTreeMap};
use euclid::Transform2D;
use geo_types::{Geometry, MultiPolygon};
use geo_rasterize::LabelBuilder;
use rayon::{iter::IntoParallelIterator, iter::ParallelIterator, ThreadPoolBuilder};

use crate::types::{RasterInput, LayerInput};
use crate::conv::wbgeom_to_geotypes;
use crate::types::UnionFind;

/// Compute zonal histogram for each feature.
///
/// For each feature (identified by its 0-based index), counts how many pixels
/// of each integer value fall within the feature boundary.
///
/// Args:
///     raster_input: Input raster source
///     layer_input: Input vector layer source
///     max_value_limit: Optional upper limit for pixel values to count
///
/// Returns:
///     Vec of HashMap, each containing {"ID": i, "VALUE_0": n, "VALUE_1": n, ...}
pub fn zonal_histogram(
    raster_input: RasterInput,
    layer_input: LayerInput,
    max_value_limit: Option<usize>,
) -> Result<Vec<BTreeMap<String, i64>>, String> {
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .map_err(|e| format!("Failed to create thread pool: {}", e))?;

    pool.install(|| {
        let (raster, layer) = rayon::join(
            || raster_input.get_raster(),
            || layer_input.get_layer(),
        );

        let raster = raster.map_err(|e| format!("Failed to read raster: {}", e))?;
        let layer = layer.map_err(|e| format!("Failed to read layer: {}", e))?;

        let data = raster.band_to_vec_f64(0);
        let rows = raster.rows;
        let cols = raster.cols;
        let nodata = raster.nodata;

        let raster_epsg = raster.crs.epsg;
        let layer_epsg = layer.crs_epsg();

        let working_layer = match (layer_epsg, raster_epsg) {
            (Some(le), Some(re)) if le != re => layer
                .clone()
                .reproject_to_epsg(re)
                .map_err(|e| format!("Failed to reproject vector: {}", e))?,
            _ => layer.clone(),
        };

        let features: Vec<_> = working_layer.iter().collect();
        let num_features = features.len();

        let mut geometries: Vec<Geometry<f64>> = Vec::with_capacity(num_features);

        for feature in &features {
            if let Some(ref geom) = feature.geometry {
                if let Some(g) = wbgeom_to_geotypes(geom) {
                    geometries.push(g);
                } else {
                    geometries.push(Geometry::MultiPolygon(MultiPolygon::new(vec![])));
                }
            } else {
                geometries.push(Geometry::MultiPolygon(MultiPolygon::new(vec![])));
            }
        }

        let geo_to_pix = Transform2D::new(
            1.0 / raster.cell_size_x,
            0.0,
            0.0,
            -1.0 / raster.cell_size_y,
            -raster.x_min / raster.cell_size_x,
            raster.y_max() / raster.cell_size_y,
        );

        let mut rasterizer = LabelBuilder::background(0usize)
            .width(cols)
            .height(rows)
            .geo_to_pix(geo_to_pix)
            .build()
            .map_err(|e| format!("Failed to create rasterizer: {}", e))?;

        for (idx, geom) in geometries.iter().enumerate() {
            let label = idx + 1;
            rasterizer
                .rasterize(geom, label)
                .map_err(|e| format!("Failed to rasterize feature {}: {}", idx, e))?;
        }

        let zone_ids = rasterizer.finish();

        let row_results: Vec<_> = (0..rows)
            .into_par_iter()
            .map(|row| {
                let mut row_max = 0u32;
                let mut row_counts: Vec<HashMap<usize, u64>> = vec![HashMap::new(); num_features];
                
                for col in 0..cols {
                    let zone_idx = zone_ids[(row, col)];
                    if zone_idx == 0 {
                        continue;
                    }
                    let data_idx = row * cols + col;
                    let val = data[data_idx];
                    if val != nodata && !val.is_nan() {
                        let val_u32 = val as u32;
                        if val_u32 > row_max {
                            row_max = val_u32;
                        }
                        
                        let feature_idx = zone_idx - 1;
                        let entry = row_counts[feature_idx].entry(val_u32 as usize).or_insert(0);
                        *entry += 1;
                    }
                }
                (row_max, row_counts)
            })
            .collect();

        let mut global_max = 0u32;
        for (rm, _) in &row_results {
            if *rm > global_max {
                global_max = *rm;
            }
        }

        let max_value = match max_value_limit {
            Some(limit) => std::cmp::min(global_max, limit as u32),
            None => global_max,
        };

        let mut histograms: Vec<Vec<u64>> = vec![vec![0u64; max_value as usize + 1]; num_features];

        for (_, row_counts) in row_results {
            for (feature_idx, counts) in row_counts.into_iter().enumerate() {
                for (&val_idx, &count) in counts.iter() {
                    if val_idx <= max_value as usize {
                        histograms[feature_idx][val_idx] += count;
                    }
                }
            }
        }

        let mut result: Vec<BTreeMap<String, i64>> = Vec::with_capacity(num_features);
        let pad = if max_value > 0 { (max_value as f64).log10().floor() as usize + 1 } else { 1 };
        for (i, counts) in histograms.into_iter().enumerate() {
            let mut row = BTreeMap::new();
            row.insert("ID".to_string(), i as i64);
            for (val_idx, count) in counts.into_iter().enumerate() {
                row.insert(format!("VALUE_{:0pad$}", val_idx, pad = pad), count as i64);
            }
            result.push(row);
        }

        Ok(result)
    })
}

/// Count connected components (patches) for each pixel value in a single-band
/// integer raster using Two-Pass Connected Component Labeling (4-connectivity).
///
/// Returns a dictionary mapping each pixel value to its number of patches.
pub fn count_patches(raster_input: RasterInput) -> Result<BTreeMap<i64, usize>, String> {
    let raster = raster_input.get_raster()?;

    if raster.bands != 1 {
        return Err(format!(
            "Input raster must be single-band, but has {} bands",
            raster.bands
        ));
    }

    let rows = raster.rows as isize;
    let cols = raster.cols as isize;
    let nodata = raster.nodata;
    let data = raster.band_to_vec_f64(0);
    let total_cells = (rows * cols) as usize;

    let mut labels: Vec<usize> = vec![0; total_cells];
    let mut uf = UnionFind::new(total_cells + 1);

    let mut next_label: usize = 1;

    // First pass: assign provisional labels, record equivalences
    for row in 0..rows {
        for col in 0..cols {
            let idx = (row * cols + col) as usize;
            let val_f64 = data[idx];
            if val_f64 == nodata || val_f64.is_nan() {
                continue;
            }
            let val = val_f64 as i64;

            let left_label = if col > 0 {
                let lidx = (row * cols + (col - 1)) as usize;
                if data[lidx] != nodata && (data[lidx] as i64) == val {
                    labels[lidx]
                } else {
                    0
                }
            } else {
                0
            };

            let above_label = if row > 0 {
                let aidx = ((row - 1) * cols + col) as usize;
                if data[aidx] != nodata && (data[aidx] as i64) == val {
                    labels[aidx]
                } else {
                    0
                }
            } else {
                0
            };

            if left_label == 0 && above_label == 0 {
                labels[idx] = next_label;
                next_label += 1;
            } else if left_label != 0 && above_label == 0 {
                labels[idx] = left_label;
            } else if left_label == 0 && above_label != 0 {
                labels[idx] = above_label;
            } else {
                let min_label = left_label.min(above_label);
                labels[idx] = min_label;
                if left_label != above_label {
                    uf.union(left_label, above_label);
                }
            }
        }
    }

    if next_label == 1 {
        return Ok(BTreeMap::new());
    }

    for i in 1..next_label {
        uf.find(i);
    }

    let mut component_id: Vec<usize> = labels.iter().map(|&l| if l == 0 { 0 } else { uf.find(l) }).collect();

    let mut root_to_compact: HashMap<usize, usize> = HashMap::new();
    let mut compact_counter: usize = 1;
    for root in component_id.iter_mut() {
        if *root == 0 {
            continue;
        }
        let compact = root_to_compact.entry(*root).or_insert_with(|| {
            let id = compact_counter;
            compact_counter += 1;
            id
        });
        *root = *compact;
    }

    let mut val_patch: HashMap<(i64, usize), ()> = HashMap::new();
    for row in 0..rows {
        for col in 0..cols {
            let idx = (row * cols + col) as usize;
            let val_f64 = data[idx];
            if val_f64 == nodata || val_f64.is_nan() {
                continue;
            }
            let val = val_f64 as i64;
            let comp = component_id[idx];
            if comp != 0 {
                val_patch.entry((val, comp)).or_insert(());
            }
        }
    }

    let mut result: BTreeMap<i64, usize> = BTreeMap::new();
    for ((val, _), _) in val_patch {
        *result.entry(val).or_insert(0) += 1;
    }

    Ok(result)
}
