mod pybridge;
mod stats;
mod spatial;
mod types;
mod conv;

use pyo3::prelude::*;
use pybridge::stats::{zonal_histogram_py, count_patches_py};
use pybridge::spatial::{
    buffer_py,
    intersect_py,
    union_py,
    erase_py,
    sym_diff_py,
    simplify_py,
    convex_hull_py,
    concave_hull_py,
    dissolve_py,
    make_valid_py,
    reproject_vector_py,
    reproject_raster_py,
};

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(zonal_histogram_py, m)?)?;
    m.add_function(wrap_pyfunction!(count_patches_py, m)?)?;
    m.add_function(wrap_pyfunction!(buffer_py, m)?)?;
    m.add_function(wrap_pyfunction!(intersect_py, m)?)?;
    m.add_function(wrap_pyfunction!(union_py, m)?)?;
    m.add_function(wrap_pyfunction!(erase_py, m)?)?;
    m.add_function(wrap_pyfunction!(sym_diff_py, m)?)?;
    m.add_function(wrap_pyfunction!(simplify_py, m)?)?;
    m.add_function(wrap_pyfunction!(convex_hull_py, m)?)?;
    m.add_function(wrap_pyfunction!(concave_hull_py, m)?)?;
    m.add_function(wrap_pyfunction!(dissolve_py, m)?)?;
    m.add_function(wrap_pyfunction!(make_valid_py, m)?)?;
    m.add_function(wrap_pyfunction!(reproject_vector_py, m)?)?;
    m.add_function(wrap_pyfunction!(reproject_raster_py, m)?)?;
    m.add_class::<pybridge::classes::Raster>()?;
    m.add_class::<pybridge::classes::Layer>()?;
    m.add_class::<pybridge::classes::Coord>()?;
    m.add_class::<pybridge::classes::BBox>()?;
    m.add_class::<pybridge::classes::FieldType>()?;
    m.add_class::<pybridge::classes::FieldDef>()?;
    m.add_class::<pybridge::classes::Geometry>()?;
    Ok(())
}
