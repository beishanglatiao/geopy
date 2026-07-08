use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use crate::pybridge::classes::{Raster, Layer};
use crate::types::{RasterInput, LayerInput};

pub fn extract_raster_input(raster: Bound<'_, PyAny>) -> PyResult<RasterInput> {
    if let Ok(path) = raster.extract::<&str>() {
        Ok(RasterInput::Path(path.to_string()))
    } else if let Ok(raster_obj) = raster.cast::<Raster>() {
        let raster_ref = raster_obj.borrow();
        Ok(RasterInput::Data(raster_ref.raster.clone()))
    } else {
        Err(PyRuntimeError::new_err(
            "raster must be a string path or Raster object",
        ))
    }
}

pub fn extract_layer_input(layer: Bound<'_, PyAny>) -> PyResult<LayerInput> {
    if let Ok(path) = layer.extract::<&str>() {
        Ok(LayerInput::Path(path.to_string()))
    } else if let Ok(layer_obj) = layer.cast::<Layer>() {
        let layer_ref = layer_obj.borrow();
        Ok(LayerInput::Data(layer_ref.layer.clone()))
    } else {
        Err(PyRuntimeError::new_err(
            "layer must be a string path or Layer object",
        ))
    }
}
