use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use crate::pybridge::classes::{Raster, Layer};
use crate::pybridge::extract::{extract_raster_input, extract_layer_input};
use crate::spatial;

#[pyfunction(signature = (layer, distance))]
pub fn buffer_py(layer: Bound<'_, PyAny>, distance: f64) -> PyResult<Layer> {
    let layer_input = extract_layer_input(layer)?;
    let layer_data = layer_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::buffer_layer(layer_data, distance)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer_a, layer_b))]
pub fn intersect_py(layer_a: Bound<'_, PyAny>, layer_b: Bound<'_, PyAny>) -> PyResult<Layer> {
    let layer_a_input = extract_layer_input(layer_a)?;
    let layer_b_input = extract_layer_input(layer_b)?;
    let layer_a_data = layer_a_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let layer_b_data = layer_b_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::intersect_layers(layer_a_data, layer_b_data)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer_a, layer_b))]
pub fn union_py(layer_a: Bound<'_, PyAny>, layer_b: Bound<'_, PyAny>) -> PyResult<Layer> {
    let layer_a_input = extract_layer_input(layer_a)?;
    let layer_b_input = extract_layer_input(layer_b)?;
    let layer_a_data = layer_a_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let layer_b_data = layer_b_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::union_layers(layer_a_data, layer_b_data)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer_a, layer_b))]
pub fn erase_py(layer_a: Bound<'_, PyAny>, layer_b: Bound<'_, PyAny>) -> PyResult<Layer> {
    let layer_a_input = extract_layer_input(layer_a)?;
    let layer_b_input = extract_layer_input(layer_b)?;
    let layer_a_data = layer_a_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let layer_b_data = layer_b_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::erase_layer(layer_a_data, layer_b_data)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer_a, layer_b))]
pub fn sym_diff_py(layer_a: Bound<'_, PyAny>, layer_b: Bound<'_, PyAny>) -> PyResult<Layer> {
    let layer_a_input = extract_layer_input(layer_a)?;
    let layer_b_input = extract_layer_input(layer_b)?;
    let layer_a_data = layer_a_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let layer_b_data = layer_b_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::sym_diff_layers(layer_a_data, layer_b_data)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer, tolerance))]
pub fn simplify_py(layer: Bound<'_, PyAny>, tolerance: f64) -> PyResult<Layer> {
    let layer_input = extract_layer_input(layer)?;
    let layer_data = layer_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::simplify_layer(layer_data, tolerance)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer))]
pub fn convex_hull_py(layer: Bound<'_, PyAny>) -> PyResult<Layer> {
    let layer_input = extract_layer_input(layer)?;
    let layer_data = layer_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::convex_hull_layer(layer_data)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer, max_edge_length))]
pub fn concave_hull_py(layer: Bound<'_, PyAny>, max_edge_length: f64) -> PyResult<Layer> {
    let layer_input = extract_layer_input(layer)?;
    let layer_data = layer_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::concave_hull_impl(layer_data, max_edge_length)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer))]
pub fn dissolve_py(layer: Bound<'_, PyAny>) -> PyResult<Layer> {
    let layer_input = extract_layer_input(layer)?;
    let layer_data = layer_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::dissolve_impl(layer_data)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer))]
pub fn make_valid_py(layer: Bound<'_, PyAny>) -> PyResult<Layer> {
    let layer_input = extract_layer_input(layer)?;
    let layer_data = layer_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = spatial::make_valid_impl(layer_data)
        .map_err(|e| PyRuntimeError::new_err(e))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (layer, target_epsg))]
pub fn reproject_vector_py(layer: Bound<'_, PyAny>, target_epsg: u32) -> PyResult<Layer> {
    let layer_input = extract_layer_input(layer)?;
    let layer_data = layer_input.get_layer().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_layer = layer_data.reproject_to_epsg(target_epsg)
        .map_err(|e| PyRuntimeError::new_err(format!("Vector reprojection failed: {}", e)))?;
    Ok(Layer { layer: result_layer })
}

#[pyfunction(signature = (raster, target_epsg))]
pub fn reproject_raster_py(raster: Bound<'_, PyAny>, target_epsg: u32) -> PyResult<Raster> {
    let raster_input = extract_raster_input(raster)?;
    let raster_data = raster_input.get_raster().map_err(|e| PyRuntimeError::new_err(e))?;
    let result_raster = raster_data.reproject_to_epsg(target_epsg, wbraster::ResampleMethod::Bilinear)
        .map_err(|e| PyRuntimeError::new_err(format!("Raster reprojection failed: {}", e)))?;
    Ok(Raster { raster: result_raster })
}
