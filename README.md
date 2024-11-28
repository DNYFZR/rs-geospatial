<h1 align="center"> Rust Geospatial </h1>

Looking around the ecosystem & hoping to learn a thing or two for now...

## Crates

- geo : core structures - points, polygons & pals - really useful
- proj4rs : crs transformations - way less overhead than the full proj crate
- crs-definitions : self explanitory - works well with proj4rs to save having to code in all the crs strings, just call the def from here.
- zip : handles zip archives
- geozero : handles wkb / wkt transformations
- reqwest : for getting geospatial files from the web
- rusqlite : for handling gpkg formatted SQLite DBs

## Notes

Current functionality :

- convert crs for point or polygon

- find closest point in polygon to other point

- find distance between two points, two polygons, or point-to-polygon
  - using Haversine or Geodesic measurements

- extract geopackage datasets from web hosted zip archives and process into memory for analysis
