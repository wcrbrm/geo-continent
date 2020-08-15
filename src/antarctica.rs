extern crate geo;
pub fn get_polygon() -> geo::Polygon<f32> {
    let exterior = geo::LineString::from(vec![
        (-180.000000, -90.000000),
        (180.000000, -90.000000),
        (180.000000, -60.000000),
        (-180.000000, -60.000000),
        (-180.000000, -90.000000),
    ]);

    let interior = vec![];
    geo::Polygon::new(exterior, interior)
}
