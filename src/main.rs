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
    let extract = bathing_waters::Polygons::get();

    let duration = start.elapsed();

    println!("server response & data extraction completed in : {:?}", duration);
    println!("Dataset CRS : {:?}", bathing_waters::Polygons::CRS.code);

    for i in 0..extract.len() {
        let demo_set = &extract[i];
    
        let object_id = &demo_set.object_id;
        let shape = &demo_set.shape.0.len();
        let bw_id = &demo_set.bw_id;
        let description = &demo_set.description;
        let year = &demo_set.year;
        let current = &demo_set.current;
        let class_id = &demo_set.class_id;
        let class_description = &demo_set.class_description;
        let bw_url = &demo_set.bw_url;
    
        println!("Entry {i} :
            object_id = {object_id}
            bw_id = {bw_id}
            description = {description}
            year = {year}
            current = {current}
            class_id = {class_id}
            class_description = {class_description}
            bw_url = {bw_url}
            shape = {:?}
        ", shape);
    }


    println!("Requesting bathing water points from SEPA GIS system...");
    let start = Instant::now();
    let extract = bathing_waters::Points::get();

    let duration = start.elapsed();

    println!("server response & data extraction completed in : {:?}", duration);
    println!("Dataset CRS : {:?}", bathing_waters::Points::CRS.code);

    for i in 0..extract.len() {
        let demo_set = &extract[i];
    
        let object_id = &demo_set.object_id;
        let shape = &demo_set.shape;
        let bw_id = &demo_set.bw_id;
        let description = &demo_set.description;
        let year = &demo_set.year;
        let current = &demo_set.current;
        let class_id = &demo_set.class_id;
        let class_description = &demo_set.class_description;
        let bw_url = &demo_set.bw_url;
    
        println!("Entry {i} :
            object_id = {object_id}
            bw_id = {bw_id}
            description = {description}
            year = {year}
            current = {current}
            class_id = {class_id}
            class_description = {class_description}
            bw_url = {bw_url}
            shape = {:?}
        ", shape);
    }
}
