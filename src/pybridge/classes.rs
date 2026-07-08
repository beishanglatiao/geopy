use pyo3::exceptions::PyRuntimeError;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyType};

#[pyclass]
pub struct Raster {
    pub raster: wbraster::Raster,
}

#[pymethods]
impl Raster {
    #[new]
    fn new() -> PyResult<Self> {
        Err(PyRuntimeError::new_err("Use Raster.read() to create"))
    }

    #[classmethod]
    fn read(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        let raster = wbraster::Raster::read(path)
            .map_err(|e| PyRuntimeError::new_err(format!("读取栅格失败: {}", e)))?;
        Ok(Raster { raster })
    }

    fn write(&mut self, path: &str) -> PyResult<()> {
        let format = wbraster::RasterFormat::for_output_path(path)
            .unwrap_or(wbraster::RasterFormat::GeoTiff);
        self.raster.write(path, format)
            .map_err(|e| PyRuntimeError::new_err(format!("写入栅格失败: {}", e)))?;
        Ok(())
    }

    fn get(&self, band: isize, row: isize, col: isize) -> PyResult<f64> {
        Ok(self.raster.get(band, row, col))
    }

    fn set(&mut self, band: isize, row: isize, col: isize, value: f64) -> PyResult<()> {
        self.raster.set(band, row, col, value)
            .map_err(|e| PyRuntimeError::new_err(format!("设置像素值失败: {}", e)))?;
        Ok(())
    }

    #[getter]
    fn rows(&self) -> usize {
        self.raster.rows
    }

    #[getter]
    fn cols(&self) -> usize {
        self.raster.cols
    }

    #[getter]
    fn bands(&self) -> usize {
        self.raster.bands
    }

    #[getter]
    fn nodata(&self) -> f64 {
        self.raster.nodata
    }

    #[getter]
    fn x_min(&self) -> f64 {
        self.raster.x_min
    }

    #[getter]
    fn y_min(&self) -> f64 {
        self.raster.y_min
    }

    #[getter]
    fn x_max(&self) -> f64 {
        self.raster.x_min + (self.raster.cols as f64) * self.raster.cell_size_x
    }

    #[getter]
    fn y_max(&self) -> f64 {
        self.raster.y_min + (self.raster.rows as f64) * self.raster.cell_size_y
    }

    #[getter]
    fn cell_size_x(&self) -> f64 {
        self.raster.cell_size_x
    }

    #[getter]
    fn cell_size_y(&self) -> f64 {
        self.raster.cell_size_y
    }

    #[getter]
    fn data_type(&self) -> String {
        format!("{:?}", self.raster.data_type)
    }

    #[getter]
    fn crs(&self) -> Option<String> {
        self.raster.crs.wkt.clone()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Raster(rows={}, cols={}, bands={})",
            self.raster.rows, self.raster.cols, self.raster.bands
        )
    }
}

#[pyclass]
pub struct Coord {
    pub coord: wbvector::Coord,
}

#[pymethods]
impl Coord {
    #[new]
    #[pyo3(signature = (x, y, z=None, m=None))]
    fn new(x: f64, y: f64, z: Option<f64>, m: Option<f64>) -> Self {
        Coord {
            coord: wbvector::Coord { x, y, z, m },
        }
    }

    #[getter]
    fn x(&self) -> f64 {
        self.coord.x
    }

    #[getter]
    fn y(&self) -> f64 {
        self.coord.y
    }

    #[getter]
    fn z(&self) -> Option<f64> {
        self.coord.z
    }

    #[getter]
    fn m(&self) -> Option<f64> {
        self.coord.m
    }

    fn has_z(&self) -> bool {
        self.coord.has_z()
    }

    fn has_m(&self) -> bool {
        self.coord.has_m()
    }

    pub fn __repr__(&self) -> String {
        match (self.coord.z, self.coord.m) {
            (None, None) => format!("Coord({}, {})", self.coord.x, self.coord.y),
            (Some(z), None) => format!("Coord({}, {}, {})", self.coord.x, self.coord.y, z),
            (None, Some(m)) => format!("Coord({}, {}, m={})", self.coord.x, self.coord.y, m),
            (Some(z), Some(m)) => format!("Coord({}, {}, {}, {})", self.coord.x, self.coord.y, z, m),
        }
    }
}

#[pyclass]
pub struct BBox {
    pub bbox: wbvector::BBox,
}

#[pymethods]
impl BBox {
    #[new]
    fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        BBox {
            bbox: wbvector::BBox::new(min_x, min_y, max_x, max_y),
        }
    }

    #[getter]
    fn min_x(&self) -> f64 {
        self.bbox.min_x
    }

    #[getter]
    fn min_y(&self) -> f64 {
        self.bbox.min_y
    }

    #[getter]
    fn max_x(&self) -> f64 {
        self.bbox.max_x
    }

    #[getter]
    fn max_y(&self) -> f64 {
        self.bbox.max_y
    }

    fn width(&self) -> f64 {
        self.bbox.width()
    }

    fn height(&self) -> f64 {
        self.bbox.height()
    }

    fn center(&self) -> (f64, f64) {
        self.bbox.center()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        self.bbox.contains(x, y)
    }

    fn intersects(&self, other: &BBox) -> bool {
        self.bbox.intersects(&other.bbox)
    }

    pub fn __repr__(&self) -> String {
        format!("BBox({}, {}, {}, {})", self.bbox.min_x, self.bbox.min_y, self.bbox.max_x, self.bbox.max_y)
    }
}

#[pyclass]
pub struct FieldType(pub wbvector::FieldType);

#[pymethods]
impl FieldType {
    #[classattr]
    const INTEGER: u8 = 0;
    #[classattr]
    const FLOAT: u8 = 1;
    #[classattr]
    const TEXT: u8 = 2;
    #[classattr]
    const BOOLEAN: u8 = 3;
    #[classattr]
    const BLOB: u8 = 4;
    #[classattr]
    const DATE: u8 = 5;
    #[classattr]
    const DATETIME: u8 = 6;
    #[classattr]
    const JSON: u8 = 7;

    #[staticmethod]
    fn integer() -> FieldType {
        FieldType(wbvector::FieldType::Integer)
    }

    #[staticmethod]
    fn float() -> FieldType {
        FieldType(wbvector::FieldType::Float)
    }

    #[staticmethod]
    fn text() -> FieldType {
        FieldType(wbvector::FieldType::Text)
    }

    #[staticmethod]
    fn boolean() -> FieldType {
        FieldType(wbvector::FieldType::Boolean)
    }

    #[staticmethod]
    fn blob() -> FieldType {
        FieldType(wbvector::FieldType::Blob)
    }

    #[staticmethod]
    fn date() -> FieldType {
        FieldType(wbvector::FieldType::Date)
    }

    #[staticmethod]
    fn datetime() -> FieldType {
        FieldType(wbvector::FieldType::DateTime)
    }

    #[staticmethod]
    fn json() -> FieldType {
        FieldType(wbvector::FieldType::Json)
    }

    fn __str__(&self) -> String {
        self.0.as_str().to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("FieldType({})", self.0.as_str())
    }
}

#[pyclass]
pub struct FieldDef {
    pub field_def: wbvector::FieldDef,
}

#[pymethods]
impl FieldDef {
    #[new]
    fn new(name: &str, field_type: &FieldType) -> Self {
        FieldDef {
            field_def: wbvector::FieldDef::new(name, field_type.0),
        }
    }

    #[getter]
    fn name(&self) -> &str {
        &self.field_def.name
    }

    #[getter]
    fn field_type(&self) -> String {
        self.field_def.field_type.as_str().to_string()
    }

    #[getter]
    fn nullable(&self) -> bool {
        self.field_def.nullable
    }

    #[getter]
    fn width(&self) -> usize {
        self.field_def.width
    }

    #[getter]
    fn precision(&self) -> usize {
        self.field_def.precision
    }

    fn not_null(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.field_def = slf.field_def.clone().not_null();
        slf
    }

    fn width_set(mut slf: PyRefMut<'_, Self>, w: usize) -> PyRefMut<'_, Self> {
        slf.field_def = slf.field_def.clone().width(w);
        slf
    }

    fn precision_set(mut slf: PyRefMut<'_, Self>, p: usize) -> PyRefMut<'_, Self> {
        slf.field_def = slf.field_def.clone().precision(p);
        slf
    }

    pub fn __repr__(&self) -> String {
        format!("FieldDef({}, {})", self.field_def.name, self.field_def.field_type.as_str())
    }
}

#[pyclass]
pub struct Geometry {
    pub geometry: wbvector::Geometry,
}

#[pymethods]
impl Geometry {
    #[new]
    fn new() -> PyResult<Self> {
        Err(PyRuntimeError::new_err("Use factory methods to create"))
    }

    #[staticmethod]
    fn point(x: f64, y: f64) -> Self {
        Geometry {
            geometry: wbvector::Geometry::point(x, y),
        }
    }

    #[staticmethod]
    fn point_z(x: f64, y: f64, z: f64) -> Self {
        Geometry {
            geometry: wbvector::Geometry::point_z(x, y, z),
        }
    }

    #[staticmethod]
    fn line_string(_py: Python<'_>, coords: &Bound<'_, PyList>) -> PyResult<Self> {
        let mut coord_list: Vec<wbvector::Coord> = Vec::new();
        for item in coords.iter() {
            let coord = item.cast::<Coord>()?;
            coord_list.push(coord.borrow().coord.clone());
        }
        Ok(Geometry {
            geometry: wbvector::Geometry::line_string(coord_list),
        })
    }

    #[staticmethod]
    fn polygon(_py: Python<'_>, exterior: &Bound<'_, PyList>, interiors: &Bound<'_, PyList>) -> PyResult<Self> {
        let ext_coords: Vec<wbvector::Coord> = exterior.iter()
            .map(|item| Ok(item.cast::<Coord>()?.borrow().coord.clone()))
            .collect::<PyResult<_>>()?;

        let int_coords: Vec<Vec<wbvector::Coord>> = interiors.iter()
            .map(|ring_item| {
                let ring = ring_item.cast::<PyList>()?;
                ring.iter()
                    .map(|item| Ok(item.cast::<Coord>()?.borrow().coord.clone()))
                    .collect::<PyResult<_>>()
            })
            .collect::<PyResult<_>>()?;

        Ok(Geometry {
            geometry: wbvector::Geometry::polygon(ext_coords, int_coords),
        })
    }

    #[staticmethod]
    fn multi_point(_py: Python<'_>, pts: &Bound<'_, PyList>) -> PyResult<Self> {
        let coords: Vec<wbvector::Coord> = pts.iter()
            .map(|item| Ok(item.cast::<Coord>()?.borrow().coord.clone()))
            .collect::<PyResult<_>>()?;
        Ok(Geometry {
            geometry: wbvector::Geometry::multi_point(coords),
        })
    }

    #[staticmethod]
    fn multi_line_string(_py: Python<'_>, lines: &Bound<'_, PyList>) -> PyResult<Self> {
        let line_coords: Vec<Vec<wbvector::Coord>> = lines.iter()
            .map(|line_item| {
                let line = line_item.cast::<PyList>()?;
                line.iter()
                    .map(|item| Ok(item.cast::<Coord>()?.borrow().coord.clone()))
                    .collect::<PyResult<_>>()
            })
            .collect::<PyResult<_>>()?;
        Ok(Geometry {
            geometry: wbvector::Geometry::multi_line_string(line_coords),
        })
    }

    #[staticmethod]
    fn from_wkb(data: &[u8]) -> PyResult<Self> {
        let geometry = wbvector::Geometry::from_wkb(data)
            .map_err(|e| PyRuntimeError::new_err(format!("解析 WKB 失败: {}", e)))?;
        Ok(Geometry { geometry })
    }

    fn geom_type(&self) -> String {
        self.geometry.geom_type().as_str().to_string()
    }

    fn has_z(&self) -> bool {
        self.geometry.has_z()
    }

    fn is_empty(&self) -> bool {
        self.geometry.is_empty()
    }

    fn bbox(&self) -> Option<BBox> {
        self.geometry.bbox().map(|bb| BBox { bbox: bb })
    }

    fn to_wkt(&self) -> String {
        self.geometry.to_wkt()
    }

    fn to_wkb(&self) -> Vec<u8> {
        self.geometry.to_wkb()
    }

    pub fn __repr__(&self) -> String {
        format!("Geometry({})", self.geometry.geom_type().as_str())
    }
}

#[pyclass]
pub struct Layer {
    pub layer: wbvector::Layer,
}

#[pymethods]
impl Layer {
    #[new]
    fn new() -> PyResult<Self> {
        Err(PyRuntimeError::new_err("Use Layer.read() or Layer.create() to create"))
    }

    #[classmethod]
    fn read(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        let layer = wbvector::read(path)
            .map_err(|e| PyRuntimeError::new_err(format!("读取矢量失败: {}", e)))?;
        Ok(Layer { layer })
    }

    #[classmethod]
    fn create(_cls: &Bound<'_, PyType>, name: &str) -> Self {
        Layer {
            layer: wbvector::Layer::new(name),
        }
    }

    fn write(&self, path: &str) -> PyResult<()> {
        let format = wbvector::VectorFormat::detect(path)
            .unwrap_or(wbvector::VectorFormat::GeoPackage);
        wbvector::write(&self.layer, path, format)
            .map_err(|e| PyRuntimeError::new_err(format!("写入矢量失败: {}", e)))?;
        Ok(())
    }

    #[getter]
    fn name(&self) -> &str {
        &self.layer.name
    }

    #[getter]
    fn feature_count(&self) -> usize {
        self.layer.features.len()
    }

    #[getter]
    fn crs(&self) -> Option<String> {
        self.layer.crs.as_ref().and_then(|c| c.wkt.clone())
    }

    #[getter]
    fn crs_epsg(&self) -> Option<u32> {
        self.layer.crs_epsg()
    }

    #[getter]
    fn geom_type(&self) -> String {
        self.layer.geom_type.as_ref().map_or("Unknown".to_string(), |gt| gt.as_str().to_string())
    }

    fn set_crs_epsg(&mut self, epsg: u32) {
        self.layer.assign_crs_epsg(epsg);
    }

    fn set_crs_wkt(&mut self, wkt: &str) {
        self.layer.assign_crs_wkt(wkt);
    }

    fn add_field(&mut self, field_def: &FieldDef) {
        self.layer.add_field(field_def.field_def.clone());
    }

    fn add_feature(&mut self, geometry: Option<&Geometry>, attrs: Vec<(String, Py<PyAny>)>) -> PyResult<()> {
        let geom = geometry.map(|g| g.geometry.clone());
        let mut field_values: Vec<(String, wbvector::FieldValue)> = Vec::with_capacity(attrs.len());
        for (name, val) in attrs {
            let field_val = convert_pyobject_to_fieldvalue(val).map_err(|e| PyRuntimeError::new_err(e))?;
            field_values.push((name, field_val));
        }
        let field_refs: Vec<(&str, wbvector::FieldValue)> = field_values
            .iter()
            .map(|(n, v)| (n.as_str(), v.clone()))
            .collect();
        self.layer.add_feature(geom, &field_refs)
            .map_err(|e| PyRuntimeError::new_err(format!("添加要素失败: {}", e)))?;
        Ok(())
    }

    fn bbox(&mut self) -> Option<BBox> {
        self.layer.bbox().map(|bb| BBox { bbox: bb })
    }

    fn get_feature(&self, index: usize) -> PyResult<Py<PyAny>> {
        let feature = self.layer.features.get(index)
            .ok_or_else(|| PyRuntimeError::new_err(format!("要素索引 {} 超出范围", index)))?;

        Python::attach(|py| {
            let geom = feature.geometry.as_ref().map(|g| Geometry { geometry: g.clone() });
            let attrs: Vec<(String, Py<PyAny>)> = self.layer.schema.fields()
                .iter()
                .enumerate()
                .map(|(i, fd)| {
                    let val = feature.attributes.get(i).unwrap_or(&wbvector::FieldValue::Null);
                    (fd.name.clone(), convert_fieldvalue_to_pyobject(py, val))
                })
                .collect();

            let dict = PyDict::new(py);
            dict.set_item("fid", feature.fid)?;
            dict.set_item("geometry", match geom {
                Some(g) => g.into_bound_py_any(py)?.into(),
                None => py.None(),
            })?;
            dict.set_item("attributes", attrs)?;
            Ok(dict.into())
        })
    }

    fn get_attribute(&self, feature_index: usize, field_name: &str) -> PyResult<Py<PyAny>> {
        let feature = self.layer.features.get(feature_index)
            .ok_or_else(|| PyRuntimeError::new_err(format!("要素索引 {} 超出范围", feature_index)))?;
        let value = feature.get(&self.layer.schema, field_name)
            .map_err(|e| PyRuntimeError::new_err(format!("获取属性失败: {}", e)))?;
        Python::attach(|py| Ok(convert_fieldvalue_to_pyobject(py, value)))
    }

    fn reproject(&mut self, target_epsg: u32) -> PyResult<Layer> {
        let reprojected = self.layer.reproject_to_epsg(target_epsg)
            .map_err(|e| PyRuntimeError::new_err(format!("投影转换失败: {}", e)))?;
        Ok(Layer { layer: reprojected })
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Layer(name={}, features={})",
            self.layer.name,
            self.layer.features.len()
        )
    }
}

fn convert_fieldvalue_to_pyobject(py: Python<'_>, val: &wbvector::FieldValue) -> Py<PyAny> {
    match val {
        wbvector::FieldValue::Integer(v) => Py::from((*v).into_pyobject(py).unwrap()),
        wbvector::FieldValue::Float(v) => Py::from((*v).into_pyobject(py).unwrap()),
        wbvector::FieldValue::Text(v) => Py::from(v.clone().into_pyobject(py).unwrap()),
        wbvector::FieldValue::Boolean(v) => Py::from((*v).into_pyobject(py).unwrap()).into(),
        wbvector::FieldValue::Blob(v) => Py::from(v.clone().into_pyobject(py).unwrap()),
        wbvector::FieldValue::Date(v) => Py::from(v.clone().into_pyobject(py).unwrap()),
        wbvector::FieldValue::DateTime(v) => Py::from(v.clone().into_pyobject(py).unwrap()),
        wbvector::FieldValue::Null => py.None(),
    }
}

fn convert_pyobject_to_fieldvalue(val: Py<PyAny>) -> Result<wbvector::FieldValue, String> {
    Python::attach(|py| {
        if val.is_none(py) {
            return Ok(wbvector::FieldValue::Null);
        }
        if let Ok(v) = val.extract::<i64>(py) {
            return Ok(wbvector::FieldValue::Integer(v));
        }
        if let Ok(v) = val.extract::<f64>(py) {
            return Ok(wbvector::FieldValue::Float(v));
        }
        if let Ok(v) = val.extract::<bool>(py) {
            return Ok(wbvector::FieldValue::Boolean(v));
        }
        if let Ok(v) = val.extract::<&str>(py) {
            return Ok(wbvector::FieldValue::Text(v.to_string()));
        }
        if let Ok(v) = val.extract::<Vec<u8>>(py) {
            return Ok(wbvector::FieldValue::Blob(v));
        }
        Err("不支持的属性值类型".to_string())
    })
}