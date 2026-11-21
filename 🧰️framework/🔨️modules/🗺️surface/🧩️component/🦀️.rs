//! 🗺️ The browser module of `semio-framework-surface`: links the facade, whose producer crates carry the `#[wasm_bindgen]` exports, and owns
//! the terrain session export, which the terrain crate cannot carry because `semio-os-infinite` links the same terrain crate and would duplicate the export.

extern crate semio_framework_surface as _;

/// 🏔️ wasm-bindgen `TerrainSession` over the target-neutral `TerrainSessionCore`, consumed by the React `World3dHost` terrain layer.
/// Browser-only: `target_arch = "wasm32"` is also true for the `wasm32-wasip2` component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod terrain_session {
    use semio_framework_surface::terrain::TerrainSessionCore;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct TerrainSession {
        core: TerrainSessionCore,
    }

    #[wasm_bindgen]
    impl TerrainSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> TerrainSession {
            TerrainSession { core: TerrainSessionCore::default() }
        }

        pub fn set_project_origin(&mut self, lon: f64, lat: f64) {
            self.core.set_project_origin(lon, lat);
        }

        pub fn set_exaggeration(&mut self, exaggeration: f64) {
            self.core.set_exaggeration(exaggeration);
        }

        pub fn set_elevation_grid_json(&mut self, json: &str) -> bool {
            self.core.set_elevation_grid_json(json)
        }

        #[wasm_bindgen(js_name = pickElevationGridJson)]
        pub fn pick_elevation_grid_json(&self, ray_json: &str) -> String {
            self.core.pick_elevation_grid_json(ray_json)
        }

        pub fn visible_terrain_tiles_json(&self, camera_json: &str) -> String {
            self.core.visible_terrain_tiles_json(camera_json)
        }

        pub fn upload_elevation_tile(&mut self, z: u32, x: u32, y: u32, bytes: &[u8]) -> bool {
            self.core.upload_elevation_tile(z, x, y, bytes)
        }

        pub fn evict_terrain_tile(&mut self, z: u32, x: u32, y: u32) {
            self.core.evict_terrain_tile(z, x, y);
        }

        pub fn terrain_tile_mesh_json(&self, z: u32, x: u32, y: u32) -> String {
            self.core.terrain_tile_mesh_json(z, x, y)
        }

        pub fn terrain_overlay_tile_json(&self, z: u32, x: u32, y: u32, overlays_json: &str) -> String {
            self.core.terrain_overlay_tile_json(z, x, y, overlays_json)
        }

        pub fn terrain_origin_elevation(&self, z: u32, x: u32, y: u32) -> Option<f32> {
            self.core.terrain_origin_elevation(z, x, y)
        }
    }

    impl Default for TerrainSession {
        fn default() -> Self {
            Self::new()
        }
    }
}
