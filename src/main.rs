// Geospatial Modelling
mod crs;

mod distance;

mod zipfile;

mod run;
use run::rs_geo;

fn main() {
    // Run configured setup
    rs_geo();
}
