// Geospatial Modelling
mod run;
mod spatial;
mod bathing_waters;
mod utils;

use std::time::Instant;

use geo::{Point, Polygon};

fn main() {

    println!("Requesting bathing water polygons from SEPA GIS system...");
    let start = Instant::now();
    let polys = bathing_waters::Polygons::get();

    let duration = start.elapsed();

    println!("server response & data extraction completed in : {:?}", duration);
    println!("Dataset CRS : {:?}", bathing_waters::Polygons::CRS.code);

    println!("Requesting bathing water points from SEPA GIS system...");
    let start = Instant::now();
    let points = bathing_waters::Points::get();

    let duration = start.elapsed();

    println!("server response & data extraction completed in : {:?}", duration);
    println!("Dataset CRS : {:?}", bathing_waters::Points::CRS.code);

    // Extract location & geometry for points & poly's
    let point_vec:Vec<(String, Point)> = points.iter().map(|p| {
        let point = spatial::update_point_crs(
            p.shape, 
            &crs_definitions::EPSG_27700,
            &crs_definitions::EPSG_4326
        );

        return (String::from(&p.description), point);
    }).collect();

    let poly_vec:Vec<(String, Polygon)> = polys.iter().map(|p| {
        let poly = spatial::update_poly_crs(
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
            let dist = spatial::point_polygon_distance(point, poly, &spatial::DistanceMethod::Haversine);
            dist_vec.push((point_loc, poly_loc, dist));
        }
    }
    let duration = start.elapsed();
    println!("distance analysis complete in : {:?} for {} pairs", duration, dist_vec.len());
    
    // Sort by dist (low to high) 
    dist_vec.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    println!("{:#?}", &dist_vec[0..100]);
}
