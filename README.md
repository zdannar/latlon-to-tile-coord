# latlon-to-tile-coord

This is just a note keeping experiement for converting latitude/longitude
(EPSG 4326) to the tile coords.

While working with the [mvt](https://crates.io/crates/mvt) crate, I had to
reverse engineer Paul Mach's [Orb](https://github.com/paulmach/orb) library
conversion to determine the valid tile coordinates.  I was unable to find a
crate that produced this conversion properly.

### Update:

Using a different search term in crates.io, I have now found a crate that
provides this functionality.
[slippy_map_tilenames](https://crates.io/crates/slippy_map_tilenames).

### TODO: 
_Make this into a simple crate_
