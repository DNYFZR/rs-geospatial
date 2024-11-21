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
    let point = Point::new(335113.4269645748, 683695.0127932389); 

    // Create a poly
    let polygon:Polygon<f64> = polygon![
        (x: 325113.5269645548, y: 673695.0227932289),
        (x: 325113.5269645948, y: 673695.0227932489),
        (x: 325113.5269646148, y: 673695.0227932689),
        (x: 325113.5269645548, y: 673695.0227932289),
    ];

    // Transform
    let point_tf = update_point_crs(point, &active_crs, &target_crs);
    let poly_tf = update_poly_crs(&polygon, &active_crs, &target_crs);
    

    // Closest point
    let closest:Point<f64> = find_closest_point(&point_tf, &poly_tf);

    // Distance - methods require lat / long - must use EPSG_4326 vals
    let dist_h = point_distance(
        &point_tf, 
        &closest, 
        DistanceMethod::Haversine,
    );

    let dist_g = point_distance(
        &point_tf, 
        &closest, 
        DistanceMethod::Geodesic,
    );


    println!("Transformed : {:?}", point_tf);
    println!("Transformed : {:?}", poly_tf);
    println!();
    
    println!("Closest Points : \n- {:?} \n- {:?}", point_tf, closest);
    println!();
    
    println!("Haversine distance (km) : {:?}", (dist_h / 1000.0));
    println!("Geodesic distance (km) : {:?}", (dist_g / 1000.0));
    println!("Variance (%) : {:?}", 100.0 * ((dist_g - dist_h) / dist_g) );
    println!();
    
}
