// Geospatial Modelling
mod run;
mod spatial;
mod bathing_waters;
mod utils;

use std::time::Instant;

fn main() {
    // run::rs_geo();

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

    for i in 0..1 {
        let point_set = &points[i];
        let poly_set = &polys[i];
    
        let object_id = &poly_set.object_id;
        let poly = &poly_set.shape.0[0];
        let point = &point_set.shape;
        let bw_id = &poly_set.bw_id;
        let description = &poly_set.description;
        let year = &poly_set.year;
        let current = &poly_set.current;
        let class_id = &poly_set.class_id;
        let class_description = &poly_set.class_description;
        let bw_url = &poly_set.bw_url;
    
        println!("Entry {i} :
            object_id = {object_id}
            description = {description}
            year = {year}
            current = {current}
            class_id = {class_id}
            class_description = {class_description}
            bw_url = {bw_url}
            point_id = {bw_id}
            point = {:?};
            poly_id = {},
            polygon = {:?}
        ", point, poly_set.bw_id, poly);
    }
}
