@capability-web-mercator-tile-selection
@oracle-mercantile
@comparison-floating-point-v1
Feature: Web-Mercator projection and slippy-tile selection agree with a third-party reference
  `projection::lonlat_to_world` / `world_to_lonlat` implement the published Web-Mercator (EPSG:3857)
  forward/inverse formulas normalized to `WORLD_HALF = 1`; `tiles::visible_tiles` and the tile
  numbering it is built on implement the published OSM/Mapbox XYZ slippy-tile scheme. Both are
  documented, standardized transforms with a mature third-party implementation, so `mercantile`
  (pure Python, zero runtime dependencies) is a genuine oracle for them: it computes EPSG:3857 meters
  and XYZ tile numbers independently of this repository, from the same published spec.

  The full vector set (projection points, tile numbers, tile bounding boxes) lives in the sibling
  `🧫️fixtures/🔣️vectors.json`, generated once from `mercantile` and frozen. Both the `🐍️.py` oracle
  adapter (re-derives every value from `mercantile` and asserts it still matches the frozen fixture)
  and the crate's own `tests/🗺️tiled_map_mercator_oracle.rs` integration test (asserts this
  repository's Rust implementation matches the same frozen fixture) read that one file, so oracle and
  subject are checked against the identical inputs.

  `lodBands` in the fixture are NOT third-party-checked: `GIS_MAP_LOD_MAX_SPAN_DEG` /
  `GIS_MAP_LOD_TILE_Z` are a repository-owned band scheme with no published external standard, so
  those vectors are specification vectors pinning `🦀️component.rs:84,90`, not an oracle claim.

  @id-lonlat-world-round-trip
  @level-fundamental
  @mode-differential
  Scenario: Forward projection matches the third-party EPSG:3857 formula
    Given the lon/lat points in the fixture's `projection` array
      | id               | lon  | lat  |
      | equator-prime-meridian | 0.0  | 0.0  |
      | zurich            | 8.5  | 47.4 |
      | san-francisco     | -122.4194 | 37.7749 |
      | near-antimeridian-north | 179.9 | 85.0 |
    Then `mercantile.xy(lon, lat)` normalized by `sphereRadius * pi` and `projection::lonlat_to_world(lon, lat)` agree within floating-point tolerance on both worldX and worldY

  @id-tile-numbering
  @level-fundamental
  @mode-differential
  Scenario: XYZ tile numbers match the third-party slippy-tile scheme
    Given the (lon, lat, z) triples in the fixture's `tileNumbering` array
      | id            | lon    | lat    | z  |
      | zurich-z10    | 8.5    | 47.4   | 10 |
      | dateline-west-z6 | -179.9 | 10.0 | 6  |
      | near-north-pole-z5 | 10.0 | 84.9 | 5 |
    Then `mercantile.tile(lon, lat, z)` and this repository's tile-selection at that same world point and zoom agree on (x, y)

  @id-tile-bounds
  @level-fundamental
  @mode-differential
  Scenario: Tile bounding boxes match the third-party reference
    Given the (z, x, y) triples in the fixture's `tileBounds` array
    Then `mercantile.bounds(x, y, z)` and `projection::world_to_lonlat` applied to `projection::tile_world_rect(z, x, y)`'s corners agree within floating-point tolerance

  @id-lod-band-selection
  @level-fundamental
  @mode-conformance
  Scenario: LOD band and tile-z selection match the frozen specification vectors
    Given a viewport longitude span in degrees from the fixture's `lodBands` array
    Then the LOD band index resolved from `GIS_MAP_LOD_MAX_SPAN_DEG` and the tile z resolved from `GIS_MAP_LOD_TILE_Z` match the fixture's `lodIndex` and `tileZ`

  @id-cursor-anchored-zoom-invariant
  @level-fundamental
  @mode-property
  Scenario: Cursor-anchored zoom keeps the world point under the cursor fixed
    Given a camera, a viewport and a screen point strictly inside the viewport
    When the camera zooms in at that screen point
    Then the world point under that same screen point is unchanged within floating-point tolerance

  @id-pan-round-trip-invariant
  @level-fundamental
  @mode-property
  Scenario: Panning by (dx, dy) then by (-dx, -dy) returns the exact original camera
    Given a camera positioned away from the world-bounds clamp
    When the camera pans by (dx, dy) screen pixels and then by (-dx, -dy) screen pixels
    Then the camera position equals the original position within floating-point tolerance

  @id-zoom-round-trip-invariant
  @level-fundamental
  @mode-property
  Scenario: Zooming in then out by the same factor returns the original zoom
    Given a camera at a known zoom level
    When the camera zooms in one wheel step and then zooms out one wheel step
    Then the camera zoom equals the original zoom within floating-point tolerance

  @id-visible-tile-count-bound
  @level-fundamental
  @mode-property
  Scenario: The visible tile count never exceeds the request budget
    Given any camera and viewport combination
    Then the number of tiles `pick_raster_tile_zoom` and `pick_vector_tile_zoom` select for is at most `MAX_VISIBLE_TILE_REQUESTS`
