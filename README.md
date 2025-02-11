<h2 align="center"> rs-geospatial </h2>

Current functionality :

- convert crs for point or polygon

- find closest point in polygon to other point

- find distance between two points, two polygons, or point-to-polygon
  - using Haversine or Geodesic measurements

- extract geopackage datasets from web hosted zip archives and process into memory for analysis

### Crates

- geo : core co-ord / point / polygon... structures
- proj4rs : crs transformations - less overhead than the full proj crate
- crs-definitions : works well with proj4rs & saves having to code in all the crs strings, just call the def from here.
- zip : handles zip archives
- geozero : handles wkb / wkt transformations
- reqwest : for getting geospatial files from the web
- rusqlite : for handling gpkg formatted SQLite DBs
