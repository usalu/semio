//! 🧱️ World3d scene resources and asset io: pools, chunking, LOD grid, environment, terrain, placeholder meshes and asset requests.
//!
//! Crate `semio-os-infinite-world-scene`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_os_infinite_world_model::{World3dBuildContext, World3dMeshAsset, World3dMeshAssetRejected, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_DYNAMIC_MESH_CAPACITY, WorldDynamicFault, WorldDynamicRejected, WorldEnvironmentRecord, WorldInteractionState, WorldMeshRegistry, WorldReferenceRecord, WorldSessionState};
use semio_os_infinite_world_components::{placeholder_normalize3, placeholder_scale3, TerrainTileMeshPayload, WORLD_REFERENCE_SOURCE_PIXELS, WorldBrushMeshRun, WorldCameraState, WorldMeshBuffers, WorldPlaceholderOwner, WorldSceneState, WorldSelectionState};
use semio_ui_declaration::layout::ActionDescriptor;
use ui_scene::math::{Camera3d, camera_grid_fade_distance, Instance3d, LineVertex3d, lod_from_camera_distance, lod_grid_fade_alpha, lod_grid_step_world, lod_orbit_distance_for_camera, Mat4, mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_seal, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, SceneColorSource3d, SceneDraw3d, Vec3};
use semio_ui_theme::geometry::Rect;
use semio_ui_theme::{Rgba, Theme};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use ui_scene::math::{directional_shadow_view_projection, ICON_SHADOW_MAP_SIZE, Mat4Math, SceneMaterial3d, SceneMaterialDraw3d, SceneShadow3d, SceneShadowRole3d, WORLD_SHADOW_MAP_SIZE};
use std::sync::{LazyLock, Mutex};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_surface_terrain::{TerrainElevationGrid, TerrainSessionCore};
pub use semio_framework_surface_terrain::TerrainBrushAction;
use semio_framework_surface_terrain::overlay::{TerrainOverlays, TerrainOverlayTile};
use serde_json::json;
#[cfg(any(test, feature = "testkit"))] use ui_scene::math::pick_closest_mesh_url;
use ui_styling::metrics::tool_run;

/// 🌐️ Terrain source and geographic overlays from `World3dScene.terrainJson`.
#[derive(semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
struct WorldTerrainStyle {
    #[value(default)]
    tile_url_template: String,
    elevation_grid: Option<TerrainElevationGrid>,
    #[value(default)]
    project_origin_lon: f64,
    #[value(default)]
    project_origin_lat: f64,
    #[value(default = "default_terrain_exaggeration")]
    exaggeration: f64,
    #[value(default = "default_terrain_min_zoom")]
    min_zoom: u32,
    #[value(default = "default_terrain_max_zoom")]
    max_zoom: u32,
    #[value(default)]
    overlays: TerrainOverlays,
    attribution: Option<TerrainAttribution>,
}

#[derive(semio_framework_value::FromValue)]
struct TerrainAttribution {
    label: String,
}

#[derive(semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
struct TerrainInteraction {
    terrain_brush: Option<TerrainBrushAction>,
}

#[derive(Deserialize)]
struct TerrainCameraFrame {
    #[serde(rename = "terrainFrame")]
    terrain_frame: Option<[f64; 3]>,
}

impl WorldTerrainStyle {
    fn signature(&self) -> (&str, f64, f64, f64, Option<&TerrainElevationGrid>) {
        (&self.tile_url_template, self.project_origin_lon, self.project_origin_lat, self.exaggeration, self.elevation_grid.as_ref())
    }

    fn has_overlays(&self) -> bool {
        !(self.overlays.positions.is_empty() && self.overlays.routes.is_empty() && self.overlays.regions.is_empty())
    }
}

fn default_terrain_exaggeration() -> f64 { 1.0 }
fn default_terrain_min_zoom() -> u32 { semio_framework_surface_terrain::tiles::TERRAIN_TILE_MIN_ZOOM }
fn default_terrain_max_zoom() -> u32 { semio_framework_surface_terrain::tiles::TERRAIN_TILE_MAX_ZOOM }

/// 🧩️ World3d terrain component: terrain style, tiles, session and build.
pub struct WorldTerrainState {

    terrain_style: Option<WorldTerrainStyle>,

    terrain_source_json: Option<String>,

    terrain_overlay_tiles: HashMap<(u32, u32, u32), TerrainOverlayTile>,

    terrain_camera_frame: Option<(u64, bool)>,

    terrain_interaction_digest: Option<u64>,

    terrain_brush: Option<TerrainBrushAction>,

    terrain_applied_signature: Option<(String, f64, f64, f64, Option<TerrainElevationGrid>)>,

    pub terrain_session: TerrainSessionCore,

    pub terrain_visible_tiles: HashSet<(u32, u32, u32)>,

    pub terrain_built_tiles: HashSet<(u32, u32, u32)>,

    pub pending_terrain_tile_urls: HashMap<String, (u32, u32, u32)>,

    pub terrain_tile_misses: HashSet<(u32, u32, u32)>,

    pub terrain_build: Option<WorldTerrainMeshCursor>,

    pub terrain_pending: Option<((u32,u32,u32),TerrainTileMeshPayload,u64,u64,u64)>,

    pub terrain_revision: u64,
}

/// 🧩️ World3d assets component: asset io authority, url bookkeeping and brush mesh announce.
pub struct WorldAssetsState {

    pub pending_glb_urls: HashSet<String>,

    pub pending_image_urls: HashSet<String>,

    /// 🥽️ Mesh url → the [`world_brush_mesh_revision`] THIS surface last announced it to the guest
    /// at. The wgpu twin of the `revision` prop React hands each mounted `BrushMeshRegistrar`: a
    /// surface announces an identity once per revision, and a surface that is rebuilt (window
    /// reopen, role switch) starts empty and announces again — see
    /// [`step_world3d_brush_mesh_announce`].
    pub brush_mesh_announced: HashMap<String, u64>,

    /// 🥽️ The ONE `registerBrushMesh` page run this surface has in flight. One page leaves per
    /// frame, so a user action never queues behind more than a single page — React's
    /// `drainPuzzle3dBrushMeshQueue` back pressure, expressed as a frame cursor.
    pub brush_mesh_run: Option<WorldBrushMeshRun>,

    pub brush_mesh_feedback_digest: Option<u64>,

    pub asset_url_misses: HashSet<String>,

    pub asset_generation: u64,

    pub asset_io: WorldAssetIoAuthority,
}

impl WorldTerrainState {
    pub fn new() -> Self {
        Self {
            terrain_style: None,
            terrain_source_json: None,
            terrain_overlay_tiles: HashMap::new(),
            terrain_camera_frame: None,
            terrain_interaction_digest: None,
            terrain_brush: None,
            terrain_applied_signature: None,
            terrain_session: TerrainSessionCore::default(),
            terrain_visible_tiles: HashSet::new(),
            terrain_built_tiles: HashSet::new(),
            pending_terrain_tile_urls: HashMap::new(),
            terrain_tile_misses: HashSet::new(),
            terrain_build: None,
            terrain_pending: None,
            terrain_revision: 0,
        }
    }

    pub fn set_terrain_json(&mut self, json: Option<&str>) {
        if self.terrain_source_json.as_deref() == json {
            return;
        }
        let style = json.and_then(|json| semio_framework_pack_json::from_json_str::<WorldTerrainStyle>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).filter(|style| {
            style.elevation_grid.as_ref().map_or_else(|| !style.tile_url_template.is_empty(), TerrainElevationGrid::is_valid)
        });
        if self.terrain_style.as_ref().map(|style| &style.overlays) != style.as_ref().map(|style| &style.overlays) {
            self.terrain_overlay_tiles.clear();
        }
        if self.terrain_style.as_ref().map(WorldTerrainStyle::signature) != style.as_ref().map(WorldTerrainStyle::signature) {
            self.terrain_camera_frame = None;
        }
        self.terrain_source_json = json.map(str::to_owned);
        self.terrain_style = style;
    }

    pub fn terrain_style_pending(&self) -> bool {
        self.terrain_style.as_ref().map(WorldTerrainStyle::signature) != self.terrain_applied_signature.as_ref().map(|(url, lon, lat, exaggeration, elevation)| (url.as_str(), *lon, *lat, *exaggeration, elevation.as_ref()))
    }

    pub fn tile_source(&self) -> Option<(&str, u32, u32)> {
        if self.terrain_style_pending() {
            return None;
        }
        let style = self.terrain_style.as_ref()?;
        Some((&style.tile_url_template, style.min_zoom, style.max_zoom))
    }

    pub fn terrain_attribution_label(&self) -> Option<&str> {
        self.terrain_style.as_ref()?.attribution.as_ref().map(|attribution| attribution.label.as_str())
    }

    /// 🖱️ Applies the terrain utility projected by the current editable World3d window.
    pub fn set_terrain_interaction(&mut self, json: Option<&str>) {
        let digest = json.map(|json| world3d_scene_digest(&[json]));
        if self.terrain_interaction_digest == digest {
            return;
        }
        self.terrain_interaction_digest = digest;
        self.terrain_brush = json.and_then(|json| semio_framework_pack_json::from_json_str::<TerrainInteraction>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok())
            .and_then(|interaction| interaction.terrain_brush)
            .filter(TerrainBrushAction::is_valid);
    }

    /// ⛏️ Returns the active brush only for an editable elevation source.
    pub fn terrain_brush(&self) -> Option<(&TerrainBrushAction, u64)> {
        self.terrain_style.as_ref()?.elevation_grid.as_ref()?;
        Some((self.terrain_brush.as_ref()?, self.terrain_interaction_digest?))
    }

    /// 🎯 Finds the nearest editable grid sample under the scene ray.
    pub fn pick_terrain_sample(&self, origin: [f64; 3], direction: [f64; 3]) -> Option<[u32; 2]> {
        if self.terrain_style_pending() {
            return None;
        }
        self.terrain_session.pick_elevation_grid(origin, direction).map(|hit| hit.sample)
    }

    pub fn sync_terrain_camera_frame(&mut self, camera_json: &str) {
        let Some(style) = &self.terrain_style else { return };
        let digest = world3d_scene_digest(&[camera_json]);
        if self.terrain_camera_frame.is_some_and(|(current, _)| current == digest) {
            return;
        }
        let pending = serde_json::from_str::<TerrainCameraFrame>(camera_json).ok()
            .and_then(|camera| camera.terrain_frame)
            .is_some_and(|[lon, lat, _]| lon.to_bits() == style.project_origin_lon.to_bits() && lat.to_bits() == style.project_origin_lat.to_bits());
        self.terrain_camera_frame = Some((digest, pending));
    }

    pub fn take_camera_origin_elevation(&mut self, camera_digest: Option<u64>, user_moved: bool) -> Option<f32> {
        if self.terrain_style_pending() {
            return None;
        }
        let (digest, pending) = self.terrain_camera_frame.as_mut()?;
        if !*pending || camera_digest != Some(*digest) {
            return None;
        }
        if user_moved {
            *pending = false;
            return None;
        }
        let elevation = self.terrain_visible_tiles.iter().find_map(|&(z, x, y)| self.terrain_session.terrain_origin_elevation(z, x, y))?;
        *pending = false;
        Some(elevation)
    }

    pub fn overlay_tile_needs_build(&self, tile: (u32, u32, u32)) -> bool {
        self.terrain_style.as_ref().is_some_and(WorldTerrainStyle::has_overlays) && !self.terrain_overlay_tiles.contains_key(&tile)
    }

    pub fn tile_overlays(&mut self, tile: (u32, u32, u32)) -> Option<&TerrainOverlayTile> {
        let style = self.terrain_style.as_ref()?;
        if !self.terrain_built_tiles.contains(&tile) || !style.has_overlays() {
            return None;
        }
        let (z, x, y) = tile;
        match self.terrain_overlay_tiles.entry(tile) {
            std::collections::hash_map::Entry::Occupied(entry) => Some(entry.into_mut()),
            std::collections::hash_map::Entry::Vacant(entry) => Some(entry.insert(self.terrain_session.overlay_tile(z, x, y, &style.overlays)?)),
        }
    }

    pub fn forget_tile(&mut self, tile: (u32, u32, u32)) {
        self.terrain_session.evict_terrain_tile(tile.0, tile.1, tile.2);
        self.terrain_built_tiles.remove(&tile);
        self.terrain_overlay_tiles.remove(&tile);
    }
}

impl WorldAssetsState {
    pub fn new() -> Self {
        Self {
            pending_glb_urls: HashSet::new(),
            pending_image_urls: HashSet::new(),
            brush_mesh_announced: HashMap::new(),
            brush_mesh_run: None,
            brush_mesh_feedback_digest: None,
            asset_url_misses: HashSet::new(),
            asset_generation: 0,
            asset_io: WorldAssetIoAuthority::default(),
        }
    }
}

/// 🔏️ FNV-1a over the payload lanes that decide whether a bridge rebuild is needed. Content
/// addressing, not a clock: an app that republishes an identical scene must not cost a rebuild.
pub fn world3d_scene_digest(parts: &[&str]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// 🖼️ Whether the active document still owns `url`; a decoded response must answer this in addition
/// to the pane-generation guard because the same live pane may replace its reference while bytes are
/// in flight.
pub fn world3d_reference_url_is_current(scene_state: &WorldSceneState, url: &str) -> bool {
    scene_state.references.iter().any(|reference| reference.url.as_deref() == Some(url))
}
//#endregion 🌉️World3dSceneBridge

#[path = "📡️asset-io/🦀️.rs"]
mod asset_io;
pub use asset_io::*;

#[path = "🥽️brush-lod-and-style/🦀️.rs"]
mod brush_lod_and_style;
pub use brush_lod_and_style::*;

#[path = "🏔️environment-and-terrain/🦀️.rs"]
mod environment_and_terrain;
pub use environment_and_terrain::*;
