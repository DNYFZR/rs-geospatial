// Geospatial Modelling

use crate::crs::{ update_point_crs, update_poly_crs };
use crate::distance::{ DistanceMethod, find_closest_point, point_distance };

use crs_definitions as crs_refs;
use geo::{ polygon, Point, Polygon };

pub fn rs_geo() {
    
    // CRS setup
    let active_crs = crs_refs::EPSG_27700;
    let target_crs = crs_refs::EPSG_4326;

    // Create a Point
    let point = Point::new(325113.4269645748, 673695.0127932389); 

    // Create a poly
    let polygon:Polygon<f64> = polygon![
        (x: 325113.5269645548, y: 673695.0227932289),
        (x: 325113.5269645948, y: 673695.0227932489),
        (x: 325113.5269646148, y: 673695.0227932689),
        (x: 325113.5269645548, y: 673695.0227932289),
    ];

    // Closest point
    let closest:Point<f64> = find_closest_point(&point, &polygon);

    // Distance
    let distance = point_distance(
        &point, 
        &closest, 
        DistanceMethod::Haversine,
    );

    // Transform
    let point_tf = update_point_crs(point, &active_crs, &target_crs);
    let poly_tf = update_poly_crs(&polygon, &active_crs, &target_crs);

    println!("Original : {:?}", point);
    println!("Transformed : {:?}", point_tf);
    println!();

    println!("Original : {:?}", polygon);
    println!();

    println!("Transformed : {:?}", poly_tf);
    println!();
    
    println!("Closest : {:?}", closest);
    println!();

    println!("Haversine distance (m) : {:?}", distance.round());
    println!()
    
}
