//! 🌐️⛰️ GIS 3D: terrain-tile engine — Terrarium DEM decode, chunked heightfield meshing, and the
//! wasm-bindgen `TerrainSession` consumed by the React `World3dHost` terrain layer. Mirrors the
//! `framework_surface_tiled_map` crate's tile/session architecture, but produces renderable mesh buffers (for the
//! existing `World3d` instancing pipeline) instead of rasterized pixels.
//!
//! 🧬️ DKM doctrine classification (ticket `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`,
//! wave 2 exemplar): every field of [`TerrainSessionCore`] was measured against consumers before
//! authoring, not assumed. Verdict — **this module owns no authoritative (tier-a) state and
//! therefore no mutation triad applies.** `origin_lon`/`origin_lat`/`exaggeration` mirror
//! `TerrainDescriptorJson.project_origin`/`exaggeration`, an artifact snapshot owned by the gis
//! plugin (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/…`), re-set by the consumer once per frame
//! when ITS signature changes — the real edit, if ever authored, belongs on that snapshot, not
//! here. `elevation` is a content-addressed cache of externally-fetched DEM tile bytes, out of the
//! tier a-e table entirely (nothing here is computable from any snapshot this module owns — see
//! [`TerrainElevationTiles`]). Everything else — [`projection`], [`tiles`], [`build_terrain_tile_mesh`],
//! [`visible_tile_coords`] — is tier-(e) pure compute. See `📓️wave2-reports/terrain-report.md` for
//! the full per-field table, the empty-dispatch rationale, and the placement recommendation for the
//! sibling `paint`/`node-graph`/`tiled-map` lanes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region Projection
/// 🧭️ Standard Web Mercator slippy-map tile math (Terrarium tiles are tiled this way), kept
/// independent of the "local meters" world-space used for mesh vertices below — the tile grid
/// must match the DEM source's global XYZ scheme, while placed geometry stays small-valued.
pub mod projection {
    use std::f64::consts::PI;

    pub fn lonlat_to_tile_xy(lon: f64, lat: f64, z: u32) -> (f64, f64) {
        let lat_rad = lat.clamp(-85.051_128_78, 85.051_128_78).to_radians();
        let n = 2f64.powi(z as i32);
        let x = (lon + 180.0) / 360.0 * n;
        let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0 * n;
        (x, y)
    }

    pub fn tile_xy_to_lonlat(x: f64, y: f64, z: u32) -> (f64, f64) {
        let n = 2f64.powi(z as i32);
        let lon = x / n * 360.0 - 180.0;
        let lat_rad = (PI * (1.0 - 2.0 * y / n)).sinh().atan();
        (lon, lat_rad.to_degrees())
    }

    /// 📐️ Tangent-plane equirectangular approximation around a project origin — adequate for a
    /// single terrain viewport (a few tiles wide), and keeps mesh-vertex coordinates small
    /// (meters from the project origin) rather than global Web Mercator meters.
    pub fn lonlat_to_local_meters(lon: f64, lat: f64, origin_lon: f64, origin_lat: f64) -> (f64, f64) {
        const M_PER_DEG_LAT: f64 = 111_320.0;
        let m_per_deg_lon = M_PER_DEG_LAT * origin_lat.to_radians().cos();
        ((lon - origin_lon) * m_per_deg_lon, (lat - origin_lat) * M_PER_DEG_LAT)
    }

    pub fn local_meters_to_lonlat(x: f64, y: f64, origin_lon: f64, origin_lat: f64) -> (f64, f64) {
        const M_PER_DEG_LAT: f64 = 111_320.0;
        let m_per_deg_lon = M_PER_DEG_LAT * origin_lat.to_radians().cos();
        (origin_lon + x / m_per_deg_lon.max(1e-9), origin_lat + y / M_PER_DEG_LAT)
    }
}
//#endregion Projection

//#region Tiles
pub mod tiles {
    pub const TERRAIN_TILE_MIN_ZOOM: u32 = 6;
    pub const TERRAIN_TILE_MAX_ZOOM: u32 = 14;
    /// 🧮️ Fixed visible-tile radius around the camera target — bounded (at most 5x5 = 25 tiles),
    /// deliberately simple rather than frustum-exact; refine only if it proves too coarse.
    pub const TERRAIN_TILE_RADIUS: i64 = 2;

    pub fn tile_key(z: u32, x: u32, y: u32) -> String {
        format!("{z}/{x}/{y}")
    }

    pub async fn parse_tile_key(key: &str) -> Option<(u32, u32, u32)> {
        let mut parts = key.split('/');
        let z = parts.next()?.parse().ok()?;
        let x = parts.next()?.parse().ok()?;
        let y = parts.next()?.parse().ok()?;
        Some((z, x, y))
    }

    /// 🔎️ Picks a DEM zoom level from camera-to-target distance (meters): closer camera -> higher
    /// zoom (finer DEM resolution), halving the reference distance per zoom level.
    pub async fn pick_zoom(distance_m: f64) -> u32 {
        const REFERENCE_DISTANCE_AT_MAX_ZOOM: f64 = 400.0;
        if distance_m <= REFERENCE_DISTANCE_AT_MAX_ZOOM {
            return TERRAIN_TILE_MAX_ZOOM;
        }
        let levels_down = (distance_m / REFERENCE_DISTANCE_AT_MAX_ZOOM).log2().floor() as i64;
        let zoom = TERRAIN_TILE_MAX_ZOOM as i64 - levels_down;
        zoom.clamp(TERRAIN_TILE_MIN_ZOOM as i64, TERRAIN_TILE_MAX_ZOOM as i64) as u32
    }

    pub fn visible_tiles(center_lon: f64, center_lat: f64, zoom: u32) -> Vec<(u32, u32, u32)> {
        let (cx, cy) = super::projection::lonlat_to_tile_xy(center_lon, center_lat, zoom);
        let n = 2i64.pow(zoom);
        let cx = cx.floor() as i64;
        let cy = cy.floor() as i64;
        let mut out = Vec::new();
        for dy in -TERRAIN_TILE_RADIUS..=TERRAIN_TILE_RADIUS {
            for dx in -TERRAIN_TILE_RADIUS..=TERRAIN_TILE_RADIUS {
                let x = cx + dx;
                let y = cy + dy;
                if x < 0 || y < 0 || x >= n || y >= n {
                    continue;
                }
                out.push((zoom, x as u32, y as u32));
            }
        }
        out
    }
}
//#endregion Tiles

//#region ElevationDecode
const TERRARIUM_TILE_PX: u32 = 256;
/// 🕸️ Vertex-grid resolution per DEM tile mesh (33x33 verts = 32x32 quads) — coarse enough to
/// keep per-tile JSON payloads and triangle counts small, fine enough to read as terrain relief.
const TERRAIN_GRID_RESOLUTION: u32 = 33;

//#region ⚠️ Errors
/// ⚠️ Terrain DEM tile decode errors.
#[derive(Debug, thiserror::Error)]
pub enum FrameworkSurfaceTerrainError {
    #[error(transparent)]
    Image(#[from] image::ImageError),
}
//#endregion ⚠️ Errors

/// 🎨️ Elevation decoded from a Mapzen/AWS "Terrarium" RGB-encoded PNG: `R*256 + G + B/256 - 32768`.
fn decode_terrarium_png(bytes: &[u8]) -> Result<image::RgbaImage, FrameworkSurfaceTerrainError> {
    Ok(image::load_from_memory(bytes)?.to_rgba8())
}

async fn sample_elevation(image: &image::RgbaImage, px: f32, py: f32) -> f32 {
    let x = px.round().clamp(0.0, (image.width().saturating_sub(1)) as f32) as u32;
    let y = py.round().clamp(0.0, (image.height().saturating_sub(1)) as f32) as u32;
    let pixel = image.get_pixel(x, y);
    let (r, g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
    (r * 256.0 + g + b / 256.0) - 32768.0
}
//#endregion ElevationDecode

//#region TerrainTileMesh
struct DecodedElevationTile {
    z: u32,
    x: u32,
    y: u32,
    image: image::RgbaImage,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TerrainTileMeshJson {
    positions: Vec<f32>,
    normals: Vec<f32>,
    indices: Vec<u32>,
    uvs: Vec<f32>,
}

async fn build_terrain_tile_mesh(tile: &DecodedElevationTile, origin_lon: f64, origin_lat: f64, exaggeration: f64) -> TerrainTileMeshJson {
    let n = TERRAIN_GRID_RESOLUTION;
    let (min_lon, max_lat) = projection::tile_xy_to_lonlat(tile.x as f64, tile.y as f64, tile.z);
    let (max_lon, min_lat) = projection::tile_xy_to_lonlat((tile.x + 1) as f64, (tile.y + 1) as f64, tile.z);

    let mut heights = vec![0.0f32; (n * n) as usize];
    let mut min_elev = f32::INFINITY;
    let mut max_elev = f32::NEG_INFINITY;
    for row in 0..n {
        for col in 0..n {
            let px = (col as f32 / (n - 1) as f32) * (TERRARIUM_TILE_PX - 1) as f32;
            let py = (row as f32 / (n - 1) as f32) * (TERRARIUM_TILE_PX - 1) as f32;
            let elevation = sample_elevation(&tile.image, px, py);
            heights[(row * n + col) as usize] = elevation.await;
            min_elev = min_elev.min(elevation.await);
            max_elev = max_elev.max(elevation.await);
        }
    }
    if !min_elev.is_finite() {
        min_elev = 0.0;
    }
    if !max_elev.is_finite() || max_elev <= min_elev {
        max_elev = min_elev + 1.0;
    }

    let mut positions = Vec::with_capacity((n * n * 3) as usize);
    let mut uvs = Vec::with_capacity((n * n * 2) as usize);
    for row in 0..n {
        for col in 0..n {
            let u = col as f64 / (n - 1) as f64;
            let v = row as f64 / (n - 1) as f64;
            let lon = min_lon + (max_lon - min_lon) * u;
            let lat = max_lat + (min_lat - max_lat) * v;
            let (local_x, local_y) = projection::lonlat_to_local_meters(lon, lat, origin_lon, origin_lat);
            let elevation = heights[(row * n + col) as usize] as f64 * exaggeration;
            positions.push(local_x as f32);
            positions.push(local_y as f32);
            positions.push(elevation as f32);
            // Vertical hypsometric-ramp texture: u is unused (constant), v is normalized elevation.
            let normalized_elevation = ((heights[(row * n + col) as usize] - min_elev) / (max_elev - min_elev)).clamp(0.0, 1.0);
            uvs.push(0.5);
            uvs.push(normalized_elevation);
        }
    }

    let cell_size_x = (positions[3] - positions[0]).abs().max(1e-3);
    let mut normals = vec![0.0f32; positions.len()];
    for row in 0..n {
        for col in 0..n {
            let left = heights[(row * n + col.saturating_sub(1)) as usize];
            let right = heights[(row * n + (col + 1).min(n - 1)) as usize];
            let up = heights[(row.saturating_sub(1) * n + col) as usize];
            let down = heights[((row + 1).min(n - 1) * n + col) as usize];
            let dzdx = (right - left) as f64 * exaggeration / (2.0 * cell_size_x as f64);
            let dzdy = (down - up) as f64 * exaggeration / (2.0 * cell_size_x as f64);
            let normal = normalize3(-dzdx, -dzdy, 1.0);
            let idx = ((row * n + col) * 3) as usize;
            normals[idx] = normal.0 as f32;
            normals[idx + 1] = normal.1 as f32;
            normals[idx + 2] = normal.2 as f32;
        }
    }

    let mut indices = Vec::with_capacity(((n - 1) * (n - 1) * 6) as usize);
    for row in 0..n - 1 {
        for col in 0..n - 1 {
            let i0 = row * n + col;
            let i1 = row * n + col + 1;
            let i2 = (row + 1) * n + col;
            let i3 = (row + 1) * n + col + 1;
            indices.extend_from_slice(&[i0, i2, i1, i1, i2, i3]);
        }
    }

    TerrainTileMeshJson { positions, normals, indices, uvs }
}

fn normalize3(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let length = (x * x + y * y + z * z).sqrt().max(1e-9);
    (x / length, y / length, z / length)
}
//#endregion TerrainTileMesh

//#region VisibleTileQuery
#[derive(Deserialize)]
struct CameraRecord {
    position: Option<[f64; 3]>,
    target: Option<[f64; 3]>,
}

#[derive(Serialize)]
struct VisibleTileRow {
    z: u32,
    x: u32,
    y: u32,
    key: String,
}

/// 🔭️ Tier-(e) pure compute: which DEM tiles a camera can see, given a project origin. Neither
/// argument is state this module owns — `camera` is per-frame ephemeral view state, `origin_lon`/
/// `origin_lat` mirror a snapshot field owned by the gis plugin's `TerrainDescriptorJson` artifact —
/// so this is deliberately a free function rather than an `InferredField`: there is no snapshot
/// here for a `DepHash` to key off, and the render loop already recomputes it every frame (see
/// `♾️infinite/🌍️world/🦀️component.rs`'s `sync_terrain_state`), which a dep-hash cache would not help.
async fn visible_tile_coords(camera: &CameraRecord, origin_lon: f64, origin_lat: f64) -> Vec<(u32, u32, u32)> {
    let target = camera.target.unwrap_or([0.0, 0.0, 0.0]);
    let position = camera.position.unwrap_or([0.0, 0.0, 100.0]);
    let dx = position[0] - target[0];
    let dy = position[1] - target[1];
    let dz = position[2] - target[2];
    let distance = (dx * dx + dy * dy + dz * dz).sqrt().max(1.0);
    let zoom = tiles::pick_zoom(distance);
    let (center_lon, center_lat) = projection::local_meters_to_lonlat(target[0], target[1], origin_lon, origin_lat);
    tiles::visible_tiles(center_lon, center_lat, zoom.await)
}
//#endregion VisibleTileQuery

//#region TerrainSession
/// 🗃️ Cache of decoded DEM tile images, keyed by tile coordinate. NOT tier-(d): a tier-(d)
/// `EngineRep` must be wholly rebuildable from a snapshot (`EngineRep::build(&P)`), and this cache
/// cannot be — its contents arrive as externally-fetched PNG bytes (`fetch_pending_terrain_tiles` in
/// `♾️infinite/🦀️component.rs`) that no artifact snapshot captures. It is analogous to
/// `World3dState.meshes`/`pending_glb_urls` in the same consumer: a per-viewer materialized cache of
/// an external resource, outside the tier a-e table entirely. Open question for the coordinator/W1
/// owner (see `📓️wave2-reports/terrain-report.md`): whether decode+mesh-build should instead route
/// through the frozen host `EngineCache` (`💻️os/🔨️modules/⚙️engine`) — its docstring scopes it to
/// "the wasm guest↔host boundary", and whether this consumer crosses that boundary is unconfirmed.
#[derive(Default)]
struct TerrainElevationTiles {
    by_key: HashMap<String, DecodedElevationTile>,
}

/// 🖥️ The browser-facing session: uploads decoded elevation tiles, reports which DEM tiles are
/// currently visible for a given camera, and produces per-tile mesh buffers on demand. Mirrors
/// `framework_surface_tiled_map`'s `MapSession`, but yields mesh JSON rather than driving a canvas itself — actual
/// rendering happens via the existing `World3d`/three.js instancing pipeline in React.
///
/// 🧬️ Despite the "session + setters" shape, this struct owns NO tier-(a) authoritative state — see
/// the module docstring and `📓️wave2-reports/terrain-report.md` for the full classification. No
/// `🧬️mutations` triad exists for it: there is nothing here for one to edit.
pub struct TerrainSessionCore {
    origin_lon: f64,
    origin_lat: f64,
    exaggeration: f64,
    elevation: TerrainElevationTiles,
}

impl Default for TerrainSessionCore {
    fn default() -> Self {
        Self { origin_lon: 0.0, origin_lat: 0.0, exaggeration: 1.0, elevation: TerrainElevationTiles::default() }
    }
}

impl TerrainSessionCore {
    /// 🪞 Mirrors `TerrainDescriptorJson.project_origin` (gis-owned artifact field, tier-a there —
    /// see the module docstring) into this render-support cache. Kept as a setter rather than
    /// threading `origin_lon`/`origin_lat` through every call for API compatibility with the current
    /// `♾️infinite/🌍️world` consumer, which this file does not own; see the report's recommended
    /// target shape (drop the setter, pass origin as a parameter) as a `sharedFileRequests` patch.
    pub async fn set_project_origin(&mut self, lon: f64, lat: f64) {
        self.origin_lon = lon;
        self.origin_lat = lat;
    }

    /// 🪞 Mirrors `TerrainDescriptorJson.exaggeration` (gis-owned, tier-a there) — same rationale as
    /// [`Self::set_project_origin`].
    pub async fn set_exaggeration(&mut self, exaggeration: f64) {
        self.exaggeration = exaggeration.max(0.0);
    }

    /// 🔭️ JSON-wrapping shim over the tier-(e) [`visible_tile_coords`] — kept `&self` for API
    /// compatibility; the actual query is pure and argument-driven, not a method of this struct's
    /// mutable state.
    pub async fn visible_terrain_tiles_json(&self, camera_json: &str) -> String {
        let camera: CameraRecord = serde_json::from_str(camera_json).unwrap_or(CameraRecord { position: None, target: None });
        let rows: Vec<VisibleTileRow> = visible_tile_coords(&camera, self.origin_lon, self.origin_lat)
            .into_iter().await.await
            .map(|(z, x, y)| VisibleTileRow { z, x, y, key: tiles::tile_key(z, x, y) })
            .collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".to_string())
    }

    /// 📥️ Decodes and stores one externally-fetched DEM tile — see [`TerrainElevationTiles`] for
    /// why this is an out-of-doctrine resource cache, not tier-(d)/(c) state.
    pub async fn upload_elevation_tile(&mut self, z: u32, x: u32, y: u32, bytes: &[u8]) -> bool {
        match decode_terrarium_png(bytes) {
            Ok(image) => {
                self.elevation.by_key.insert(tiles::tile_key(z, x, y), DecodedElevationTile { z, x, y, image });
                true
            }
            Err(_) => false,
        }
    }

    /// 🗑️ Drops one cached tile — a cache eviction, not a `delete-*` mutation: nothing here is
    /// authoritative, so there is nothing to capture for an inverse.
    pub async fn evict_terrain_tile(&mut self, z: u32, x: u32, y: u32) {
        self.elevation.by_key.remove(&tiles::tile_key(z, x, y));
    }

    /// 🕸️ Tier-(e) mesh build ([`build_terrain_tile_mesh`]) over a cached decoded tile, JSON-wrapped.
    pub async fn terrain_tile_mesh_json(&self, z: u32, x: u32, y: u32) -> String {
        match self.elevation.by_key.get(&tiles::tile_key(z, x, y)) {
            Some(tile) => {
                let mesh = build_terrain_tile_mesh(tile, self.origin_lon, self.origin_lat, self.exaggeration);
                serde_json::to_string(&mesh).unwrap_or_else(|_| "null".to_string())
            }
            None => "null".to_string(),
        }
    }
}
//#endregion TerrainSession

//#region WasmBindings
#[cfg(all(target_arch = "wasm32", feature = "session-bindgen"))]
mod wasm_bridge {
    use super::TerrainSessionCore;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct TerrainSession {
        core: TerrainSessionCore,
    }

    #[wasm_bindgen]
    impl TerrainSession {
        #[wasm_bindgen(constructor)]
        pub async fn new() -> TerrainSession {
            TerrainSession { core: TerrainSessionCore::default() }
        }

        pub async fn set_project_origin(&mut self, lon: f64, lat: f64) {
            self.core.set_project_origin(lon, lat);
        }

        pub async fn set_exaggeration(&mut self, exaggeration: f64) {
            self.core.set_exaggeration(exaggeration);
        }

        pub async fn visible_terrain_tiles_json(&self, camera_json: &str) -> String {
            self.core.visible_terrain_tiles_json(camera_json)
        }

        pub async fn upload_elevation_tile(&mut self, z: u32, x: u32, y: u32, bytes: &[u8]) -> bool {
            self.core.upload_elevation_tile(z, x, y, bytes)
        }

        pub async fn evict_terrain_tile(&mut self, z: u32, x: u32, y: u32) {
            self.core.evict_terrain_tile(z, x, y);
        }

        pub async fn terrain_tile_mesh_json(&self, z: u32, x: u32, y: u32) -> String {
            self.core.terrain_tile_mesh_json(z, x, y)
        }
    }

    impl Default for TerrainSession {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "session-bindgen"))]
pub use wasm_bridge::TerrainSession;
//#endregion WasmBindings

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn tile_key_roundtrip() {
        assert_eq!(tiles::tile_key(10, 3, 7), "10/3/7");
        assert_eq!(tiles::parse_tile_key("10/3/7"), Some((10, 3, 7)));
        assert_eq!(tiles::parse_tile_key("garbage"), None);
    }

    #[test]
    async fn lonlat_tile_xy_roundtrip_is_stable() {
        let (x, y) = projection::lonlat_to_tile_xy(9.7382, 52.3759, 12);
        let (lon, lat) = projection::tile_xy_to_lonlat(x.floor(), y.floor(), 12);
        // Round-tripping the tile's top-left corner should land close to (not equal to, since we
        // floor to the tile origin) the original point.
        assert!((lon - 9.7382).abs() < 1.0);
        assert!((lat - 52.3759).abs() < 1.0);
    }

    #[test]
    async fn local_meters_roundtrip() {
        let origin_lon = 9.7382;
        let origin_lat = 52.3759;
        let (x, y) = projection::lonlat_to_local_meters(9.75, 52.38, origin_lon, origin_lat);
        let (lon, lat) = projection::local_meters_to_lonlat(x, y, origin_lon, origin_lat);
        assert!((lon - 9.75).abs() < 1e-6);
        assert!((lat - 52.38).abs() < 1e-6);
    }

    #[test]
    async fn pick_zoom_clamps_to_bounds() {
        assert_eq!(tiles::pick_zoom(1.0), tiles::TERRAIN_TILE_MAX_ZOOM);
        assert_eq!(tiles::pick_zoom(1_000_000_000.0), tiles::TERRAIN_TILE_MIN_ZOOM);
    }

    #[test]
    async fn visible_tiles_returns_bounded_grid_around_center() {
        let rows = tiles::visible_tiles(9.7382, 52.3759, 12);
        assert!(!rows.is_empty());
        assert!(rows.len() <= 25);
    }

    fn solid_terrarium_png(elevation: f32) -> Vec<u8> {
        let value = (elevation + 32768.0).round() as i64;
        let r = ((value >> 8) & 0xff) as u8;
        let remainder = value - ((r as i64) << 8);
        let g = remainder.clamp(0, 255) as u8;
        let mut image = image::RgbaImage::new(TERRARIUM_TILE_PX, TERRARIUM_TILE_PX);
        for pixel in image.pixels_mut() {
            *pixel = image::Rgba([r, g, 0, 255]);
        }
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
        bytes
    }

    #[test]
    async fn upload_and_mesh_a_flat_tile_produces_grid_geometry() {
        let mut session = TerrainSessionCore::default();
        session.set_project_origin(9.7382, 52.3759);
        let bytes = solid_terrarium_png(123.0);
        assert!(session.upload_elevation_tile(12, 2000, 1300, &bytes));
        let mesh_json = session.terrain_tile_mesh_json(12, 2000, 1300);
        assert_ne!(mesh_json, "null");
        let mesh: TerrainTileMeshJson = serde_json::from_str(&mesh_json).expect("valid mesh json");
        let n = TERRAIN_GRID_RESOLUTION as usize;
        assert_eq!(mesh.positions.len(), n * n * 3);
        assert_eq!(mesh.normals.len(), n * n * 3);
        assert_eq!(mesh.uvs.len(), n * n * 2);
        assert_eq!(mesh.indices.len(), (n - 1) * (n - 1) * 6);
        // A uniform-elevation tile should have (near-)zero elevation variance across the grid.
        let first_elevation = mesh.positions[2];
        for chunk in mesh.positions.chunks_exact(3) {
            assert!((chunk[2] - first_elevation).abs() < 5.0, "expected uniform elevation, got {} vs {}", chunk[2], first_elevation);
        }
    }

    #[test]
    async fn missing_tile_mesh_is_null() {
        let session = TerrainSessionCore::default();
        assert_eq!(session.terrain_tile_mesh_json(12, 0, 0), "null");
    }

    #[test]
    async fn pick_zoom_halves_reference_distance_per_level() {
        assert_eq!(tiles::pick_zoom(400.0), tiles::TERRAIN_TILE_MAX_ZOOM);
        assert_eq!(tiles::pick_zoom(800.0), tiles::TERRAIN_TILE_MAX_ZOOM - 1);
        assert_eq!(tiles::pick_zoom(1600.0), tiles::TERRAIN_TILE_MAX_ZOOM - 2);
    }

    #[test]
    async fn visible_tiles_clamps_at_world_edge() {
        let rows = tiles::visible_tiles(-179.9, 0.1, 0);
        assert_eq!(rows, vec![(0, 0, 0)]);
    }

    #[test]
    async fn decode_terrarium_png_invalid_bytes_returns_error() {
        let error = decode_terrarium_png(b"not a real png").expect_err("garbage bytes should not decode");
        assert!(!error.to_string().is_empty());
    }

    #[test]
    async fn sample_elevation_clamps_out_of_bounds_coordinates() {
        let mut image = image::RgbaImage::new(2, 2);
        image.put_pixel(1, 1, image::Rgba([128, 0, 0, 255]));
        let inside = sample_elevation(&image, 1.0, 1.0);
        let clamped_high = sample_elevation(&image, 999.0, 999.0);
        let clamped_low = sample_elevation(&image, -50.0, -50.0);
        assert_eq!(inside, clamped_high);
        assert_ne!(inside, clamped_low);
    }

    #[test]
    async fn normalize3_degenerate_vector_does_not_panic_or_nan() {
        let (x, y, z) = normalize3(0.0, 0.0, 0.0);
        assert!(x.is_finite() && y.is_finite() && z.is_finite());
    }

    #[test]
    async fn upload_elevation_tile_invalid_bytes_returns_false_and_no_mesh() {
        let mut session = TerrainSessionCore::default();
        assert!(!session.upload_elevation_tile(5, 1, 1, b"garbage"));
        assert_eq!(session.terrain_tile_mesh_json(5, 1, 1), "null");
    }

    #[test]
    async fn evict_terrain_tile_removes_previously_uploaded_tile() {
        let mut session = TerrainSessionCore::default();
        let bytes = solid_terrarium_png(50.0);
        assert!(session.upload_elevation_tile(8, 10, 10, &bytes));
        assert_ne!(session.terrain_tile_mesh_json(8, 10, 10), "null");
        session.evict_terrain_tile(8, 10, 10);
        assert_eq!(session.terrain_tile_mesh_json(8, 10, 10), "null");
    }

    #[test]
    async fn set_exaggeration_clamps_negative_to_zero() {
        let mut session = TerrainSessionCore::default();
        session.set_exaggeration(-3.0);
        let bytes = solid_terrarium_png(200.0);
        session.upload_elevation_tile(9, 5, 5, &bytes);
        let mesh_json = session.terrain_tile_mesh_json(9, 5, 5);
        let mesh: TerrainTileMeshJson = serde_json::from_str(&mesh_json).expect("valid mesh json");
        for chunk in mesh.positions.chunks_exact(3) {
            assert_eq!(chunk[2], 0.0);
        }
    }

    #[test]
    async fn visible_terrain_tiles_json_falls_back_to_defaults_on_invalid_camera_json() {
        let session = TerrainSessionCore::default();
        let json = session.visible_terrain_tiles_json("not json");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&json).expect("valid array");
        assert!(!rows.is_empty());
    }

    #[test]
    async fn visible_terrain_tiles_json_reflects_camera_distance_in_zoom() {
        let session = TerrainSessionCore::default();
        let close_json = session.visible_terrain_tiles_json(r#"{"position":[0,0,100],"target":[0,0,0]}"#);
        let far_json = session.visible_terrain_tiles_json(r#"{"position":[0,0,1000000],"target":[0,0,0]}"#);
        let close_rows: Vec<serde_json::Value> = serde_json::from_str(&close_json).expect("valid array");
        let far_rows: Vec<serde_json::Value> = serde_json::from_str(&far_json).expect("valid array");
        assert!(close_rows[0]["z"].as_u64().unwrap() > far_rows[0]["z"].as_u64().unwrap());
    }

    async fn gradient_terrarium_png() -> Vec<u8> {
        let mut image = image::RgbaImage::new(TERRARIUM_TILE_PX, TERRARIUM_TILE_PX);
        for (px, _py, pixel) in image.enumerate_pixels_mut() {
            let value = (500.0 + px as f32 * 10.0 + 32768.0).round() as i64;
            let r = ((value >> 8) & 0xff) as u8;
            let g = (value - ((r as i64) << 8)).clamp(0, 255) as u8;
            *pixel = image::Rgba([r, g, 0, 255]);
        }
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
        bytes
    }

    #[test]
    async fn sloped_tile_mesh_has_varying_elevation_and_nontrivial_normals() {
        let mut session = TerrainSessionCore::default();
        session.set_project_origin(9.7382, 52.3759);
        let bytes = gradient_terrarium_png();
        assert!(session.upload_elevation_tile(12, 2000, 1300, &bytes));
        let mesh_json = session.terrain_tile_mesh_json(12, 2000, 1300);
        let mesh: TerrainTileMeshJson = serde_json::from_str(&mesh_json).expect("valid mesh json");
        let min_z = mesh.positions.iter().skip(2).step_by(3).cloned().fold(f32::INFINITY, f32::min);
        let max_z = mesh.positions.iter().skip(2).step_by(3).cloned().fold(f32::NEG_INFINITY, f32::max);
        assert!(max_z - min_z > 1.0, "expected sloped tile to have elevation spread, got {min_z}..{max_z}");
        let has_tilted_normal = mesh.normals.chunks_exact(3).any(|n| n[0].abs() > 1e-3);
        assert!(has_tilted_normal, "expected at least one non-vertical normal on a sloped tile");
    }

    #[test]
    async fn visible_tile_coords_is_deterministic_for_identical_input() {
        let camera = CameraRecord { position: Some([0.0, 0.0, 500.0]), target: Some([10.0, 10.0, 0.0]) };
        let first = visible_tile_coords(&camera, 9.7382, 52.3759);
        let second = visible_tile_coords(&camera, 9.7382, 52.3759);
        assert_eq!(first, second, "a tier-(e) pure query must return identical output for identical input");
    }

    #[test]
    async fn terrain_tile_mesh_json_is_deterministic_for_the_same_cached_tile() {
        let mut session = TerrainSessionCore::default();
        session.set_project_origin(9.7382, 52.3759);
        let bytes = gradient_terrarium_png();
        assert!(session.upload_elevation_tile(12, 2000, 1300, &bytes));
        let first = session.terrain_tile_mesh_json(12, 2000, 1300);
        let second = session.terrain_tile_mesh_json(12, 2000, 1300);
        assert_eq!(first, second, "mesh build over an unchanged cached tile must be byte-identical across calls");
    }
}
//#endregion Tests
