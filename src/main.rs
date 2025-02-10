// Geospatial Modelling
mod coord;
mod dist;
mod utils;
mod points;
mod polygons;

fn main() {
   let target = "spatial";
   match target {
      "bathing waters" => get_bw(),
      "spatial" => get_spatial(),
      _ => print!("Current functionality : 'bathing waters', 'spatial', "),
  }
}

use std::time::Instant;
use crs_definitions as crs_refs;
use geo::{ polygon, Point, Polygon };

fn get_bw() {
    println!("Requesting bathing water polygons from SEPA GIS system...");
    let start = Instant::now();
    let polys = polygons::Polygons::get();

    let duration = start.elapsed();

    println!("server response & data extraction completed in : {:?}", duration);
    println!("Dataset CRS : {:?}", polygons::Polygons::CRS.code);

    println!("Requesting bathing water points from SEPA GIS system...");
    let start = Instant::now();
    let points = points::Points::get();

    let duration = start.elapsed();

    println!("server response & data extraction completed in : {:?}", duration);
    println!("Dataset CRS : {:?}", points::Points::CRS.code);

    // Extract location & geometry for points & poly's
    let point_vec:Vec<(String, Point)> = points.iter().map(|p| {
        let point = coord::update_point_crs(
            p.shape, 
            &crs_definitions::EPSG_27700,
            &crs_definitions::EPSG_4326
        );

        return (String::from(&p.description), point);
    }).collect();

    let poly_vec:Vec<(String, Polygon)> = polys.iter().map(|p| {
        let poly = coord::update_poly_crs(
            &p.shape.0[0], 
            &crs_definitions::EPSG_27700, 
            &crs_definitions::EPSG_4326
        );

        return (String::from(&p.description), poly);
    }).collect();

    // Calculate distance matrix
    let mut dist_vec = vec![];
    let start = Instant::now();

    for (point_loc, point) in &point_vec {
        for (poly_loc, poly) in &poly_vec {
            let dist = dist::point_polygon_distance(point, poly, &dist::DistanceMethod::Haversine);
            dist_vec.push((point_loc, poly_loc, dist));
        }
    }
    let duration = start.elapsed();
    println!("distance analysis complete in : {:?} for {} pairs", duration, dist_vec.len());

    // Sort by dist (low to high) 
    dist_vec.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    println!("{:#?}", &dist_vec[0..100]);
}

fn get_spatial() {
     // CRS setup
     let active_crs = crs_refs::EPSG_27700;
     let target_crs = crs_refs::EPSG_4326;
 
     // Create a Point
     let point = Point::new(335113.4269645748, 683695.0127932389); 
 
     // Create a poly
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
     let point_tf = coord::update_point_crs(point, &active_crs, &target_crs);
     let poly_tf = coord::update_poly_crs(&polygon, &active_crs, &target_crs);
     let poly_alt_tf = coord::update_poly_crs(&polygon_alt, &active_crs, &target_crs);
     
 
     // Closest point
     let closest:Point<f64> = dist::find_closest_point(&point_tf, &poly_tf);
 
     // Distance - methods require lat / long - must use EPSG_4326 vals
     let dist_h = dist::point_distance(
         &point_tf, 
         &closest, 
         &dist::DistanceMethod::Haversine,
     );
 
     let dist_g = dist::point_distance(
         &point_tf, 
         &closest, 
         &dist::DistanceMethod::Geodesic,
     );
 
     // Poly to poly dist 
     let poly_dist_h = dist::polygon_distance(&poly_tf, &poly_alt_tf, &dist::DistanceMethod::Haversine);
     let poly_dist_g = dist::polygon_distance(&poly_tf, &poly_alt_tf, &dist::DistanceMethod::Geodesic);
 
     println!("Transformed : {:?}", point_tf);
     println!("Transformed : {:?}", poly_tf);
     println!();
     
     println!("Closest Points : \n- {:?} \n- {:?}", point_tf, closest);
     println!();
     
     println!("Point to Point");
     println!("Haversine distance (km) : {:?}", (dist_h / 1000.0));
     println!("Geodesic distance (km) : {:?}", (dist_g / 1000.0));
     println!("Variance (%) : {:?}", 100.0 * ((dist_g - dist_h) / dist_g) );
     println!();
     
     println!("Polygon to Polygon");
     println!("Haversine distance (km) : {:?}", (poly_dist_h / 1000.0));
     println!("Geodesic distance (km) : {:?}", (poly_dist_g / 1000.0));
     println!("Variance (%) : {:?}", 100.0 * ((poly_dist_g - poly_dist_h) / poly_dist_g) );
 
}

