// Geospatial Distance Calculations
use geo::{ Coord, Point, Polygon, Closest, ClosestPoint, };
use geo::{ Distance, Geodesic, Haversine };

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
  use crate::coord;
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
    let poly_tf = coord::update_poly_crs(&polygon, &active_crs, &target_crs);
    let poly_alt_tf = coord::update_poly_crs(&polygon_alt, &active_crs, &target_crs);

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

#[test]
fn test_point_to_polygon_distance() {
  use crate::coord;
  use geo::{point, polygon};
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

    let point:Point = point!(x: 335113.5269645548, y: 773695.0227932289);

    // Transform
    let polygon_tf = coord::update_poly_crs(&polygon, &active_crs, &target_crs);
    let point_tf = coord::update_point_crs(point, &active_crs, &target_crs);

    // Point to poly dist 
    let dist = point_polygon_distance(&point_tf, &polygon_tf, &DistanceMethod::Haversine);

    // Test distances (km)
    let test_dist = 100.3894682674663;
    assert_eq!(dist  / 1000.0, test_dist);
}

