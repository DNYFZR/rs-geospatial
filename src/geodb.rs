// Geodatabase handler module
use crate::utils::unzip;
use std::fs::{ File, read_dir, create_dir, remove_file };
use std::io::{ Cursor, copy };
use reqwest::blocking::get;
use rusqlite::Connection;
use crs_definitions;
use geo::{Geometry, Point};
use geozero::wkb::{ FromWkb, WkbDialect };

#[derive(Debug)]
pub struct Points {
    pub object_id:i64,
    pub shape:Point,
    pub bw_id:String,
    pub description:String,
    pub year:i64,
    pub current:String,
    pub class_id:String,
    pub class_description:String,
    pub bw_url:String,
}

#[derive(Debug)]
pub struct GeoDB {
    url:String,
    zipfile:Option<String>,
    db:String,
    table:String,
    crs:crs_definitions::Def,
}

impl GeoDB {
    pub fn example_points() -> GeoDB {
      return GeoDB {
        url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip".to_string(),
        zipfile: Some("SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip".to_string()),
        db: "SEPA_BATHING_WATER_POINTS_BNG.gpkg".to_string(),
        table: "SEPA_BATHING_WATER_POINTS_BNG".to_string(),
        crs: crs_definitions::EPSG_27700,
      };      
    } 

    pub fn example_polygons() -> GeoDB {
      return GeoDB {
        url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip".to_string(),
        zipfile: Some("SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip".to_string()),
        db: "SEPA_BATHING_WATER_POLYGONS_BNG.gpkg".to_string(),
        table: "SEPA_BATHING_WATER_POLYGONS_BNG".to_string(),
        crs: crs_definitions::EPSG_27700,
      };      
    } 

    pub fn extract_points(&self) -> Vec<Points> {
        // Extract from url
        match read_dir("tmp") {
            Ok(_) => (),
            Err(_) => create_dir("tmp").expect("failed to create working dir"), 
        }

        // Get archive from url
        let mut response = get(&self.url).expect("failed to get file from url");

        // Copy to working dir
        if self.zipfile.is_some() {
            let zip_path = format!("tmp/{}", self.zipfile.clone().unwrap());
            let mut file = File::create(&format!("{}", &zip_path))
                    .expect("failed to create db_path");
            
            copy(&mut response, &mut file)
                    .expect("failed to copy content to db_path");

            // Unzip archive
            unzip(&format!("{}", &zip_path), "tmp");
        }
        
        //  Connect & query
        let conn = Connection::open(&format!("tmp/{}", &self.db)).expect("failed to connect to DB");
        let mut engine = conn.prepare(&format!("SELECT * FROM {}", &self.table)).expect("SQL prep error");
        let mut rows = engine.query([]).expect("Row query failed");
        
        // Extract data
        let mut data = vec![];
        while let Some(row) = rows.next().expect("while error") {
            // Get shape col as bytes
            let shape_entry: Vec<u8> = row.get(1).expect("failed to get row");

            // Extract geometry from bytes array
            let mut point: Point = Point::new(0.0, 0.0);
            let mut bytes_cursor = Cursor::new(&shape_entry);
            let geometry = FromWkb::from_wkb(
                    &mut bytes_cursor, 
                    WkbDialect::Geopackage
            );
            
            match geometry {
                Ok(Geometry::Point(mp)) => {point = Point::from(mp);}
                _ => ()
            }
            
            // Add row to output container
            data.push(Points {
                object_id : row.get(0).expect("failed to get row"),
                shape : point,
                bw_id : row.get(2).expect("failed to get row"),
                description : row.get(3).expect("failed to get row"),
                year : row.get(4).expect("failed to get row"),
                current : row.get(5).expect("failed to get row"),
                class_id : String::from("null"),
                class_description : row.get(7).expect("failed to get row"),
                bw_url :  row.get(8).expect("failed to get row"),
            });
        };

        // Clean up temp files
        if self.zipfile.is_some() {
            remove_file(&format!("tmp/{}", &self.zipfile.clone().unwrap())).expect("failed to remove zip archive from working dir");
        }
        // remove_file(&format!("tmp/{}", &self.db)).expect("failed to remove DB from working dir");

        return data;  
    }

}

// ToDo - tests