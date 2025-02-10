// Geopackage Extraction Module
use crate::utils;
use std::{ fs, io };
use rusqlite::Connection;
use reqwest;
use geo::{ polygon, Geometry, MultiPolygon };
use geozero::wkb;
use crs_definitions;

#[derive(Debug)]
pub struct Polygons {
    pub object_id:i64,
    pub shape:MultiPolygon,
    pub bw_id:String,
    pub description:String,
    pub year:i64,
    pub current:String,
    pub class_id:String,
    pub class_description:String,
    pub bw_url:String,
}

impl Polygons {
    pub const CRS:crs_definitions::Def = crs_definitions::EPSG_27700;
    const SEPA_URL:&str = "https://map.sepa.org.uk/atom/Data";
    const SOURCE_DB:&str = "SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip";
    const TMP_DIR:&str = "tmp";
    const TABLE:&str = "SEPA_BATHING_WATER_POLYGONS_BNG";
    const DB:&str = "SEPA_BATHING_WATER_POLYGONS_BNG.gpkg"; 

    pub fn get() -> Vec<Polygons> {
            // Create temp dir (if not available)
            match fs::read_dir(Self::TMP_DIR) {
                Ok(_) => (),
                Err(_) => fs::create_dir(Self::TMP_DIR).expect("failed to create working dir"), 
            }

            // Get archive from url
            let url = format!("{}/{}", Self::SEPA_URL, Self::SOURCE_DB);
            let mut response = reqwest::blocking::get(&url).expect("failed to get file from url");

            // Copy to working dir
            let db_zip_path = &format!("{}/{}", Self::TMP_DIR, Self::SOURCE_DB);
            let mut file = fs::File::create(&db_zip_path)
                    .expect("failed to create db_path");
            
            io::copy(&mut response, &mut file)
                    .expect("failed to copy content to db_path");

            // Unzip archive
            utils::unzip(&db_zip_path, &Self::TMP_DIR);

            // Query the unzipped database
            let data = Self::query();

            // Clean up temp files
            fs::remove_file(&db_zip_path).expect("failed to remove zip archive from working dir");
            fs::remove_file(&format!("{}/{}", Self::TMP_DIR, Self::DB)).expect("failed to remove DB from working dir");
    
            return data;  

    }

    fn query() -> Vec<Polygons> {
        //  Connect to the extracted DB
        let db_path = format!("{}/{}", Self::TMP_DIR, Self::DB);
        let conn = Connection::open(&db_path)
            .expect("failed to connect to DB");

        // Execute query
        let sql = format!("SELECT * FROM {}", Self::TABLE);
        let mut engine = conn.prepare(&sql).expect("SQL prep error");
        let mut rows = engine.query([]).expect("Row query failed");
        let mut data = vec![];
        
        while let Some(row) = rows.next().expect("while error") {
            // Get shape col as bytes
            let shape_entry: Vec<u8> = row.get(1).expect("failed to get row");

            // Extract geometry from bytes array
            let mut multipoly: MultiPolygon<f64> = MultiPolygon::new(vec![polygon!(), ]);
            let mut bytes_cursor = io::Cursor::new(&shape_entry);
            let geometry = wkb::FromWkb::from_wkb(
                    &mut bytes_cursor, 
                    wkb::WkbDialect::Geopackage
            );
            
            match geometry {
                Ok(Geometry::MultiPolygon(mp)) => {multipoly = MultiPolygon::from(mp);}
                _ => ()
            }
            
            // Add row to output container
            data.push(Polygons {
                object_id : row.get(0).expect("failed to get row"),
                shape : multipoly,
                bw_id : row.get(2).expect("failed to get row"),
                description : row.get(3).expect("failed to get row"),
                year : row.get(4).expect("failed to get row"),
                current : row.get(5).expect("failed to get row"),
                class_id : String::from("null"),
                class_description : row.get(7).expect("failed to get row"),
                bw_url :  row.get(8).expect("failed to get row"),
            });
        };

        return data;
    }
}
