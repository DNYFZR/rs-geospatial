// Geodatabase handler
use crate::utils::unzip;
use crs_definitions;
use geo::{Geometry, MultiPolygon, Point, Polygon};
use geozero::wkb::{FromWkb, WkbDialect};
use reqwest::blocking::get;
use rusqlite::Connection;
use std::fs::{File, create_dir, read_dir, remove_file};
use std::io::{Cursor, copy};

#[derive(Debug, PartialEq)]
pub struct GeoData {
    pub uuid: String,
    pub point: Option<Point>,
    pub polygon: Option<Polygon>,
    pub multipolygon: Option<MultiPolygon>,
}

#[derive(Debug, PartialEq)]
pub struct GeoDB {
    pub url: String,
    pub zipfile: Option<String>,
    pub db: String,
    pub table: String,
    pub crs: crs_definitions::Def,
    pub uuid_col_idx: i32,
    pub geometry_col_idx: i32,
}

impl GeoDB {
    pub fn example_points_db() -> GeoDB {
        return GeoDB {
            url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip"
                .to_string(),
            zipfile: Some("SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip".to_string()),
            db: "SEPA_BATHING_WATER_POINTS_BNG.gpkg".to_string(),
            table: "SEPA_BATHING_WATER_POINTS_BNG".to_string(),
            crs: crs_definitions::EPSG_27700,
            geometry_col_idx: 1,
            uuid_col_idx: 8,
        };
    }

    pub fn example_polygons_db() -> GeoDB {
        return GeoDB {
            url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip"
                .to_string(),
            zipfile: Some("SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip".to_string()),
            db: "SEPA_BATHING_WATER_POLYGONS_BNG.gpkg".to_string(),
            table: "SEPA_BATHING_WATER_POLYGONS_BNG".to_string(),
            crs: crs_definitions::EPSG_27700,
            geometry_col_idx: 1,
            uuid_col_idx: 8,
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
            let mut file =
                File::create(&format!("{}", &zip_path)).expect("failed to create db_path");

            copy(&mut response, &mut file).expect("failed to copy content to db_path");

            // Unzip archive
            unzip(&format!("{}", &zip_path), "tmp");
        }
    }

    pub fn extract(&self) -> Vec<GeoData> {
        let _ = &self.get_gdb();
        let mut data = vec![];

        {
            // Context block ensures db connection is closed & subsiquent deletion can run
            let conn =
                Connection::open(&format!("tmp/{}", &self.db)).expect("failed to connect to DB");
            let mut engine = conn
                .prepare(&format!("SELECT * FROM {}", &self.table))
                .expect("SQL prep error");
            let mut rows = engine.query([]).expect("Row query failed");

            while let Some(row) = rows.next().expect("while error") {
                // Extract geometry from bytes array
                let shape_entry: Vec<u8> = row
                    .get(self.geometry_col_idx as usize)
                    .expect("failed to get row");
                let mut bytes_cursor = Cursor::new(&shape_entry);
                let geometry = FromWkb::from_wkb(&mut bytes_cursor, WkbDialect::Geopackage);

                match geometry {
                    Ok(Geometry::Point(mp)) => {
                        data.push(GeoData {
                            uuid: row
                                .get(self.uuid_col_idx as usize)
                                .expect("failed to get uuid"),
                            point: Some(mp),
                            polygon: None,
                            multipolygon: None,
                        });
                    }
                    Ok(Geometry::Polygon(mp)) => {
                        data.push(GeoData {
                            uuid: row
                                .get(self.uuid_col_idx as usize)
                                .expect("failed to get uuid"),
                            point: None,
                            polygon: Some(mp),
                            multipolygon: None,
                        });
                    }
                    Ok(Geometry::MultiPolygon(mp)) => {
                        data.push(GeoData {
                            uuid: row
                                .get(self.uuid_col_idx as usize)
                                .expect("failed to get uuid"),
                            point: None,
                            polygon: None,
                            multipolygon: Some(mp),
                        });
                    }
                    _ => (),
                }
            }
        }

        // Clean up
        remove_file(&format!("tmp/{}", &self.db)).expect("failed to remove DB from working dir");
        if self.zipfile.is_some() {
            remove_file(&format!("tmp/{}", &self.zipfile.clone().unwrap()))
                .expect("failed to remove zip archive from working dir");
        }

        return data;
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
        geometry_col_idx: 1,
        uuid_col_idx: 8,
    };

    assert_eq!(test, GeoDB::example_points_db());
}

#[test]
fn test_example_polygons() {
    let test = GeoDB {
        url: "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip"
            .to_string(),
        zipfile: Some("SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip".to_string()),
        db: "SEPA_BATHING_WATER_POLYGONS_BNG.gpkg".to_string(),
        table: "SEPA_BATHING_WATER_POLYGONS_BNG".to_string(),
        crs: crs_definitions::EPSG_27700,
        geometry_col_idx: 1,
        uuid_col_idx: 8,
    };

    assert_eq!(test, GeoDB::example_polygons_db());
}

#[test]
fn test_extract() {
    // All points dataset test
    let test = GeoDB::example_points_db().extract();
    for row in test {
        assert!(row.point.is_some());
        assert!(row.multipolygon.is_none());
    }

    // All polygon dataset test
    let test = GeoDB::example_polygons_db().extract();
    for row in test {
        assert!(row.point.is_none());
        assert!(row.multipolygon.is_some());
    }
}
