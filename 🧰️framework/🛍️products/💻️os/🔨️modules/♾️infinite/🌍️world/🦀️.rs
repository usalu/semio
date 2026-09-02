//! 🌐️ Application-neutral 3D world canvas: mesh loading, orbit camera, picking, and marquee selection.

use crate::framework_surface_terrain::TerrainSessionCore;
// 🧩️ Every name below is target-neutral (ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS's
// wgpu-tier split): `draw_text`/`WidgetContext`/the paint half of `gizmo` are genuinely GPU-adjacent
// (font/icon atlases) and are imported locally inside `render_world_3d`, the one function that is
// itself `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`-gated instead.
use ui_wgpu::wgpu::{
    aabb_intersects_frustum, axis_rotate_angle, frustum_planes, gizmo, grid_placement_anchor, gumball_extent, gumball_eye, gumball_project_ray_onto_axis, interpolate_mesh_uv, lod_from_camera_distance, lod_progressive_grid_layers,
    marquee_is_crossing_from_path, mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_seal, mesh3d_terminal_is_empty, mesh3d_write_u32, mesh3d_write_vec3, mesh_content_version,
    paint_selection_marquee, pick_closest_mesh_url, quat_from_basis, ray_aabb_slab, ray_pick_instance, ray_pick_mesh_detail, ray_plane_point, ray_segment_distance, rotate_vector, screen_select_components, screen_select_instances, transform_aabb,
    vec3_from_f64, world3d_snapshot_claim_draw_permit, world3d_snapshot_with_page, ActionDescriptor, Camera3d, HitKind, HitTarget, Instance3d, LineDraw3d, LineVertex3d, LocalizedLabel, Mat4, Mesh3dField, Mesh3dLease, Mesh3dSchema,
    Mesh3dWriteToken, OrbitController, PointerModifiers, PreparedRasterProducer, PreparedRasterRejected, PreparedRenderEviction, PreparedRenderUpload, Rect, Rgba, SceneDraw3d, ScenePass3d, TexturedDraw3d, TexturedInstance3d, UiComponentSceneNode,
    Vec3, World3dSnapshotDrawPermit, World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPageKind,
};

//#region 📦️PreparedWorldResources
const WORLD3D_FRAME_RESOURCE_CAPACITY: usize = 256;

pub enum World3dBuildRejected {
    Upload(PreparedRenderUpload),
    RasterProducer(PreparedRasterProducer),
    RasterAdmission(PreparedRasterRejected),
    Eviction(PreparedRenderEviction),
}

impl World3dBuildRejected {
    pub fn close_step(&mut self) -> bool {
        match self {
            Self::Upload(upload) => upload.close_step(),
            Self::RasterProducer(producer) => {
                producer.begin_close();
                producer.close_step()
            }
            Self::RasterAdmission(rejected) => rejected.close_step(),
            Self::Eviction(PreparedRenderEviction::Mesh { key }) => key.pop().is_none(),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            #[cfg(test)]
            Self::Upload(PreparedRenderUpload::GlyphAtlas { pixels, .. } | PreparedRenderUpload::IconAtlas { pixels, .. }) => pixels.is_empty(),
            Self::Upload(PreparedRenderUpload::GlyphAtlasPages { pixels } | PreparedRenderUpload::IconAtlasPages { pixels }) => pixels.terminal_is_empty(),
            #[cfg(test)]
            Self::Upload(PreparedRenderUpload::Raster { key, pixels, .. }) => key.is_empty() && pixels.is_empty(),
            Self::Upload(PreparedRenderUpload::RasterPages { key, .. }) => key.is_empty(),
            Self::Upload(PreparedRenderUpload::Mesh { key, .. }) => key.is_empty(),
            Self::RasterProducer(producer) => producer.terminal_is_empty(),
            Self::RasterAdmission(rejected) => rejected.terminal_is_empty(),
            Self::Eviction(PreparedRenderEviction::Mesh { key }) => key.is_empty(),
        }
    }
}

/// 📦️ Worker-owned World3d resource requests. It contains only CPU buffers and cache keys; device,
/// queue, texture, and surface authority remain in the prepared renderer presenter.
pub struct World3dBuildContext {
    uploads: Box<[Option<PreparedRenderUpload>; WORLD3D_FRAME_RESOURCE_CAPACITY]>,
    upload_len: usize,
    raster_producers: Box<[Option<PreparedRasterProducer>; WORLD3D_FRAME_RESOURCE_CAPACITY]>,
    raster_producer_len: usize,
    evictions: Box<[Option<PreparedRenderEviction>; WORLD3D_FRAME_RESOURCE_CAPACITY]>,
    eviction_len: usize,
    mesh_requests: Box<[Option<(String, u64)>; WORLD3D_FRAME_RESOURCE_CAPACITY]>,
    mesh_request_len: usize,
    raster_requests: Box<[Option<String>; WORLD3D_FRAME_RESOURCE_CAPACITY]>,
    raster_request_len: usize,
    rejected: Option<World3dBuildRejected>,
    cursor_wake: WorldCursorWakeAuthority,
    cursor_wake_token: Option<WorldCursorWakeToken>,
    cursor_wake_fault: Option<WorldCursorWakeFault>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WorldCursorWakeToken {
    generation: u64,
}

impl WorldCursorWakeToken {
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldCursorWakeFault {
    Closed,
    GenerationExhausted,
}

#[derive(Default)]
struct WorldCursorWakeState {
    generation: u64,
    acknowledged_generation: u64,
    pending_generation: Option<u64>,
    close_phase: u8,
    closing: bool,
    terminal: bool,
}

#[derive(Clone, Default)]
pub struct WorldCursorWakeAuthority(std::sync::Arc<std::sync::Mutex<WorldCursorWakeState>>);

impl WorldCursorWakeAuthority {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request(&self) -> Result<WorldCursorWakeToken, WorldCursorWakeFault> {
        let mut state = self.0.lock().expect("World cursor wake authority lock");
        if state.closing {
            return Err(WorldCursorWakeFault::Closed);
        }
        let generation = match state.pending_generation {
            Some(generation) => generation,
            None => {
                let generation = state.generation.checked_add(1).ok_or(WorldCursorWakeFault::GenerationExhausted)?;
                state.generation = generation;
                state.pending_generation = Some(generation);
                generation
            }
        };
        Ok(WorldCursorWakeToken { generation })
    }

    pub fn acknowledge(&self, token: &WorldCursorWakeToken) -> bool {
        let mut state = self.0.lock().expect("World cursor wake authority lock");
        if state.closing || state.pending_generation != Some(token.generation) {
            return false;
        }
        state.pending_generation = None;
        state.acknowledged_generation = token.generation;
        true
    }

    pub fn close_step(&self) -> bool {
        let mut state = self.0.lock().expect("World cursor wake authority lock");
        state.closing = true;
        match state.close_phase {
            0 => {
                state.pending_generation = None;
                state.close_phase = 1;
                false
            }
            1 => {
                state.generation = 0;
                state.close_phase = 2;
                false
            }
            2 => {
                state.acknowledged_generation = 0;
                state.close_phase = 3;
                false
            }
            _ => {
                state.terminal = true;
                true
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        let state = self.0.lock().expect("World cursor wake authority lock");
        state.terminal && state.pending_generation.is_none() && state.generation == 0 && state.acknowledged_generation == 0
    }

    #[cfg(test)]
    fn pending_generation(&self) -> Option<u64> {
        self.0.lock().expect("World cursor wake authority lock").pending_generation
    }
}

impl World3dBuildContext {
    pub fn new(cursor_wake: WorldCursorWakeAuthority) -> Self {
        Self {
            uploads: Box::new([const { None }; WORLD3D_FRAME_RESOURCE_CAPACITY]),
            upload_len: 0,
            raster_producers: Box::new([const { None }; WORLD3D_FRAME_RESOURCE_CAPACITY]),
            raster_producer_len: 0,
            evictions: Box::new([const { None }; WORLD3D_FRAME_RESOURCE_CAPACITY]),
            eviction_len: 0,
            mesh_requests: Box::new([const { None }; WORLD3D_FRAME_RESOURCE_CAPACITY]),
            mesh_request_len: 0,
            raster_requests: Box::new([const { None }; WORLD3D_FRAME_RESOURCE_CAPACITY]),
            raster_request_len: 0,
            rejected: None,
            cursor_wake,
            cursor_wake_token: None,
            cursor_wake_fault: None,
        }
    }

    pub fn ensure_mesh(&mut self, key: &str, version: u64, lease: Mesh3dLease) {
        if self.mesh_requests[..self.mesh_request_len].iter().flatten().any(|candidate| candidate.0 == key && candidate.1 == version) {
            return;
        }
        let upload = PreparedRenderUpload::Mesh { key: key.to_string(), version, lease };
        if self.mesh_request_len == WORLD3D_FRAME_RESOURCE_CAPACITY || self.upload_len == WORLD3D_FRAME_RESOURCE_CAPACITY {
            if self.rejected.is_none() {
                self.rejected = Some(World3dBuildRejected::Upload(upload));
            }
            return;
        }
        self.mesh_requests[self.mesh_request_len] = Some((key.to_string(), version));
        self.mesh_request_len += 1;
        self.uploads[self.upload_len] = Some(upload);
        self.upload_len += 1;
    }

    pub fn ensure_world_plane_texture(&mut self, key: &str, pixels: &[u8], width: u32, height: u32) {
        if self.raster_requests[..self.raster_request_len].iter().flatten().any(|candidate| candidate == key) {
            return;
        }
        let producer = match PreparedRasterProducer::try_admit(key.to_string(), pixels.to_vec(), width, height) {
            Ok((producer, _)) => producer,
            Err(rejected) => {
                if self.rejected.is_none() {
                    self.rejected = Some(World3dBuildRejected::RasterAdmission(rejected));
                }
                return;
            }
        };
        if self.raster_request_len == WORLD3D_FRAME_RESOURCE_CAPACITY || self.raster_producer_len == WORLD3D_FRAME_RESOURCE_CAPACITY {
            if self.rejected.is_none() {
                self.rejected = Some(World3dBuildRejected::RasterProducer(producer));
            }
            return;
        }
        self.raster_requests[self.raster_request_len] = Some(key.to_string());
        self.raster_request_len += 1;
        self.raster_producers[self.raster_producer_len] = Some(producer);
        self.raster_producer_len += 1;
    }

    pub fn evict_mesh(&mut self, key: &str) {
        let eviction = PreparedRenderEviction::Mesh { key: key.to_string() };
        if self.eviction_len == WORLD3D_FRAME_RESOURCE_CAPACITY {
            if self.rejected.is_none() {
                self.rejected = Some(World3dBuildRejected::Eviction(eviction));
            }
            return;
        }
        self.evictions[self.eviction_len] = Some(eviction);
        self.eviction_len += 1;
    }

    fn request_cursor_wake(&mut self) {
        if self.cursor_wake_token.is_some() || self.cursor_wake_fault.is_some() {
            return;
        }
        match self.cursor_wake.request() {
            Ok(token) => self.cursor_wake_token = Some(token),
            Err(fault) => self.cursor_wake_fault = Some(fault),
        }
    }

    pub fn take_cursor_wake(&mut self) -> Result<Option<WorldCursorWakeToken>, WorldCursorWakeFault> {
        match self.cursor_wake_fault.take() {
            Some(fault) => Err(fault),
            None => Ok(self.cursor_wake_token.take()),
        }
    }

    pub fn append_step(&mut self, input: &mut ui_wgpu::wgpu::PreparedRenderInput) -> Result<bool, World3dBuildRejected> {
        if let Some(rejected) = self.rejected.take() {
            return Err(rejected);
        }
        if let Some(index) = self.raster_producer_len.checked_sub(1) {
            self.raster_producer_len = index;
            let Some(producer) = self.raster_producers[index].take() else { return Ok(false) };
            let Some(next) = input.raster_producers.len().checked_add(input.uploads.len()).and_then(|count| count.checked_add(1)) else {
                return Err(World3dBuildRejected::RasterProducer(producer));
            };
            if next > input.limits.max_upload_items {
                return Err(World3dBuildRejected::RasterProducer(producer));
            }
            if let Err(producer) = input.try_push_raster_producer(producer) {
                return Err(World3dBuildRejected::RasterProducer(producer));
            }
            return Ok(false);
        }
        if let Some(index) = self.upload_len.checked_sub(1) {
            self.upload_len = index;
            let Some(upload) = self.uploads[index].take() else { return Ok(false) };
            let Some(next) = input.raster_producers.len().checked_add(input.uploads.len()).and_then(|count| count.checked_add(1)) else {
                return Err(World3dBuildRejected::Upload(upload));
            };
            if next > input.limits.max_upload_items {
                return Err(World3dBuildRejected::Upload(upload));
            }
            if let Err(upload) = input.try_push_upload(upload) {
                return Err(World3dBuildRejected::Upload(upload));
            }
            return Ok(false);
        }
        if let Some(index) = self.eviction_len.checked_sub(1) {
            self.eviction_len = index;
            let Some(eviction) = self.evictions[index].take() else { return Ok(false) };
            if let Err(eviction) = input.try_push_eviction(eviction) {
                return Err(World3dBuildRejected::Eviction(eviction));
            }
            return Ok(false);
        }
        if let Some(index) = self.mesh_request_len.checked_sub(1) {
            self.mesh_request_len = index;
            self.mesh_requests[index] = None;
            return Ok(false);
        }
        if let Some(index) = self.raster_request_len.checked_sub(1) {
            self.raster_request_len = index;
            self.raster_requests[index] = None;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.upload_len == 0
            && self.raster_producer_len == 0
            && self.eviction_len == 0
            && self.mesh_request_len == 0
            && self.raster_request_len == 0
            && self.rejected.is_none()
            && self.uploads.iter().all(Option::is_none)
            && self.raster_producers.iter().all(Option::is_none)
            && self.evictions.iter().all(Option::is_none)
            && self.mesh_requests.iter().all(Option::is_none)
            && self.raster_requests.iter().all(Option::is_none)
    }
}
//#endregion 📦️PreparedWorldResources

use semio_framework::{optional_json_to_dsl, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
use serde::de::Error as DeError;
use serde::Deserialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
use std::sync::{LazyLock, Mutex};

fn action_args(value: serde_json::Value) -> Option<semio_framework::DslValue> {
    optional_json_to_dsl(Some(value))
}

//#region SceneRecords
fn deserialize_optional_string_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(serde_json::Value::Array(items)) => Ok(Some(
            items
                .into_iter()
                .filter_map(|item| match item {
                    serde_json::Value::String(value) => Some(value),
                    serde_json::Value::Number(value) => value.as_u64().map(|id| id.to_string()),
                    _ => None,
                })
                .collect(),
        )),
        Some(other) => Err(D::Error::custom(format!("expected array for component ids, got {other}"))),
    }
}

fn json_id_to_string(value: &serde_json::Value) -> Option<String> {
    value.as_str().map(str::to_string).or_else(|| value.as_u64().map(|id| id.to_string()))
}

fn dsl_id_to_string(value: &semio_framework::DslValue) -> Option<String> {
    value.as_str().map(str::to_string).or_else(|| value.as_f64().map(|n| if n.fract() == 0.0 { format!("{}", n as u64) } else { n.to_string() }))
}

fn dsl_string_vec(value: &semio_framework::DslValue) -> Vec<String> {
    value.as_array().map(|items| items.iter().filter_map(dsl_id_to_string).collect()).unwrap_or_default()
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldCameraRecord {
    position: Option<[f64; 3]>,
    target: Option<[f64; 3]>,
    up: Option<[f64; 3]>,
    fov: Option<f64>,
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldMeshLodEntry {
    lod: f64,
    url: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldMeshRecord {
    id: String,
    url: Option<String>,
    lods: Option<Vec<WorldMeshLodEntry>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldLodRecord {
    #[serde(default = "default_true")]
    automatic: bool,
    #[serde(default = "default_manual_lod")]
    manual: f64,
    #[serde(default = "default_distance_reference")]
    distance_reference: f64,
    #[serde(default)]
    depth_variable: bool,
    #[serde(default = "default_grid_factor")]
    grid_factor: f64,
    #[serde(default = "default_true")]
    show_grid: bool,
    #[serde(default)]
    grid_datum: Option<[f64; 3]>,
}

impl Default for WorldLodRecord {
    fn default() -> Self {
        default_lod_record()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldChunkingRecord {
    chunk_size: f64,
    max_distance: f64,
}

fn default_manual_lod() -> f64 {
    100.0
}

fn default_distance_reference() -> f64 {
    100.0
}

fn default_grid_factor() -> f64 {
    10.0
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldInstanceRecord {
    id: String,
    mesh_id: Option<String>,
    position: Option<[f64; 3]>,
    rotation: Option<[f64; 4]>,
    scale: Option<[f64; 3]>,
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    color: Option<String>,
    selected: Option<bool>,
    hovered: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldSelectionTargets {
    #[serde(default)]
    vertex: bool,
    #[serde(default)]
    edge: bool,
    #[serde(default)]
    face: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldSelectionRecord {
    method: Option<String>,
    ids: Option<Vec<String>>,
    hovered_id: Option<String>,
    granularity: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string_vec")]
    component_ids: Option<Vec<String>>,
    transform_mode: Option<String>,
    interaction_mode: Option<String>,
    gumball_target: Option<[f64; 3]>,
    selection_mode: Option<String>,
    show_edges: Option<bool>,
    targets: Option<WorldSelectionTargets>,
    active_object_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldVortexRecord {
    full_id: String,
    position: Option<[f64; 3]>,
    direction: Option<[f64; 3]>,
    display_direction: Option<String>,
    radius: Option<f64>,
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldAttractionRecord {
    from: Option<[f64; 3]>,
    to: Option<[f64; 3]>,
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldTargetVolumeRecord {
    origin: Option<[f64; 3]>,
    orientation: Option<[f64; 4]>,
    scale: Option<[f64; 3]>,
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldReferenceRecord {
    url: Option<String>,
    origin: Option<[f64; 3]>,
    width_world: Option<f64>,
    hidden: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldBrushPreviewRecord {
    mesh_url: Option<String>,
    origin: Option<[f64; 3]>,
    orientation: Option<[f64; 4]>,
    scale: Option<serde_json::Value>,
    target_vortex_full_id: Option<String>,
    object_kind_id: Option<String>,
    source_vortex_index: Option<usize>,
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldInteractionRecord {
    active_utility: Option<String>,
    hovered_vortex_full_id: Option<String>,
}

//#region Environment
/// ☀️ Directional sun light — `enabled` gates whether `azimuth`/`elevation` (degrees, horizontal
/// coordinate system) replace the renderer's default `light_dir`; `intensity`/`color` have no
/// representable channel in `ScenePass3d` (single direction vector, no color/intensity) yet.
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WorldEnvironmentSunRecord {
    enabled: Option<bool>,
    azimuth: Option<f64>,
    elevation: Option<f64>,
    #[allow(dead_code)] // 🔌️ no per-light intensity channel in ScenePass3d yet; wiring gap, see report.
    intensity: Option<f64>,
    #[allow(dead_code)] // 🔌️ no per-light color channel in ScenePass3d yet; wiring gap, see report.
    color: Option<String>,
}

/// 💡️ Ambient light — parsed for scene-shape completeness; `ScenePass3d` has no ambient
/// color/intensity channel to apply it to (wiring gap, see report).
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct WorldEnvironmentAmbientRecord {
    intensity: Option<f64>,
    color: Option<String>,
}

/// 🌑️ Shadow toggle — dead in the React reference too (no shadow-map consumer there either);
/// kept for scene-shape completeness only.
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct WorldEnvironmentShadowRecord {
    enabled: Option<bool>,
    opacity: Option<f64>,
    softness: Option<f64>,
}

/// 🎨️ Neutral-instance material override — `color` becomes the base-color fallback for instances
/// without an explicit per-instance color (mirrors the React reference's "only applies when the
/// instance isn't selected/hovered" rule, since Rust's selection/hover highlighting is a separate
/// boolean layered on top rather than a color premix). `metalness`/`roughness`/`emissive*` have no
/// PBR channel on `Instance3d` yet (wiring gap, see report).
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WorldEnvironmentMaterialRecord {
    color: Option<String>,
    #[allow(dead_code)]
    metalness: Option<f64>,
    #[allow(dead_code)]
    roughness: Option<f64>,
    #[allow(dead_code)]
    emissive: Option<String>,
    #[allow(dead_code)]
    emissive_intensity: Option<f64>,
}

/// 🌍️ `World3dScene.environmentJson` mirror — see `world-3d-host.tsx`'s `WorldEnvironmentRecord`.
/// Only `background` (canvas clear color), `sun` (light direction), and `material.color` (neutral
/// instance base-color fallback) are representable in this renderer today; the rest is parsed for
/// forward-compat and documented per-field above.
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WorldEnvironmentRecord {
    background: Option<String>,
    ambient: Option<WorldEnvironmentAmbientRecord>,
    sun: Option<WorldEnvironmentSunRecord>,
    shadow: Option<WorldEnvironmentShadowRecord>,
    material: Option<WorldEnvironmentMaterialRecord>,
}
//#endregion Environment

//#region TerrainStyle
/// 🌐️⛰️ `World3dScene.terrainJson` mirror — GIS-3D terrain style/source descriptor consumed by
/// `WorldTerrainLayer` in the React reference. `color_ramp`/`min_zoom`/`max_zoom` are parsed but
/// not branched on, mirroring the React reference (single hardcoded hypsometric ramp; zoom bounds
/// fixed inside `framework_surface_terrain::tiles`) — not a gap, a faithful match of upstream.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WorldTerrainStyle {
    tile_url_template: String,
    #[serde(default)]
    project_origin_lon: f64,
    #[serde(default)]
    project_origin_lat: f64,
    #[serde(default = "default_terrain_exaggeration")]
    exaggeration: f64,
    #[serde(default = "default_terrain_color_ramp")]
    #[allow(dead_code)]
    color_ramp: String,
    #[serde(default = "default_terrain_min_zoom")]
    #[allow(dead_code)]
    min_zoom: u32,
    #[serde(default = "default_terrain_max_zoom")]
    #[allow(dead_code)]
    max_zoom: u32,
}

fn default_terrain_exaggeration() -> f64 {
    1.0
}

fn default_terrain_color_ramp() -> String {
    "hypsometric".into()
}

fn default_terrain_min_zoom() -> u32 {
    6
}

fn default_terrain_max_zoom() -> u32 {
    14
}
//#endregion TerrainStyle
//#endregion SceneRecords

//#region World3dState
const WORLD_DYNAMIC_MESH_CAPACITY: usize = 256;
const WORLD_DYNAMIC_DRAW_CAPACITY: usize = 256;
const WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY: usize = 4_096;
const WORLD_DYNAMIC_DRAW_BYTE_CAPACITY: usize = 16 * 1024 * 1024;
const WORLD_DYNAMIC_PIXEL_CAPACITY: usize = 256;
const WORLD_OPAQUE_QUARANTINE_CAPACITY: usize = 1024;
const WORLD_DYNAMIC_ID_BYTE_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldDynamicToken {
    slot: u16,
    epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldDynamicFault {
    IdCapacity,
    RegistryCapacity,
    StaleToken,
    QuarantineCapacity,
    Closing,
    InstanceCapacity,
    ByteCapacity,
}

#[derive(Debug)]
struct WorldDynamicEntry<T> {
    id: String,
    epoch: u64,
    value: T,
}

#[derive(Debug)]
pub struct WorldDynamicRejected<T> {
    pub fault: WorldDynamicFault,
    pub id: String,
    pub value: T,
}

struct WorldDynamicRegistry<T, const N: usize> {
    slots: Box<[Option<WorldDynamicEntry<T>>; N]>,
    epochs: [u64; N],
    len: u16,
    closing: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorldDynamicInsertPlan {
    slot: u16,
    epoch: u64,
    occupied: bool,
}

impl<T, const N: usize> Default for WorldDynamicRegistry<T, N> {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; N]), epochs: [0; N], len: 0, closing: false }
    }
}

impl<T, const N: usize> WorldDynamicRegistry<T, N> {
    fn get(&self, id: &str) -> Option<&T> {
        self.slots.iter().flatten().find(|entry| entry.id == id).map(|entry| &entry.value)
    }

    fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        self.slots.iter_mut().flatten().find(|entry| entry.id == id).map(|entry| &mut entry.value)
    }

    fn contains_key(&self, id: &str) -> bool {
        self.get(id).is_some()
    }

    fn keys(&self) -> impl Iterator<Item = &String> {
        self.slots.iter().flatten().map(|entry| &entry.id)
    }

    fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.slots.iter().flatten().map(|entry| (&entry.id, &entry.value))
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn len(&self) -> usize {
        usize::from(self.len)
    }

    fn token(&self, id: &str) -> Option<WorldDynamicToken> {
        self.slots.iter().enumerate().find_map(|(slot, entry)| entry.as_ref().filter(|entry| entry.id == id).map(|entry| WorldDynamicToken { slot: slot as u16, epoch: entry.epoch }))
    }

    fn insert(&mut self, id: String, value: T) -> Result<(WorldDynamicToken, Option<WorldDynamicEntry<T>>), WorldDynamicRejected<T>> {
        if self.closing {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value });
        }
        if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::IdCapacity, id, value });
        }
        if let Some((slot, entry)) = self.slots.iter_mut().enumerate().find(|(_, entry)| entry.as_ref().is_some_and(|entry| entry.id == id)) {
            self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
            let epoch = self.epochs[slot];
            let previous = entry.replace(WorldDynamicEntry { id, epoch, value });
            return Ok((WorldDynamicToken { slot: slot as u16, epoch }, previous));
        }
        let Some(slot) = self.slots.iter().position(Option::is_none) else {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::RegistryCapacity, id, value });
        };
        self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
        let epoch = self.epochs[slot];
        self.slots[slot] = Some(WorldDynamicEntry { id, epoch, value });
        self.len += 1;
        Ok((WorldDynamicToken { slot: slot as u16, epoch }, None))
    }

    fn plan_insert(&self, id: &str) -> Result<WorldDynamicInsertPlan, WorldDynamicFault> {
        if self.closing {
            return Err(WorldDynamicFault::Closing);
        }
        if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(WorldDynamicFault::IdCapacity);
        }
        if let Some((slot, entry)) = self.slots.iter().enumerate().find_map(|(slot, entry)| entry.as_ref().filter(|entry| entry.id == id).map(|entry| (slot, entry))) {
            return Ok(WorldDynamicInsertPlan { slot: slot as u16, epoch: entry.epoch, occupied: true });
        }
        let Some(slot) = self.slots.iter().position(Option::is_none) else {
            return Err(WorldDynamicFault::RegistryCapacity);
        };
        Ok(WorldDynamicInsertPlan { slot: slot as u16, epoch: self.epochs[slot], occupied: false })
    }

    fn commit_insert(&mut self, plan: WorldDynamicInsertPlan, id: String, value: T) -> Result<(WorldDynamicToken, Option<WorldDynamicEntry<T>>), WorldDynamicRejected<T>> {
        if self.closing {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value });
        }
        let slot = usize::from(plan.slot);
        let Some(entry) = self.slots.get(slot) else {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::StaleToken, id, value });
        };
        let matches = if plan.occupied { entry.as_ref().is_some_and(|entry| entry.id == id && entry.epoch == plan.epoch) } else { entry.is_none() && self.epochs[slot] == plan.epoch };
        if !matches {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::StaleToken, id, value });
        }
        let epoch = self.epochs[slot].wrapping_add(1).max(1);
        self.epochs[slot] = epoch;
        let previous = self.slots[slot].replace(WorldDynamicEntry { id, epoch, value });
        if previous.is_none() {
            self.len += 1;
        }
        Ok((WorldDynamicToken { slot: plan.slot, epoch }, previous))
    }

    fn remove(&mut self, id: &str) -> Option<WorldDynamicEntry<T>> {
        let slot = self.slots.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.id == id))?;
        self.len -= 1;
        self.slots[slot].take()
    }

    fn remove_token(&mut self, token: WorldDynamicToken) -> Result<WorldDynamicEntry<T>, WorldDynamicFault> {
        let slot = usize::from(token.slot);
        let Some(entry) = self.slots.get(slot).and_then(Option::as_ref) else {
            return Err(WorldDynamicFault::StaleToken);
        };
        if entry.epoch != token.epoch {
            return Err(WorldDynamicFault::StaleToken);
        }
        self.len -= 1;
        Ok(self.slots[slot].take().expect("validated world dynamic slot"))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn take_one(&mut self) -> Option<WorldDynamicEntry<T>> {
        let slot = self.slots.iter().position(Option::is_some)?;
        self.len -= 1;
        self.slots[slot].take()
    }

    fn restore(&mut self, entry: WorldDynamicEntry<T>) {
        let slot = self.slots.iter().position(Option::is_none).expect("world registry restores only an owner detached from itself");
        self.slots[slot] = Some(entry);
        self.len += 1;
    }
}

#[cfg(not(test))]
impl<T, const N: usize> Drop for WorldDynamicRegistry<T, N> {
    fn drop(&mut self) {
        assert!(self.is_empty(), "world dynamic registry reached Drop before its exact terminal-empty witness");
    }
}

struct WorldDrawRegistry {
    slots: Box<[Option<WorldDynamicEntry<SceneDraw3d>>; WORLD_DYNAMIC_DRAW_CAPACITY]>,
    epochs: [u64; WORLD_DYNAMIC_DRAW_CAPACITY],
    len: u16,
    closing: bool,
}

impl Default for WorldDrawRegistry {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; WORLD_DYNAMIC_DRAW_CAPACITY]), epochs: [0; WORLD_DYNAMIC_DRAW_CAPACITY], len: 0, closing: false }
    }
}

impl WorldDrawRegistry {
    fn get(&self, index: usize) -> Option<&SceneDraw3d> {
        self.slots.get(index).and_then(Option::as_ref).map(|entry| &entry.value)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut SceneDraw3d> {
        self.slots.get_mut(index).and_then(Option::as_mut).map(|entry| &mut entry.value)
    }

    fn iter(&self) -> impl Iterator<Item = &SceneDraw3d> {
        self.slots[..usize::from(self.len)].iter().filter_map(|entry| entry.as_ref().map(|entry| &entry.value))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut SceneDraw3d> {
        self.slots[..usize::from(self.len)].iter_mut().filter_map(|entry| entry.as_mut().map(|entry| &mut entry.value))
    }

    #[cfg(test)]
    fn push(&mut self, value: SceneDraw3d) -> Result<WorldDynamicToken, WorldDynamicRejected<SceneDraw3d>> {
        let id = value.mesh_key.clone();
        if self.closing {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value });
        }
        if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::IdCapacity, id, value });
        }
        if value.instances.len() > WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::InstanceCapacity, id, value });
        }
        let mut bytes = id.len();
        for instance in &value.instances {
            if instance.id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
                return Err(WorldDynamicRejected { fault: WorldDynamicFault::IdCapacity, id, value });
            }
            let Some(next) = bytes.checked_add(instance.id.len()).and_then(|bytes| bytes.checked_add(std::mem::size_of::<Instance3d>())) else {
                return Err(WorldDynamicRejected { fault: WorldDynamicFault::ByteCapacity, id, value });
            };
            if next > WORLD_DYNAMIC_DRAW_BYTE_CAPACITY {
                return Err(WorldDynamicRejected { fault: WorldDynamicFault::ByteCapacity, id, value });
            }
            bytes = next;
        }
        self.push_prevalidated(value)
    }

    fn push_prevalidated(&mut self, value: SceneDraw3d) -> Result<WorldDynamicToken, WorldDynamicRejected<SceneDraw3d>> {
        let id = value.mesh_key.clone();
        if self.closing {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value });
        }
        let slot = usize::from(self.len);
        if slot == WORLD_DYNAMIC_DRAW_CAPACITY {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::RegistryCapacity, id, value });
        }
        self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
        let epoch = self.epochs[slot];
        self.slots[slot] = Some(WorldDynamicEntry { id, epoch, value });
        self.len += 1;
        Ok(WorldDynamicToken { slot: slot as u16, epoch })
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(test)]
    fn clear_into_quarantine(&mut self) -> Result<(), WorldDynamicFault> {
        while let Some(entry) = self.take_last() {
            if let Err(entry) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                self.slots[usize::from(self.len)] = Some(match entry {
                    WorldOpaqueOwner::Draw(entry) => entry,
                    _ => unreachable!("draw quarantine returned draw owner"),
                });
                self.len += 1;
                return Err(WorldDynamicFault::QuarantineCapacity);
            }
        }
        Ok(())
    }

    fn take_last(&mut self) -> Option<WorldDynamicEntry<SceneDraw3d>> {
        let len = usize::from(self.len);
        if len == 0 {
            return None;
        }
        self.len -= 1;
        self.slots[len - 1].take()
    }

    fn restore_last(&mut self, entry: WorldDynamicEntry<SceneDraw3d>) {
        let slot = usize::from(self.len);
        self.slots[slot] = Some(entry);
        self.len += 1;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldDrawRebuildDescriptor {
    pub generation: u64,
    pub revision: u64,
    pub draw_count: u16,
    pub instance_count: u32,
    pub byte_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldDrawRebuildStep {
    Pending,
    Complete,
    Stale,
    Fault,
}

struct WorldDrawDraft {
    mesh_key: String,
    mesh_version: u64,
    slots: Box<[MaybeUninit<Instance3d>]>,
    admitted: usize,
    taken: usize,
}

impl WorldDrawDraft {
    fn new(mesh_key: String, mesh_version: u64, instance_count: usize) -> Self {
        Self { mesh_key, mesh_version, slots: Box::<[Instance3d]>::new_uninit_slice(instance_count), admitted: 0, taken: 0 }
    }

    fn push(&mut self, instance: Instance3d) -> Result<(), Instance3d> {
        if self.admitted == self.slots.len() {
            return Err(instance);
        }
        self.slots[self.admitted].write(instance);
        self.admitted += 1;
        Ok(())
    }

    fn take_next(&mut self) -> Option<Instance3d> {
        if self.taken == self.admitted {
            return None;
        }
        let instance = unsafe { self.slots[self.taken].assume_init_read() };
        self.taken += 1;
        Some(instance)
    }

    fn complete(&self) -> bool {
        self.admitted == self.slots.len()
    }

    fn terminal_is_empty(&self) -> bool {
        self.taken == self.admitted
    }
}

impl Drop for WorldDrawDraft {
    fn drop(&mut self) {
        #[cfg(not(test))]
        assert!(self.terminal_is_empty(), "WorldDrawDraft reached Drop before every admitted instance reached its terminal handback");
        #[cfg(test)]
        while self.take_next().is_some() {}
    }
}

struct WorldDrawRebuildCursor {
    descriptor: WorldDrawRebuildDescriptor,
    drafts: Box<[Option<WorldDrawDraft>; WORLD_DYNAMIC_DRAW_CAPACITY]>,
    admitted_draws: u16,
    admitted_instances: u32,
    admitted_bytes: u32,
    publishing: bool,
    publish_draw: u16,
    output: Option<SceneDraw3d>,
    staged: WorldDrawRegistry,
    faulted: bool,
}

impl WorldDrawRebuildCursor {
    fn new(descriptor: WorldDrawRebuildDescriptor) -> Result<Self, WorldDynamicFault> {
        if usize::from(descriptor.draw_count) > WORLD_DYNAMIC_DRAW_CAPACITY || usize::try_from(descriptor.instance_count).ok().is_none_or(|count| count > WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY) {
            return Err(WorldDynamicFault::InstanceCapacity);
        }
        if usize::try_from(descriptor.byte_count).ok().is_none_or(|bytes| bytes > WORLD_DYNAMIC_DRAW_BYTE_CAPACITY) {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        Ok(Self {
            descriptor,
            drafts: Box::new([const { None }; WORLD_DYNAMIC_DRAW_CAPACITY]),
            admitted_draws: 0,
            admitted_instances: 0,
            admitted_bytes: 0,
            publishing: false,
            publish_draw: 0,
            output: None,
            staged: WorldDrawRegistry::default(),
            faulted: false,
        })
    }

    fn admit_draw(&mut self, mesh_key: &str, mesh_version: u64, instance_count: u16) -> Result<(), WorldDynamicFault> {
        if self.publishing || self.faulted || self.admitted_draws == self.descriptor.draw_count {
            return Err(WorldDynamicFault::Closing);
        }
        if mesh_key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY || usize::from(instance_count) > WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY {
            return Err(if mesh_key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY { WorldDynamicFault::IdCapacity } else { WorldDynamicFault::InstanceCapacity });
        }
        let bytes = mesh_key.len().checked_add(std::mem::size_of::<SceneDraw3d>()).ok_or(WorldDynamicFault::ByteCapacity)?;
        let next = usize::try_from(self.admitted_bytes).unwrap_or(usize::MAX).checked_add(bytes).ok_or(WorldDynamicFault::ByteCapacity)?;
        if next > usize::try_from(self.descriptor.byte_count).unwrap_or(0) {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        let slot = usize::from(self.admitted_draws);
        self.drafts[slot] = Some(WorldDrawDraft::new(mesh_key.to_owned(), mesh_version, usize::from(instance_count)));
        self.admitted_draws += 1;
        self.admitted_bytes = next as u32;
        Ok(())
    }

    fn admit_instance(&mut self, draw: u16, id: &str, model: Mat4, color: [f32; 4], selected: bool, hovered: bool) -> Result<(), WorldDynamicFault> {
        if self.publishing || self.faulted || id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY { WorldDynamicFault::IdCapacity } else { WorldDynamicFault::Closing });
        }
        let Some(draft) = self.drafts.get_mut(usize::from(draw)).and_then(Option::as_mut) else {
            return Err(WorldDynamicFault::StaleToken);
        };
        let bytes = id.len().checked_add(std::mem::size_of::<Instance3d>()).unwrap_or(usize::MAX);
        let next = usize::try_from(self.admitted_bytes).unwrap_or(usize::MAX).checked_add(bytes).unwrap_or(usize::MAX);
        if next > usize::try_from(self.descriptor.byte_count).unwrap_or(0) || self.admitted_instances == self.descriptor.instance_count {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        if draft.admitted == draft.slots.len() {
            return Err(WorldDynamicFault::InstanceCapacity);
        }
        let instance = Instance3d { id: id.to_owned(), model, color, selected, hovered };
        draft.push(instance).map_err(|_| WorldDynamicFault::InstanceCapacity)?;
        self.admitted_instances += 1;
        self.admitted_bytes = next as u32;
        Ok(())
    }

    fn seal(&mut self) -> Result<(), WorldDynamicFault> {
        if self.admitted_draws != self.descriptor.draw_count
            || self.admitted_instances != self.descriptor.instance_count
            || self.admitted_bytes != self.descriptor.byte_count
            || self.drafts[..usize::from(self.admitted_draws)].iter().any(|draft| draft.as_ref().is_none_or(|draft| !draft.complete()))
        {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        self.publishing = true;
        Ok(())
    }

    fn terminal_is_empty(&self) -> bool {
        self.output.as_ref().is_none_or(|draw| draw.instances.is_empty()) && self.drafts.iter().flatten().all(WorldDrawDraft::terminal_is_empty) && self.staged.is_empty()
    }
}

#[cfg(not(test))]
impl Drop for WorldDrawRebuildCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "WorldDrawRebuildCursor reached Drop before every draft, instance, output, and staged draw reached terminal handback");
    }
}

impl<'a> IntoIterator for &'a WorldDrawRegistry {
    type Item = &'a SceneDraw3d;
    type IntoIter = Box<dyn Iterator<Item = &'a SceneDraw3d> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl<'a> IntoIterator for &'a mut WorldDrawRegistry {
    type Item = &'a mut SceneDraw3d;
    type IntoIter = Box<dyn Iterator<Item = &'a mut SceneDraw3d> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter_mut())
    }
}

impl Index<usize> for WorldDrawRegistry {
    type Output = SceneDraw3d;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("world draw index")
    }
}

impl IndexMut<usize> for WorldDrawRegistry {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("world draw index")
    }
}

#[cfg(not(test))]
impl Drop for WorldDrawRegistry {
    fn drop(&mut self) {
        assert!(self.is_empty(), "world draw registry reached Drop before its exact terminal-empty witness");
    }
}

enum WorldOpaqueOwner {
    Draw(WorldDynamicEntry<SceneDraw3d>),
    ReferencePixels(WorldDynamicEntry<(u32, u32, Vec<u8>)>),
    PaintPixels(WorldDynamicEntry<(u32, u32, Vec<u8>)>),
}

struct WorldOpaqueQuarantine<const N: usize> {
    slots: Box<[Option<WorldOpaqueOwner>; N]>,
    len: u16,
    saturated: u64,
}

impl<const N: usize> Default for WorldOpaqueQuarantine<N> {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; N]), len: 0, saturated: 0 }
    }
}

impl<const N: usize> WorldOpaqueQuarantine<N> {
    fn admit(&mut self, owner: WorldOpaqueOwner) -> Result<WorldDynamicToken, WorldOpaqueOwner> {
        let slot = usize::from(self.len);
        if slot == N {
            self.saturated = self.saturated.saturating_add(1);
            return Err(owner);
        }
        self.slots[slot] = Some(owner);
        self.len += 1;
        Ok(WorldDynamicToken { slot: slot as u16, epoch: 1 })
    }

    #[cfg(test)]
    fn take_one(&mut self) -> Option<WorldOpaqueOwner> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.slots[usize::from(self.len)].take()
    }
}

static WORLD_OPAQUE_QUARANTINE: LazyLock<Mutex<WorldOpaqueQuarantine<WORLD_OPAQUE_QUARANTINE_CAPACITY>>> = LazyLock::new(|| Mutex::new(WorldOpaqueQuarantine::default()));

fn quarantine_world_owner(owner: WorldOpaqueOwner) -> Result<WorldDynamicToken, WorldOpaqueOwner> {
    let Ok(mut quarantine) = WORLD_OPAQUE_QUARANTINE.lock() else {
        return Err(owner);
    };
    quarantine.admit(owner)
}

pub fn world3d_opaque_quarantine_status() -> (usize, u64) {
    WORLD_OPAQUE_QUARANTINE.lock().map(|quarantine| (usize::from(quarantine.len), quarantine.saturated)).unwrap_or((WORLD_OPAQUE_QUARANTINE_CAPACITY, u64::MAX))
}

pub struct World3dState {
    pub surface_id: String,
    pub controller_id: String,
    pub bounds: Rect,
    pub pick_bounds: Rect,
    pub orbit: OrbitController,
    meshes: WorldDynamicRegistry<Mesh3dLease, WORLD_DYNAMIC_MESH_CAPACITY>,
    mesh_versions: WorldDynamicRegistry<u64, WORLD_DYNAMIC_MESH_CAPACITY>,
    draws: WorldDrawRegistry,
    pub selection_method: String,
    pub local_hover_id: Option<String>,
    pub pending_glb_urls: HashSet<String>,
    pub marquee_points: Vec<[f32; 2]>,
    pub marquee_active: bool,
    scene_camera_json: Option<String>,
    scene_meshes_json: Option<String>,
    scene_instances_json: Option<String>,
    scene_selection_json: Option<String>,
    scene_vortices_json: Option<String>,
    scene_attractions_json: Option<String>,
    scene_target_volumes_json: Option<String>,
    scene_references_json: Option<String>,
    scene_brush_preview_json: Option<String>,
    scene_interaction_json: Option<String>,
    scene_engagement_preview_json: Option<String>,
    /// 🪟️ The app-bound `InteractionDefinition` id for this window (see `World3dScene.domain_id`'s
    /// doc comment) — `None` means this window binds no app domain, so plain picks/hover target the
    /// OS's own shared `world` board domain (see `resolved_domain_id`).
    bound_domain_id: Option<String>,
    /// 🎯️ The bound domain's granularity id for a plain (non-component) pick/hover hit.
    bound_domain_granularity_id: Option<String>,
    vortices: Vec<WorldVortexRecord>,
    attractions: Vec<WorldAttractionRecord>,
    target_volumes: Vec<WorldTargetVolumeRecord>,
    references: Vec<WorldReferenceRecord>,
    brush_preview: Option<WorldBrushPreviewRecord>,
    active_utility: String,
    hovered_vortex_id: Option<String>,
    drag_object_id: Option<String>,
    drag_object_z: f32,
    drag_last_position: Option<[f32; 3]>,
    selected_ids: Vec<String>,
    transform_mode: String,
    gumball_handle: Option<GumballHandle>,
    gumball_pivot: Vec3,
    gumball_drag_anchor: f32,
    gumball_drag_start_vec: Vec3,
    gumball_preview_translate: Vec3,
    gumball_preview_angle: f32,
    gumball_preview_scale: Vec3,
    pending_image_urls: HashSet<String>,
    reference_pixels: WorldDynamicRegistry<(u32, u32, Vec<u8>), WORLD_DYNAMIC_PIXEL_CAPACITY>,
    granularity: String,
    component_ids: Vec<String>,
    interaction_mode: String,
    gumball_target: Option<[f32; 3]>,
    marquee_preview_ids: Vec<String>,
    paint_stroke_active: bool,
    hovered_component_id: Option<String>,
    hovered_component_object_id: Option<String>,
    hovered_component_mode: Option<String>,
    show_edges: bool,
    selection_targets: WorldSelectionTargets,
    active_object_id: Option<String>,
    press_object_id: Option<String>,
    mesh_paint_textures: WorldDynamicRegistry<(u32, u32, Vec<u8>), WORLD_DYNAMIC_PIXEL_CAPACITY>,
    lod: WorldLodRecord,
    chunking: Option<WorldChunkingRecord>,
    visible_chunks: HashSet<(i64, i64, i64)>,
    mesh_lod_catalog: HashMap<String, Vec<WorldMeshLodEntry>>,
    mesh_url_fallback: HashMap<String, String>,
    instance_positions: HashMap<String, [f64; 3]>,
    parsed_instances: Vec<WorldInstanceRecord>,
    mesh_pool: RefCountPool<String>,
    mesh_source_urls: HashMap<String, String>,
    resolved_lod_pick: Option<f64>,
    scene_lod_json: Option<String>,
    scene_chunking_json: Option<String>,
    environment: WorldEnvironmentRecord,
    scene_environment_json: Option<String>,
    terrain_style: Option<WorldTerrainStyle>,
    scene_terrain_json: Option<String>,
    terrain_applied_signature: Option<(String, f64, f64, f64)>,
    terrain_session: TerrainSessionCore,
    terrain_visible_tiles: HashSet<(u32, u32, u32)>,
    terrain_built_tiles: HashSet<(u32, u32, u32)>,
    pending_terrain_tile_urls: HashMap<String, (u32, u32, u32)>,
    right_press_point: Option<[f32; 2]>,
    gizmo_hovered_tip: Option<usize>,
    interaction_revision: u64,
    interaction_authority: Option<WorldInteractionAuthority>,
    interaction_meshes: WorldInteractionMeshRegistry,
    interaction_objects: WorldInteractionObjectRegistry,
    snapshot_lease: Option<World3dSnapshotLease>,
    snapshot_apply: Option<World3dSnapshotApplyCursor>,
    snapshot_fault: Option<World3dSnapshotFault>,
    prepared_status: [Option<World3dPreparedStatus>; 2],
    dynamic_blocked_owner: Option<WorldOpaqueOwner>,
    dynamic_mesh_close: Option<WorldDynamicEntry<Mesh3dLease>>,
    dynamic_blocked_mesh: Option<WorldDynamicEntry<Mesh3dLease>>,
    placeholder_generation: u64,
    placeholder_build: Option<WorldPlaceholderMeshCursor>,
    terrain_build: Option<WorldTerrainMeshCursor>,
    terrain_revision: u64,
    face_overlay_build: Option<WorldFaceOverlayMeshCursor>,
    face_overlay_generation: Option<u64>,
    face_overlay_retired_generation: Option<u64>,
    face_overlay_colors: [Option<[f32; 4]>; 3],
    face_overlay_applied_revision: u64,
    face_overlay_applied_draw_generation: u64,
    draw_generation: u64,
    draw_rebuild: Option<WorldDrawRebuildCursor>,
    retired_draws: Option<WorldDrawRegistry>,
    asset_generation: u64,
    asset_io: WorldAssetIoAuthority,
    dynamic_retirement: Option<World3dDynamicRetirement>,
}

impl World3dState {
    pub fn new(surface_id: String, controller_id: String) -> Self {
        Self {
            surface_id,
            controller_id,
            bounds: Rect::default(),
            pick_bounds: Rect::default(),
            orbit: OrbitController::default(),
            meshes: WorldDynamicRegistry::default(),
            mesh_versions: WorldDynamicRegistry::default(),
            draws: WorldDrawRegistry::default(),
            selection_method: "rectangle".into(),
            local_hover_id: None,
            pending_glb_urls: HashSet::new(),
            marquee_points: Vec::new(),
            marquee_active: false,
            scene_camera_json: None,
            scene_meshes_json: None,
            scene_instances_json: None,
            scene_selection_json: None,
            scene_vortices_json: None,
            scene_attractions_json: None,
            scene_target_volumes_json: None,
            scene_references_json: None,
            scene_brush_preview_json: None,
            scene_interaction_json: None,
            scene_engagement_preview_json: None,
            bound_domain_id: None,
            bound_domain_granularity_id: None,
            vortices: Vec::new(),
            attractions: Vec::new(),
            target_volumes: Vec::new(),
            references: Vec::new(),
            brush_preview: None,
            active_utility: "select".into(),
            hovered_vortex_id: None,
            drag_object_id: None,
            drag_object_z: 0.0,
            drag_last_position: None,
            selected_ids: Vec::new(),
            transform_mode: "translate".into(),
            gumball_handle: None,
            gumball_pivot: Vec3::ZERO,
            gumball_drag_anchor: 0.0,
            gumball_drag_start_vec: Vec3::ZERO,
            gumball_preview_translate: Vec3::ZERO,
            gumball_preview_angle: 0.0,
            gumball_preview_scale: Vec3::new(1.0, 1.0, 1.0),
            pending_image_urls: HashSet::new(),
            reference_pixels: WorldDynamicRegistry::default(),
            granularity: "object".into(),
            component_ids: Vec::new(),
            interaction_mode: "model".into(),
            gumball_target: None,
            marquee_preview_ids: Vec::new(),
            paint_stroke_active: false,
            hovered_component_id: None,
            hovered_component_object_id: None,
            hovered_component_mode: None,
            show_edges: true,
            selection_targets: WorldSelectionTargets::default(),
            active_object_id: None,
            press_object_id: None,
            mesh_paint_textures: WorldDynamicRegistry::default(),
            lod: WorldLodRecord { automatic: true, manual: default_manual_lod(), distance_reference: default_distance_reference(), depth_variable: false, grid_factor: default_grid_factor(), show_grid: true, grid_datum: Some([0.0, 0.0, 0.0]) },
            chunking: None,
            visible_chunks: HashSet::new(),
            mesh_lod_catalog: HashMap::new(),
            mesh_url_fallback: HashMap::new(),
            instance_positions: HashMap::new(),
            parsed_instances: Vec::new(),
            mesh_pool: RefCountPool::new(),
            mesh_source_urls: HashMap::new(),
            resolved_lod_pick: None,
            scene_lod_json: None,
            scene_chunking_json: None,
            environment: WorldEnvironmentRecord::default(),
            scene_environment_json: None,
            terrain_style: None,
            scene_terrain_json: None,
            terrain_applied_signature: None,
            terrain_session: TerrainSessionCore::default(),
            terrain_visible_tiles: HashSet::new(),
            terrain_built_tiles: HashSet::new(),
            pending_terrain_tile_urls: HashMap::new(),
            right_press_point: None,
            gizmo_hovered_tip: None,
            interaction_revision: 0,
            interaction_authority: Some(WorldInteractionAuthority::default()),
            interaction_meshes: WorldInteractionMeshRegistry::default(),
            interaction_objects: WorldInteractionObjectRegistry::default(),
            snapshot_lease: None,
            snapshot_apply: None,
            snapshot_fault: None,
            prepared_status: [None; 2],
            dynamic_blocked_owner: None,
            dynamic_mesh_close: None,
            dynamic_blocked_mesh: None,
            placeholder_generation: 0,
            placeholder_build: None,
            terrain_build: None,
            terrain_revision: 0,
            face_overlay_build: None,
            face_overlay_generation: None,
            face_overlay_retired_generation: None,
            face_overlay_colors: [None; 3],
            face_overlay_applied_revision: u64::MAX,
            face_overlay_applied_draw_generation: u64::MAX,
            draw_generation: 0,
            draw_rebuild: None,
            retired_draws: None,
            asset_generation: 0,
            asset_io: WorldAssetIoAuthority::default(),
            dynamic_retirement: None,
        }
    }

    pub fn dynamic_retirement_is_idle(&self) -> bool {
        self.dynamic_retirement.is_none()
    }
}

#[cfg(not(test))]
impl Drop for World3dState {
    fn drop(&mut self) {
        assert!(world3d_dynamic_retirement_terminal_is_empty(self), "World3dState reached Drop before retained dynamic owners reached terminal empty");
    }
}
//#endregion World3dState

//#region 🧹️World3dDynamicRetirement

struct World3dDynamicRetirement {
    phase: u8,
    blocked: Option<WorldDynamicFault>,
}

impl World3dDynamicRetirement {
    fn begin(state: &mut World3dState) -> Self {
        state.meshes.begin_close();
        state.mesh_versions.begin_close();
        state.draws.begin_close();
        state.reference_pixels.begin_close();
        state.mesh_paint_textures.begin_close();
        state.asset_io.begin_close();
        Self { phase: 0, blocked: None }
    }

    fn step(&mut self, state: &mut World3dState) -> bool {
        self.blocked = None;
        if state.dynamic_mesh_close.is_none() {
            if let Some(entry) = state.dynamic_blocked_mesh.take() {
                state.dynamic_mesh_close = Some(entry);
                return false;
            }
        }
        if let Some(owner) = state.dynamic_blocked_owner.take() {
            if let Err(owner) = quarantine_world_owner(owner) {
                state.dynamic_blocked_owner = Some(owner);
                self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
            }
            return false;
        }
        match self.phase {
            0 => {
                if let Some(entry) = state.dynamic_mesh_close.as_mut() {
                    match mesh3d_begin_close(entry.value) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => {}
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => {
                            entry.id.clear();
                            state.dynamic_mesh_close = None;
                        }
                        Err(_) => {
                            self.blocked = Some(WorldDynamicFault::Closing);
                        }
                    }
                    if state.dynamic_mesh_close.as_ref().is_some_and(|entry| mesh3d_close_step(entry.value).is_ok_and(|complete| complete)) {
                        state.dynamic_mesh_close.as_mut().expect("mesh close owner retained above").id.clear();
                        state.dynamic_mesh_close = None;
                    }
                    return false;
                }
                let Some(entry) = state.meshes.take_one() else {
                    self.phase = 1;
                    return false;
                };
                state.dynamic_mesh_close = Some(entry);
            }
            1 => {
                if state.mesh_versions.take_one().is_some() {
                    return false;
                }
                self.phase = 2;
                return false;
            }
            2 => {
                let Some(entry) = state.draws.take_last() else {
                    self.phase = 3;
                    return false;
                };
                if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                    let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("draw quarantine owner") };
                    state.draws.restore_last(entry);
                    self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
                }
            }
            3 => {
                let Some(entry) = state.reference_pixels.take_one() else {
                    self.phase = 4;
                    return false;
                };
                if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::ReferencePixels(entry)) {
                    let WorldOpaqueOwner::ReferencePixels(entry) = owner else { unreachable!("reference pixel quarantine owner") };
                    state.reference_pixels.restore(entry);
                    self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
                }
            }
            4 => {
                let Some(entry) = state.mesh_paint_textures.take_one() else {
                    self.phase = 5;
                    return false;
                };
                if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::PaintPixels(entry)) {
                    let WorldOpaqueOwner::PaintPixels(entry) = owner else { unreachable!("paint pixel quarantine owner") };
                    state.mesh_paint_textures.restore(entry);
                    self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
                }
            }
            _ => return true,
        }
        false
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase >= 5 && self.blocked.is_none()
    }
}

pub fn begin_world3d_dynamic_retirement(state: &mut World3dState) -> bool {
    if state.dynamic_retirement.is_some() {
        return false;
    }
    state.snapshot_fault = Some(World3dSnapshotFault::Closing);
    state.dynamic_retirement = Some(World3dDynamicRetirement::begin(state));
    true
}

pub fn step_world3d_dynamic_retirement(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    if let Some(cursor) = state.placeholder_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.placeholder_build = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(cursor) = state.terrain_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.terrain_build = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(cursor) = state.face_overlay_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.face_overlay_build = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(index) = state.face_overlay_colors.iter().position(Option::is_some) {
        state.face_overlay_colors[index] = None;
        context.consume_fuel(1);
        return false;
    }
    if state.face_overlay_generation.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if state.face_overlay_retired_generation.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if !state.asset_io.terminal_is_empty() {
        let _ = state.asset_io.close_step();
        context.consume_fuel(1);
        return false;
    }
    if !close_world3d_draw_rebuild_step(state, context) {
        return false;
    }
    let Some(mut retirement) = state.dynamic_retirement.take() else {
        return true;
    };
    let complete = retirement.step(state);
    context.consume_fuel(1);
    if complete && retirement.terminal_is_empty() {
        return true;
    }
    state.dynamic_retirement = Some(retirement);
    false
}

pub fn world3d_dynamic_retirement_terminal_is_empty(state: &World3dState) -> bool {
    state.dynamic_retirement.is_none()
        && state.dynamic_blocked_owner.is_none()
        && state.dynamic_mesh_close.is_none()
        && state.dynamic_blocked_mesh.is_none()
        && state.placeholder_build.is_none()
        && state.terrain_build.is_none()
        && state.face_overlay_build.is_none()
        && state.face_overlay_generation.is_none()
        && state.face_overlay_retired_generation.is_none()
        && state.face_overlay_colors.iter().all(Option::is_none)
        && world3d_draw_rebuild_terminal_is_empty(state)
        && state.meshes.is_empty()
        && state.mesh_versions.is_empty()
        && state.draws.is_empty()
        && state.reference_pixels.is_empty()
        && state.mesh_paint_textures.is_empty()
        && state.asset_io.terminal_is_empty()
}

pub fn world3d_cursor_work_pending(state: &World3dState) -> bool {
    state.placeholder_build.is_some() || state.terrain_build.is_some() || state.face_overlay_build.is_some()
}

//#endregion 🧹️World3dDynamicRetirement

//#region 🧱️WorldDrawRebuild
pub fn begin_world3d_draw_rebuild(state: &mut World3dState, descriptor: WorldDrawRebuildDescriptor) -> Result<(), WorldDynamicFault> {
    if state.dynamic_retirement.is_some() || state.draw_rebuild.is_some() || state.retired_draws.is_some() || state.dynamic_blocked_owner.is_some() {
        return Err(WorldDynamicFault::Closing);
    }
    if descriptor.revision != state.interaction_revision || descriptor.generation != state.draw_generation.wrapping_add(1) {
        return Err(WorldDynamicFault::StaleToken);
    }
    state.draw_rebuild = Some(WorldDrawRebuildCursor::new(descriptor)?);
    Ok(())
}

pub fn world3d_draw_rebuild_admit_draw(state: &mut World3dState, mesh_key: &str, mesh_version: u64, instance_count: u16) -> Result<(), WorldDynamicFault> {
    let Some(cursor) = state.draw_rebuild.as_mut() else {
        return Err(WorldDynamicFault::StaleToken);
    };
    cursor.admit_draw(mesh_key, mesh_version, instance_count)
}

pub fn world3d_draw_rebuild_admit_instance(state: &mut World3dState, draw: u16, id: &str, model: Mat4, color: [f32; 4], selected: bool, hovered: bool) -> Result<(), WorldDynamicFault> {
    let Some(cursor) = state.draw_rebuild.as_mut() else {
        return Err(WorldDynamicFault::StaleToken);
    };
    cursor.admit_instance(draw, id, model, color, selected, hovered)
}

pub fn world3d_draw_rebuild_seal(state: &mut World3dState) -> Result<(), WorldDynamicFault> {
    state.draw_rebuild.as_mut().ok_or(WorldDynamicFault::StaleToken)?.seal()
}

pub fn step_world3d_draw_rebuild(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> WorldDrawRebuildStep {
    if context.should_yield() {
        return WorldDrawRebuildStep::Pending;
    }
    if let Some(retired) = state.retired_draws.as_mut() {
        let Some(entry) = retired.take_last() else {
            state.retired_draws = None;
            context.consume_fuel(1);
            return WorldDrawRebuildStep::Pending;
        };
        if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
            let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("retired draw owner") };
            state.retired_draws.as_mut().expect("retired draw registry remains owned").restore_last(entry);
            mark_world_dynamic_fault(state, WorldDynamicFault::QuarantineCapacity);
            return WorldDrawRebuildStep::Fault;
        }
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    let Some(mut cursor) = state.draw_rebuild.take() else {
        return WorldDrawRebuildStep::Complete;
    };
    if cursor.descriptor.revision != state.interaction_revision || cursor.descriptor.generation != state.draw_generation.wrapping_add(1) {
        state.draw_rebuild = Some(cursor);
        return WorldDrawRebuildStep::Stale;
    }
    if !cursor.publishing || cursor.faulted {
        state.draw_rebuild = Some(cursor);
        return WorldDrawRebuildStep::Pending;
    }
    if let Some(output) = cursor.output.as_mut() {
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].as_mut().expect("publishing draw draft");
        if let Some(instance) = draft.take_next() {
            output.instances.push(instance);
            state.draw_rebuild = Some(cursor);
            context.consume_fuel(1);
            return WorldDrawRebuildStep::Pending;
        }
        let output = cursor.output.take().expect("completed draw output");
        if let Err(rejected) = cursor.staged.push_prevalidated(output) {
            retain_world_blocked_owner(state, WorldOpaqueOwner::Draw(WorldDynamicEntry { id: rejected.id, epoch: 0, value: rejected.value }));
            cursor.faulted = true;
            state.draw_rebuild = Some(cursor);
            mark_world_dynamic_fault(state, rejected.fault);
            return WorldDrawRebuildStep::Fault;
        }
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].take().expect("terminal draw draft");
        assert!(draft.terminal_is_empty());
        drop(draft);
        cursor.publish_draw += 1;
        state.draw_rebuild = Some(cursor);
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    if cursor.publish_draw < cursor.descriptor.draw_count {
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].as_ref().expect("sealed draw draft");
        cursor.output = Some(SceneDraw3d { mesh_key: draft.mesh_key.clone(), mesh_version: draft.mesh_version, instances: Vec::with_capacity(draft.admitted) });
        state.draw_rebuild = Some(cursor);
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    let staged = std::mem::take(&mut cursor.staged);
    let previous = std::mem::replace(&mut state.draws, staged);
    if !previous.is_empty() {
        state.retired_draws = Some(previous);
    }
    state.draw_generation = cursor.descriptor.generation;
    assert!(cursor.terminal_is_empty());
    context.consume_fuel(1);
    WorldDrawRebuildStep::Complete
}

pub fn close_world3d_draw_rebuild_step(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    if let Some(retired) = state.retired_draws.as_mut() {
        if let Some(entry) = retired.take_last() {
            if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("retired draw owner") };
                retired.restore_last(entry);
                return false;
            }
            context.consume_fuel(1);
            return false;
        }
        state.retired_draws = None;
        context.consume_fuel(1);
        return false;
    }
    let Some(cursor) = state.draw_rebuild.as_mut() else {
        return true;
    };
    if let Some(output) = cursor.output.as_mut() {
        if output.instances.pop().is_some() {
            context.consume_fuel(1);
            return false;
        }
        cursor.output = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(draft) = cursor.drafts.iter_mut().flatten().find(|draft| !draft.terminal_is_empty()) {
        drop(draft.take_next());
        context.consume_fuel(1);
        return false;
    }
    if let Some(slot) = cursor.drafts.iter().position(Option::is_some) {
        let draft = cursor.drafts[slot].take().expect("terminal draft shell");
        assert!(draft.terminal_is_empty());
        drop(draft);
        context.consume_fuel(1);
        return false;
    }
    if let Some(entry) = cursor.staged.take_last() {
        if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
            let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("staged draw owner") };
            cursor.staged.restore_last(entry);
            return false;
        }
        context.consume_fuel(1);
        return false;
    }
    assert!(cursor.terminal_is_empty());
    state.draw_rebuild = None;
    context.consume_fuel(1);
    true
}

pub fn world3d_draw_rebuild_terminal_is_empty(state: &World3dState) -> bool {
    state.draw_rebuild.is_none() && state.retired_draws.is_none()
}
//#endregion 🧱️WorldDrawRebuild

//#region 🧵️WorldInteractionTransaction
pub const WORLD_INTERACTION_ITEM_CAPACITY: usize = 256;
pub const WORLD_INTERACTION_BYTE_CAPACITY: usize = 16 * 1024;
pub const WORLD_INTERACTION_INTENT_CAPACITY: usize = 64;
pub const WORLD_INTERACTION_MESH_CAPACITY: usize = 256;
pub const WORLD_INTERACTION_OBJECT_CAPACITY: usize = 1024;
pub const WORLD_INTERACTION_MARQUEE_POINT_CAPACITY: usize = 256;
pub const WORLD_INTERACTION_ID_BYTE_CAPACITY: usize = 256;
pub const WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY: usize = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorldInteractionId {
    bytes: [u8; WORLD_INTERACTION_ID_BYTE_CAPACITY],
    len: u16,
}

impl WorldInteractionId {
    fn new(value: &str) -> Option<Self> {
        if value.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY {
            return None;
        }
        let mut bytes = [0; WORLD_INTERACTION_ID_BYTE_CAPACITY];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Some(Self { bytes, len: value.len() as u16 })
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("world registry id originates from UTF-8")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorldInteractionMeshToken {
    slot: u16,
    generation: u64,
}

#[derive(Clone, Copy)]
struct WorldInteractionMeshSlot {
    id: WorldInteractionId,
    generation: u64,
    version: u64,
    vertices: u32,
    triangles: u32,
    edges: u32,
    topology_bytes: u32,
}

struct WorldInteractionMeshRegistry {
    slots: Box<[Option<WorldInteractionMeshSlot>; WORLD_INTERACTION_MESH_CAPACITY]>,
    epochs: Box<[u64; WORLD_INTERACTION_MESH_CAPACITY]>,
    faulted: bool,
}

#[derive(Clone, Copy)]
struct WorldInteractionMeshPlan {
    slot: u16,
    epoch: u64,
    existing_generation: Option<u64>,
    value: WorldInteractionMeshSlot,
}

fn mesh3d_schema_bytes(schema: Mesh3dSchema) -> Option<usize> {
    usize::try_from(schema.vertices)
        .ok()?
        .checked_mul(24)?
        .checked_add(usize::try_from(schema.indices).ok()?.checked_mul(4)?)?
        .checked_add(usize::try_from(schema.face_ids).ok()?.checked_mul(4)?)?
        .checked_add(usize::try_from(schema.vertex_ids).ok()?.checked_mul(4)?)?
        .checked_add(usize::try_from(schema.edges).ok()?.checked_mul(24)?)?
        .checked_add(usize::try_from(schema.edge_ids).ok()?.checked_mul(4)?)?
        .checked_add(usize::try_from(schema.uvs).ok()?.checked_mul(8)?)?
        .checked_add(usize::try_from(schema.colors).ok()?.checked_mul(16)?)
}

impl Default for WorldInteractionMeshRegistry {
    fn default() -> Self {
        Self { slots: Box::new([None; WORLD_INTERACTION_MESH_CAPACITY]), epochs: Box::new([0; WORLD_INTERACTION_MESH_CAPACITY]), faulted: false }
    }
}

impl WorldInteractionMeshRegistry {
    fn hash(id: &str) -> usize {
        (id.as_bytes().iter().fold(2_166_136_261u64, |hash, byte| hash.wrapping_mul(16_777_619) ^ u64::from(*byte)) % WORLD_INTERACTION_MESH_CAPACITY as u64) as usize
    }

    fn plan_admit(&self, id: &str, version: u64, mesh: Mesh3dLease) -> Result<WorldInteractionMeshPlan, WorldDynamicFault> {
        let Some(id) = WorldInteractionId::new(id) else {
            return Err(WorldDynamicFault::IdCapacity);
        };
        let Ok(schema) = mesh.schema() else {
            return Err(WorldDynamicFault::StaleToken);
        };
        let vertices = schema.vertices;
        let triangles = schema.indices / 3;
        let edges = schema.edges;
        let topology_bytes = mesh3d_schema_bytes(schema);
        let Some(topology_bytes) = topology_bytes else {
            return Err(WorldDynamicFault::ByteCapacity);
        };
        if topology_bytes > WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        let start = Self::hash(id.as_str());
        let mut empty = None;
        for offset in 0..WORLD_INTERACTION_MESH_CAPACITY {
            let slot = (start + offset) % WORLD_INTERACTION_MESH_CAPACITY;
            match self.slots[slot] {
                Some(existing) if existing.id == id => {
                    if existing.version == version && existing.vertices == vertices && existing.triangles == triangles && existing.edges == edges && existing.topology_bytes == topology_bytes as u32 {
                        return Ok(WorldInteractionMeshPlan { slot: slot as u16, epoch: self.epochs[slot], existing_generation: Some(existing.generation), value: existing });
                    }
                    let generation = self.epochs[slot].checked_add(1).ok_or(WorldDynamicFault::StaleToken)?;
                    return Ok(WorldInteractionMeshPlan {
                        slot: slot as u16,
                        epoch: self.epochs[slot],
                        existing_generation: Some(existing.generation),
                        value: WorldInteractionMeshSlot { id, generation, version, vertices, triangles, edges, topology_bytes: topology_bytes as u32 },
                    });
                }
                None => {
                    empty = Some(slot);
                    break;
                }
                _ => {}
            }
        }
        let slot = empty.ok_or(WorldDynamicFault::RegistryCapacity)?;
        let generation = self.epochs[slot].checked_add(1).ok_or(WorldDynamicFault::StaleToken)?;
        Ok(WorldInteractionMeshPlan { slot: slot as u16, epoch: self.epochs[slot], existing_generation: None, value: WorldInteractionMeshSlot { id, generation, version, vertices, triangles, edges, topology_bytes: topology_bytes as u32 } })
    }

    fn commit_admit(&mut self, plan: WorldInteractionMeshPlan) -> Result<WorldInteractionMeshToken, WorldDynamicFault> {
        let slot = usize::from(plan.slot);
        let current = self.slots.get(slot).copied().ok_or(WorldDynamicFault::StaleToken)?;
        let matches = match (plan.existing_generation, current) {
            (Some(generation), Some(current)) => current.generation == generation && self.epochs[slot] == plan.epoch,
            (None, None) => self.epochs[slot] == plan.epoch,
            _ => false,
        };
        if !matches {
            return Err(WorldDynamicFault::StaleToken);
        }
        self.epochs[slot] = plan.value.generation;
        self.slots[slot] = Some(plan.value);
        Ok(WorldInteractionMeshToken { slot: plan.slot, generation: plan.value.generation })
    }

    fn admit(&mut self, id: &str, version: u64, mesh: Mesh3dLease) -> Option<WorldInteractionMeshToken> {
        let plan = match self.plan_admit(id, version, mesh) {
            Ok(plan) => plan,
            Err(_) => {
                self.faulted = true;
                return None;
            }
        };
        match self.commit_admit(plan) {
            Ok(token) => Some(token),
            Err(_) => {
                self.faulted = true;
                None
            }
        }
    }

    fn resolve(&self, token: WorldInteractionMeshToken) -> Option<&WorldInteractionMeshSlot> {
        let slot = self.slots.get(usize::from(token.slot))?.as_ref()?;
        (slot.generation == token.generation).then_some(slot)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldInteractionObjectKind {
    Instance,
    Vortex,
    Reference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorldInteractionObjectToken {
    slot: u16,
    generation: u64,
    revision: u64,
}

#[derive(Clone, Copy)]
struct WorldInteractionObjectSlot {
    id: WorldInteractionId,
    kind: WorldInteractionObjectKind,
    generation: u64,
    revision: u64,
    mesh: Option<WorldInteractionMeshToken>,
    model: Mat4,
    values: [f32; 8],
}

struct WorldInteractionObjectRegistry {
    slots: Box<[Option<WorldInteractionObjectSlot>; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    epochs: Box<[u64; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    instance_order: Box<[Option<WorldInteractionObjectToken>; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    instance_len: u16,
    revision: u64,
    faulted: bool,
}

impl Default for WorldInteractionObjectRegistry {
    fn default() -> Self {
        Self {
            slots: Box::new([None; WORLD_INTERACTION_OBJECT_CAPACITY]),
            epochs: Box::new([0; WORLD_INTERACTION_OBJECT_CAPACITY]),
            instance_order: Box::new([None; WORLD_INTERACTION_OBJECT_CAPACITY]),
            instance_len: 0,
            revision: u64::MAX,
            faulted: false,
        }
    }
}

impl WorldInteractionObjectRegistry {
    fn hash(kind: WorldInteractionObjectKind, id: &str) -> usize {
        let seed = match kind {
            WorldInteractionObjectKind::Instance => 0x11u64,
            WorldInteractionObjectKind::Vortex => 0x22,
            WorldInteractionObjectKind::Reference => 0x33,
        };
        (id.as_bytes().iter().fold(2_166_136_261u64 ^ seed, |hash, byte| hash.wrapping_mul(16_777_619) ^ u64::from(*byte)) % WORLD_INTERACTION_OBJECT_CAPACITY as u64) as usize
    }

    fn admit(&mut self, revision: u64, kind: WorldInteractionObjectKind, id: &str, mesh: Option<WorldInteractionMeshToken>, model: Mat4, values: [f32; 8]) -> Option<WorldInteractionObjectToken> {
        let Some(id) = WorldInteractionId::new(id) else {
            self.faulted = true;
            return None;
        };
        let start = Self::hash(kind, id.as_str());
        let mut reusable = None;
        for offset in 0..WORLD_INTERACTION_OBJECT_CAPACITY {
            let slot = (start + offset) % WORLD_INTERACTION_OBJECT_CAPACITY;
            match self.slots[slot] {
                Some(existing) if existing.revision == revision && existing.kind == kind && existing.id == id => {
                    if existing.mesh == mesh && existing.model.cols == model.cols && existing.values == values {
                        return Some(WorldInteractionObjectToken { slot: slot as u16, generation: existing.generation, revision });
                    }
                    reusable = Some(slot);
                    break;
                }
                Some(existing) if existing.revision != revision => {
                    reusable.get_or_insert(slot);
                }
                None => {
                    reusable = Some(slot);
                    break;
                }
                _ => {}
            }
        }
        let Some(slot) = reusable else {
            self.faulted = true;
            return None;
        };
        let Some(generation) = self.epochs[slot].checked_add(1) else {
            self.faulted = true;
            return None;
        };
        self.epochs[slot] = generation;
        self.slots[slot] = Some(WorldInteractionObjectSlot { id, kind, generation, revision, mesh, model, values });
        Some(WorldInteractionObjectToken { slot: slot as u16, generation, revision })
    }

    fn resolve(&self, token: WorldInteractionObjectToken) -> Option<&WorldInteractionObjectSlot> {
        let slot = self.slots.get(usize::from(token.slot))?.as_ref()?;
        (slot.generation == token.generation && slot.revision == token.revision).then_some(slot)
    }

    fn begin_instance_order(&mut self) {
        self.instance_len = 0;
    }

    fn push_instance_order(&mut self, token: WorldInteractionObjectToken) -> bool {
        let index = usize::from(self.instance_len);
        if index == WORLD_INTERACTION_OBJECT_CAPACITY {
            self.faulted = true;
            return false;
        }
        self.instance_order[index] = Some(token);
        self.instance_len += 1;
        true
    }

    fn terminal_for_revision(&self, revision: u64) -> bool {
        self.revision == revision && !self.faulted
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldInteractionRegistryBuildPhase {
    Instances,
    Vortices,
    References,
    Complete,
}

struct WorldInteractionRegistryBuildCursor {
    revision: u64,
    initialized: bool,
    phase: WorldInteractionRegistryBuildPhase,
    draw: usize,
    instance: usize,
    mesh_probe: u16,
    vortex: usize,
    reference: usize,
    faulted: bool,
}

impl WorldInteractionRegistryBuildCursor {
    fn new(revision: u64) -> Self {
        Self { revision, initialized: false, phase: WorldInteractionRegistryBuildPhase::Instances, draw: 0, instance: 0, mesh_probe: 0, vortex: 0, reference: 0, faulted: false }
    }

    fn step(&mut self, state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted || state.interaction_objects.faulted || state.interaction_meshes.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != state.interaction_revision {
            return WorldInteractionStep::Stale;
        }
        if !self.initialized {
            state.interaction_objects.begin_instance_order();
            self.initialized = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        match self.phase {
            WorldInteractionRegistryBuildPhase::Instances => {
                let Some(draw) = state.draws.get(self.draw) else {
                    self.phase = WorldInteractionRegistryBuildPhase::Vortices;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                if usize::from(self.mesh_probe) == WORLD_INTERACTION_MESH_CAPACITY || draw.mesh_key.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                let mesh_slot = (WorldInteractionMeshRegistry::hash(&draw.mesh_key) + usize::from(self.mesh_probe)) % WORLD_INTERACTION_MESH_CAPACITY;
                let mesh = match state.interaction_meshes.slots[mesh_slot] {
                    Some(entry) if entry.id.as_str() == draw.mesh_key && entry.version == draw.mesh_version => Some(WorldInteractionMeshToken { slot: mesh_slot as u16, generation: entry.generation }),
                    Some(_) => {
                        self.mesh_probe += 1;
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    }
                    None => {
                        self.faulted = true;
                        return WorldInteractionStep::Fault;
                    }
                };
                let Some(instance) = draw.instances.get(self.instance) else {
                    self.draw += 1;
                    self.instance = 0;
                    self.mesh_probe = 0;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let translation = instance.model.cols[3];
                let Some(token) = state.interaction_objects.admit(
                    self.revision,
                    WorldInteractionObjectKind::Instance,
                    &instance.id,
                    mesh,
                    instance.model,
                    [self.draw as f32, self.instance as f32, if instance.selected { 1.0 } else { 0.0 }, translation[0], translation[1], translation[2], 0.0, 0.0],
                ) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                if !state.interaction_objects.push_instance_order(token) {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                self.instance += 1;
            }
            WorldInteractionRegistryBuildPhase::Vortices => {
                let Some(vortex) = state.vortices.get(self.vortex) else {
                    self.phase = WorldInteractionRegistryBuildPhase::References;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
                let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
                let values = [position[0] as f32, position[1] as f32, position[2] as f32, direction[0] as f32, direction[1] as f32, direction[2] as f32, vortex.radius.unwrap_or(0.36) as f32, 0.0];
                if state.interaction_objects.admit(self.revision, WorldInteractionObjectKind::Vortex, &vortex.full_id, None, Mat4::identity(), values).is_none() {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                self.vortex += 1;
            }
            WorldInteractionRegistryBuildPhase::References => {
                let Some(reference) = state.references.get(self.reference) else {
                    self.phase = WorldInteractionRegistryBuildPhase::Complete;
                    state.interaction_objects.revision = self.revision;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                self.reference += 1;
                if reference.hidden.unwrap_or(false) {
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                }
                let Some(url) = reference.url.as_deref() else {
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
                let width = reference.width_world.unwrap_or(1.0) as f32;
                let aspect = reference_image_aspect(state, url);
                let values = [origin[0] as f32, origin[1] as f32, origin[2] as f32, width, width / aspect, 0.0, 0.0, 0.0];
                if state.interaction_objects.admit(self.revision, WorldInteractionObjectKind::Reference, url, None, Mat4::identity(), values).is_none() {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
            }
            WorldInteractionRegistryBuildPhase::Complete => return WorldInteractionStep::Complete,
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        self.phase = WorldInteractionRegistryBuildPhase::Complete;
        context.consume_fuel(1);
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionPhase {
    PointerMove,
    PointerButton,
    PointerDrag,
    Wheel,
    Close,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldInteractionIntent {
    pub phase: WorldInteractionPhase,
    pub generation: u64,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub delta: f32,
    pub button: i16,
    pub down: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl WorldInteractionIntent {
    pub fn pointer_move(x: f32, y: f32, dx: f32, dy: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Self {
        Self { phase: WorldInteractionPhase::PointerMove, generation: 0, x, y, dx, dy, delta: 0.0, button, down, shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
    }

    pub fn pointer_button(x: f32, y: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Self {
        Self { phase: WorldInteractionPhase::PointerButton, generation: 0, x, y, dx: 0.0, dy: 0.0, delta: 0.0, button, down, shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
    }

    pub fn wheel(x: f32, y: f32, delta: f32, modifiers: &PointerModifiers) -> Self {
        Self { phase: WorldInteractionPhase::Wheel, generation: 0, x, y, dx: 0.0, dy: 0.0, delta, button: 0, down: false, shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
    }
}

pub struct WorldInteractionIntentQueue {
    slots: Box<[Option<WorldInteractionIntent>; WORLD_INTERACTION_INTENT_CAPACITY]>,
    head: u8,
    len: u8,
    closing: bool,
}

impl Default for WorldInteractionIntentQueue {
    fn default() -> Self {
        Self { slots: Box::new([None; WORLD_INTERACTION_INTENT_CAPACITY]), head: 0, len: 0, closing: false }
    }
}

impl WorldInteractionIntentQueue {
    pub fn push(&mut self, intent: WorldInteractionIntent) -> Result<(), WorldInteractionIntent> {
        if self.closing || usize::from(self.len) == WORLD_INTERACTION_INTENT_CAPACITY {
            return Err(intent);
        }
        let index = (usize::from(self.head) + usize::from(self.len)) % WORLD_INTERACTION_INTENT_CAPACITY;
        self.slots[index] = Some(intent);
        self.len += 1;
        Ok(())
    }

    pub fn front(&self) -> Option<&WorldInteractionIntent> {
        self.slots[usize::from(self.head)].as_ref()
    }

    pub fn retire_front(&mut self, generation: u64) -> bool {
        let index = usize::from(self.head);
        let Some(intent) = self.slots[index] else {
            return false;
        };
        if intent.generation != generation {
            return false;
        }
        self.slots[index] = None;
        self.head = ((index + 1) % WORLD_INTERACTION_INTENT_CAPACITY) as u8;
        self.len -= 1;
        true
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> bool {
        if self.len == 0 {
            return true;
        }
        let index = usize::from(self.head);
        self.slots[index] = None;
        self.head = ((index + 1) % WORLD_INTERACTION_INTENT_CAPACITY) as u8;
        self.len -= 1;
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.len == 0 && self.slots.iter().all(Option::is_none)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldFlatActionKind {
    PaintAt,
    PaintStrokeBegin,
    PaintStrokeEnd,
    VortexSelect,
    VortexHover,
    SurfacePlace,
    Select,
    Hover,
    ComponentSelect,
    ComponentHover,
    Relocate,
    GumballCommit,
    BrushObject,
    ContextMenu,
    Camera,
}

#[derive(Clone, Copy, Debug)]
struct WorldInteractionSpan {
    start: u16,
    len: u16,
}

#[derive(Clone, Copy, Debug)]
struct WorldFlatAction {
    kind: WorldFlatActionKind,
    strings: [Option<WorldInteractionSpan>; 8],
    numbers: [f64; 10],
    number_len: u8,
}

pub struct WorldInteractionPlan {
    revision: u64,
    generation: u64,
    bytes: Box<[u8; WORLD_INTERACTION_BYTE_CAPACITY]>,
    byte_len: u16,
    actions: Box<[Option<WorldFlatAction>; WORLD_INTERACTION_ITEM_CAPACITY]>,
    action_len: u16,
    cursor: u16,
    faulted: bool,
}

enum WorldInteractionActive {
    Plan { plan: WorldInteractionPlan, retirement: Option<WorldInteractionAuthorityStep> },
    Pick { cursor: WorldRayPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ObjectPick { cursor: WorldObjectPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ComponentPick { cursor: WorldComponentPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ContextMenu { cursor: WorldContextMenuCursor, retirement: Option<WorldInteractionAuthorityStep> },
    MarqueePick { cursor: WorldMarqueePickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    MarqueePublish { job: WorldMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },
    ComponentMarqueePublish { job: WorldComponentMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },
    GumballPick { cursor: WorldGumballPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    GumballCommit { job: WorldGumballCommitJob, retirement: Option<WorldInteractionAuthorityStep> },
    BrushCommit { job: WorldBrushCommitJob, retirement: Option<WorldInteractionAuthorityStep> },
}

struct WorldInteractionAuthority {
    queue: WorldInteractionIntentQueue,
    blocked: Option<WorldInteractionIntent>,
    registry: Option<WorldInteractionRegistryBuildCursor>,
    active: Option<WorldInteractionActive>,
    right_press: Option<[f32; 2]>,
    right_dragged: bool,
    marquee: Option<WorldMarqueeGesture>,
    gumball: Option<WorldGumballGesture>,
    next_generation: u64,
    faulted: bool,
    closing: bool,
}

impl Default for WorldInteractionAuthority {
    fn default() -> Self {
        Self { queue: WorldInteractionIntentQueue::default(), blocked: None, registry: None, active: None, right_press: None, right_dragged: false, marquee: None, gumball: None, next_generation: 1, faulted: false, closing: false }
    }
}

struct WorldMarqueeGesture {
    revision: u64,
    start_generation: u64,
    points: Box<[Option<[f32; 2]>; WORLD_INTERACTION_MARQUEE_POINT_CAPACITY]>,
    len: u16,
    retiring: bool,
}

const WORLD_MARQUEE_RESULT_PAGE_CAPACITY: usize = 64;
const WORLD_MARQUEE_RESULT_PAGE_COUNT: usize = 16;
const WORLD_COMPONENT_MARQUEE_CAPACITY: usize = 64;

#[derive(Clone, Copy)]
enum WorldMarqueeResult {
    Object(WorldInteractionObjectToken),
    Component { object: WorldInteractionObjectToken, id: u32 },
}

struct WorldMarqueeResultPages {
    pages: Box<[[Option<WorldMarqueeResult>; WORLD_MARQUEE_RESULT_PAGE_CAPACITY]; WORLD_MARQUEE_RESULT_PAGE_COUNT]>,
    lens: [u8; WORLD_MARQUEE_RESULT_PAGE_COUNT],
    id_bytes: [u16; WORLD_MARQUEE_RESULT_PAGE_COUNT],
    page_len: u8,
}

impl Default for WorldMarqueeResultPages {
    fn default() -> Self {
        Self { pages: Box::new([[None; WORLD_MARQUEE_RESULT_PAGE_CAPACITY]; WORLD_MARQUEE_RESULT_PAGE_COUNT]), lens: [0; WORLD_MARQUEE_RESULT_PAGE_COUNT], id_bytes: [0; WORLD_MARQUEE_RESULT_PAGE_COUNT], page_len: 0 }
    }
}

impl WorldMarqueeResultPages {
    fn push(&mut self, result: WorldMarqueeResult, id_bytes: usize) -> bool {
        if let WorldMarqueeResult::Component { id, .. } = result {
            if self.pages[0][..usize::from(self.lens[0])].iter().flatten().any(|entry| matches!(entry, WorldMarqueeResult::Component { id: existing, .. } if *existing == id)) {
                return true;
            }
        }
        if matches!(result, WorldMarqueeResult::Component { .. }) && (self.page_len > 1 || self.page_len == 1 && usize::from(self.lens[0]) == WORLD_COMPONENT_MARQUEE_CAPACITY) {
            return false;
        }
        let mut page = usize::from(self.page_len.saturating_sub(1));
        if self.page_len == 0 || usize::from(self.lens[page]) == WORLD_MARQUEE_RESULT_PAGE_CAPACITY {
            page = usize::from(self.page_len);
            if page == WORLD_MARQUEE_RESULT_PAGE_COUNT {
                return false;
            }
            self.page_len += 1;
        }
        let item = usize::from(self.lens[page]);
        let Some(bytes) = usize::from(self.id_bytes[page]).checked_add(id_bytes).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY) else {
            return false;
        };
        self.pages[page][item] = Some(result);
        self.lens[page] += 1;
        self.id_bytes[page] = bytes as u16;
        true
    }

    fn close_step(&mut self) -> bool {
        if self.page_len == 0 {
            return true;
        }
        let page = usize::from(self.page_len - 1);
        if self.lens[page] > 0 {
            self.lens[page] -= 1;
            self.pages[page][usize::from(self.lens[page])] = None;
            return false;
        }
        self.id_bytes[page] = 0;
        self.page_len -= 1;
        false
    }
}

struct WorldMarqueePickCursor {
    generation: u64,
    gesture: WorldMarqueeGesture,
    viewport: Rect,
    view_projection: Mat4,
    rectangle: bool,
    component_kind: Option<WorldComponentKind>,
    crossing: Option<bool>,
    direction_index: u16,
    slot: u16,
    current: Option<WorldInteractionObjectToken>,
    topology: u32,
    polygon_edge: u16,
    candidate: Option<WorldMarqueeCandidate>,
    winding: [i16; 3],
    intersects: bool,
    any_visible: bool,
    results: WorldMarqueeResultPages,
    complete: bool,
    faulted: bool,
}

#[derive(Clone, Copy)]
enum WorldMarqueeCandidate {
    Point { point: [f32; 2], component: Option<u32> },
    Segment { points: [[f32; 2]; 2], component: u32 },
    Triangle { points: [[f32; 2]; 3], component: Option<u32> },
}

impl WorldMarqueePickCursor {
    fn new(state: &World3dState, generation: u64, gesture: WorldMarqueeGesture) -> Option<Self> {
        let viewport = render_pick_viewport(state);
        if viewport.w <= 0.0 || viewport.h <= 0.0 || gesture.len < 2 {
            return None;
        }
        let camera = state.orbit.to_camera();
        let view_projection = camera.view_proj((viewport.w / viewport.h.max(1.0)).max(0.1));
        let rectangle = state.selection_method != "lasso";
        let component_kind = match state.granularity.as_str() {
            "vertex" => Some(WorldComponentKind::Vertex),
            "edge" => Some(WorldComponentKind::Edge),
            "face" | "component" => Some(WorldComponentKind::Face),
            _ => None,
        };
        let crossing = rectangle.then(|| {
            let start = gesture.points[0].expect("marquee start");
            let end = gesture.points[usize::from(gesture.len - 1)].expect("marquee end");
            end[0] < start[0]
        });
        Some(Self {
            generation,
            gesture,
            viewport,
            view_projection,
            rectangle,
            component_kind,
            crossing,
            direction_index: 1,
            slot: 0,
            current: None,
            topology: 0,
            polygon_edge: 0,
            candidate: None,
            winding: [0; 3],
            intersects: false,
            any_visible: false,
            results: WorldMarqueeResultPages::default(),
            complete: false,
            faulted: false,
        })
    }

    fn local_point(&self, index: usize) -> [f32; 2] {
        let point = self.gesture.points[index].expect("marquee point within retained length");
        [point[0] - self.viewport.x, point[1] - self.viewport.y]
    }

    fn admit_candidate(&mut self, state: &World3dState, result: WorldMarqueeResult) -> WorldInteractionStep {
        let token = match result {
            WorldMarqueeResult::Object(token) | WorldMarqueeResult::Component { object: token, .. } => token,
        };
        let Some(entry) = state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let id_bytes = if matches!(result, WorldMarqueeResult::Object(_)) { entry.id.as_str().len() } else { 0 };
        if !self.results.push(result, id_bytes) {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        }
        WorldInteractionStep::Pending
    }

    fn orient(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
        (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    }

    fn on_segment(point: [f32; 2], a: [f32; 2], b: [f32; 2]) -> bool {
        point[0] >= a[0].min(b[0]) && point[0] <= a[0].max(b[0]) && point[1] >= a[1].min(b[1]) && point[1] <= a[1].max(b[1])
    }

    fn segments_intersect(a0: [f32; 2], a1: [f32; 2], b0: [f32; 2], b1: [f32; 2]) -> bool {
        let o1 = Self::orient(a0, a1, b0);
        let o2 = Self::orient(a0, a1, b1);
        let o3 = Self::orient(b0, b1, a0);
        let o4 = Self::orient(b0, b1, a1);
        (o1 == 0.0 && Self::on_segment(b0, a0, a1)) || (o2 == 0.0 && Self::on_segment(b1, a0, a1)) || (o3 == 0.0 && Self::on_segment(a0, b0, b1)) || (o4 == 0.0 && Self::on_segment(a1, b0, b1)) || ((o1 > 0.0) != (o2 > 0.0) && (o3 > 0.0) != (o4 > 0.0))
    }

    fn rect(&self) -> [f32; 4] {
        let start = self.local_point(0);
        let end = self.local_point(usize::from(self.gesture.len - 1));
        [start[0].min(end[0]), start[1].min(end[1]), start[0].max(end[0]), start[1].max(end[1])]
    }

    fn rect_contains(rect: [f32; 4], point: [f32; 2]) -> bool {
        point[0] >= rect[0] && point[0] <= rect[2] && point[1] >= rect[1] && point[1] <= rect[3]
    }

    fn triangle_intersects_rect(points: [[f32; 2]; 3], rect: [f32; 4]) -> bool {
        if points.iter().any(|point| Self::rect_contains(rect, *point)) {
            return true;
        }
        let corners = [[rect[0], rect[1]], [rect[2], rect[1]], [rect[2], rect[3]], [rect[0], rect[3]]];
        (0..3).any(|edge| (0..4).any(|side| Self::segments_intersect(points[edge], points[(edge + 1) % 3], corners[side], corners[(side + 1) % 4])))
    }

    fn begin_candidate(&mut self, candidate: WorldMarqueeCandidate) {
        self.candidate = Some(candidate);
        self.polygon_edge = 0;
        self.winding = [0; 3];
        self.intersects = false;
    }

    fn candidate_step(&mut self) -> Option<bool> {
        let candidate = self.candidate?;
        let edge = usize::from(self.polygon_edge);
        if edge == usize::from(self.gesture.len) {
            let selected = match candidate {
                WorldMarqueeCandidate::Point { .. } => self.winding[0] != 0,
                WorldMarqueeCandidate::Segment { .. } => {
                    if self.crossing.expect("marquee direction resolved") {
                        self.intersects || self.winding[0] != 0 || self.winding[1] != 0
                    } else {
                        self.winding[0] != 0 && self.winding[1] != 0
                    }
                }
                WorldMarqueeCandidate::Triangle { .. } => {
                    if self.crossing.expect("marquee direction resolved") {
                        self.intersects || self.winding.iter().any(|winding| *winding != 0)
                    } else {
                        self.winding.iter().all(|winding| *winding != 0)
                    }
                }
            };
            self.candidate = None;
            self.polygon_edge = 0;
            self.winding = [0; 3];
            self.intersects = false;
            return Some(selected);
        }
        let a = self.local_point(edge);
        let b = self.local_point((edge + 1) % usize::from(self.gesture.len));
        let points = match candidate {
            WorldMarqueeCandidate::Point { point, .. } => [point, point, point],
            WorldMarqueeCandidate::Segment { points, .. } => [points[0], points[1], points[1]],
            WorldMarqueeCandidate::Triangle { points, .. } => points,
        };
        let count = match candidate {
            WorldMarqueeCandidate::Point { .. } => 1,
            WorldMarqueeCandidate::Segment { .. } => 2,
            WorldMarqueeCandidate::Triangle { .. } => 3,
        };
        for index in 0..count {
            let point = points[index];
            if a[1] <= point[1] {
                if b[1] > point[1] && Self::orient(a, b, point) > 0.0 {
                    self.winding[index] += 1;
                }
            } else if b[1] <= point[1] && Self::orient(a, b, point) < 0.0 {
                self.winding[index] -= 1;
            }
        }
        match candidate {
            WorldMarqueeCandidate::Segment { points, .. } => {
                self.intersects |= Self::segments_intersect(points[0], points[1], a, b);
            }
            WorldMarqueeCandidate::Triangle { points, .. } => {
                self.intersects |= (0..3).any(|index| Self::segments_intersect(points[index], points[(index + 1) % 3], a, b));
            }
            WorldMarqueeCandidate::Point { .. } => {}
        }
        self.polygon_edge += 1;
        None
    }

    fn step(&mut self, state: &World3dState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.gesture.revision != state.interaction_revision || self.generation != generation || !state.interaction_objects.terminal_for_revision(self.gesture.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.crossing.is_none() {
            let index = usize::from(self.direction_index);
            if index < usize::from(self.gesture.len) {
                let start = self.local_point(0);
                let point = self.local_point(index);
                self.direction_index += 1;
                if (point[0] - start[0]).abs() >= 2.0 {
                    self.crossing = Some(point[0] < start[0]);
                }
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let start = self.local_point(0);
            let end = self.local_point(usize::from(self.gesture.len - 1));
            self.crossing = Some(end[0] < start[0]);
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        if self.candidate.is_some() {
            let candidate = self.candidate.expect("retained marquee candidate");
            let Some(selected) = self.candidate_step() else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            let token = self.current.expect("marquee candidate object");
            let component = match candidate {
                WorldMarqueeCandidate::Point { component, .. } | WorldMarqueeCandidate::Triangle { component, .. } => component,
                WorldMarqueeCandidate::Segment { component, .. } => Some(component),
            };
            if let Some(id) = component {
                let outcome = if selected { self.admit_candidate(state, WorldMarqueeResult::Component { object: token, id }) } else { WorldInteractionStep::Pending };
                context.consume_fuel(1);
                return outcome;
            }
            if !selected && !self.crossing.expect("crossing resolved") {
                self.current = None;
                self.topology = 0;
                self.any_visible = false;
            }
            let outcome = if selected && self.crossing.expect("crossing resolved") {
                self.current = None;
                self.topology = 0;
                self.any_visible = false;
                self.admit_candidate(state, WorldMarqueeResult::Object(token))
            } else {
                WorldInteractionStep::Pending
            };
            context.consume_fuel(1);
            return outcome;
        }
        if self.current.is_none() {
            let index = usize::from(self.slot);
            if index >= usize::from(state.interaction_objects.instance_len) {
                if self.results.page_len == 0 {
                    self.results.page_len = 1;
                }
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.slot += 1;
            let Some(token) = state.interaction_objects.instance_order[index] else {
                return WorldInteractionStep::Fault;
            };
            let Some(entry) = state.interaction_objects.resolve(token).filter(|entry| entry.revision == self.gesture.revision && entry.kind == WorldInteractionObjectKind::Instance) else {
                return WorldInteractionStep::Stale;
            };
            if self.component_kind.is_some() && state.active_object_id.as_deref().is_some_and(|active| active != entry.id.as_str()) {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.current = Some(token);
            self.topology = 0;
            self.any_visible = false;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.current.expect("marquee current object");
        let Some(entry) = state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(mesh_token) = entry.mesh else {
            return WorldInteractionStep::Fault;
        };
        let Some(admitted) = state.interaction_meshes.resolve(mesh_token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(&mesh) = state.meshes.get(admitted.id.as_str()) else {
            return WorldInteractionStep::Fault;
        };
        let Ok(schema) = mesh.schema() else {
            return WorldInteractionStep::Stale;
        };
        if admitted.vertices != schema.vertices || admitted.edges != schema.edges || admitted.triangles != schema.indices / 3 {
            return WorldInteractionStep::Stale;
        }
        let crossing = self.crossing.expect("crossing resolved");
        if let Some(kind) = self.component_kind {
            let count = match kind {
                WorldComponentKind::Vertex => admitted.vertices,
                WorldComponentKind::Edge => admitted.edges,
                WorldComponentKind::Face => admitted.triangles,
            };
            if self.topology >= count {
                self.current = None;
                self.topology = 0;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let index = self.topology;
            self.topology += 1;
            let id = match kind {
                WorldComponentKind::Vertex => world_mesh_component_id(mesh, Mesh3dField::VertexIds, index),
                WorldComponentKind::Edge => world_mesh_component_id(mesh, Mesh3dField::EdgeIds, index),
                WorldComponentKind::Face => world_mesh_component_id(mesh, Mesh3dField::FaceIds, index),
            };
            let mut selected = false;
            match kind {
                WorldComponentKind::Vertex => {
                    let Ok(point) = mesh.vec3(Mesh3dField::Positions, index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let world = entry.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                    let Some(screen) = ui_wgpu::wgpu::project_point(self.view_projection, world, self.viewport.w, self.viewport.h) else {
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    };
                    if self.rectangle {
                        selected = Self::rect_contains(self.rect(), screen);
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Point { point: screen, component: Some(id) });
                    }
                }
                WorldComponentKind::Edge => {
                    let Ok(edge) = mesh.edge(index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let a = entry.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                    let b = entry.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                    let (Some(screen_a), Some(screen_b)) = (ui_wgpu::wgpu::project_point(self.view_projection, a, self.viewport.w, self.viewport.h), ui_wgpu::wgpu::project_point(self.view_projection, b, self.viewport.w, self.viewport.h)) else {
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    };
                    if self.rectangle {
                        selected = if crossing {
                            Self::rect_contains(self.rect(), screen_a) || Self::rect_contains(self.rect(), screen_b) || {
                                let rect = self.rect();
                                let corners = [[rect[0], rect[1]], [rect[2], rect[1]], [rect[2], rect[3]], [rect[0], rect[3]]];
                                (0..4).any(|side| Self::segments_intersect(screen_a, screen_b, corners[side], corners[(side + 1) % 4]))
                            }
                        } else {
                            Self::rect_contains(self.rect(), screen_a) && Self::rect_contains(self.rect(), screen_b)
                        };
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Segment { points: [screen_a, screen_b], component: id });
                    }
                }
                WorldComponentKind::Face => {
                    let Some(indices) = world_mesh_triangle(mesh, index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let mut points = [[0.0; 2]; 3];
                    for corner in 0..3 {
                        let Some(world) = world_mesh_vertex(mesh, indices[corner]).map(|vertex| entry.model.transform_point(vertex)) else {
                            return WorldInteractionStep::Stale;
                        };
                        let Some(screen) = ui_wgpu::wgpu::project_point(self.view_projection, world, self.viewport.w, self.viewport.h) else {
                            context.consume_fuel(1);
                            return WorldInteractionStep::Pending;
                        };
                        points[corner] = screen;
                    }
                    if self.rectangle {
                        selected = if crossing { Self::triangle_intersects_rect(points, self.rect()) } else { points.iter().all(|point| Self::rect_contains(self.rect(), *point)) };
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Triangle { points, component: Some(id) });
                    }
                }
            }
            let outcome = if selected { self.admit_candidate(state, WorldMarqueeResult::Component { object: token, id }) } else { WorldInteractionStep::Pending };
            context.consume_fuel(1);
            return outcome;
        }
        if !crossing {
            if self.topology >= admitted.vertices {
                self.current = None;
                self.topology = 0;
                let selected = self.any_visible;
                self.any_visible = false;
                let outcome = if selected { self.admit_candidate(state, WorldMarqueeResult::Object(token)) } else { WorldInteractionStep::Pending };
                context.consume_fuel(1);
                return outcome;
            }
            let index = self.topology;
            self.topology += 1;
            let Some(world) = world_mesh_vertex(mesh, index).map(|vertex| entry.model.transform_point(vertex)) else {
                return WorldInteractionStep::Stale;
            };
            let Some(point) = ui_wgpu::wgpu::project_point(self.view_projection, world, self.viewport.w, self.viewport.h) else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            self.any_visible = true;
            if self.rectangle {
                if !Self::rect_contains(self.rect(), point) {
                    self.current = None;
                    self.topology = 0;
                    self.any_visible = false;
                }
            } else {
                self.begin_candidate(WorldMarqueeCandidate::Point { point, component: None });
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        if self.topology >= admitted.triangles {
            self.current = None;
            self.topology = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let triangle = self.topology;
        self.topology += 1;
        let Some(indices) = world_mesh_triangle(mesh, triangle) else {
            return WorldInteractionStep::Stale;
        };
        let mut points = [[0.0; 2]; 3];
        for index in 0..3 {
            let Some(world) = world_mesh_vertex(mesh, indices[index]).map(|vertex| entry.model.transform_point(vertex)) else {
                return WorldInteractionStep::Stale;
            };
            let Some(point) = ui_wgpu::wgpu::project_point(self.view_projection, world, self.viewport.w, self.viewport.h) else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            points[index] = point;
        }
        let selected = if self.rectangle {
            Self::triangle_intersects_rect(points, self.rect())
        } else {
            self.begin_candidate(WorldMarqueeCandidate::Triangle { points, component: None });
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let outcome = if selected {
            self.current = None;
            self.topology = 0;
            self.admit_candidate(state, WorldMarqueeResult::Object(token))
        } else {
            WorldInteractionStep::Pending
        };
        context.consume_fuel(1);
        outcome
    }

    fn close_step(&mut self) -> bool {
        if self.candidate.take().is_some() {
            self.polygon_edge = 0;
            self.winding = [0; 3];
            self.intersects = false;
            return false;
        }
        if self.current.take().is_some() {
            self.topology = 0;
            self.any_visible = false;
            return false;
        }
        if !self.results.close_step() {
            return false;
        }
        self.gesture.close_step()
    }

    fn finish(self, state: &World3dState, generation: u64) -> Result<(WorldMarqueeGesture, WorldMarqueeResultPages), (Self, WorldInteractionStep)> {
        if !self.complete {
            return Err((self, WorldInteractionStep::Pending));
        }
        if self.gesture.revision != state.interaction_revision || self.generation != generation {
            return Err((self, WorldInteractionStep::Stale));
        }
        Ok((self.gesture, self.results))
    }
}

struct WorldMarqueePublishJob {
    generation: u64,
    gesture: WorldMarqueeGesture,
    results: WorldMarqueeResultPages,
    merge: &'static str,
    prepared: Option<ui_wgpu::wgpu::PreparedClaimedActionBatch>,
    draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    page: u8,
    stage: u16,
    published: bool,
}

impl WorldMarqueePublishJob {
    fn new(generation: u64, gesture: WorldMarqueeGesture, results: WorldMarqueeResultPages, shift: bool, ctrl: bool) -> Self {
        let merge = if shift {
            merge_mode_wire_str(MergeMode::Additive)
        } else if ctrl {
            merge_mode_wire_str(MergeMode::Invertive)
        } else {
            merge_mode_wire_str(MergeMode::Replace)
        };
        Self { generation, gesture, results, merge, prepared: None, draft: None, page: 0, stage: 0, published: false }
    }

    fn page_credit(&self, state: &World3dState, page: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
        let target_count = usize::from(self.results.lens[page]);
        let target_keys = target_count.checked_mul("granularity".len() + "id".len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let granularities = target_count.checked_mul(resolved_domain_granularity_id(state).len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let id_prefix = if state.bound_domain_id.is_some() { 0 } else { target_count.checked_mul(state.surface_id.len() + WORLD_ITEM_PATH_DELIMITER.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)? };
        ui_wgpu::wgpu::checked_action_string_bytes(&[&state.controller_id, "interactionSelect", "domainId", resolved_domain_id(state), "targets", "merge", self.merge, "method", selection_method_wire_str(SelectionMethod::Rectangle)])?
            .checked_add(target_keys)
            .and_then(|bytes| bytes.checked_add(granularities))
            .and_then(|bytes| bytes.checked_add(id_prefix))
            .and_then(|bytes| bytes.checked_add(usize::from(self.results.id_bytes[page])))
            .filter(|bytes| *bytes <= ui_wgpu::wgpu::ACTION_ITEM_BYTE_CAPACITY)
            .ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
    }

    fn step(&mut self, state: &World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.gesture.revision != state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if self.published {
            if !self.results.close_step() {
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            if !self.gesture.close_step() {
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            return Ok(WorldInteractionStep::Complete);
        }
        if self.prepared.is_none() {
            let mut credits = [0usize; WORLD_MARQUEE_RESULT_PAGE_COUNT];
            for page in 0..usize::from(self.results.page_len) {
                credits[page] = self.page_credit(state, page)?;
            }
            let claims = input.claim_actions(&credits[..usize::from(self.results.page_len)])?;
            self.prepared = Some(ui_wgpu::wgpu::PreparedClaimedActionBatch::new(claims));
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if usize::from(self.page) == usize::from(self.results.page_len) {
            input.publish_prepared_claimed_actions(self.prepared.take().expect("terminal marquee claimed batch"))?;
            self.published = true;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let page = usize::from(self.page);
        if self.draft.is_none() {
            let claim = self.prepared.as_ref().and_then(|batch| batch.claim(page)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            self.draft = Some(input.draft_claimed_action(claim, &state.controller_id, "interactionSelect")?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let target_count = u16::from(self.results.lens[page]);
        let target_stage_end = 3 + target_count * 4;
        let target_index = (self.stage >= 3 && self.stage < target_stage_end).then_some(usize::from((self.stage - 3) / 4));
        let target_field = (self.stage >= 3 && self.stage < target_stage_end).then_some((self.stage - 3) % 4);
        let token = target_index
            .and_then(|index| self.results.pages[page][index])
            .map(|result| match result {
                WorldMarqueeResult::Object(token) => Ok(token),
                WorldMarqueeResult::Component { .. } => Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
            })
            .transpose()?;
        let entry = token.map(|token| state.interaction_objects.resolve(token).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)).transpose()?;
        let draft = self.draft.as_mut().expect("marquee page draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("domainId"), resolved_domain_id(state))?,
            2 => draft.builder().begin_array(Some("targets"))?,
            stage if stage < target_stage_end => match target_field.expect("target field") {
                0 => draft.builder().begin_object(None)?,
                1 => draft.builder().string(Some("granularity"), resolved_domain_granularity_id(state))?,
                2 => {
                    let id = entry.expect("marquee token resolved").id.as_str();
                    if state.bound_domain_id.is_some() {
                        draft.builder().string(Some("id"), id)?;
                    } else {
                        draft.builder().string_joined(Some("id"), &[&state.surface_id, WORLD_ITEM_PATH_DELIMITER, id])?;
                    }
                }
                3 => draft.builder().end_container()?,
                _ => unreachable!("four target fields"),
            },
            stage if stage == target_stage_end => draft.builder().end_container()?,
            stage if stage == target_stage_end + 1 => draft.builder().string(Some("merge"), self.merge)?,
            stage if stage == target_stage_end + 2 => draft.builder().string(Some("method"), selection_method_wire_str(SelectionMethod::Rectangle))?,
            stage if stage == target_stage_end + 3 => draft.builder().end_container()?,
            _ => {
                let action = self.draft.take().expect("terminal marquee page draft").finish()?;
                self.prepared.as_mut().expect("marquee claimed batch").push(action)?;
                self.page += 1;
                self.stage = 0;
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(prepared) = self.prepared.as_mut() {
            if let Some(claim) = prepared.take_last_claim() {
                if input.release_action_claim(claim).is_err() {
                    prepared.restore_last_claim(claim).expect("marquee close restores retained claim slot");
                    return false;
                }
                return false;
            }
            self.prepared = None;
            return false;
        }
        if !self.results.close_step() {
            return false;
        }
        self.gesture.close_step()
    }
}

struct WorldComponentMarqueePublishJob {
    generation: u64,
    gesture: WorldMarqueeGesture,
    results: WorldMarqueeResultPages,
    kind: WorldComponentKind,
    merge: &'static str,
    merged: [Option<u32>; WORLD_COMPONENT_MARQUEE_CAPACITY],
    merged_len: u8,
    existing_cursor: u16,
    result_cursor: u8,
    merged_ready: bool,
    claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    prepared: Option<ui_wgpu::wgpu::PreparedClaimedAction>,
    stage: u16,
    published: bool,
}

impl WorldComponentMarqueePublishJob {
    fn new(generation: u64, gesture: WorldMarqueeGesture, results: WorldMarqueeResultPages, kind: WorldComponentKind, shift: bool, ctrl: bool) -> Self {
        let merge = if shift {
            merge_mode_wire_str(MergeMode::Additive)
        } else if ctrl {
            merge_mode_wire_str(MergeMode::Invertive)
        } else {
            merge_mode_wire_str(MergeMode::Replace)
        };
        Self {
            generation,
            gesture,
            results,
            kind,
            merge,
            merged: [None; WORLD_COMPONENT_MARQUEE_CAPACITY],
            merged_len: 0,
            existing_cursor: 0,
            result_cursor: 0,
            merged_ready: false,
            claim: None,
            draft: None,
            prepared: None,
            stage: 0,
            published: false,
        }
    }

    fn find(&self, id: u32) -> Option<usize> {
        self.merged[..usize::from(self.merged_len)].iter().position(|entry| *entry == Some(id))
    }

    fn add(&mut self, id: u32) -> bool {
        if self.find(id).is_some() {
            return true;
        }
        let index = usize::from(self.merged_len);
        if index == WORLD_COMPONENT_MARQUEE_CAPACITY {
            return false;
        }
        self.merged[index] = Some(id);
        self.merged_len += 1;
        true
    }

    fn remove(&mut self, index: usize) {
        let len = usize::from(self.merged_len);
        for slot in index..len.saturating_sub(1) {
            self.merged[slot] = self.merged[slot + 1];
        }
        self.merged[len - 1] = None;
        self.merged_len -= 1;
    }

    fn merge_step(&mut self, state: &World3dState) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        if self.merge != merge_mode_wire_str(MergeMode::Replace) && usize::from(self.existing_cursor) < state.component_ids.len() {
            let value = &state.component_ids[usize::from(self.existing_cursor)];
            self.existing_cursor += 1;
            if value.len() > 10 {
                return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
            }
            let id = value.parse::<u32>().map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            return self.add(id).then_some(false).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        let index = usize::from(self.result_cursor);
        if index < usize::from(self.results.lens[0]) {
            let Some(WorldMarqueeResult::Component { object, id }) = self.results.pages[0][index] else {
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            };
            state.interaction_objects.resolve(object).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            self.result_cursor += 1;
            if self.merge == merge_mode_wire_str(MergeMode::Invertive) {
                if let Some(position) = self.find(id) {
                    self.remove(position);
                } else if !self.add(id) {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
                }
            } else if !self.add(id) {
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            }
            return Ok(false);
        }
        Ok(true)
    }

    fn step(&mut self, state: &World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.gesture.revision != state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if self.published {
            if self.merged_len > 0 {
                self.merged_len -= 1;
                self.merged[usize::from(self.merged_len)] = None;
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            if !self.results.close_step() || !self.gesture.close_step() {
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            return Ok(WorldInteractionStep::Complete);
        }
        if !self.merged_ready {
            self.merged_ready = self.merge_step(state)?;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.claim.is_none() {
            let credits = ui_wgpu::wgpu::checked_action_string_bytes(&[&state.controller_id, "setSelection", "mode", self.kind.as_str(), "ids"])?;
            self.claim = Some(input.claim_action(credits)?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() && self.prepared.is_none() {
            self.draft = Some(input.draft_claimed_action(self.claim.expect("component marquee claim"), &state.controller_id, "setSelection")?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if let Some(prepared) = self.prepared.take() {
            input.publish_prepared_claimed_action(prepared)?;
            self.claim = None;
            self.published = true;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let ids_end = 3 + u16::from(self.merged_len);
        let draft = self.draft.as_mut().expect("component marquee draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("mode"), self.kind.as_str())?,
            2 => draft.builder().begin_array(Some("ids"))?,
            stage if stage < ids_end => {
                let id = self.merged[usize::from(stage - 3)].ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                draft.builder().number(None, id as f64)?;
            }
            stage if stage == ids_end => draft.builder().end_container()?,
            stage if stage == ids_end + 1 => draft.builder().end_container()?,
            _ => {
                self.prepared = Some(self.draft.take().expect("terminal component marquee draft").finish()?);
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.prepared.take().is_some() || self.draft.take().is_some() {
            return false;
        }
        if let Some(claim) = self.claim.take() {
            if input.release_action_claim(claim).is_err() {
                self.claim = Some(claim);
            }
            return false;
        }
        if self.merged_len > 0 {
            self.merged_len -= 1;
            self.merged[usize::from(self.merged_len)] = None;
            return false;
        }
        if !self.results.close_step() {
            return false;
        }
        self.gesture.close_step()
    }
}

impl WorldMarqueeGesture {
    fn new(revision: u64, generation: u64, point: [f32; 2]) -> Self {
        let mut points = Box::new([None; WORLD_INTERACTION_MARQUEE_POINT_CAPACITY]);
        points[0] = Some(point);
        Self { revision, start_generation: generation, points, len: 1, retiring: false }
    }

    fn push(&mut self, point: [f32; 2]) -> bool {
        let index = usize::from(self.len);
        if self.retiring || index == WORLD_INTERACTION_MARQUEE_POINT_CAPACITY {
            return false;
        }
        self.points[index] = Some(point);
        self.len += 1;
        true
    }

    fn is_click(&self, end: [f32; 2]) -> bool {
        let start = self.points[0].expect("marquee gesture owns start point");
        let dx = end[0] - start[0];
        let dy = end[1] - start[1];
        (dx * dx + dy * dy).sqrt() <= CLICK_DRAG_THRESHOLD_PX
    }

    fn close_step(&mut self) -> bool {
        self.retiring = true;
        if self.len == 0 {
            return true;
        }
        self.len -= 1;
        self.points[usize::from(self.len)] = None;
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionAuthorityStep {
    Idle,
    Pending,
    OutputBlocked,
    Complete,
    Stale,
    Fault,
}

impl WorldInteractionPlan {
    fn new(revision: u64, generation: u64) -> Self {
        Self { revision, generation, bytes: Box::new([0; WORLD_INTERACTION_BYTE_CAPACITY]), byte_len: 0, actions: Box::new([None; WORLD_INTERACTION_ITEM_CAPACITY]), action_len: 0, cursor: 0, faulted: false }
    }

    fn push_string(&mut self, value: &str) -> Option<WorldInteractionSpan> {
        if value.len() > u16::MAX as usize {
            self.faulted = true;
            return None;
        }
        let start = usize::from(self.byte_len);
        let Some(end) = start.checked_add(value.len()).filter(|end| *end <= WORLD_INTERACTION_BYTE_CAPACITY) else {
            self.faulted = true;
            return None;
        };
        self.bytes[start..end].copy_from_slice(value.as_bytes());
        self.byte_len = end as u16;
        Some(WorldInteractionSpan { start: start as u16, len: value.len() as u16 })
    }

    fn push_joined(&mut self, parts: &[&str]) -> Option<WorldInteractionSpan> {
        let len = parts.iter().try_fold(0usize, |total, part| total.checked_add(part.len()))?;
        if len > u16::MAX as usize {
            self.faulted = true;
            return None;
        }
        let start = usize::from(self.byte_len);
        let Some(end) = start.checked_add(len).filter(|end| *end <= WORLD_INTERACTION_BYTE_CAPACITY) else {
            self.faulted = true;
            return None;
        };
        let mut cursor = start;
        for part in parts {
            let next = cursor + part.len();
            self.bytes[cursor..next].copy_from_slice(part.as_bytes());
            cursor = next;
        }
        self.byte_len = end as u16;
        Some(WorldInteractionSpan { start: start as u16, len: len as u16 })
    }

    fn push_action(&mut self, action: WorldFlatAction) -> bool {
        let index = usize::from(self.action_len);
        if index == WORLD_INTERACTION_ITEM_CAPACITY {
            self.faulted = true;
            return false;
        }
        self.actions[index] = Some(action);
        self.action_len += 1;
        true
    }

    fn string(&self, span: WorldInteractionSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&self.bytes[start..end]).expect("world interaction strings originate from UTF-8")
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.cursor == self.action_len && self.byte_len == 0
    }

    pub fn close_step(&mut self) -> bool {
        if self.cursor < self.action_len {
            self.actions[usize::from(self.cursor)] = None;
            self.cursor += 1;
            return false;
        }
        self.byte_len = 0;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionStep {
    Pending,
    Complete,
    Stale,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldRayPickPurpose {
    Instance,
    Hover,
    Paint,
    Surface,
}

#[derive(Clone, Copy, Debug)]
struct WorldRayHitRef {
    mesh: WorldInteractionMeshToken,
    draw: u16,
    instance: u16,
    triangle: u32,
    distance: f32,
    bary_u: f32,
    bary_v: f32,
    point: Vec3,
    normal: Vec3,
}

pub struct WorldRayPickCursor {
    revision: u64,
    generation: u64,
    purpose: WorldRayPickPurpose,
    origin: Vec3,
    direction: Vec3,
    draw: usize,
    instance: usize,
    triangle: usize,
    mesh: Option<WorldInteractionMeshToken>,
    mesh_probe: u16,
    merge: u8,
    best: Option<WorldRayHitRef>,
    complete: bool,
    faulted: bool,
}

impl WorldRayPickCursor {
    pub fn new(state: &World3dState, generation: u64, purpose: WorldRayPickPurpose, x: f32, y: f32) -> Option<Self> {
        let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
        let camera = state.orbit.to_camera();
        let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
        let (origin, direction) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
        Some(Self { revision: state.interaction_revision, generation, purpose, origin, direction, draw: 0, instance: 0, triangle: 0, mesh: None, mesh_probe: 0, merge: 0, best: None, complete: false, faulted: false })
    }

    pub fn step(&mut self, state: &World3dState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if state.interaction_meshes.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != state.interaction_revision || self.generation != generation {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        let Some(draw) = state.draws.get(self.draw) else {
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        if self.mesh.is_none() {
            if draw.mesh_key.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY || usize::from(self.mesh_probe) == WORLD_INTERACTION_MESH_CAPACITY {
                self.faulted = true;
                return WorldInteractionStep::Fault;
            }
            let slot = (WorldInteractionMeshRegistry::hash(&draw.mesh_key) + usize::from(self.mesh_probe)) % WORLD_INTERACTION_MESH_CAPACITY;
            match state.interaction_meshes.slots[slot] {
                Some(entry) if entry.id.as_str() == draw.mesh_key => {
                    self.mesh = Some(WorldInteractionMeshToken { slot: slot as u16, generation: entry.generation });
                    self.mesh_probe = 0;
                }
                Some(_) => self.mesh_probe += 1,
                None => {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.mesh.expect("world pick mesh token resolved above");
        let Some(admitted) = state.interaction_meshes.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        if admitted.id.as_str() != draw.mesh_key || admitted.version != draw.mesh_version {
            return WorldInteractionStep::Stale;
        }
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let Ok(schema) = mesh.schema() else {
            return WorldInteractionStep::Stale;
        };
        if admitted.vertices != schema.vertices || admitted.triangles != schema.indices / 3 {
            return WorldInteractionStep::Stale;
        }
        let Some(instance) = draw.instances.get(self.instance) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            self.mesh = None;
            self.mesh_probe = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let Some(indices) = u32::try_from(self.triangle).ok().and_then(|triangle| world_mesh_triangle(mesh, triangle)) else {
            self.instance += 1;
            self.triangle = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let Some(a) = world_mesh_vertex(mesh, indices[0]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let Some(b) = world_mesh_vertex(mesh, indices[1]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let Some(c) = world_mesh_vertex(mesh, indices[2]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let a = instance.model.transform_point(a);
        let b = instance.model.transform_point(b);
        let c = instance.model.transform_point(c);
        if let Some((distance, bary_u, bary_v)) = world_ray_triangle_barycentric(self.origin, self.direction, a, b, c) {
            if self.best.is_none_or(|best| distance < best.distance) {
                let point = self.origin.add(self.direction.scale(distance));
                let mut normal = b.sub(a).cross(c.sub(a));
                if normal.length() > 1e-6 {
                    normal = normal.normalize();
                }
                let Ok(draw_index) = u16::try_from(self.draw) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                let Ok(instance_index) = u16::try_from(self.instance) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                let Ok(triangle_index) = u32::try_from(self.triangle) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                self.best = Some(WorldRayHitRef { mesh: token, draw: draw_index, instance: instance_index, triangle: triangle_index, distance, bary_u, bary_v, point, normal });
            }
        }
        self.triangle += 1;
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    pub fn finish_plan(&self, state: &World3dState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if self.faulted {
            return Err(WorldInteractionStep::Fault);
        }
        if self.revision != state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        let Some(hit) = self.best else {
            return match self.purpose {
                WorldRayPickPurpose::Hover => {
                    if state.local_hover_id.is_none() {
                        return Ok(None);
                    }
                    let mut plan = WorldInteractionPlan::new(self.revision, generation);
                    let controller = plan.push_string(&state.controller_id).ok_or(WorldInteractionStep::Fault)?;
                    let domain = plan.push_string(resolved_domain_id(state)).ok_or(WorldInteractionStep::Fault)?;
                    let action = WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), None, None, Some(domain), None, None, None, None], numbers: [0.0; 10], number_len: 0 };
                    plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
                }
                WorldRayPickPurpose::Instance => {
                    let mut plan = WorldInteractionPlan::new(self.revision, generation);
                    let controller = plan.push_string(&state.controller_id).ok_or(WorldInteractionStep::Fault)?;
                    let domain = plan.push_string(resolved_domain_id(state)).ok_or(WorldInteractionStep::Fault)?;
                    let merge = plan
                        .push_string(match self.merge {
                            1 => merge_mode_wire_str(MergeMode::Additive),
                            2 => merge_mode_wire_str(MergeMode::Invertive),
                            _ => merge_mode_wire_str(MergeMode::Replace),
                        })
                        .ok_or(WorldInteractionStep::Fault)?;
                    let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                    let action = WorldFlatAction { kind: WorldFlatActionKind::Select, strings: [Some(controller), None, None, Some(domain), None, Some(merge), Some(method), None], numbers: [0.0; 10], number_len: 0 };
                    plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
                }
                _ => Ok(None),
            };
        };
        let draw = state.draws.get(usize::from(hit.draw)).ok_or(WorldInteractionStep::Fault)?;
        let admitted = state.interaction_meshes.resolve(hit.mesh).ok_or(WorldInteractionStep::Stale)?;
        if admitted.id.as_str() != draw.mesh_key || admitted.version != draw.mesh_version {
            return Err(WorldInteractionStep::Stale);
        }
        let mesh = *state.meshes.get(&draw.mesh_key).ok_or(WorldInteractionStep::Fault)?;
        let instance = draw.instances.get(usize::from(hit.instance)).ok_or(WorldInteractionStep::Fault)?;
        if self.purpose == WorldRayPickPurpose::Hover && state.local_hover_id.as_deref() == Some(instance.id.as_str()) {
            return Ok(None);
        }
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let object = plan.push_string(&instance.id).ok_or(WorldInteractionStep::Fault)?;
        let action = match self.purpose {
            WorldRayPickPurpose::Paint => {
                let (u, v) = interpolate_mesh_uv(mesh, hit.triangle as usize, hit.bary_u, hit.bary_v).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction { kind: WorldFlatActionKind::PaintAt, strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None], numbers: [u as f64, v as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], number_len: 2 }
            }
            WorldRayPickPurpose::Surface => WorldFlatAction {
                kind: WorldFlatActionKind::SurfacePlace,
                strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None],
                numbers: [hit.point.x as f64, hit.point.y as f64, hit.point.z as f64, hit.normal.x as f64, hit.normal.y as f64, hit.normal.z as f64, 0.0, 0.0, 0.0, 0.0],
                number_len: 6,
            },
            WorldRayPickPurpose::Instance => {
                let domain = plan.push_string(resolved_domain_id(state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = plan.push_string(resolved_domain_granularity_id(state)).ok_or(WorldInteractionStep::Fault)?;
                let merge = plan
                    .push_string(match self.merge {
                        1 => merge_mode_wire_str(MergeMode::Additive),
                        2 => merge_mode_wire_str(MergeMode::Invertive),
                        _ => merge_mode_wire_str(MergeMode::Replace),
                    })
                    .ok_or(WorldInteractionStep::Fault)?;
                let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Select,
                    strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), Some(merge), Some(method), None],
                    numbers: [if state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    number_len: 1,
                }
            }
            WorldRayPickPurpose::Hover => {
                let domain = plan.push_string(resolved_domain_id(state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = plan.push_string(resolved_domain_granularity_id(state)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Hover,
                    strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), None, None, None],
                    numbers: [if state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    number_len: 1,
                }
            }
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete && self.best.is_none()
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldObjectPickPurpose {
    VortexHover,
    VortexSelect,
    ReferenceHover,
}

struct WorldObjectPickCursor {
    revision: u64,
    generation: u64,
    purpose: WorldObjectPickPurpose,
    origin: Vec3,
    direction: Vec3,
    slot: u16,
    merge: u8,
    best: Option<(WorldInteractionObjectToken, f32)>,
    complete: bool,
}

impl WorldObjectPickCursor {
    fn new(state: &World3dState, generation: u64, purpose: WorldObjectPickPurpose, x: f32, y: f32) -> Option<Self> {
        let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
        let camera = state.orbit.to_camera();
        let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
        let (origin, direction) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
        Some(Self { revision: state.interaction_revision, generation, purpose, origin, direction, slot: 0, merge: 0, best: None, complete: false })
    }

    fn from_ray(revision: u64, generation: u64, purpose: WorldObjectPickPurpose, origin: Vec3, direction: Vec3) -> Self {
        Self { revision, generation, purpose, origin, direction, slot: 0, merge: 0, best: None, complete: false }
    }

    fn step(&mut self, state: &World3dState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.revision != state.interaction_revision || self.generation != generation || !state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        let index = usize::from(self.slot);
        let Some(entry) = state.interaction_objects.slots.get(index) else {
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        self.slot += 1;
        let Some(entry) = entry else {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        if entry.revision != self.revision {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let distance = match (self.purpose, entry.kind) {
            (WorldObjectPickPurpose::VortexHover | WorldObjectPickPurpose::VortexSelect, WorldInteractionObjectKind::Vortex) => {
                let center = Vec3::new(entry.values[0], entry.values[1], entry.values[2]);
                let radius = entry.values[6].max(0.0);
                ray_aabb_slab(self.origin, self.direction, [center.x - radius, center.y - radius, center.z - radius], [center.x + radius, center.y + radius, center.z + radius])
            }
            (WorldObjectPickPurpose::ReferenceHover, WorldInteractionObjectKind::Reference) => {
                let center = Vec3::new(entry.values[0], entry.values[1], entry.values[2]);
                ray_plane_point(self.origin, self.direction, center, Vec3::new(0.0, 0.0, 1.0)).and_then(|hit| {
                    let offset = hit.sub(center);
                    (offset.x.abs() <= entry.values[3] * 0.5 && offset.y.abs() <= entry.values[4] * 0.5).then_some(self.origin.sub(hit).length())
                })
            }
            _ => None,
        };
        if let Some(distance) = distance {
            if self.best.is_none_or(|(_, best)| distance < best) {
                self.best = Some((WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision }, distance));
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    fn finish_plan(&self, state: &World3dState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if self.revision != state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        let entry = match self.best {
            Some((token, _)) => Some(state.interaction_objects.resolve(token).ok_or(WorldInteractionStep::Stale)?),
            None => None,
        };
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let action = match self.purpose {
            WorldObjectPickPurpose::VortexHover => {
                let hit = entry.map(|entry| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
                WorldFlatAction { kind: WorldFlatActionKind::VortexHover, strings: [Some(controller), Some(surface), hit, None, None, None, None, None], numbers: [0.0; 10], number_len: 0 }
            }
            WorldObjectPickPurpose::VortexSelect => {
                let Some(entry) = entry else {
                    return Ok(None);
                };
                let hit = plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)?;
                let merge = plan
                    .push_string(match self.merge {
                        1 => "add",
                        2 => "toggle",
                        _ => "replace",
                    })
                    .ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction { kind: WorldFlatActionKind::VortexSelect, strings: [Some(controller), Some(surface), Some(hit), Some(merge), None, None, None, None], numbers: [0.0; 10], number_len: 0 }
            }
            WorldObjectPickPurpose::ReferenceHover => {
                let hit = entry.map(|entry| plan.push_joined(&["reference:", entry.id.as_str()]).ok_or(WorldInteractionStep::Fault)).transpose()?;
                if hit.is_none() && state.local_hover_id.is_none() {
                    return Ok(None);
                }
                let domain = plan.push_string(resolved_domain_id(state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = match hit {
                    Some(_) => Some(plan.push_string(resolved_domain_granularity_id(state)).ok_or(WorldInteractionStep::Fault)?),
                    None => None,
                };
                WorldFlatAction {
                    kind: WorldFlatActionKind::Hover,
                    strings: [Some(controller), Some(surface), hit, Some(domain), granularity, None, None, None],
                    numbers: [if state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    number_len: 1,
                }
            }
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldComponentKind {
    Vertex,
    Edge,
    Face,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldComponentPickPurpose {
    Hover,
    Select,
}

#[derive(Clone, Copy)]
struct WorldComponentHit {
    object: WorldInteractionObjectToken,
    id: u32,
    primary: f32,
    secondary: f32,
}

struct WorldComponentPickCursor {
    revision: u64,
    generation: u64,
    purpose: WorldComponentPickPurpose,
    kind: WorldComponentKind,
    local_x: f32,
    local_y: f32,
    viewport: Rect,
    view_projection: Mat4,
    origin: Vec3,
    direction: Vec3,
    slot: u16,
    current: Option<WorldInteractionObjectToken>,
    topology: u32,
    merge: u8,
    best: Option<WorldComponentHit>,
    complete: bool,
}

impl WorldComponentPickCursor {
    fn new(state: &World3dState, generation: u64, purpose: WorldComponentPickPurpose, x: f32, y: f32) -> Option<Self> {
        let kind = match state.granularity.as_str() {
            "vertex" => WorldComponentKind::Vertex,
            "edge" => WorldComponentKind::Edge,
            "face" | "component" => WorldComponentKind::Face,
            _ => return None,
        };
        if state.active_object_id.as_ref().is_some_and(|id| id.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY) {
            return None;
        }
        let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
        let camera = state.orbit.to_camera();
        let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
        let view_projection = camera.view_proj(aspect);
        let (origin, direction) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
        Some(Self { revision: state.interaction_revision, generation, purpose, kind, local_x, local_y, viewport, view_projection, origin, direction, slot: 0, current: None, topology: 0, merge: 0, best: None, complete: false })
    }

    fn step(&mut self, state: &World3dState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.revision != state.interaction_revision || self.generation != generation || !state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.current.is_none() {
            let index = usize::from(self.slot);
            let Some(entry) = state.interaction_objects.slots.get(index) else {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            self.slot += 1;
            if let Some(entry) = entry.as_ref().filter(|entry| entry.revision == self.revision && entry.kind == WorldInteractionObjectKind::Instance) {
                if state.active_object_id.as_deref().is_none_or(|active| active == entry.id.as_str()) {
                    self.current = Some(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision });
                    self.topology = 0;
                }
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.current.expect("component cursor current token");
        let Some(instance) = state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(mesh_token) = instance.mesh else {
            return WorldInteractionStep::Fault;
        };
        let Some(admitted) = state.interaction_meshes.resolve(mesh_token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(&mesh) = state.meshes.get(admitted.id.as_str()) else {
            return WorldInteractionStep::Fault;
        };
        let count = match self.kind {
            WorldComponentKind::Vertex => admitted.vertices,
            WorldComponentKind::Edge => admitted.edges,
            WorldComponentKind::Face => admitted.triangles,
        };
        if self.topology >= count {
            self.current = None;
            self.topology = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let index = self.topology;
        self.topology += 1;
        let candidate = match self.kind {
            WorldComponentKind::Vertex => mesh.vec3(Mesh3dField::Positions, index).ok().and_then(|point| {
                let world = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                ui_wgpu::wgpu::project_point(self.view_projection, world, self.viewport.w, self.viewport.h).and_then(|screen| {
                    let dx = screen[0] - self.local_x;
                    let dy = screen[1] - self.local_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    (distance <= PICK_VERTEX_SCREEN_PX).then_some((distance, 0.0))
                })
            }),
            WorldComponentKind::Edge => mesh.edge(index).ok().and_then(|edge| {
                let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                let (Some(screen_a), Some(screen_b)) = (ui_wgpu::wgpu::project_point(self.view_projection, a, self.viewport.w, self.viewport.h), ui_wgpu::wgpu::project_point(self.view_projection, b, self.viewport.w, self.viewport.h)) else {
                    return None;
                };
                let screen_distance = ui_wgpu::wgpu::screen_segment_distance(self.local_x, self.local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]);
                (screen_distance <= PICK_EDGE_SCREEN_PX).then(|| (a.add(b).scale(0.5).sub(self.origin).dot(self.direction), ray_segment_distance(self.origin, self.direction, a, b).unwrap_or(f32::INFINITY)))
            }),
            WorldComponentKind::Face => world_mesh_triangle(mesh, index).and_then(|indices| {
                let a = instance.model.transform_point(world_mesh_vertex(mesh, indices[0])?);
                let b = instance.model.transform_point(world_mesh_vertex(mesh, indices[1])?);
                let c = instance.model.transform_point(world_mesh_vertex(mesh, indices[2])?);
                world_ray_triangle_barycentric(self.origin, self.direction, a, b, c).map(|(distance, _, _)| (distance, 0.0))
            }),
        };
        if let Some((primary, secondary)) = candidate {
            let better = self.best.is_none_or(|best| primary < best.primary - 1e-4 || ((primary - best.primary).abs() <= 1e-4 && secondary < best.secondary));
            if better {
                let id = match self.kind {
                    WorldComponentKind::Vertex => world_mesh_component_id(mesh, Mesh3dField::VertexIds, index),
                    WorldComponentKind::Edge => world_mesh_component_id(mesh, Mesh3dField::EdgeIds, index),
                    WorldComponentKind::Face => world_mesh_component_id(mesh, Mesh3dField::FaceIds, index),
                };
                self.best = Some(WorldComponentHit { object: token, id, primary, secondary });
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    fn finish_plan(&self, state: &World3dState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if self.revision != state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        let object = match self.best {
            Some(hit) => Some((state.interaction_objects.resolve(hit.object).ok_or(WorldInteractionStep::Stale)?, hit.id)),
            None => None,
        };
        if self.purpose == WorldComponentPickPurpose::Hover {
            let unchanged = match object {
                Some((entry, id)) => {
                    state.hovered_component_id.as_deref().and_then(|value| value.parse::<u32>().ok()) == Some(id)
                        && state.hovered_component_object_id.as_deref() == Some(entry.id.as_str())
                        && state.hovered_component_mode.as_deref() == Some(self.kind.as_str())
                }
                None => state.hovered_component_id.is_none() && state.hovered_component_object_id.is_none() && state.hovered_component_mode.is_none(),
            };
            if unchanged {
                return Ok(None);
            }
        }
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let object_span = object.map(|(entry, _)| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
        let mode = plan.push_string(self.kind.as_str()).ok_or(WorldInteractionStep::Fault)?;
        let merge = if self.purpose == WorldComponentPickPurpose::Select {
            Some(
                plan.push_string(match self.merge {
                    1 => merge_mode_wire_str(MergeMode::Additive),
                    2 => merge_mode_wire_str(MergeMode::Invertive),
                    _ => merge_mode_wire_str(MergeMode::Replace),
                })
                .ok_or(WorldInteractionStep::Fault)?,
            )
        } else {
            None
        };
        let action = WorldFlatAction {
            kind: if self.purpose == WorldComponentPickPurpose::Hover { WorldFlatActionKind::ComponentHover } else { WorldFlatActionKind::ComponentSelect },
            strings: [Some(controller), Some(surface), object_span, Some(mode), merge, None, None, None],
            numbers: [object.map(|(_, id)| id as f64).unwrap_or(0.0), if object.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            number_len: 2,
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        if self.current.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}

impl WorldComponentKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Edge => "edge",
            Self::Face => "face",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldContextTargetKind {
    Vortex,
    Object,
    Reference,
}

struct WorldContextMenuCursor {
    revision: u64,
    generation: u64,
    kind: WorldContextTargetKind,
    id: WorldInteractionId,
    x: f32,
    y: f32,
    slot: u16,
    target: Option<WorldInteractionObjectToken>,
    complete: bool,
}

impl WorldContextMenuCursor {
    fn new(state: &World3dState, generation: u64, x: f32, y: f32) -> Option<Self> {
        let (kind, id) = if let Some(id) = state.hovered_vortex_id.as_deref() {
            (WorldContextTargetKind::Vortex, id)
        } else if state.hovered_component_mode.is_some() {
            (WorldContextTargetKind::Object, state.hovered_component_object_id.as_deref()?)
        } else {
            (WorldContextTargetKind::Reference, state.local_hover_id.as_deref()?.strip_prefix("reference:")?)
        };
        Some(Self { revision: state.interaction_revision, generation, kind, id: WorldInteractionId::new(id)?, x, y, slot: 0, target: None, complete: false })
    }

    fn step(&mut self, state: &World3dState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.revision != state.interaction_revision || self.generation != generation || !state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        let index = usize::from(self.slot);
        let Some(entry) = state.interaction_objects.slots.get(index) else {
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        self.slot += 1;
        let expected_kind = match self.kind {
            WorldContextTargetKind::Vortex => WorldInteractionObjectKind::Vortex,
            WorldContextTargetKind::Object => WorldInteractionObjectKind::Instance,
            WorldContextTargetKind::Reference => WorldInteractionObjectKind::Reference,
        };
        if let Some(entry) = entry.as_ref().filter(|entry| entry.revision == self.revision && entry.kind == expected_kind && entry.id == self.id) {
            self.target = Some(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision });
            self.complete = true;
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    fn finish_plan(&self, state: &World3dState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if self.revision != state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        let Some(target) = self.target else {
            return Ok(None);
        };
        let target = state.interaction_objects.resolve(target).ok_or(WorldInteractionStep::Stale)?;
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let id = plan.push_string(target.id.as_str()).ok_or(WorldInteractionStep::Fault)?;
        let kind = plan
            .push_string(match self.kind {
                WorldContextTargetKind::Vortex => "vortex",
                WorldContextTargetKind::Object => "object",
                WorldContextTargetKind::Reference => "reference",
            })
            .ok_or(WorldInteractionStep::Fault)?;
        let action = WorldFlatAction {
            kind: WorldFlatActionKind::ContextMenu,
            strings: [Some(controller), Some(surface), Some(id), Some(kind), None, None, None, None],
            numbers: [self.x as f64, self.y as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            number_len: 2,
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    fn close_step(&mut self) -> bool {
        if self.target.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}

const WORLD_GUMBALL_SELECTED_CAPACITY: usize = 64;

struct WorldGumballPickCursor {
    revision: u64,
    generation: u64,
    x: f32,
    y: f32,
    slot: u16,
    selected: Box<[Option<WorldInteractionObjectToken>; WORLD_GUMBALL_SELECTED_CAPACITY]>,
    selected_len: u8,
    selected_bytes: u16,
    sum: Vec3,
    pivot: Option<Vec3>,
    origin: Vec3,
    direction: Vec3,
    eye: Vec3,
    extent: f32,
    handle: u8,
    validate: u8,
    best: Option<(f32, GumballHandle)>,
    complete: bool,
    faulted: bool,
}

struct WorldGumballGesture {
    revision: u64,
    start_generation: u64,
    handle: GumballHandle,
    pivot: Vec3,
    anchor: f32,
    start: Vec3,
    translate: Vec3,
    angle: f32,
    scale: Vec3,
    selected: Box<[Option<WorldInteractionObjectToken>; WORLD_GUMBALL_SELECTED_CAPACITY]>,
    selected_len: u8,
    selected_bytes: u16,
    validation: u8,
    pending: Option<WorldGumballUpdate>,
}

#[derive(Clone, Copy)]
struct WorldGumballUpdate {
    generation: u64,
    x: f32,
    y: f32,
}

impl WorldGumballPickCursor {
    fn new(state: &World3dState, generation: u64, x: f32, y: f32) -> Self {
        Self {
            revision: state.interaction_revision,
            generation,
            x,
            y,
            slot: 0,
            selected: Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]),
            selected_len: 0,
            selected_bytes: 0,
            sum: Vec3::ZERO,
            pivot: None,
            origin: Vec3::ZERO,
            direction: Vec3::ZERO,
            eye: Vec3::ZERO,
            extent: 0.0,
            handle: 0,
            validate: 0,
            best: None,
            complete: false,
            faulted: false,
        }
    }

    fn step(&mut self, state: &World3dState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != state.interaction_revision || self.generation != generation || !state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.pivot.is_none() {
            let index = usize::from(self.slot);
            if let Some(entry) = state.interaction_objects.slots.get(index) {
                self.slot += 1;
                if let Some(entry) = entry.as_ref().filter(|entry| entry.revision == self.revision && entry.kind == WorldInteractionObjectKind::Instance && entry.values[2] != 0.0) {
                    let selected_index = usize::from(self.selected_len);
                    if selected_index == WORLD_GUMBALL_SELECTED_CAPACITY {
                        self.faulted = true;
                        return WorldInteractionStep::Fault;
                    }
                    self.selected[selected_index] = Some(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision });
                    let Some(selected_bytes) = usize::from(self.selected_bytes).checked_add(entry.id.as_str().len()).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY) else {
                        self.faulted = true;
                        return WorldInteractionStep::Fault;
                    };
                    self.selected_bytes = selected_bytes as u16;
                    self.selected_len += 1;
                    self.sum = self.sum.add(Vec3::new(entry.values[3], entry.values[4], entry.values[5]));
                }
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            if self.selected_len == 0 {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let pivot = state.gumball_target.map(|target| Vec3::new(target[0], target[1], target[2])).unwrap_or_else(|| self.sum.scale(1.0 / f32::from(self.selected_len)));
            let Some((local_x, local_y, viewport)) = pointer_in_pick_rect(state, self.x, self.y) else {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            let camera = state.orbit.to_camera();
            let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
            let (origin, direction) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
            self.pivot = Some(pivot);
            self.origin = origin;
            self.direction = direction;
            self.eye = gumball_eye(&camera, pivot);
            self.extent = gumball_extent(camera.position.sub(pivot).length());
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let handles = [
            GumballHandle::MoveX,
            GumballHandle::MoveY,
            GumballHandle::MoveZ,
            GumballHandle::MoveXY,
            GumballHandle::MoveYZ,
            GumballHandle::MoveXZ,
            GumballHandle::RotateX,
            GumballHandle::RotateY,
            GumballHandle::RotateZ,
            GumballHandle::ScaleX,
            GumballHandle::ScaleY,
            GumballHandle::ScaleZ,
        ];
        let Some(handle) = handles.get(usize::from(self.handle)).copied() else {
            if self.validate < self.selected_len {
                let token = self.selected[usize::from(self.validate)].expect("gumball selected token");
                self.validate += 1;
                if state.interaction_objects.resolve(token).is_none() {
                    return WorldInteractionStep::Stale;
                }
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        self.handle += 1;
        let pivot = self.pivot.expect("gumball pivot resolved");
        let pick_radius = self.extent * 0.08;
        let candidate = if handle.is_translate() && handle.axis_dir().is_some() {
            let axis = handle.axis_dir().expect("translation axis");
            ray_segment_distance(self.origin, self.direction, pivot, pivot.add(axis.scale(self.extent))).filter(|distance| *distance <= pick_radius)
        } else if matches!(handle, GumballHandle::MoveXY | GumballHandle::MoveYZ | GumballHandle::MoveXZ) {
            let normal = handle.plane_normal().expect("translation plane");
            ray_plane_point(self.origin, self.direction, pivot, normal).and_then(|hit| {
                let offset = hit.sub(pivot);
                let u = if normal.z.abs() > 0.9 {
                    offset.x.abs()
                } else if normal.x.abs() > 0.9 {
                    offset.y.abs()
                } else {
                    offset.x.abs()
                };
                let v = if normal.z.abs() > 0.9 { offset.y.abs() } else { offset.z.abs() };
                (u <= self.extent * 0.35 && v <= self.extent * 0.35).then_some(self.origin.sub(hit).length())
            })
        } else if handle.is_rotate() && matches!(state.transform_mode.as_str(), "rotate" | "rotateSelection") {
            let normal = handle.plane_normal().expect("rotation plane");
            ray_plane_point(self.origin, self.direction, pivot, normal).and_then(|hit| {
                let distance = (hit.sub(pivot).length() - self.extent * 0.85).abs();
                (distance <= pick_radius * 2.0).then_some(distance)
            })
        } else if handle.is_scale() && matches!(state.transform_mode.as_str(), "scale" | "scaleSelection") {
            let axis = handle.axis_dir().expect("scale axis");
            ray_segment_distance(self.origin, self.direction, pivot, pivot.add(axis.scale(self.extent * 1.1))).filter(|distance| *distance <= pick_radius)
        } else {
            None
        };
        if let Some(distance) = candidate {
            if self.best.is_none_or(|(best, _)| distance < best) {
                self.best = Some((distance, handle));
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    fn finish(mut self, state: &World3dState, generation: u64) -> Result<Option<WorldGumballGesture>, (Self, WorldInteractionStep)> {
        if !self.complete {
            return Err((self, WorldInteractionStep::Pending));
        }
        if self.revision != state.interaction_revision || self.generation != generation {
            return Err((self, WorldInteractionStep::Stale));
        }
        let Some((_, handle)) = self.best else {
            return Ok(None);
        };
        let pivot = self.pivot.expect("gumball terminal pivot");
        let anchor = handle.axis_dir().and_then(|axis| gumball_project_ray_onto_axis(self.origin, self.direction, pivot, axis, self.eye)).unwrap_or(0.0);
        let start = handle.plane_normal().and_then(|normal| ray_plane_point(self.origin, self.direction, pivot, normal)).map(|point| if handle.is_rotate() { point.sub(pivot) } else { point }).unwrap_or(pivot);
        Ok(Some(WorldGumballGesture {
            revision: self.revision,
            start_generation: self.generation,
            handle,
            pivot,
            anchor,
            start,
            translate: Vec3::ZERO,
            angle: 0.0,
            scale: Vec3::new(1.0, 1.0, 1.0),
            selected: std::mem::replace(&mut self.selected, Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY])),
            selected_len: self.selected_len,
            selected_bytes: self.selected_bytes,
            validation: 0,
            pending: None,
        }))
    }

    fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        if self.selected_len > 0 {
            self.selected_len -= 1;
            self.selected[usize::from(self.selected_len)] = None;
            return false;
        }
        self.selected_bytes = 0;
        self.complete = true;
        true
    }
}

impl WorldGumballGesture {
    fn begin_update(&mut self, generation: u64, x: f32, y: f32) -> WorldInteractionStep {
        if generation <= self.start_generation {
            return WorldInteractionStep::Stale;
        }
        match self.pending {
            None => {
                self.pending = Some(WorldGumballUpdate { generation, x, y });
                self.validation = 0;
                WorldInteractionStep::Pending
            }
            Some(pending) if pending.generation == generation && pending.x == x && pending.y == y => WorldInteractionStep::Pending,
            Some(_) => WorldInteractionStep::Fault,
        }
    }

    fn update_step(&mut self, state: &World3dState) -> WorldInteractionStep {
        if self.revision != state.interaction_revision {
            return WorldInteractionStep::Stale;
        }
        let Some(pending) = self.pending else {
            return WorldInteractionStep::Fault;
        };
        if self.validation < self.selected_len {
            let token = self.selected[usize::from(self.validation)].expect("gumball selected token");
            self.validation += 1;
            return if state.interaction_objects.resolve(token).is_some() { WorldInteractionStep::Pending } else { WorldInteractionStep::Stale };
        }
        let Some((local_x, local_y, viewport)) = pointer_in_pick_rect(state, pending.x, pending.y) else {
            self.pending = None;
            self.validation = 0;
            return WorldInteractionStep::Pending;
        };
        let camera = state.orbit.to_camera();
        let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
        let (origin, direction) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
        let eye = gumball_eye(&camera, self.pivot);
        self.translate = Vec3::ZERO;
        self.angle = 0.0;
        self.scale = Vec3::new(1.0, 1.0, 1.0);
        if self.handle.is_translate() {
            if let Some(axis) = self.handle.axis_dir() {
                if let Some(current) = gumball_project_ray_onto_axis(origin, direction, self.pivot, axis, eye) {
                    self.translate = axis.normalize().scale(current - self.anchor);
                }
            } else if let Some(normal) = self.handle.plane_normal() {
                if let Some(current) = ray_plane_point(origin, direction, self.pivot, normal) {
                    self.translate = current.sub(self.start);
                }
            }
        } else if self.handle.is_rotate() {
            if let Some(normal) = self.handle.plane_normal() {
                if let Some(current) = ray_plane_point(origin, direction, self.pivot, normal) {
                    self.angle = axis_rotate_angle(self.start, current.sub(self.pivot), normal);
                }
            }
        } else if let Some(axis) = self.handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, direction, self.pivot, axis, eye) {
                let factor = if self.anchor.abs() > 1e-4 { (current / self.anchor).clamp(0.05, 20.0) } else { 1.0 };
                match self.handle {
                    GumballHandle::ScaleX => self.scale.x = factor,
                    GumballHandle::ScaleY => self.scale.y = factor,
                    GumballHandle::ScaleZ => self.scale.z = factor,
                    _ => {}
                }
            }
        }
        self.pending = None;
        self.validation = 0;
        WorldInteractionStep::Complete
    }

    fn close_step(&mut self) -> bool {
        if self.pending.take().is_some() {
            self.validation = 0;
            return false;
        }
        if self.selected_len > 0 {
            self.selected_len -= 1;
            self.selected[usize::from(self.selected_len)] = None;
            return false;
        }
        self.selected_bytes = 0;
        true
    }
}

struct WorldGumballCommitJob {
    generation: u64,
    gesture: WorldGumballGesture,
    claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    stage: u16,
    complete: bool,
}

const WORLD_BRUSH_COPY_CHUNK_BYTES: usize = 256;

#[derive(Clone, Copy)]
enum WorldBrushScale {
    Null,
    Number(f64),
    Array { values: [f64; 3], len: u8 },
}

struct WorldBrushCommitJob {
    generation: u64,
    revision: u64,
    bytes: Box<[u8; WORLD_INTERACTION_BYTE_CAPACITY]>,
    target: WorldInteractionSpan,
    kind: WorldInteractionSpan,
    source_index: usize,
    origin: [f64; 3],
    orientation: [f64; 4],
    scale: WorldBrushScale,
    copy_field: u8,
    copy_offset: u16,
    validating: bool,
    claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    stage: u8,
    scale_stage: u8,
    complete: bool,
}

impl WorldBrushCommitJob {
    fn new(state: &World3dState, generation: u64) -> Result<Option<Self>, ui_wgpu::wgpu::BoundedActionFault> {
        let Some(preview) = state.brush_preview.as_ref() else {
            return Ok(None);
        };
        let (Some(target), Some(kind), Some(source_index)) = (preview.target_vortex_full_id.as_deref(), preview.object_kind_id.as_deref(), preview.source_vortex_index) else {
            return Ok(None);
        };
        if source_index > u32::MAX as usize {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        if target.len() > ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY || kind.len() > ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        let kind_start = target.len();
        kind_start.checked_add(kind.len()).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let scale = match preview.scale.as_ref() {
            None | Some(serde_json::Value::Null) => WorldBrushScale::Null,
            Some(serde_json::Value::Number(value)) => WorldBrushScale::Number(value.as_f64().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?),
            Some(serde_json::Value::Array(values)) if values.len() <= 3 => {
                let mut scalars = [0.0; 3];
                for index in 0..values.len() {
                    scalars[index] = values[index].as_f64().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                }
                WorldBrushScale::Array { values: scalars, len: values.len() as u8 }
            }
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        };
        Ok(Some(Self {
            generation,
            revision: state.interaction_revision,
            bytes: Box::new([0; WORLD_INTERACTION_BYTE_CAPACITY]),
            target: WorldInteractionSpan { start: 0, len: target.len() as u16 },
            kind: WorldInteractionSpan { start: kind_start as u16, len: kind.len() as u16 },
            source_index,
            origin: preview.origin.unwrap_or([0.0, 0.0, 0.0]),
            orientation: preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
            scale,
            copy_field: 0,
            copy_offset: 0,
            validating: false,
            claim: None,
            draft: None,
            stage: 0,
            scale_stage: 0,
            complete: false,
        }))
    }

    fn text(&self, span: WorldInteractionSpan) -> Result<&str, ui_wgpu::wgpu::BoundedActionFault> {
        let start = usize::from(span.start);
        let end = start.checked_add(usize::from(span.len)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        std::str::from_utf8(self.bytes.get(start..end).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }

    fn source<'a>(&self, state: &'a World3dState) -> Result<(&'a str, &'a str), ui_wgpu::wgpu::BoundedActionFault> {
        let preview = state.brush_preview.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let target = preview.target_vortex_full_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let kind = preview.object_kind_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if target.len() != usize::from(self.target.len) || kind.len() != usize::from(self.kind.len) || preview.source_vortex_index != Some(self.source_index) {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        Ok((target, kind))
    }

    fn copy_step(&mut self, state: &World3dState) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        let (target, kind) = self.source(state)?;
        let (source, span) = if self.copy_field == 0 { (target.as_bytes(), self.target) } else { (kind.as_bytes(), self.kind) };
        let offset = usize::from(self.copy_offset);
        if offset == source.len() {
            self.copy_field += 1;
            self.copy_offset = 0;
            if self.copy_field == 2 {
                if self.validating {
                    return Ok(true);
                }
                self.validating = true;
                self.copy_field = 0;
            }
            return Ok(false);
        }
        let take = (source.len() - offset).min(WORLD_BRUSH_COPY_CHUNK_BYTES);
        let destination = usize::from(span.start) + offset;
        let existing = &mut self.bytes[destination..destination + take];
        if self.validating {
            if existing != &source[offset..offset + take] {
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            }
        } else {
            existing.copy_from_slice(&source[offset..offset + take]);
        }
        self.copy_offset += take as u16;
        Ok(false)
    }

    fn credit(&self, state: &World3dState) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
        ui_wgpu::wgpu::checked_action_string_bytes(&[&state.controller_id, "addBrushObject", "targetVortexFullId", self.text(self.target)?, "objectKindId", self.text(self.kind)?, "sourceVortexIndex", "origin", "orientation", "scale"])
    }

    fn step(&mut self, state: &World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.revision != state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if !self.complete {
            self.complete = self.copy_step(state)?;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.claim.is_none() {
            self.claim = Some(input.claim_action(self.credit(state)?)?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() {
            self.draft = Some(input.draft_claimed_action(self.claim.expect("brush claim"), &state.controller_id, "addBrushObject")?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let target_start = usize::from(self.target.start);
        let target_end = target_start + usize::from(self.target.len);
        let kind_start = usize::from(self.kind.start);
        let kind_end = kind_start + usize::from(self.kind.len);
        let target = std::str::from_utf8(&self.bytes[target_start..target_end]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let kind = std::str::from_utf8(&self.bytes[kind_start..kind_end]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let draft = self.draft.as_mut().expect("brush draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("targetVortexFullId"), target)?,
            2 => draft.builder().string(Some("objectKindId"), kind)?,
            3 => draft.builder().number(Some("sourceVortexIndex"), self.source_index as f64)?,
            4 => draft.builder().begin_array(Some("origin"))?,
            5..=7 => draft.builder().number(None, self.origin[usize::from(self.stage - 5)])?,
            8 => draft.builder().end_container()?,
            9 => draft.builder().begin_array(Some("orientation"))?,
            10..=13 => draft.builder().number(None, self.orientation[usize::from(self.stage - 10)])?,
            14 => draft.builder().end_container()?,
            15 => match self.scale {
                WorldBrushScale::Null => draft.builder().null(Some("scale"))?,
                WorldBrushScale::Number(value) => draft.builder().number(Some("scale"), value)?,
                WorldBrushScale::Array { .. } => draft.builder().begin_array(Some("scale"))?,
            },
            16 if matches!(self.scale, WorldBrushScale::Array { .. }) => {
                let WorldBrushScale::Array { values, len } = self.scale else { unreachable!("array scale") };
                if self.scale_stage < len {
                    draft.builder().number(None, values[usize::from(self.scale_stage)])?;
                    self.scale_stage += 1;
                    context.consume_fuel(1);
                    return Ok(WorldInteractionStep::Pending);
                }
                draft.builder().end_container()?;
            }
            16 => draft.builder().end_container()?,
            17 if matches!(self.scale, WorldBrushScale::Array { .. }) => draft.builder().end_container()?,
            _ => {
                let prepared = self.draft.take().expect("terminal brush draft").finish()?;
                input.publish_prepared_claimed_action(prepared)?;
                self.claim = None;
                return Ok(WorldInteractionStep::Complete);
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(claim) = self.claim.take() {
            if input.release_action_claim(claim).is_err() {
                self.claim = Some(claim);
            }
            return false;
        }
        true
    }
}

impl WorldGumballCommitJob {
    fn new(generation: u64, gesture: WorldGumballGesture) -> Self {
        Self { generation, gesture, claim: None, draft: None, stage: 0, complete: false }
    }

    fn action_id(&self) -> &'static str {
        if self.gesture.handle.is_translate() {
            "translateSelection"
        } else if self.gesture.handle.is_rotate() {
            "rotateSelection"
        } else {
            "scaleSelection"
        }
    }

    fn string_bytes(&self, state: &World3dState) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
        let keys: &[&str] = if self.gesture.handle.is_translate() {
            &["surfaceId", "mode", "ids", "dx", "dy", "dz"]
        } else if self.gesture.handle.is_rotate() {
            &["surfaceId", "mode", "ids", "ax", "ay", "az", "angle"]
        } else {
            &["surfaceId", "mode", "ids", "sx", "sy", "sz"]
        };
        let mut bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&state.controller_id, self.action_id(), &state.surface_id, "mesh"])?;
        for key in keys {
            bytes = bytes.checked_add(key.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        }
        bytes = bytes.checked_add(usize::from(self.gesture.selected_bytes)).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        if bytes > WORLD_INTERACTION_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        Ok(bytes)
    }

    fn numeric(&self, index: usize) -> Option<(&'static str, f64)> {
        if self.gesture.handle.is_translate() {
            [("dx", self.gesture.translate.x as f64), ("dy", self.gesture.translate.y as f64), ("dz", self.gesture.translate.z as f64)].get(index).copied()
        } else if self.gesture.handle.is_rotate() {
            let axis = self.gesture.handle.axis_dir().unwrap_or(Vec3::ZERO);
            [("ax", axis.x as f64), ("ay", axis.y as f64), ("az", axis.z as f64), ("angle", self.gesture.angle as f64)].get(index).copied()
        } else {
            [("sx", self.gesture.scale.x as f64), ("sy", self.gesture.scale.y as f64), ("sz", self.gesture.scale.z as f64)].get(index).copied()
        }
    }

    fn step(&mut self, state: &World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.complete {
            if self.gesture.selected_len > 0 {
                self.gesture.selected_len -= 1;
                self.gesture.selected[usize::from(self.gesture.selected_len)] = None;
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            self.gesture.selected_bytes = 0;
            return Ok(WorldInteractionStep::Complete);
        }
        if self.gesture.revision != state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if self.claim.is_none() {
            let claim = input.claim_action(self.string_bytes(state)?)?;
            self.claim = Some(claim);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() {
            self.draft = Some(input.draft_claimed_action(self.claim.expect("gumball claim"), &state.controller_id, self.action_id())?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let selected_end = 4 + u16::from(self.gesture.selected_len);
        let selected = if self.stage >= 4 && self.stage < selected_end {
            let token = self.gesture.selected[usize::from(self.stage - 4)].expect("gumball selected token");
            Some(state.interaction_objects.resolve(token).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?.id)
        } else {
            None
        };
        let numeric = (self.stage > selected_end).then(|| self.numeric(usize::from(self.stage - selected_end - 1))).flatten();
        let draft = self.draft.as_mut().expect("gumball draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("surfaceId"), &state.surface_id)?,
            2 => draft.builder().string(Some("mode"), "mesh")?,
            3 => draft.builder().begin_array(Some("ids"))?,
            stage if usize::from(stage - 4) < usize::from(self.gesture.selected_len) => {
                draft.builder().string(None, selected.expect("selected id resolved").as_str())?;
            }
            stage if stage == 4 + u16::from(self.gesture.selected_len) => draft.builder().end_container()?,
            _ => {
                if let Some((key, value)) = numeric {
                    draft.builder().number(Some(key), value)?;
                } else {
                    draft.builder().end_container()?;
                    let prepared = self.draft.take().expect("complete gumball draft").finish()?;
                    input.publish_prepared_claimed_action(prepared)?;
                    self.claim = None;
                    self.complete = true;
                    context.consume_fuel(1);
                    return Ok(WorldInteractionStep::Pending);
                }
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(claim) = self.claim.take() {
            if input.release_action_claim(claim).is_err() {
                self.claim = Some(claim);
            }
            return false;
        }
        self.gesture.close_step()
    }

    fn terminal_is_empty(&self) -> bool {
        self.complete && self.claim.is_none() && self.draft.is_none() && self.gesture.selected_len == 0 && self.gesture.selected_bytes == 0 && self.gesture.pending.is_none()
    }
}

fn world_mesh_vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    let point = mesh.vec3(Mesh3dField::Positions, index).ok()?;
    Some(Vec3::new(point[0], point[1], point[2]))
}

fn world_mesh_triangle(mesh: Mesh3dLease, triangle: u32) -> Option<[u32; 3]> {
    let base = triangle.checked_mul(3)?;
    Some([mesh.u32(Mesh3dField::Indices, base).ok()?, mesh.u32(Mesh3dField::Indices, base + 1).ok()?, mesh.u32(Mesh3dField::Indices, base + 2).ok()?])
}

fn world_mesh_component_id(mesh: Mesh3dLease, field: Mesh3dField, index: u32) -> u32 {
    mesh.u32(field, index).unwrap_or(index)
}

fn world_ray_triangle_barycentric(origin: Vec3, direction: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, f32, f32)> {
    let edge1 = b.sub(a);
    let edge2 = c.sub(a);
    let h = direction.cross(edge2);
    let determinant = edge1.dot(h);
    if determinant.abs() < 1e-8 {
        return None;
    }
    let inverse = 1.0 / determinant;
    let offset = origin.sub(a);
    let u = inverse * offset.dot(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = offset.cross(edge1);
    let v = inverse * direction.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = inverse * edge2.dot(q);
    (distance > 1e-6).then_some((distance, u, v))
}

pub fn enqueue_world3d_intent(state: &mut World3dState, intent: WorldInteractionIntent) -> Result<(), WorldInteractionIntent> {
    let Some(authority) = state.interaction_authority.as_mut() else {
        return Err(intent);
    };
    if authority.faulted || authority.closing || authority.blocked.is_some() || authority.next_generation == u64::MAX || intent.generation != authority.next_generation {
        return Err(intent);
    }
    match authority.queue.push(intent) {
        Ok(()) => {
            authority.next_generation += 1;
            Ok(())
        }
        Err(intent) if authority.blocked.is_none() => {
            authority.blocked = Some(intent);
            authority.next_generation += 1;
            Ok(())
        }
        Err(intent) => Err(intent),
    }
}

pub fn enqueue_world3d_event(state: &mut World3dState, mut intent: WorldInteractionIntent) -> Result<u64, WorldInteractionIntent> {
    let Some(authority) = state.interaction_authority.as_ref() else {
        return Err(intent);
    };
    intent.generation = authority.next_generation;
    let generation = intent.generation;
    enqueue_world3d_intent(state, intent).map(|()| generation)
}

pub fn enqueue_world3d_events<const N: usize>(state: &mut World3dState, mut intents: [WorldInteractionIntent; N]) -> Result<[u64; N], [WorldInteractionIntent; N]> {
    let Some(authority) = state.interaction_authority.as_mut() else {
        return Err(intents);
    };
    let available = if authority.blocked.is_some() { 0 } else { WORLD_INTERACTION_INTENT_CAPACITY - usize::from(authority.queue.len) + 1 };
    if N == 0 || N > 4 || authority.faulted || authority.closing || authority.next_generation.checked_add(N as u64).is_none() || N > available {
        return Err(intents);
    }
    let mut generations = [0; N];
    for index in 0..N {
        intents[index].generation = authority.next_generation;
        generations[index] = authority.next_generation;
        match authority.queue.push(intents[index]) {
            Ok(()) => {}
            Err(intent) if authority.blocked.is_none() => authority.blocked = Some(intent),
            Err(_) => unreachable!("world intent batch preflight owns exact fixed credits"),
        }
        authority.next_generation += 1;
    }
    Ok(generations)
}

pub fn world3d_interaction_front_generation(state: &World3dState) -> Option<u64> {
    state.interaction_authority.as_ref()?.queue.front().map(|intent| intent.generation)
}

pub fn step_world3d_interaction(state: &mut World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
    let Some(mut authority) = state.interaction_authority.take() else {
        return WorldInteractionAuthorityStep::Fault;
    };
    let step = authority.step(state, generation, input, context);
    state.interaction_authority = Some(authority);
    step
}

pub fn begin_world3d_interaction_close(state: &mut World3dState) {
    if let Some(authority) = state.interaction_authority.as_mut() {
        authority.closing = true;
        authority.queue.begin_close();
    }
}

pub fn close_world3d_interaction_step(state: &mut World3dState, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    let Some(authority) = state.interaction_authority.as_mut() else {
        return true;
    };
    authority.closing = true;
    authority.queue.begin_close();
    if let Some(active) = authority.active.as_mut() {
        let complete = match active {
            WorldInteractionActive::Plan { plan, .. } => plan.close_step(),
            WorldInteractionActive::Pick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::ObjectPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::ComponentPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::ContextMenu { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::MarqueePick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::MarqueePublish { job, .. } => job.close_step(input),
            WorldInteractionActive::ComponentMarqueePublish { job, .. } => job.close_step(input),
            WorldInteractionActive::GumballPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::GumballCommit { job, .. } => job.close_step(input),
            WorldInteractionActive::BrushCommit { job, .. } => job.close_step(input),
        };
        context.consume_fuel(1);
        if complete {
            authority.active = None;
        }
        return false;
    }
    if let Some(registry) = authority.registry.as_mut() {
        if registry.close_step(context) {
            authority.registry = None;
        }
        return false;
    }
    if authority.blocked.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if authority.right_press.take().is_some() || authority.right_dragged {
        authority.right_dragged = false;
        context.consume_fuel(1);
        return false;
    }
    if let Some(marquee) = authority.marquee.as_mut() {
        if marquee.close_step() {
            authority.marquee = None;
        }
        context.consume_fuel(1);
        return false;
    }
    if let Some(gumball) = authority.gumball.as_mut() {
        if gumball.close_step() {
            authority.gumball = None;
        }
        context.consume_fuel(1);
        return false;
    }
    let complete = authority.queue.close_step();
    context.consume_fuel(1);
    complete
}

pub fn world3d_interaction_terminal_is_empty(state: &World3dState) -> bool {
    state.interaction_authority.as_ref().is_none_or(|authority| {
        authority.closing
            && authority.registry.is_none()
            && authority.active.is_none()
            && authority.right_press.is_none()
            && !authority.right_dragged
            && authority.marquee.is_none()
            && authority.gumball.is_none()
            && authority.blocked.is_none()
            && authority.queue.terminal_is_empty()
    })
}

impl WorldInteractionAuthority {
    fn step(&mut self, state: &mut World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
        if self.closing || context.should_yield() {
            return WorldInteractionAuthorityStep::Pending;
        }
        if let Some(active) = self.active.take() {
            return self.step_active(active, state, generation, input, context);
        }
        if self.faulted {
            return WorldInteractionAuthorityStep::Fault;
        }
        if self.marquee.as_ref().is_some_and(|marquee| marquee.revision != state.interaction_revision || marquee.retiring) {
            let complete = self.marquee.as_mut().expect("stale marquee owner").close_step();
            if complete {
                self.marquee = None;
            }
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if self.gumball.as_ref().is_some_and(|gumball| gumball.revision != state.interaction_revision) {
            let complete = self.gumball.as_mut().expect("stale gumball owner").close_step();
            if complete {
                self.gumball = None;
            }
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if state.interaction_objects.revision != state.interaction_revision || self.registry.is_some() {
            let mut registry = self.registry.take().unwrap_or_else(|| WorldInteractionRegistryBuildCursor::new(state.interaction_revision));
            match registry.step(state, context) {
                WorldInteractionStep::Pending => {
                    self.registry = Some(registry);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Complete => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Stale => {
                    self.registry = Some(WorldInteractionRegistryBuildCursor::new(state.interaction_revision));
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Stale;
                }
                WorldInteractionStep::Fault => {
                    self.registry = Some(registry);
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
            }
        }
        if let Some(blocked) = self.blocked.take() {
            match self.queue.push(blocked) {
                Ok(()) => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                Err(blocked) => {
                    self.blocked = Some(blocked);
                }
            }
        }
        let Some(intent) = self.queue.front().copied() else {
            return WorldInteractionAuthorityStep::Idle;
        };
        if intent.generation < generation {
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if intent.generation > generation {
            self.faulted = true;
            return WorldInteractionAuthorityStep::Fault;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 2 && intent.down {
            self.right_press = Some([intent.x, intent.y]);
            self.right_dragged = false;
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerMove && intent.button == 2 && intent.down {
            if let Some(start) = self.right_press {
                let dx = intent.x - start[0];
                let dy = intent.y - start[1];
                self.right_dragged |= (dx * dx + dy * dy).sqrt() > CLICK_DRAG_THRESHOLD_PX;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 2 && !intent.down {
            let click = self.right_press.take().is_some() && !self.right_dragged;
            self.right_dragged = false;
            if !click {
                self.queue.retire_front(intent.generation);
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Complete;
            }
            let Some(cursor) = WorldContextMenuCursor::new(state, generation, intent.x, intent.y) else {
                self.queue.retire_front(intent.generation);
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Complete;
            };
            self.active = Some(WorldInteractionActive::ContextMenu { cursor, retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && intent.down && state.active_utility == "select" && !component_mode_active(state) && self.gumball.is_none() {
            self.active = Some(WorldInteractionActive::GumballPick { cursor: WorldGumballPickCursor::new(state, generation, intent.x, intent.y), retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        if intent.phase == WorldInteractionPhase::PointerMove && intent.button == 0 && intent.down && self.gumball.is_some() {
            let gumball = self.gumball.as_mut().expect("gumball gesture retained above");
            match gumball.begin_update(intent.generation, intent.x, intent.y) {
                WorldInteractionStep::Stale => return WorldInteractionAuthorityStep::Stale,
                WorldInteractionStep::Fault => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
                _ => {}
            }
            match gumball.update_step(state) {
                WorldInteractionStep::Pending => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Stale => return WorldInteractionAuthorityStep::Stale,
                WorldInteractionStep::Fault => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
                WorldInteractionStep::Complete => {}
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && !intent.down && self.gumball.is_some() {
            let gesture = self.gumball.take().expect("gumball gesture retained above");
            self.active = Some(WorldInteractionActive::GumballCommit { job: WorldGumballCommitJob::new(intent.generation, gesture), retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && !intent.down && state.active_utility == "brush" {
            match WorldBrushCommitJob::new(state, intent.generation) {
                Ok(Some(job)) => {
                    self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                Ok(None) => {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                Err(_) => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
            }
        }
        let select_marquee_route = state.active_utility == "select";
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && intent.down && select_marquee_route {
            if self.marquee.is_some() {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            self.marquee = Some(WorldMarqueeGesture::new(state.interaction_revision, intent.generation, [intent.x, intent.y]));
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerMove && intent.button == 0 && intent.down && self.marquee.is_some() {
            let marquee = self.marquee.as_mut().expect("marquee gesture retained above");
            if intent.generation <= marquee.start_generation || !marquee.push([intent.x, intent.y]) {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && !intent.down && self.marquee.is_some() {
            let marquee = self.marquee.as_mut().expect("marquee gesture retained above");
            if intent.generation <= marquee.start_generation {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            if marquee.is_click([intent.x, intent.y]) {
                marquee.retiring = true;
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            let marquee = self.marquee.take().expect("non-click marquee owner");
            let Some(cursor) = WorldMarqueePickCursor::new(state, intent.generation, marquee) else {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            };
            self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        let active = match intent.phase {
            WorldInteractionPhase::Wheel => plan_world3d_wheel(state, generation, intent.delta).map(|plan| WorldInteractionActive::Plan { plan, retirement: None }),
            WorldInteractionPhase::PointerDrag => {
                let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
                let Some(plan) = plan_world3d_drag(state, generation, intent.dx, intent.dy, intent.button, &modifiers) else {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                };
                Some(WorldInteractionActive::Plan { plan, retirement: None })
            }
            WorldInteractionPhase::PointerMove => {
                let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
                if intent.down {
                    if let Some(plan) = plan_world3d_drag(state, generation, intent.dx, intent.dy, intent.button, &modifiers) {
                        Some(WorldInteractionActive::Plan { plan, retirement: None })
                    } else if intent.button == 0 && state.interaction_mode == "paint" && state.paint_stroke_active {
                        WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Paint, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                    } else {
                        None
                    }
                } else {
                    if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
                        WorldObjectPickCursor::new(state, generation, WorldObjectPickPurpose::VortexHover, intent.x, intent.y).map(|cursor| WorldInteractionActive::ObjectPick { cursor, retirement: None })
                    } else if component_mode_active(state) {
                        WorldComponentPickCursor::new(state, generation, WorldComponentPickPurpose::Hover, intent.x, intent.y).map(|cursor| WorldInteractionActive::ComponentPick { cursor, retirement: None })
                    } else {
                        WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Hover, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                    }
                }
            }
            WorldInteractionPhase::PointerButton => {
                if let Some(plan) = plan_world3d_paint_stroke(state, generation, intent.down, intent.button) {
                    Some(WorldInteractionActive::Plan { plan, retirement: None })
                } else if !intent.down && intent.button == 0 && state.active_utility == "surfaceBrush" {
                    WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Surface, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                } else if intent.down && intent.button == 0 && (state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex")) {
                    WorldObjectPickCursor::new(state, generation, WorldObjectPickPurpose::VortexSelect, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = if intent.shift {
                            1
                        } else if intent.ctrl {
                            2
                        } else {
                            0
                        };
                        WorldInteractionActive::ObjectPick { cursor, retirement: None }
                    })
                } else if !intent.down && intent.button == 0 && state.active_utility == "select" && component_mode_active(state) {
                    WorldComponentPickCursor::new(state, generation, WorldComponentPickPurpose::Select, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = if intent.shift {
                            1
                        } else if intent.ctrl {
                            2
                        } else {
                            0
                        };
                        WorldInteractionActive::ComponentPick { cursor, retirement: None }
                    })
                } else if intent.down && intent.button == 0 && state.active_utility == "select" && component_mode_active(state) {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                } else if intent.button == 0 && state.active_utility == "select" && !component_mode_active(state) {
                    if intent.down {
                        self.queue.retire_front(intent.generation);
                        context.consume_fuel(1);
                        return WorldInteractionAuthorityStep::Complete;
                    }
                    WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Instance, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = if intent.shift {
                            1
                        } else if intent.ctrl {
                            2
                        } else {
                            0
                        };
                        WorldInteractionActive::Pick { cursor, retirement: None }
                    })
                } else {
                    None
                }
            }
            WorldInteractionPhase::Close => {
                self.closing = true;
                self.queue.begin_close();
                None
            }
            _ => None,
        };
        let Some(active) = active else {
            if intent.phase == WorldInteractionPhase::Close {
                return WorldInteractionAuthorityStep::Pending;
            }
            self.faulted = true;
            return WorldInteractionAuthorityStep::Fault;
        };
        self.active = Some(active);
        context.consume_fuel(1);
        WorldInteractionAuthorityStep::Pending
    }

    fn step_active(&mut self, active: WorldInteractionActive, state: &mut World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
        match active {
            WorldInteractionActive::Plan { mut plan, retirement } => {
                if let Some(outcome) = retirement {
                    if plan.close_step() {
                        self.queue.retire_front(plan.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match publish_world3d_plan_step(state, &mut plan, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(plan.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::Pick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete if cursor.purpose == WorldRayPickPurpose::Hover && cursor.best.is_none() => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor: WorldObjectPickCursor::from_ray(cursor.revision, cursor.generation, WorldObjectPickPurpose::ReferenceHover, cursor.origin, cursor.direction), retirement: None });
                        context.consume_fuel(1);
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ObjectPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ComponentPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ContextMenu { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ContextMenu { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ContextMenu { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::ContextMenu { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::ContextMenu { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::ContextMenu { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::MarqueePick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => {
                        let cursor_generation = cursor.generation;
                        let component_kind = cursor.component_kind;
                        match cursor.finish(state, generation) {
                            Ok((gesture, results)) => {
                                let Some(intent) = self.queue.front().copied().filter(|intent| intent.generation == cursor_generation) else {
                                    self.faulted = true;
                                    return WorldInteractionAuthorityStep::Fault;
                                };
                                self.active = if let Some(kind) = component_kind {
                                    Some(WorldInteractionActive::ComponentMarqueePublish { job: WorldComponentMarqueePublishJob::new(cursor_generation, gesture, results, kind, intent.shift, intent.ctrl), retirement: None })
                                } else {
                                    Some(WorldInteractionActive::MarqueePublish { job: WorldMarqueePublishJob::new(cursor_generation, gesture, results, intent.shift, intent.ctrl), retirement: None })
                                };
                                context.consume_fuel(1);
                                WorldInteractionAuthorityStep::Pending
                            }
                            Err((cursor, outcome)) => {
                                let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                                self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(outcome) });
                                WorldInteractionAuthorityStep::Pending
                            }
                        }
                    }
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::MarqueePublish { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.prepared.is_none() => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ComponentMarqueePublish { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::GumballPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => {
                        let cursor_generation = cursor.generation;
                        let cursor_revision = cursor.revision;
                        let point = [cursor.x, cursor.y];
                        match cursor.finish(state, generation) {
                            Ok(Some(gesture)) => {
                                self.gumball = Some(gesture);
                                self.queue.retire_front(cursor_generation);
                                WorldInteractionAuthorityStep::Complete
                            }
                            Ok(None) => {
                                self.marquee = Some(WorldMarqueeGesture::new(cursor_revision, cursor_generation, point));
                                self.queue.retire_front(cursor_generation);
                                WorldInteractionAuthorityStep::Complete
                            }
                            Err((cursor, outcome)) => {
                                let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                                self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(outcome) });
                                WorldInteractionAuthorityStep::Pending
                            }
                        }
                    }
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::GumballCommit { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) if job.terminal_is_empty() => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::BrushCommit { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
        }
    }
}

pub fn plan_world3d_wheel(state: &World3dState, generation: u64, delta: f32) -> Option<WorldInteractionPlan> {
    let mut next = state.orbit.clone();
    next.zoom(delta);
    let camera = next.to_camera();
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let action = WorldFlatAction {
        kind: WorldFlatActionKind::Camera,
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64, camera.target.x as f64, camera.target.y as f64, camera.target.z as f64, next.fov_y.to_degrees() as f64, delta as f64, 0.0, 0.0],
        number_len: 9,
    };
    plan.push_action(action).then_some(plan)
}

pub fn plan_world3d_drag(state: &World3dState, generation: u64, dx: f32, dy: f32, button: i16, modifiers: &PointerModifiers) -> Option<WorldInteractionPlan> {
    let operation = if button == 1 || (button == 2 && modifiers.shift) {
        1.0
    } else if button == 2 && (modifiers.alt || modifiers.meta) {
        2.0
    } else {
        return None;
    };
    let mut next = state.orbit.clone();
    if operation == 1.0 {
        next.pan(-dx, -dy);
    } else {
        next.orbit(dx, dy);
    }
    let camera = next.to_camera();
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let action = WorldFlatAction {
        kind: WorldFlatActionKind::Camera,
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64, camera.target.x as f64, camera.target.y as f64, camera.target.z as f64, next.fov_y.to_degrees() as f64, dx as f64, operation, dy as f64],
        number_len: 10,
    };
    plan.push_action(action).then_some(plan)
}

pub fn plan_world3d_paint_stroke(state: &World3dState, generation: u64, down: bool, button: i16) -> Option<WorldInteractionPlan> {
    if state.interaction_mode != "paint" || button != 0 || state.paint_stroke_active == down {
        return None;
    }
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let action = WorldFlatAction {
        kind: if down { WorldFlatActionKind::PaintStrokeBegin } else { WorldFlatActionKind::PaintStrokeEnd },
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [if down { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        number_len: 1,
    };
    plan.push_action(action).then_some(plan)
}

pub fn publish_world3d_plan_step(
    state: &mut World3dState,
    plan: &mut WorldInteractionPlan,
    generation: u64,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    context: &mut semio_framework_job::StepContext<'_>,
) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
    if context.should_yield() {
        return Ok(WorldInteractionStep::Pending);
    }
    if plan.faulted {
        return Ok(WorldInteractionStep::Fault);
    }
    if plan.generation != generation || plan.revision != state.interaction_revision {
        return Ok(WorldInteractionStep::Stale);
    }
    if plan.cursor == plan.action_len {
        plan.byte_len = 0;
        return Ok(WorldInteractionStep::Complete);
    }
    let action = plan.actions[usize::from(plan.cursor)].expect("world interaction action cursor remains admitted");
    match action.kind {
        WorldFlatActionKind::Camera => {
            let controller = plan.string(action.strings[0].expect("camera controller span"));
            let surface = plan.string(action.strings[1].expect("camera surface span"));
            let action_id = "setCamera";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "camera", "position", "target", "fov"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.begin_object(Some("camera"))?;
            builder.begin_array(Some("position"))?;
            for value in &action.numbers[..3] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.begin_array(Some("target"))?;
            for value in &action.numbers[3..6] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.number(Some("fov"), action.numbers[6])?;
            builder.end_container()?;
            builder.end_container()?;
            let first = action.numbers[7] as f32;
            let operation = action.numbers[8] as u8;
            let second = action.numbers[9] as f32;
            reservation.publish_with(|| {
                match operation {
                    0 => state.orbit.zoom(first),
                    1 => state.orbit.pan(-first, -second),
                    2 => state.orbit.orbit(first, second),
                    _ => unreachable!("world camera plan operation is schema-bounded"),
                }
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::PaintStrokeBegin | WorldFlatActionKind::PaintStrokeEnd => {
            let controller = plan.string(action.strings[0].expect("paint controller span"));
            let surface = plan.string(action.strings[1].expect("paint surface span"));
            let action_id = if action.kind == WorldFlatActionKind::PaintStrokeBegin { "paintStrokeBegin" } else { "paintStrokeEnd" };
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.end_container()?;
            let active = action.numbers[0] != 0.0;
            reservation.publish_with(|| {
                state.paint_stroke_active = active;
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::PaintAt => {
            let controller = plan.string(action.strings[0].expect("paint controller span"));
            let surface = plan.string(action.strings[1].expect("paint surface span"));
            let object = plan.string(action.strings[2].expect("paint object span"));
            let action_id = "paintAt";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "objectId", object, "u", "v"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("objectId"), object)?;
            builder.number(Some("u"), action.numbers[0])?;
            builder.number(Some("v"), action.numbers[1])?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::SurfacePlace => {
            let controller = plan.string(action.strings[0].expect("surface controller span"));
            let surface = plan.string(action.strings[1].expect("surface span"));
            let object = plan.string(action.strings[2].expect("surface object span"));
            let action_id = "worldSurfacePlace";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "pane", surface, "objectId", object, "position", "normal"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("pane"), surface)?;
            builder.string(Some("objectId"), object)?;
            builder.begin_array(Some("position"))?;
            for value in &action.numbers[..3] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.begin_array(Some("normal"))?;
            for value in &action.numbers[3..6] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::Select => {
            let controller = plan.string(action.strings[0].expect("selection controller span"));
            let surface = action.strings[1].map(|span| plan.string(span));
            let object = action.strings[2].map(|span| plan.string(span));
            let domain = plan.string(action.strings[3].expect("selection domain span"));
            let granularity = action.strings[4].map(|span| plan.string(span));
            let merge = plan.string(action.strings[5].expect("selection merge span"));
            let method = plan.string(action.strings[6].expect("selection method span"));
            let action_id = "interactionSelect";
            let bytes = match (surface, object, granularity) {
                (Some(surface), Some(object), Some(granularity)) if action.numbers[0] == 0.0 => {
                    ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "targets", "granularity", granularity, "id", surface, WORLD_ITEM_PATH_DELIMITER, object, "merge", merge, "method", method])?
                }
                (Some(_), Some(object), Some(granularity)) => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "targets", "granularity", granularity, "id", object, "merge", merge, "method", method])?,
                _ => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "targets", "merge", merge, "method", method])?,
            };
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain)?;
            builder.begin_array(Some("targets"))?;
            if let (Some(surface), Some(object), Some(granularity)) = (surface, object, granularity) {
                builder.begin_object(None)?;
                builder.string(Some("granularity"), granularity)?;
                if action.numbers[0] != 0.0 {
                    builder.string(Some("id"), object)?;
                } else {
                    builder.string_joined(Some("id"), &[surface, WORLD_ITEM_PATH_DELIMITER, object])?;
                }
                builder.end_container()?;
            }
            builder.end_container()?;
            builder.string(Some("merge"), merge)?;
            builder.string(Some("method"), method)?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::Hover => {
            let controller = plan.string(action.strings[0].expect("hover controller span"));
            let domain = plan.string(action.strings[3].expect("hover domain span"));
            let object = action.strings[2].map(|span| plan.string(span));
            let surface = action.strings[1].map(|span| plan.string(span));
            let granularity = action.strings[4].map(|span| plan.string(span));
            let action_id = "interactionHover";
            let bytes = match (surface, object, granularity) {
                (Some(surface), Some(object), Some(granularity)) if action.numbers[0] == 0.0 => {
                    ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "channel", "pointer", "targets", "granularity", granularity, "id", surface, WORLD_ITEM_PATH_DELIMITER, object])?
                }
                (Some(_), Some(object), Some(granularity)) => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "channel", "pointer", "targets", "granularity", granularity, "id", object])?,
                _ => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "channel", "pointer", "targets"])?,
            };
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain)?;
            builder.string(Some("channel"), "pointer")?;
            builder.begin_array(Some("targets"))?;
            if let (Some(surface), Some(object), Some(granularity)) = (surface, object, granularity) {
                builder.begin_object(None)?;
                builder.string(Some("granularity"), granularity)?;
                if action.numbers[0] != 0.0 {
                    builder.string(Some("id"), object)?;
                } else {
                    builder.string_joined(Some("id"), &[surface, WORLD_ITEM_PATH_DELIMITER, object])?;
                }
                builder.end_container()?;
            }
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish_with(|| {
                state.local_hover_id = object.map(str::to_owned);
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::VortexHover => {
            let controller = plan.string(action.strings[0].expect("vortex hover controller span"));
            let surface = plan.string(action.strings[1].expect("vortex hover surface span"));
            let hit = action.strings[2].map(|span| plan.string(span));
            let action_id = "worldVortexHover";
            let bytes = match hit {
                Some(hit) => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId", hit])?,
                None => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId"])?,
            };
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            match hit {
                Some(hit) => builder.string(Some("fullId"), hit)?,
                None => builder.null(Some("fullId"))?,
            }
            builder.end_container()?;
            reservation.publish_with(|| {
                state.hovered_vortex_id = hit.map(str::to_owned);
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::VortexSelect => {
            let controller = plan.string(action.strings[0].expect("vortex selection controller span"));
            let surface = plan.string(action.strings[1].expect("vortex selection surface span"));
            let hit = plan.string(action.strings[2].expect("vortex selection id span"));
            let merge = plan.string(action.strings[3].expect("vortex selection merge span"));
            let action_id = "worldVortexSelect";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId", hit, "merge", merge])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("fullId"), hit)?;
            builder.string(Some("merge"), merge)?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::ComponentHover => {
            let controller = plan.string(action.strings[0].expect("component hover controller span"));
            let object = action.strings[2].map(|span| plan.string(span));
            let mode = plan.string(action.strings[3].expect("component hover mode span"));
            let action_id = "setHover";
            let bytes = match object {
                Some(object) => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "objectId", object, "mode", mode, "id"])?,
                None => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id])?,
            };
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            if let Some(object) = object {
                let builder = reservation.builder();
                builder.begin_object(None)?;
                builder.string(Some("objectId"), object)?;
                builder.string(Some("mode"), mode)?;
                builder.number(Some("id"), action.numbers[0])?;
                builder.end_container()?;
            }
            reservation.publish_with(|| {
                state.hovered_component_id = object.map(|_| (action.numbers[0] as u32).to_string());
                state.hovered_component_object_id = object.map(str::to_owned);
                state.hovered_component_mode = object.map(|_| mode.to_owned());
                if object.is_none() {
                    state.local_hover_id = None;
                }
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::ComponentSelect => {
            let controller = plan.string(action.strings[0].expect("component selection controller span"));
            let surface = plan.string(action.strings[1].expect("component selection surface span"));
            let mode = plan.string(action.strings[3].expect("component selection mode span"));
            let merge = plan.string(action.strings[4].expect("component selection merge span"));
            let action_id = "worldPick";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "granularity", mode, "id", "merge", merge])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("granularity"), mode)?;
            if action.numbers[1] != 0.0 {
                builder.number(Some("id"), action.numbers[0])?;
            } else {
                builder.null(Some("id"))?;
            }
            builder.string(Some("merge"), merge)?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::ContextMenu => {
            let controller = plan.string(action.strings[0].expect("context menu controller span"));
            let surface = plan.string(action.strings[1].expect("context menu surface span"));
            let id = plan.string(action.strings[2].expect("context menu id span"));
            let kind = plan.string(action.strings[3].expect("context menu kind span"));
            let action_id = "worldContextMenuAt";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "kind", kind, "id", id, "x", "y"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("kind"), kind)?;
            builder.string(Some("id"), id)?;
            builder.number(Some("x"), action.numbers[0])?;
            builder.number(Some("y"), action.numbers[1])?;
            builder.end_container()?;
            reservation.publish()?;
        }
        _ => return Ok(WorldInteractionStep::Fault),
    }
    plan.actions[usize::from(plan.cursor)] = None;
    plan.cursor += 1;
    plan.revision = state.interaction_revision;
    context.consume_fuel(1);
    Ok(WorldInteractionStep::Pending)
}
//#endregion 🧵️WorldInteractionTransaction

//#region Pool
struct RefCountPool<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    counts: HashMap<K, u32>,
}

impl<K> RefCountPool<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    fn new() -> Self {
        Self { counts: HashMap::new() }
    }

    fn acquire(&mut self, key: K) {
        *self.counts.entry(key).or_insert(0) += 1;
    }

    fn release(&mut self, key: K) -> bool {
        match self.counts.get_mut(&key) {
            Some(count) if *count > 1 => {
                *count -= 1;
                false
            }
            Some(_) => {
                self.counts.remove(&key);
                true
            }
            None => false,
        }
    }

    fn contains(&self, key: &K) -> bool {
        self.counts.contains_key(key)
    }

    fn keys(&self) -> impl Iterator<Item = &K> {
        self.counts.keys()
    }
}
//#endregion Pool

//#region Chunking
fn chunk_key_indices(position: [f64; 3], chunk_size: f64) -> (i64, i64, i64) {
    ((position[0] / chunk_size).floor() as i64, (position[1] / chunk_size).floor() as i64, (position[2] / chunk_size).floor() as i64)
}

fn chunk_center(key: (i64, i64, i64), chunk_size: f64) -> Vec3 {
    let size = chunk_size as f32;
    Vec3::new((key.0 as f32 + 0.5) * size, (key.1 as f32 + 0.5) * size, (key.2 as f32 + 0.5) * size)
}

fn chunk_bounds_radius(chunk_size: f64) -> f64 {
    chunk_size * 0.866
}

fn chunk_distance_visible(cam_pos: Vec3, chunk_center: Vec3, chunk_size: f64, max_dist: f64, was_visible: bool) -> bool {
    let bounds_r = chunk_bounds_radius(chunk_size);
    let dist = cam_pos.sub(chunk_center).length() as f64;
    let enter_dist = max_dist + bounds_r;
    let exit_dist = enter_dist + chunk_size * 0.5;
    if dist <= enter_dist {
        return true;
    }
    if was_visible && dist <= exit_dist {
        return true;
    }
    false
}

fn update_visible_chunks(state: &mut World3dState, cam_pos: Vec3) {
    let Some(chunking) = state.chunking.clone() else {
        return;
    };
    let chunk_size = chunking.chunk_size;
    let max_distance = chunking.max_distance;
    let mut chunk_keys = HashSet::new();
    for position in state.instance_positions.values() {
        chunk_keys.insert(chunk_key_indices(*position, chunk_size));
    }
    let previous = state.visible_chunks.clone();
    let mut next_visible = HashSet::new();
    for key in chunk_keys.iter().chain(previous.iter()) {
        let center = chunk_center(*key, chunk_size);
        let was = previous.contains(key);
        if chunk_distance_visible(cam_pos, center, chunk_size, max_distance, was) {
            next_visible.insert(*key);
        }
    }
    state.visible_chunks = next_visible;
}

fn instance_chunk_visible(state: &World3dState, position: [f64; 3]) -> bool {
    let Some(chunking) = &state.chunking else {
        return true;
    };
    let key = chunk_key_indices(position, chunking.chunk_size);
    state.visible_chunks.contains(&key)
}
//#endregion Chunking

//#region LodGrid
const WORLD_LOD_EPSILON: f64 = 0.01;
const WORLD_GRID_SIZE: f32 = 12_000.0;

fn default_lod_record() -> WorldLodRecord {
    WorldLodRecord { automatic: true, manual: default_manual_lod(), distance_reference: default_distance_reference(), depth_variable: false, grid_factor: default_grid_factor(), show_grid: true, grid_datum: Some([0.0, 0.0, 0.0]) }
}

fn scene_lod(state: &World3dState) -> f64 {
    let camera = state.orbit.to_camera();
    let distance = camera.position.sub(camera.target).length() as f64;
    let auto_lod = lod_from_camera_distance(distance, state.lod.distance_reference);
    if state.lod.automatic || state.lod.depth_variable {
        auto_lod
    } else {
        state.lod.manual
    }
}

fn resolve_physical_mesh_id(state: &World3dState, logical_id: &str, desired_lod: f64) -> String {
    if let Some(lods) = state.mesh_lod_catalog.get(logical_id) {
        let entries: Vec<(f64, &str)> = lods.iter().map(|entry| (entry.lod, entry.url.as_str())).collect();
        let fallback = state.mesh_url_fallback.get(logical_id).map(String::as_str);
        if let Some(url) = pick_closest_mesh_url(&entries, desired_lod, fallback) {
            return mesh_id_from_url(url);
        }
    }
    if let Some(url) = state.mesh_url_fallback.get(logical_id) {
        return mesh_id_from_url(url);
    }
    logical_id.to_string()
}

fn append_lod_grid_lines(line_vertices: &mut Vec<LineVertex3d>, lod: f64, grid_factor: f64, anchor: Vec3, base_color: [f32; 4]) {
    for (step_world, opacity) in lod_progressive_grid_layers(lod, grid_factor) {
        let step = step_world as f32;
        let divs = ((WORLD_GRID_SIZE / step).round() as i32).clamp(2, 512);
        let half = WORLD_GRID_SIZE * 0.5;
        let step_size = WORLD_GRID_SIZE / divs as f32;
        let color = [base_color[0], base_color[1], base_color[2], base_color[3] * opacity];
        let z = anchor.z + 0.002;
        for i in 0..=divs {
            let offset = -half + i as f32 * step_size;
            line_vertices.push(LineVertex3d { position: [anchor.x - half, anchor.y + offset, z], color });
            line_vertices.push(LineVertex3d { position: [anchor.x + half, anchor.y + offset, z], color });
            line_vertices.push(LineVertex3d { position: [anchor.x + offset, anchor.y - half, z], color });
            line_vertices.push(LineVertex3d { position: [anchor.x + offset, anchor.y + half, z], color });
        }
    }
}

fn sync_mesh_pool(state: &mut World3dState, needed_mesh_keys: &HashSet<String>, gpu: &mut World3dBuildContext) {
    const PINNED: &[&str] = &["vortex-marker", "cylinder", "cone", "reference-plane", "vertex-marker"];
    for key in needed_mesh_keys {
        if !state.mesh_pool.contains(key) {
            state.mesh_pool.acquire(key.clone());
        }
    }
    let stale: Vec<String> = state.mesh_pool.keys().filter(|key| !needed_mesh_keys.contains(*key) && !PINNED.contains(&key.as_str())).cloned().collect();
    for key in stale {
        if state.mesh_pool.release(key.clone()) {
            if !retire_world_mesh(state, &key) || !retire_world_pixels(state, &key, true) {
                return;
            }
            state.mesh_source_urls.remove(&key);
            state.pending_glb_urls.remove(&key);
            gpu.evict_mesh(&key);
        }
    }
}

fn queue_lod_mesh_fetch(state: &mut World3dState, logical_id: &str, scene_lod: f64) {
    let url = {
        let entries: Vec<(f64, &str)> = state.mesh_lod_catalog.get(logical_id).map(|lods| lods.iter().map(|entry| (entry.lod, entry.url.as_str())).collect()).unwrap_or_default();
        let fallback = state.mesh_url_fallback.get(logical_id).map(String::as_str);
        pick_closest_mesh_url(&entries, scene_lod, fallback).or(fallback).map(str::to_owned)
    };
    if let Some(url) = url {
        if reserve_world3d_asset_request(state, WorldAssetRequestKind::Glb, &url).is_err() {
            mark_world_dynamic_fault(state, WorldDynamicFault::RegistryCapacity);
        }
    }
}

//#endregion LodGrid

//#region Environment
/// ☀️🎨️ Resolves the renderer's scene-pass light direction from `environment.sun` when the sun is
/// explicitly enabled (horizontal coordinate system, see https://en.wikipedia.org/wiki/Horizontal_coordinate_system,
/// matching `sunPositionFromAzimuthElevation` in `ui/js/react/index.tsx`), else keeps the default.
fn environment_light_dir(environment: &WorldEnvironmentRecord) -> [f32; 3] {
    const DEFAULT_LIGHT_DIR: [f32; 3] = [0.4, 0.6, 0.8];
    let Some(sun) = environment.sun.as_ref() else {
        return DEFAULT_LIGHT_DIR;
    };
    if sun.enabled != Some(true) {
        return DEFAULT_LIGHT_DIR;
    }
    let azimuth = sun.azimuth.unwrap_or(45.0).to_radians();
    let elevation = sun.elevation.unwrap_or(35.0).to_radians();
    let direction = Vec3::new((elevation.cos() * azimuth.cos()) as f32, (elevation.cos() * azimuth.sin()) as f32, elevation.sin() as f32);
    if direction.length() < 1e-6 {
        DEFAULT_LIGHT_DIR
    } else {
        direction.normalize().to_array()
    }
}

/// 🖼️ Resolves the canvas clear color from `environment.background`, falling back to the ambient
/// theme clear color when absent or `"transparent"` (mirrors `isTransparentWorldBackground`).
fn environment_clear_color(environment: &WorldEnvironmentRecord, theme_clear: Rgba) -> Rgba {
    let Some(background) = environment.background.as_deref() else {
        return theme_clear;
    };
    if background.eq_ignore_ascii_case("transparent") {
        return theme_clear;
    }
    let [r, g, b, a] = parse_color(background);
    Rgba::new(r, g, b, a)
}
//#endregion Environment

//#region Terrain
/// 🧮️ Elevation-band count for terrain tile shading — `Instance3d`/`World3dVertex` carry no
/// per-vertex color channel (wiring gap, see report), so the continuous hypsometric ramp from the
/// React reference is approximated by bucketing each tile's triangles into flat-colored bands by
/// their (per-tile-normalized) average elevation, reusing the same per-color-bucket technique as
/// `append_component_face_translucent_overlays`.
const TERRAIN_COLOR_BANDS: usize = 10;

#[derive(Deserialize)]
struct TerrainVisibleTileRow {
    z: u32,
    x: u32,
    y: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TerrainTileMeshPayload {
    positions: Vec<f32>,
    normals: Vec<f32>,
    indices: Vec<u32>,
    uvs: Vec<f32>,
}

fn terrain_tile_url(template: &str, z: u32, x: u32, y: u32) -> String {
    template.replace("{z}", &z.to_string()).replace("{x}", &x.to_string()).replace("{y}", &y.to_string())
}

fn terrain_band_mesh_key(surface_id: &str, z: u32, x: u32, y: u32, band: usize) -> String {
    format!("terrain:{surface_id}:{z}:{x}:{y}:{band}")
}

/// 🎨️ Vertical hypsometric ramp — same stops as `getHypsometricTexture` in `world-terrain-layer.tsx`
/// (green low ground -> tan -> grey -> white peaks), sampled at a band's center elevation ratio.
fn hypsometric_color(t: f32) -> [f32; 4] {
    let stops: [(f32, [f32; 3]); 4] =
        [(0.0, [0x4b as f32 / 255.0, 0x6b as f32 / 255.0, 0x3a as f32 / 255.0]), (0.5, [0xa6 as f32 / 255.0, 0x8a as f32 / 255.0, 0x5b as f32 / 255.0]), (0.85, [0x8f as f32 / 255.0, 0x8f as f32 / 255.0, 0x8f as f32 / 255.0]), (1.0, [1.0, 1.0, 1.0])];
    let t = t.clamp(0.0, 1.0);
    for window in stops.windows(2) {
        let (t0, c0) = window[0];
        let (t1, c1) = window[1];
        if t <= t1 {
            let f = if (t1 - t0).abs() < 1e-6 { 0.0 } else { (t - t0) / (t1 - t0) };
            return [c0[0] + (c1[0] - c0[0]) * f, c0[1] + (c1[1] - c0[1]) * f, c0[2] + (c1[2] - c0[2]) * f, 1.0];
        }
    }
    [1.0, 1.0, 1.0, 1.0]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldTerrainMeshPhase {
    Count,
    Begin,
    Allocate,
    Positions,
    Normals,
    Indices,
    Seal,
    Publish,
    NextBand,
    RetireSource,
}

struct WorldTerrainMeshCursor {
    surface_id: String,
    z: u32,
    x: u32,
    y: u32,
    payload: TerrainTileMeshPayload,
    generation: u64,
    source_revision: u64,
    terrain_revision: u64,
    band: u8,
    phase: WorldTerrainMeshPhase,
    triangle: u32,
    vertex: u8,
    matched: u32,
    item: u32,
    owner: WorldPlaceholderOwner,
    close_started: bool,
    retire_source: u8,
    faulted: bool,
}

enum WorldTerrainMeshStep {
    Pending,
    Ready(String, Mesh3dLease),
    Complete((u32, u32, u32)),
    Fault,
}

impl WorldTerrainMeshCursor {
    fn new(surface_id: &str, z: u32, x: u32, y: u32, payload: TerrainTileMeshPayload, generation: u64, source_revision: u64, terrain_revision: u64) -> Result<Self, WorldDynamicFault> {
        if surface_id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY || !payload.positions.len().is_multiple_of(3) || !payload.normals.len().is_multiple_of(3) || !payload.indices.len().is_multiple_of(3) || !payload.uvs.len().is_multiple_of(2) {
            return Err(if surface_id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY { WorldDynamicFault::IdCapacity } else { WorldDynamicFault::ByteCapacity });
        }
        u32::try_from(payload.indices.len() / 3).map_err(|_| WorldDynamicFault::InstanceCapacity)?;
        Ok(Self {
            surface_id: surface_id.to_owned(),
            z,
            x,
            y,
            payload,
            generation,
            source_revision,
            terrain_revision,
            band: 0,
            phase: WorldTerrainMeshPhase::Count,
            triangle: 0,
            vertex: 0,
            matched: 0,
            item: 0,
            owner: WorldPlaceholderOwner::Empty,
            close_started: false,
            retire_source: 0,
            faulted: false,
        })
    }

    fn triangle_count(&self) -> u32 {
        (self.payload.indices.len() / 3) as u32
    }

    fn triangle_indices(&self, triangle: u32) -> Result<[u32; 3], WorldDynamicFault> {
        let start = usize::try_from(triangle).ok().and_then(|triangle| triangle.checked_mul(3)).ok_or(WorldDynamicFault::InstanceCapacity)?;
        let values = self.payload.indices.get(start..start + 3).ok_or(WorldDynamicFault::StaleToken)?;
        let result = [values[0], values[1], values[2]];
        if result.iter().any(|index| usize::try_from(*index).ok().and_then(|index| index.checked_mul(3)).is_none_or(|offset| self.payload.positions.get(offset..offset + 3).is_none())) {
            return Err(WorldDynamicFault::StaleToken);
        }
        Ok(result)
    }

    fn triangle_band(&self, triangle: u32) -> Result<u8, WorldDynamicFault> {
        let indices = self.triangle_indices(triangle)?;
        let elevation = indices.map(|index| self.payload.uvs.get(index as usize * 2 + 1).copied().unwrap_or(0.0));
        if elevation.iter().any(|value| !value.is_finite()) {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        Ok((((elevation[0] + elevation[1] + elevation[2]) / 3.0 * TERRAIN_COLOR_BANDS as f32) as usize).min(TERRAIN_COLOR_BANDS - 1) as u8)
    }

    fn value(&self, triangle: u32, vertex: u8, normal: bool) -> Result<[f32; 3], WorldDynamicFault> {
        let index = self.triangle_indices(triangle)?[usize::from(vertex)] as usize;
        let source = if normal { &self.payload.normals } else { &self.payload.positions };
        let offset = index.checked_mul(3).ok_or(WorldDynamicFault::ByteCapacity)?;
        let value = match source.get(offset..offset + 3) {
            Some(value) => [value[0], value[1], value[2]],
            None if normal => [0.0, 0.0, 1.0],
            None => return Err(WorldDynamicFault::StaleToken),
        };
        if value.iter().all(|value| value.is_finite()) {
            Ok(value)
        } else {
            Err(WorldDynamicFault::ByteCapacity)
        }
    }

    fn token(&self) -> Result<Mesh3dWriteToken, WorldDynamicFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(WorldDynamicFault::StaleToken),
        }
    }

    fn step(&mut self, source_revision: u64, terrain_revision: u64) -> WorldTerrainMeshStep {
        if source_revision != self.source_revision || terrain_revision != self.terrain_revision {
            self.faulted = true;
        }
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() { WorldTerrainMeshStep::Fault } else { WorldTerrainMeshStep::Pending };
        }
        match self.step_live() {
            Ok(step) => step,
            Err(_) => {
                self.faulted = true;
                let _ = self.close_step();
                WorldTerrainMeshStep::Pending
            }
        }
    }

    fn step_live(&mut self) -> Result<WorldTerrainMeshStep, WorldDynamicFault> {
        match self.phase {
            WorldTerrainMeshPhase::Count => {
                if self.triangle == self.triangle_count() {
                    self.triangle = 0;
                    self.phase = if self.matched == 0 { WorldTerrainMeshPhase::NextBand } else { WorldTerrainMeshPhase::Begin };
                    return Ok(WorldTerrainMeshStep::Pending);
                }
                if self.triangle_band(self.triangle)? == self.band {
                    self.matched = self.matched.checked_add(1).ok_or(WorldDynamicFault::InstanceCapacity)?;
                }
                self.triangle += 1;
            }
            WorldTerrainMeshPhase::Begin => {
                let items = self.matched.checked_mul(3).ok_or(WorldDynamicFault::InstanceCapacity)?;
                let generation = self.generation.checked_add(u64::from(self.band)).ok_or(WorldDynamicFault::StaleToken)?;
                self.owner = WorldPlaceholderOwner::Writing(mesh3d_begin(generation, self.source_revision, Mesh3dSchema::triangle_mesh(items, items)).map_err(|_| WorldDynamicFault::ByteCapacity)?);
                self.phase = WorldTerrainMeshPhase::Allocate;
            }
            WorldTerrainMeshPhase::Allocate => {
                if mesh3d_allocate_step(self.token()?).map_err(|_| WorldDynamicFault::Closing)? {
                    self.phase = WorldTerrainMeshPhase::Positions;
                }
            }
            WorldTerrainMeshPhase::Positions | WorldTerrainMeshPhase::Normals => {
                if self.triangle == self.triangle_count() {
                    self.triangle = 0;
                    self.vertex = 0;
                    self.phase = if self.phase == WorldTerrainMeshPhase::Positions { WorldTerrainMeshPhase::Normals } else { WorldTerrainMeshPhase::Indices };
                    return Ok(WorldTerrainMeshStep::Pending);
                }
                if self.triangle_band(self.triangle)? != self.band {
                    self.triangle += 1;
                    return Ok(WorldTerrainMeshStep::Pending);
                }
                let normal = self.phase == WorldTerrainMeshPhase::Normals;
                mesh3d_write_vec3(self.token()?, if normal { Mesh3dField::Normals } else { Mesh3dField::Positions }, self.value(self.triangle, self.vertex, normal)?).map_err(|_| WorldDynamicFault::Closing)?;
                self.vertex += 1;
                if self.vertex == 3 {
                    self.vertex = 0;
                    self.triangle += 1;
                }
            }
            WorldTerrainMeshPhase::Indices => {
                let items = self.matched * 3;
                mesh3d_write_u32(self.token()?, Mesh3dField::Indices, self.item).map_err(|_| WorldDynamicFault::Closing)?;
                self.item += 1;
                if self.item == items {
                    self.item = 0;
                    self.phase = WorldTerrainMeshPhase::Seal;
                }
            }
            WorldTerrainMeshPhase::Seal => {
                let lease = mesh3d_seal(self.token()?).map_err(|_| WorldDynamicFault::Closing)?;
                self.owner = WorldPlaceholderOwner::Ready(lease);
                self.phase = WorldTerrainMeshPhase::Publish;
            }
            WorldTerrainMeshPhase::Publish => {
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(WorldDynamicFault::StaleToken) };
                self.owner = WorldPlaceholderOwner::Empty;
                self.phase = WorldTerrainMeshPhase::NextBand;
                return Ok(WorldTerrainMeshStep::Ready(terrain_band_mesh_key(&self.surface_id, self.z, self.x, self.y, usize::from(self.band)), lease));
            }
            WorldTerrainMeshPhase::NextBand => {
                self.band += 1;
                self.matched = 0;
                self.triangle = 0;
                self.vertex = 0;
                self.item = 0;
                self.phase = if usize::from(self.band) == TERRAIN_COLOR_BANDS { WorldTerrainMeshPhase::RetireSource } else { WorldTerrainMeshPhase::Count };
            }
            WorldTerrainMeshPhase::RetireSource => {
                if self.close_step() && self.terminal_is_empty() {
                    return Ok(WorldTerrainMeshStep::Complete((self.z, self.x, self.y)));
                }
            }
        }
        Ok(WorldTerrainMeshStep::Pending)
    }

    fn close_step(&mut self) -> bool {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match mesh3d_abort_step(token) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => {
                        self.owner = WorldPlaceholderOwner::Empty;
                        self.close_started = false;
                    }
                    Ok(false) | Err(_) => return false,
                }
                return false;
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match mesh3d_close_step(lease) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => {
                        self.owner = WorldPlaceholderOwner::Empty;
                        self.close_started = false;
                    }
                    Ok(false) | Err(_) => return false,
                }
                return false;
            }
            WorldPlaceholderOwner::Empty => {}
        }
        match self.retire_source {
            0 => self.payload.positions = Vec::new(),
            1 => self.payload.normals = Vec::new(),
            2 => self.payload.indices = Vec::new(),
            3 => self.payload.uvs = Vec::new(),
            _ => {
                if self.surface_id.pop().is_some() {
                    return false;
                }
                return true;
            }
        }
        self.retire_source += 1;
        false
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.owner, WorldPlaceholderOwner::Empty) && self.payload.positions.capacity() == 0 && self.payload.normals.capacity() == 0 && self.payload.indices.capacity() == 0 && self.payload.uvs.capacity() == 0 && self.surface_id.is_empty()
    }
}

#[cfg(not(test))]
impl Drop for WorldTerrainMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "terrain mesh cursor reached Drop before its exact page/source witness");
    }
}

fn step_world_terrain_mesh(state: &mut World3dState) {
    let Some(mut cursor) = state.terrain_build.take() else { return };
    if !state.terrain_visible_tiles.contains(&(cursor.z, cursor.x, cursor.y)) || state.dynamic_blocked_mesh.is_some() || state.dynamic_mesh_close.is_some() {
        cursor.faulted = true;
    }
    match cursor.step(state.terrain_revision, state.terrain_revision) {
        WorldTerrainMeshStep::Pending => state.terrain_build = Some(cursor),
        WorldTerrainMeshStep::Ready(key, lease) => match publish_world3d_mesh_lease(state, key, lease) {
            Ok(()) => state.terrain_build = Some(cursor),
            Err(rejected) => {
                retain_rejected_world_mesh(state, rejected);
                cursor.faulted = true;
                state.terrain_build = Some(cursor);
            }
        },
        WorldTerrainMeshStep::Complete(tile) => {
            state.terrain_built_tiles.insert(tile);
        }
        WorldTerrainMeshStep::Fault => mark_world_dynamic_fault(state, WorldDynamicFault::Closing),
    }
}

#[cfg(test)]
struct LegacyMeshOracleData {
    positions: Vec<f32>,
    normals: Vec<f32>,
    indices: Vec<u32>,
    face_ids: Vec<u32>,
    vertex_ids: Vec<u32>,
    edge_positions: Vec<f32>,
    edge_ids: Vec<u32>,
    uvs: Vec<f32>,
    colors: Vec<f32>,
}

#[cfg(test)]
fn build_terrain_band_mesh(mesh: &TerrainTileMeshPayload, band: usize, band_count: usize) -> Option<LegacyMeshOracleData> {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    for triangle in mesh.indices.as_chunks::<3>().0 {
        let elevations = triangle.map(|index| mesh.uvs.get(index as usize * 2 + 1).copied().unwrap_or(0.0));
        let selected = (((elevations[0] + elevations[1] + elevations[2]) / 3.0 * band_count as f32) as usize).min(band_count - 1);
        if selected != band {
            continue;
        }
        let base = (positions.len() / 3) as u32;
        for index in triangle {
            let offset = *index as usize * 3;
            let position = mesh.positions.get(offset..offset + 3)?;
            let normal = mesh.normals.get(offset..offset + 3).unwrap_or(&[0.0, 0.0, 1.0]);
            positions.extend_from_slice(position);
            normals.extend_from_slice(normal);
        }
        indices.extend_from_slice(&[base, base + 1, base + 2]);
    }
    (!positions.is_empty()).then_some(LegacyMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() })
}

/// 🔄️ GPU-free half of `apply_terrain_style_if_changed`: applies `state.terrain_style` to the tile
/// session and purges cached tile meshes from CPU-side maps whenever the tile source, project
/// origin, or exaggeration changes (their old positions/heights are baked into cached geometry and
/// would otherwise render stale). Returns the mesh keys the caller must also evict from the GPU.
fn apply_terrain_style_if_changed_state(state: &mut World3dState) -> Vec<String> {
    let signature = state.terrain_style.as_ref().map(|style| (style.tile_url_template.clone(), style.project_origin_lon, style.project_origin_lat, style.exaggeration));
    if signature == state.terrain_applied_signature {
        return Vec::new();
    }
    state.terrain_revision = state.terrain_revision.wrapping_add(1).max(1);
    if let Some(cursor) = state.terrain_build.as_mut() {
        cursor.faulted = true;
    }
    let prefix = format!("terrain:{}:", state.surface_id);
    let stale_keys: Vec<String> = state.meshes.keys().filter(|key| key.starts_with(&prefix)).cloned().collect();
    for key in &stale_keys {
        if !retire_world_mesh(state, key) {
            return Vec::new();
        }
    }
    let visible = std::mem::take(&mut state.terrain_visible_tiles);
    for (z, x, y) in visible {
        state.terrain_session.evict_terrain_tile(z, x, y);
    }
    state.terrain_built_tiles.clear();
    state.pending_terrain_tile_urls.clear();
    if let Some(style) = &state.terrain_style {
        state.terrain_session.set_project_origin(style.project_origin_lon, style.project_origin_lat);
        state.terrain_session.set_exaggeration(style.exaggeration);
    }
    state.terrain_applied_signature = signature;
    stale_keys
}

fn apply_terrain_style_if_changed(state: &mut World3dState, gpu: &mut World3dBuildContext) {
    for key in apply_terrain_style_if_changed_state(state) {
        gpu.evict_mesh(&key);
    }
}

/// 🏔️ One terrain band's resolved GPU draw inputs: mesh key/version (already present in
/// `state.meshes`/`state.mesh_versions`) plus the flat hypsometric color for that band.
struct TerrainBandDraw {
    mesh_key: String,
    mesh_version: u64,
    color: [f32; 4],
}

fn terrain_family_visible(state: &World3dState, tile: (u32, u32, u32)) -> bool {
    state.terrain_built_tiles.contains(&tile)
}

/// 🏔️ GPU-free half of `sync_terrain`: asks `TerrainSessionCore` which DEM tiles are visible for
/// the current camera, evicts (CPU-side) tiles that scrolled out of view, queues byte-fetches for
/// tiles not yet uploaded (see `fetch_pending_terrain_tiles`), and builds/caches banded meshes for
/// tiles whose elevation data is already available. Returns the bands to draw this frame plus the
/// mesh keys the caller must evict from the GPU.
fn sync_terrain_state(state: &mut World3dState, camera: &Camera3d) -> (Vec<TerrainBandDraw>, Vec<String>) {
    let Some(style) = state.terrain_style.clone() else {
        return (Vec::new(), Vec::new());
    };
    let camera_json = json!({
        "position": [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        "target": [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
    })
    .to_string();
    let visible_json = state.terrain_session.visible_terrain_tiles_json(&camera_json);
    let visible_rows: Vec<TerrainVisibleTileRow> = serde_json::from_str(&visible_json).unwrap_or_default();
    let visible_set: HashSet<(u32, u32, u32)> = visible_rows.iter().map(|row| (row.z, row.x, row.y)).collect();

    let stale: Vec<(u32, u32, u32)> = state.terrain_visible_tiles.iter().copied().filter(|key| !visible_set.contains(key)).collect();
    let mut evicted_mesh_keys = Vec::new();
    for (z, x, y) in stale {
        state.terrain_session.evict_terrain_tile(z, x, y);
        state.terrain_built_tiles.remove(&(z, x, y));
        for band in 0..TERRAIN_COLOR_BANDS {
            let mesh_key = terrain_band_mesh_key(&state.surface_id, z, x, y, band);
            if !retire_world_mesh(state, &mesh_key) {
                return (Vec::new(), evicted_mesh_keys);
            }
            evicted_mesh_keys.push(mesh_key);
        }
    }
    state.pending_terrain_tile_urls.retain(|_, tile| visible_set.contains(tile));
    state.terrain_visible_tiles = visible_set.clone();
    step_world_terrain_mesh(state);

    let mut band_draws = Vec::new();
    for (z, x, y) in visible_set {
        if !terrain_family_visible(state, (z, x, y)) {
            let mesh_json = state.terrain_session.terrain_tile_mesh_json(z, x, y);
            if mesh_json == "null" {
                state.pending_terrain_tile_urls.insert(terrain_tile_url(&style.tile_url_template, z, x, y), (z, x, y));
            } else if state.terrain_build.is_none() && state.dynamic_mesh_close.is_none() && state.dynamic_blocked_mesh.is_none() && state.snapshot_fault.is_none() {
                if let Ok(mesh_payload) = serde_json::from_str::<TerrainTileMeshPayload>(&mesh_json) {
                    let next = state.placeholder_generation.checked_add(TERRAIN_COLOR_BANDS as u64);
                    if let Some(next) = next {
                        let generation = state.placeholder_generation + 1;
                        match WorldTerrainMeshCursor::new(&state.surface_id, z, x, y, mesh_payload, generation, state.terrain_revision, state.terrain_revision) {
                            Ok(cursor) => {
                                state.placeholder_generation = next;
                                state.terrain_build = Some(cursor);
                            }
                            Err(fault) => mark_world_dynamic_fault(state, fault),
                        }
                    } else {
                        mark_world_dynamic_fault(state, WorldDynamicFault::StaleToken);
                    }
                }
            }
            continue;
        }
        for band in 0..TERRAIN_COLOR_BANDS {
            let mesh_key = terrain_band_mesh_key(&state.surface_id, z, x, y, band);
            if !state.meshes.contains_key(&mesh_key) {
                continue;
            }
            let mesh_version = *state.mesh_versions.get(&mesh_key).unwrap_or(&0);
            let band_center = (band as f32 + 0.5) / TERRAIN_COLOR_BANDS as f32;
            band_draws.push(TerrainBandDraw { mesh_key, mesh_version, color: hypsometric_color(band_center) });
        }
    }
    (band_draws, evicted_mesh_keys)
}

/// 🏔️ Per-frame terrain sync entry point used by `render_world_3d` — see `sync_terrain_state` for
/// the (unit-testable) tile-visibility/meshing/fetch-queueing logic this wraps with GPU upload and
/// eviction calls.
fn sync_terrain(state: &mut World3dState, gpu: &mut World3dBuildContext, camera: &Camera3d) -> Vec<SceneDraw3d> {
    let (band_draws, evicted_mesh_keys) = sync_terrain_state(state, camera);
    for key in evicted_mesh_keys {
        gpu.evict_mesh(&key);
    }
    band_draws
        .into_iter()
        .filter_map(|band| {
            let mesh = state.meshes.get(&band.mesh_key)?;
            gpu.ensure_mesh(&band.mesh_key, band.mesh_version, *mesh);
            Some(SceneDraw3d { mesh_key: band.mesh_key.clone(), mesh_version: band.mesh_version, instances: vec![Instance3d { id: format!("terrain-{}", band.mesh_key), model: Mat4::identity(), color: band.color, selected: false, hovered: false }] })
        })
        .collect()
}
//#endregion Terrain

//#region Gumball
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GumballHandle {
    MoveX,
    MoveY,
    MoveZ,
    MoveXY,
    MoveYZ,
    MoveXZ,
    RotateX,
    RotateY,
    RotateZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}

impl GumballHandle {
    fn axis_dir(self) -> Option<Vec3> {
        match self {
            Self::MoveX | Self::RotateX | Self::ScaleX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveY | Self::RotateY | Self::ScaleY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::MoveZ | Self::RotateZ | Self::ScaleZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    fn plane_normal(self) -> Option<Vec3> {
        match self {
            Self::MoveXY => Some(Vec3::new(0.0, 0.0, 1.0)),
            Self::MoveYZ => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveXZ => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::RotateY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    fn is_translate(self) -> bool {
        matches!(self, Self::MoveX | Self::MoveY | Self::MoveZ | Self::MoveXY | Self::MoveYZ | Self::MoveXZ)
    }

    fn is_rotate(self) -> bool {
        matches!(self, Self::RotateX | Self::RotateY | Self::RotateZ)
    }

    fn is_scale(self) -> bool {
        matches!(self, Self::ScaleX | Self::ScaleY | Self::ScaleZ)
    }
}

//#region MeshHelpers
//#region PlaceholderMeshAuthority
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldPlaceholderKind {
    Box,
    Plane,
    Cylinder,
    Cone,
    Icosphere,
}

impl WorldPlaceholderKind {
    fn resolve(value: &str) -> Self {
        match value {
            "vortex-marker" => Self::Icosphere,
            "plane" => Self::Plane,
            "cylinder" => Self::Cylinder,
            "cone" => Self::Cone,
            _ => Self::Box,
        }
    }

    fn triangles(self) -> u32 {
        match self {
            Self::Box => 12,
            Self::Plane => 2,
            Self::Cylinder => 64,
            Self::Cone => 32,
            Self::Icosphere => 80,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldPlaceholderMeshPhase {
    Allocate,
    Positions,
    Normals,
    Indices,
    Seal,
    Publish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldPlaceholderOwner {
    Writing(Mesh3dWriteToken),
    Ready(Mesh3dLease),
    Empty,
}

struct WorldPlaceholderMeshCursor {
    key: String,
    kind: WorldPlaceholderKind,
    phase: WorldPlaceholderMeshPhase,
    item: u32,
    owner: WorldPlaceholderOwner,
    close_started: bool,
    faulted: bool,
}

enum WorldPlaceholderMeshStep {
    Pending,
    Ready(String, Mesh3dLease),
    Fault,
}

impl WorldPlaceholderMeshCursor {
    fn new(key: &str, kind: WorldPlaceholderKind, generation: u64, revision: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        if key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity);
        }
        let items = kind.triangles().checked_mul(3).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        let owner = WorldPlaceholderOwner::Writing(mesh3d_begin(generation, revision, Mesh3dSchema::triangle_mesh(items, items))?);
        Ok(Self { key: key.to_owned(), kind, phase: WorldPlaceholderMeshPhase::Allocate, item: 0, owner, close_started: false, faulted: false })
    }

    fn token(&self) -> Result<Mesh3dWriteToken, ui_wgpu::wgpu::Mesh3dFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(ui_wgpu::wgpu::Mesh3dFault::Stale),
        }
    }

    fn step(&mut self) -> WorldPlaceholderMeshStep {
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() { WorldPlaceholderMeshStep::Fault } else { WorldPlaceholderMeshStep::Pending };
        }
        let result = self.step_live();
        if result.is_err() {
            self.faulted = true;
            let _ = self.close_step();
            return WorldPlaceholderMeshStep::Pending;
        }
        result.expect("placeholder result checked above")
    }

    fn step_live(&mut self) -> Result<WorldPlaceholderMeshStep, ui_wgpu::wgpu::Mesh3dFault> {
        let items = self.kind.triangles() * 3;
        match self.phase {
            WorldPlaceholderMeshPhase::Allocate => {
                if mesh3d_allocate_step(self.token()?)? {
                    self.phase = WorldPlaceholderMeshPhase::Positions;
                }
            }
            WorldPlaceholderMeshPhase::Positions => {
                let triangle = placeholder_triangle(self.kind, self.item / 3);
                mesh3d_write_vec3(self.token()?, Mesh3dField::Positions, triangle[(self.item % 3) as usize])?;
                self.item += 1;
                if self.item == items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Normals;
                }
            }
            WorldPlaceholderMeshPhase::Normals => {
                let triangle = placeholder_triangle(self.kind, self.item / 3);
                mesh3d_write_vec3(self.token()?, Mesh3dField::Normals, placeholder_triangle_normal(triangle))?;
                self.item += 1;
                if self.item == items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Indices;
                }
            }
            WorldPlaceholderMeshPhase::Indices => {
                mesh3d_write_u32(self.token()?, Mesh3dField::Indices, self.item)?;
                self.item += 1;
                if self.item == items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Seal;
                }
            }
            WorldPlaceholderMeshPhase::Seal => {
                let lease = mesh3d_seal(self.token()?)?;
                self.owner = WorldPlaceholderOwner::Ready(lease);
                self.phase = WorldPlaceholderMeshPhase::Publish;
            }
            WorldPlaceholderMeshPhase::Publish => {
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(ui_wgpu::wgpu::Mesh3dFault::Stale) };
                self.owner = WorldPlaceholderOwner::Empty;
                return Ok(WorldPlaceholderMeshStep::Ready(std::mem::take(&mut self.key), lease));
            }
        }
        Ok(WorldPlaceholderMeshStep::Pending)
    }

    fn close_step(&mut self) -> bool {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match mesh3d_abort_step(token) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match mesh3d_close_step(lease) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Empty => {
                if self.key.pop().is_some() {
                    return false;
                }
                true
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match self.owner {
            WorldPlaceholderOwner::Writing(_) => false,
            WorldPlaceholderOwner::Ready(lease) => mesh3d_terminal_is_empty(lease),
            WorldPlaceholderOwner::Empty => self.key.is_empty(),
        }
    }
}

#[cfg(not(test))]
impl Drop for WorldPlaceholderMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "placeholder mesh cursor reached Drop before its exact authority witness");
    }
}

fn placeholder_triangle_normal(triangle: [[f32; 3]; 3]) -> [f32; 3] {
    let ab = [triangle[1][0] - triangle[0][0], triangle[1][1] - triangle[0][1], triangle[1][2] - triangle[0][2]];
    let ac = [triangle[2][0] - triangle[0][0], triangle[2][1] - triangle[0][1], triangle[2][2] - triangle[0][2]];
    placeholder_normalize3([ab[1] * ac[2] - ab[2] * ac[1], ab[2] * ac[0] - ab[0] * ac[2], ab[0] * ac[1] - ab[1] * ac[0]])
}

fn placeholder_triangle(kind: WorldPlaceholderKind, triangle: u32) -> [[f32; 3]; 3] {
    match kind {
        WorldPlaceholderKind::Box => {
            let faces = [
                ([-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]),
                ([0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5]),
                ([-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]),
                ([-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]),
                ([0.5, -0.5, 0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5]),
                ([-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5]),
            ];
            let (a, b, c, d) = faces[(triangle / 2) as usize];
            if triangle.is_multiple_of(2) {
                [a, b, c]
            } else {
                [a, c, d]
            }
        }
        WorldPlaceholderKind::Plane => {
            if triangle == 0 {
                [[-0.5, 0.0, -0.5], [0.5, 0.0, -0.5], [0.5, 0.0, 0.5]]
            } else {
                [[-0.5, 0.0, -0.5], [0.5, 0.0, 0.5], [-0.5, 0.0, 0.5]]
            }
        }
        WorldPlaceholderKind::Cylinder => {
            let segment = triangle / 4;
            let a0 = segment as f32 / 16.0 * std::f32::consts::TAU;
            let a1 = (segment + 1) as f32 / 16.0 * std::f32::consts::TAU;
            let p00 = [0.5 * a0.cos(), -0.5, 0.5 * a0.sin()];
            let p01 = [0.5 * a1.cos(), -0.5, 0.5 * a1.sin()];
            let p10 = [0.5 * a0.cos(), 0.5, 0.5 * a0.sin()];
            let p11 = [0.5 * a1.cos(), 0.5, 0.5 * a1.sin()];
            match triangle % 4 {
                0 => [p00, p01, p11],
                1 => [p00, p11, p10],
                2 => [[0.0, -0.5, 0.0], p01, p00],
                _ => [[0.0, 0.5, 0.0], p10, p11],
            }
        }
        WorldPlaceholderKind::Cone => {
            let segment = triangle / 2;
            let a0 = segment as f32 / 16.0 * std::f32::consts::TAU;
            let a1 = (segment + 1) as f32 / 16.0 * std::f32::consts::TAU;
            let p0 = [0.5 * a0.cos(), 0.0, 0.5 * a0.sin()];
            let p1 = [0.5 * a1.cos(), 0.0, 0.5 * a1.sin()];
            if triangle.is_multiple_of(2) {
                [[0.0, 1.0, 0.0], p1, p0]
            } else {
                [[0.0, 0.0, 0.0], p0, p1]
            }
        }
        WorldPlaceholderKind::Icosphere => {
            let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
            let vertices = [[-1.0, t, 0.0], [1.0, t, 0.0], [-1.0, -t, 0.0], [1.0, -t, 0.0], [0.0, -1.0, t], [0.0, 1.0, t], [0.0, -1.0, -t], [0.0, 1.0, -t], [t, 0.0, -1.0], [t, 0.0, 1.0], [-t, 0.0, -1.0], [-t, 0.0, 1.0]];
            let faces =
                [[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
            let face = faces[(triangle / 4) as usize];
            let a = placeholder_normalize3(vertices[face[0]]);
            let b = placeholder_normalize3(vertices[face[1]]);
            let c = placeholder_normalize3(vertices[face[2]]);
            let ab = placeholder_normalize3([(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5]);
            let bc = placeholder_normalize3([(b[0] + c[0]) * 0.5, (b[1] + c[1]) * 0.5, (b[2] + c[2]) * 0.5]);
            let ca = placeholder_normalize3([(c[0] + a[0]) * 0.5, (c[1] + a[1]) * 0.5, (c[2] + a[2]) * 0.5]);
            let triangle = match triangle % 4 {
                0 => [a, ab, ca],
                1 => [b, bc, ab],
                2 => [c, ca, bc],
                _ => [ab, bc, ca],
            };
            triangle.map(|vertex| placeholder_scale3(vertex, 0.12))
        }
    }
}

fn begin_world_placeholder_mesh(state: &mut World3dState, key: &str, kind: WorldPlaceholderKind) {
    if state.meshes.contains_key(key) || state.placeholder_build.is_some() || state.dynamic_retirement.is_some() {
        return;
    }
    state.placeholder_generation = state.placeholder_generation.wrapping_add(1).max(1);
    match WorldPlaceholderMeshCursor::new(key, kind, state.placeholder_generation, state.interaction_revision) {
        Ok(cursor) => state.placeholder_build = Some(cursor),
        Err(_) => mark_world_dynamic_fault(state, WorldDynamicFault::ByteCapacity),
    }
}

fn step_world_placeholder_mesh(state: &mut World3dState) {
    let Some(mut cursor) = state.placeholder_build.take() else { return };
    match cursor.step() {
        WorldPlaceholderMeshStep::Pending => state.placeholder_build = Some(cursor),
        WorldPlaceholderMeshStep::Ready(key, lease) => store_mesh(state, key, lease),
        WorldPlaceholderMeshStep::Fault => mark_world_dynamic_fault(state, WorldDynamicFault::Closing),
    }
}
//#endregion PlaceholderMeshAuthority

//#region WorldMeshBuffers
/// 🧱️ Infinite-owned flat render-buffer twin of the renderer's `WorldMeshData` (see
/// `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️.tsx`
/// ~line 98). Ephemeral view state, not document content: the document owns mesh content through
/// the artifact system, this is only the wire shape the viewport deserializes and rasterizes.
#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldMeshBuffers {
    #[serde(default)]
    positions: Vec<f32>,
    #[serde(default)]
    normals: Vec<f32>,
    #[serde(default)]
    indices: Vec<u32>,
    #[serde(default)]
    #[allow(dead_code)]
    colors: Vec<f32>,
    #[serde(default)]
    uvs: Vec<f32>,
    #[serde(default)]
    face_ids: Vec<u32>,
    #[serde(default)]
    vertex_ids: Vec<u32>,
    #[serde(default)]
    edge_positions: Vec<f32>,
    #[serde(default)]
    edge_ids: Vec<u32>,
    #[serde(default)]
    paint_texture_base64: Option<String>,
}

impl WorldMeshBuffers {
    fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    /// 🧭️ Per-triangle flat-shaded normals, accumulated per vertex and renormalized — same
    /// algorithm as the dissolved mesh-engine's `MeshData::compute_normals`.
    fn compute_normals(&mut self) {
        let count = self.vertex_count();
        self.normals = vec![0.0; count * 3];
        for tri in self.indices.chunks_exact(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p0 = [self.positions[i0 * 3], self.positions[i0 * 3 + 1], self.positions[i0 * 3 + 2]];
            let p1 = [self.positions[i1 * 3], self.positions[i1 * 3 + 1], self.positions[i1 * 3 + 2]];
            let p2 = [self.positions[i2 * 3], self.positions[i2 * 3 + 1], self.positions[i2 * 3 + 2]];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            for &idx in tri {
                let i = idx as usize * 3;
                self.normals[i] += n[0];
                self.normals[i + 1] += n[1];
                self.normals[i + 2] += n[2];
            }
        }
        for chunk in self.normals.chunks_exact_mut(3) {
            let len = (chunk[0] * chunk[0] + chunk[1] * chunk[1] + chunk[2] * chunk[2]).sqrt();
            if len > 1e-8 {
                chunk[0] /= len;
                chunk[1] /= len;
                chunk[2] /= len;
            }
        }
    }
}

//#endregion WorldMeshBuffers

//#region PlaceholderMesh
/// 🧊️ Viewport placeholder markers only (gumball plane, reference plane, vortex arrow parts, and
/// the box fallback for an unresolved `mesh_id`) — demo geometry, never document content. Ports
/// only the primitive kinds actually reachable from this file's `mesh_from_kind` call sites
/// (census in 📓️wave-g1b-infinite-report.md); the dissolved mesh-engine's other primitives
/// (uv-sphere, ico-sphere at other radii, torus, …) are not reachable here and were not ported.
#[cfg(test)]
fn placeholder_mesh(kind: &str) -> WorldMeshBuffers {
    match kind {
        "vortex-marker" => placeholder_ico_sphere(0.12, 1),
        "plane" => placeholder_plane(1.0, 1.0),
        "cylinder" => placeholder_cylinder(0.5, 1.0, 16),
        "cone" => placeholder_cone(0.5, 1.0, 16),
        _ => placeholder_box(1.0, 1.0, 1.0),
    }
}

#[cfg(test)]
fn placeholder_push_triangle(mesh: &mut WorldMeshBuffers, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

#[cfg(test)]
fn placeholder_box(width: f32, height: f32, depth: f32) -> WorldMeshBuffers {
    let hw = width * 0.5;
    let hh = height * 0.5;
    let hd = depth * 0.5;
    let mut mesh = WorldMeshBuffers::default();
    let faces = [
        ([-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]),
        ([hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]),
        ([-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]),
        ([-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]),
        ([hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]),
        ([-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]),
    ];
    for (a, b, c, d) in faces {
        placeholder_push_triangle(&mut mesh, a, b, c);
        placeholder_push_triangle(&mut mesh, a, c, d);
    }
    mesh.compute_normals();
    mesh
}

#[cfg(test)]
fn placeholder_plane(width: f32, depth: f32) -> WorldMeshBuffers {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = WorldMeshBuffers::default();
    placeholder_push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    placeholder_push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

#[cfg(test)]
fn placeholder_cylinder(radius: f32, height: f32, segments: u32) -> WorldMeshBuffers {
    let mut mesh = WorldMeshBuffers::default();
    let half = height * 0.5;
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p00 = [radius * a0.cos(), -half, radius * a0.sin()];
        let p01 = [radius * a1.cos(), -half, radius * a1.sin()];
        let p10 = [radius * a0.cos(), half, radius * a0.sin()];
        let p11 = [radius * a1.cos(), half, radius * a1.sin()];
        placeholder_push_triangle(&mut mesh, p00, p01, p11);
        placeholder_push_triangle(&mut mesh, p00, p11, p10);
        placeholder_push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00);
        placeholder_push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11);
    }
    mesh.compute_normals();
    mesh
}

#[cfg(test)]
fn placeholder_cone(radius: f32, height: f32, segments: u32) -> WorldMeshBuffers {
    let mut mesh = WorldMeshBuffers::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        placeholder_push_triangle(&mut mesh, apex, p1, p0);
        placeholder_push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1);
    }
    mesh.compute_normals();
    mesh
}

fn placeholder_normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn placeholder_scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[cfg(test)]
fn placeholder_midpoint(verts: &mut Vec<[f32; 3]>, cache: &mut HashMap<(u32, u32), u32>, a: u32, b: u32) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = placeholder_normalize3([(verts[a as usize][0] + verts[b as usize][0]) * 0.5, (verts[a as usize][1] + verts[b as usize][1]) * 0.5, (verts[a as usize][2] + verts[b as usize][2]) * 0.5]);
    let index = verts.len() as u32;
    verts.push(mid);
    cache.insert(key, index);
    index
}

#[cfg(test)]
fn placeholder_ico_sphere(radius: f32, subdivisions: u32) -> WorldMeshBuffers {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        placeholder_normalize3([-1.0, t, 0.0]),
        placeholder_normalize3([1.0, t, 0.0]),
        placeholder_normalize3([-1.0, -t, 0.0]),
        placeholder_normalize3([1.0, -t, 0.0]),
        placeholder_normalize3([0.0, -1.0, t]),
        placeholder_normalize3([0.0, 1.0, t]),
        placeholder_normalize3([0.0, -1.0, -t]),
        placeholder_normalize3([0.0, 1.0, -t]),
        placeholder_normalize3([t, 0.0, -1.0]),
        placeholder_normalize3([t, 0.0, 1.0]),
        placeholder_normalize3([-t, 0.0, -1.0]),
        placeholder_normalize3([-t, 0.0, 1.0]),
    ];
    let mut faces =
        vec![[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = HashMap::new();
        for face in &faces {
            let a = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[[face[0], a, c], [face[1], b, a], [face[2], c, b], [a, b, c]]);
        }
        faces = next;
    }
    let mut mesh = WorldMeshBuffers::default();
    for face in faces {
        let a = placeholder_scale3(verts[face[0] as usize], radius);
        let b = placeholder_scale3(verts[face[1] as usize], radius);
        let c = placeholder_scale3(verts[face[2] as usize], radius);
        placeholder_push_triangle(&mut mesh, a, b, c);
    }
    mesh.compute_normals();
    mesh
}
//#endregion PlaceholderMesh

fn selection_mode_label(state: &World3dState) -> &'static str {
    match state.granularity.as_str() {
        "vertex" | "edge" | "face" => "component",
        _ => "mesh",
    }
}

fn component_mode_active(state: &World3dState) -> bool {
    matches!(state.granularity.as_str(), "vertex" | "edge" | "face" | "component")
}

const CLICK_DRAG_THRESHOLD_PX: f32 = 4.0;
const PICK_VERTEX_SCREEN_PX: f32 = 14.0;
const PICK_EDGE_SCREEN_PX: f32 = 18.0;
const FACE_OVERLAY_OFFSET: f32 = 0.003;

fn world_pick_rect(state: &World3dState) -> Rect {
    if state.pick_bounds.w > 0.0 && state.pick_bounds.h > 0.0 {
        state.pick_bounds
    } else {
        state.bounds
    }
}

fn render_pick_viewport(state: &World3dState) -> Rect {
    state.bounds
}

fn pointer_in_pick_rect(state: &World3dState, x: f32, y: f32) -> Option<(f32, f32, Rect)> {
    let clip = world_pick_rect(state);
    if !clip.contains(x, y) {
        return None;
    }
    let viewport = render_pick_viewport(state);
    if !viewport.contains(x, y) {
        return None;
    }
    Some((x - viewport.x, y - viewport.y, viewport))
}

fn pick_targets_instance(state: &World3dState, instance_id: &str) -> bool {
    state.active_object_id.as_deref().is_none_or(|active_id| active_id == instance_id)
}

fn pointer_drag_distance(state: &World3dState, x: f32, y: f32) -> f32 {
    let Some(start) = state.marquee_points.first() else {
        return 0.0;
    };
    let dx = x - start[0];
    let dy = y - start[1];
    (dx * dx + dy * dy).sqrt()
}

fn is_click_gesture(state: &World3dState, x: f32, y: f32) -> bool {
    pointer_drag_distance(state, x, y) <= CLICK_DRAG_THRESHOLD_PX
}

fn push_line_segment(lines: &mut Vec<LineVertex3d>, from: Vec3, to: Vec3, color: [f32; 4]) {
    lines.push(LineVertex3d { position: from.to_array(), color });
    lines.push(LineVertex3d { position: to.to_array(), color });
}

const VERTEX_MARKER_MESH: &str = "vertex-marker";
const VERTEX_BASE_SCALE: f32 = 0.05;
const VERTEX_HOVER_SCALE: f32 = 0.09;
const VERTEX_SELECT_SCALE: f32 = 0.09;

fn component_overlay_color(id: &str, selected: &HashSet<String>, preview: &HashSet<String>, hovered: &Option<String>) -> Option<([f32; 4], f32)> {
    if preview.contains(id) {
        return Some(([1.0, 0.85, 0.35, 1.0], VERTEX_HOVER_SCALE));
    }
    if hovered.as_deref() == Some(id) {
        return Some(([0.35, 0.75, 1.0, 0.9], VERTEX_HOVER_SCALE));
    }
    if selected.contains(id) {
        return Some(([0.35, 0.75, 1.0, 1.0], VERTEX_SELECT_SCALE));
    }
    None
}

fn mesh_face_id(mesh: Mesh3dLease, tri_index: u32) -> String {
    world_mesh_component_id(mesh, Mesh3dField::FaceIds, tri_index).to_string()
}

fn face_component_mode_active(state: &World3dState) -> bool {
    state.selection_targets.face || state.granularity == "face"
}

fn apply_hovered_component_from_selection(state: &mut World3dState, selection_json: &str) {
    let Some(selection_value) = serde_json::from_str::<serde_json::Value>(selection_json).ok() else {
        return;
    };
    match selection_value.get("hoveredComponent") {
        None => return,
        Some(value) if value.is_null() => {
            state.hovered_component_id = None;
            state.hovered_component_object_id = None;
            state.hovered_component_mode = None;
        }
        Some(value) => {
            state.hovered_component_id = value.get("id").and_then(json_id_to_string);
            state.hovered_component_object_id = value.get("objectId").and_then(|entry| entry.as_str()).map(str::to_string);
            state.hovered_component_mode = value.get("mode").and_then(|entry| entry.as_str()).map(str::to_string);
        }
    }
    if state.hovered_component_mode.as_deref() != Some(state.granularity.as_str()) {
        state.hovered_component_id = None;
        state.hovered_component_object_id = None;
        state.hovered_component_mode = None;
    }
}
fn mesh_vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    world_mesh_vertex(mesh, index)
}

fn instance_hovered_component_id(state: &World3dState, instance_id: &str) -> Option<String> {
    if !pick_targets_instance(state, instance_id) {
        return None;
    }
    if state.hovered_component_mode.as_deref() != Some(state.granularity.as_str()) {
        return None;
    }
    if state.hovered_component_object_id.as_deref().is_some_and(|object_id| object_id != instance_id) {
        return None;
    }
    state.hovered_component_id.clone()
}

fn append_component_vertex_spheres(_state: &mut World3dState) -> Vec<Instance3d> {
    Vec::new()
}

fn append_component_overlays(state: &World3dState, lines: &mut Vec<LineVertex3d>) {
    let wire_color = [0.55, 0.65, 0.8, 0.75];
    if state.interaction_mode == "paint" || component_mode_active(state) || state.show_edges || state.selection_targets.edge || (state.granularity == "mesh" && !state.component_ids.is_empty()) {
        for draw in &state.draws {
            let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            if schema.edges == 0 {
                continue;
            }
            for instance in &draw.instances {
                for edge_index in 0..schema.edges {
                    let Ok(edge) = mesh.edge(edge_index) else { continue };
                    let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                    let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                    lines.push(LineVertex3d { position: a.to_array(), color: wire_color });
                    lines.push(LineVertex3d { position: b.to_array(), color: wire_color });
                }
            }
        }
    }
    let selected: HashSet<String> = state.component_ids.iter().cloned().collect();
    let preview: HashSet<String> = state.marquee_preview_ids.iter().cloned().collect();
    for draw in &state.draws {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let Ok(schema) = mesh.schema() else { continue };
        for instance in &draw.instances {
            let hovered = instance_hovered_component_id(state, &instance.id);
            if state.granularity.as_str() != "edge" {
                continue;
            }
            if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                continue;
            }
            for edge_index in 0..schema.edges {
                let Ok(edge) = mesh.edge(edge_index) else { continue };
                let id = world_mesh_component_id(mesh, Mesh3dField::EdgeIds, edge_index).to_string();
                let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered) else {
                    continue;
                };
                let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                push_line_segment(lines, a, b, color);
            }
        }
    }
    if face_component_mode_active(state) {
        for draw in &state.draws {
            let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            for instance in &draw.instances {
                let hovered = instance_hovered_component_id(state, &instance.id);
                if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                    continue;
                }
                for tri_index in 0..schema.indices / 3 {
                    let Some(tri) = world_mesh_triangle(mesh, tri_index) else { continue };
                    let id = mesh_face_id(mesh, tri_index);
                    let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered) else {
                        continue;
                    };
                    let Some(verts) = Option::zip(Option::zip(mesh_vertex(mesh, tri[0]), mesh_vertex(mesh, tri[1])), mesh_vertex(mesh, tri[2]))
                        .map(|((a, b), c)| [instance.model.transform_point(a), instance.model.transform_point(b), instance.model.transform_point(c)])
                    else {
                        continue;
                    };
                    push_line_segment(lines, verts[0], verts[1], color);
                    push_line_segment(lines, verts[1], verts[2], color);
                    push_line_segment(lines, verts[2], verts[0], color);
                }
            }
        }
    }
    if state.selection_targets.vertex || state.granularity == "vertex" {
        let wire_color = [0.55, 0.65, 0.8, 0.9];
        for draw in &state.draws {
            let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            for instance in &draw.instances {
                let hovered = instance_hovered_component_id(state, &instance.id);
                for vertex_index in 0..schema.vertices {
                    let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                    let id = world_mesh_component_id(mesh, Mesh3dField::VertexIds, vertex_index).to_string();
                    let center = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                    let (color, scale) = component_overlay_color(&id, &selected, &preview, &hovered).unwrap_or((wire_color, VERTEX_BASE_SCALE));
                    if !state.selection_targets.vertex && component_overlay_color(&id, &selected, &preview, &hovered).is_none() {
                        continue;
                    }
                    let d = scale * 0.15;
                    push_line_segment(lines, center.sub(Vec3::new(d, 0.0, 0.0)), center.add(Vec3::new(d, 0.0, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, d, 0.0)), center.add(Vec3::new(0.0, d, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, 0.0, d)), center.add(Vec3::new(0.0, 0.0, d)), color);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldFaceOverlayPhase {
    Count,
    Prepare,
    Allocate,
    Geometry,
    Indices,
    Seal,
    Publish,
    NextBucket,
    Complete,
}

#[derive(Clone, Copy)]
struct WorldFaceOverlayMatch {
    draw: u16,
    instance: u16,
    triangle: u32,
    category: u8,
    hovered: bool,
}

#[derive(Clone, Copy, Default)]
struct WorldFaceOverlayScan {
    draw: u16,
    instance: u16,
    triangle: u32,
    candidate: Option<u32>,
    hovered: bool,
    preview: usize,
    selected: usize,
}

enum WorldFaceOverlayScanStep {
    Pending,
    Match(WorldFaceOverlayMatch),
    Complete,
}

impl WorldFaceOverlayScan {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn finish_candidate(&mut self, category: Option<u8>) -> WorldFaceOverlayScanStep {
        let matched = category.map(|category| WorldFaceOverlayMatch { draw: self.draw, instance: self.instance, triangle: self.triangle, category, hovered: self.hovered });
        self.triangle = self.triangle.saturating_add(1);
        self.candidate = None;
        self.hovered = false;
        self.preview = 0;
        self.selected = 0;
        matched.map_or(WorldFaceOverlayScanStep::Pending, WorldFaceOverlayScanStep::Match)
    }

    fn step(&mut self, state: &World3dState) -> WorldFaceOverlayScanStep {
        if let Some(face_id) = self.candidate {
            if self.preview < state.marquee_preview_ids.len() {
                let matches = decimal_component_id_matches(&state.marquee_preview_ids[self.preview], face_id);
                self.preview += 1;
                return if matches { self.finish_candidate(Some(0)) } else { WorldFaceOverlayScanStep::Pending };
            }
            if self.hovered {
                return self.finish_candidate(Some(1));
            }
            if self.selected < state.component_ids.len() {
                let matches = decimal_component_id_matches(&state.component_ids[self.selected], face_id);
                self.selected += 1;
                return if matches { self.finish_candidate(Some(2)) } else { WorldFaceOverlayScanStep::Pending };
            }
            return self.finish_candidate(None);
        }
        if usize::from(self.draw) >= usize::from(state.draws.len) {
            return WorldFaceOverlayScanStep::Complete;
        }
        let Some(draw) = state.draws.get(usize::from(self.draw)) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        if usize::from(self.instance) >= draw.instances.len() {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        }
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        let Ok(schema) = mesh.schema() else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        if self.triangle >= schema.indices / 3 {
            self.instance += 1;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        }
        let face_id = world_mesh_component_id(mesh, Mesh3dField::FaceIds, self.triangle);
        self.candidate = Some(face_id);
        self.hovered = instance_hovered_component_matches(state, &draw.instances[usize::from(self.instance)].id, face_id);
        WorldFaceOverlayScanStep::Pending
    }
}

#[derive(Clone, Copy)]
struct WorldFaceOverlayGeometry {
    vertices: [Vec3; 3],
    normal: Vec3,
}

enum WorldFaceOverlayMeshStep {
    Pending,
    Ready { index: usize, color: [f32; 4], key: String, lease: Mesh3dLease },
    Complete { generation: u64, revision: u64, draw_generation: u64, colors: [Option<[f32; 4]>; 3] },
    Stale,
    Fault,
}

struct WorldFaceOverlayMeshCursor {
    surface_id: String,
    generation: u64,
    revision: u64,
    draw_generation: u64,
    phase: WorldFaceOverlayPhase,
    scan: WorldFaceOverlayScan,
    counts: [u32; 3],
    order: [u8; 3],
    order_len: u8,
    published_colors: [Option<[f32; 4]>; 3],
    bucket: u8,
    geometry: Option<WorldFaceOverlayGeometry>,
    geometry_item: u8,
    index_item: u32,
    owner: WorldPlaceholderOwner,
    retry_key: String,
    close_started: bool,
    faulted: bool,
    stale: bool,
}

impl WorldFaceOverlayMeshCursor {
    fn new(surface_id: &str, generation: u64, revision: u64, draw_generation: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        if surface_id.len().checked_add("component-face-overlay::2".len()).is_none_or(|bytes| bytes > WORLD_DYNAMIC_ID_BYTE_CAPACITY) {
            return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity);
        }
        Ok(Self {
            surface_id: surface_id.to_owned(),
            generation,
            revision,
            draw_generation,
            phase: WorldFaceOverlayPhase::Count,
            scan: WorldFaceOverlayScan::default(),
            counts: [0; 3],
            order: [0; 3],
            order_len: 0,
            published_colors: [None; 3],
            bucket: 0,
            geometry: None,
            geometry_item: 0,
            index_item: 0,
            owner: WorldPlaceholderOwner::Empty,
            retry_key: String::new(),
            close_started: false,
            faulted: false,
            stale: false,
        })
    }

    fn key(&self, index: usize) -> String {
        format!("component-face-overlay:{}:{}:{index}", self.surface_id, self.generation)
    }

    fn token(&self) -> Result<Mesh3dWriteToken, ui_wgpu::wgpu::Mesh3dFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(ui_wgpu::wgpu::Mesh3dFault::Stale),
        }
    }

    fn step(&mut self, state: &World3dState) -> WorldFaceOverlayMeshStep {
        if state.interaction_revision != self.revision || state.draw_generation != self.draw_generation {
            self.faulted = true;
            self.stale = true;
        }
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() {
                if self.stale {
                    WorldFaceOverlayMeshStep::Stale
                } else {
                    WorldFaceOverlayMeshStep::Fault
                }
            } else {
                WorldFaceOverlayMeshStep::Pending
            };
        }
        match self.step_live(state) {
            Ok(step) => step,
            Err(_) => {
                self.faulted = true;
                let _ = self.close_step();
                WorldFaceOverlayMeshStep::Pending
            }
        }
    }

    fn step_live(&mut self, state: &World3dState) -> Result<WorldFaceOverlayMeshStep, ui_wgpu::wgpu::Mesh3dFault> {
        match self.phase {
            WorldFaceOverlayPhase::Count => match self.scan.step(state) {
                WorldFaceOverlayScanStep::Pending => {}
                WorldFaceOverlayScanStep::Match(found) => {
                    let category = usize::from(found.category);
                    if self.counts[category] == 0 {
                        self.order[usize::from(self.order_len)] = found.category;
                        self.order_len += 1;
                    }
                    self.counts[category] = self.counts[category].checked_add(1).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                }
                WorldFaceOverlayScanStep::Complete => {
                    self.scan.reset();
                    self.phase = WorldFaceOverlayPhase::Prepare;
                }
            },
            WorldFaceOverlayPhase::Prepare => {
                if self.bucket == self.order_len {
                    self.phase = WorldFaceOverlayPhase::Complete;
                } else {
                    let count = self.counts[usize::from(self.order[usize::from(self.bucket)])];
                    let vertices = count.checked_mul(3).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                    let indices = count.checked_mul(6).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                    self.owner = WorldPlaceholderOwner::Writing(mesh3d_begin(self.generation + u64::from(self.bucket), self.revision, Mesh3dSchema::triangle_mesh(vertices, indices))?);
                    self.phase = WorldFaceOverlayPhase::Allocate;
                }
            }
            WorldFaceOverlayPhase::Allocate => {
                if mesh3d_allocate_step(self.token()?)? {
                    self.scan.reset();
                    self.phase = WorldFaceOverlayPhase::Geometry;
                }
            }
            WorldFaceOverlayPhase::Geometry => {
                if let Some(geometry) = self.geometry {
                    let vertex = usize::from(self.geometry_item / 2);
                    let field = self.geometry_item % 2;
                    if field == 0 {
                        mesh3d_write_vec3(self.token()?, Mesh3dField::Positions, geometry.vertices[vertex].to_array())?;
                    } else {
                        mesh3d_write_vec3(self.token()?, Mesh3dField::Normals, geometry.normal.to_array())?;
                    }
                    self.geometry_item += 1;
                    if self.geometry_item == 6 {
                        self.geometry_item = 0;
                        self.geometry = None;
                    }
                } else {
                    match self.scan.step(state) {
                        WorldFaceOverlayScanStep::Pending => {}
                        WorldFaceOverlayScanStep::Match(found) if found.category == self.order[usize::from(self.bucket)] => {
                            self.geometry = Some(face_overlay_geometry(state, found).ok_or(ui_wgpu::wgpu::Mesh3dFault::Stale)?);
                        }
                        WorldFaceOverlayScanStep::Match(_) => {}
                        WorldFaceOverlayScanStep::Complete => {
                            self.index_item = 0;
                            self.phase = WorldFaceOverlayPhase::Indices;
                        }
                    }
                }
            }
            WorldFaceOverlayPhase::Indices => {
                let category = usize::from(self.order[usize::from(self.bucket)]);
                let total = self.counts[category] * 6;
                let pattern = [0, 1, 2, 0, 2, 1];
                let value = (self.index_item / 6) * 3 + pattern[(self.index_item % 6) as usize];
                mesh3d_write_u32(self.token()?, Mesh3dField::Indices, value)?;
                self.index_item += 1;
                if self.index_item == total {
                    self.phase = WorldFaceOverlayPhase::Seal;
                }
            }
            WorldFaceOverlayPhase::Seal => {
                self.owner = WorldPlaceholderOwner::Ready(mesh3d_seal(self.token()?)?);
                self.phase = WorldFaceOverlayPhase::Publish;
            }
            WorldFaceOverlayPhase::Publish => {
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(ui_wgpu::wgpu::Mesh3dFault::Stale) };
                self.owner = WorldPlaceholderOwner::Empty;
                self.phase = WorldFaceOverlayPhase::NextBucket;
                let index = usize::from(self.bucket);
                let key = if self.retry_key.is_empty() { self.key(index) } else { std::mem::take(&mut self.retry_key) };
                return Ok(WorldFaceOverlayMeshStep::Ready { index, color: face_overlay_color(self.order[index]), key, lease });
            }
            WorldFaceOverlayPhase::NextBucket => {
                self.bucket += 1;
                self.scan.reset();
                self.phase = WorldFaceOverlayPhase::Prepare;
            }
            WorldFaceOverlayPhase::Complete => {
                self.surface_id.clear();
                return Ok(WorldFaceOverlayMeshStep::Complete { generation: self.generation, revision: self.revision, draw_generation: self.draw_generation, colors: self.published_colors });
            }
        }
        Ok(WorldFaceOverlayMeshStep::Pending)
    }

    fn close_step(&mut self) -> bool {
        self.geometry = None;
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match mesh3d_abort_step(token) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match mesh3d_close_step(lease) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Empty => {
                if self.retry_key.pop().is_some() {
                    return false;
                }
                if self.surface_id.pop().is_some() {
                    return false;
                }
                true
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.owner, WorldPlaceholderOwner::Empty) && self.surface_id.is_empty() && self.retry_key.is_empty() && self.geometry.is_none()
    }
}

#[cfg(not(test))]
impl Drop for WorldFaceOverlayMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "face overlay mesh cursor reached Drop before its exact authority witness");
    }
}

fn decimal_component_id_matches(value: &str, mut expected: u32) -> bool {
    if expected == 0 {
        return value == "0";
    }
    let digits = (expected.ilog10() + 1) as usize;
    if value.len() != digits || !value.is_ascii() {
        return false;
    }
    for digit in value.as_bytes().iter().rev() {
        if *digit != b'0' + (expected % 10) as u8 {
            return false;
        }
        expected /= 10;
    }
    true
}

fn instance_hovered_component_matches(state: &World3dState, instance_id: &str, face_id: u32) -> bool {
    pick_targets_instance(state, instance_id)
        && state.hovered_component_mode.as_deref() == Some(state.granularity.as_str())
        && state.hovered_component_object_id.as_deref().is_none_or(|object_id| object_id == instance_id)
        && state.hovered_component_id.as_deref().is_some_and(|id| decimal_component_id_matches(id, face_id))
}

fn face_overlay_color(category: u8) -> [f32; 4] {
    match category {
        0 => [1.0, 0.85, 0.35, 0.36],
        1 => [0.35, 0.75, 1.0, 0.48],
        _ => [0.35, 0.75, 1.0, 0.62],
    }
}

fn face_overlay_geometry(state: &World3dState, found: WorldFaceOverlayMatch) -> Option<WorldFaceOverlayGeometry> {
    let draw = state.draws.get(usize::from(found.draw))?;
    let instance = draw.instances.get(usize::from(found.instance))?;
    let mesh = *state.meshes.get(&draw.mesh_key)?;
    let triangle = world_mesh_triangle(mesh, found.triangle)?;
    let vertices = [instance.model.transform_point(mesh_vertex(mesh, triangle[0])?), instance.model.transform_point(mesh_vertex(mesh, triangle[1])?), instance.model.transform_point(mesh_vertex(mesh, triangle[2])?)];
    let normal = vertices[1].sub(vertices[0]).cross(vertices[2].sub(vertices[0])).normalize();
    let offset = if found.hovered { FACE_OVERLAY_OFFSET } else { FACE_OVERLAY_OFFSET * 0.5 };
    Some(WorldFaceOverlayGeometry { vertices: vertices.map(|vertex| vertex.add(normal.scale(offset))), normal })
}

fn step_component_face_overlay_build(state: &mut World3dState) {
    let Some(mut cursor) = state.face_overlay_build.take() else { return };
    match cursor.step(state) {
        WorldFaceOverlayMeshStep::Pending => state.face_overlay_build = Some(cursor),
        WorldFaceOverlayMeshStep::Ready { index, color, key, lease } => match publish_world3d_mesh_lease(state, key, lease) {
            Ok(()) => {
                cursor.published_colors[index] = Some(color);
                state.face_overlay_build = Some(cursor);
            }
            Err(rejected) => {
                cursor.owner = WorldPlaceholderOwner::Ready(rejected.value);
                cursor.retry_key = rejected.id;
                cursor.phase = WorldFaceOverlayPhase::Publish;
                cursor.close_started = false;
                state.face_overlay_build = Some(cursor);
            }
        },
        WorldFaceOverlayMeshStep::Complete { generation, revision, draw_generation, colors } => {
            state.face_overlay_retired_generation = state.face_overlay_generation.replace(generation);
            state.face_overlay_colors = colors;
            state.face_overlay_applied_revision = revision;
            state.face_overlay_applied_draw_generation = draw_generation;
        }
        WorldFaceOverlayMeshStep::Stale => {}
        WorldFaceOverlayMeshStep::Fault => mark_world_dynamic_fault(state, WorldDynamicFault::Closing),
    }
}

fn append_component_face_translucent_overlays(state: &mut World3dState, gpu: &mut World3dBuildContext, translucent: &mut Vec<SceneDraw3d>) {
    if !face_component_mode_active(state) {
        state.face_overlay_applied_revision = u64::MAX;
        state.face_overlay_applied_draw_generation = u64::MAX;
        if let Some(cursor) = state.face_overlay_build.as_mut() {
            cursor.faulted = true;
            cursor.stale = true;
            step_component_face_overlay_build(state);
            return;
        }
        if state.face_overlay_generation.is_some() {
            state.face_overlay_retired_generation = state.face_overlay_generation.take();
            state.face_overlay_colors = [None; 3];
        }
        return;
    }
    let had_build = state.face_overlay_build.is_some();
    step_component_face_overlay_build(state);
    if !had_build
        && state.face_overlay_build.is_none()
        && state.face_overlay_retired_generation.is_none()
        && (state.face_overlay_applied_revision != state.interaction_revision || state.face_overlay_applied_draw_generation != state.draw_generation)
        && state.dynamic_mesh_close.is_none()
        && state.dynamic_blocked_mesh.is_none()
        && state.snapshot_fault.is_none()
    {
        let generation = state.placeholder_generation.checked_add(3);
        match generation.and_then(|next| WorldFaceOverlayMeshCursor::new(&state.surface_id, state.placeholder_generation + 1, state.interaction_revision, state.draw_generation).ok().map(|cursor| (next, cursor))) {
            Some((next, cursor)) => {
                state.placeholder_generation = next;
                state.face_overlay_build = Some(cursor);
            }
            None => mark_world_dynamic_fault(state, WorldDynamicFault::ByteCapacity),
        }
    }
    let Some(generation) = state.face_overlay_generation else { return };
    for index in 0..3 {
        let Some(color) = state.face_overlay_colors[index] else { continue };
        let mesh_key = format!("component-face-overlay:{}:{generation}:{index}", state.surface_id);
        let Some(mesh) = state.meshes.get(&mesh_key) else { continue };
        let mesh_version = *state.mesh_versions.get(&mesh_key).unwrap_or(&0);
        gpu.ensure_mesh(&mesh_key, mesh_version, *mesh);
        translucent.push(SceneDraw3d { mesh_key, mesh_version, instances: vec![Instance3d { id: format!("face-overlay-{index}"), model: Mat4::identity(), color, selected: false, hovered: false }] });
    }
}

fn selection_centroid(state: &World3dState) -> Option<Vec3> {
    if let Some(target) = state.gumball_target {
        return Some(Vec3::new(target[0], target[1], target[2]));
    }
    if state.selected_ids.is_empty() {
        return None;
    }
    let mut sum = Vec3::ZERO;
    let mut count = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            if state.selected_ids.iter().any(|id| id == &instance.id) {
                let t = instance.model.cols[3];
                sum = sum.add(Vec3::new(t[0], t[1], t[2]));
                count += 1;
            }
        }
    }
    if count == 0 {
        None
    } else {
        Some(sum.scale(1.0 / count as f32))
    }
}

fn pick_gumball_handle_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<GumballHandle> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let pivot = selection_centroid(state)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let pick_radius = extent * 0.08;
    let eye = gumball_eye(&camera, pivot);
    let mut best: Option<(f32, GumballHandle)> = None;
    let axes = [(GumballHandle::MoveX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]), (GumballHandle::MoveY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]), (GumballHandle::MoveZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0])];
    for (handle, axis, _) in axes {
        let end = pivot.add(axis.scale(extent));
        if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
            if dist <= pick_radius && best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                best = Some((dist, handle));
            }
        }
    }
    let planes = [(GumballHandle::MoveXY, Vec3::new(0.0, 0.0, 1.0), extent * 0.35), (GumballHandle::MoveYZ, Vec3::new(1.0, 0.0, 0.0), extent * 0.35), (GumballHandle::MoveXZ, Vec3::new(0.0, 1.0, 0.0), extent * 0.35)];
    for (handle, normal, half) in planes {
        if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
            let offset = hit.sub(pivot);
            let u = if normal.z.abs() > 0.9 {
                offset.x.abs()
            } else if normal.x.abs() > 0.9 {
                offset.y.abs()
            } else {
                offset.x.abs()
            };
            let v = if normal.z.abs() > 0.9 { offset.y.abs() } else { offset.z.abs() };
            if u <= half && v <= half {
                let dist = origin.sub(hit).length();
                if best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    if matches!(state.transform_mode.as_str(), "rotate" | "rotateSelection") {
        for handle in [GumballHandle::RotateX, GumballHandle::RotateY, GumballHandle::RotateZ] {
            let Some(normal) = handle.plane_normal() else {
                continue;
            };
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let radial = hit.sub(pivot);
                let dist_ring = (radial.length() - extent * 0.85).abs();
                if dist_ring <= pick_radius * 2.0 && best.as_ref().is_none_or(|(best_dist, _)| dist_ring < *best_dist) {
                    best = Some((dist_ring, handle));
                }
            }
        }
    }
    if matches!(state.transform_mode.as_str(), "scale" | "scaleSelection") {
        for handle in [GumballHandle::ScaleX, GumballHandle::ScaleY, GumballHandle::ScaleZ] {
            let Some(axis) = handle.axis_dir() else {
                continue;
            };
            let end = pivot.add(axis.scale(extent * 1.1));
            if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
                if dist <= pick_radius && best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    let _ = eye;
    best.map(|(_, handle)| handle)
}

fn append_gumball_geometry(
    lines: &mut Vec<LineVertex3d>,
    translucent: &mut Vec<SceneDraw3d>,
    state: &World3dState,
    camera: &Camera3d,
    meshes: &WorldDynamicRegistry<Mesh3dLease, WORLD_DYNAMIC_MESH_CAPACITY>,
    mesh_versions: &WorldDynamicRegistry<u64, WORLD_DYNAMIC_MESH_CAPACITY>,
) {
    let Some(pivot) = selection_centroid(state) else {
        return;
    };
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let axis_colors = [(Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]), (Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]), (Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0])];
    for (axis, color) in axis_colors {
        let end = pivot.add(axis.scale(extent));
        lines.push(LineVertex3d { position: pivot.to_array(), color });
        lines.push(LineVertex3d { position: end.to_array(), color });
    }
    let ring_segments = 48usize;
    for (normal, color) in [(Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 0.85]), (Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 0.85]), (Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 0.85])] {
        let tangent_a = if normal.x.abs() > 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
        let tangent_b = normal.cross(tangent_a).normalize();
        let tangent_a = tangent_b.cross(normal).normalize();
        let radius = extent * 0.85;
        for step in 0..ring_segments {
            let a0 = step as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let a1 = (step + 1) as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let p0 = pivot.add(tangent_a.scale(a0.cos() * radius)).add(tangent_b.scale(a0.sin() * radius));
            let p1 = pivot.add(tangent_a.scale(a1.cos() * radius)).add(tangent_b.scale(a1.sin() * radius));
            lines.push(LineVertex3d { position: p0.to_array(), color });
            lines.push(LineVertex3d { position: p1.to_array(), color });
        }
    }
    if meshes.contains_key("gumball-plane") {
        let mesh_version = *mesh_versions.get("gumball-plane").unwrap_or(&0);
        let half = extent * 0.35;
        let plane_specs = [(Vec3::new(0.0, 0.0, 1.0), [half, half, 1.0]), (Vec3::new(1.0, 0.0, 0.0), [1.0, half, half]), (Vec3::new(0.0, 1.0, 0.0), [half, 1.0, half])];
        for (normal, scale) in plane_specs {
            let tangent = if normal.z.abs() > 0.9 {
                Vec3::new(1.0, 0.0, 0.0)
            } else if normal.x.abs() > 0.9 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            };
            let bitangent = normal.cross(tangent).normalize();
            let tangent = bitangent.cross(normal).normalize();
            let rotation = quat_from_basis(tangent, bitangent, normal);
            translucent.push(SceneDraw3d {
                mesh_key: "gumball-plane".into(),
                mesh_version,
                instances: vec![Instance3d { id: "gumball-plane".into(), model: Instance3d::model_from_trs(pivot.to_array(), rotation, scale), color: [0.75, 0.8, 0.9, 0.22], selected: false, hovered: false }],
            });
        }
    }
}

#[cfg(test)]
fn apply_gumball_preview(state: &mut World3dState) {
    if state.gumball_handle.is_none() {
        return;
    }
    let pivot = state.gumball_pivot;
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            if !state.selected_ids.iter().any(|id| id == &instance.id) {
                continue;
            }
            let translation = instance.model.cols[3];
            let base = Vec3::new(translation[0], translation[1], translation[2]);
            if state.gumball_preview_translate.length() > 1e-4 {
                let next = base.add(state.gumball_preview_translate);
                instance.model.cols[3] = [next.x, next.y, next.z, 1.0];
            } else if state.gumball_preview_angle.abs() > 1e-6 {
                if let Some(handle) = state.gumball_handle {
                    if let Some(axis) = handle.axis_dir() {
                        let offset = base.sub(pivot);
                        let rotated = rotate_vector(offset, axis, state.gumball_preview_angle);
                        let next = pivot.add(rotated);
                        instance.model.cols[3] = [next.x, next.y, next.z, 1.0];
                    }
                }
            } else if (state.gumball_preview_scale.x - 1.0).abs() > 1e-6 || (state.gumball_preview_scale.y - 1.0).abs() > 1e-6 || (state.gumball_preview_scale.z - 1.0).abs() > 1e-6 {
                if let Some(handle) = state.gumball_handle {
                    if let Some(axis) = handle.axis_dir() {
                        let offset = base.sub(pivot);
                        let scale = match handle {
                            GumballHandle::ScaleX => state.gumball_preview_scale.x,
                            GumballHandle::ScaleY => state.gumball_preview_scale.y,
                            GumballHandle::ScaleZ => state.gumball_preview_scale.z,
                            _ => 1.0,
                        };
                        let scaled = pivot.add(axis.scale(offset.dot(axis) * (scale - 1.0)).add(offset));
                        instance.model.cols[3] = [scaled.x, scaled.y, scaled.z, 1.0];
                    }
                }
            }
        }
    }
}

fn retained_gumball_gesture(state: &World3dState) -> Option<&WorldGumballGesture> {
    let authority = state.interaction_authority.as_ref()?;
    authority.gumball.as_ref().or_else(|| match authority.active.as_ref() {
        Some(WorldInteractionActive::GumballCommit { job, .. }) => Some(&job.gesture),
        _ => None,
    })
}

fn retained_gumball_preview_model(state: &World3dState, draw_index: usize, instance_index: usize, model: Mat4) -> Mat4 {
    let Some(gesture) = retained_gumball_gesture(state) else {
        return model;
    };
    let selected = gesture.selected[..usize::from(gesture.selected_len)]
        .iter()
        .flatten()
        .any(|token| state.interaction_objects.resolve(*token).is_some_and(|entry| entry.kind == WorldInteractionObjectKind::Instance && entry.values[0] == draw_index as f32 && entry.values[1] == instance_index as f32));
    if !selected {
        return model;
    }
    let mut preview = model;
    let translation = model.cols[3];
    let base = Vec3::new(translation[0], translation[1], translation[2]);
    let next = if gesture.translate.length() > 1e-4 {
        base.add(gesture.translate)
    } else if gesture.angle.abs() > 1e-6 {
        gesture.handle.axis_dir().map(|axis| gesture.pivot.add(rotate_vector(base.sub(gesture.pivot), axis, gesture.angle))).unwrap_or(base)
    } else if let Some(axis) = gesture.handle.axis_dir() {
        let factor = match gesture.handle {
            GumballHandle::ScaleX => gesture.scale.x,
            GumballHandle::ScaleY => gesture.scale.y,
            GumballHandle::ScaleZ => gesture.scale.z,
            _ => 1.0,
        };
        gesture.pivot.add(axis.scale(base.sub(gesture.pivot).dot(axis) * (factor - 1.0)).add(base.sub(gesture.pivot)))
    } else {
        base
    };
    preview.cols[3] = [next.x, next.y, next.z, 1.0];
    preview
}

fn reset_gumball_preview(state: &mut World3dState) {
    state.gumball_preview_translate = Vec3::ZERO;
    state.gumball_preview_angle = 0.0;
    state.gumball_preview_scale = Vec3::new(1.0, 1.0, 1.0);
}

fn gumball_commit_action(state: &World3dState) -> Option<ActionDescriptor> {
    let handle = state.gumball_handle?;
    let ids = state.selected_ids.clone();
    if ids.is_empty() {
        return None;
    }
    if handle.is_translate() && state.gumball_preview_translate.length() > 1e-4 {
        let delta = state.gumball_preview_translate;
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "translateSelection".into(),
            args: action_args(json!({
                "surfaceId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "dx": delta.x as f64,
                "dy": delta.y as f64,
                "dz": delta.z as f64,
            })),
        });
    }
    if handle.is_rotate() && state.gumball_preview_angle.abs() > 1e-6 {
        let axis = handle.axis_dir()?;
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "rotateSelection".into(),
            args: action_args(json!({
                "surfaceId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "ax": axis.x as f64,
                "ay": axis.y as f64,
                "az": axis.z as f64,
                "angle": state.gumball_preview_angle as f64,
            })),
        });
    }
    if handle.is_scale() {
        let scale = match handle {
            GumballHandle::ScaleX => state.gumball_preview_scale.x,
            GumballHandle::ScaleY => state.gumball_preview_scale.y,
            GumballHandle::ScaleZ => state.gumball_preview_scale.z,
            _ => 1.0,
        };
        if (scale - 1.0).abs() > 1e-6 {
            let mut args = json!({
                "surfaceId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "sx": 1.0,
                "sy": 1.0,
                "sz": 1.0,
            });
            if handle == GumballHandle::ScaleX {
                args["sx"] = json!(scale);
            } else if handle == GumballHandle::ScaleY {
                args["sy"] = json!(scale);
            } else {
                args["sz"] = json!(scale);
            }
            return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "scaleSelection".into(), args: action_args(args) });
        }
    }
    None
}

/// 🕒️ `pub` (was crate-private) so the wgpu renderer's wheel-zoom settle-then-dispatch sweep
/// (`AppRuntime::frame`'s `pending_camera_dispatch_deadlines_ms`) can build the same `setCamera`
/// action the pointer-release path below already dispatches — one orbit-to-action mapping, two
/// trigger sites (immediate on release, debounced on wheel settle).
pub fn orbit_camera_action(state: &World3dState) -> ActionDescriptor {
    let camera = state.orbit.to_camera();
    ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "setCamera".into(),
        args: action_args(json!({
            "surfaceId": state.surface_id,
            "camera": {
                "position": [
                    camera.position.x as f64,
                    camera.position.y as f64,
                    camera.position.z as f64,
                ],
                "target": [
                    camera.target.x as f64,
                    camera.target.y as f64,
                    camera.target.z as f64,
                ],
                "fov": state.orbit.fov_y.to_degrees() as f64,
            }
        })),
    }
}

pub fn publish_world3d_mesh_lease(state: &mut World3dState, id: String, mesh: Mesh3dLease) -> Result<(), WorldDynamicRejected<Mesh3dLease>> {
    let version = mesh.generation().rotate_left(17) ^ mesh.revision();
    if state.dynamic_mesh_close.is_some() || state.dynamic_blocked_mesh.is_some() {
        return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value: mesh });
    }
    let mesh_plan = state.meshes.plan_insert(&id).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let version_plan = state.mesh_versions.plan_insert(&id).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let interaction_plan = state.interaction_meshes.plan_admit(&id, version, mesh).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let (_, previous_mesh) = state.meshes.commit_insert(mesh_plan, id.clone(), mesh).map_err(|rejected| WorldDynamicRejected { fault: rejected.fault, id: rejected.id, value: mesh })?;
    state.interaction_meshes.commit_admit(interaction_plan).expect("world mesh transaction revalidates every fixed interaction slot before its first mutation");
    let (_, previous_version) = state.mesh_versions.commit_insert(version_plan, id, version).expect("world mesh transaction revalidates every fixed version slot before its first mutation");
    if let Some(mut previous_version) = previous_version {
        previous_version.id.clear();
    }
    if let Some(previous_mesh) = previous_mesh {
        state.dynamic_mesh_close = Some(previous_mesh);
    }
    Ok(())
}

fn store_mesh(state: &mut World3dState, id: String, mesh: Mesh3dLease) {
    if let Err(rejected) = publish_world3d_mesh_lease(state, id, mesh) {
        retain_rejected_world_mesh(state, rejected);
    }
}

fn retain_rejected_world_mesh(state: &mut World3dState, rejected: WorldDynamicRejected<Mesh3dLease>) {
    let fault = rejected.fault;
    if state.dynamic_blocked_mesh.is_none() {
        state.dynamic_blocked_mesh = Some(WorldDynamicEntry { id: rejected.id, epoch: 0, value: rejected.value });
    }
    mark_world_dynamic_fault(state, fault);
}

fn retain_world_blocked_owner(state: &mut World3dState, owner: WorldOpaqueOwner) {
    assert!(state.dynamic_blocked_owner.is_none(), "world dynamic producers must stop after an observable ownership fault");
    state.dynamic_blocked_owner = Some(owner);
}

fn set_world_mesh_version(state: &mut World3dState, id: String, version: u64) {
    match state.mesh_versions.insert(id, version) {
        Ok((_, previous)) => drop(previous),
        Err(rejected) => {
            drop(rejected.id);
            mark_world_dynamic_fault(state, rejected.fault);
        }
    }
}

fn mark_world_dynamic_fault(state: &mut World3dState, _fault: WorldDynamicFault) {
    state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
    if let Some(authority) = state.interaction_authority.as_mut() {
        authority.faulted = true;
    }
}

fn retire_world_mesh(state: &mut World3dState, id: &str) -> bool {
    if state.dynamic_mesh_close.is_some() || state.dynamic_blocked_mesh.is_some() {
        return false;
    }
    let Some(entry) = state.meshes.remove(id) else {
        return true;
    };
    state.dynamic_mesh_close = Some(entry);
    drop(state.mesh_versions.remove(id));
    true
}

fn publish_world_pixels(state: &mut World3dState, id: String, value: (u32, u32, Vec<u8>), paint: bool) -> bool {
    let registry = if paint { &mut state.mesh_paint_textures } else { &mut state.reference_pixels };
    match registry.insert(id, value) {
        Ok((token, None)) => {
            let _ = token;
            true
        }
        Ok((token, Some(previous))) => {
            let owner = if paint { WorldOpaqueOwner::PaintPixels(previous) } else { WorldOpaqueOwner::ReferencePixels(previous) };
            match quarantine_world_owner(owner) {
                Ok(_) => true,
                Err(owner) => {
                    let previous = match owner {
                        WorldOpaqueOwner::PaintPixels(entry) | WorldOpaqueOwner::ReferencePixels(entry) => entry,
                        _ => unreachable!("pixel replacement owner"),
                    };
                    let replacement = registry.remove_token(token).expect("new pixel token remains current while replacement is published");
                    registry.restore(previous);
                    let replacement = if paint { WorldOpaqueOwner::PaintPixels(replacement) } else { WorldOpaqueOwner::ReferencePixels(replacement) };
                    retain_world_blocked_owner(state, replacement);
                    mark_world_dynamic_fault(state, WorldDynamicFault::QuarantineCapacity);
                    false
                }
            }
        }
        Err(rejected) => {
            let owner = WorldDynamicEntry { id: rejected.id, epoch: 0, value: rejected.value };
            let owner = if paint { WorldOpaqueOwner::PaintPixels(owner) } else { WorldOpaqueOwner::ReferencePixels(owner) };
            let fault = rejected.fault;
            if let Err(owner) = quarantine_world_owner(owner) {
                retain_world_blocked_owner(state, owner);
            }
            mark_world_dynamic_fault(state, fault);
            false
        }
    }
}

fn retire_world_pixels(state: &mut World3dState, id: &str, paint: bool) -> bool {
    let registry = if paint { &mut state.mesh_paint_textures } else { &mut state.reference_pixels };
    let Some(entry) = registry.remove(id) else {
        return true;
    };
    let owner = if paint { WorldOpaqueOwner::PaintPixels(entry) } else { WorldOpaqueOwner::ReferencePixels(entry) };
    match quarantine_world_owner(owner) {
        Ok(_) => true,
        Err(owner) => {
            let entry = match owner {
                WorldOpaqueOwner::PaintPixels(entry) | WorldOpaqueOwner::ReferencePixels(entry) => entry,
                _ => unreachable!("pixel retirement owner"),
            };
            registry.restore(entry);
            mark_world_dynamic_fault(state, WorldDynamicFault::QuarantineCapacity);
            false
        }
    }
}

const WORLD3D_PREPARED_STATUS_BYTES: usize = 192;

#[derive(Clone, Copy)]
struct World3dPreparedStatus {
    bytes: [u8; WORLD3D_PREPARED_STATUS_BYTES],
    len: u8,
    progress: [f64; 3],
    total: u32,
    state: u32,
}

impl World3dPreparedStatus {
    fn from_parts(text: &str, progress: [f64; 3], total: u32, state: u32) -> Option<Self> {
        if text.len() > WORLD3D_PREPARED_STATUS_BYTES {
            return None;
        }
        let mut bytes = [0; WORLD3D_PREPARED_STATUS_BYTES];
        bytes[..text.len()].copy_from_slice(text.as_bytes());
        Some(Self { bytes, len: text.len() as u8, progress, total, state })
    }

    fn text(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok()
    }
}

struct World3dSnapshotApplyCursor {
    lease: World3dSnapshotLease,
    page: u16,
    item: u8,
    staged_orbit: Option<OrbitController>,
    draw_started: bool,
    draw_permit: Option<World3dSnapshotDrawPermit>,
    faulted: bool,
    status: [Option<World3dPreparedStatus>; 2],
}

impl World3dSnapshotApplyCursor {
    fn new(lease: World3dSnapshotLease) -> Self {
        Self { lease, page: 0, item: 0, staged_orbit: None, draw_started: false, draw_permit: None, faulted: false, status: [None; 2] }
    }

    fn close_step(&mut self) -> bool {
        if self.staged_orbit.take().is_some() {
            return false;
        }
        self.page = self.lease.page_count;
        self.item = 0;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSnapshotApplyStep {
    Idle,
    Pending,
    Complete,
    Stale,
    Fault,
}

pub fn step_world3d_snapshot(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> World3dSnapshotApplyStep {
    if context.should_yield() {
        return World3dSnapshotApplyStep::Pending;
    }
    let Some(mut cursor) = state.snapshot_apply.take() else {
        return World3dSnapshotApplyStep::Idle;
    };
    if cursor.faulted {
        state.snapshot_apply = Some(cursor);
        return World3dSnapshotApplyStep::Fault;
    }
    if cursor.page == cursor.lease.page_count {
        if cursor.draw_started && world3d_draw_rebuild_seal(state).is_err() {
            state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
            cursor.faulted = true;
            state.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Fault;
        }
        if let Some(orbit) = cursor.staged_orbit.take() {
            state.orbit = orbit;
        }
        state.interaction_revision = cursor.lease.revision;
        state.snapshot_lease = Some(cursor.lease);
        state.prepared_status = cursor.status;
        state.snapshot_fault = None;
        context.consume_fuel(1);
        return World3dSnapshotApplyStep::Complete;
    }
    let resolved = world3d_snapshot_with_page(cursor.lease, cursor.page, |page| {
        let item_count = page.item_count();
        let item = page.item(usize::from(cursor.item)).copied();
        let id = item.and_then(|item| item.strings[0]).and_then(|span| page.string(span)).map(str::to_owned);
        let status = item.and_then(|item| {
            let text = item.strings[1].and_then(|span| page.string(span))?;
            World3dPreparedStatus::from_parts(text, [item.numbers[0], item.numbers[1], item.numbers[2]], item.indexes[0], item.indexes[1])
        });
        (page.kind(), item_count, item, id, status)
    });
    let (kind, item_count, item, id, status) = match resolved {
        Ok(resolved) => resolved,
        Err(fault) => {
            state.snapshot_fault = Some(fault);
            cursor.faulted = true;
            state.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Stale;
        }
    };
    if usize::from(cursor.item) == item_count {
        cursor.page += 1;
        cursor.item = 0;
        state.snapshot_apply = Some(cursor);
        context.consume_fuel(1);
        return World3dSnapshotApplyStep::Pending;
    }
    let Some(item) = item else {
        state.snapshot_fault = Some(World3dSnapshotFault::PageState);
        cursor.faulted = true;
        state.snapshot_apply = Some(cursor);
        return World3dSnapshotApplyStep::Fault;
    };
    match kind {
        World3dSnapshotPageKind::Mesh if item.flags == 30 && item.index_len >= 2 => {
            let Some(mesh_key) = id.as_deref() else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let instance_count = item.indexes[0];
            let byte_count = item.indexes[1].checked_add(std::mem::size_of::<SceneDraw3d>() as u32).and_then(|bytes| bytes.checked_add(instance_count.checked_mul(std::mem::size_of::<Instance3d>() as u32)?));
            let Some(byte_count) = byte_count else {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let draw_bytes = item.indexes[1];
            let permit = match world3d_snapshot_claim_draw_permit(cursor.lease, 1, instance_count, draw_bytes) {
                Ok(permit) => permit,
                Err(fault) => {
                    state.snapshot_fault = Some(fault);
                    cursor.faulted = true;
                    state.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                }
            };
            begin_world_placeholder_mesh(state, mesh_key, WorldPlaceholderKind::Box);
            let descriptor = WorldDrawRebuildDescriptor { generation: state.draw_generation.wrapping_add(1), revision: state.interaction_revision, draw_count: 1, instance_count, byte_count };
            let admitted = begin_world3d_draw_rebuild(state, descriptor).and_then(|()| world3d_draw_rebuild_admit_draw(state, mesh_key, 0, u16::try_from(instance_count).map_err(|_| WorldDynamicFault::InstanceCapacity)?));
            if admitted.is_err() {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
            cursor.draw_permit = Some(permit);
            cursor.draw_started = true;
        }
        World3dSnapshotPageKind::Instance if item.number_len >= 14 && cursor.draw_started => {
            let Some(id) = id.as_deref() else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let position = [item.numbers[0] as f32, item.numbers[1] as f32, item.numbers[2] as f32];
            let rotation = [item.numbers[3] as f32, item.numbers[4] as f32, item.numbers[5] as f32, item.numbers[6] as f32];
            let scale = [item.numbers[7] as f32, item.numbers[8] as f32, item.numbers[9] as f32];
            let color = [item.numbers[10] as f32, item.numbers[11] as f32, item.numbers[12] as f32, item.numbers[13] as f32];
            let model = Instance3d::model_from_trs(position, rotation, scale);
            if world3d_draw_rebuild_admit_instance(state, 0, id, model, color, false, false).is_err() {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        World3dSnapshotPageKind::Camera if item.number_len >= 10 => {
            cursor.staged_orbit = Some(OrbitController::from_camera(&Camera3d {
                position: Vec3::new(item.numbers[0] as f32, item.numbers[1] as f32, item.numbers[2] as f32),
                target: Vec3::new(item.numbers[3] as f32, item.numbers[4] as f32, item.numbers[5] as f32),
                up: Vec3::new(item.numbers[6] as f32, item.numbers[7] as f32, item.numbers[8] as f32),
                fov_y: item.numbers[9] as f32 * std::f32::consts::PI / 180.0,
                near: 0.1,
                far: 1000.0,
            }));
        }
        World3dSnapshotPageKind::Camera => {
            state.snapshot_fault = Some(World3dSnapshotFault::PageState);
            cursor.faulted = true;
            state.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Fault;
        }
        World3dSnapshotPageKind::Status if (10..=14).contains(&item.flags) && item.number_len >= 14 && cursor.draw_started => {
            let Some(id) = id.as_deref() else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let vector = [item.numbers[0], item.numbers[1], item.numbers[2]];
            let magnitude = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt().max(item.numbers[3].abs()).max(1.0e-6) as f32;
            let position = [item.numbers[11] as f32, item.numbers[12] as f32, item.numbers[13] as f32];
            let color = match item.flags {
                10 => [0.957, 0.447, 0.714, 1.0],
                11 => [0.918, 0.702, 0.031, 1.0],
                12 => [0.984, 0.443, 0.522, 1.0],
                13 => [0.133, 0.827, 0.933, 1.0],
                _ => [0.655, 0.545, 0.98, 1.0],
            };
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [0.025, 0.025, magnitude.min(1.0)]);
            if world3d_draw_rebuild_admit_instance(state, 0, id, model, color, false, false).is_err() {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        World3dSnapshotPageKind::Status if item.flags == 20 && item.number_len >= 3 && item.index_len >= 2 => {
            let Some(status) = status else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            cursor.status[usize::from(id.as_deref() == Some("de"))] = Some(status);
        }
        _ => {}
    }
    cursor.item += 1;
    state.snapshot_apply = Some(cursor);
    context.consume_fuel(1);
    World3dSnapshotApplyStep::Pending
}

pub fn close_world3d_snapshot_apply_step(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    let Some(cursor) = state.snapshot_apply.as_mut() else {
        return true;
    };
    let complete = cursor.close_step();
    context.consume_fuel(1);
    if complete {
        state.snapshot_apply = None;
    }
    complete
}

pub fn sync_world3d_state(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    state.bounds = bounds;
    let Some(world) = scene.world_3d.as_ref() else {
        state.snapshot_fault = Some(World3dSnapshotFault::Unavailable);
        return;
    };
    let Some(lease) = world.snapshot else {
        state.snapshot_fault = Some(World3dSnapshotFault::Unavailable);
        return;
    };
    if state.snapshot_lease == Some(lease) || state.snapshot_apply.as_ref().is_some_and(|cursor| cursor.lease == lease) {
        return;
    }
    if state.snapshot_apply.is_some() {
        state.snapshot_fault = Some(World3dSnapshotFault::Closing);
        return;
    }
    state.snapshot_apply = Some(World3dSnapshotApplyCursor::new(lease));
    state.snapshot_fault = None;
}

#[cfg(test)]
fn sync_world3d_state_legacy(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    state.bounds = bounds;
    let Some(world) = &scene.world_3d else {
        if state.scene_camera_json.is_some() || state.scene_meshes_json.is_some() || state.scene_instances_json.is_some() {
            state.interaction_revision = state.interaction_revision.wrapping_add(1);
        }
        if state.draws.clear_into_quarantine().is_err() {
            mark_world_dynamic_fault(state, WorldDynamicFault::QuarantineCapacity);
            return;
        }
        state.scene_camera_json = None;
        state.scene_meshes_json = None;
        state.scene_instances_json = None;
        state.scene_selection_json = None;
        state.scene_vortices_json = None;
        state.scene_attractions_json = None;
        state.scene_target_volumes_json = None;
        state.scene_references_json = None;
        state.scene_brush_preview_json = None;
        state.scene_interaction_json = None;
        state.scene_engagement_preview_json = None;
        state.scene_lod_json = None;
        state.scene_chunking_json = None;
        state.scene_environment_json = None;
        state.environment = WorldEnvironmentRecord::default();
        state.scene_terrain_json = None;
        state.terrain_style = None;
        state.bound_domain_id = None;
        state.bound_domain_granularity_id = None;
        return;
    };
    let unchanged = state.scene_camera_json.as_deref() == Some(world.camera_json.as_str())
        && state.scene_meshes_json.as_deref() == Some(world.meshes_json.as_str())
        && state.scene_instances_json.as_deref() == Some(world.instances_json.as_str())
        && state.scene_selection_json.as_deref() == Some(world.selection_json.as_str())
        && state.scene_vortices_json.as_deref() == world.vortices_json.as_deref()
        && state.scene_attractions_json.as_deref() == world.attractions_json.as_deref()
        && state.scene_target_volumes_json.as_deref() == world.target_volumes_json.as_deref()
        && state.scene_references_json.as_deref() == world.references_json.as_deref()
        && state.scene_brush_preview_json.as_deref() == world.brush_preview_json.as_deref()
        && state.scene_interaction_json.as_deref() == world.interaction_json.as_deref()
        && state.scene_engagement_preview_json.as_deref() == world.engagement_preview_json.as_deref()
        && state.scene_lod_json.as_deref() == world.lod_json.as_deref()
        && state.scene_chunking_json.as_deref() == world.chunking_json.as_deref()
        && state.scene_environment_json.as_deref() == world.environment_json.as_deref()
        && state.scene_terrain_json.as_deref() == world.terrain_json.as_deref()
        && state.bound_domain_id.as_deref() == world.domain_id.as_deref()
        && state.bound_domain_granularity_id.as_deref() == world.domain_granularity_id.as_deref();
    if unchanged {
        return;
    }
    state.interaction_revision = state.interaction_revision.wrapping_add(1);
    let geometry_unchanged = state.scene_camera_json.as_deref() == Some(world.camera_json.as_str())
        && state.scene_meshes_json.as_deref() == Some(world.meshes_json.as_str())
        && state.scene_instances_json.as_deref() == Some(world.instances_json.as_str())
        && state.scene_attractions_json.as_deref() == world.attractions_json.as_deref()
        && state.scene_target_volumes_json.as_deref() == world.target_volumes_json.as_deref()
        && state.scene_references_json.as_deref() == world.references_json.as_deref()
        && state.scene_brush_preview_json.as_deref() == world.brush_preview_json.as_deref()
        && state.scene_interaction_json.as_deref() == world.interaction_json.as_deref()
        && state.scene_engagement_preview_json.as_deref() == world.engagement_preview_json.as_deref()
        && state.scene_lod_json.as_deref() == world.lod_json.as_deref()
        && state.scene_chunking_json.as_deref() == world.chunking_json.as_deref()
        && state.scene_environment_json.as_deref() == world.environment_json.as_deref()
        && state.scene_terrain_json.as_deref() == world.terrain_json.as_deref()
        && state.bound_domain_id.as_deref() == world.domain_id.as_deref()
        && state.bound_domain_granularity_id.as_deref() == world.domain_granularity_id.as_deref();
    if geometry_unchanged {
        let selection_changed = state.scene_selection_json.as_deref() != Some(world.selection_json.as_str());
        let vortices_changed = state.scene_vortices_json.as_deref() != world.vortices_json.as_deref();
        if selection_changed {
            state.scene_selection_json = Some(world.selection_json.clone());
            let selection: WorldSelectionRecord = serde_json::from_str(&world.selection_json).unwrap_or_default();
            state.selection_method = selection.method.unwrap_or_else(|| "rectangle".into());
            state.local_hover_id = selection.hovered_id;
            state.selected_ids = selection.ids.clone().unwrap_or_default();
            state.component_ids = selection.component_ids.clone().unwrap_or_default();
            state.granularity = selection.granularity.or(selection.selection_mode).unwrap_or_else(|| "object".into());
            if state.granularity == "object" {
                state.granularity = "mesh".into();
            }
            state.interaction_mode = selection.interaction_mode.unwrap_or_else(|| "model".into());
            state.gumball_target = selection.gumball_target.map(|target| [target[0] as f32, target[1] as f32, target[2] as f32]);
            apply_hovered_component_from_selection(state, &world.selection_json);
            state.show_edges = selection.show_edges.unwrap_or(true);
            state.selection_targets = selection.targets.unwrap_or_default();
            state.active_object_id = selection.active_object_id;
            state.transform_mode = selection.transform_mode.unwrap_or_else(|| "translate".into());
        }
        if vortices_changed {
            state.scene_vortices_json = world.vortices_json.clone();
            state.vortices = world.vortices_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
        }
        if selection_changed || vortices_changed {
            return;
        }
    }
    let camera_changed = state.scene_camera_json.as_deref() != Some(world.camera_json.as_str());
    state.scene_camera_json = Some(world.camera_json.clone());
    state.scene_meshes_json = Some(world.meshes_json.clone());
    state.scene_instances_json = Some(world.instances_json.clone());
    state.scene_selection_json = Some(world.selection_json.clone());
    state.scene_vortices_json = world.vortices_json.clone();
    state.scene_attractions_json = world.attractions_json.clone();
    state.scene_target_volumes_json = world.target_volumes_json.clone();
    state.scene_references_json = world.references_json.clone();
    state.scene_brush_preview_json = world.brush_preview_json.clone();
    state.scene_interaction_json = world.interaction_json.clone();
    state.scene_engagement_preview_json = world.engagement_preview_json.clone();
    state.scene_lod_json = world.lod_json.clone();
    state.scene_chunking_json = world.chunking_json.clone();
    state.scene_environment_json = world.environment_json.clone();
    state.scene_terrain_json = world.terrain_json.clone();
    state.bound_domain_id = world.domain_id.clone();
    state.bound_domain_granularity_id = world.domain_granularity_id.clone();
    state.lod = world.lod_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_else(default_lod_record);
    state.chunking = world.chunking_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    state.environment = world.environment_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.terrain_style = world.terrain_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    state.vortices = world.vortices_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.attractions = world.attractions_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.target_volumes = world.target_volumes_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.references = world.references_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.brush_preview = world.brush_preview_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    let interaction: WorldInteractionRecord = world.interaction_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.active_utility = interaction.active_utility.unwrap_or_else(|| "select".into());
    state.hovered_vortex_id = interaction.hovered_vortex_full_id;
    for reference in &state.references {
        if reference.hidden.unwrap_or(false) {
            continue;
        }
        if let Some(url) = reference.url.as_deref() {
            if !state.reference_pixels.contains_key(url) {
                state.pending_image_urls.insert(url.to_string());
            }
        }
    }
    let camera: WorldCameraRecord = serde_json::from_str(&world.camera_json).unwrap_or_default();
    if camera_changed {
        if let (Some(position), Some(target)) = (camera.position, camera.target) {
            state.orbit = OrbitController::from_camera(&Camera3d {
                position: vec3_from_f64(position),
                target: vec3_from_f64(target),
                up: camera.up.map(vec3_from_f64).unwrap_or(Vec3::new(0.0, 0.0, 1.0)),
                fov_y: camera.fov.unwrap_or(45.0) as f32 * std::f32::consts::PI / 180.0,
                near: 0.1,
                far: 1000.0,
            });
        } else if camera.x.is_some() || camera.y.is_some() || camera.z.is_some() {
            state.orbit = OrbitController::from_camera(&Camera3d {
                position: Vec3::new(camera.x.unwrap_or(4.0) as f32, camera.y.unwrap_or(-4.0) as f32, camera.z.unwrap_or(3.0) as f32),
                target: Vec3::ZERO,
                up: Vec3::new(0.0, 0.0, 1.0),
                fov_y: camera.fov.unwrap_or(45.0) as f32 * std::f32::consts::PI / 180.0,
                near: 0.1,
                far: 1000.0,
            });
        }
    }
    let meshes: Vec<WorldMeshRecord> = serde_json::from_str(&world.meshes_json).unwrap_or_default();
    state.mesh_lod_catalog.clear();
    state.mesh_url_fallback.clear();
    for mesh in meshes {
        if let Some(lods) = mesh.lods.filter(|entries| !entries.is_empty()) {
            state.mesh_lod_catalog.insert(mesh.id.clone(), lods);
            if let Some(url) = mesh.url.clone() {
                state.mesh_url_fallback.insert(mesh.id.clone(), url);
            }
            queue_lod_mesh_fetch(state, &mesh.id, scene_lod(state));
        } else if let Some(url) = mesh.url {
            state.mesh_url_fallback.insert(mesh.id.clone(), url.clone());
            state.pending_glb_urls.insert(url);
        }
    }
    state.parsed_instances = serde_json::from_str(&world.instances_json).unwrap_or_default();
    let selection: WorldSelectionRecord = serde_json::from_str(&world.selection_json).unwrap_or_default();
    state.selection_method = selection.method.unwrap_or_else(|| "rectangle".into());
    state.local_hover_id = selection.hovered_id;
    state.selected_ids = selection.ids.clone().unwrap_or_default();
    state.component_ids = selection.component_ids.clone().unwrap_or_default();
    state.granularity = selection.granularity.or(selection.selection_mode).unwrap_or_else(|| "object".into());
    if state.granularity == "object" {
        state.granularity = "mesh".into();
    }
    state.interaction_mode = selection.interaction_mode.unwrap_or_else(|| "model".into());
    state.gumball_target = selection.gumball_target.map(|target| [target[0] as f32, target[1] as f32, target[2] as f32]);
    apply_hovered_component_from_selection(state, &world.selection_json);
    state.show_edges = selection.show_edges.unwrap_or(true);
    state.selection_targets = selection.targets.unwrap_or_default();
    state.active_object_id = selection.active_object_id;
    state.transform_mode = selection.transform_mode.unwrap_or_else(|| "translate".into());
    let current_lod = scene_lod(state);
    rebuild_instance_draws_legacy(state, current_lod);
    state.resolved_lod_pick = Some(current_lod);
}

fn apply_runtime_draw_flags(state: &mut World3dState) {
    let granularity = state.granularity.clone();
    let component_ids: HashSet<String> = state.component_ids.iter().cloned().collect();
    let local_hover_id = state.local_hover_id.clone();
    let hovered_component_object_id = state.hovered_component_object_id.clone();
    let selected_ids: HashSet<String> = state.selected_ids.iter().cloned().collect();
    let mut object_index_map = HashMap::new();
    let mut index = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            object_index_map.insert(instance.id.clone(), index);
            index += 1;
        }
    }
    let component_mode = component_mode_active(state);
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            let mesh_selected = granularity == "mesh" && object_index_map.get(&instance.id).is_some_and(|object_index| component_ids.contains(&object_index.to_string()));
            let local_hovered = if component_mode { false } else { local_hover_id.as_deref() == Some(instance.id.as_str()) || hovered_component_object_id.as_deref() == Some(instance.id.as_str()) };
            let local_selected = selected_ids.contains(&instance.id) || mesh_selected;
            // 🎨️ Selection/hover flags must follow the live selection snapshot — OR-ing with the
            // instancesJson bits left deselected meshes painted selected until a later hover rebuild.
            instance.hovered = local_hovered;
            instance.selected = local_selected;
        }
    }
}

// 🖥️ Native/browser rendering-host entry point only (called from `📺️renderer`'s engine, never from
// wasip2 plugin guest logic — confirmed by grepping every `render_world_3d` call site in the repo).
// `WidgetContext` bundles the font/icon atlases, which are genuinely GPU-adjacent (real `wgpu`
// crate reachable through `wgpu-engine`), so this one function stays excluded from
// `wasm32-wasip2`: `target_arch = "wasm32"` is TRUE for wasip2 too, hence `not(target_env = "p2")`.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn render_world_3d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, state: &mut World3dState, gpu: &mut World3dBuildContext) {
    use ui_wgpu::wgpu::widgets::{draw_text, gizmo as gpu_gizmo};
    let theme = ctx.theme;
    step_world_placeholder_mesh(state);
    state.pick_bounds = ctx.pick_clip.unwrap_or(bounds);
    sync_world3d_state(state, scene, bounds);
    apply_terrain_style_if_changed(state, gpu);
    let current_lod = scene_lod(state);
    let lod_changed = state.resolved_lod_pick.is_none_or(|previous| (previous - current_lod).abs() > WORLD_LOD_EPSILON);
    if lod_changed {
        state.resolved_lod_pick = Some(current_lod);
    }
    apply_runtime_draw_flags(state);
    let inner = bounds;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], environment_clear_color(&state.environment, theme.canvas_clear));
    let camera = state.orbit.to_camera();
    let light_dir = environment_light_dir(&state.environment);
    let terrain_draws = sync_terrain(state, gpu, &camera);
    update_visible_chunks(state, camera.position);
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let planes = frustum_planes(view_proj);
    let mut culled_draws = Vec::new();
    let mut culled_count = 0u32;
    let mut needed_mesh_keys = HashSet::new();
    let missing_mesh_urls: HashSet<String> = state.draws.iter().filter(|draw| !state.meshes.contains_key(&draw.mesh_key)).filter_map(|draw| state.mesh_source_urls.get(&draw.mesh_key).cloned()).collect();
    for url in missing_mesh_urls {
        if reserve_world3d_asset_request(state, WorldAssetRequestKind::Glb, &url).is_err() {
            mark_world_dynamic_fault(state, WorldDynamicFault::RegistryCapacity);
        }
    }
    for (draw_index, draw) in state.draws.iter().enumerate() {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let Ok((mesh_min, mesh_max)) = mesh.aabb() else { continue };
        let mesh_version = *state.mesh_versions.get(&draw.mesh_key).unwrap_or(&0);
        let instances: Vec<Instance3d> = draw
            .instances
            .iter()
            .enumerate()
            .filter_map(|(instance_index, instance)| {
                let mut instance = instance.clone();
                instance.model = retained_gumball_preview_model(state, draw_index, instance_index, instance.model);
                let position = state.instance_positions.get(&instance.id).copied().unwrap_or([0.0, 0.0, 0.0]);
                if !instance_chunk_visible(state, position) {
                    return None;
                }
                let (min, max) = transform_aabb(instance.model, mesh_min, mesh_max);
                let visible = aabb_intersects_frustum(&planes, min, max);
                if !visible {
                    culled_count += 1;
                }
                visible.then_some(instance)
            })
            .collect();
        if !instances.is_empty() {
            needed_mesh_keys.insert(draw.mesh_key.clone());
            gpu.ensure_mesh(&draw.mesh_key, mesh_version, mesh);
            culled_draws.push(SceneDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, instances });
        }
    }
    sync_mesh_pool(state, &needed_mesh_keys, gpu);
    let mut line_vertices = Vec::new();
    if state.lod.show_grid {
        let datum = state.lod.grid_datum.unwrap_or([0.0, 0.0, 0.0]);
        let anchor = grid_placement_anchor(camera.target, datum);
        append_lod_grid_lines(&mut line_vertices, current_lod, state.lod.grid_factor, anchor, [theme.text_element.r, theme.text_element.g, theme.text_element.b, theme.text_element.a]);
    }
    append_component_overlays(state, &mut line_vertices);
    for attraction in &state.attractions {
        let Some(from) = attraction.from else { continue };
        let Some(to) = attraction.to else { continue };
        let color = parse_color(attraction.color.as_deref().unwrap_or("#60a5fa"));
        line_vertices.push(LineVertex3d { position: [from[0] as f32, from[1] as f32, from[2] as f32], color });
        line_vertices.push(LineVertex3d { position: [to[0] as f32, to[1] as f32, to[2] as f32], color });
    }
    for volume in &state.target_volumes {
        append_box_wireframe(&mut line_vertices, volume.origin.unwrap_or([0.0, 0.0, 0.0]), volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), volume.scale.unwrap_or([1.0, 1.0, 1.0]), parse_color(volume.color.as_deref().unwrap_or("#f472b6")));
    }
    let mut extra_draws = Vec::new();
    append_vortex_arrow_draws(state, gpu, &mut extra_draws);
    let vertex_instances = append_component_vertex_spheres(state);
    if !vertex_instances.is_empty() {
        let mesh_version = *state.mesh_versions.get(VERTEX_MARKER_MESH).unwrap_or(&0);
        if let Some(mesh) = state.meshes.get(VERTEX_MARKER_MESH) {
            gpu.ensure_mesh(VERTEX_MARKER_MESH, mesh_version, *mesh);
        }
        extra_draws.push(SceneDraw3d { mesh_key: VERTEX_MARKER_MESH.into(), mesh_version, instances: vertex_instances });
    }
    let mut translucent_draws = Vec::new();
    append_component_face_translucent_overlays(state, gpu, &mut translucent_draws);
    if let Some(preview) = state.brush_preview.clone() {
        // 👻️ Mirrors `BrushPreviewGhost`: renders whenever `origin` is present, regardless of
        // `meshUrl` — a translucent unit box is the fallback ghost when there's no mesh URL (or
        // its GLB hasn't resolved into `state.meshes` yet), not "nothing at all".
        if let Some(origin) = preview.origin {
            let mesh_id = brush_preview_mesh_id(preview.mesh_url.as_deref());
            if !state.meshes.contains_key(&mesh_id) {
                begin_world_placeholder_mesh(state, &mesh_id, WorldPlaceholderKind::Box);
            }
            let rotation = preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            let scale = preview_scale(preview.scale.as_ref());
            let mesh_version = *state.mesh_versions.get(&mesh_id).unwrap_or(&0);
            if let Some(mesh) = state.meshes.get(&mesh_id) {
                gpu.ensure_mesh(&mesh_id, mesh_version, *mesh);
            }
            translucent_draws.push(SceneDraw3d {
                mesh_key: mesh_id,
                mesh_version,
                instances: vec![Instance3d {
                    id: "brush-preview".into(),
                    model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [rotation[0] as f32, rotation[1] as f32, rotation[2] as f32, rotation[3] as f32], scale),
                    color: {
                        let mut color = parse_color(preview.color.as_deref().unwrap_or("#59bfff"));
                        color[3] = 0.45;
                        color
                    },
                    selected: false,
                    hovered: false,
                }],
            });
        }
    }
    let mut textured_draws = Vec::new();
    if !state.meshes.contains_key("reference-plane") {
        begin_world_placeholder_mesh(state, "reference-plane", WorldPlaceholderKind::Plane);
    }
    let mut textured_instances = Vec::new();
    for reference in &state.references {
        if reference.hidden.unwrap_or(false) {
            continue;
        }
        let Some(url) = reference.url.as_deref() else {
            continue;
        };
        let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let width = reference.width_world.unwrap_or(1.0) as f32;
        let aspect = reference_image_aspect(state, url);
        let height = width / aspect.max(0.01);
        textured_instances.push(TexturedInstance3d { texture_key: url.to_string(), model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [0.0, 0.0, 0.0, 1.0], [width, height, 1.0]), tint: [1.0, 1.0, 1.0, 0.85] });
        if let Some((pixel_w, pixel_h, pixels)) = state.reference_pixels.get(url) {
            gpu.ensure_world_plane_texture(url, pixels, *pixel_w, *pixel_h);
        }
    }
    if !textured_instances.is_empty() {
        textured_draws.push(TexturedDraw3d { instances: textured_instances });
    }
    if !state.meshes.contains_key("gumball-plane") {
        begin_world_placeholder_mesh(state, "gumball-plane", WorldPlaceholderKind::Plane);
    }
    if !state.selected_ids.is_empty() && state.active_utility == "select" {
        append_gumball_geometry(&mut line_vertices, &mut translucent_draws, state, &camera, &state.meshes, &state.mesh_versions);
    }
    culled_draws.extend(extra_draws);
    culled_draws.extend(terrain_draws);
    ctx.draw.push_scene_pass(ScenePass3d {
        viewport: [inner.x, inner.y, inner.w, inner.h],
        view_proj: view_proj.to_cols_array(),
        light_dir,
        draws: culled_draws,
        line_draws: if line_vertices.is_empty() { Vec::new() } else { vec![LineDraw3d { vertices: line_vertices }] },
        translucent_draws,
        textured_draws,
        ..Default::default()
    });
    if state.marquee_active && state.marquee_points.len() >= 2 {
        let crossing = marquee_is_crossing_from_path(&state.marquee_points, state.selection_method == "lasso");
        paint_selection_marquee(ctx.draw, theme, crossing, state.selection_method == "lasso", &state.marquee_points, false);
    }
    gpu_gizmo::paint_orbit_view_gizmo(ctx, &camera, inner, state.gizmo_hovered_tip);
    for (index, status) in state.prepared_status.iter().flatten().enumerate() {
        if let Some(text) = status.text() {
            let y = inner.y + 20.0 + index as f32 * (theme.font_size_small + 6.0);
            draw_text(ctx, text, inner.x + 12.0, y, theme.font_size_small, theme.text);
        }
    }
    if let Some(status) = state.prepared_status[0] {
        let total = f64::from(status.total).max(1.0);
        let ratio = (status.progress[2].min(total) / total) as f32;
        let tone = if status.state == 6 { theme.text } else { theme.text_element };
        ctx.draw.push_solid([inner.x + 12.0, inner.y + 54.0, 160.0 * ratio, 3.0], tone);
    }
    if scene.world_3d.is_none() {
        draw_text(ctx, "world-3d (empty)", inner.x + 12.0, inner.y + 20.0, theme.font_size_small, theme.text_muted);
    }
    if world3d_cursor_work_pending(state) {
        gpu.request_cursor_wake();
    }
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(state.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None });
}

//#region 🧭️WorldOrbitViewGizmo
/** 🧭️ The pure placement/tip-geometry/paint logic relocated to `ui_wgpu::wgpu::widgets::gizmo` (see
`.🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`) — this
region now only keeps the `World3dState`-specific hover-state plumbing (app config, not paint), calling
through to `gizmo::orbit_view_gizmo_placement`/`gizmo::orbit_view_gizmo_tips`/`gizmo::orbit_view_gizmo_hit_test`. */
fn update_world_orbit_view_gizmo_hover(state: &mut World3dState, x: f32, y: f32, inner: Rect) {
    let (margin_x, margin_y) = gizmo::orbit_view_gizmo_placement(inner);
    let origin_x = inner.x + inner.w - margin_x;
    let origin_y = inner.y + inner.h - margin_y;
    let zone_radius = 56.0;
    if ((x - origin_x).powi(2) + (y - origin_y).powi(2)).sqrt() > zone_radius {
        state.gizmo_hovered_tip = None;
        return;
    }
    let camera = state.orbit.to_camera();
    let tips = gizmo::orbit_view_gizmo_tips(&camera, inner);
    state.gizmo_hovered_tip = gizmo::orbit_view_gizmo_hit_test(x, y, &tips);
}
//#endregion 🧭️WorldOrbitViewGizmo

pub fn world3d_hit_target(scene: &UiComponentSceneNode, bounds: Rect) -> HitTarget<ActionDescriptor> {
    HitTarget { rect: bounds, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None }
}

#[cfg(test)]
#[cfg(test)]
fn handle_world3d_pointer_move(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        state.gizmo_hovered_tip = None;
        return None;
    }
    if down && button == 0 {
        if state.marquee_active {
            state.marquee_points.push([x, y]);
            update_marquee_preview(state, render_pick_viewport(state));
        } else if state.paint_stroke_active && state.interaction_mode == "paint" {
            if let Some((object_id, u, v)) = pick_paint_hit(state, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: state.controller_id.clone(),
                    action: "paintAt".into(),
                    args: action_args(json!({
                        "surfaceId": state.surface_id,
                        "objectId": object_id,
                        "u": u,
                        "v": v,
                    })),
                });
            }
        } else if button == 2 || button == 1 {
            return None;
        }
    }
    if !down {
        update_world_orbit_view_gizmo_hover(state, x, y, inner);
        return pick_hover_action(state, x, y, inner);
    }
    if button == 2 {
        return None;
    }
    None
}

#[cfg(test)]
fn handle_world3d_paint_actions(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16) -> Vec<ActionDescriptor> {
    if state.interaction_mode != "paint" || button != 0 {
        return Vec::new();
    }
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        return Vec::new();
    }
    if down && state.paint_stroke_active {
        let Some((object_id, u, v)) = pick_paint_hit(state, x, y, inner) else {
            return Vec::new();
        };
        return vec![ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "paintAt".into(),
            args: action_args(json!({
                "surfaceId": state.surface_id,
                "objectId": object_id,
                "u": u,
                "v": v,
            })),
        }];
    }
    Vec::new()
}

#[cfg(test)]
#[cfg(test)]
fn handle_world3d_pointer_button(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        return None;
    }
    let shift = modifiers.shift;
    let ctrl = modifiers.ctrl;
    if down {
        if button == 0 {
            if state.active_utility == "surfaceBrush" {
                return None;
            }
            if state.interaction_mode == "paint" {
                state.paint_stroke_active = true;
                return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "paintStrokeBegin".into(), args: action_args(json!({ "surfaceId": state.surface_id })) });
            }
            if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
                if let Some(full_id) = pick_vortex_at(state, x, y, inner) {
                    let merge = if shift {
                        "add"
                    } else if ctrl {
                        "toggle"
                    } else {
                        "replace"
                    };
                    return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldVortexSelect".into(), args: action_args(json!({ "surfaceId": state.surface_id, "fullId": full_id, "merge": merge })) });
                }
            } else if state.active_utility == "select" {
                if !state.selected_ids.is_empty() {
                    if let Some(handle) = pick_gumball_handle_at(state, x, y, inner) {
                        start_gumball_drag(state, handle, x, y, inner);
                        return None;
                    }
                }
                if state.gumball_handle.is_none() && !component_mode_active(state) {
                    state.press_object_id = pick_instance_at(state, x, y, inner);
                } else {
                    state.press_object_id = None;
                }
            }
            state.marquee_active = true;
            state.marquee_points = vec![[x, y]];
            return None;
        }
        if button == 2 {
            state.marquee_active = false;
            state.marquee_points.clear();
            state.right_press_point = Some([x, y]);
        }
        return None;
    }
    if button == 0 && state.marquee_active {
        state.marquee_active = false;
        if is_click_gesture(state, x, y) {
            state.marquee_points.clear();
            state.marquee_preview_ids.clear();
            state.press_object_id = None;
            state.drag_object_id = None;
            state.drag_last_position = None;
            if state.active_utility == "surfaceBrush" {
                if let Some((object_id, position, normal)) = pick_surface_at(state, x, y, inner) {
                    return Some(ActionDescriptor {
                        controller_id: state.controller_id.clone(),
                        action: "worldSurfacePlace".into(),
                        args: action_args(json!({
                            "surfaceId": state.surface_id,
                            "pane": state.surface_id,
                            "objectId": object_id,
                            "position": position,
                            "normal": normal,
                        })),
                    });
                }
                return None;
            }
            if state.gumball_handle.is_none() {
                return pick_select_action(state, x, y, inner, shift, ctrl);
            }
        } else {
            return marquee_select_action(state, render_pick_viewport(state), shift, ctrl);
        }
    }
    if button == 0 && state.interaction_mode == "paint" && state.paint_stroke_active {
        state.paint_stroke_active = false;
        let mut actions = Vec::new();
        actions.push(ActionDescriptor { controller_id: state.controller_id.clone(), action: "paintStrokeEnd".into(), args: action_args(json!({ "surfaceId": state.surface_id })) });
        return actions.first().cloned();
    }
    if button == 0 {
        if state.active_utility == "surfaceBrush" {
            if let Some((object_id, position, normal)) = pick_surface_at(state, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: state.controller_id.clone(),
                    action: "worldSurfacePlace".into(),
                    args: action_args(json!({
                        "surfaceId": state.surface_id,
                        "pane": state.surface_id,
                        "objectId": object_id,
                        "position": position,
                        "normal": normal,
                    })),
                });
            }
            return None;
        }
        if let Some(action) = gumball_commit_action(state) {
            state.gumball_handle = None;
            reset_gumball_preview(state);
            return Some(action);
        }
        state.gumball_handle = None;
        reset_gumball_preview(state);
        state.press_object_id = None;
        if let Some(object_id) = state.drag_object_id.take() {
            if let Some(position) = state.drag_last_position {
                if !is_click_gesture(state, x, y) {
                    return Some(ActionDescriptor {
                        controller_id: state.controller_id.clone(),
                        action: "worldRelocate".into(),
                        args: action_args(json!({
                            "surfaceId": state.surface_id,
                            "objectId": object_id,
                            "position": position,
                        })),
                    });
                }
            }
        }
        if state.active_utility == "brush" {
            if let Some(preview) = state.brush_preview.clone() {
                if let (Some(target), Some(kind), Some(index)) = (preview.target_vortex_full_id, preview.object_kind_id, preview.source_vortex_index) {
                    let origin = preview.origin.unwrap_or([0.0, 0.0, 0.0]);
                    return Some(ActionDescriptor {
                        controller_id: state.controller_id.clone(),
                        action: "addBrushObject".into(),
                        args: action_args(json!({
                            "targetVortexFullId": target,
                            "objectKindId": kind,
                            "sourceVortexIndex": index,
                            "origin": origin,
                            "orientation": preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                            "scale": preview.scale,
                        })),
                    });
                }
            }
        }
        return pick_select_action(state, x, y, inner, shift, ctrl);
    }
    if button == 2 {
        // 🖱️ A right-drag (orbit-via-right-button, see `handle_world3d_pointer_drag`) must not
        // also pop a context menu — only a right-*click* (no meaningful movement since press)
        // resolves+dispatches a context-menu target, mirroring the React reference's
        // `onContextMenu` (which only fires on a genuine click, not a drag-then-release).
        let is_click = state
            .right_press_point
            .map(|start| {
                let dx = x - start[0];
                let dy = y - start[1];
                (dx * dx + dy * dy).sqrt() <= CLICK_DRAG_THRESHOLD_PX
            })
            .unwrap_or(true);
        state.right_press_point = None;
        if is_click {
            if let Some((kind, id)) = resolve_world_context_menu_target(state) {
                return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "contextMenuAt".into(), args: action_args(json!({ "surfaceId": state.surface_id, "kind": kind, "id": id })) });
            }
        }
        return Some(orbit_camera_action(state));
    }
    if button == 1 {
        return Some(orbit_camera_action(state));
    }
    None
}

/// 🖱️📋️ Resolves which entity a right-click context menu targets, in the React reference's exact
/// priority order — `resolveWorldContextMenuTarget` in `world-3d-host.tsx`: a hovered vortex wins
/// first, then a hovered mesh component (vertex/edge/face — reported as kind `"object"`, the
/// component's owning instance, matching the React source's own naming), then a hovered reference
/// image plane; `None` (no context menu) if nothing is currently hovered.
fn resolve_world_context_menu_target(state: &World3dState) -> Option<(&'static str, String)> {
    if let Some(vortex_id) = state.hovered_vortex_id.clone() {
        return Some(("vortex", vortex_id));
    }
    if state.hovered_component_mode.is_some() {
        if let Some(object_id) = state.hovered_component_object_id.clone() {
            return Some(("object", object_id));
        }
    }
    if let Some(reference_id) = state.local_hover_id.as_deref().and_then(|hovered| hovered.strip_prefix("reference:")) {
        return Some(("reference", reference_id.to_string()));
    }
    None
}

#[cfg(test)]
fn handle_world3d_pointer_drag(state: &mut World3dState, x: f32, y: f32, dx: f32, dy: f32, button: i16, modifiers: &PointerModifiers) {
    let inner = world_pick_rect(state);
    if button == 0 {
        if state.gumball_handle.is_some() && inner.contains(x, y) {
            gumball_drag_update(state, x, y, inner);
            return;
        }
        if state.drag_object_id.is_none() && state.gumball_handle.is_none() && !component_mode_active(state) && pointer_drag_distance(state, x, y) > CLICK_DRAG_THRESHOLD_PX {
            if let Some(object_id) = state.press_object_id.take().or_else(|| pick_instance_at(state, x, y, inner)) {
                state.drag_object_id = Some(object_id.clone());
                state.marquee_active = false;
                state.marquee_points.clear();
                state.marquee_preview_ids.clear();
                state.drag_last_position = object_world_position(state, &object_id);
                state.drag_object_z = state.drag_last_position.map(|p| p[2]).unwrap_or(0.0);
            }
        }
        if state.drag_object_id.is_some() && inner.contains(x, y) {
            if let Some(position) = ground_plane_pick(state, x, y, inner, state.drag_object_z) {
                state.drag_last_position = Some(position);
                if let Some(object_id) = state.drag_object_id.clone() {
                    update_dragged_instance_position(state, &object_id, position);
                }
            }
            return;
        }
    }
    if button == 1 || (button == 2 && modifiers.shift) {
        state.orbit.pan(-dx, -dy);
    } else if button == 2 && (modifiers.alt || modifiers.meta) {
        state.orbit.orbit(dx, dy);
    }
}

#[cfg(test)]
#[cfg(test)]
fn handle_world3d_wheel(state: &mut World3dState, delta: f32) {
    state.orbit.zoom(delta);
}

fn merge_string_ids(existing: &[String], incoming: &[String], merge: &str) -> Vec<String> {
    match merge {
        // 🕹️ `"additive"` is the framework `MergeMode` wire label (see `world_interaction_definition`);
        // `"add"` is `worldPick`'s own pre-existing, untouched-this-wave vocabulary — both accepted here.
        "add" | "additive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(id.clone());
                }
            }
            merged
        }
        "toggle" | "invertive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id.clone());
                }
            }
            merged
        }
        _ => incoming.to_vec(),
    }
}

//#region 🔖️WorldInteractionDomain
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the OS's own bare-board `world`
/// domain — plain (non-component) 3D-world object picking/hover for a world3d window that binds NO
/// app `InteractionDefinition`. Granularity `"item"` targets are `"{surfaceId}/{objectId}"`, split by
/// [`HierarchyProvider::PathDelimited`] — the cross-surface addressing a shared, multi-board OS domain
/// needs (`"surface"` covers a future whole-surface selection, not emitted by pointer picking today).
///
/// Most world3d apps bind their OWN domain instead (`WindowKindDefinition.interactions` /
/// `.window_kind_interactions(...)`) — e.g. CAD's `"cad"` domain with `HierarchyProvider::Flat`. That
/// binding reaches this file only through `World3dScene.domain_id`/`domain_granularity_id` (set by the
/// app's own `render()`, the only party that knows both its `window_kind_id` and which domain it bound
/// there — mirrors `UiTreeNode.interaction_domain`/`PanelTreeBuilder::interaction_domain`, extended to
/// `Scene` surfaces), captured onto `World3dState.bound_domain_id`/`bound_domain_granularity_id` by
/// `sync_world3d_state`. `resolved_domain_id`/`resolved_domain_granularity_id`/`resolved_item_id` below
/// are what every plain pick/hover emit site actually reads — never `WORLD_INTERACTION_DOMAIN_ID`
/// directly, so a bound app domain and this OS domain never both light up for the same click (the
/// two-selection-universe bug this indirection exists to prevent). A bound domain is inherently
/// single-surface-scoped (one app document per window), so its ids are always bare — no `surfaceId/`
/// prefix, unlike `world`'s own multi-surface `PathDelimited` shape.
///
/// Component-level (vertex/edge/face) picking is a separate, unconverted mechanism (`worldPick`/
/// `setSelection`) — out of this file's scope (see the module's W3c task brief).
pub const WORLD_INTERACTION_DOMAIN_ID: &str = "world";
const WORLD_ITEM_GRANULARITY_ID: &str = "item";
const WORLD_ITEM_PATH_DELIMITER: &str = "/";

pub fn world_interaction_definition() -> InteractionDefinition {
    InteractionDefinition {
        id: WORLD_INTERACTION_DOMAIN_ID.into(),
        label: LocalizedLabel::native("World", "Welt"),
        granularities: vec![
            GranularityDefinition { id: "surface".into(), label: LocalizedLabel::native("Surface", "Fläche"), icon_id: "layers".into() },
            GranularityDefinition { id: WORLD_ITEM_GRANULARITY_ID.into(), label: LocalizedLabel::native("Item", "Objekt"), icon_id: "box".into() },
        ],
        hierarchy: HierarchyProvider::PathDelimited { delimiter: WORLD_ITEM_PATH_DELIMITER.into() },
        hover: HoverSpec::default(),
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🪟️ The interaction domain a plain pick/hover on this surface targets: the app-bound domain from
/// `World3dScene.domain_id`, falling back to the OS's own `world` board domain when unbound.
fn resolved_domain_id(state: &World3dState) -> &str {
    state.bound_domain_id.as_deref().unwrap_or(WORLD_INTERACTION_DOMAIN_ID)
}

/// 🎯️ The granularity id to stamp on a plain pick/hover target — the app-declared granularity when a
/// domain is bound, else the `world` domain's own `"item"` granularity.
fn resolved_domain_granularity_id(state: &World3dState) -> &str {
    state.bound_domain_granularity_id.as_deref().unwrap_or(WORLD_ITEM_GRANULARITY_ID)
}

fn world_item_target_id(surface_id: &str, object_id: &str) -> String {
    format!("{surface_id}{WORLD_ITEM_PATH_DELIMITER}{object_id}")
}

/// 🔤️ Strips this state's own `surfaceId/` prefix from a `world`-domain item target id — the inverse
/// of [`world_item_target_id`]. Targets for a different surface (cross-surface batch, not produced by
/// this file's single-surface pointer handlers) are dropped rather than mis-parsed.
fn world_item_id_for_surface<'a>(state: &World3dState, target_id: &'a str) -> Option<&'a str> {
    target_id.strip_prefix(&state.surface_id).and_then(|rest| rest.strip_prefix(WORLD_ITEM_PATH_DELIMITER))
}

/// 🧮️ Builds the wire target id for a plain pick/hover hit: a bare object id when a real app domain is
/// bound (`HierarchyProvider::Flat`-style, single-surface-scoped), else `world_item_target_id`'s
/// `"surfaceId/id"` `PathDelimited` shape for the shared `world` domain.
fn resolved_item_id(state: &World3dState, object_id: &str) -> String {
    if state.bound_domain_id.is_some() {
        object_id.to_string()
    } else {
        world_item_target_id(&state.surface_id, object_id)
    }
}

/// 🔤️ Inverse of [`resolved_item_id`], for parsing ids back out of an incoming action's targets during
/// optimistic local preview.
fn parse_resolved_item_id<'a>(state: &World3dState, target_id: &'a str) -> Option<&'a str> {
    if state.bound_domain_id.is_some() {
        Some(target_id)
    } else {
        world_item_id_for_surface(state, target_id)
    }
}

fn merge_mode_wire_str(merge: MergeMode) -> &'static str {
    match merge {
        MergeMode::Replace => "replace",
        MergeMode::Additive => "additive",
        MergeMode::Subtractive => "subtractive",
        MergeMode::Invertive => "invertive",
        MergeMode::Range => "range",
    }
}

fn selection_method_wire_str(method: SelectionMethod) -> &'static str {
    match method {
        SelectionMethod::Pick => "pick",
        SelectionMethod::Rectangle => "rectangle",
        SelectionMethod::Lasso => "lasso",
    }
}
//#endregion 🔖️WorldInteractionDomain

/// Applies hover/selection action payloads to renderer-local world state before the plugin round-trip.
pub fn apply_world_action_preview(state: &mut World3dState, action: &ActionDescriptor) {
    let Some(args) = action.args.as_ref() else {
        if action.action == "setHover" {
            state.hovered_component_id = None;
            state.hovered_component_object_id = None;
            state.hovered_component_mode = None;
            state.local_hover_id = None;
        }
        return;
    };
    match action.action.as_str() {
        "setHover" => {
            if args.get("objectId").is_none() {
                state.hovered_component_id = None;
                state.hovered_component_object_id = None;
                state.hovered_component_mode = None;
                state.local_hover_id = None;
            } else {
                let object_id = args.get("objectId").and_then(|value| value.as_str()).map(str::to_string);
                let mode = args.get("mode").and_then(|value| value.as_str()).map(str::to_string);
                let id = args.get("id").and_then(dsl_id_to_string);
                state.hovered_component_object_id = object_id.clone();
                state.hovered_component_mode = mode.clone();
                state.hovered_component_id = id;
                if mode.as_deref() == Some("mesh") {
                    state.local_hover_id = object_id;
                } else {
                    state.local_hover_id = None;
                }
            }
        }
        "worldPick" => {
            let merge = args.get("merge").and_then(|value| value.as_str()).unwrap_or("replace");
            if let Some(granularity) = args.get("granularity").and_then(|value| value.as_str()) {
                state.granularity = granularity.to_string();
            }
            if args.get("id").is_none_or(|value| value.is_null()) {
                if merge == "replace" {
                    state.component_ids.clear();
                }
            } else if let Some(id) = args.get("id").and_then(dsl_id_to_string) {
                state.component_ids = merge_string_ids(&state.component_ids, &[id], merge);
            }
        }
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: replaces the deleted ad-hoc
        // `worldSelect`/`worldHover` command strings with the framework interaction verbs
        // (`domainId`/`targets`/`merge`/`method`, `domainId`/`channel`/`targets`) — see
        // `resolved_domain_id` (the domain this surface actually targets: its bound app domain, or the
        // OS `world` fallback). This is still the OPTIMISTIC LOCAL PREVIEW only; the framework's
        // `next_selection`/`next_hover` machine (not this file) is the source of truth once the
        // round-trip settles.
        "interactionSelect" if args.get("domainId").and_then(|value| value.as_str()) == Some(resolved_domain_id(state)) => {
            let merge = args.get("merge").and_then(|value| value.as_str()).unwrap_or("replace");
            let ids: Vec<String> = args
                .get("targets")
                .and_then(|value| value.as_array())
                .map(|targets| targets.iter().filter_map(|target| target.get("id").and_then(dsl_id_to_string)).filter_map(|id| parse_resolved_item_id(state, &id).map(str::to_string)).collect())
                .unwrap_or_default();
            state.selected_ids = merge_string_ids(&state.selected_ids, &ids, merge);
        }
        "interactionHover" if args.get("domainId").and_then(|value| value.as_str()) == Some(resolved_domain_id(state)) => {
            state.local_hover_id =
                args.get("targets").and_then(|value| value.as_array()).and_then(|targets| targets.first()).and_then(|target| target.get("id").and_then(dsl_id_to_string)).and_then(|id| parse_resolved_item_id(state, &id).map(str::to_string));
        }
        "setSelection" => {
            if let Some(mode) = args.get("mode").and_then(|value| value.as_str()) {
                state.granularity = mode.to_string();
            }
            let ids = args.get("ids").map(dsl_string_vec).unwrap_or_default();
            state.component_ids = ids;
        }
        _ => {}
    }
}

fn pick_hover_action(state: &mut World3dState, x: f32, y: f32, inner: Rect) -> Option<ActionDescriptor> {
    if state.active_utility == "surfaceBrush" {
        if let Some((object_id, position, normal)) = pick_surface_at(state, x, y, inner) {
            return Some(ActionDescriptor {
                controller_id: state.controller_id.clone(),
                action: "worldSurfaceHover".into(),
                args: action_args(json!({
                    "surfaceId": state.surface_id,
                    "pane": state.surface_id,
                    "objectId": object_id,
                    "position": position,
                    "normal": normal,
                })),
            });
        }
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldSurfaceLeave".into(), args: action_args(json!({ "surfaceId": state.surface_id, "pane": state.surface_id })) });
    }
    if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
        let hit = pick_vortex_at(state, x, y, inner);
        if state.hovered_vortex_id == hit {
            return None;
        }
        state.hovered_vortex_id = hit.clone();
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldVortexHover".into(), args: action_args(json!({ "surfaceId": state.surface_id, "fullId": hit })) });
    }
    if component_mode_active(state) {
        if let Some((mode, id, object_id)) = pick_component_at(state, x, y, inner) {
            if state.hovered_component_id.as_deref() == Some(id.as_str()) && state.hovered_component_object_id.as_deref() == Some(object_id.as_str()) && state.hovered_component_mode.as_deref() == Some(mode.as_str()) {
                return None;
            }
            state.hovered_component_id = Some(id.clone());
            state.hovered_component_object_id = Some(object_id.clone());
            state.hovered_component_mode = Some(mode.clone());
            let id_num = id.parse::<u64>().unwrap_or(0);
            return Some(ActionDescriptor {
                controller_id: state.controller_id.clone(),
                action: "setHover".into(),
                args: action_args(json!({
                    "objectId": object_id,
                    "mode": mode,
                    "id": id_num,
                })),
            });
        }
        if state.hovered_component_id.is_none() && state.hovered_component_object_id.is_none() && state.hovered_component_mode.is_none() {
            return None;
        }
        state.hovered_component_id = None;
        state.hovered_component_object_id = None;
        state.hovered_component_mode = None;
        state.local_hover_id = None;
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "setHover".into(), args: None });
    }
    // 🖼️ Falls back to reference-image plane hit-testing when no mesh instance is under the
    // cursor — mirrors React's `hoveredId` covering both mesh instances and `"reference:"`-prefixed
    // reference planes (see `resolveWorldContextMenuTarget`'s reference tier).
    let hit = pick_instance_at(state, x, y, inner).or_else(|| pick_reference_at(state, x, y, inner).map(|url| format!("reference:{url}")));
    if state.local_hover_id == hit {
        return None;
    }
    state.local_hover_id = hit.clone();
    // 🕹️ `interactionHover` — empty `targets` clears the channel (see `HoverInput`/`next_hover`);
    // `domainId`/target id shape follow this surface's resolved (app-bound or `world`-fallback) domain.
    let targets = match &hit {
        Some(id) => json!([{ "granularity": resolved_domain_granularity_id(state), "id": resolved_item_id(state, id) }]),
        None => json!([]),
    };
    Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "interactionHover".into(), args: action_args(json!({ "domainId": resolved_domain_id(state), "channel": "pointer", "targets": targets })) })
}

fn pick_select_action(state: &World3dState, x: f32, y: f32, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    // 🕹️ Canonical `MergeMode` wire labels (see `merge_mode_wire_str`) — `merge_string_ids` accepts
    // these directly for both `worldPick` (unconverted, component-level picking) and the `interactionSelect`
    // emission below, so this one computation feeds both branches unchanged.
    let merge = if shift {
        merge_mode_wire_str(MergeMode::Additive)
    } else if ctrl {
        merge_mode_wire_str(MergeMode::Invertive)
    } else {
        merge_mode_wire_str(MergeMode::Replace)
    };
    if state.interaction_mode == "paint" {
        return None;
    }
    if component_mode_active(state) {
        let Some((granularity, id, _object_id)) = pick_component_at(state, x, y, inner) else {
            return Some(ActionDescriptor {
                controller_id: state.controller_id.clone(),
                action: "worldPick".into(),
                args: action_args(json!({
                    "surfaceId": state.surface_id,
                    "granularity": state.granularity,
                    "id": null,
                    "merge": merge,
                })),
            });
        };
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "worldPick".into(),
            args: action_args(json!({
                "surfaceId": state.surface_id,
                "granularity": granularity,
                "id": id.parse::<u64>().ok(),
                "merge": merge,
            })),
        });
    }
    if state.granularity == "mesh" {
        let hit = pick_instance_at(state, x, y, inner);
        let id = hit.as_deref().and_then(|object_id| instance_object_index(state, object_id));
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "worldPick".into(),
            args: action_args(json!({
                "surfaceId": state.surface_id,
                "granularity": "mesh",
                "id": id,
                "merge": merge,
            })),
        });
    }
    let hit = pick_instance_at(state, x, y, inner);
    let targets: Vec<serde_json::Value> = hit.into_iter().map(|id| json!({ "granularity": resolved_domain_granularity_id(state), "id": resolved_item_id(state, &id) })).collect();
    Some(ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "interactionSelect".into(),
        args: action_args(json!({
            "domainId": resolved_domain_id(state),
            "targets": targets,
            "merge": merge,
            "method": selection_method_wire_str(SelectionMethod::Pick),
        })),
    })
}

fn instance_object_index(state: &World3dState, object_id: &str) -> Option<u32> {
    let mut index = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            if instance.id == object_id {
                return Some(index);
            }
            index += 1;
        }
    }
    None
}

fn merge_u32_ids(existing: &[String], incoming: &[String], merge: &str) -> Vec<u32> {
    let parse = |ids: &[String]| -> Vec<u32> { ids.iter().filter_map(|id| id.parse().ok()).collect() };
    let existing_ids = parse(existing);
    let incoming_ids = parse(incoming);
    match merge {
        // 🕹️ `"additive"` is the framework `MergeMode` wire label — see `merge_string_ids`.
        "add" | "additive" => {
            let mut merged = existing_ids;
            for id in incoming_ids {
                if !merged.contains(&id) {
                    merged.push(id);
                }
            }
            merged
        }
        "toggle" | "invertive" => {
            let mut merged = existing_ids;
            for id in incoming_ids {
                if let Some(index) = merged.iter().position(|entry| *entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id);
                }
            }
            merged
        }
        _ => incoming_ids,
    }
}

#[cfg(test)]
fn marquee_select_action(state: &mut World3dState, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    if state.marquee_points.len() < 2 {
        return None;
    }
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let (polygon, rectangle, crossing) = marquee_local_polygon(state, inner);
    let (meshes, draws) = legacy_geometry_fixture(state);
    let ids = if component_mode_active(state) {
        screen_select_components(&meshes, &draws, view_proj, inner.w, inner.h, &polygon, rectangle, state.granularity.as_str(), state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&meshes, &draws, view_proj, inner.w, inner.h, &polygon, rectangle, crossing)
    };
    state.marquee_points.clear();
    state.marquee_preview_ids.clear();
    // 🕹️ Canonical `MergeMode` wire labels — see `pick_select_action`'s identical rationale.
    let merge = if shift {
        merge_mode_wire_str(MergeMode::Additive)
    } else if ctrl {
        merge_mode_wire_str(MergeMode::Invertive)
    } else {
        merge_mode_wire_str(MergeMode::Replace)
    };
    if component_mode_active(state) {
        let merged = merge_u32_ids(&state.component_ids, &ids, merge);
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "setSelection".into(),
            args: action_args(json!({
                "mode": state.granularity,
                "ids": merged,
            })),
        });
    }
    // 🕹️ Marquee/lasso stays GEOMETRIC here — `screen_select_instances` above is the surface's own
    // hit-test, this just batches its raw hits into ONE `interactionSelect`; the merge/mode algebra is
    // the os-kernel `next_selection` machine's job, not this file's.
    let targets: Vec<serde_json::Value> = ids.iter().map(|id| json!({ "granularity": resolved_domain_granularity_id(state), "id": resolved_item_id(state, id) })).collect();
    Some(ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "interactionSelect".into(),
        args: action_args(json!({
            "domainId": resolved_domain_id(state),
            "targets": targets,
            "merge": merge,
            "method": selection_method_wire_str(SelectionMethod::Rectangle),
        })),
    })
}

#[cfg(test)]
fn gumball_drag_update(state: &mut World3dState, x: f32, y: f32, inner: Rect) {
    let Some(handle) = state.gumball_handle else {
        return;
    };
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    let pivot = state.gumball_pivot;
    let eye = gumball_eye(&camera, pivot);
    reset_gumball_preview(state);
    if handle.is_translate() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let delta = current - state.gumball_drag_anchor;
                state.gumball_preview_translate = axis.normalize().scale(delta);
            }
        } else if let Some(normal) = handle.plane_normal() {
            if let Some(current) = ray_plane_point(origin, dir, pivot, normal) {
                state.gumball_preview_translate = current.sub(state.gumball_drag_start_vec);
            }
        }
    } else if handle.is_rotate() {
        if let Some(normal) = handle.plane_normal() {
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let current = hit.sub(pivot);
                if current.length() > 1e-4 && state.gumball_drag_start_vec.length() > 1e-4 {
                    state.gumball_preview_angle = axis_rotate_angle(state.gumball_drag_start_vec, current, normal);
                }
            }
        }
    } else if handle.is_scale() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let factor = if state.gumball_drag_anchor.abs() > 1e-4 { current / state.gumball_drag_anchor } else { 1.0 };
                let factor = factor.clamp(0.05, 20.0);
                match handle {
                    GumballHandle::ScaleX => state.gumball_preview_scale.x = factor,
                    GumballHandle::ScaleY => state.gumball_preview_scale.y = factor,
                    GumballHandle::ScaleZ => state.gumball_preview_scale.z = factor,
                    _ => {}
                }
            }
        }
    }
    apply_gumball_preview(state);
}

fn start_gumball_drag(state: &mut World3dState, handle: GumballHandle, x: f32, y: f32, inner: Rect) {
    let Some(pivot) = selection_centroid(state) else {
        return;
    };
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    let eye = gumball_eye(&camera, pivot);
    state.gumball_handle = Some(handle);
    state.gumball_pivot = pivot;
    reset_gumball_preview(state);
    if let Some(axis) = handle.axis_dir() {
        state.gumball_drag_anchor = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye).unwrap_or(0.0);
    } else if let Some(normal) = handle.plane_normal() {
        state.gumball_drag_start_vec = ray_plane_point(origin, dir, pivot, normal).unwrap_or(pivot);
        if handle.is_rotate() {
            state.gumball_drag_start_vec = state.gumball_drag_start_vec.sub(pivot);
        }
        state.gumball_drag_anchor = 0.0;
    }
}

fn pick_component_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<(String, String, String)> {
    let (local_x, local_y, rect) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (rect.w / rect.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let granularity = state.granularity.as_str();
    match granularity {
        "vertex" => {
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                let Ok(schema) = mesh.schema() else { continue };
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    for vertex_index in 0..schema.vertices {
                        let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                        let world = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                        let Some(screen) = ui_wgpu::wgpu::project_point(view_proj, world, rect.w, rect.h) else {
                            continue;
                        };
                        let dx = screen[0] - local_x;
                        let dy = screen[1] - local_y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist <= PICK_VERTEX_SCREEN_PX && best.as_ref().is_none_or(|(best_dist, _, _)| dist < *best_dist) {
                            let id = world_mesh_component_id(mesh, Mesh3dField::VertexIds, vertex_index).to_string();
                            best = Some((dist, id, instance.id.clone()));
                        }
                    }
                }
            }
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "edge" => {
            let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, f32, String, String)> = None;
            for draw in &state.draws {
                let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                let Ok(schema) = mesh.schema() else { continue };
                if schema.edges == 0 {
                    continue;
                }
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    for edge_index in 0..schema.edges {
                        let Ok(edge) = mesh.edge(edge_index) else { continue };
                        let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                        let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                        let (Some(screen_a), Some(screen_b)) = (ui_wgpu::wgpu::project_point(view_proj, a, rect.w, rect.h), ui_wgpu::wgpu::project_point(view_proj, b, rect.w, rect.h)) else {
                            continue;
                        };
                        let screen_dist = ui_wgpu::wgpu::screen_segment_distance(local_x, local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]);
                        if screen_dist > PICK_EDGE_SCREEN_PX {
                            continue;
                        }
                        let ray_dist = ray_segment_distance(origin, dir, a, b).unwrap_or(f32::INFINITY);
                        let depth = a.add(b).scale(0.5).sub(origin).dot(dir);
                        let better = match &best {
                            None => true,
                            Some((best_ray, best_depth, _, _)) => depth < *best_depth - 1e-4 || ((depth - *best_depth).abs() <= 1e-4 && ray_dist < *best_ray),
                        };
                        if better {
                            let id = world_mesh_component_id(mesh, Mesh3dField::EdgeIds, edge_index).to_string();
                            best = Some((ray_dist, depth, id, instance.id.clone()));
                        }
                    }
                }
            }
            return best.map(|(_, _, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "face" => {
            let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    let Some(hit) = ray_pick_mesh_detail(origin, dir, mesh, instance) else {
                        continue;
                    };
                    if best.as_ref().is_none_or(|(best_depth, _, _)| hit.distance < *best_depth) {
                        let id = mesh_face_id(mesh, u32::try_from(hit.triangle_index).ok()?);
                        best = Some((hit.distance, id, instance.id.clone()));
                    }
                }
            }
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        _ => {}
    }
    None
}

fn pick_paint_hit(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<(String, f32, f32)> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, f32, f32)> = None;
    for draw in &state.draws {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(hit) = ray_pick_mesh_detail(origin, dir, mesh, instance) {
                if let Some((u, v)) = interpolate_mesh_uv(mesh, hit.triangle_index, hit.bary_u, hit.bary_v) {
                    if best.as_ref().is_none_or(|(best_dist, _, _, _)| hit.distance < *best_dist) {
                        best = Some((hit.distance, instance.id.clone(), u, v));
                    }
                }
            }
        }
    }
    best.map(|(_, object_id, u, v)| (object_id, u, v))
}

fn marquee_local_polygon(state: &World3dState, rect: Rect) -> (Vec<[f32; 2]>, bool, bool) {
    let rectangle = state.selection_method != "lasso";
    let crossing = marquee_is_crossing_from_path(&state.marquee_points, !rectangle);
    let global: Vec<[f32; 2]> = if rectangle {
        let start = state.marquee_points[0];
        let end = state.marquee_points[state.marquee_points.len() - 1];
        vec![start, end]
    } else {
        state.marquee_points.clone()
    };
    let local = global.iter().map(|point| [point[0] - rect.x, point[1] - rect.y]).collect();
    (local, rectangle, crossing)
}

#[cfg(test)]
fn update_marquee_preview(state: &mut World3dState, inner: Rect) {
    if state.marquee_points.len() < 2 {
        state.marquee_preview_ids.clear();
        return;
    }
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let (polygon, rectangle, crossing) = marquee_local_polygon(state, inner);
    let (meshes, draws) = legacy_geometry_fixture(state);
    state.marquee_preview_ids = if component_mode_active(state) {
        screen_select_components(&meshes, &draws, view_proj, inner.w, inner.h, &polygon, rectangle, state.granularity.as_str(), state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&meshes, &draws, view_proj, inner.w, inner.h, &polygon, rectangle, crossing)
    };
}

#[cfg(test)]
fn legacy_geometry_fixture(state: &World3dState) -> (HashMap<String, Mesh3dLease>, Vec<SceneDraw3d>) {
    (state.meshes.iter().map(|(id, mesh)| (id.clone(), *mesh)).collect(), state.draws.iter().cloned().collect())
}

fn pick_instance_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for draw in &state.draws {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(distance) = ray_pick_instance(origin, dir, mesh, instance) {
                if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                    best = Some((distance, instance.id.clone()));
                }
            }
        }
    }
    best.map(|(_, id)| id)
}

fn pick_surface_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<(String, [f64; 3], [f64; 3])> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, Vec3, Vec3)> = None;
    for draw in &state.draws {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            let Some(hit) = ray_pick_mesh_detail(origin, dir, mesh, instance) else {
                continue;
            };
            if best.as_ref().is_none_or(|(best_distance, _, _, _)| hit.distance < *best_distance) {
                best = Some((hit.distance, instance.id.clone(), hit.point, hit.normal));
            }
        }
    }
    best.map(|(_, id, point, normal)| (id, [point.x as f64, point.y as f64, point.z as f64], [normal.x as f64, normal.y as f64, normal.z as f64]))
}

//#region VortexArrow
const VORTEX_ARROW_SHAFT_MESH: &str = "cylinder";
const VORTEX_ARROW_HEAD_MESH: &str = "cone";

struct VortexArrowLayout {
    point_radius: f32,
    shaft_radius: f32,
    shaft_length: f32,
    head_length: f32,
    shaft_center: [f32; 3],
    head_base: [f32; 3],
    rotation: [f32; 4],
}

fn quat_normalize_f32(quat: [f32; 4]) -> [f32; 4] {
    let len = (quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]).sqrt();
    if len < 1e-9 {
        [0.0, 0.0, 0.0, 1.0]
    } else {
        [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len]
    }
}

fn quat_from_unit_vectors(from: Vec3, to: Vec3) -> [f32; 4] {
    let from = from.normalize();
    let to = to.normalize();
    let r = from.dot(to) + 1.0;
    let quat = if r < 0.000_001 {
        if from.x.abs() > from.z.abs() {
            [-from.y, from.x, 0.0, r]
        } else {
            [0.0, -from.z, from.y, r]
        }
    } else {
        let cross = from.cross(to);
        [cross.x, cross.y, cross.z, r]
    };
    quat_normalize_f32(quat)
}

fn vortex_unit_direction(direction: Option<[f64; 3]>) -> Vec3 {
    let dir = direction.map(|value| Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32)).unwrap_or(Vec3::new(0.0, 0.0, -1.0));
    if dir.dot(dir) < 1e-12 {
        Vec3::new(0.0, 0.0, -1.0)
    } else {
        dir.normalize()
    }
}

fn vortex_arrow_layout(position: [f64; 3], direction: Option<[f64; 3]>, radius: f32, display_direction: Option<&str>) -> VortexArrowLayout {
    let dir = vortex_unit_direction(direction);
    let pos = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
    let arrow_length = radius;
    let head_length = radius * 0.28;
    let shaft_length = (arrow_length - head_length).max(radius * 0.2);
    let shaft_radius = radius * 0.055;
    let point_radius = radius * 0.18;
    let outward = !display_direction.is_some_and(|mode| mode == "inwards");
    let rotation = quat_from_unit_vectors(Vec3::new(0.0, 1.0, 0.0), dir);
    let (shaft_center, head_base) = if outward { (pos.add(dir.scale(shaft_length * 0.5)), pos.add(dir.scale(shaft_length))) } else { (pos.sub(dir.scale(head_length + shaft_length * 0.5)), pos.sub(dir.scale(head_length))) };
    VortexArrowLayout { point_radius, shaft_radius, shaft_length, head_length, shaft_center: shaft_center.to_array(), head_base: head_base.to_array(), rotation }
}

fn ensure_primitive_mesh(state: &mut World3dState, mesh_key: &str) {
    if state.meshes.contains_key(mesh_key) {
        return;
    }
    begin_world_placeholder_mesh(state, mesh_key, WorldPlaceholderKind::resolve(mesh_key));
}

fn append_vortex_arrow_draws(state: &mut World3dState, gpu: &mut World3dBuildContext, extra_draws: &mut Vec<SceneDraw3d>) {
    if state.vortices.is_empty() {
        return;
    }
    let mut point_instances = Vec::new();
    let mut shaft_instances = Vec::new();
    let mut head_instances = Vec::new();
    for vortex in &state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let layout = vortex_arrow_layout(position, vortex.direction, radius, vortex.display_direction.as_deref());
        let color = parse_color(vortex.color.as_deref().unwrap_or("#38bdf8"));
        let hovered = state.hovered_vortex_id.as_deref() == Some(vortex.full_id.as_str());
        let id = vortex.full_id.clone();
        point_instances.push(Instance3d {
            id: format!("{id}:point"),
            model: Instance3d::model_from_trs([position[0] as f32, position[1] as f32, position[2] as f32], [0.0, 0.0, 0.0, 1.0], [layout.point_radius, layout.point_radius, layout.point_radius]),
            color,
            selected: false,
            hovered,
        });
        shaft_instances.push(Instance3d {
            id: format!("{id}:shaft"),
            model: Instance3d::model_from_trs(layout.shaft_center, layout.rotation, [layout.shaft_radius * 2.0, layout.shaft_length, layout.shaft_radius * 2.0]),
            color,
            selected: false,
            hovered,
        });
        head_instances.push(Instance3d { id: format!("{id}:head"), model: Instance3d::model_from_trs(layout.head_base, layout.rotation, [layout.shaft_radius * 3.6, layout.head_length, layout.shaft_radius * 3.6]), color, selected: false, hovered });
    }
    ensure_primitive_mesh(state, "vortex-marker");
    ensure_primitive_mesh(state, VORTEX_ARROW_SHAFT_MESH);
    ensure_primitive_mesh(state, VORTEX_ARROW_HEAD_MESH);
    for (mesh_key, instances) in [("vortex-marker", point_instances), (VORTEX_ARROW_SHAFT_MESH, shaft_instances), (VORTEX_ARROW_HEAD_MESH, head_instances)] {
        let mesh_version = *state.mesh_versions.get(mesh_key).unwrap_or(&0);
        if let Some(mesh) = state.meshes.get(mesh_key) {
            gpu.ensure_mesh(mesh_key, mesh_version, *mesh);
        }
        extra_draws.push(SceneDraw3d { mesh_key: mesh_key.into(), mesh_version, instances });
    }
}
//#endregion VortexArrow

fn pick_vortex_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for vortex in &state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let center = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
        let min = [center.x - radius, center.y - radius, center.z - radius];
        let max = [center.x + radius, center.y + radius, center.z + radius];
        if let Some(distance) = ray_aabb_slab(origin, dir, min, max) {
            if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                best = Some((distance, vortex.full_id.clone()));
            }
        }
    }
    best.map(|(_, id)| id)
}

/// 🖼️ Ray-vs-quad hit test against visible reference-image planes, returning the closest hit
/// reference's `url` (used as its hover/context-menu identifier, since `WorldReferenceRecord` has
/// no separate id field). References are flat rectangles lying in the local XY plane (normal +Z,
/// width along X, height along Y) centered at `origin` — matching this renderer's Z-up convention.
fn pick_reference_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let plane_normal = Vec3::new(0.0, 0.0, 1.0);
    let mut best: Option<(f32, String)> = None;
    for reference in &state.references {
        if reference.hidden.unwrap_or(false) {
            continue;
        }
        let Some(url) = reference.url.as_deref() else {
            continue;
        };
        let plane_position = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let plane_origin = Vec3::new(plane_position[0] as f32, plane_position[1] as f32, plane_position[2] as f32);
        let width = reference.width_world.unwrap_or(1.0) as f32;
        let image_aspect = reference_image_aspect(state, url);
        let height = width / image_aspect.max(0.01);
        let Some(hit) = ray_plane_point(origin, dir, plane_origin, plane_normal) else {
            continue;
        };
        let offset = hit.sub(plane_origin);
        if offset.x.abs() > width * 0.5 || offset.y.abs() > height * 0.5 {
            continue;
        }
        let distance = origin.sub(hit).length();
        if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
            best = Some((distance, url.to_string()));
        }
    }
    best.map(|(_, url)| url)
}

fn object_world_position(state: &World3dState, object_id: &str) -> Option<[f32; 3]> {
    for draw in &state.draws {
        for instance in &draw.instances {
            if instance.id == object_id {
                let translation = instance.model.cols[3];
                return Some([translation[0], translation[1], translation[2]]);
            }
        }
    }
    None
}

fn update_dragged_instance_position(state: &mut World3dState, object_id: &str, position: [f32; 3]) {
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            if instance.id == object_id {
                instance.model.cols[3] = [position[0], position[1], position[2], 1.0];
            }
        }
    }
}

fn ground_plane_pick(state: &World3dState, x: f32, y: f32, _inner: Rect, plane_z: f32) -> Option<[f32; 3]> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    if dir.z.abs() < 1e-5 {
        return None;
    }
    let t = (plane_z - origin.z) / dir.z;
    if t < 0.0 {
        return None;
    }
    let hit = origin.add(dir.scale(t));
    Some([hit.x, hit.y, hit.z])
}

fn preview_scale(scale: Option<&serde_json::Value>) -> [f32; 3] {
    match scale {
        Some(serde_json::Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0) as f32, values[1].as_f64().unwrap_or(1.0) as f32, values[2].as_f64().unwrap_or(1.0) as f32],
        _ => [1.0, 1.0, 1.0],
    }
}

fn reference_image_aspect(state: &World3dState, url: &str) -> f32 {
    state.reference_pixels.get(url).map(|(width, height, _)| *width as f32 / (*height).max(1) as f32).unwrap_or(1.0).max(0.01)
}

/// 👻️ Mesh key for `BrushPreviewGhost`: the real GLB's resolved id when a `meshUrl` is given
/// (loaded lazily, same as any other mesh), else the shared "box" primitive fallback ghost.
fn brush_preview_mesh_id(mesh_url: Option<&str>) -> String {
    mesh_url.map(mesh_id_from_url).unwrap_or_else(|| "box".to_string())
}

fn append_box_wireframe(lines: &mut Vec<LineVertex3d>, origin: [f64; 3], orientation: [f64; 4], scale: [f64; 3], color: [f32; 4]) {
    let corners = [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5], [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]];
    let model = Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [orientation[0] as f32, orientation[1] as f32, orientation[2] as f32, orientation[3] as f32], [scale[0] as f32, scale[1] as f32, scale[2] as f32]);
    let world_corners: Vec<[f32; 3]> = corners.iter().map(|corner| model.transform_point(Vec3::new(corner[0], corner[1], corner[2])).to_array()).collect();
    let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    for (a, b) in edges {
        lines.push(LineVertex3d { position: world_corners[a], color });
        lines.push(LineVertex3d { position: world_corners[b], color });
    }
}

fn parse_color(value: &str) -> [f32; 4] {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix('#') {
        let expanded = match hex.len() {
            3 => hex.chars().map(|ch| format!("{ch}{ch}")).collect::<String>(),
            _ => hex.to_string(),
        };
        if expanded.len() >= 6 {
            let r = u8::from_str_radix(&expanded[0..2], 16).unwrap_or(148) as f32 / 255.0;
            let g = u8::from_str_radix(&expanded[2..4], 16).unwrap_or(163) as f32 / 255.0;
            let b = u8::from_str_radix(&expanded[4..6], 16).unwrap_or(184) as f32 / 255.0;
            return [r, g, b, 1.0];
        }
    }
    [0.58, 0.64, 0.72, 1.0]
}

//#region 📡️WorldAssetIoAuthority
pub const WORLD_ASSET_REQUEST_CAPACITY: usize = 64;
pub const WORLD_ASSET_URL_BYTE_CAPACITY: usize = 2_048;
pub const WORLD_ASSET_RESPONSE_PAGE_BYTES: usize = 16 * 1024;
pub const WORLD_ASSET_RESPONSE_PAGE_CAPACITY: usize = 1_024;
pub const WORLD_ASSET_RESPONSE_BYTE_CAPACITY: usize = WORLD_ASSET_RESPONSE_PAGE_BYTES * WORLD_ASSET_RESPONSE_PAGE_CAPACITY;
pub const WORLD_ASSET_METADATA_ID_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldAssetMetadataId {
    bytes: [u8; WORLD_ASSET_METADATA_ID_BYTES],
    len: u16,
}

impl WorldAssetMetadataId {
    pub fn try_from_str(value: &str) -> Result<Self, WorldAssetFault> {
        if value.len() > WORLD_ASSET_METADATA_ID_BYTES {
            return Err(WorldAssetFault::MetadataCapacity);
        }
        let mut bytes = [0; WORLD_ASSET_METADATA_ID_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("asset metadata originates from UTF-8")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldAssetRequestKind {
    Glb,
    ReferenceImage,
    Terrain { z: u32, x: u32, y: u32 },
    MapTile { surface: WorldAssetMetadataId, key: WorldAssetMetadataId, vector: bool, z: u32, x: u32, y: u32 },
    UiImage { id: WorldAssetMetadataId },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldAssetRequestToken {
    slot: u8,
    epoch: u64,
    generation: u64,
    revision: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldAssetFault {
    Closing,
    ItemCapacity,
    ByteCapacity,
    UrlCapacity,
    MetadataCapacity,
    PageCapacity,
    Stale,
    Incomplete,
}

#[derive(Debug)]
pub struct WorldAssetResponsePage {
    bytes: Box<[u8]>,
}

impl WorldAssetResponsePage {
    pub fn try_from_owned(bytes: Vec<u8>) -> Result<Self, Vec<u8>> {
        if bytes.len() > WORLD_ASSET_RESPONSE_PAGE_BYTES {
            return Err(bytes);
        }
        Ok(Self { bytes: bytes.into_boxed_slice() })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug)]
pub struct WorldAssetFetchOwner {
    token: WorldAssetRequestToken,
    generation: u64,
    revision: u64,
    kind: WorldAssetRequestKind,
    url: String,
    reserved_bytes: usize,
    received_bytes: usize,
    pages: Box<[Option<WorldAssetResponsePage>; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]>,
    page_len: u16,
    page_read: u16,
    close_page: u16,
    sealed: bool,
    closing: bool,
}

impl WorldAssetFetchOwner {
    pub fn token(&self) -> WorldAssetRequestToken {
        self.token
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn kind(&self) -> WorldAssetRequestKind {
        self.kind
    }

    pub fn push_page(&mut self, page: WorldAssetResponsePage) -> Result<(), WorldAssetResponsePage> {
        let next = self.received_bytes.checked_add(page.bytes.len()).unwrap_or(usize::MAX);
        if self.closing || self.sealed || usize::from(self.page_len) == WORLD_ASSET_RESPONSE_PAGE_CAPACITY || next > self.reserved_bytes {
            return Err(page);
        }
        self.pages[usize::from(self.page_len)] = Some(page);
        self.page_len += 1;
        self.received_bytes = next;
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), WorldAssetFault> {
        if self.closing {
            return Err(WorldAssetFault::Closing);
        }
        self.sealed = true;
        Ok(())
    }

    pub fn take_decode_page(&mut self) -> Result<Option<WorldAssetResponsePage>, WorldAssetFault> {
        if !self.sealed {
            return Err(WorldAssetFault::Incomplete);
        }
        if self.page_read == self.page_len {
            return Ok(None);
        }
        let page = self.pages[usize::from(self.page_read)].take().ok_or(WorldAssetFault::Stale)?;
        self.page_read += 1;
        Ok(Some(page))
    }

    pub fn decode_page(&self) -> Result<Option<&WorldAssetResponsePage>, WorldAssetFault> {
        if !self.sealed {
            return Err(WorldAssetFault::Incomplete);
        }
        if self.page_read == self.page_len {
            return Ok(None);
        }
        self.pages[usize::from(self.page_read)].as_ref().map(Some).ok_or(WorldAssetFault::Stale)
    }

    pub fn decode_page_at(&self, index: u16) -> Result<Option<&WorldAssetResponsePage>, WorldAssetFault> {
        if !self.sealed || self.closing {
            return Err(WorldAssetFault::Incomplete);
        }
        if index >= self.page_len {
            return Ok(None);
        }
        self.pages[usize::from(index)].as_ref().map(Some).ok_or(WorldAssetFault::Stale)
    }

    pub fn decode_page_len(&self) -> Result<u16, WorldAssetFault> {
        if !self.sealed || self.closing {
            return Err(WorldAssetFault::Incomplete);
        }
        Ok(self.page_len)
    }

    pub fn advance_decode_page(&mut self) -> Result<(), WorldAssetFault> {
        if self.decode_page()?.is_none() {
            return Err(WorldAssetFault::Incomplete);
        }
        self.page_read += 1;
        Ok(())
    }

    pub fn rewind_decode_pages(&mut self) -> Result<(), WorldAssetFault> {
        if !self.sealed || self.closing {
            return Err(WorldAssetFault::Incomplete);
        }
        self.page_read = 0;
        Ok(())
    }

    pub fn received_bytes(&self) -> usize {
        self.received_bytes
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        if self.close_page < self.page_len {
            self.pages[usize::from(self.close_page)] = None;
            self.close_page += 1;
            return false;
        }
        if !self.url.is_empty() {
            self.url.clear();
            return false;
        }
        self.page_len = 0;
        self.page_read = 0;
        self.close_page = 0;
        self.received_bytes = 0;
        self.reserved_bytes = 0;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.url.is_empty() && self.pages.iter().all(Option::is_none) && self.reserved_bytes == 0 && self.received_bytes == 0
    }
}

#[cfg(not(test))]
impl Drop for WorldAssetFetchOwner {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "WorldAssetFetchOwner reached Drop before every response page and request string reached terminal handback");
    }
}

struct WorldAssetClaim {
    epoch: u64,
    generation: u64,
    revision: u64,
    kind: WorldAssetRequestKind,
    url_len: u16,
    url_bytes: Box<[u8; WORLD_ASSET_URL_BYTE_CAPACITY]>,
    reserved_bytes: usize,
    in_flight: bool,
    fetch_complete: bool,
    owner: Option<WorldAssetFetchOwner>,
}

pub struct WorldAssetIoAuthority {
    slots: Box<[Option<WorldAssetClaim>; WORLD_ASSET_REQUEST_CAPACITY]>,
    epochs: [u64; WORLD_ASSET_REQUEST_CAPACITY],
    completed_cursor: u8,
    reserved_bytes: usize,
    closing: bool,
}

impl Default for WorldAssetIoAuthority {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; WORLD_ASSET_REQUEST_CAPACITY]), epochs: [0; WORLD_ASSET_REQUEST_CAPACITY], completed_cursor: 0, reserved_bytes: 0, closing: false }
    }
}

impl WorldAssetIoAuthority {
    pub fn reserve_request(&mut self, generation: u64, revision: u64, kind: WorldAssetRequestKind, url: &str) -> Result<WorldAssetRequestToken, WorldAssetFault> {
        self.reserve(generation, revision, kind, url, 0)
    }

    pub fn reserve(&mut self, generation: u64, revision: u64, kind: WorldAssetRequestKind, url: &str, byte_credits: usize) -> Result<WorldAssetRequestToken, WorldAssetFault> {
        if self.closing {
            return Err(WorldAssetFault::Closing);
        }
        if url.len() > WORLD_ASSET_URL_BYTE_CAPACITY {
            return Err(WorldAssetFault::UrlCapacity);
        }
        if let Some((slot, claim)) = self.slots.iter().enumerate().find_map(|(slot, claim)| {
            let claim = claim.as_ref()?;
            (claim.kind == kind && usize::from(claim.url_len) == url.len() && &claim.url_bytes[..url.len()] == url.as_bytes()).then_some((slot, claim))
        }) {
            if claim.revision != revision {
                return Err(WorldAssetFault::Stale);
            }
            return Ok(WorldAssetRequestToken { slot: slot as u8, epoch: claim.epoch, generation: claim.generation, revision: claim.revision });
        }
        let next = self.reserved_bytes.checked_add(byte_credits).ok_or(WorldAssetFault::ByteCapacity)?;
        if byte_credits > WORLD_ASSET_RESPONSE_BYTE_CAPACITY || next > WORLD_ASSET_RESPONSE_BYTE_CAPACITY {
            return Err(WorldAssetFault::ByteCapacity);
        }
        let slot = self.slots.iter().position(Option::is_none).ok_or(WorldAssetFault::ItemCapacity)?;
        self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
        let token = WorldAssetRequestToken { slot: slot as u8, epoch: self.epochs[slot], generation, revision };
        let mut url_bytes = Box::new([0; WORLD_ASSET_URL_BYTE_CAPACITY]);
        url_bytes[..url.len()].copy_from_slice(url.as_bytes());
        let owner = WorldAssetFetchOwner {
            token,
            generation,
            revision,
            kind,
            url: url.to_owned(),
            reserved_bytes: byte_credits,
            received_bytes: 0,
            pages: Box::new([const { None }; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]),
            page_len: 0,
            page_read: 0,
            close_page: 0,
            sealed: false,
            closing: false,
        };
        self.slots[slot] = Some(WorldAssetClaim { epoch: token.epoch, generation, revision, kind, url_len: url.len() as u16, url_bytes, reserved_bytes: byte_credits, in_flight: false, fetch_complete: false, owner: Some(owner) });
        self.reserved_bytes = next;
        Ok(token)
    }

    pub fn take_next(&mut self) -> Option<WorldAssetFetchOwner> {
        let claim = self.slots.iter_mut().flatten().find(|claim| !claim.in_flight && !claim.fetch_complete && claim.owner.is_some())?;
        claim.in_flight = true;
        claim.owner.take()
    }

    pub fn reserve_response(&mut self, owner: &mut WorldAssetFetchOwner, byte_credits: usize) -> Result<(), WorldAssetFault> {
        let slot = usize::from(owner.token.slot);
        let claim = self.slots.get_mut(slot).and_then(Option::as_mut).ok_or(WorldAssetFault::Stale)?;
        if claim.epoch != owner.token.epoch || claim.generation != owner.generation || claim.revision != owner.revision || !claim.in_flight || claim.reserved_bytes != 0 || owner.reserved_bytes != 0 {
            return Err(WorldAssetFault::Stale);
        }
        let next = self.reserved_bytes.checked_add(byte_credits).ok_or(WorldAssetFault::ByteCapacity)?;
        if byte_credits > WORLD_ASSET_RESPONSE_BYTE_CAPACITY || next > WORLD_ASSET_RESPONSE_BYTE_CAPACITY {
            return Err(WorldAssetFault::ByteCapacity);
        }
        claim.reserved_bytes = byte_credits;
        owner.reserved_bytes = byte_credits;
        self.reserved_bytes = next;
        Ok(())
    }

    pub fn return_owner(&mut self, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
        let slot = usize::from(owner.token.slot);
        let Some(claim) = self.slots.get_mut(slot).and_then(Option::as_mut) else {
            return Err(owner);
        };
        if claim.epoch != owner.token.epoch || claim.generation != owner.generation || claim.revision != owner.revision || !claim.in_flight || claim.owner.is_some() {
            return Err(owner);
        }
        claim.in_flight = false;
        claim.fetch_complete = owner.sealed || owner.closing;
        claim.owner = Some(owner);
        Ok(())
    }

    pub fn seal_response(&mut self, owner: &mut WorldAssetFetchOwner) -> Result<(), WorldAssetFault> {
        let slot = usize::from(owner.token.slot);
        let claim = self.slots.get_mut(slot).and_then(Option::as_mut).ok_or(WorldAssetFault::Stale)?;
        if claim.epoch != owner.token.epoch || claim.generation != owner.generation || claim.revision != owner.revision || !claim.in_flight || claim.reserved_bytes != owner.reserved_bytes || owner.received_bytes > owner.reserved_bytes {
            return Err(WorldAssetFault::Stale);
        }
        owner.seal()?;
        let unused = owner.reserved_bytes - owner.received_bytes;
        owner.reserved_bytes = owner.received_bytes;
        claim.reserved_bytes = owner.received_bytes;
        self.reserved_bytes -= unused;
        Ok(())
    }

    pub fn take_completed(&mut self, token: WorldAssetRequestToken, generation: u64, revision: u64) -> Result<WorldAssetFetchOwner, WorldAssetFault> {
        let slot = usize::from(token.slot);
        let claim = self.slots.get_mut(slot).and_then(Option::as_mut).ok_or(WorldAssetFault::Stale)?;
        if claim.epoch != token.epoch || claim.generation != generation || claim.revision != revision || claim.in_flight || !claim.fetch_complete || claim.owner.as_ref().is_none_or(|owner| !owner.sealed) {
            return Err(WorldAssetFault::Stale);
        }
        claim.in_flight = true;
        Ok(claim.owner.take().expect("validated completed asset owner"))
    }

    pub fn take_next_completed_step(&mut self) -> Option<WorldAssetFetchOwner> {
        let slot = usize::from(self.completed_cursor);
        self.completed_cursor = ((slot + 1) % WORLD_ASSET_REQUEST_CAPACITY) as u8;
        let claim = self.slots[slot].as_mut()?;
        if claim.in_flight || !claim.fetch_complete || claim.owner.as_ref().is_none_or(|owner| !owner.sealed) {
            return None;
        }
        claim.in_flight = true;
        claim.owner.take()
    }

    pub fn finish(&mut self, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
        let slot = usize::from(owner.token.slot);
        let Some(claim) = self.slots.get(slot).and_then(Option::as_ref) else {
            return Err(owner);
        };
        if claim.epoch != owner.token.epoch || !claim.in_flight || !claim.fetch_complete || !owner.terminal_is_empty() {
            return Err(owner);
        }
        let claim = self.slots[slot].take().expect("validated terminal asset claim");
        self.reserved_bytes -= claim.reserved_bytes;
        drop(owner);
        Ok(())
    }

    pub fn cancellation_requested(&self, token: WorldAssetRequestToken) -> bool {
        if self.closing {
            return true;
        }
        self.slots.get(usize::from(token.slot)).and_then(Option::as_ref).is_none_or(|claim| claim.epoch != token.epoch || claim.generation != token.generation || claim.revision != token.revision)
    }

    pub fn retire_cancelled_step(&mut self) -> bool {
        let Some(slot) = self.slots.iter().position(|claim| claim.as_ref().is_some_and(|claim| !claim.in_flight && claim.owner.as_ref().is_some_and(|owner| owner.closing))) else {
            return false;
        };
        let claim = self.slots[slot].as_mut().expect("cancelled asset claim");
        let owner = claim.owner.as_mut().expect("cancelled asset owner");
        if !owner.close_step() {
            return true;
        }
        self.reserved_bytes -= claim.reserved_bytes;
        self.slots[slot] = None;
        true
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        let Some(slot) = self.slots.iter().position(Option::is_some) else {
            return self.reserved_bytes == 0;
        };
        let claim = self.slots[slot].as_mut().expect("asset close claim");
        if claim.in_flight {
            return false;
        }
        let owner = claim.owner.as_mut().expect("non-flight asset claim owns request");
        if !owner.close_step() {
            return false;
        }
        self.reserved_bytes -= claim.reserved_bytes;
        self.slots[slot] = None;
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.reserved_bytes == 0 && self.slots.iter().all(Option::is_none)
    }
}

pub fn reserve_world3d_asset(state: &mut World3dState, kind: WorldAssetRequestKind, url: &str, byte_credits: usize) -> Result<WorldAssetRequestToken, WorldAssetFault> {
    let generation = state.asset_generation.wrapping_add(1).max(1);
    let token = state.asset_io.reserve(generation, state.interaction_revision, kind, url, byte_credits)?;
    state.asset_generation = state.asset_generation.max(token.generation);
    Ok(token)
}

pub fn reserve_world3d_asset_request(state: &mut World3dState, kind: WorldAssetRequestKind, url: &str) -> Result<WorldAssetRequestToken, WorldAssetFault> {
    let generation = state.asset_generation.wrapping_add(1).max(1);
    let token = state.asset_io.reserve_request(generation, state.interaction_revision, kind, url)?;
    state.asset_generation = state.asset_generation.max(token.generation);
    Ok(token)
}

pub fn take_next_world3d_asset(state: &mut World3dState) -> Option<WorldAssetFetchOwner> {
    state.asset_io.take_next()
}

pub fn reserve_world3d_asset_response(state: &mut World3dState, owner: &mut WorldAssetFetchOwner, byte_credits: usize) -> Result<(), WorldAssetFault> {
    state.asset_io.reserve_response(owner, byte_credits)
}

pub fn return_world3d_asset(state: &mut World3dState, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    state.asset_io.return_owner(owner)
}

pub fn seal_world3d_asset_response(state: &mut World3dState, owner: &mut WorldAssetFetchOwner) -> Result<(), WorldAssetFault> {
    state.asset_io.seal_response(owner)
}

pub fn take_completed_world3d_asset(state: &mut World3dState, token: WorldAssetRequestToken) -> Result<WorldAssetFetchOwner, WorldAssetFault> {
    if token.revision != state.interaction_revision {
        return Err(WorldAssetFault::Stale);
    }
    state.asset_io.take_completed(token, token.generation, token.revision)
}

pub fn take_next_completed_world3d_asset_step(state: &mut World3dState) -> Option<WorldAssetFetchOwner> {
    state.asset_io.take_next_completed_step()
}

pub fn finish_world3d_asset(state: &mut World3dState, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    state.asset_io.finish(owner)
}

pub fn retire_cancelled_world3d_asset_step(state: &mut World3dState) -> bool {
    state.asset_io.retire_cancelled_step()
}

pub fn world3d_asset_cancellation_requested(state: &World3dState, token: WorldAssetRequestToken) -> bool {
    state.asset_io.cancellation_requested(token)
}

#[cfg(not(test))]
impl Drop for WorldAssetIoAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "WorldAssetIoAuthority reached Drop before every request claim reached terminal handback");
    }
}
//#endregion 📡️WorldAssetIoAuthority

fn mesh_id_from_url(url: &str) -> String {
    let slug = url.trim_start_matches('/').rsplit('/').next().unwrap_or(url).trim_end_matches(".glb").trim_end_matches(".gltf");
    format!("mesh:{slug}")
}

pub fn publish_world3d_asset_mesh_lease(state: &mut World3dState, url: &str, mesh: Mesh3dLease) -> Result<(), WorldDynamicRejected<Mesh3dLease>> {
    if mesh.revision() != state.interaction_revision {
        return Err(WorldDynamicRejected { fault: WorldDynamicFault::StaleToken, id: mesh_id_from_url(url), value: mesh });
    }
    publish_world3d_mesh_lease(state, mesh_id_from_url(url), mesh)
}

// 🌉️ Dead on every target: repo-wide grep found zero callers of `apply_reference_image_bytes`
// (not even a test). Gated rather than deleted, to keep the diff minimal and reversible if a
// future caller lands. Does NOT remove `image` from the `wasm32-wasip2` link graph by itself —
// `🖼️canvas`'s `icon_codec::board_resolve_icon_kind` (used by flow's genuinely guest-reachable
// `preview_media_natural_size` widget-layout path, see
// `🔍️research/📓️infinite-host-deps-split.md`) still needs it unconditionally. RUNTIME-DEPENDENCY-
// ELIMINATION ticket 26/09/01.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn apply_reference_image_bytes(state: &mut World3dState, url: &str, bytes: &[u8]) {
    let reader = image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok();
    let Some(reader) = reader else {
        return;
    };
    if let Ok(image) = reader.decode() {
        let rgba = image.to_rgba8();
        if !publish_world_pixels(state, url.to_string(), (rgba.width(), rgba.height(), rgba.into_raw()), false) {
            return;
        }
        state.pending_image_urls.remove(url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_wgpu::wgpu::{SurfaceKind, UiComponentSceneNode, UiPresence, World3dScene};

    fn triangle_mesh_oracle() -> LegacyMeshOracleData {
        mesh_oracle_from_buffers(vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])
    }

    fn mesh_oracle_from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> LegacyMeshOracleData {
        LegacyMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
    }

    fn publish_oracle_mesh(data: LegacyMeshOracleData) -> Mesh3dLease {
        assert!(data.positions.len().is_multiple_of(3));
        assert_eq!(data.normals.len(), data.positions.len());
        assert!(data.edge_positions.len().is_multiple_of(6));
        assert!(data.uvs.len().is_multiple_of(2));
        assert!(data.colors.len().is_multiple_of(4));
        static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(10_000);
        let schema = Mesh3dSchema {
            vertices: (data.positions.len() / 3) as u32,
            indices: data.indices.len() as u32,
            face_ids: data.face_ids.len() as u32,
            vertex_ids: data.vertex_ids.len() as u32,
            edges: (data.edge_positions.len() / 6) as u32,
            edge_ids: data.edge_ids.len() as u32,
            uvs: (data.uvs.len() / 2) as u32,
            colors: (data.colors.len() / 4) as u32,
        };
        let generation = GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let token = mesh3d_begin(generation, 0, schema).expect("oracle mesh claim");
        while !mesh3d_allocate_step(token).expect("oracle mesh page allocation") {}
        for value in data.positions.as_chunks::<3>().0 {
            mesh3d_write_vec3(token, Mesh3dField::Positions, *value).unwrap();
        }
        for value in data.normals.as_chunks::<3>().0 {
            mesh3d_write_vec3(token, Mesh3dField::Normals, *value).unwrap();
        }
        for value in data.indices {
            mesh3d_write_u32(token, Mesh3dField::Indices, value).unwrap();
        }
        for value in data.face_ids {
            mesh3d_write_u32(token, Mesh3dField::FaceIds, value).unwrap();
        }
        for value in data.vertex_ids {
            mesh3d_write_u32(token, Mesh3dField::VertexIds, value).unwrap();
        }
        for value in data.edge_positions.as_chunks::<6>().0 {
            mesh3d_write_edge(token, [value[..3].try_into().unwrap(), value[3..].try_into().unwrap()]).unwrap();
        }
        for value in data.edge_ids {
            mesh3d_write_u32(token, Mesh3dField::EdgeIds, value).unwrap();
        }
        for value in data.uvs.as_chunks::<2>().0 {
            ui_wgpu::wgpu::mesh3d_write_vec2(token, Mesh3dField::Uvs, *value).unwrap();
        }
        for value in data.colors.as_chunks::<4>().0 {
            ui_wgpu::wgpu::mesh3d_write_vec4(token, Mesh3dField::Colors, *value).unwrap();
        }
        mesh3d_seal(token).expect("oracle mesh publication")
    }

    fn assert_send<T: Send>() {}

    fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(fuel, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        step(&mut context)
    }

    #[test]
    fn cursor_wake_coalesces_duplicates_and_rearms_after_exact_take() {
        let wake = WorldCursorWakeAuthority::new();
        let mut first_frame = World3dBuildContext::new(wake.clone());
        first_frame.request_cursor_wake();
        first_frame.request_cursor_wake();
        let first = first_frame.take_cursor_wake().unwrap().expect("first frame wake token");
        assert_eq!(first_frame.take_cursor_wake().unwrap(), None, "one frame carries one exact token");
        for _ in 0..128 {
            assert_eq!(wake.request().unwrap().generation(), first.generation(), "a pending wake storm coalesces onto one retained generation");
        }

        let mut consecutive_frame = World3dBuildContext::new(wake.clone());
        consecutive_frame.request_cursor_wake();
        let duplicate = consecutive_frame.take_cursor_wake().unwrap().expect("pending wake coalesces across frames");
        assert_eq!(duplicate.generation(), first.generation());
        assert!(wake.acknowledge(&first));
        assert!(!wake.acknowledge(&duplicate), "duplicate acknowledgement cannot consume another generation");

        let mut resumed_frame = World3dBuildContext::new(wake.clone());
        resumed_frame.request_cursor_wake();
        let resumed = resumed_frame.take_cursor_wake().unwrap().expect("resumed work receives another token");
        assert!(resumed.generation() > first.generation(), "consecutive presented frames retain an ABA-distinguishable generation");
        assert!(!wake.acknowledge(&duplicate), "stale token cannot consume the newer pending generation");
        assert_eq!(wake.pending_generation(), Some(resumed.generation()));
        assert!(wake.acknowledge(&resumed));
        assert_eq!(wake.pending_generation(), None);

        let pending = wake.request().expect("close fixture pending wake");
        assert_eq!(wake.pending_generation(), Some(pending.generation()));
        assert!(!wake.close_step(), "first close grant retires only the pending token scalar");
        assert!(!wake.close_step(), "second close grant retires only the generation scalar");
        assert!(!wake.close_step(), "third close grant retires only the acknowledgement scalar");
        assert!(wake.close_step());
        assert!(wake.terminal_is_empty());
        assert_eq!(wake.request(), Err(WorldCursorWakeFault::Closed));
    }

    #[test]
    fn live_renderer_retains_generation_wake_and_rejects_recreation_erasure_loss_and_duplicate_consumption() {
        const TOKEN_FIELD: &str = "cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>";
        const HOST_TOKEN_FIELD: &str = "cursor_wake_requested: Option<crate::infinite_world::world::WorldCursorWakeToken>";

        fn region<'a>(source: &'a str, start: &str, end: &str) -> Option<&'a str> {
            let start = source.find(start)?;
            let end = source[start..].find(end)?.checked_add(start)?;
            Some(&source[start..end])
        }

        fn replace_region(source: &str, start: &str, end: &str, from: &str, to: &str) -> Option<String> {
            let start = source.find(start)?;
            let end = source[start..].find(end)?.checked_add(start)?;
            let owned = source[start..end].replacen(from, to, 1);
            let mut output = String::with_capacity(source.len().saturating_add(to.len()).saturating_sub(from.len()));
            output.push_str(&source[..start]);
            output.push_str(&owned);
            output.push_str(&source[end..]);
            Some(output)
        }

        fn exact(glue: &str, host: &str, native: &str, browser: &str) -> bool {
            let typed_handoffs = [
                ("AppFrameBuild", "pub(crate) struct AppFrameBuild {", "struct AppFrameAfterChrome {"),
                ("AppFrameAfterChrome", "struct AppFrameAfterChrome {", "struct FrameWheelCursor {"),
                ("AppFramePresentation", "pub(crate) struct AppFramePresentation {", "impl AppFrameBuild {"),
                ("AppFramePreparation", "pub(crate) struct AppFramePreparation {", "impl AppFramePreparation {"),
                ("AppPresentStep::Complete", "pub(crate) enum AppPresentStep {", "impl AppPresenter {"),
            ];
            let host_handoffs = [("OsHost", "pub struct OsHost {", "pub(crate) struct OsHostRetirement {"), ("OsHostRetirement", "pub(crate) struct OsHostRetirement {", "impl OsHost {")];
            glue.contains("world_cursor_wake: infinite_world::world::WorldCursorWakeAuthority")
                && glue.contains("World3dBuildContext::new(runtime.world_cursor_wake_authority())")
                && glue.matches(TOKEN_FIELD).count() == typed_handoffs.len()
                && typed_handoffs.iter().all(|(_, start, end)| region(glue, start, end).is_some_and(|handoff| handoff.matches(TOKEN_FIELD).count() == 1))
                && region(glue, "pub(crate) enum AppPresentStep {", "impl AppPresenter {")
                    .is_some_and(|handoff| handoff.contains("Complete { fullscreen: Option<bool>, cursor_wake: Option<infinite_world::world::WorldCursorWakeToken> }") && handoff.matches(TOKEN_FIELD).count() == 1)
                && region(glue, "pub(crate) struct AppPresenter {", "#[derive(Clone, Copy, Debug, PartialEq, Eq)]").is_some_and(|presenter| presenter.matches("pending: Option<AppPresentCursor>").count() == 1)
                && region(glue, "struct AppPresentCursor {", "pub(crate) enum AppPresentStep {").is_some_and(|presenter| presenter.matches("frame: AppFramePresentation").count() == 1)
                && !glue.contains("cursor_wake: bool")
                && !glue.contains("cursor_wake: Option<bool>")
                && !glue.contains("request_frame: bool")
                && glue.contains("fn close_world_cursor_wake_step(&self) -> bool")
                && glue.contains("pub(crate) fn close_cursor_wake_step(&mut self) -> bool")
                && !glue.contains("World3dBuildContext::default()")
                && host.matches(HOST_TOKEN_FIELD).count() == host_handoffs.len()
                && host_handoffs.iter().all(|(_, start, end)| region(host, start, end).is_some_and(|handoff| handoff.matches(HOST_TOKEN_FIELD).count() == 1))
                && !host.contains("cursor_wake_requested: bool")
                && !host.contains("cursor_wake_requested: Option<bool>")
                && host.contains("token.generation() > pending.generation()")
                && host.contains("if self.cursor_wake_requested.take().is_some()")
                && native.contains("self.runtime.acknowledge_world_cursor_wake(&token)")
                && native.contains("self.retain_cursor_wake_directive(token)")
                && native.matches("host.take_cursor_wake_directive()").count() == 1
                && browser.contains("host.take_cursor_wake_directive().is_some()")
        }

        let glue = include_str!("../../📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
        let host = include_str!("../../📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs");
        let native = include_str!("../../📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs");
        let browser = include_str!("../../📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧵️browser-worker/🦀️.rs");
        assert!(exact(glue, host, native, browser));
        assert!(!exact(&glue.replace("World3dBuildContext::new(runtime.world_cursor_wake_authority())", "World3dBuildContext::default()"), host, native, browser));
        for (name, start, end) in [
            ("AppFrameBuild", "pub(crate) struct AppFrameBuild {", "struct AppFrameAfterChrome {"),
            ("AppFrameAfterChrome", "struct AppFrameAfterChrome {", "struct FrameWheelCursor {"),
            ("AppFramePresentation", "pub(crate) struct AppFramePresentation {", "impl AppFrameBuild {"),
            ("AppFramePreparation", "pub(crate) struct AppFramePreparation {", "impl AppFramePreparation {"),
            ("AppPresentStep::Complete", "pub(crate) enum AppPresentStep {", "impl AppPresenter {"),
        ] {
            let erased = replace_region(glue, start, end, TOKEN_FIELD, "request_frame: bool").expect("enumerated typed wake handoff");
            assert!(!exact(&erased, host, native, browser), "erasing {name} must violate the exact typed-token handoff census");
        }
        let erased_all = glue.replace(TOKEN_FIELD, "request_frame: bool");
        assert!(!exact(&erased_all, host, native, browser), "erasing every internal typed wake handoff must be rejected");
        let extra_bool = glue.replacen("pub(crate) struct AppFrameBuild {", "pub(crate) struct AppFrameBuild {\n    request_frame: bool,", 1);
        assert!(!exact(&extra_bool, host, native, browser), "an extra internal boolean wake channel must be rejected");
        let presenter_erased = glue.replacen("frame: AppFramePresentation", "request_frame: bool", 1);
        assert!(!exact(&presenter_erased, host, native, browser), "presenter ownership must retain the typed presentation handoff");
        let pending_presenter_erased = glue.replacen("pending: Option<AppPresentCursor>", "request_frame: bool", 1);
        assert!(!exact(&pending_presenter_erased, host, native, browser), "AppPresenter must retain its typed pending cursor");
        for (name, start, end) in [("OsHost", "pub struct OsHost {", "pub(crate) struct OsHostRetirement {"), ("OsHostRetirement", "pub(crate) struct OsHostRetirement {", "impl OsHost {")] {
            let erased = replace_region(host, start, end, HOST_TOKEN_FIELD, "cursor_wake_requested: bool").expect("enumerated host token field");
            assert!(!exact(glue, &erased, native, browser), "erasing {name}'s retained host token must be rejected");
        }
        assert!(!exact(glue, host, &native.replace("self.runtime.acknowledge_world_cursor_wake(&token)", "true"), browser));
        assert!(!exact(glue, host, &native.replace("self.retain_cursor_wake_directive(token)", "drop(token)"), browser));
        assert!(!exact(glue, &host.replace("token.generation() > pending.generation()", "true"), native, browser));
        assert!(!exact(glue, &host.replace("if self.cursor_wake_requested.take().is_some()", "if false"), native, browser));
        assert!(!exact(glue, host, native, &browser.replace("host.take_cursor_wake_directive().is_some()", "host.cursor_wake_requested.is_some()")));
    }

    #[test]
    fn terrain_visibility_requires_the_complete_family_marker() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let tile = (3, 4, 5);
        assert!(!terrain_family_visible(&state, tile));
        state.terrain_built_tiles.insert(tile);
        assert!(terrain_family_visible(&state, tile));
    }

    #[test]
    fn placeholder_writer_matches_legacy_geometry_and_closes_interrupted_authority() {
        for (name, kind) in [("box", WorldPlaceholderKind::Box), ("plane", WorldPlaceholderKind::Plane), ("cylinder", WorldPlaceholderKind::Cylinder), ("cone", WorldPlaceholderKind::Cone), ("vortex-marker", WorldPlaceholderKind::Icosphere)] {
            let legacy = placeholder_mesh(name);
            let mut positions = Vec::new();
            let mut normals = Vec::new();
            let mut indices = Vec::new();
            for triangle in 0..kind.triangles() {
                let vertices = placeholder_triangle(kind, triangle);
                let normal = placeholder_triangle_normal(vertices);
                for vertex in vertices {
                    positions.extend_from_slice(&vertex);
                    normals.extend_from_slice(&normal);
                    indices.push(indices.len() as u32);
                }
            }
            assert_eq!(positions, legacy.positions);
            assert_eq!(indices, legacy.indices);
            for (actual, expected) in normals.iter().zip(&legacy.normals) {
                assert!((actual - expected).abs() <= 1e-6, "{name} normal mismatch: {actual} != {expected}");
            }
        }

        let mut interrupted = WorldPlaceholderMeshCursor::new("interrupted", WorldPlaceholderKind::Icosphere, 41, 7).expect("placeholder authority");
        assert!(matches!(interrupted.step(), WorldPlaceholderMeshStep::Pending));
        assert!(matches!(interrupted.step(), WorldPlaceholderMeshStep::Pending));
        let mut turns = 0;
        while !interrupted.close_step() || !interrupted.terminal_is_empty() {
            turns += 1;
            assert!(turns < 64);
        }
        assert!(interrupted.terminal_is_empty());
    }

    #[test]
    fn terrain_writer_matches_legacy_bands_and_closes_interrupted_authority() {
        fn payload() -> TerrainTileMeshPayload {
            TerrainTileMeshPayload {
                positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 3.0, 0.0, 0.0, 2.0, 1.0, 0.0],
                normals: [0.0, 0.0, 1.0].repeat(6),
                indices: vec![0, 1, 2, 3, 4, 5],
                uvs: vec![0.0, 0.05, 0.0, 0.05, 0.0, 0.05, 0.0, 0.95, 0.0, 0.95, 0.0, 0.95],
            }
        }

        let mut cursor = WorldTerrainMeshCursor::new("surface", 3, 4, 5, payload(), 50, 9, 9).expect("terrain cursor");
        let mut turns = 0;
        loop {
            turns += 1;
            assert!(turns < 512);
            match cursor.step(9, 9) {
                WorldTerrainMeshStep::Pending => {}
                WorldTerrainMeshStep::Ready(key, lease) => {
                    let band = key.rsplit(':').next().and_then(|value| value.parse::<usize>().ok()).expect("band key");
                    let legacy = build_terrain_band_mesh(&payload(), band, TERRAIN_COLOR_BANDS).expect("published band has legacy geometry");
                    let schema = lease.schema().expect("terrain lease schema");
                    assert_eq!(schema.vertices as usize * 3, legacy.positions.len());
                    assert_eq!(schema.indices as usize, legacy.indices.len());
                    for item in 0..schema.vertices {
                        assert_eq!(lease.vec3(Mesh3dField::Positions, item).unwrap(), legacy.positions[item as usize * 3..item as usize * 3 + 3]);
                        assert_eq!(lease.vec3(Mesh3dField::Normals, item).unwrap(), legacy.normals[item as usize * 3..item as usize * 3 + 3]);
                    }
                    for item in 0..schema.indices {
                        assert_eq!(lease.u32(Mesh3dField::Indices, item).unwrap(), legacy.indices[item as usize]);
                    }
                    mesh3d_begin_close(lease).unwrap();
                    while !mesh3d_close_step(lease).unwrap() {}
                }
                WorldTerrainMeshStep::Complete(tile) => {
                    assert_eq!(tile, (3, 4, 5));
                    break;
                }
                WorldTerrainMeshStep::Fault => panic!("valid terrain cursor faulted"),
            }
        }
        assert!(cursor.terminal_is_empty());

        let mut interrupted = WorldTerrainMeshCursor::new("surface", 3, 4, 5, payload(), 70, 11, 11).expect("terrain cursor");
        assert!(matches!(interrupted.step(11, 11), WorldTerrainMeshStep::Pending));
        let mut close_turns = 0;
        while !interrupted.close_step() || !interrupted.terminal_is_empty() {
            close_turns += 1;
            assert!(close_turns < 64);
        }
        assert!(interrupted.terminal_is_empty());
        assert!(WorldTerrainMeshCursor::new("surface", 0, 0, 0, TerrainTileMeshPayload { positions: vec![0.0, 1.0], normals: Vec::new(), indices: vec![0, 1, 2], uvs: Vec::new() }, 80, 1, 1).is_err());
    }

    fn face_overlay_test_mesh_with_faces(generation: u64, revision: u64, face_ids: &[u32]) -> Mesh3dLease {
        let triangles = u32::try_from(face_ids.len()).expect("fixture triangle count");
        let vertices = triangles * 3;
        let schema = Mesh3dSchema { vertices, indices: vertices, face_ids: triangles, vertex_ids: 0, edges: 0, edge_ids: 0, uvs: 0, colors: 0 };
        let token = mesh3d_begin(generation, revision, schema).expect("face overlay fixture claim");
        while !mesh3d_allocate_step(token).expect("face overlay fixture page") {}
        for triangle in 0..triangles {
            let x = triangle as f32 * 2.0;
            for position in [[x, 0.0, 0.0], [x + 1.0, 0.0, 0.0], [x, 1.0, 0.0]] {
                mesh3d_write_vec3(token, Mesh3dField::Positions, position).unwrap();
                mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0]).unwrap();
            }
        }
        for index in 0..vertices {
            mesh3d_write_u32(token, Mesh3dField::Indices, index).unwrap();
        }
        for face_id in face_ids {
            mesh3d_write_u32(token, Mesh3dField::FaceIds, *face_id).unwrap();
        }
        mesh3d_seal(token).expect("face overlay fixture lease")
    }

    fn face_overlay_test_mesh(generation: u64, revision: u64, face_id: u32) -> Mesh3dLease {
        face_overlay_test_mesh_with_faces(generation, revision, &[face_id])
    }

    #[test]
    fn face_overlay_writer_matches_legacy_winding_and_retires_stale_generation() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 7;
        state.draw_generation = 11;
        state.granularity = "face".into();
        state.component_ids.push("12".into());
        let source = face_overlay_test_mesh(400, 7, 12);
        publish_world3d_mesh_lease(&mut state, "source".into(), source).unwrap();
        let version = state.mesh_versions["source"];
        state.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] }).unwrap();

        let mut cursor = WorldFaceOverlayMeshCursor::new("surface", 500, 7, 11).unwrap();
        let mut published = None;
        for _ in 0..128 {
            match cursor.step(&state) {
                WorldFaceOverlayMeshStep::Pending => {}
                WorldFaceOverlayMeshStep::Ready { index, color, key, lease } => {
                    assert_eq!((index, color, key.as_str()), (0, [0.35, 0.75, 1.0, 0.62], "component-face-overlay:surface:500:0"));
                    let schema = lease.schema().unwrap();
                    assert_eq!((schema.vertices, schema.indices), (3, 6));
                    assert_eq!(lease.vec3(Mesh3dField::Positions, 0).unwrap(), [0.0, 0.0, FACE_OVERLAY_OFFSET * 0.5]);
                    assert_eq!(lease.vec3(Mesh3dField::Positions, 1).unwrap(), [1.0, 0.0, FACE_OVERLAY_OFFSET * 0.5]);
                    assert_eq!(lease.vec3(Mesh3dField::Positions, 2).unwrap(), [0.0, 1.0, FACE_OVERLAY_OFFSET * 0.5]);
                    assert_eq!((0..6).map(|item| lease.u32(Mesh3dField::Indices, item).unwrap()).collect::<Vec<_>>(), vec![0, 1, 2, 0, 2, 1]);
                    published = Some(lease);
                    cursor.published_colors[index] = Some(color);
                }
                WorldFaceOverlayMeshStep::Complete { generation, revision, draw_generation, colors } => {
                    assert_eq!((generation, revision, draw_generation), (500, 7, 11));
                    assert_eq!(colors, [Some([0.35, 0.75, 1.0, 0.62]), None, None]);
                    break;
                }
                WorldFaceOverlayMeshStep::Stale | WorldFaceOverlayMeshStep::Fault => panic!("valid overlay fixture faulted"),
            }
        }
        assert!(cursor.terminal_is_empty());
        let published = published.expect("selected overlay published");
        mesh3d_begin_close(published).unwrap();
        while !mesh3d_close_step(published).unwrap() {}

        let mut stale = WorldFaceOverlayMeshCursor::new("surface", 600, 7, 11).unwrap();
        assert!(matches!(stale.step(&state), WorldFaceOverlayMeshStep::Pending));
        state.interaction_revision = 8;
        let mut terminal = false;
        for _ in 0..32 {
            if matches!(stale.step(&state), WorldFaceOverlayMeshStep::Stale) {
                terminal = true;
                break;
            }
        }
        assert!(terminal && stale.terminal_is_empty());
        assert!(WorldFaceOverlayMeshCursor::new(&"x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY), 700, 1, 1).is_err());
    }

    #[test]
    fn face_overlay_family_becomes_visible_only_after_every_bucket_is_published() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 7;
        state.draw_generation = 11;
        state.granularity = "face".into();
        state.marquee_preview_ids.push("10".into());
        state.component_ids.push("12".into());
        state.hovered_component_id = Some("11".into());
        state.hovered_component_object_id = Some("object".into());
        state.hovered_component_mode = Some("face".into());
        let source = face_overlay_test_mesh_with_faces(750, 7, &[10, 11, 12]);
        publish_world3d_mesh_lease(&mut state, "source".into(), source).unwrap();
        let version = state.mesh_versions["source"];
        state.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] }).unwrap();
        state.face_overlay_build = Some(WorldFaceOverlayMeshCursor::new("surface", 800, 7, 11).unwrap());

        let mut staged_seen = false;
        for _ in 0..1_024 {
            step_component_face_overlay_build(&mut state);
            if state.meshes.contains_key("component-face-overlay:surface:800:0") && state.face_overlay_generation.is_none() {
                staged_seen = true;
            }
            if state.face_overlay_generation == Some(800) {
                break;
            }
        }
        assert!(staged_seen, "a completed bucket remains invisible while the family transaction is partial");
        assert_eq!(state.face_overlay_generation, Some(800));
        assert_eq!(state.face_overlay_colors, [Some([1.0, 0.85, 0.35, 0.36]), Some([0.35, 0.75, 1.0, 0.48]), Some([0.35, 0.75, 1.0, 0.62])]);
        for index in 0..3 {
            assert!(state.meshes.contains_key(&format!("component-face-overlay:surface:800:{index}")), "every nonempty category publishes before family visibility");
        }

        state.face_overlay_build = Some(WorldFaceOverlayMeshCursor::new("surface", 900, 7, 11).unwrap());
        for _ in 0..1_024 {
            step_component_face_overlay_build(&mut state);
            if state.meshes.contains_key("component-face-overlay:surface:900:0") {
                break;
            }
        }
        assert_eq!(state.face_overlay_generation, Some(800), "superseding partial family never changes the visible generation");
        state.interaction_revision = 8;
        for _ in 0..64 {
            step_component_face_overlay_build(&mut state);
            if state.face_overlay_build.is_none() {
                break;
            }
        }
        assert_eq!(state.face_overlay_generation, Some(800), "stale family retirement preserves the last complete publication");
        assert!(state.meshes.contains_key("component-face-overlay:surface:900:0"), "stale partial publication remains owned by the generation-qualified registry until bounded retirement");
    }

    #[test]
    fn world_flat_plan_enforces_exact_item_and_byte_caps() {
        let mut bytes = WorldInteractionPlan::new(1, 1);
        assert!(bytes.push_string(&"x".repeat(WORLD_INTERACTION_BYTE_CAPACITY)).is_some());
        assert!(bytes.push_string("x").is_none());
        assert!(bytes.faulted);

        let mut items = WorldInteractionPlan::new(1, 1);
        let action = WorldFlatAction { kind: WorldFlatActionKind::Camera, strings: [None; 8], numbers: [0.0; 10], number_len: 0 };
        for _ in 0..WORLD_INTERACTION_ITEM_CAPACITY {
            assert!(items.push_action(action));
        }
        assert!(!items.push_action(action));
        assert!(items.faulted);
    }

    #[test]
    fn world_mesh_registry_enforces_fixed_capacity_id_topology_and_aba() {
        let mesh = publish_oracle_mesh(triangle_mesh_oracle());
        let mut registry = WorldInteractionMeshRegistry::default();
        let first = registry.admit("mesh-0", 1, mesh).expect("first mesh token");
        assert_eq!(registry.admit("mesh-0", 1, mesh), Some(first));
        let replacement = registry.admit("mesh-0", 2, mesh).expect("replacement token");
        assert_ne!(replacement, first);
        assert!(registry.resolve(first).is_none());
        assert_eq!(registry.resolve(replacement).map(|slot| slot.version), Some(2));
        for index in 1..WORLD_INTERACTION_MESH_CAPACITY {
            assert!(registry.admit(&format!("mesh-{index}"), 1, mesh).is_some());
        }
        assert!(registry.admit("mesh-overflow", 1, mesh).is_none());
        assert!(registry.faulted);

        let mut oversized = WorldInteractionMeshRegistry::default();
        assert!(oversized.admit(&"x".repeat(WORLD_INTERACTION_ID_BYTE_CAPACITY + 1), 1, mesh).is_none());
        assert!(oversized.faulted);
        assert_eq!(mesh3d_begin(99, 0, Mesh3dSchema::triangle_mesh(0, 0)), Err(ui_wgpu::wgpu::Mesh3dFault::Schema));
    }

    #[test]
    fn world_object_registry_enforces_capacity_revision_and_aba() {
        let mut registry = WorldInteractionObjectRegistry::default();
        let first = registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [0.0; 8]).expect("first object token");
        assert_eq!(registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [0.0; 8]), Some(first));
        let replacement = registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [1.0; 8]).expect("same-revision replacement");
        assert_ne!(replacement, first);
        assert!(registry.resolve(first).is_none());
        let next_revision = registry.admit(2, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [1.0; 8]).expect("new revision replacement");
        assert_ne!(next_revision, replacement);
        assert!(registry.resolve(replacement).is_none());
        for index in 1..WORLD_INTERACTION_OBJECT_CAPACITY {
            assert!(registry.admit(2, WorldInteractionObjectKind::Instance, &format!("object-{index}"), None, Mat4::identity(), [index as f32; 8]).is_some());
        }
        assert!(registry.admit(2, WorldInteractionObjectKind::Instance, "object-overflow", None, Mat4::identity(), [0.0; 8]).is_none());
        assert!(registry.faulted);

        let mut oversized = WorldInteractionObjectRegistry::default();
        assert!(oversized.admit(1, WorldInteractionObjectKind::Reference, &"x".repeat(WORLD_INTERACTION_ID_BYTE_CAPACITY + 1), None, Mat4::identity(), [0.0; 8]).is_none());
        assert!(oversized.faulted);
    }

    #[test]
    fn world_object_registry_build_is_one_owner_per_turn_and_interruptible() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 7;
        let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]));
        store_mesh(&mut state, "mesh".into(), mesh);
        let mesh_version = state.mesh_versions["mesh"];
        state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { id: "instance".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] });
        state.vortices.push(WorldVortexRecord { full_id: "vortex".into(), position: Some([1.0, 2.0, 3.0]), radius: Some(0.5), ..Default::default() });
        state.references.push(WorldReferenceRecord { url: Some("reference".into()), origin: Some([4.0, 5.0, 6.0]), width_world: Some(2.0), hidden: Some(false) });
        state.reference_pixels.insert("reference".into(), (2, 1, Vec::new()));

        let mut cursor = WorldInteractionRegistryBuildCursor::new(7);
        assert_eq!(with_world_step_context(0, |context| cursor.step(&mut state, context)), WorldInteractionStep::Pending);
        let mut turns = 0;
        while with_world_step_context(1, |context| cursor.step(&mut state, context)) == WorldInteractionStep::Pending {
            turns += 1;
            assert!(turns < 16);
        }
        assert!(state.interaction_objects.terminal_for_revision(7));
        let live: Vec<_> = state.interaction_objects.slots.iter().flatten().filter(|slot| slot.revision == 7).collect();
        assert_eq!(live.len(), 3);
        assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Instance && slot.id.as_str() == "instance"));
        assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Vortex && slot.id.as_str() == "vortex"));
        assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Reference && slot.id.as_str() == "reference"));

        let token = state
            .interaction_objects
            .slots
            .iter()
            .enumerate()
            .find_map(|(index, slot)| slot.as_ref().filter(|slot| slot.kind == WorldInteractionObjectKind::Reference).map(|slot| WorldInteractionObjectToken { slot: index as u16, generation: slot.generation, revision: slot.revision }))
            .expect("reference token");
        assert_eq!(state.interaction_objects.resolve(token).map(|slot| slot.values[4]), Some(1.0));

        let mut interrupted = WorldInteractionRegistryBuildCursor::new(8);
        assert!(with_world_step_context(1, |context| interrupted.close_step(context)));
        assert_eq!(interrupted.phase, WorldInteractionRegistryBuildPhase::Complete);
    }

    #[test]
    fn world_vortex_and_reference_pick_cursors_resume_revalidate_and_close() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 3;
        state.interaction_objects.revision = 3;
        let vortex = state.interaction_objects.admit(3, WorldInteractionObjectKind::Vortex, "vortex", None, Mat4::identity(), [0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.5, 0.0]).expect("vortex token");
        state.interaction_objects.admit(3, WorldInteractionObjectKind::Reference, "reference", None, Mat4::identity(), [0.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 0.0]).expect("reference token");

        let mut vortex_cursor = WorldObjectPickCursor::from_ray(3, 8, WorldObjectPickPurpose::VortexSelect, Vec3::new(0.0, -2.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(with_world_step_context(0, |context| vortex_cursor.step(&state, 8, context)), WorldInteractionStep::Pending);
        for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
            if with_world_step_context(1, |context| vortex_cursor.step(&state, 8, context)) == WorldInteractionStep::Complete {
                break;
            }
        }
        assert_eq!(vortex_cursor.best.map(|(token, _)| token), Some(vortex));
        let mut plan = vortex_cursor.finish_plan(&state, 8).expect("vortex plan").expect("vortex hit");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        let actions = input.drain_events();
        assert_eq!(actions.len(), 1);

        state.interaction_objects.revision = state.interaction_revision;
        let mut reference_cursor = WorldObjectPickCursor::from_ray(state.interaction_revision, 9, WorldObjectPickPurpose::ReferenceHover, Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
        for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
            if with_world_step_context(1, |context| reference_cursor.step(&state, 9, context)) == WorldInteractionStep::Complete {
                break;
            }
        }
        assert!(reference_cursor.best.is_some());
        state.interaction_revision = state.interaction_revision.wrapping_add(1);
        assert!(matches!(reference_cursor.finish_plan(&state, 9), Err(WorldInteractionStep::Stale)));
        assert!(!reference_cursor.close_step());
        assert!(reference_cursor.close_step());
    }

    #[test]
    fn world_component_cursor_steps_one_topology_item_and_rejects_aba() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 5;
        state.interaction_objects.revision = 5;
        state.granularity = "face".into();
        let mesh = publish_oracle_mesh(triangle_mesh_oracle());
        store_mesh(&mut state, "mesh".into(), mesh);
        let mesh_token = state
            .interaction_meshes
            .slots
            .iter()
            .enumerate()
            .find_map(|(index, slot)| slot.as_ref().filter(|slot| slot.id.as_str() == "mesh").map(|slot| WorldInteractionMeshToken { slot: index as u16, generation: slot.generation }))
            .expect("mesh token");
        let object = state.interaction_objects.admit(5, WorldInteractionObjectKind::Instance, "object", Some(mesh_token), Mat4::identity(), [0.0; 8]).expect("object token");
        let mut cursor = WorldComponentPickCursor {
            revision: 5,
            generation: 10,
            purpose: WorldComponentPickPurpose::Select,
            kind: WorldComponentKind::Face,
            local_x: 0.0,
            local_y: 0.0,
            viewport: Rect { x: 0.0, y: 0.0, w: 100.0, h: 100.0 },
            view_projection: Mat4::identity(),
            origin: Vec3::new(0.0, 0.0, 2.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
            slot: object.slot,
            current: None,
            topology: 0,
            merge: 0,
            best: None,
            complete: false,
        };
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 10, context)), WorldInteractionStep::Pending);
        assert_eq!(cursor.current, Some(object));
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 10, context)), WorldInteractionStep::Pending);
        assert_eq!(cursor.topology, 1);
        assert_eq!(cursor.best.map(|hit| hit.id), Some(0));
        cursor.current = None;
        cursor.slot = WORLD_INTERACTION_OBJECT_CAPACITY as u16;
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 10, context)), WorldInteractionStep::Pending);
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 10, context)), WorldInteractionStep::Complete);
        let mut plan = cursor.finish_plan(&state, 10).expect("component plan").expect("component hit");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        assert_eq!(input.drain_events().len(), 1);

        state.interaction_objects.revision = state.interaction_revision;
        let mut replacement_model = Mat4::identity();
        replacement_model.cols[3][0] = 1.0;
        let replacement = state.interaction_objects.admit(state.interaction_revision, WorldInteractionObjectKind::Instance, "object", Some(mesh_token), replacement_model, [0.0; 8]).expect("replacement object token");
        assert_ne!(replacement, object);
        cursor.revision = state.interaction_revision;
        cursor.complete = true;
        cursor.best = Some(WorldComponentHit { object, id: 0, primary: 1.0, secondary: 0.0 });
        assert!(matches!(cursor.finish_plan(&state, 10), Err(WorldInteractionStep::Stale)));
        assert!(!cursor.close_step());
        assert!(cursor.close_step());
    }

    #[test]
    fn world_context_menu_cursor_is_revisioned_and_right_drag_suppresses_publication() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 4;
        state.interaction_objects.revision = 4;
        state.hovered_vortex_id = Some("vortex".into());
        let token = state.interaction_objects.admit(4, WorldInteractionObjectKind::Vortex, "vortex", None, Mat4::identity(), [0.0; 8]).expect("vortex token");
        let mut cursor = WorldContextMenuCursor::new(&state, 11, 12.0, 13.0).expect("context cursor");
        cursor.slot = token.slot;
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 11, context)), WorldInteractionStep::Pending);
        assert_eq!(cursor.target, Some(token));
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 11, context)), WorldInteractionStep::Complete);
        let mut plan = cursor.finish_plan(&state, 11).expect("context plan").expect("context target");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 11, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        assert_eq!(input.drain_events().len(), 1);

        let mut authority = WorldInteractionAuthority::default();
        authority.next_generation = 4;
        authority.queue.push(WorldInteractionIntent::pointer_button(0.0, 0.0, true, 2, &PointerModifiers::default())).unwrap();
        authority.queue.slots[0].as_mut().unwrap().generation = 1;
        authority.queue.push(WorldInteractionIntent::pointer_move(10.0, 0.0, 10.0, 0.0, true, 2, &PointerModifiers::default())).unwrap();
        authority.queue.slots[1].as_mut().unwrap().generation = 2;
        authority.queue.push(WorldInteractionIntent::pointer_button(10.0, 0.0, false, 2, &PointerModifiers::default())).unwrap();
        authority.queue.slots[2].as_mut().unwrap().generation = 3;
        state.interaction_authority = Some(authority);
        state.interaction_objects.revision = state.interaction_revision;
        for generation in 1..=3 {
            assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context)), WorldInteractionAuthorityStep::Complete);
        }
        assert!(input.drain_events().is_empty());
    }

    #[test]
    fn world_marquee_gesture_has_fixed_points_generation_and_cursorized_close() {
        let mut gesture = WorldMarqueeGesture::new(7, 11, [0.0, 0.0]);
        for index in 1..WORLD_INTERACTION_MARQUEE_POINT_CAPACITY {
            assert!(gesture.push([index as f32, 0.0]));
        }
        assert!(!gesture.push([999.0, 0.0]));
        assert!(!gesture.is_click([10.0, 0.0]));
        let mut turns = 0;
        while !gesture.close_step() {
            turns += 1;
            assert!(turns <= WORLD_INTERACTION_MARQUEE_POINT_CAPACITY);
        }
        assert_eq!(turns, WORLD_INTERACTION_MARQUEE_POINT_CAPACITY);
        assert_eq!(gesture.len, 0);
        assert!(gesture.points.iter().all(Option::is_none));
    }

    #[test]
    fn world_marquee_click_retires_points_before_exact_release_retry() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_objects.revision = state.interaction_revision;
        let modifiers = PointerModifiers::default();
        let mut down = WorldInteractionIntent::pointer_button(0.0, 0.0, true, 0, &modifiers);
        down.generation = 1;
        let mut up = WorldInteractionIntent::pointer_button(1.0, 1.0, false, 0, &modifiers);
        up.generation = 2;
        state.interaction_authority.as_mut().unwrap().queue.push(down).unwrap();
        state.interaction_authority.as_mut().unwrap().queue.push(up).unwrap();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete);
        assert_eq!(state.interaction_authority.as_ref().unwrap().marquee.as_ref().map(|gesture| gesture.len), Some(1));
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
        assert!(state.interaction_authority.as_ref().unwrap().marquee.as_ref().is_some_and(|gesture| gesture.retiring));
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Stale);
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Stale);
        assert!(state.interaction_authority.as_ref().unwrap().marquee.is_none());
        assert_eq!(world3d_interaction_front_generation(&state), Some(2));
    }

    #[test]
    fn world_marquee_result_pages_admit_exact_capacity_and_retire_one_target_per_grant() {
        let token = WorldInteractionObjectToken { slot: 0, generation: 1, revision: 1 };
        let mut pages = WorldMarqueeResultPages::default();
        for _ in 0..WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT {
            assert!(pages.push(WorldMarqueeResult::Object(token), 1));
        }
        assert_eq!(pages.page_len as usize, WORLD_MARQUEE_RESULT_PAGE_COUNT);
        assert!(!pages.push(WorldMarqueeResult::Object(token), 1));
        let mut turns = 0;
        while !pages.close_step() {
            turns += 1;
            assert!(turns <= WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT + WORLD_MARQUEE_RESULT_PAGE_COUNT);
        }
        assert_eq!(turns, WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT + WORLD_MARQUEE_RESULT_PAGE_COUNT);
        assert_eq!(pages.page_len, 0);
    }

    #[test]
    fn world_marquee_pages_build_one_target_field_per_grant_and_publish_atomically_fifo() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 6;
        state.interaction_objects.revision = 6;
        let mut results = WorldMarqueeResultPages::default();
        for index in 0..=WORLD_MARQUEE_RESULT_PAGE_CAPACITY {
            let id = format!("target-{index:03}");
            let token = state.interaction_objects.admit(6, WorldInteractionObjectKind::Instance, &id, None, Mat4::identity(), [0.0; 8]).expect("marquee result token");
            assert!(results.push(WorldMarqueeResult::Object(token), id.len()));
        }
        let mut gesture = WorldMarqueeGesture::new(6, 7, [0.0, 0.0]);
        assert!(gesture.push([100.0, 100.0]));
        let mut job = WorldMarqueePublishJob::new(8, gesture, results, false, false);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let mut turns = 0;
        loop {
            turns += 1;
            let step = with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).expect("bounded marquee page step");
            if step == WorldInteractionStep::Complete {
                break;
            }
            assert!(turns < 400);
        }
        let actions = input.drain_events();
        assert_eq!(actions.len(), 2);
        assert!(matches!(actions[0].args.as_ref().and_then(|args| args.get("targets")), Some(dsl::DslValue::Array(values)) if values.len() == WORLD_MARQUEE_RESULT_PAGE_CAPACITY));
        assert!(matches!(actions[1].args.as_ref().and_then(|args| args.get("targets")), Some(dsl::DslValue::Array(values)) if values.len() == 1));
        assert_eq!(job.results.page_len, 0);
        assert_eq!(job.gesture.len, 0);
    }

    #[test]
    fn world_marquee_page_claim_saturation_preserves_all_results_for_exact_retry() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 6;
        state.interaction_objects.revision = 6;
        let id = "target";
        let token = state.interaction_objects.admit(6, WorldInteractionObjectKind::Instance, id, None, Mat4::identity(), [0.0; 8]).expect("target token");
        let mut results = WorldMarqueeResultPages::default();
        assert!(results.push(WorldMarqueeResult::Object(token), id.len()));
        let mut gesture = WorldMarqueeGesture::new(6, 7, [0.0, 0.0]);
        assert!(gesture.push([100.0, 100.0]));
        let mut job = WorldMarqueePublishJob::new(8, gesture, results, false, false);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let mut blockers = Vec::new();
        while let Ok(claim) = input.claim_action(1) {
            blockers.push(claim);
        }
        assert!(matches!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)));
        assert!(job.prepared.is_none());
        assert_eq!(job.results.lens[0], 1);
        input.release_action_claim(blockers.pop().expect("retry claim")).expect("release retry claim");
        assert_eq!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        assert!(job.prepared.is_some());
        while !job.close_step(&mut input) {}
        for claim in blockers {
            input.release_action_claim(claim).expect("release blocker");
        }
        assert!(input.drain_events().is_empty());
        assert_eq!(job.results.page_len, 0);
        assert_eq!(job.gesture.len, 0);
    }

    fn world_marquee_geometry_fixture(instance_count: usize) -> World3dState {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.pick_bounds = state.bounds;
        state.interaction_revision = 2;
        let mesh = publish_oracle_mesh(triangle_mesh_oracle());
        store_mesh(&mut state, "mesh".into(), mesh);
        let mesh_version = state.mesh_versions["mesh"];
        let instances = (0..instance_count).map(|index| Instance3d { id: format!("object-{index:03}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }).collect();
        state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances });
        let mut registry = WorldInteractionRegistryBuildCursor::new(2);
        let mut turns = 0;
        while with_world_step_context(1, |context| registry.step(&mut state, context)) == WorldInteractionStep::Pending {
            turns += 1;
            assert!(turns < instance_count + 16);
        }
        assert!(state.interaction_objects.terminal_for_revision(2));
        state
    }

    fn world_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<String> {
        let mut gesture = WorldMarqueeGesture::new(state.interaction_revision, 7, points[0]);
        for point in points.iter().skip(1) {
            assert!(gesture.push(*point));
        }
        let mut cursor = WorldMarqueePickCursor::new(state, 8, gesture).expect("marquee cursor");
        let mut turns = 0;
        loop {
            turns += 1;
            if with_world_step_context(1, |context| cursor.step(state, 8, context)) == WorldInteractionStep::Complete {
                break;
            }
            assert!(turns < 8_000);
        }
        let mut ids = Vec::new();
        for page in 0..usize::from(cursor.results.page_len) {
            for item in 0..usize::from(cursor.results.lens[page]) {
                let WorldMarqueeResult::Object(token) = cursor.results.pages[page][item].expect("marquee result token") else {
                    panic!("object marquee produced component token");
                };
                ids.push(state.interaction_objects.resolve(token).expect("live result token").id.as_str().to_owned());
            }
        }
        ids
    }

    #[test]
    fn world_marquee_mesh_cursor_matches_legacy_window_crossing_disjoint_and_degenerate_cases() {
        let state = world_marquee_geometry_fixture(1);
        let viewport = render_pick_viewport(&state);
        let view_projection = state.orbit.to_camera().view_proj(1.0);
        let mesh = state.meshes.get("mesh").unwrap();
        let projected: Vec<_> = (0..3).map(|index| ui_wgpu::wgpu::project_point(view_projection, world_mesh_vertex(mesh, index).unwrap(), viewport.w, viewport.h).unwrap()).collect();
        let min_x = projected.iter().map(|point| point[0]).fold(f32::INFINITY, f32::min);
        let max_x = projected.iter().map(|point| point[0]).fold(f32::NEG_INFINITY, f32::max);
        let min_y = projected.iter().map(|point| point[1]).fold(f32::INFINITY, f32::min);
        let max_y = projected.iter().map(|point| point[1]).fold(f32::NEG_INFINITY, f32::max);
        let cases = [[[min_x - 2.0, min_y - 2.0], [max_x + 2.0, max_y + 2.0]], [[(min_x + max_x) * 0.5, min_y - 2.0], [min_x - 2.0, (min_y + max_y) * 0.5]], [[0.0, 0.0], [2.0, 2.0]], [[3.0, 3.0], [3.0, 3.0]]];
        for points in cases {
            let crossing = marquee_is_crossing_from_path(&points, false);
            let (meshes, draws) = legacy_geometry_fixture(&state);
            let legacy = screen_select_instances(&meshes, &draws, view_projection, viewport.w, viewport.h, &points, true, crossing);
            let retained = world_marquee_cursor_ids(&state, &points);
            assert_eq!(retained, legacy, "points={points:?}");
        }
    }

    #[test]
    fn world_marquee_mesh_cursor_preserves_legacy_multi_page_draw_order() {
        let state = world_marquee_geometry_fixture(WORLD_MARQUEE_RESULT_PAGE_CAPACITY + 1);
        let points = [[0.0, 0.0], [400.0, 400.0]];
        let view_projection = state.orbit.to_camera().view_proj(1.0);
        let (meshes, draws) = legacy_geometry_fixture(&state);
        let legacy = screen_select_instances(&meshes, &draws, view_projection, 400.0, 400.0, &points, true, false);
        let retained = world_marquee_cursor_ids(&state, &points);
        assert_eq!(retained, legacy);
        assert_eq!(retained.len(), WORLD_MARQUEE_RESULT_PAGE_CAPACITY + 1);
    }

    #[test]
    fn world_marquee_lasso_edge_cursor_matches_legacy_and_rejects_object_aba() {
        let mut state = world_marquee_geometry_fixture(1);
        state.selection_method = "lasso".into();
        let viewport = render_pick_viewport(&state);
        let view_projection = state.orbit.to_camera().view_proj(1.0);
        let points = [[0.0, 0.0], [400.0, 0.0], [400.0, 400.0], [0.0, 400.0]];
        let (meshes, draws) = legacy_geometry_fixture(&state);
        let legacy = screen_select_instances(&meshes, &draws, view_projection, viewport.w, viewport.h, &points, false, marquee_is_crossing_from_path(&points, true));
        assert_eq!(world_marquee_cursor_ids(&state, &points), legacy);

        let mut gesture = WorldMarqueeGesture::new(state.interaction_revision, 7, points[0]);
        for point in points.iter().skip(1) {
            assert!(gesture.push(*point));
        }
        let mut cursor = WorldMarqueePickCursor::new(&state, 8, gesture).unwrap();
        while cursor.current.is_none() {
            assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 8, context)), WorldInteractionStep::Pending);
        }
        let token = cursor.current.unwrap();
        let entry = *state.interaction_objects.resolve(token).unwrap();
        let mut replacement = entry.model;
        replacement.cols[3][0] += 1.0;
        let next = state.interaction_objects.admit(state.interaction_revision, WorldInteractionObjectKind::Instance, entry.id.as_str(), entry.mesh, replacement, entry.values).expect("replacement token");
        assert_ne!(next, token);
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 8, context)), WorldInteractionStep::Stale);
        let mut turns = 0;
        while !cursor.close_step() {
            turns += 1;
            assert!(turns < WORLD_INTERACTION_MARQUEE_POINT_CAPACITY + 4);
        }
        assert_eq!(cursor.gesture.len, 0);
    }

    fn world_component_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<u32> {
        let mut gesture = WorldMarqueeGesture::new(state.interaction_revision, 7, points[0]);
        for point in points.iter().skip(1) {
            assert!(gesture.push(*point));
        }
        let mut cursor = WorldMarqueePickCursor::new(state, 8, gesture).expect("component marquee cursor");
        let mut turns = 0;
        loop {
            turns += 1;
            if with_world_step_context(1, |context| cursor.step(state, 8, context)) == WorldInteractionStep::Complete {
                break;
            }
            assert!(turns < 8_000);
        }
        let mut ids = Vec::new();
        for item in 0..usize::from(cursor.results.lens[0]) {
            let WorldMarqueeResult::Component { id, .. } = cursor.results.pages[0][item].expect("component marquee result") else {
                panic!("component marquee produced object result");
            };
            ids.push(id);
        }
        ids.sort_unstable();
        ids
    }

    #[test]
    fn world_component_marquee_cursor_matches_legacy_vertex_edge_face_geometry() {
        let mut state = world_marquee_geometry_fixture(1);
        let viewport = render_pick_viewport(&state);
        let view_projection = state.orbit.to_camera().view_proj(1.0);
        let points = [[0.0, 0.0], [400.0, 400.0]];
        for granularity in ["vertex", "edge", "face"] {
            state.granularity = granularity.into();
            let (meshes, draws) = legacy_geometry_fixture(&state);
            let mut legacy: Vec<u32> = screen_select_components(&meshes, &draws, view_projection, viewport.w, viewport.h, &points, true, granularity, None, false).into_iter().map(|id| id.parse().expect("numeric component id")).collect();
            legacy.sort_unstable();
            assert_eq!(world_component_marquee_cursor_ids(&state, &points), legacy, "granularity={granularity}");
        }
    }

    #[test]
    fn world_component_marquee_publish_merges_before_one_atomic_set_selection() {
        let mut state = world_marquee_geometry_fixture(1);
        state.granularity = "vertex".into();
        state.component_ids = vec!["7".into()];
        let object = state.interaction_objects.instance_order[0].expect("instance token");
        let mut results = WorldMarqueeResultPages::default();
        assert!(results.push(WorldMarqueeResult::Component { object, id: 8 }, 0));
        let mut gesture = WorldMarqueeGesture::new(state.interaction_revision, 7, [0.0, 0.0]);
        assert!(gesture.push([400.0, 400.0]));
        let mut job = WorldComponentMarqueePublishJob::new(8, gesture, results, WorldComponentKind::Vertex, true, false);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let mut turns = 0;
        loop {
            turns += 1;
            if with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).expect("component marquee step") == WorldInteractionStep::Complete {
                break;
            }
            assert!(turns < 128);
        }
        let events = input.drain_events();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0].args.as_ref().and_then(|args| args.get("ids")), Some(dsl::DslValue::Array(ids)) if ids == &vec![dsl::DslValue::int(7), dsl::DslValue::int(8)]));
        assert_eq!(job.results.page_len, 0);
        assert_eq!(job.gesture.len, 0);
    }

    #[test]
    fn world_component_marquee_capacity_plus_one_fails_closed_and_retires() {
        let token = WorldInteractionObjectToken { slot: 0, generation: 1, revision: 1 };
        let mut results = WorldMarqueeResultPages::default();
        for id in 0..WORLD_COMPONENT_MARQUEE_CAPACITY as u32 {
            assert!(results.push(WorldMarqueeResult::Component { object: token, id }, 0));
        }
        assert!(!results.push(WorldMarqueeResult::Component { object: token, id: WORLD_COMPONENT_MARQUEE_CAPACITY as u32 }, 0));
        let mut turns = 0;
        while !results.close_step() {
            turns += 1;
            assert!(turns <= WORLD_COMPONENT_MARQUEE_CAPACITY + 1);
        }
        assert_eq!(results.page_len, 0);
    }

    #[test]
    fn world_gumball_cursor_caps_selected_tokens_and_closes_one_owner_per_grant() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 9;
        state.interaction_objects.revision = 9;
        for index in 0..=WORLD_GUMBALL_SELECTED_CAPACITY {
            let mut model = Mat4::identity();
            model.cols[3][0] = index as f32;
            assert!(state.interaction_objects.admit(9, WorldInteractionObjectKind::Instance, &format!("selected-{index}"), None, model, [0.0, 0.0, 1.0, index as f32, 0.0, 0.0, 0.0, 0.0]).is_some());
        }
        let mut cursor = WorldGumballPickCursor::new(&state, 12, 0.0, 0.0);
        for _ in 0..WORLD_INTERACTION_OBJECT_CAPACITY + 1 {
            let step = with_world_step_context(1, |context| cursor.step(&state, 12, context));
            if step == WorldInteractionStep::Fault {
                break;
            }
        }
        assert!(cursor.faulted);
        assert_eq!(cursor.selected_len as usize, WORLD_GUMBALL_SELECTED_CAPACITY);
        let mut turns = 0;
        while !cursor.close_step() {
            turns += 1;
            assert!(turns <= WORLD_GUMBALL_SELECTED_CAPACITY + 1);
        }
        assert_eq!(turns, WORLD_GUMBALL_SELECTED_CAPACITY);
        assert!(cursor.selected.iter().all(Option::is_none));
    }

    #[test]
    fn world_gumball_update_validates_one_selected_aba_token_per_turn() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 3;
        state.interaction_objects.revision = 3;
        state.bounds = Rect { x: 0.0, y: 0.0, w: 100.0, h: 100.0 };
        let token = state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, Mat4::identity(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]).expect("selected token");
        let mut selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
        selected[0] = Some(token);
        let mut gesture = WorldGumballGesture {
            revision: 3,
            start_generation: 5,
            handle: GumballHandle::MoveX,
            pivot: Vec3::ZERO,
            anchor: 0.0,
            start: Vec3::ZERO,
            translate: Vec3::ZERO,
            angle: 0.0,
            scale: Vec3::new(1.0, 1.0, 1.0),
            selected,
            selected_len: 1,
            selected_bytes: "selected".len() as u16,
            validation: 0,
            pending: None,
        };
        assert_eq!(gesture.begin_update(6, 50.0, 50.0), WorldInteractionStep::Pending);
        assert_eq!(gesture.update_step(&state), WorldInteractionStep::Pending);
        let mut replacement = Mat4::identity();
        replacement.cols[3][0] = 1.0;
        let next = state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, replacement, [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]).expect("replacement token");
        assert_ne!(next, token);
        gesture.validation = 0;
        assert_eq!(gesture.update_step(&state), WorldInteractionStep::Stale);
        assert!(!gesture.close_step());
        assert!(!gesture.close_step());
        assert!(gesture.close_step());
    }

    fn world_gumball_commit_fixture() -> (World3dState, WorldGumballGesture) {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 3;
        state.interaction_objects.revision = 3;
        let token = state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, Mat4::identity(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]).expect("selected token");
        let mut selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
        selected[0] = Some(token);
        let gesture = WorldGumballGesture {
            revision: 3,
            start_generation: 5,
            handle: GumballHandle::MoveX,
            pivot: Vec3::ZERO,
            anchor: 0.0,
            start: Vec3::ZERO,
            translate: Vec3::new(2.0, 0.0, 0.0),
            angle: 0.0,
            scale: Vec3::new(1.0, 1.0, 1.0),
            selected,
            selected_len: 1,
            selected_bytes: "selected".len() as u16,
            validation: 0,
            pending: None,
        };
        (state, gesture)
    }

    #[test]
    fn world_gumball_commit_builds_one_flat_node_per_grant_then_retires_tokens() {
        let (state, gesture) = world_gumball_commit_fixture();
        let mut job = WorldGumballCommitJob::new(8, gesture);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        assert_eq!(with_world_step_context(0, |context| job.step(&state, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        let mut turns = 0;
        loop {
            turns += 1;
            let step = with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).expect("bounded gumball commit");
            if step == WorldInteractionStep::Complete {
                break;
            }
            assert!(turns < 32);
        }
        assert!(job.terminal_is_empty());
        let actions = input.drain_events();
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, "translateSelection");
        assert!(turns > 8);
    }

    #[test]
    fn world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority() {
        let (mut state, gesture) = world_gumball_commit_fixture();
        let mut job = WorldGumballCommitJob::new(8, gesture);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let mut blockers = Vec::new();
        while let Ok(claim) = input.claim_action(1) {
            blockers.push(claim);
        }
        assert!(matches!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)));
        assert!(job.claim.is_none());
        input.release_action_claim(blockers.pop().expect("retry credit")).expect("release retry credit");
        assert_eq!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        assert!(job.claim.is_some());
        assert_eq!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        assert!(job.draft.is_some());
        let mut replacement = Mat4::identity();
        replacement.cols[3][0] = 1.0;
        state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, replacement, [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]).expect("ABA replacement");
        for _ in 0..4 {
            let _ = with_world_step_context(1, |context| job.step(&state, 8, &mut input, context));
        }
        assert!(matches!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::Structure)));
        let mut close_turns = 0;
        while !job.close_step(&mut input) {
            close_turns += 1;
            assert!(close_turns < 8);
        }
        for claim in blockers {
            input.release_action_claim(claim).expect("release blocker");
        }
        assert!(input.drain_events().is_empty());
        assert_eq!(job.gesture.selected_len, 0);
        assert_eq!(job.gesture.selected_bytes, 0);
    }

    #[test]
    fn world_gumball_fixed_gesture_projects_preview_without_mutating_source_draw() {
        let (mut state, gesture) = world_gumball_commit_fixture();
        state.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
        let source = Mat4::identity();
        let preview = retained_gumball_preview_model(&state, 0, 0, source);
        assert_eq!(source.cols[3], [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(preview.cols[3], [2.0, 0.0, 0.0, 1.0]);
        let unmatched = retained_gumball_preview_model(&state, 0, 1, source);
        assert_eq!(unmatched.cols, source.cols);
    }

    fn world_brush_commit_fixture(target: String) -> World3dState {
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.interaction_revision = 4;
        state.brush_preview = Some(WorldBrushPreviewRecord {
            target_vortex_full_id: Some(target),
            object_kind_id: Some("kind".into()),
            source_vortex_index: Some(7),
            origin: Some([1.0, 2.0, 3.0]),
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: Some(serde_json::json!([1.0, 2.0, 3.0])),
            ..Default::default()
        });
        state
    }

    #[test]
    fn world_brush_commit_copies_and_revalidates_fixed_chunks_before_claimed_publication() {
        let state = world_brush_commit_fixture("v".repeat(WORLD_BRUSH_COPY_CHUNK_BYTES * 2 + 1));
        let mut job = WorldBrushCommitJob::new(&state, 9).unwrap().expect("brush job");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let mut turns = 0;
        loop {
            turns += 1;
            let step = with_world_step_context(1, |context| job.step(&state, 9, &mut input, context)).expect("bounded brush step");
            if step == WorldInteractionStep::Complete {
                break;
            }
            assert!(turns < 64);
        }
        assert!(turns > 20);
        let actions = input.drain_events();
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, "addBrushObject");
        assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("targetVortexFullId")).and_then(dsl::DslValue::as_str).map(str::len), Some(WORLD_BRUSH_COPY_CHUNK_BYTES * 2 + 1));
    }

    #[test]
    fn world_brush_validation_detects_same_length_replacement_and_close_releases_draft_claim() {
        let mut state = world_brush_commit_fixture("first".into());
        let mut stale = WorldBrushCommitJob::new(&state, 9).unwrap().expect("brush job");
        while !stale.validating {
            assert_eq!(with_world_step_context(1, |context| stale.step(&state, 9, &mut ui_wgpu::wgpu::InputState::default(), context)).unwrap(), WorldInteractionStep::Pending);
        }
        state.brush_preview.as_mut().unwrap().target_vortex_full_id = Some("other".into());
        assert!(matches!(with_world_step_context(1, |context| stale.step(&state, 9, &mut ui_wgpu::wgpu::InputState::default(), context)), Err(ui_wgpu::wgpu::BoundedActionFault::Structure)));

        let state = world_brush_commit_fixture("target".into());
        let mut interrupted = WorldBrushCommitJob::new(&state, 10).unwrap().expect("brush job");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        while !interrupted.complete {
            let _ = with_world_step_context(1, |context| interrupted.step(&state, 10, &mut input, context));
        }
        assert_eq!(with_world_step_context(1, |context| interrupted.step(&state, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        assert!(interrupted.claim.is_some());
        assert_eq!(with_world_step_context(1, |context| interrupted.step(&state, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        assert!(interrupted.draft.is_some());
        assert!(!interrupted.close_step(&mut input));
        assert!(!interrupted.close_step(&mut input));
        assert!(interrupted.close_step(&mut input));
        assert!(input.drain_events().is_empty());
    }

    fn world_intent(generation: u64, delta: f32) -> WorldInteractionIntent {
        WorldInteractionIntent { phase: WorldInteractionPhase::Wheel, generation, x: 0.0, y: 0.0, dx: 0.0, dy: 0.0, delta, button: 0, down: false, shift: false, ctrl: false, alt: false, meta: false }
    }

    #[test]
    fn world_intent_queue_retains_exact_fifo_owner_on_saturation_and_closes_one_per_turn() {
        let mut queue = WorldInteractionIntentQueue::default();
        for index in 0..WORLD_INTERACTION_INTENT_CAPACITY {
            queue.push(world_intent(index as u64, index as f32)).expect("fixed intent slot");
        }
        let rejected = queue.push(world_intent(99, 99.0)).expect_err("capacity plus one retains intent");
        assert_eq!(rejected.generation, 99);
        assert_eq!(queue.front().map(|intent| intent.generation), Some(0));
        assert!(!queue.retire_front(1));
        assert_eq!(queue.front().map(|intent| intent.generation), Some(0));
        assert!(queue.retire_front(0));
        assert_eq!(queue.front().map(|intent| intent.generation), Some(1));
        queue.begin_close();
        let mut turns = 0;
        while !queue.close_step() {
            turns += 1;
            assert!(turns < WORLD_INTERACTION_INTENT_CAPACITY);
        }
        assert_eq!(turns, WORLD_INTERACTION_INTENT_CAPACITY - 1);
        assert!(queue.terminal_is_empty());
        assert_eq!(queue.push(rejected).expect_err("closed authority retains late intent").generation, 99);
    }

    #[test]
    fn world_wheel_plan_revalidates_before_mutation_and_publishes_flat_action() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let original_distance = state.orbit.distance;
        let mut stale = plan_world3d_wheel(&state, 7, 20.0).expect("bounded wheel plan");
        state.interaction_revision = state.interaction_revision.wrapping_add(1);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let stale_step = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut stale, 7, &mut input, context)).unwrap();
        assert_eq!(stale_step, WorldInteractionStep::Stale);
        assert_eq!(state.orbit.distance, original_distance);
        assert!(input.drain_events().is_empty());

        let mut plan = plan_world3d_wheel(&state, 8, 20.0).expect("bounded wheel plan");
        let pending = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 8, &mut input, context)).unwrap();
        assert_eq!(pending, WorldInteractionStep::Pending);
        assert_ne!(state.orbit.distance, original_distance);
        let actions = input.drain_events();
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, "setCamera");
        let complete = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 8, &mut input, context)).unwrap();
        assert_eq!(complete, WorldInteractionStep::Complete);
        assert!(plan.terminal_is_empty());
    }

    #[test]
    fn world_drag_and_paint_plans_reserve_before_exact_mutation() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let original_target = state.orbit.target;
        let modifiers = PointerModifiers { shift: false, ctrl: false, alt: false, meta: false };
        let mut drag = plan_world3d_drag(&state, 3, 8.0, -4.0, 1, &modifiers).expect("middle drag plan");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let step = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut drag, 3, &mut input, context)).unwrap();
        assert_eq!(step, WorldInteractionStep::Pending);
        assert_ne!(state.orbit.target, original_target);
        assert_eq!(input.drain_events().into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["setCamera"]);

        state.interaction_mode = "paint".into();
        let mut begin = plan_world3d_paint_stroke(&state, 4, true, 0).expect("paint begin plan");
        let zero = with_world_step_context(0, |context| publish_world3d_plan_step(&mut state, &mut begin, 4, &mut input, context)).unwrap();
        assert_eq!(zero, WorldInteractionStep::Pending);
        assert!(!state.paint_stroke_active);
        let published = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut begin, 4, &mut input, context)).unwrap();
        assert_eq!(published, WorldInteractionStep::Pending);
        assert!(state.paint_stroke_active);
        assert_eq!(input.drain_events().into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["paintStrokeBegin"]);

        let mut end = plan_world3d_paint_stroke(&state, 5, false, 0).expect("paint end plan");
        state.interaction_revision = state.interaction_revision.wrapping_add(1);
        let stale = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut end, 5, &mut input, context)).unwrap();
        assert_eq!(stale, WorldInteractionStep::Stale);
        assert!(state.paint_stroke_active);
        assert!(input.drain_events().is_empty());
    }

    fn world_pick_fixture() -> World3dState {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let mut data = triangle_mesh_oracle();
        data.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
        store_mesh(&mut state, "mesh".into(), publish_oracle_mesh(data));
        let mesh_version = state.mesh_versions["mesh"];
        state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] });
        state
    }

    #[test]
    fn world_ray_pick_cursor_advances_one_triangle_or_boundary_per_grant() {
        let state = world_pick_fixture();
        let mut cursor = WorldRayPickCursor {
            revision: state.interaction_revision,
            generation: 9,
            purpose: WorldRayPickPurpose::Paint,
            origin: Vec3::new(0.0, 0.0, 1.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
            draw: 0,
            instance: 0,
            triangle: 0,
            mesh: None,
            mesh_probe: 0,
            merge: 0,
            best: None,
            complete: false,
            faulted: false,
        };
        let zero = with_world_step_context(0, |context| cursor.step(&state, 9, context));
        assert_eq!(zero, WorldInteractionStep::Pending);
        assert_eq!(cursor.triangle, 0);
        let first = with_world_step_context(1, |context| cursor.step(&state, 9, context));
        assert_eq!(first, WorldInteractionStep::Pending);
        assert_eq!(cursor.triangle, 0);
        assert!(cursor.mesh.is_some());
        let triangle = with_world_step_context(1, |context| cursor.step(&state, 9, context));
        assert_eq!(triangle, WorldInteractionStep::Pending);
        assert_eq!(cursor.triangle, 1);
        assert!(cursor.best.is_some());
        for _ in 0..3 {
            let _ = with_world_step_context(1, |context| cursor.step(&state, 9, context));
        }
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 9, context)), WorldInteractionStep::Complete);
        let mut plan = cursor.finish_plan(&state, 9).expect("live cursor").expect("paint hit");
        let mut state = state;
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 9, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
        assert_eq!(input.drain_events().into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["paintAt"]);
    }

    #[test]
    fn world_ray_pick_cursor_stale_and_interrupted_close_do_not_publish() {
        let mut state = world_pick_fixture();
        let mut cursor = WorldRayPickCursor {
            revision: state.interaction_revision,
            generation: 12,
            purpose: WorldRayPickPurpose::Surface,
            origin: Vec3::new(0.0, 0.0, 1.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
            draw: 0,
            instance: 0,
            triangle: 0,
            mesh: None,
            mesh_probe: 0,
            merge: 0,
            best: None,
            complete: false,
            faulted: false,
        };
        let _ = with_world_step_context(1, |context| cursor.step(&state, 12, context));
        let _ = with_world_step_context(1, |context| cursor.step(&state, 12, context));
        state.interaction_revision = state.interaction_revision.wrapping_add(1);
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 12, context)), WorldInteractionStep::Stale);
        assert_eq!(cursor.finish_plan(&state, 12).expect_err("stale cursor"), WorldInteractionStep::Stale);
        assert!(!cursor.close_step());
        assert!(cursor.close_step());
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn world_authority_retains_front_plan_across_output_saturation_and_retries_in_order() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let original_distance = state.orbit.distance;
        enqueue_world3d_intent(&mut state, world_intent(1, 10.0)).expect("wheel intent");
        enqueue_world3d_intent(&mut state, world_intent(2, 20.0)).expect("second wheel intent");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Pending);
        let mut claims = Vec::new();
        while let Ok(claim) = input.claim_action(1) {
            claims.push(claim);
        }
        assert!(!claims.is_empty());
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::OutputBlocked);
        assert_eq!(state.orbit.distance, original_distance);
        let released = claims.pop().expect("one retry credit");
        input.release_action_claim(released).expect("release retry credit");
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Pending);
        assert_ne!(state.orbit.distance, original_distance);
        for claim in claims {
            input.release_action_claim(claim).expect("release retained credit");
        }
        assert_eq!(input.drain_events().into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["setCamera"]);
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete);
        assert_eq!(state.interaction_authority.as_ref().and_then(|authority| authority.queue.front()).map(|intent| intent.generation), Some(2));
    }

    #[test]
    fn world_authority_close_drains_active_and_queued_fixed_owners_to_terminal() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        enqueue_world3d_intent(&mut state, world_intent(1, 10.0)).expect("active intent");
        enqueue_world3d_intent(&mut state, world_intent(2, 20.0)).expect("queued intent");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
        begin_world3d_interaction_close(&mut state);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        assert!(!with_world_step_context(0, |context| close_world3d_interaction_step(&mut state, &mut input, context)));
        assert!(!world3d_interaction_terminal_is_empty(&state));
        let mut turns = 0;
        while !with_world_step_context(1, |context| close_world3d_interaction_step(&mut state, &mut input, context)) {
            turns += 1;
            assert!(turns < 8);
        }
        assert!(world3d_interaction_terminal_is_empty(&state));
        assert!(input.drain_events().is_empty());
    }

    #[test]
    fn world_saturation_owner_blocks_new_ingress_until_exact_fifo_transfer() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        for generation in 1..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
            enqueue_world3d_intent(&mut state, world_intent(generation, generation as f32)).expect("queue or retained saturation owner");
        }
        let rejected_generation = WORLD_INTERACTION_INTENT_CAPACITY as u64 + 2;
        assert_eq!(enqueue_world3d_intent(&mut state, world_intent(rejected_generation, rejected_generation as f32)).expect_err("blocked owner seals ingress").generation, rejected_generation);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        for _ in 0..3 {
            let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
        }
        assert_eq!(input.drain_events().len(), 1);
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
        let authority = state.interaction_authority.as_mut().expect("authority");
        assert!(authority.blocked.is_none());
        for generation in 2..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
            assert_eq!(authority.queue.front().map(|intent| intent.generation), Some(generation));
            assert!(authority.queue.retire_front(generation));
        }
        assert!(authority.queue.front().is_none());
    }

    #[test]
    fn renderer_world_consumers_use_only_fixed_intent_ingress() {
        let glue = include_str!("../../📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
        let shell = include_str!("../../📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs");
        for source in [glue, shell] {
            assert!(source.contains("enqueue_world3d_event"));
            assert!(!source.contains("handle_world3d_pointer_button"));
            assert!(!source.contains("handle_world3d_pointer_move"));
            assert!(!source.contains("handle_world3d_paint_actions"));
            assert!(!source.contains("handle_world3d_pointer_drag"));
            assert!(!source.contains("handle_world3d_wheel"));
            assert!(!source.contains(concat!("let mut world_actions", " = Vec")));
        }
        let world = include_str!("🦀️.rs");
        assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_button")));
        assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_move")));
        assert!(!world.contains(concat!("pub fn ", "handle_world3d_paint_actions")));
        assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_drag")));
        assert!(!world.contains(concat!("pub fn ", "handle_world3d_wheel")));
    }

    #[test]
    fn prepared_world_resources_are_send_and_deduplicate_uploads() {
        assert_send::<World3dBuildContext>();
        let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
        resources.ensure_mesh("mesh", 3, &[0.0, 1.0, 2.0], &[0.0, 0.0, 1.0], &[0, 1, 2]);
        resources.ensure_mesh("mesh", 3, &[9.0], &[9.0], &[9]);
        resources.ensure_world_plane_texture("image", &[1, 2, 3, 4], 1, 1);
        resources.ensure_world_plane_texture("image", &[9, 9, 9, 9], 1, 1);
        resources.evict_mesh("stale");
        let mut input = ui_wgpu::wgpu::PreparedRenderInput::new(1, 2, ui_wgpu::wgpu::DrawList::default(), None, 0.0);
        assert_eq!(resources.append_step(&mut input).ok(), Some(false));
        assert_eq!(resources.append_step(&mut input).ok(), Some(false));
        assert_eq!(resources.append_step(&mut input).ok(), Some(false));
        assert_eq!(resources.append_step(&mut input).ok(), Some(false));
        assert_eq!(resources.append_step(&mut input).ok(), Some(false));
        assert_eq!(resources.append_step(&mut input).ok(), Some(true));
        assert_eq!(input.uploads.len(), 2);
        assert_eq!(input.evictions, vec![PreparedRenderEviction::Mesh { key: "stale".into() }]);
    }

    #[test]
    fn world_orbit_view_gizmo_placement_matches_react_bottom_right_insets() {
        assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 1280.0, h: 720.0 }), (32.0, 32.0));
        assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 120.0, h: 160.0 }), (32.0, 32.0));
        assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 40.0, h: 48.0 }), (22.0, 22.0));
    }

    #[test]
    fn world_orbit_view_gizmo_preserves_label_free_hit_targets() {
        let tips = gizmo::orbit_view_gizmo_tips(&Camera3d::default(), Rect { x: 0.0, y: 0.0, w: 1280.0, h: 720.0 });
        assert_eq!(tips.len(), 15);
        assert_eq!(tips.iter().filter(|tip| tip.prominent).count(), 8);
        assert!(tips.iter().all(|tip| tip.pick_radius >= 7.0));
    }

    fn topology_mesh() -> Mesh3dLease {
        let mut data = mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2, 1, 3, 2]);
        data.face_ids = vec![10, 11];
        data.vertex_ids = vec![1, 2, 3, 4];
        data.edge_positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
        data.edge_ids = vec![5, 6];
        publish_oracle_mesh(data)
    }

    fn scene_with_selection(selection_json: &str) -> UiComponentSceneNode {
        scene_with_selection_and_domain(selection_json, None)
    }

    /// 🪟️ Like `scene_with_selection`, but also stamps `World3dScene.domain_id`/`domain_granularity_id`
    /// when `domain` is `Some((domain_id, granularity_id))` — exercises the app-bound-domain path of
    /// `resolved_domain_id`/`resolved_domain_granularity_id`/`resolved_item_id`.
    fn scene_with_selection_and_domain(selection_json: &str, domain: Option<(&str, &str)>) -> UiComponentSceneNode {
        UiComponentSceneNode {
            presence: UiPresence::default(),
            surface_id: "surface-1".into(),
            controller_id: "controller-1".into(),
            component_kind: SurfaceKind::World3d,
            pane_id: None,
            binding_id: None,
            canvas_2d: None,
            world_3d: Some(World3dScene {
                snapshot: None,
                camera_json: r#"{"position":[4.0,4.0,4.0],"target":[0.0,0.0,0.0],"up":[0.0,0.0,1.0],"fov":45.0}"#.into(),
                meshes_json: r#"[{"id":"mesh-1","data":{"positions":[0,0,0,1,0,0,0,1,0],"normals":[0,0,1,0,0,1,0,0,1],"indices":[0,1,2],"faceIds":[10],"vertexIds":[1,2,3],"edgePositions":[0,0,0,1,0,0],"edgeIds":[5]}}]"#.into(),
                instances_json: r#"[{"id":"obj-1","meshId":"mesh-1","position":[0,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into(),
                selection_json: selection_json.into(),
                vortices_json: None,
                attractions_json: None,
                target_volumes_json: None,
                references_json: None,
                brush_preview_json: None,
                interaction_json: None,
                engagement_preview_json: None,
                lod_json: None,
                chunking_json: None,
                environment_json: None,
                frame_json: None,
                fit_json: None,
                terrain_json: None,
                points_json: None,
                status_json: None,
                domain_id: domain.map(|(id, _)| id.to_string()),
                domain_granularity_id: domain.map(|(_, granularity)| granularity.to_string()),
            }),
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
            menu: None,
        }
    }

    #[test]
    fn sync_parses_selection_targets_and_active_object() {
        let selection = r#"{
            "granularity":"vertex",
            "targets":{"mesh":true,"vertex":true,"edge":false,"face":false},
            "activeObjectId":"obj-1"
        }"#;
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        sync_world3d_state(&mut state, &scene_with_selection(selection), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert!(state.selection_targets.vertex);
        assert!(!state.selection_targets.edge);
        assert_eq!(state.active_object_id.as_deref(), Some("obj-1"));
    }

    #[test]
    fn sync_parses_numeric_component_ids_and_hovered_component() {
        let selection = r#"{
            "granularity":"vertex",
            "componentIds":[1,2],
            "hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":3},
            "showEdges":true
        }"#;
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        sync_world3d_state(&mut state, &scene_with_selection(selection), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert_eq!(state.granularity, "vertex");
        assert_eq!(state.component_ids, vec!["1".to_string(), "2".to_string()]);
        assert_eq!(state.hovered_component_id.as_deref(), Some("3"));
        assert_eq!(state.hovered_component_object_id.as_deref(), Some("obj-1"));
        assert_eq!(state.hovered_component_mode.as_deref(), Some("vertex"));
        assert!(state.show_edges);
    }

    #[test]
    fn append_component_vertex_spheres_render_base_vertices() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.selection_targets.vertex = true;
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let instances = append_component_vertex_spheres(&mut state);
        assert_eq!(instances.len(), 0);

        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert_eq!(lines.len(), 28); // 4 from edges + 24 from 4 vertex crosses
    }

    #[test]
    fn append_component_overlays_highlights_only_hovered_edge() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "edge".into();
        state.selection_targets.edge = true;
        state.hovered_component_id = Some("5".into());
        state.hovered_component_object_id = Some("obj-1".into());
        state.hovered_component_mode = Some("edge".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert!(lines.len() >= 2);
        assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
    }

    #[test]
    fn append_component_overlays_highlights_selected_edge() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "edge".into();
        state.component_ids = vec!["6".into()];
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert!(lines.len() >= 2);
        assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
    }

    #[test]
    fn component_mode_does_not_apply_mesh_instance_hover() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.hovered_component_object_id = Some("obj-1".into());
        state.local_hover_id = None;
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        apply_runtime_draw_flags(&mut state);
        assert!(!state.draws[0].instances[0].hovered);
    }

    #[test]
    fn runtime_draw_flags_clear_stale_instance_selected_when_selection_is_empty() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.selected_ids.clear();
        state.local_hover_id = None;
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: true }] });
        apply_runtime_draw_flags(&mut state);
        assert!(!state.draws[0].instances[0].selected, "empty selection must clear a stale instancesJson selected bit");
        assert!(!state.draws[0].instances[0].hovered, "empty hover must clear a stale instancesJson hovered bit");
    }

    #[test]
    fn merge_u32_ids_supports_add_and_toggle() {
        assert_eq!(merge_u32_ids(&["1".into()], &["2".into()], "add"), vec![1, 2]);
        assert_eq!(merge_u32_ids(&["1".into(), "2".into()], &["2".into(), "3".into()], "toggle"), vec![1, 3]);
    }

    #[test]
    fn pick_select_emits_numeric_world_pick_id() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("vertex projects");
        let action = pick_select_action(&state, screen[0], screen[1], inner, false, false).expect("pick action");
        assert_eq!(action.action, "worldPick");
        let args = action.args.expect("args");
        assert_eq!(args["id"].as_f64(), Some(1.0));
    }

    #[test]
    fn marquee_preview_respects_pick_bounds_offset() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.active_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 100.0, y: 50.0, w: 400.0, h: 400.0 };
        state.pick_bounds = inner;
        state.marquee_points = vec![[110.0, 60.0], [490.0, 450.0]];
        update_marquee_preview(&mut state, inner);
        assert!(!state.marquee_preview_ids.is_empty(), "preview ids: {:?}", state.marquee_preview_ids);
    }

    #[test]
    fn marquee_crossing_includes_partial_overlap_window_does_not() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "mesh".into();
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let view_proj = camera.view_proj(1.0);
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        for corner in [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]] {
            let screen = ui_wgpu::wgpu::project_point(view_proj, Vec3::from_array(corner), inner.w, inner.h).expect("screen");
            min_x = min_x.min(screen[0]);
            min_y = min_y.min(screen[1]);
            max_x = max_x.max(screen[0]);
            max_y = max_y.max(screen[1]);
        }
        let center_x = (min_x + max_x) * 0.5;
        let center_y = (min_y + max_y) * 0.5;
        state.marquee_points = vec![[inner.x + min_x, inner.y + min_y], [inner.x + center_x, inner.y + center_y]];
        update_marquee_preview(&mut state, inner);
        assert!(state.marquee_preview_ids.is_empty(), "window marquee should not select partially enclosed mesh");
        state.marquee_points = vec![[inner.x + center_x, inner.y + min_y], [inner.x + min_x, inner.y + center_y]];
        update_marquee_preview(&mut state, inner);
        assert!(!state.marquee_preview_ids.is_empty(), "crossing marquee should select partially enclosed mesh");
    }

    #[test]
    fn marquee_component_mode_emits_set_selection_with_numeric_ids() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.component_ids = vec!["1".into()];
        state.marquee_points = vec![[10.0, 10.0], [390.0, 390.0]];
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let action = marquee_select_action(&mut state, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, true, false).expect("marquee action");
        assert_eq!(action.action, "setSelection");
        let args = action.args.expect("args");
        assert_eq!(args["mode"], json!("vertex"));
        assert!(args["ids"].as_array().is_some());
    }

    #[test]
    fn click_release_routes_to_pick_select_instead_of_empty_marquee() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.active_utility = "select".into();
        state.granularity = "mesh".into();
        state.marquee_active = true;
        state.marquee_points = vec![[120.0, 140.0]];
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let action = handle_world3d_pointer_button(&mut state, 120.0, 140.0, false, 0, &PointerModifiers::default()).expect("click should pick");
        assert_eq!(action.action, "worldPick");
        assert!(!state.marquee_active);
    }

    #[test]
    fn marquee_face_preview_and_overlay_use_logical_face_ids() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.active_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = bounds;
        state.pick_bounds = bounds;
        state.marquee_points = vec![[390.0, 10.0], [10.0, 390.0]];
        update_marquee_preview(&mut state, bounds);
        assert!(state.marquee_preview_ids.iter().any(|id| id == "10" || id == "11"), "preview ids: {:?}", state.marquee_preview_ids);
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert!(!lines.is_empty(), "face marquee preview should draw triangle edge lines");
    }

    #[test]
    fn hovered_component_preserved_when_selection_json_omits_hover_field() {
        let selection = r#"{"granularity":"face","componentIds":[10]}"#;
        let scene = scene_with_selection(selection);
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.hovered_component_id = Some("11".into());
        state.hovered_component_object_id = Some("obj-1".into());
        state.hovered_component_mode = Some("face".into());
        sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert_eq!(state.hovered_component_id.as_deref(), Some("11"));
    }

    #[test]
    fn apply_world_action_preview_updates_component_hover_and_selection() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "setHover".into(),
                args: action_args(json!({
                    "objectId": "obj-1",
                    "mode": "vertex",
                    "id": 2,
                })),
            },
        );
        assert_eq!(state.hovered_component_id.as_deref(), Some("2"));
        assert_eq!(state.hovered_component_mode.as_deref(), Some("vertex"));
        assert!(state.local_hover_id.is_none());

        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "worldPick".into(),
                args: action_args(json!({
                    "granularity": "vertex",
                    "id": 4,
                    "merge": "replace",
                })),
            },
        );
        assert_eq!(state.component_ids, vec!["4".to_string()]);
        assert_eq!(state.granularity, "vertex");
    }

    #[test]
    fn preview_survives_sync_when_scene_json_unchanged() {
        let selection = r#"{"granularity":"vertex","componentIds":[1],"hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":2}}"#;
        let scene = scene_with_selection(selection);
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "worldPick".into(),
                args: action_args(json!({
                    "granularity": "vertex",
                    "id": 5,
                    "merge": "replace",
                })),
            },
        );
        sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert_eq!(state.component_ids, vec!["5".to_string()]);
    }

    #[test]
    fn typed_camera_snapshot_matches_current_camera_fixture_without_production_parsing() {
        let mut scene = scene_with_selection("{}");
        let mut page = ui_wgpu::wgpu::World3dSnapshotPage::new(ui_wgpu::wgpu::World3dSnapshotPageKind::Camera);
        page.push_item(ui_wgpu::wgpu::World3dSnapshotItem { numbers: [4.0, 4.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], number_len: 10, ..Default::default() }).unwrap();
        page.seal().unwrap();
        let descriptor = ui_wgpu::wgpu::World3dSnapshotDescriptor { revision: 5, generation: 7, page_count: 1, item_count: 1, byte_count: 0, draw_count: 0, draw_instance_count: 0, draw_byte_count: 0 };
        let token = ui_wgpu::wgpu::world3d_snapshot_begin(descriptor).unwrap();
        ui_wgpu::wgpu::world3d_snapshot_admit_page(token, page).unwrap();
        let lease = ui_wgpu::wgpu::world3d_snapshot_seal(token).unwrap();
        scene.world_3d.as_mut().unwrap().snapshot = Some(lease);

        let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        let mut typed = World3dState::new("surface".into(), "controller".into());
        sync_world3d_state(&mut typed, &scene, bounds);
        let mut turns = 0;
        loop {
            turns += 1;
            if with_world_step_context(1, |context| step_world3d_snapshot(&mut typed, context)) == World3dSnapshotApplyStep::Complete {
                break;
            }
            assert!(turns < 8);
        }
        let mut current = World3dState::new("surface".into(), "controller".into());
        sync_world3d_state_legacy(&mut current, &scene, bounds);
        let typed_camera = typed.orbit.to_camera();
        let current_camera = current.orbit.to_camera();
        assert_eq!(typed_camera.position, current_camera.position);
        assert_eq!(typed_camera.target, current_camera.target);
        assert_eq!(typed.snapshot_lease, Some(lease));

        ui_wgpu::wgpu::world3d_snapshot_begin_close(lease).unwrap();
        assert!(!ui_wgpu::wgpu::world3d_snapshot_close_step(lease).unwrap());
        assert!(ui_wgpu::wgpu::world3d_snapshot_close_step(lease).unwrap());
    }

    #[test]
    fn dynamic_world_owners_retire_one_nested_owner_per_grant_to_terminal_empty() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0; 3 * 128], vec![0.0; 3 * 128], vec![0; 3 * 64]));
        state.meshes.insert("mesh".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances: (0..32).map(|index| Instance3d { id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }).collect() });
        state.reference_pixels.insert("reference".into(), (64, 64, vec![0; 64 * 64 * 4]));
        state.mesh_paint_textures.insert("paint".into(), (64, 64, vec![0; 64 * 64 * 4]));
        assert!(begin_world3d_dynamic_retirement(&mut state));
        assert!(!begin_world3d_dynamic_retirement(&mut state));
        assert!(!with_world_step_context(0, |context| step_world3d_dynamic_retirement(&mut state, context)));
        let mut turns = 0;
        while !with_world_step_context(1, |context| step_world3d_dynamic_retirement(&mut state, context)) {
            turns += 1;
            assert!(turns < 16);
        }
        assert!(turns >= 8, "each registry transition and opaque owner transfer consumes a distinct grant");
        assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
    }

    #[test]
    fn dynamic_registry_returns_capacity_and_identifier_owners_exactly() {
        let mut registry = WorldDynamicRegistry::<u32, 2>::default();
        assert!(registry.insert("a".into(), 1).is_ok());
        assert!(registry.insert("b".into(), 2).is_ok());
        let rejected = registry.insert("c".into(), 3).expect_err("capacity owner");
        assert_eq!(rejected.fault, WorldDynamicFault::RegistryCapacity);
        assert_eq!((rejected.id.as_str(), rejected.value), ("c", 3));
        let rejected = registry.insert("x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY + 1), 4).expect_err("identifier owner");
        assert_eq!(rejected.fault, WorldDynamicFault::IdCapacity);
        assert_eq!(rejected.value, 4);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn dynamic_registry_replacement_invalidates_aba_token_and_retains_previous_owner() {
        let mut registry = WorldDynamicRegistry::<u32, 1>::default();
        let (first, _) = registry.insert("mesh".into(), 1).unwrap();
        let (second, previous) = registry.insert("mesh".into(), 2).unwrap();
        let previous = previous.expect("replacement returns previous owner");
        assert_eq!((previous.id.as_str(), previous.value), ("mesh", 1));
        assert_eq!(registry.remove_token(first).unwrap_err(), WorldDynamicFault::StaleToken);
        assert_eq!(registry.remove_token(second).unwrap().value, 2);
        assert!(registry.is_empty());
    }

    #[test]
    fn opaque_quarantine_saturation_returns_the_exact_rejected_owner() {
        let mut quarantine = WorldOpaqueQuarantine::<1>::default();
        let first = WorldOpaqueOwner::ReferencePixels(WorldDynamicEntry { id: "first".into(), epoch: 1, value: (1, 1, vec![1]) });
        quarantine.admit(first).unwrap();
        let second = WorldOpaqueOwner::PaintPixels(WorldDynamicEntry { id: "second".into(), epoch: 2, value: (1, 1, vec![2]) });
        let rejected = quarantine.admit(second).expect_err("quarantine returns saturated owner");
        let WorldOpaqueOwner::PaintPixels(rejected) = rejected else { panic!("exact paint owner") };
        assert_eq!((rejected.id.as_str(), rejected.epoch, rejected.value.2.as_slice()), ("second", 2, &[2][..]));
        assert_eq!(quarantine.saturated, 1);
        assert!(quarantine.take_one().is_some());
        assert!(quarantine.take_one().is_none());
    }

    #[test]
    fn dynamic_close_is_interruptible_and_rejects_late_insertions() {
        let mut registry = WorldDynamicRegistry::<u32, 2>::default();
        registry.insert("a".into(), 1).unwrap();
        registry.insert("b".into(), 2).unwrap();
        registry.begin_close();
        assert_eq!(registry.take_one().expect("one owner per close grant").value, 1);
        assert_eq!(registry.len(), 1);
        let rejected = registry.insert("late".into(), 3).expect_err("closing registry rejects late owner");
        assert_eq!((rejected.fault, rejected.id.as_str(), rejected.value), (WorldDynamicFault::Closing, "late", 3));
        assert_eq!(registry.take_one().expect("resumed close owner").value, 2);
        assert!(registry.is_empty());
    }

    #[test]
    fn draw_registry_returns_the_exact_instance_capacity_owner() {
        let mut draws = WorldDrawRegistry::default();
        let instances = (0..=WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY).map(|index| Instance3d { id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }).collect();
        let rejected = draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances }).expect_err("draw instance capacity owner");
        assert_eq!(rejected.fault, WorldDynamicFault::InstanceCapacity);
        assert_eq!(rejected.value.instances.len(), WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY + 1);
        assert!(draws.is_empty());
    }

    fn draw_fixture_instance(id: &str) -> Instance3d {
        Instance3d { id: id.into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }
    }

    fn admit_draw_fixture_instance(state: &mut World3dState, draw: u16, id: &str) -> Result<(), WorldDynamicFault> {
        world3d_draw_rebuild_admit_instance(state, draw, id, Mat4::identity(), [1.0; 4], false, false)
    }

    fn draw_fixture_bytes(draws: &[(&str, &[&str])]) -> u32 {
        draws.iter().map(|(mesh, instances)| mesh.len() + std::mem::size_of::<SceneDraw3d>() + instances.iter().map(|id| id.len() + std::mem::size_of::<Instance3d>()).sum::<usize>()).sum::<usize>() as u32
    }

    #[test]
    fn retained_draw_rebuild_preserves_mixed_group_and_instance_fifo_then_swaps_atomically() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let bytes = draw_fixture_bytes(&[("mesh-a", &["a-0", "a-1"]), ("mesh-b", &["b-0"])]);
        begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 2, instance_count: 3, byte_count: bytes }).unwrap();
        world3d_draw_rebuild_admit_draw(&mut state, "mesh-a", 1, 2).unwrap();
        world3d_draw_rebuild_admit_draw(&mut state, "mesh-b", 2, 1).unwrap();
        admit_draw_fixture_instance(&mut state, 0, "a-0").unwrap();
        admit_draw_fixture_instance(&mut state, 0, "a-1").unwrap();
        admit_draw_fixture_instance(&mut state, 1, "b-0").unwrap();
        world3d_draw_rebuild_seal(&mut state).unwrap();
        assert!(state.draws.is_empty(), "sealed drafts never publish partially");
        let mut turns = 0;
        while with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state, context)) == WorldDrawRebuildStep::Pending {
            turns += 1;
            assert!(turns < 16);
        }
        assert_eq!(state.draws.iter().map(|draw| draw.mesh_key.as_str()).collect::<Vec<_>>(), vec!["mesh-a", "mesh-b"]);
        assert_eq!(state.draws[0].instances.iter().map(|instance| instance.id.as_str()).collect::<Vec<_>>(), vec!["a-0", "a-1"]);
        assert_eq!(state.draws[1].instances[0].id, "b-0");
        assert_eq!(state.draw_generation, 1);
    }

    #[test]
    fn retained_draw_rebuild_rejects_byte_and_identifier_plus_one_before_owner_publication() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 0, byte_count: 0 }).unwrap();
        let rejected = world3d_draw_rebuild_admit_draw(&mut state, "mesh", 1, 0).expect_err("byte +1 leaves borrowed mesh owner at producer");
        assert_eq!(rejected, WorldDynamicFault::ByteCapacity);
        while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state, context)) {}
        let long = "x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY + 1);
        begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 0, byte_count: WORLD_DYNAMIC_DRAW_BYTE_CAPACITY as u32 }).unwrap();
        let rejected = world3d_draw_rebuild_admit_draw(&mut state, &long, 1, 0).expect_err("ID +1 leaves borrowed mesh owner at producer");
        assert_eq!(rejected, WorldDynamicFault::IdCapacity);
        while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state, context)) {}
    }

    #[test]
    fn retained_draw_rebuild_rejects_aggregate_instance_plus_one_before_draft_ownership() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let result =
            begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: (WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY + 1) as u32, byte_count: WORLD_DYNAMIC_DRAW_BYTE_CAPACITY as u32 });
        assert_eq!(result, Err(WorldDynamicFault::InstanceCapacity));
        assert!(world3d_draw_rebuild_terminal_is_empty(&state));
    }

    #[test]
    fn retained_draw_rebuild_stale_and_interrupted_close_never_publish() {
        let mut state = World3dState::new("surface".into(), "controller".into());
        let bytes = draw_fixture_bytes(&[("mesh", &["one", "two"])]);
        begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 2, byte_count: bytes }).unwrap();
        world3d_draw_rebuild_admit_draw(&mut state, "mesh", 1, 2).unwrap();
        admit_draw_fixture_instance(&mut state, 0, "one").unwrap();
        admit_draw_fixture_instance(&mut state, 0, "two").unwrap();
        world3d_draw_rebuild_seal(&mut state).unwrap();
        state.interaction_revision = 1;
        assert_eq!(with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state, context)), WorldDrawRebuildStep::Stale);
        assert!(!with_world_step_context(0, |context| close_world3d_draw_rebuild_step(&mut state, context)));
        let mut turns = 0;
        while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state, context)) {
            turns += 1;
            assert!(turns < 8);
        }
        assert!(state.draws.is_empty());
        assert!(world3d_draw_rebuild_terminal_is_empty(&state));
    }

    #[test]
    fn production_dynamic_owners_have_no_hash_map_vec_or_direct_pixel_mutation_bypass() {
        let source = include_str!("🦀️.rs");
        let production = source.split("#[cfg(test)]\nmod tests").next().expect("production source");
        for forbidden in [
            concat!("pub meshes: HashMap<String, Mesh", "3d>"),
            "pub draws: Vec<SceneDraw3d>",
            "reference_pixels: HashMap<String, (u32, u32, Vec<u8>)>",
            "mesh_paint_textures: HashMap<String, (u32, u32, Vec<u8>)>",
            "state.reference_pixels.insert(",
            "state.mesh_paint_textures.insert(",
            "state.draws =",
            "fn rebuild_instance_draws(state:",
        ] {
            assert!(!production.contains(forbidden), "production dynamic owner bypass returned: {forbidden}");
        }
        assert!(production.contains("state.meshes.plan_insert(&id)"), "mesh publication remains centralized at the observed-slot replacement authority");
        assert!(production.contains("step_world3d_dynamic_retirement"), "World3d exposes the one-grant retained close pump");
        assert!(production.contains("struct WorldPlaceholderMeshCursor"));
        assert!(production.contains("mesh3d_allocate_step(self.token()?)"));
        assert!(production.contains("struct WorldFaceOverlayMeshCursor"));
        assert!(production.contains("self.owner = WorldPlaceholderOwner::Writing(mesh3d_begin("));
        let face_route = production.split("fn append_component_face_translucent_overlays").nth(1).and_then(|source| source.split("fn selection_centroid").next()).expect("face overlay production route");
        for forbidden in [concat!("Mesh", "3d::from_buffers"), "FaceOverlayBucket", "HashSet<String>", "Vec<f32>"] {
            assert!(!face_route.contains(forbidden), "face overlay recursive/contiguous constructor returned: {forbidden}");
        }
        let mounted_root = include_str!("../🦀️.rs");
        assert!(mounted_root.contains("pub use crate::world::*;"));
        assert!(!mounted_root.contains(concat!("Mesh", "3d")));
    }

    #[test]
    fn pick_viewport_uses_render_bounds_not_pick_clip_offset() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.active_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let bounds = Rect { x: 0.0, y: 50.0, w: 400.0, h: 400.0 };
        let clip = Rect { x: 0.0, y: 100.0, w: 400.0, h: 400.0 };
        state.bounds = bounds;
        state.pick_bounds = clip;
        let camera = state.orbit.to_camera();
        let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, bounds.w, bounds.h).expect("vertex projects");
        let global_x = bounds.x + screen[0];
        let global_y = bounds.y + screen[1];
        let picked = pick_component_at(&state, global_x, global_y, bounds).expect("vertex pick respects render viewport");
        assert_eq!(picked.1, "1");
    }

    #[test]
    fn append_component_face_overlay_lines_include_hovered_face() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.hovered_component_mode = Some("face".into());
        state.hovered_component_id = Some("10".into());
        state.hovered_component_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert!(lines.len() >= 6, "hovered face should emit triangle edge lines, got {}", lines.len());
    }

    #[test]
    fn pick_component_at_face_mode_uses_ray_pick() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.active_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let mesh_ref = state.meshes.get("mesh-1").expect("mesh");
        let tri = mesh_ref.indices.get(0..3).expect("triangle");
        let centroid = mesh_vertex(mesh_ref, tri[0]).add(mesh_vertex(mesh_ref, tri[1])).add(mesh_vertex(mesh_ref, tri[2])).scale(1.0 / 3.0);
        let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), centroid, inner.w, inner.h).expect("face centroid projects");
        let picked = pick_component_at(&state, screen[0], screen[1], inner).expect("face pick");
        assert_eq!(picked.0, "face");
        assert_eq!(picked.2, "obj-1");
    }

    #[test]
    fn pick_component_at_edge_mode_uses_ray_pick() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "edge".into();
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let chunk = state.meshes.get("mesh-1").and_then(|mesh| mesh.edge_positions.get(0..6)).expect("edge");
        let a = Vec3::new(chunk[0], chunk[1], chunk[2]);
        let b = Vec3::new(chunk[3], chunk[4], chunk[5]);
        let mid = a.add(b).scale(0.5);
        let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), mid, inner.w, inner.h).expect("edge midpoint projects");
        let picked = pick_component_at(&state, screen[0], screen[1], inner).expect("edge pick");
        assert_eq!(picked.0, "edge");
        assert_eq!(picked.2, "obj-1");
    }

    #[test]
    fn chunk_key_indices_buckets_negative_coordinates() {
        assert_eq!(chunk_key_indices([-10.0, 5.0, 0.0], 256.0), (-1, 0, 0));
        assert_eq!(chunk_key_indices([300.0, 300.0, 0.0], 256.0), (1, 1, 0));
    }

    #[test]
    fn chunk_distance_visible_uses_hysteresis() {
        let center = chunk_center((0, 0, 0), 256.0);
        let cam_near = Vec3::new(0.0, 0.0, 0.0);
        assert!(chunk_distance_visible(cam_near, center, 256.0, 8000.0, false));
        let cam_far = Vec3::new(8400.0, 128.0, 0.0);
        assert!(!chunk_distance_visible(cam_far, center, 256.0, 8000.0, false));
        assert!(chunk_distance_visible(cam_far, center, 256.0, 8000.0, true));
    }

    #[test]
    fn mesh_pool_release_clears_at_zero_refcount() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.mesh_pool.acquire("mesh-1".into());
        assert!(state.mesh_pool.release("mesh-1".into()));
        assert!(!state.mesh_pool.contains(&"mesh-1".to_string()));
        state.mesh_pool.acquire("mesh-1".into());
        state.mesh_pool.acquire("mesh-1".into());
        assert!(!state.mesh_pool.release("mesh-1".into()));
        assert!(state.mesh_pool.contains(&"mesh-1".to_string()));
        assert!(state.mesh_pool.release("mesh-1".into()));
        assert!(!state.mesh_pool.contains(&"mesh-1".to_string()));
    }

    #[test]
    fn resolve_physical_mesh_id_picks_closest_lod_url() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.mesh_lod_catalog.insert("tower".into(), vec![WorldMeshLodEntry { lod: 1.0, url: "https://example.com/tower-high.glb".into() }, WorldMeshLodEntry { lod: 100.0, url: "https://example.com/tower-low.glb".into() }]);
        let detailed = resolve_physical_mesh_id(&state, "tower", 2.0);
        let coarse = resolve_physical_mesh_id(&state, "tower", 200.0);
        assert_eq!(detailed, "mesh:tower-high");
        assert_eq!(coarse, "mesh:tower-low");
    }

    //#region GlbAssetTests
    #[test]
    fn asset_authority_rejects_request_and_byte_capacity_plus_one_before_string_ownership() {
        let mut lane = WorldAssetIoAuthority::default();
        for index in 0..WORLD_ASSET_REQUEST_CAPACITY {
            lane.reserve(1, 1, WorldAssetRequestKind::Glb, &format!("asset-{index}"), 1).unwrap();
        }
        assert_eq!(lane.reserve(1, 1, WorldAssetRequestKind::Glb, "overflow", 1), Err(WorldAssetFault::ItemCapacity));
        lane.begin_close();
        while !lane.close_step() {}
        assert!(lane.terminal_is_empty());

        let mut lane = WorldAssetIoAuthority::default();
        assert_eq!(lane.reserve(1, 1, WorldAssetRequestKind::Glb, "large", WORLD_ASSET_RESPONSE_BYTE_CAPACITY + 1), Err(WorldAssetFault::ByteCapacity));
        assert!(lane.terminal_is_empty());
    }

    #[test]
    fn asset_response_rejects_page_plus_one_and_retires_partial_stream_one_page_per_step() {
        let bytes = vec![7; WORLD_ASSET_RESPONSE_PAGE_BYTES + 1];
        assert_eq!(WorldAssetResponsePage::try_from_owned(bytes).expect_err("page +1 exact owner").len(), WORLD_ASSET_RESPONSE_PAGE_BYTES + 1);

        let mut lane = WorldAssetIoAuthority::default();
        let token = lane.reserve_request(2, 3, WorldAssetRequestKind::ReferenceImage, "image").unwrap();
        let mut owner = lane.take_next().unwrap();
        lane.reserve_response(&mut owner, WORLD_ASSET_RESPONSE_PAGE_BYTES * 2).unwrap();
        owner.push_page(WorldAssetResponsePage::try_from_owned(vec![1; WORLD_ASSET_RESPONSE_PAGE_BYTES]).unwrap()).unwrap();
        owner.push_page(WorldAssetResponsePage::try_from_owned(vec![2; 7]).unwrap()).unwrap();
        lane.return_owner(owner).unwrap();
        lane.begin_close();
        assert!(!lane.close_step(), "first grant retires one response page");
        assert!(!lane.close_step(), "second grant retires the second response page");
        while !lane.close_step() {}
        assert!(lane.terminal_is_empty());
        let _ = token;
    }

    #[test]
    fn asset_decode_resume_and_stale_generation_keep_claimed_pages_until_terminal_return() {
        let mut lane = WorldAssetIoAuthority::default();
        let token = lane.reserve(4, 9, WorldAssetRequestKind::Glb, "mesh", 16).unwrap();
        let mut fetch = lane.take_next().unwrap();
        fetch.push_page(WorldAssetResponsePage::try_from_owned(vec![1; 8]).unwrap()).unwrap();
        fetch.push_page(WorldAssetResponsePage::try_from_owned(vec![2; 8]).unwrap()).unwrap();
        fetch.seal().unwrap();
        lane.return_owner(fetch).unwrap();
        assert!(matches!(lane.take_completed(token, 5, 9), Err(WorldAssetFault::Stale)));
        let mut decode = lane.take_completed(token, 4, 9).unwrap();
        assert_eq!(decode.take_decode_page().unwrap().unwrap().bytes(), &[1; 8]);
        lane.return_owner(decode).unwrap();
        let mut decode = lane.take_completed(token, 4, 9).unwrap();
        assert_eq!(decode.take_decode_page().unwrap().unwrap().bytes(), &[2; 8]);
        assert!(decode.take_decode_page().unwrap().is_none());
        decode.begin_close();
        while !decode.close_step() {}
        lane.finish(decode).unwrap();
        assert!(lane.terminal_is_empty());
    }

    #[test]
    fn asset_unknown_length_releases_unused_aggregate_credit_at_seal() {
        let mut lane = WorldAssetIoAuthority::default();
        let token = lane.reserve_request(7, 11, WorldAssetRequestKind::Glb, "chunked").unwrap();
        let mut owner = lane.take_next().unwrap();
        lane.reserve_response(&mut owner, WORLD_ASSET_RESPONSE_BYTE_CAPACITY).unwrap();
        owner.push_page(WorldAssetResponsePage::try_from_owned(vec![3; 7]).unwrap()).unwrap();
        lane.seal_response(&mut owner).unwrap();
        assert_eq!(lane.reserved_bytes, 7);
        lane.return_owner(owner).unwrap();
        let mut owner = lane.take_completed(token, 7, 11).unwrap();
        assert_eq!(owner.take_decode_page().unwrap().unwrap().bytes(), &[3; 7]);
        owner.begin_close();
        while !owner.close_step() {}
        lane.finish(owner).unwrap();
        assert!(lane.terminal_is_empty());
    }

    #[test]
    fn asset_completed_cursor_advances_one_fixed_slot_per_grant_and_hands_back_exact_owner() {
        let mut lane = WorldAssetIoAuthority::default();
        let token = lane.reserve_request(8, 12, WorldAssetRequestKind::Glb, "cursor.glb").unwrap();
        let mut fetch = lane.take_next().unwrap();
        assert!(lane.take_next_completed_step().is_none(), "the first grant observes the in-flight slot without scanning ahead");
        lane.reserve_response(&mut fetch, 12).unwrap();
        fetch.push_page(WorldAssetResponsePage::try_from_owned(b"glTF\x02\0\0\0\x0c\0\0\0".to_vec()).unwrap()).unwrap();
        lane.seal_response(&mut fetch).unwrap();
        lane.return_owner(fetch).unwrap();
        let mut decode = (0..WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| lane.take_next_completed_step()).expect("the retained cursor reaches the completed slot");
        assert_eq!(decode.token(), token);
        assert_eq!(decode.decode_page().unwrap().unwrap().bytes(), b"glTF\x02\0\0\0\x0c\0\0\0");
        decode.advance_decode_page().unwrap();
        assert!(decode.decode_page().unwrap().is_none());
        decode.rewind_decode_pages().unwrap();
        assert!(decode.decode_page().unwrap().is_some());
        decode.begin_close();
        while !decode.close_step() {}
        lane.finish(decode).unwrap();
        assert!(lane.terminal_is_empty());
    }

    /// 📦️ URL-backed meshes remain fetchable instead of becoming empty procedural primitives.
    #[test]
    fn rebuild_instance_draws_keeps_url_backed_mesh_pending() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.mesh_url_fallback.insert("mesh:capsule".into(), "/mesh/capsule.glb".into());
        state.parsed_instances = vec![WorldInstanceRecord { id: "capsule-1".into(), mesh_id: Some("mesh:capsule".into()), position: Some([0.0, 0.0, 0.0]), ..Default::default() }];

        rebuild_instance_draws_legacy(&mut state, 1.0);

        assert!(!state.meshes.contains_key("mesh:capsule"), "a URL-backed mesh must not be shadowed by an empty placeholder_mesh placeholder");
        let mut owner = take_next_world3d_asset(&mut state).expect("URL-backed mesh publishes one retained asset request");
        assert_eq!(owner.url(), "/mesh/capsule.glb");
        owner.begin_close();
        return_world3d_asset(&mut state, owner).expect("retained request returns to its exact surface authority");
        while retire_cancelled_world3d_asset_step(&mut state) {}
        assert!(state.asset_io.terminal_is_empty());
    }

    //#endregion GlbAssetTests

    #[test]
    fn lod_grid_lines_generate_for_near_camera() {
        let mut lines = Vec::new();
        append_lod_grid_lines(&mut lines, 2.0, 10.0, Vec3::ZERO, [0.5, 0.5, 0.5, 1.0]);
        assert!(!lines.is_empty());
    }

    //#region EnvironmentTests
    #[test]
    fn environment_clear_color_uses_opaque_background() {
        let environment = WorldEnvironmentRecord { background: Some("#112233".into()), ..Default::default() };
        let theme_clear = Rgba::new(0.0, 0.0, 0.0, 1.0);
        let clear = environment_clear_color(&environment, theme_clear);
        assert!((clear.r - (0x11 as f32 / 255.0)).abs() < 1e-3);
        assert!((clear.g - (0x22 as f32 / 255.0)).abs() < 1e-3);
        assert!((clear.b - (0x33 as f32 / 255.0)).abs() < 1e-3);
    }

    #[test]
    fn environment_clear_color_falls_back_when_transparent_or_absent() {
        let theme_clear = Rgba::new(0.1, 0.2, 0.3, 1.0);
        let transparent = WorldEnvironmentRecord { background: Some("transparent".into()), ..Default::default() };
        let absent = WorldEnvironmentRecord::default();
        assert_eq!(environment_clear_color(&transparent, theme_clear).r, theme_clear.r);
        assert_eq!(environment_clear_color(&absent, theme_clear).r, theme_clear.r);
    }

    #[test]
    fn environment_light_dir_uses_sun_direction_only_when_enabled() {
        let disabled = WorldEnvironmentRecord { sun: Some(WorldEnvironmentSunRecord { enabled: Some(false), azimuth: Some(90.0), elevation: Some(0.0), ..Default::default() }), ..Default::default() };
        assert_eq!(environment_light_dir(&disabled), [0.4, 0.6, 0.8]);

        let enabled = WorldEnvironmentRecord { sun: Some(WorldEnvironmentSunRecord { enabled: Some(true), azimuth: Some(90.0), elevation: Some(0.0), ..Default::default() }), ..Default::default() };
        let dir = environment_light_dir(&enabled);
        // azimuth=90, elevation=0 -> pure +Y direction (cos(0)*cos(90)=~0, cos(0)*sin(90)=1, sin(0)=0).
        assert!(dir[0].abs() < 1e-3);
        assert!((dir[1] - 1.0).abs() < 1e-3);
        assert!(dir[2].abs() < 1e-3);
    }

    #[test]
    fn rebuild_instance_draws_applies_environment_material_color_as_neutral_default() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.environment = WorldEnvironmentRecord { material: Some(WorldEnvironmentMaterialRecord { color: Some("#ff0000".into()), ..Default::default() }), ..Default::default() };
        state.parsed_instances = vec![WorldInstanceRecord { id: "obj-1".into(), mesh_id: Some("box".into()), position: Some([0.0, 0.0, 0.0]), ..Default::default() }];
        rebuild_instance_draws_legacy(&mut state, 1.0);
        let instance = &state.draws.iter().find(|draw| draw.mesh_key == "box").expect("box draw").instances[0];
        assert!((instance.color[0] - 1.0).abs() < 1e-3);
        assert!(instance.color[1].abs() < 1e-3);
    }
    //#endregion EnvironmentTests

    //#region TerrainTests
    #[test]
    fn hypsometric_color_matches_reference_stops() {
        let low = hypsometric_color(0.0);
        assert!((low[0] - 0x4b as f32 / 255.0).abs() < 1e-3);
        let peak = hypsometric_color(1.0);
        assert!((peak[0] - 1.0).abs() < 1e-3);
        assert!((peak[1] - 1.0).abs() < 1e-3);
    }

    #[test]
    fn build_terrain_band_mesh_buckets_by_average_elevation() {
        // Two triangles, 6 verts (no sharing, to keep each triangle's average elevation exact):
        // triangle 0 is flat at elevation ratio 0.0, triangle 1 is flat at elevation ratio 1.0.
        let mesh = TerrainTileMeshPayload {
            positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 10.0, 3.0, 0.0, 10.0, 2.0, 1.0, 10.0],
            normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
            indices: vec![0, 1, 2, 3, 4, 5],
            uvs: vec![0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0],
        };
        let low_band = build_terrain_band_mesh(&mesh, 0, TERRAIN_COLOR_BANDS);
        assert!(low_band.is_some(), "triangle 0 (all-zero elevation) should fall in band 0");
        let high_band = build_terrain_band_mesh(&mesh, TERRAIN_COLOR_BANDS - 1, TERRAIN_COLOR_BANDS);
        assert!(high_band.is_some(), "triangle 1 (all-one elevation) should fall in the top band");
        let empty_band = build_terrain_band_mesh(&mesh, 5, TERRAIN_COLOR_BANDS);
        assert!(empty_band.is_none(), "no triangle should land in a middle band for this fixture");
    }

    #[test]
    fn terrain_tile_url_substitutes_z_x_y() {
        assert_eq!(terrain_tile_url("/dem/{z}/{x}/{y}.png", 12, 34, 56), "/dem/12/34/56.png");
    }

    #[test]
    fn sync_terrain_state_queues_fetch_for_uncached_tile_and_builds_after_upload() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 9.7382, project_origin_lat: 52.3759, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
        apply_terrain_style_if_changed_state(&mut state);
        let camera = Camera3d { position: Vec3::new(0.0, 0.0, 300.0), target: Vec3::ZERO, up: Vec3::new(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0 };
        let (band_draws, evicted) = sync_terrain_state(&mut state, &camera);
        assert!(band_draws.is_empty(), "no elevation data uploaded yet, nothing to draw");
        assert!(evicted.is_empty(), "nothing was cached yet, nothing to evict");
        assert!(!state.pending_terrain_tile_urls.is_empty(), "an uncached visible tile should be queued for byte-fetch");

        let (_, &(z, x, y)) = state.pending_terrain_tile_urls.iter().next().expect("a pending tile");
        let value = (100.0_f64 + 32768.0).round() as i64;
        let r = ((value >> 8) & 0xff) as u8;
        let g = (value - ((r as i64) << 8)).clamp(0, 255) as u8;
        let mut image = image::RgbaImage::new(256, 256);
        for pixel in image.pixels_mut() {
            *pixel = image::Rgba([r, g, 0, 255]);
        }
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
        assert!(state.terrain_session.upload_elevation_tile(z, x, y, &bytes));

        let (band_draws_after_upload, _) = sync_terrain_state(&mut state, &camera);
        assert!(!band_draws_after_upload.is_empty(), "an uploaded tile should produce at least one banded draw");
    }

    #[test]
    fn apply_terrain_style_if_changed_state_purges_stale_meshes_on_origin_change() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 0.0, project_origin_lat: 0.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
        assert!(apply_terrain_style_if_changed_state(&mut state).is_empty(), "first application has nothing to purge");
        let mesh_key = terrain_band_mesh_key(&state.surface_id, 10, 1, 2, 0);
        store_mesh(&mut state, mesh_key, publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])));
        state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 5.0, project_origin_lat: 5.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
        let purged = apply_terrain_style_if_changed_state(&mut state);
        assert_eq!(purged.len(), 1, "an origin change should purge the previously-cached terrain mesh");
        assert!(state.meshes.is_empty());
    }
    //#endregion TerrainTests

    //#region BrushPreviewTests
    #[test]
    fn brush_preview_mesh_id_falls_back_to_box_without_mesh_url() {
        assert_eq!(brush_preview_mesh_id(None), "box");
        assert_eq!(brush_preview_mesh_id(Some("/assets/tower.glb")), mesh_id_from_url("/assets/tower.glb"));
    }
    //#endregion BrushPreviewTests

    //#region ContextMenuTests
    #[test]
    fn resolve_world_context_menu_target_prioritizes_vortex_over_object_over_reference() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.local_hover_id = Some("reference:site-plan.png".into());
        assert_eq!(resolve_world_context_menu_target(&state), Some(("reference", "site-plan.png".to_string())));

        state.hovered_component_mode = Some("face".into());
        state.hovered_component_object_id = Some("obj-1".into());
        assert_eq!(resolve_world_context_menu_target(&state), Some(("object", "obj-1".to_string())));

        state.hovered_vortex_id = Some("vortex-1".into());
        assert_eq!(resolve_world_context_menu_target(&state), Some(("vortex", "vortex-1".to_string())));
    }

    #[test]
    fn resolve_world_context_menu_target_is_none_without_any_hover() {
        let state = World3dState::new("surface-1".into(), "controller-1".into());
        assert_eq!(resolve_world_context_menu_target(&state), None);
    }

    #[test]
    fn right_click_dispatches_context_menu_at_for_hovered_vortex() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        state.hovered_vortex_id = Some("vortex-1".into());
        handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
        let action = handle_world3d_pointer_button(&mut state, 200.0, 200.0, false, 2, &PointerModifiers::default()).expect("right click should dispatch");
        assert_eq!(action.action, "contextMenuAt");
        let args = action.args.expect("args");
        assert_eq!(args["kind"], json!("vortex"));
        assert_eq!(args["id"], json!("vortex-1"));
    }

    #[test]
    fn right_drag_does_not_dispatch_context_menu_even_with_a_hovered_vortex() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        state.hovered_vortex_id = Some("vortex-1".into());
        handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
        let action = handle_world3d_pointer_button(&mut state, 260.0, 260.0, false, 2, &PointerModifiers::default()).expect("right release should still sync camera");
        assert_eq!(action.action, "setCamera", "a right-drag should fall back to the orbit camera sync, not open a context menu");
    }
    //#endregion ContextMenuTests

    //#region 🔖️WorldInteractionVerbs
    #[test]
    fn world_interaction_definition_declares_path_delimited_item_domain() {
        let def = world_interaction_definition();
        assert_eq!(def.id, WORLD_INTERACTION_DOMAIN_ID);
        assert_eq!(def.granularities.iter().map(|granularity| granularity.id.clone()).collect::<Vec<_>>(), vec!["surface".to_string(), "item".to_string()]);
        assert!(matches!(def.hierarchy, HierarchyProvider::PathDelimited { ref delimiter } if delimiter == "/"));
        assert!(def.selection.methods.contains(&SelectionMethod::Pick));
        assert!(def.selection.methods.contains(&SelectionMethod::Rectangle));
        assert!(def.selection.merges.contains(&MergeMode::Additive));
    }

    #[test]
    fn pick_select_emits_batched_interaction_select_for_plain_object_pick() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        // 🕹️ default `granularity` ("object") is neither component-mode nor "mesh" — this is the
        // plain `world`-domain item pick path (see `pick_select_action`'s final fallback branch).
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("object projects");
        let action = pick_select_action(&state, screen[0], screen[1], inner, true, false).expect("pick action");
        assert_eq!(action.action, "interactionSelect");
        let args = action.args.expect("args");
        assert_eq!(args["domainId"], json!(WORLD_INTERACTION_DOMAIN_ID));
        assert_eq!(args["method"], json!("pick"));
        assert_eq!(args["merge"], json!("additive"), "shift modifier maps to the canonical MergeMode label");
        let targets = args["targets"].as_array().expect("targets array");
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0]["granularity"], json!(WORLD_ITEM_GRANULARITY_ID));
        assert_eq!(targets[0]["id"], json!("surface-1/obj-1"), "item target id is surfaceId/objectId (PathDelimited)");
    }

    #[test]
    fn marquee_select_emits_batched_interaction_select_with_rectangle_method() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        state.marquee_points = vec![[0.0, 0.0], [400.0, 400.0]];
        let action = marquee_select_action(&mut state, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, false, true).expect("marquee action");
        assert_eq!(action.action, "interactionSelect");
        let args = action.args.expect("args");
        assert_eq!(args["method"], json!("rectangle"));
        assert_eq!(args["merge"], json!("invertive"), "ctrl modifier maps to the canonical MergeMode label");
        assert!(state.marquee_points.is_empty(), "marquee is consumed after gathering targets");
    }

    #[test]
    fn pick_hover_emits_interaction_hover_and_clears_when_nothing_hit() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("object projects");
        let action = pick_hover_action(&mut state, screen[0], screen[1], inner).expect("hover action");
        assert_eq!(action.action, "interactionHover");
        let args = action.args.expect("args");
        assert_eq!(args["domainId"], json!(WORLD_INTERACTION_DOMAIN_ID));
        assert_eq!(args["channel"], json!("pointer"));
        let targets = args["targets"].as_array().expect("targets array");
        assert_eq!(targets[0]["id"], json!("surface-1/obj-1"));

        // 🖱️ Moving off the instance clears — empty `targets` is `next_hover`'s clear signal.
        let action = pick_hover_action(&mut state, 5.0, 5.0, inner).expect("clear action");
        assert_eq!(action.action, "interactionHover");
        let args = action.args.expect("args");
        assert!(args["targets"].as_array().expect("targets array").is_empty());
    }

    #[test]
    fn apply_world_action_preview_applies_interaction_select_and_hover_for_world_domain() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "interactionSelect".into(),
                args: action_args(json!({
                    "domainId": WORLD_INTERACTION_DOMAIN_ID,
                    "targets": [{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-1" }],
                    "merge": "replace",
                    "method": "pick",
                })),
            },
        );
        assert_eq!(state.selected_ids, vec!["obj-1".to_string()], "surfaceId/ prefix is stripped for this surface");

        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "interactionHover".into(),
                args: action_args(json!({
                    "domainId": WORLD_INTERACTION_DOMAIN_ID,
                    "channel": "pointer",
                    "targets": [{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-2" }],
                })),
            },
        );
        assert_eq!(state.local_hover_id.as_deref(), Some("obj-2"));

        apply_world_action_preview(&mut state, &ActionDescriptor { controller_id: "controller-1".into(), action: "interactionHover".into(), args: action_args(json!({ "domainId": WORLD_INTERACTION_DOMAIN_ID, "channel": "pointer", "targets": [] })) });
        assert!(state.local_hover_id.is_none(), "empty targets clears hover");
    }

    #[test]
    fn pick_select_emits_bare_id_into_bound_app_domain_when_window_binds_one() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.bound_domain_id = Some("cad".into());
        state.bound_domain_granularity_id = Some("object".into());
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("object projects");
        let action = pick_select_action(&state, screen[0], screen[1], inner, false, false).expect("pick action");
        assert_eq!(action.action, "interactionSelect");
        let args = action.args.expect("args");
        assert_eq!(args["domainId"], json!("cad"), "targets the window's bound app domain, not the OS `world` fallback");
        let targets = args["targets"].as_array().expect("targets array");
        assert_eq!(targets[0]["granularity"], json!("object"), "uses the bound domain's own granularity, not `item`");
        assert_eq!(targets[0]["id"], json!("obj-1"), "bare id — a bound domain is single-surface-scoped, no surfaceId/ prefix");
    }

    #[test]
    fn pick_hover_emits_bare_id_into_bound_app_domain() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.bound_domain_id = Some("cad".into());
        state.bound_domain_granularity_id = Some("object".into());
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("object projects");
        let action = pick_hover_action(&mut state, screen[0], screen[1], inner).expect("hover action");
        let args = action.args.expect("args");
        assert_eq!(args["domainId"], json!("cad"));
        let targets = args["targets"].as_array().expect("targets array");
        assert_eq!(targets[0]["granularity"], json!("object"));
        assert_eq!(targets[0]["id"], json!("obj-1"));
    }

    #[test]
    fn marquee_select_emits_bare_ids_into_bound_app_domain() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.bound_domain_id = Some("features".into());
        state.bound_domain_granularity_id = Some("pin".into());
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        state.marquee_points = vec![[0.0, 0.0], [400.0, 400.0]];
        let action = marquee_select_action(&mut state, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, false, false).expect("marquee action");
        let args = action.args.expect("args");
        assert_eq!(args["domainId"], json!("features"));
        let targets = args["targets"].as_array().expect("targets array");
        assert_eq!(targets[0]["granularity"], json!("pin"));
        assert_eq!(targets[0]["id"], json!("obj-1"));
    }

    #[test]
    fn apply_world_action_preview_respects_bound_app_domain_and_ignores_other_domains() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.bound_domain_id = Some("cad".into());
        state.bound_domain_granularity_id = Some("object".into());
        // 🚫️ An action for the OS `world` fallback domain must NOT apply once this window is bound to
        // its own app domain — otherwise the same click could ever light up two selection universes.
        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "interactionSelect".into(),
                args: action_args(json!({ "domainId": WORLD_INTERACTION_DOMAIN_ID, "targets": [{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-1" }], "merge": "replace", "method": "pick" })),
            },
        );
        assert!(state.selected_ids.is_empty(), "an unbound-domain action must not apply once this window binds its own domain");

        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "interactionSelect".into(),
                args: action_args(json!({ "domainId": "cad", "targets": [{ "granularity": "object", "id": "obj-1" }], "merge": "replace", "method": "pick" })),
            },
        );
        assert_eq!(state.selected_ids, vec!["obj-1".to_string()], "bare id applies as-is — no surfaceId/ stripping for a bound domain");
    }

    #[test]
    fn sync_world3d_state_captures_scene_bound_domain() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        let scene = scene_with_selection_and_domain("{}", Some(("cad", "object")));
        sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert_eq!(state.bound_domain_id.as_deref(), Some("cad"));
        assert_eq!(state.bound_domain_granularity_id.as_deref(), Some("object"));
        assert_eq!(resolved_domain_id(&state), "cad");
        assert_eq!(resolved_domain_granularity_id(&state), "object");
    }
    //#endregion 🔖️WorldInteractionVerbs
}
