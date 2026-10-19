//! 🧱️ World3d scene resources and asset io: pools, chunking, LOD grid, environment, terrain, placeholder meshes and asset requests.
//!
//! Crate `semio-framework-os-infinite-world-scene`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_framework_os_infinite_world_model::{World3dBuildContext, World3dMeshAsset, World3dMeshAssetRejected, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_DYNAMIC_MESH_CAPACITY, WorldDynamicFault, WorldDynamicRejected, WorldEnvironmentRecord, WorldInteractionState, WorldMeshRegistry, WorldReferenceRecord, WorldSessionState, WorldTerrainStyle};
use semio_framework_os_infinite_world_components::{placeholder_normalize3, placeholder_scale3, TerrainTileMeshPayload, WORLD_REFERENCE_SOURCE_PIXELS, WorldBrushMeshRun, WorldCameraState, WorldMeshBuffers, WorldPlaceholderOwner, WorldSceneState, WorldSelectionState};
use semio_framework_ui_declaration::layout::ActionDescriptor;
use ui_scene::math::{Camera3d, camera_grid_fade_distance, Instance3d, LineVertex3d, lod_from_camera_distance, lod_grid_fade_alpha, lod_grid_step_world, lod_orbit_distance_for_camera, Mat4, mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_seal, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, SceneColorSource3d, SceneDraw3d, Vec3};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::{Rgba, Theme};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use ui_scene::math::{directional_shadow_view_projection, ICON_SHADOW_MAP_SIZE, SceneMaterial3d, SceneMaterialDraw3d, SceneShadow3d, SceneShadowRole3d, WORLD_SHADOW_MAP_SIZE};
use std::sync::{LazyLock, Mutex};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_surface_terrain::TerrainSessionCore;
use serde_json::json;
#[cfg(any(test, feature = "testkit"))] use ui_scene::math::pick_closest_mesh_url;
use ui_styling::metrics::tool_run;

/// 🧩️ World3d terrain component: terrain style, tiles, session and build.
pub struct WorldTerrainState {

    pub terrain_style: Option<WorldTerrainStyle>,

    pub terrain_applied_signature: Option<(String, f64, f64, f64)>,

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
