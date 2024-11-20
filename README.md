<h1 align="center"> Rust Geospatial </h1>

Looking around the ecosystem & hoping to learn a thing or two for now...

## Crates

- geo : core structures - points, polygons & pals - really useful
- proj4rs : crs transformations - way less overhead than the full proj crate
- crs-definitions : self explanitory - works well with proj4rs to save having to code in all the crs strings, just call the def from here.

## Notes

Current functionality :

- convert crs for point or polygon

- find closest point in polygon to other point

- find distance between two points
  - using Haversine or Geodesic measurements
