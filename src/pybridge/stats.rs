use std::collections::BTreeMap;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use crate::stats::{zonal_histogram, count_patches};
use crate::pybridge::extract::{extract_raster_input, extract_layer_input};

#[pyfunction(signature = (raster))]
pub fn count_patches_py(raster: Bound<'_, PyAny>) -> PyResult<BTreeMap<i64, usize>> {
    let raster_input = extract_raster_input(raster)?;
    let result = count_patches(raster_input)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(result)
}

#[pyfunction(signature = (raster, layer, max_value_limit=None))]
pub fn zonal_histogram_py(
    raster: Bound<'_, PyAny>,
    layer: Bound<'_, PyAny>,
    max_value_limit: Option<usize>,
) -> PyResult<Vec<BTreeMap<String, i64>>> {
    let raster_input = extract_raster_input(raster)?;
    let layer_input = extract_layer_input(layer)?;

    let result = zonal_histogram(raster_input, layer_input, max_value_limit)
        .map_err(|e| PyRuntimeError::new_err(e))?;

    Ok(result)
}
