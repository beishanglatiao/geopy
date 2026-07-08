use wbtopology::{
    buffer_linestring,
    buffer_polygon,
    buffer_point,
    BufferOptions,
    convex_hull_geometry,
    concave_hull_geometry,
    simplify_geometry,
    make_valid_geometry,
    polygon_intersection,
    polygon_union,
    polygon_difference,
    polygon_sym_diff,
    polygon_unary_union,
    geom::Geometry as TopoGeometry,
    geom::Polygon as TopoPolygon,
    vector_io::{geometries_from_layer, layer_from_geometries},
};

const DEFAULT_EPSILON: f64 = 1.0e-9;

/// Buffer all geometries in a layer by a given distance.
///
/// Supports Point, LineString, Polygon, and their Multi-variants.
/// Returns a new layer containing buffered polygons.
pub fn buffer_layer(layer: wbvector::Layer, distance: f64) -> Result<wbvector::Layer, String> {
    let geoms = geometries_from_layer(&layer)
        .map_err(|e| format!("Failed to read geometries: {}", e))?;
    
    let options = BufferOptions::default();
    let buffered_geoms: Vec<TopoGeometry> = geoms
        .into_iter()
        .map(|g| match g {
            TopoGeometry::Point(c) => TopoGeometry::Polygon(buffer_point(c, distance, options)),
            TopoGeometry::LineString(ls) => TopoGeometry::Polygon(buffer_linestring(&ls, distance, options)),
            TopoGeometry::Polygon(p) => TopoGeometry::Polygon(buffer_polygon(&p, distance, options)),
            TopoGeometry::MultiPoint(pts) => {
                let mut result = Vec::new();
                for pt in pts { result.push(buffer_point(pt, distance, options)); }
                TopoGeometry::MultiPolygon(result)
            }
            TopoGeometry::MultiLineString(lines) => {
                let mut result = Vec::new();
                for ls in lines { result.push(buffer_linestring(&ls, distance, options)); }
                TopoGeometry::MultiPolygon(result)
            }
            TopoGeometry::MultiPolygon(polys) => {
                let mut result = Vec::new();
                for p in polys { result.push(buffer_polygon(&p, distance, options)); }
                TopoGeometry::MultiPolygon(result)
            }
            TopoGeometry::GeometryCollection(parts) => {
                let mut result = Vec::new();
                for part in parts {
                    match part {
                        TopoGeometry::Point(c) => result.push(buffer_point(c, distance, options)),
                        TopoGeometry::LineString(ls) => result.push(buffer_linestring(&ls, distance, options)),
                        TopoGeometry::Polygon(p) => result.push(buffer_polygon(&p, distance, options)),
                        _ => {}
                    }
                }
                TopoGeometry::MultiPolygon(result)
            }
        })
        .collect();
    
    let epsg = layer.crs_epsg();
    layer_from_geometries(&layer.name, &buffered_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Extract all Polygon geometries from a slice of geometries.
///
/// Recursively unwraps MultiPolygon and GeometryCollection to collect individual polygons.
fn extract_polygons(geoms: &[TopoGeometry]) -> Vec<TopoPolygon> {
    let mut result = Vec::new();
    for g in geoms {
        match g {
            TopoGeometry::Polygon(p) => result.push(p.clone()),
            TopoGeometry::MultiPolygon(polys) => result.extend(polys.iter().cloned()),
            TopoGeometry::GeometryCollection(parts) => {
                for part in parts {
                    match part {
                        TopoGeometry::Polygon(p) => result.push(p.clone()),
                        TopoGeometry::MultiPolygon(polys) => result.extend(polys.iter().cloned()),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    result
}

/// Compute intersection of two polygon layers.
///
/// For each polygon pair (A×B), returns the overlapping region.
/// Result layer preserves CRS from layer_a.
pub fn intersect_layers(layer_a: wbvector::Layer, layer_b: wbvector::Layer) -> Result<wbvector::Layer, String> {
    let geoms_a = geometries_from_layer(&layer_a)
        .map_err(|e| format!("Failed to read geometries from layer A: {}", e))?;
    let geoms_b = geometries_from_layer(&layer_b)
        .map_err(|e| format!("Failed to read geometries from layer B: {}", e))?;
    
    let polys_a = extract_polygons(&geoms_a);
    let polys_b = extract_polygons(&geoms_b);
    
    let mut result_polys = Vec::new();
    for poly_a in &polys_a {
        for poly_b in &polys_b {
            result_polys.extend(polygon_intersection(poly_a, poly_b, DEFAULT_EPSILON));
        }
    }
    
    let result_geoms: Vec<TopoGeometry> = result_polys.into_iter().map(TopoGeometry::Polygon).collect();
    let epsg = layer_a.crs_epsg();
    layer_from_geometries(&layer_a.name, &result_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Compute union of two polygon layers.
///
/// For each polygon pair (A×B), merges overlapping regions into a single polygon.
/// Result layer preserves CRS from layer_a.
pub fn union_layers(layer_a: wbvector::Layer, layer_b: wbvector::Layer) -> Result<wbvector::Layer, String> {
    let geoms_a = geometries_from_layer(&layer_a)
        .map_err(|e| format!("Failed to read geometries from layer A: {}", e))?;
    let geoms_b = geometries_from_layer(&layer_b)
        .map_err(|e| format!("Failed to read geometries from layer B: {}", e))?;
    
    let polys_a = extract_polygons(&geoms_a);
    let polys_b = extract_polygons(&geoms_b);
    
    let mut result_polys = Vec::new();
    for poly_a in &polys_a {
        for poly_b in &polys_b {
            result_polys.extend(polygon_union(poly_a, poly_b, DEFAULT_EPSILON));
        }
    }
    
    let result_geoms: Vec<TopoGeometry> = result_polys.into_iter().map(TopoGeometry::Polygon).collect();
    let epsg = layer_a.crs_epsg();
    layer_from_geometries(&layer_a.name, &result_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Erase (clip out) parts of layer_a covered by layer_b.
///
/// For each polygon A, subtracts overlapping portions of all polygons B.
/// Result layer preserves CRS from layer_a.
pub fn erase_layer(layer_a: wbvector::Layer, layer_b: wbvector::Layer) -> Result<wbvector::Layer, String> {
    let geoms_a = geometries_from_layer(&layer_a)
        .map_err(|e| format!("Failed to read geometries from layer A: {}", e))?;
    let geoms_b = geometries_from_layer(&layer_b)
        .map_err(|e| format!("Failed to read geometries from layer B: {}", e))?;
    
    let polys_a = extract_polygons(&geoms_a);
    let polys_b = extract_polygons(&geoms_b);
    
    let mut result_polys = Vec::new();
    for poly_a in &polys_a {
        let mut remaining = vec![poly_a.clone()];
        for poly_b in &polys_b {
            let mut new_remaining = Vec::new();
            for p in &remaining {
                new_remaining.extend(polygon_difference(p, poly_b, DEFAULT_EPSILON));
            }
            remaining = new_remaining;
            if remaining.is_empty() { break; }
        }
        result_polys.extend(remaining);
    }
    
    let result_geoms: Vec<TopoGeometry> = result_polys.into_iter().map(TopoGeometry::Polygon).collect();
    let epsg = layer_a.crs_epsg();
    layer_from_geometries(&layer_a.name, &result_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Compute symmetric difference of two polygon layers.
///
/// Returns areas covered by exactly one of the two polygons (A xor B).
/// Result layer preserves CRS from layer_a.
pub fn sym_diff_layers(layer_a: wbvector::Layer, layer_b: wbvector::Layer) -> Result<wbvector::Layer, String> {
    let geoms_a = geometries_from_layer(&layer_a)
        .map_err(|e| format!("Failed to read geometries from layer A: {}", e))?;
    let geoms_b = geometries_from_layer(&layer_b)
        .map_err(|e| format!("Failed to read geometries from layer B: {}", e))?;
    
    let polys_a = extract_polygons(&geoms_a);
    let polys_b = extract_polygons(&geoms_b);
    
    let mut result_polys = Vec::new();
    for poly_a in &polys_a {
        for poly_b in &polys_b {
            result_polys.extend(polygon_sym_diff(poly_a, poly_b, DEFAULT_EPSILON));
        }
    }
    
    let result_geoms: Vec<TopoGeometry> = result_polys.into_iter().map(TopoGeometry::Polygon).collect();
    let epsg = layer_a.crs_epsg();
    layer_from_geometries(&layer_a.name, &result_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Simplify geometry in a layer using Ramer-Douglas-Peucker algorithm.
///
/// `tolerance` controls how much detail is removed (larger = more simplification).
pub fn simplify_layer(layer: wbvector::Layer, tolerance: f64) -> Result<wbvector::Layer, String> {
    let geoms = geometries_from_layer(&layer)
        .map_err(|e| format!("Failed to read geometries: {}", e))?;
    
    let simplified_geoms: Vec<TopoGeometry> = geoms
        .into_iter()
        .map(|g| simplify_geometry(&g, tolerance))
        .collect();
    
    let epsg = layer.crs_epsg();
    layer_from_geometries(&layer.name, &simplified_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Compute convex hull for each geometry in a layer.
pub fn convex_hull_layer(layer: wbvector::Layer) -> Result<wbvector::Layer, String> {
    let geoms = geometries_from_layer(&layer)
        .map_err(|e| format!("Failed to read geometries: {}", e))?;
    
    let hull_geoms: Vec<TopoGeometry> = geoms
        .into_iter()
        .map(|g| convex_hull_geometry(&g, DEFAULT_EPSILON))
        .collect();
    
    let epsg = layer.crs_epsg();
    layer_from_geometries(&layer.name, &hull_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Compute concave hull for each geometry in a layer.
///
/// `max_edge_length` limits edge length in the resulting hull.
pub fn concave_hull_impl(layer: wbvector::Layer, max_edge_length: f64) -> Result<wbvector::Layer, String> {
    let geoms = geometries_from_layer(&layer)
        .map_err(|e| format!("Failed to read geometries: {}", e))?;
    
    let hull_geoms: Vec<TopoGeometry> = geoms
        .into_iter()
        .map(|g| concave_hull_geometry(&g, max_edge_length, DEFAULT_EPSILON))
        .collect();
    
    let epsg = layer.crs_epsg();
    layer_from_geometries(&layer.name, &hull_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Dissolve (unary union) all polygons in a layer into a single polygon.
pub fn dissolve_impl(layer: wbvector::Layer) -> Result<wbvector::Layer, String> {
    let geoms = geometries_from_layer(&layer)
        .map_err(|e| format!("Failed to read geometries: {}", e))?;
    
    let polys = extract_polygons(&geoms);
    let dissolved = polygon_unary_union(&polys, DEFAULT_EPSILON);
    
    let result_geoms: Vec<TopoGeometry> = dissolved.into_iter().map(TopoGeometry::Polygon).collect();
    let epsg = layer.crs_epsg();
    layer_from_geometries(&layer.name, &result_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}

/// Repair invalid geometries in a layer.
///
/// Uses `GeometryFixOptions::default()` to fix self-intersections, ring orientation, etc.
pub fn make_valid_impl(layer: wbvector::Layer) -> Result<wbvector::Layer, String> {
    let geoms = geometries_from_layer(&layer)
        .map_err(|e| format!("Failed to read geometries: {}", e))?;
    
    let opts = wbtopology::GeometryFixOptions::default();
    let valid_geoms: Vec<TopoGeometry> = geoms
        .into_iter()
        .map(|g| make_valid_geometry(&g, opts.clone()))
        .collect();
    
    let epsg = layer.crs_epsg();
    layer_from_geometries(&layer.name, &valid_geoms, epsg)
        .map_err(|e| format!("Failed to create result layer: {}", e))
}
