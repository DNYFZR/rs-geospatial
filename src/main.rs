// Geospatial Modelling
#![allow(dead_code, unused_imports, unused_variables)]
mod coord;
mod dist;
mod geodb;
mod utils;

fn main() {
    let mut geofiles = std::collections::HashMap::new();
    geofiles.insert("spa_sites",	"https://ogc.nature.scot/geoserver/protectedareas/wfs?request=GetFeature&service=WFS&version=1.1.0&typeName=protectedareas:spa&outputFormat=SHAPE-ZIP");
    geofiles.insert("sssi_sites",	"https://ogc.nature.scot/geoserver/protectedareas/wfs?request=GetFeature&service=WFS&version=1.1.0&typeName=protectedareas:sssi&outputFormat=SHAPE-ZIP");
    geofiles.insert("ramsar_sites",	"https://ogc.nature.scot/geoserver/protectedareas/wfs?request=GetFeature&service=WFS&version=1.1.0&typeName=protectedareas:ramsar&outputFormat=SHAPE-ZIP");
    geofiles.insert("sac_sites",	"https://ogc.nature.scot/geoserver/protectedareas/wfs?request=GetFeature&service=WFS&version=1.1.0&typeName=protectedareas:sac&outputFormat=SHAPE-ZIP");
    geofiles.insert(
        "bathing_waters_areas",
        "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POLYGONS_BNG_gpkg.zip",
    );
    geofiles.insert(
        "bathing_waters_points",
        "https://map.sepa.org.uk/atom/Data/SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip",
    );
    geofiles.insert(
        "river_class",
        "http://map.sepa.org.uk/atom/Data/SEPA_RIVER_WATERBODY_CLASSIFICATIONS_BNG.gdb.zip",
    );
    geofiles.insert(
        "coastal_class",
        "https://map.sepa.org.uk/atom/Data/SEPA_COASTAL_WATERBODY_CLASSIFICATIONS_BNG_gpkg.zip",
    );
    geofiles.insert(
        "eastury_class",
        "https://map.sepa.org.uk/atom/Data/SEPA_ESTUARIES_WATERBODY_CLASSIFICATIONS_BNG_gpkg.zip",
    );
    geofiles.insert(
        "loch_class",
        "http://map.sepa.org.uk/atom/Data/SEPA_LOCH_WATERBODY_CLASSIFICATIONS_BNG_gpkg.zip",
    );
    geofiles.insert(
        "shellfish_waters",
        "https://map.sepa.org.uk/atom/Data/Shellfish_Water_Protected_Areas_BNG.zip",
    );

    let geo_db = geodb::GeoDB {
        url: geofiles["bathing_waters_points"].to_string(),
        zipfile: Some("SEPA_BATHING_WATER_POINTS_BNG_gpkg.zip".to_string()),
        db: "SEPA_BATHING_WATER_POINTS_BNG.gpkg".to_string(),
        table: "SEPA_BATHING_WATER_POINTS_BNG".to_string(),
        crs: crs_definitions::EPSG_27700,
        uuid_col_idx: 8,
        geometry_col_idx: 1,
    };

    let geo_db_data = geo_db.extract();
    println!("{:#?}", geo_db_data);
}

// spatial distance calcs...
fn get_spatial() {
    use crs_definitions as crs_refs;
    use geo::{Point, Polygon, polygon};
    // CRS setup
    let active_crs = crs_refs::EPSG_27700;
    let target_crs = crs_refs::EPSG_4326;

    // Create a Point
    let point = Point::new(335113.4269645748, 683695.0127932389);

    // Create a poly
    let polygon: Polygon<f64> = polygon![
        (x: 225113.5269645548, y: 673695.0227932289),
        (x: 325113.5269645948, y: 673695.0227932489),
        (x: 325113.5269646148, y: 673695.0227932689),
    ];

    let polygon_alt: Polygon<f64> = polygon![
        (x: 335113.5269645548, y: 773695.0227932289),
        (x: 335113.5269645948, y: 773695.0227932489),
        (x: 335113.5269646148, y: 773695.0227932689),
    ];

    // Transform
    let point_tf = coord::update_point_crs(point, &active_crs, &target_crs);
    let poly_tf = coord::update_poly_crs(&polygon, &active_crs, &target_crs);
    let poly_alt_tf = coord::update_poly_crs(&polygon_alt, &active_crs, &target_crs);

    // Closest point
    let closest: Point<f64> = dist::find_closest_point(&point_tf, &poly_tf);

    // Distance - methods require lat / long - must use EPSG_4326 vals
    let dist_h = dist::point_distance(&point_tf, &closest, &dist::DistanceMethod::Haversine);

    let dist_g = dist::point_distance(&point_tf, &closest, &dist::DistanceMethod::Geodesic);

    // Poly to poly dist
    let poly_dist_h =
        dist::polygon_distance(&poly_tf, &poly_alt_tf, &dist::DistanceMethod::Haversine);
    let poly_dist_g =
        dist::polygon_distance(&poly_tf, &poly_alt_tf, &dist::DistanceMethod::Geodesic);

    println!("Spatial Distance Analysis");
    println!("Transformed : {:?}", point_tf);
    println!("Transformed : {:?}", poly_tf);
    println!();

    println!("Closest Points : \n- {:?} \n- {:?}", point_tf, closest);
    println!();

    println!("Point to Point");
    println!("Haversine distance (km) : {:?}", (dist_h / 1000.0));
    println!("Geodesic distance (km) : {:?}", (dist_g / 1000.0));
    println!("Variance (%) : {:?}", 100.0 * ((dist_g - dist_h) / dist_g));
    println!();

    println!("Polygon to Polygon");
    println!("Haversine distance (km) : {:?}", (poly_dist_h / 1000.0));
    println!("Geodesic distance (km) : {:?}", (poly_dist_g / 1000.0));
    println!(
        "Variance (%) : {:?}",
        100.0 * ((poly_dist_g - poly_dist_h) / poly_dist_g)
    );
}
