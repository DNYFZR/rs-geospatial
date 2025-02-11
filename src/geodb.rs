// Geodatabase handler module
use crate::utils::unzip;
use std::fs::{ File, read_dir, create_dir, remove_file };
use std::io::{ Cursor, copy };
use reqwest::blocking::get;
use rusqlite::Connection;
use crs_definitions;
use geo::{Geometry, MultiPolygon, Point};
use geozero::wkb::{ FromWkb, WkbDialect };

#[derive(Debug, PartialEq)]
pub struct GeoRow {
    pub object_id:i64,
    pub point: Option<Point>,
    pub polygon: Option<MultiPolygon>,
    pub bw_id:String,
    pub description:String,
    pub year:i64,
    pub current:String,
    pub class_id:String,
    pub class_description:String,
    pub bw_url:String,
}

#[derive(Debug, PartialEq)]
pub struct GeoTable {
    pub rows:Vec<GeoRow>,
}

#[derive(Debug, PartialEq)]
pub struct GeoDB {
    url:String,
    zipfile:Option<String>,
    db:String,
    table:String,
    crs:crs_definitions::Def,
}

impl GeoDB {
    pub fn example_points_db() -> GeoDB {
      return GeoDB {
        url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip".to_string(),
        zipfile: Some("SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip".to_string()),
        db: "SEPA_BATHING_WATER_POINTS_BNG.gpkg".to_string(),
        table: "SEPA_BATHING_WATER_POINTS_BNG".to_string(),
        crs: crs_definitions::EPSG_27700,
      };      
    } 

    pub fn example_polygons_db() -> GeoDB {
      return GeoDB {
        url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip".to_string(),
        zipfile: Some("SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip".to_string()),
        db: "SEPA_BATHING_WATER_POLYGONS_BNG.gpkg".to_string(),
        table: "SEPA_BATHING_WATER_POLYGONS_BNG".to_string(),
        crs: crs_definitions::EPSG_27700,
      };      
    } 

    fn get_gdb(&self) {
        // Setup working dir if req'd
        match read_dir("tmp") {
            Ok(_) => (),
            Err(_) => create_dir("tmp").expect("failed to create working dir"), 
        }

        // Extract from url
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
    }

    pub fn extract(&self) -> GeoTable {
        let _ = &self.get_gdb();
        let mut data = vec![];
        
        { // Context block ensures db connection is closed & subsiquent deletion can run
            let conn = Connection::open(&format!("tmp/{}", &self.db)).expect("failed to connect to DB");
            let mut engine = conn.prepare(&format!("SELECT * FROM {}", &self.table)).expect("SQL prep error");
            let mut rows = engine.query([]).expect("Row query failed");
            
            while let Some(row) = rows.next().expect("while error") {
                // Extract geometry from bytes array
                let shape_entry: Vec<u8> = row.get(1).expect("failed to get row");
                let mut bytes_cursor = Cursor::new(&shape_entry);
                let geometry = FromWkb::from_wkb(
                        &mut bytes_cursor, 
                        WkbDialect::Geopackage
                );
                
                match geometry {
                    Ok(Geometry::Point(mp)) => {
                        data.push(GeoRow {
                            object_id : row.get(0).expect("failed to get row"),
                            point: Some(mp),
                            polygon: None,
                            bw_id : row.get(2).expect("failed to get row"),
                            description : row.get(3).expect("failed to get row"),
                            year : row.get(4).expect("failed to get row"),
                            current : row.get(5).expect("failed to get row"),
                            class_id : String::from("null"),
                            class_description : row.get(7).expect("failed to get row"),
                            bw_url :  row.get(8).expect("failed to get row"),
                        });
                    }
                    Ok(Geometry::MultiPolygon(mp)) => {
                        data.push(GeoRow {
                            object_id : row.get(0).expect("failed to get row"),
                            point: None,
                            polygon: Some(mp),
                            bw_id : row.get(2).expect("failed to get row"),
                            description : row.get(3).expect("failed to get row"),
                            year : row.get(4).expect("failed to get row"),
                            current : row.get(5).expect("failed to get row"),
                            class_id : String::from("null"),
                            class_description : row.get(7).expect("failed to get row"),
                            bw_url :  row.get(8).expect("failed to get row"),
                        });
                    }
                    _ => ()
                }
            };
        }

        // Clean up 
        remove_file(&format!("tmp/{}", &self.db)).expect("failed to remove DB from working dir");
        if self.zipfile.is_some() {
            remove_file(&format!("tmp/{}", &self.zipfile.clone().unwrap())).expect("failed to remove zip archive from working dir");
        }
        
        return GeoTable{rows: data};  
    }


}

#[test]
fn test_example_points() {
    let test = GeoDB {
        url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip".to_string(),
        zipfile: Some("SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip".to_string()),
        db: "SEPA_BATHING_WATER_POINTS_BNG.gpkg".to_string(),
        table: "SEPA_BATHING_WATER_POINTS_BNG".to_string(),
        crs: crs_definitions::EPSG_27700,
      };

    assert_eq!(test, GeoDB::example_points_db());
}

#[test]
fn test_example_polygons() {
    let test = GeoDB {
        url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip".to_string(),
        zipfile: Some("SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip".to_string()),
        db: "SEPA_BATHING_WATER_POLYGONS_BNG.gpkg".to_string(),
        table: "SEPA_BATHING_WATER_POLYGONS_BNG".to_string(),
        crs: crs_definitions::EPSG_27700,
      };

    assert_eq!(test, GeoDB::example_polygons_db());
}

#[test]
fn test_extract() {
    // All points dataset test
    let test = GeoDB::example_points_db().extract();
    for row in test.rows {
        assert!(row.point.is_some());
        assert!(row.polygon.is_none());
    }

    // All polygon dataset test
    let test = GeoDB::example_polygons_db().extract();
    for row in test.rows {
        assert!(row.point.is_none());
        assert!(row.polygon.is_some());
    }
}
