// Geospatial Modelling

use crs_definitions as crs_refs;
use proj4rs::proj::Proj;
use geo::{ Coord, MapCoords, Point, Polygon, point, Closest, ClosestPoint, };
use geo::{ Distance, Geodesic, Haversine };

// Coord Ref Systems
pub fn update_poly_crs(polygon:&Polygon, active_crs:&crs_refs::Def, target_crs:&crs_refs::Def) -> Polygon {
  return polygon.map_coords(|Coord{x, y}| {
      let new_point:Point = update_point_crs(Point::new(x, y), active_crs, target_crs);
      return Coord { x: new_point.x(), y: new_point.y() };
  });
}

pub fn update_point_crs(point:Point, active_crs:&crs_refs::Def, target_crs:&crs_refs::Def) -> Point {
  // Setup mutable copy
  let mut point_mut = (point.x(), point.y());
  
  // Transform
  let projection = proj4rs::transform::transform(
      &Proj::from_proj_string(active_crs.proj4).unwrap(), 
      &Proj::from_proj_string(target_crs.proj4).unwrap(),
      &mut point_mut
  );

  match projection {
      Ok(_) => return point!(( point_mut.0.to_degrees(), point_mut.1.to_degrees() )),
      Err(_) => return point,
  }
}

// Spatial Distance
pub enum DistanceMethod {
  Haversine,
  Geodesic,
}

pub fn find_closest_point(point: &Point, polygon: &Polygon) -> Point{
  // Closest point
  match polygon.closest_point(point) {
      Closest::SinglePoint(point) => {
          return point;
      }
      Closest::Intersection(intersection) => {
          return intersection;
      }
      Closest::Indeterminate => {
          return Point::new(0.0, 0.0);
      }
  };
}

pub fn point_distance(point:&Point, to_point:&Point, method:&DistanceMethod) -> f64 {
  match method {
      &DistanceMethod::Haversine => return Haversine::distance(point.clone(),to_point.clone()),
      &DistanceMethod::Geodesic => return Geodesic::distance(point.clone(), to_point.clone()), 
  }
}

pub fn point_polygon_distance(point: &Point, to_polygon:&Polygon, method:&DistanceMethod) -> f64 {
  let to_point = find_closest_point(point, to_polygon);
  return point_distance(point, &to_point, method);
}

pub fn polygon_distance(polygon: &Polygon, to_polygon:&Polygon, method:&DistanceMethod) -> f64 {
    // get list of to_polygon points
    let poly_vec: Vec<Coord> = to_polygon.exterior().0.clone();
    let mut dist_map: Vec<(Point, Point, f64)> = vec![];

    for point in poly_vec {
      let closest = find_closest_point(&Point(point), &polygon);
      let dist = point_distance(&Point(point), &closest, method);

      dist_map.push((Point(point), closest, dist));
    }

    let mut closest = dist_map[0];
    for set in dist_map {
      if set.2 > closest.2 {
        closest = set;
      }
    }

    return closest.2;
}


// Testing

// Coord Ref

#[test]
fn test_update_poly_crs() {
  use geo::polygon;
    
  // CRS setup
  let active_crs = crs_refs::EPSG_27700;
  let target_crs = crs_refs::EPSG_4326;

  // Create polygons
  let polygon:Polygon<f64> = polygon![
      (x:-3.2007650172960296, y:55.95032325369335),
      (x:-3.3007650172960296, y:55.95042325369335),
      (x:-3.4007650172960296, y:55.95052325369335),
  ];

  let test_poly:Polygon<f64> = polygon![
      (x: -7.557261484156532, y: 49.767305964448056), 
      (x: -7.5572628663338755, y: 49.76730589884599), 
      (x: -7.557264248511213, y: 49.767305833243896), 
      (x: -7.557261484156532, y: 49.767305964448056 ),
  ];

  // Transform
  let poly_tf = update_poly_crs(&polygon, &active_crs, &target_crs);

  // Test results
  assert_eq!(poly_tf, test_poly);

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
  assert_eq!((point_test.x() * 1000000.0).round() / 1000000.0, (point_tf.x() * 1000000.0).round() / 1000000.0);
  assert_eq!((point_test.y() * 1000000.0).round() / 1000000.0, (point_tf.y() * 1000000.0).round() / 1000000.0);
}


// Spatial Distance 

#[test]
fn test_find_closest_point(){
  use geo::{polygon, Coord};    
  let test_target:Point<f64> = Point(Coord { x: 325113.5269645548, y: 673695.0227932289 });

  let point = Point::new(325113.4269645748, 673695.0127932389); 
  let polygon:Polygon<f64> = polygon![
      (x: 325113.5269645548, y: 673695.0227932289),
      (x: 325113.5269645948, y: 673695.0227932489),
      (x: 325113.5269646148, y: 673695.0227932689),
      (x: 325113.5269645548, y: 673695.0227932289),
  ];

  let closest = find_closest_point(&point, &polygon);

  assert_eq!(closest, test_target);
  
}

#[test]
fn test_point_distance() {
  let new_york_city:Point<f64> = Point::new(-74.006f64, 40.7128f64);
  let edinburgh: Point<f64> = Point::new(-3.2007650172960296, 55.95042325369335);
  
  let distance = point_distance(&new_york_city, &edinburgh, &DistanceMethod::Haversine);
  
  assert_eq!(
      5_243_773.0, // meters
      distance.round()
  );
}

#[test]
fn test_polygon_distance() {
  use geo::polygon;
  use crs_definitions as crs_refs;

    // CRS setup
    let active_crs = crs_refs::EPSG_27700;
    let target_crs = crs_refs::EPSG_4326;

    // Create polygons
    let polygon:Polygon<f64> = polygon![
        (x: 225113.5269645548, y: 673695.0227932289),
        (x: 325113.5269645948, y: 673695.0227932489),
        (x: 325113.5269646148, y: 673695.0227932689),
    ];

    let polygon_alt:Polygon<f64> = polygon![
        (x: 335113.5269645548, y: 773695.0227932289),
        (x: 335113.5269645948, y: 773695.0227932489),
        (x: 335113.5269646148, y: 773695.0227932689),
    ];

    // Transform
    let poly_tf = update_poly_crs(&polygon, &active_crs, &target_crs);
    let poly_alt_tf = update_poly_crs(&polygon_alt, &active_crs, &target_crs);

    // Poly to poly dist 
    let poly_dist_h = polygon_distance(&poly_tf, &poly_alt_tf, &DistanceMethod::Haversine);
    let poly_dist_g = polygon_distance(&poly_tf, &poly_alt_tf, &DistanceMethod::Geodesic);
    let poly_dist_var = 100.0 * ((poly_dist_g - poly_dist_h) / poly_dist_g);

    // Test distances (km)
    let test_dist_h = 100.38946826751382;
    let test_dist_g = 100.52992635399895;
    let test_var = 0.139717685647686;

    assert_eq!(poly_dist_g / 1000.0, test_dist_g);
    assert_eq!(poly_dist_h  / 1000.0, test_dist_h);
    assert_eq!(poly_dist_var, test_var);
}

