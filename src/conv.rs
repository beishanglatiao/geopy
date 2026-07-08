use geo_types::{Coord, Geometry, MultiPolygon, Polygon};

/// Convert wbvector geometry to geo_types geometry for rasterization.
pub fn wbgeom_to_geotypes(geom: &wbvector::geometry::Geometry) -> Option<Geometry<f64>> {
    match geom {
        wbvector::geometry::Geometry::Polygon { exterior, interiors } => {
            let ext_coords: Vec<Coord<f64>> = exterior
                .coords()
                .into_iter()
                .map(|c| Coord { x: c.x, y: c.y })
                .collect();
            let int_coords: Vec<geo_types::LineString<f64>> = interiors
                .iter()
                .map(|ring| {
                    geo_types::LineString::new(
                        ring.coords()
                            .into_iter()
                            .map(|c| Coord { x: c.x, y: c.y })
                            .collect(),
                    )
                })
                .collect();
            Some(Geometry::Polygon(Polygon::new(
                geo_types::LineString::new(ext_coords),
                int_coords,
            )))
        }
        wbvector::geometry::Geometry::MultiPolygon(polys) => {
            let multi_polys: Vec<Polygon<f64>> = polys
                .iter()
                .filter_map(|(ext, ints)| {
                    let ext_coords: Vec<Coord<f64>> = ext
                        .coords()
                        .into_iter()
                        .map(|c| Coord { x: c.x, y: c.y })
                        .collect();
                    let int_coords: Vec<geo_types::LineString<f64>> = ints
                        .iter()
                        .map(|ring| {
                            geo_types::LineString::new(
                                ring.coords()
                                    .into_iter()
                                    .map(|c| Coord { x: c.x, y: c.y })
                                    .collect(),
                            )
                        })
                        .collect();
                    Some(Polygon::new(
                        geo_types::LineString::new(ext_coords),
                        int_coords,
                    ))
                })
                .collect();
            Some(Geometry::MultiPolygon(MultiPolygon::new(multi_polys)))
        }
        _ => None,
    }
}
