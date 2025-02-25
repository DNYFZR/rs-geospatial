// Geospatial Modelling
use crs_definitions as crs_refs;
use geo::{Coord, MapCoords, Point, Polygon, point};
use proj4rs::proj::Proj;

pub fn update_poly_crs(
    polygon: &Polygon,
    active_crs: &crs_refs::Def,
    target_crs: &crs_refs::Def,
) -> Polygon {
    return polygon.map_coords(|Coord { x, y }| {
        let new_point: Point = update_point_crs(Point::new(x, y), active_crs, target_crs);
        return Coord {
            x: new_point.x(),
            y: new_point.y(),
        };
    });
}

pub fn update_point_crs(
    point: Point,
    active_crs: &crs_refs::Def,
    target_crs: &crs_refs::Def,
) -> Point {
    // Setup mutable copy
    let mut point_mut = (point.x(), point.y());

    // Transform
    let projection = proj4rs::transform::transform(
        &Proj::from_proj_string(active_crs.proj4).unwrap(),
        &Proj::from_proj_string(target_crs.proj4).unwrap(),
        &mut point_mut,
    );

    match projection {
        Ok(_) => return point!((point_mut.0.to_degrees(), point_mut.1.to_degrees())),
        Err(_) => return point,
    }
}

#[test]
fn test_update_poly_crs() {
    use geo::polygon;

    // CRS setup
    let active_crs = crs_refs::EPSG_27700;
    let target_crs = crs_refs::EPSG_4326;

    // Create polygons
    let polygon: Polygon<f64> = polygon![
        (x:-3.2007650172960296, y:55.95032325369335),
        (x:-3.3007650172960296, y:55.95042325369335),
        (x:-3.4007650172960296, y:55.95052325369335),
    ];

    let test_poly: Polygon<f64> = polygon![
        (x: -7.557261484156533, y: 49.76730596444806),
        (x: -7.5572628663338755, y: 49.76730589884599),
        (x: -7.557264248511213, y: 49.767305833243896),
        (x: -7.557261484156533, y: 49.76730596444806 ),
    ];

    // Transform
    let poly_tf = update_poly_crs(&polygon, &active_crs, &target_crs);

    // Test results
    assert_eq!(poly_tf.exterior(), test_poly.exterior());
}

#[test]
fn test_update_point_crs() {
    // CRS setup
    let active_crs = crs_refs::EPSG_3034;
    let target_crs = crs_refs::EPSG_4326;

    // Create a Point
    let point_test: Point<f64> = Point::new(-3.2007650172960296, 55.95042325369335); // EPSG_4326
    let point: Point<f64> = Point::new(3204612.663745465, 3296560.540899951); // EPSG_3034

    // Transform
    let point_tf = update_point_crs(point, &active_crs, &target_crs);

    // Test results
    assert_eq!(
        (point_test.x() * 1000000.0).round() / 1000000.0,
        (point_tf.x() * 1000000.0).round() / 1000000.0
    );
    assert_eq!(
        (point_test.y() * 1000000.0).round() / 1000000.0,
        (point_tf.y() * 1000000.0).round() / 1000000.0
    );
}
