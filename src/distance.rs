// Distance Analytics
use geo::{ Closest, ClosestPoint, Geodesic, Point, Polygon };
use geo::{ Haversine, Distance };

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

pub fn point_distance(point:&Point, to_point:&Point, method:DistanceMethod) -> f64 {
  match method {
      DistanceMethod::Haversine => return Haversine::distance(point.clone(),to_point.clone()),
      DistanceMethod::Geodesic => return Geodesic::distance(point.clone(), to_point.clone()), 
  }
}

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
  
  let distance = point_distance(&new_york_city, &edinburgh, DistanceMethod::Haversine);
  
  assert_eq!(
      5_243_773.0, // meters
      distance.round()
  );
}