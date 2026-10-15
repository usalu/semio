//! 🌉️ World 3D scene bridge: scene snapshot staging and camera fit publication.

use super::*;

//#region 🌉️World3dSceneBridge
/// 🌉️ How many `mesh3d_*` authority writes one bridge step performs. That ladder writes ONE
/// vec3/u32 per call, so a 20k-vertex preview would otherwise need 60k frames to publish; a batch
/// keeps the publication inside the frame transaction's own fuel accounting while still finishing a
/// typical procedural preview in a handful of turns.
const WORLD3D_BRIDGE_MESH_WRITES_PER_STEP: u32 = 4_096;

/// 🌉️📡️ Records how one geometry-less `meshes_json` entry resolves.
///
/// A `url` entry is left PENDING on purpose: its id is bound to the url in
/// [`World3dState::mesh_source_urls`], which is the ledger `render_world_3d`'s missing-mesh loop
/// reads to reserve a [`WorldAssetRequestKind::Glb`] fetch — the mesh table itself stays empty until
/// the decoded GLB lands through [`publish_world3d_asset_mesh_lease`]. Publishing a placeholder box
/// here would be worse than drawing nothing: the loop offers a url only while
/// `!state.meshes.contains_key(key)`, so a stand-in mesh would suppress the fetch forever, which is
/// precisely the shape of the dead url lane this replaces (ticket 26/09/17/WGPU-RENDERER-REACT-
/// PARITY, `📓️w3a-asset-decoder-boot-fault.md` §6). React draws nothing for an unloaded url either —
/// `GlbInstanceMesh` suspends until `useLoader` resolves.
///
/// A `kind` entry is the opposite: both renderers own the generator, so it resolves immediately
/// through the same fixed-credit placeholder ladder inline buffers ride.
///
/// Bounded by the mesh registry's own capacity and id budget — an id that could never publish a mesh
/// is not worth a ledger row, and the ledger never outgrows the table it indexes.
fn declare_scene_mesh_source(state: &mut World3dState, id: &str, url: Option<&str>, kind: Option<&str>) {
    if let Some(url) = url {
        if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY || url.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return;
        }
        if state.mesh_source_urls.len() >= WORLD_DYNAMIC_MESH_CAPACITY && !state.mesh_source_urls.contains_key(id) {
            return;
        }
        state.mesh_source_urls.insert(id.to_owned(), url.to_owned());
        return;
    }
    if let Some(kind) = kind {
        begin_world_placeholder_mesh(state, id, WorldPlaceholderKind::resolve(kind));
    }
}

/// 🥽️ Whether this mesh key's geometry is owed by the world-asset pipeline rather than by the
/// placeholder ladder. The snapshot apply consults it before minting a stand-in box — see
/// [`declare_scene_mesh_source`].
pub(super) fn scene_mesh_awaits_its_asset(state: &World3dState, mesh_key: &str) -> bool {
    state.mesh_source_urls.contains_key(mesh_key)
}

/// 🎨️ Resolves the standard-material pair React assigns after geometry provenance is known:
/// inline/procedural meshes use `0/1`, while `GlbInstanceMesh` uses the environment override.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn world3d_standard_material_for_mesh(state: &World3dState, mesh_key: &str, environment: SceneMaterial3d) -> [f32; 2] {
    if state.mesh_source_urls.contains_key(mesh_key) { [environment.metalness, environment.roughness] } else { [0.0, 1.0] }
}

/// 🌉️ One `World3dScene.meshes_json` entry. Three mutually exclusive ways a producer names one
/// mesh, all of which React's `WorldMeshRecord` (`🌐️World3dHost/🟦️.tsx`) accepts:
///
/// - `{"id": …, "data": {positions, normals, indices, …}}` — INLINE geometry (generation3d's
///   `preview_payload`, CAD, fem).
/// - `{"id": "mesh:…", "url": "/mesh/….glb"}` — a GLB the renderer fetches itself
///   (`world3d_meshes_json_from_urls`). React loads it with `GLTFLoader`; here it rides the bounded
///   world-asset pipeline as a [`WorldAssetRequestKind::Glb`], see [`declare_scene_mesh_source`].
/// - `{"id": "box", "kind": "box"}` — a built-in procedural kind
///   (`world3d_mesh_kind_entry`), a REFERENCE whose tessellation both renderers own. React resolves
///   it through `meshDataFromKind`, this one through [`WorldPlaceholderKind::resolve`].
#[derive(Clone, Debug, semio_framework_value_derive::FromValue)]
#[value(rename_all="camelCase")]
#[derive(semio_framework_value_derive::RetireOwned)]
pub(super) struct World3dSceneMeshEntry {
    pub(super) id:String,
    #[value(default)]
    pub(super) data:WorldMeshBuffers,
    #[value(default)]
    pub(super) url:Option<String>,
    #[value(default)]
    pub(super) kind:Option<String>,
}

impl World3dSceneMeshEntry {
    /// 🧊️ Retains original surface, wire, and point geometry in the Scene mesh owner.
    fn has_inline_geometry(&self) -> bool {
        !self.data.edge_positions.is_empty() || (self.data.vertex_count() > 0 && (self.data.indices.len() >= 3 || (self.data.indices.is_empty() && self.data.vertex_ids.iter().any(|id|*id!=u32::MAX))))
    }

    /// 🌉️ Resolves retained inline geometry or an authored mesh reference.
    fn names_a_mesh(&self) -> bool {
        self.has_inline_geometry() || self.url.is_some() || self.kind.is_some()
    }
}

/// 🧬️ The exact analytic source in the original instance wire.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(semio_framework_value_derive::RetireOwned)]
struct World3dSceneComponentSource {
    pub(super) handle: String,
    pub(super) revision: String,
}

/// 🌉️ One `World3dScene.instances_json` entry — a placement of one mesh, carrying the
/// channel-qualified instance id every pick/hover observation is addressed by.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(semio_framework_value_derive::RetireOwned)]
pub(super) struct World3dSceneInstanceEntry {
    #[serde(default)]
    pub(super) component_source: Option<World3dSceneComponentSource>,
    pub(super) id: String,
    pub(super) mesh_id: String,
    /// 🎯️ The TOPOLOGY target this rendered instance stands for — several instances of one channel
    /// share it. Absent means "the render id is its own target", exactly React's
    /// `instances.find(e => e.id === id)?.interactionId ?? id`
    /// (`🌐️World3dHost/🟦️.tsx`, `world3dInstanceInteractionTarget`).
    #[serde(default)]
    pub(super) interaction_id: Option<String>,
    #[serde(default)]
    pub(super) interaction_granularity_id: Option<String>,
    #[serde(default)]
    pub(super) position: Option<[f64; 3]>,
    #[serde(default)]
    pub(super) rotation: Option<[f64; 4]>,
    #[serde(default)]
    pub(super) scale: Option<[f64; 3]>,
    #[serde(default)]
    pub(super) color: Option<String>,
    /// 🟩️ The instance stands for an entity a running tool placed provisionally (`ArtifactView::tool_run()`).
    #[serde(default)]
    pub(super) provisional: bool,
    /// 🎨️ React's `WorldInstanceRecord.highlighted` — the compatible/suggested state (a catalogue
    /// kind hovered in puzzle), which resolves to the SECONDARY `highlighted` row of
    /// `MESH_STYLE_PAINT`.
    #[serde(default)]
    pub(super) highlighted: bool,
    /// 🎨️ React's `WorldInstanceRecord.disabled` — non-interactive/locked, the muted `disabled` row
    /// at 0.45 opacity.
    #[serde(default)]
    pub(super) disabled: bool,
    /// 🎉️ React's `isWorldInstanceCelebrating` — the `celebrated` row (primary + the conic triad).
    #[serde(default)]
    pub(super) celebrating: bool,
    /// 🌫️ Per-instance opacity MULTIPLIER, applied to the authored colour's alpha before any style
    /// row. This is how a plugin dims a LOCKED object on a committed mesh
    /// (`WORLD_LOCKED_OPACITY_SCALE = 0.35`, `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:178`): the instance
    /// lane had no opacity field at all, so `target_style`'s locked arm could be computed and never
    /// reach a mesh (ticket 26/09/17 packet W2f §5.5). `None` leaves the alpha untouched.
    #[serde(default)]
    pub(super) opacity: Option<f64>,
}

/// 🎥️ `World3dScene.camera_json` — the window's own camera measure, authored host-side.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(semio_framework_value_derive::RetireOwned)]
struct World3dSceneCameraRecord {
    #[serde(default)]
    pub(super) position: Option<[f64; 3]>,
    #[serde(default)]
    pub(super) target: Option<[f64; 3]>,
    #[serde(default)]
    pub(super) up: Option<[f64; 3]>,
    #[serde(default)]
    fov: Option<f64>,
    #[serde(default)]
    pub(super) zoom: Option<f64>,
    /// 📐️ The complete composed mode × orientation taxonomy accepted by React and WGPU.
    #[serde(default)]
    pub(super) projection: Option<Viewport3dProjectionSpec>,
    #[serde(default)]
    projection_frame: Viewport3dProjectionFramePolicy,
}

impl World3dSceneCameraRecord {
    /// 📐️ React's `worldProjectionFamily` over the delivered spec.
    pub(super) fn projection(&self) -> CameraProjection3d {
        self.projection.map_or(CameraProjection3d::Perspective, ui_wgpu::wgpu::projection_spec_family)
    }

    /// 📐️ React's `worldProjectionPerspectiveFov` fallback chain: the record's own `fov`, else the
    /// mode's, else the 45° `parseCameraState` default.
    fn fov_degrees(&self) -> f64 {
        self.projection.map(|spec| f64::from(ui_wgpu::wgpu::projection_spec_fov_degrees(spec))).or(self.fov).unwrap_or(45.0)
    }
}

/// 🎯️ `World3dScene.fit_json` — the producer's "frame this document once" lane, with the delivered
/// extent it wants framed (`world3d_fit_json`, `🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct World3dSceneFitRecord {
    #[serde(default)]
    pub(super) enabled: bool,
    #[serde(default)]
    pub(super) revision: u32,
    #[serde(default)]
    padding: Option<f64>,
    #[serde(default)]
    bounds_min: Option<[f64; 3]>,
    #[serde(default)]
    bounds_max: Option<[f64; 3]>,
}

/// 📷️ React's `WorldProjectionContentFrame` — the one-shot framing every explicit projection
/// seed gets. `onFramed` immediately marks React's viewport owned and unmounts the framer, so later
/// content-bound changes must never move an accepted camera until another selection or external seed
/// creates a new debt.
/// `frameWorldProjectionPose` applies the spec's look and fits both families: orthographic through
/// viewport zoom, perspective/curvilinear through `max(span · 1.5, 2)` eye distance.
///
/// The extent is React's `worldSceneContentBounds`: every non-provisional instance position plus
/// every VISIBLE reference plane's footprint. `camera_user_moved` is the ownership latch — React's
/// `viewportOwned`.
///
/// 📦️ The bridge exposes content positions before its camera lease becomes accepted. Framing in
/// that interval would be overwritten by `step_world3d_snapshot`; wait for the same accepted
/// snapshot boundary used by the camera-fit lane.
pub(super) fn sync_world3d_projection_content_frame(state: &mut World3dState) {
    if state.projection_frame_policy == Viewport3dProjectionFramePolicy::PreserveCamera || !state.projection_frame_owed {
        return;
    }
    if state.camera_user_moved {
        state.projection_frame_owed = false;
        return;
    }
    if state.scene_bridge.is_some()
        || state.snapshot_apply.is_some()
        || state.scene_bridge_lease.is_some_and(|lease| state.snapshot_lease != Some(lease))
    {
        return;
    }
    let Some((minimum, maximum)) = world3d_content_bounds(state) else { return };
    let key = world3d_content_bounds_key(minimum, maximum);
    let framed = ui_wgpu::wgpu::frame_projection_orbit_to_spec_bounds(&state.orbit, state.projection_spec, minimum, maximum, state.bounds.w, state.bounds.h, ui_wgpu::wgpu::WORLD_PROJECTION_FRAME_PADDING);
    state.projection_frame_key = Some(key);
    state.projection_frame_zoom = Some(framed.zoom);
    state.projection_frame_owed = false;
    state.orbit = framed;
    advance_world3d_view_revision(state);
}

/// 📦️ React's `worldSceneContentBounds` — instance positions and visible reference footprints, with
/// each axis given the same half-unit floor React's does.
pub(super) fn world3d_content_bounds(state: &World3dState) -> Option<([f32; 3], [f32; 3])> {
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    let mut any = false;
    let mut expand = |point: [f64; 3]| {
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(point[axis]);
            maximum[axis] = maximum[axis].max(point[axis]);
        }
        any = true;
    };
    for (id, position) in &state.instance_positions {
        if state.provisional_instance_ids.contains(id) {
            continue;
        }
        expand(*position);
    }
    for reference in state.references.iter().filter(|reference| !reference.hidden.unwrap_or(false)) {
        let origin = reference.origin.unwrap_or([0.0; 3]);
        let half = f64::from(world_reference_width(reference).max(1e-3)) * 0.5;
        expand([origin[0] - half, origin[1] - half, origin[2]]);
        expand([origin[0] + half, origin[1] + half, origin[2]]);
    }
    if !any || (0..3).any(|axis| !minimum[axis].is_finite() || !maximum[axis].is_finite()) {
        return None;
    }
    let centre = [0, 1, 2].map(|axis| (minimum[axis] + maximum[axis]) * 0.5);
    let half = [0, 1, 2].map(|axis| ((maximum[axis] - minimum[axis]) * 0.5).max(0.5));
    Some(([0, 1, 2].map(|axis| (centre[axis] - half[axis]) as f32), [0, 1, 2].map(|axis| (centre[axis] + half[axis]) as f32)))
}

/// 🔑️ React's `worldSceneContentBoundsKey` — the identity that decides whether a framing is owed.
fn world3d_content_bounds_key(minimum: [f32; 3], maximum: [f32; 3]) -> u64 {
    let mut key = 0xcbf2_9ce4_8422_2325_u64;
    for value in minimum.iter().chain(maximum.iter()) {
        key ^= u64::from((value * 1e3).round() as i32 as u32);
        key = key.wrapping_mul(0x0100_0000_01b3);
    }
    key
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct World3dCameraFitRequest {
    pub(super) revision: u32,
    padding: f32,
    pub(super) bounds: Option<([f32; 3], [f32; 3])>,
}

const WORLD3D_RENDERED_OUTLINE_SCALE: f32 = 1.001;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct World3dCameraFitOwner {
    pub(super) revision: u32,
    camera_digest: Option<u64>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct World3dCameraBoundsCursor {
    pub(super) key: u64,
    pub(super) draw_generation: u64,
    pub(super) geometry_generation: u64,
    pub(super) draw: u16,
    pub(super) instance: u32,
    corner: u8,
    minimum: [f32; 3],
    maximum: [f32; 3],
    pub(super) any: bool,
}

impl World3dCameraBoundsCursor {
    pub(super) fn new(key: u64, draw_generation: u64, geometry_generation: u64) -> Self {
        Self { key, draw_generation, geometry_generation, draw: 0, instance: 0, corner: 0, minimum: [f32::INFINITY; 3], maximum: [f32::NEG_INFINITY; 3], any: false }
    }

    /// 💤️ A completed empty generation parks on the draw index no bounded registry can publish.
    /// Reusing this existing scalar preserves the cursor's `Option` niche and the measured surface
    /// table receipt; another boolean enlarged every admitted World state by eight bytes.
    fn empty_generation(self) -> bool {
        self.draw == u16::MAX
    }

    fn expand(&mut self, point: Vec3) {
        for (axis, value) in [point.x, point.y, point.z].into_iter().enumerate() {
            self.minimum[axis] = self.minimum[axis].min(value);
            self.maximum[axis] = self.maximum[axis].max(value);
        }
        self.any = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dCameraFitStep {
    Idle,
    Pending,
    Applied,
    Stale,
}

fn world3d_camera_fit_key(owner: World3dCameraFitOwner, bounds: Option<([f32; 3], [f32; 3])>) -> u64 {
    let mut key = 0xcbf2_9ce4_8422_2325_u64;
    for value in [u64::from(owner.revision), owner.camera_digest.unwrap_or(0), u64::from(owner.camera_digest.is_some())] {
        key ^= value;
        key = key.wrapping_mul(0x0000_0100_0000_01b3);
    }
    if let Some((minimum, maximum)) = bounds {
        for value in minimum.into_iter().chain(maximum) {
            let rounded = (value * 1e3).round() / 1e3;
            key ^= u64::from(if rounded == 0.0 { 0.0_f32.to_bits() } else { rounded.to_bits() });
            key = key.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    key
}

fn valid_world3d_camera_bounds(minimum: [f32; 3], maximum: [f32; 3]) -> bool {
    (0..3).all(|axis| minimum[axis].is_finite() && maximum[axis].is_finite() && maximum[axis] >= minimum[axis])
        && ((maximum[0] - minimum[0]).powi(2) + (maximum[1] - minimum[1]).powi(2) + (maximum[2] - minimum[2]).powi(2)).sqrt() > 0.0
}

fn apply_world3d_camera_fit(state: &mut World3dState, key: u64, request: World3dCameraFitRequest, minimum: [f32; 3], maximum: [f32; 3]) -> World3dCameraFitStep {
    if !valid_world3d_camera_bounds(minimum, maximum) {
        state.fit_bounds_cursor = None;
        return World3dCameraFitStep::Idle;
    }
    state.orbit = ui_wgpu::wgpu::frame_orbit_to_bounds(&state.orbit, minimum, maximum, state.bounds.w, state.bounds.h, request.padding);
    state.fit_applied_key = Some(key);
    state.fit_bounds_cursor = None;
    advance_world3d_view_revision(state);
    World3dCameraFitStep::Applied
}

/// 🎯️ Retains React's `WorldAutoFit` request. The actual frame waits for the delivered camera and
/// resident draw geometry, then advances through [`step_world3d_camera_fit`] without a traversal burst.
pub(super) fn sync_world3d_scene_fit(state: &mut World3dState, fit_json: Option<&str>) {
    let request = fit_json
        .and_then(|json| serde_json::from_str::<World3dSceneFitRecord>(json).ok())
        .filter(|fit| fit.enabled)
        .map(|fit| World3dCameraFitRequest {
            revision: fit.revision,
            padding: fit.padding.unwrap_or(f64::from(ui_wgpu::wgpu::WORLD_FRAME_BOUNDS_MARGIN)).max(1.0) as f32,
            bounds: fit
                .bounds_min
                .zip(fit.bounds_max)
                .map(|(minimum, maximum)| (minimum.map(|value| value as f32), maximum.map(|value| value as f32)))
                .filter(|(minimum, maximum)| valid_world3d_camera_bounds(*minimum, *maximum)),
        });
    if state.fit_request != request {
        state.fit_request = request;
        state.fit_bounds_cursor = None;
    }
    if request.is_none() {
        state.fit_owner = None;
        state.fit_applied_key = None;
    }
}

/// ⏰️ Whether the fit lane owes another frame. An empty measurement sleeps on the exact residency
/// generations it inspected; a later mesh publication or draw seal differs and wakes it once.
pub(super) fn world3d_camera_fit_work_pending(state: &World3dState) -> bool {
    let Some(request) = state.fit_request else { return false };
    let owner = World3dCameraFitOwner { revision: request.revision, camera_digest: state.scene_camera_digest };
    if state.fit_owner != Some(owner) {
        return true;
    }
    if state.fit_applied_key == Some(world3d_camera_fit_key(owner, request.bounds)) || state.camera_user_moved {
        return false;
    }
    if request.bounds.is_some() {
        return true;
    }
    !state.fit_bounds_cursor.is_some_and(|cursor| {
        cursor.empty_generation() && cursor.key == world3d_camera_fit_key(owner, None) && cursor.draw_generation == state.draw_generation && cursor.geometry_generation == state.geometry_generation
    })
}

/// 📦️ Advances at most one retained draw transition or one transformed mesh-AABB corner. This is
/// Three's `Box3.setFromObject(instancesGroup)` expressed over the resident WGPU draw registry.
pub fn step_world3d_camera_fit(state: &mut World3dState) -> World3dCameraFitStep {
    if state.dynamic_retirement.is_some() {
        state.fit_bounds_cursor = None;
        return World3dCameraFitStep::Idle;
    }
    let Some(request) = state.fit_request else { return World3dCameraFitStep::Idle };
    let owner = World3dCameraFitOwner { revision: request.revision, camera_digest: state.scene_camera_digest };
    if state.fit_owner != Some(owner) {
        state.fit_owner = Some(owner);
        state.fit_bounds_cursor = None;
        state.camera_user_moved = false;
    }
    let key = world3d_camera_fit_key(owner, request.bounds);
    if state.fit_applied_key == Some(key) || state.camera_user_moved {
        state.fit_bounds_cursor = None;
        return World3dCameraFitStep::Idle;
    }
    if state.scene_bridge.is_some()
        || state.snapshot_apply.is_some()
        || state.draw_rebuild.is_some()
        || state.retired_draws.is_some()
        || state.scene_bridge_lease.is_some_and(|lease| state.snapshot_lease != Some(lease))
    {
        return World3dCameraFitStep::Pending;
    }
    sync_world3d_projection_content_frame(state);
    if let Some((minimum, maximum)) = request.bounds {
        return apply_world3d_camera_fit(state, key, request, minimum, maximum);
    }
    let mut cursor = state.fit_bounds_cursor.take().unwrap_or_else(|| World3dCameraBoundsCursor::new(key, state.draw_generation, state.geometry_generation));
    if cursor.key != key || cursor.draw_generation != state.draw_generation || cursor.geometry_generation != state.geometry_generation {
        let completed_empty_generation = cursor.empty_generation();
        cursor = World3dCameraBoundsCursor::new(key, state.draw_generation, state.geometry_generation);
        if !completed_empty_generation {
            state.fit_bounds_cursor = Some(cursor);
            return World3dCameraFitStep::Stale;
        }
    }
    if usize::from(cursor.draw) >= usize::from(state.draws.len) {
        if cursor.any {
            return apply_world3d_camera_fit(state, key, request, cursor.minimum, cursor.maximum);
        }
        cursor.draw = u16::MAX;
        state.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Idle;
    }
    let Some(draw) = state.draws.get(usize::from(cursor.draw)) else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Some(mesh) = state.meshes.get(&draw.mesh_key).copied() else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Ok((local_minimum, local_maximum)) = mesh.aabb() else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Some(instance) = draw.instances.get(cursor.instance as usize) else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let corner = cursor.corner;
    let rendered_scale = if state.show_edges
        && !state.provisional_instance_ids.contains(&instance.id)
        && mesh.schema().is_ok_and(|schema| schema.edges == 0)
    {
        WORLD3D_RENDERED_OUTLINE_SCALE
    } else {
        1.0
    };
    let point = instance.model.transform_point(Vec3::new(
        (if corner & 1 == 0 { local_minimum[0] } else { local_maximum[0] }) * rendered_scale,
        (if corner & 2 == 0 { local_minimum[1] } else { local_maximum[1] }) * rendered_scale,
        (if corner & 4 == 0 { local_minimum[2] } else { local_maximum[2] }) * rendered_scale,
    ));
    if point.x.is_finite() && point.y.is_finite() && point.z.is_finite() {
        cursor.expand(point);
    }
    cursor.corner += 1;
    if cursor.corner == 8 {
        cursor.corner = 0;
        cursor.instance += 1;
    }
    state.fit_bounds_cursor = Some(cursor);
    World3dCameraFitStep::Pending
}

/// 🎯️ `World3dScene.selection_json` — both the plain-instance half (`ids`/`hoveredId`, what the
/// framework `interactionSelect`/`interactionHover` verbs address) and the component half
/// (`granularity`/`componentIds`/`hoveredComponent`, the unconverted `worldPick` mechanism).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct World3dSceneSelectionRecord {
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    ids: Vec<String>,
    #[serde(default)]
    pub(super) gumball_selection_ids: Vec<String>,
    #[serde(default)]
    pub(super) gumball_target: Option<[f32; 3]>,
    #[serde(default)]
    hovered_id: Option<String>,
    #[serde(default)]
    pub(super) granularity: Option<String>,
    #[serde(default)]
    pub(super) component_ids: Vec<serde_json::Value>,
    #[serde(default)]
    hovered_component: Option<World3dSceneHoveredComponent>,
    #[serde(default)]
    pub(super) targets: Option<WorldSelectionTargets>,
    #[serde(default)]
    pub(super) active_object_id: Option<String>,
    #[serde(default)]
    pub(super) show_edges: Option<bool>,
    /// 🎛️ `selection_json.gumballConfig` — the PLUGIN-AUTHORED handle groups, which override the
    /// single-mode `transformMode` and let one gumball offer several groups at once (React's
    /// `WorldSelectionRecord.gumballConfig` / `UnifiedGumball`).
    #[serde(default)]
    gumball_config: Option<World3dSceneGumballConfigRecord>,
    /// 🎛️ `selection_json.transformMode` — the single-mode fallback
    /// (`gumballConfigForTransformMode`) when no `gumballConfig` is authored.
    #[serde(default)]
    pub(super) transform_mode: Option<String>,
    /// ⚡️ `selection_json.gumballLiveDispatch` — React's `WorldSelectionRecord.gumballLiveDispatch`.
    #[serde(default)]
    pub(super) gumball_live_dispatch: Option<bool>,
}

/// 🎛️ `selection_json.gumballConfig` — React's `GumballConfig`
/// (`🌐️World3dHost/🟦️.tsx:236`, `gumballConfigForTransformMode`).
///
/// ⚖️ Every flag is `Option<bool>`, not `bool`: React's `resolveGumballConfig` reads each group as
/// `config?.group !== false`, so an ABSENT key means ENABLED and only an explicit `false` hides a
/// group. A plain `#[serde(default)] bool` would read every authored config as "all groups off".
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct World3dSceneGumballConfigRecord {
    #[serde(default)]
    move_axes: Option<bool>,
    #[serde(default)]
    move_planes: Option<bool>,
    #[serde(default)]
    rotate: Option<bool>,
    #[serde(default)]
    scale_axes: Option<bool>,
    #[serde(default)]
    scale_planes: Option<bool>,
    #[serde(default)]
    scale_uniform: Option<bool>,
    /// 📐️ The planar subset a window projection implies (`xy`/`xz`/`yz`), from React's
    /// `worldGumballConfigForProjection`. `None` offers all three axes.
    #[serde(default)]
    plane: Option<String>,
}

impl World3dSceneGumballConfigRecord {
    /// 🎛️ React's `gumballConfigForTransformMode` (`🌐️World3dHost/🟦️.tsx:2683`) — the single-mode
    /// fallback when a producer authors no `gumballConfig`.
    ///
    /// ⚖️ React switches on three literals (`transform`/`rotate`/`scale`) and falls through to the
    /// move config; this host's own `transform_mode` also carries the verb spellings
    /// (`rotateSelection`/`scaleSelection`, and the default `translate`), so the arms match on the
    /// PREFIX. Same three groups, same fall-through.
    fn for_transform_mode(mode: &str) -> Self {
        let flags = |move_axes, move_planes, rotate, scale_axes, scale_planes, scale_uniform| Self {
            move_axes: Some(move_axes),
            move_planes: Some(move_planes),
            rotate: Some(rotate),
            scale_axes: Some(scale_axes),
            scale_planes: Some(scale_planes),
            scale_uniform: Some(scale_uniform),
            plane: None,
        };
        if mode.starts_with("transform") {
            return flags(true, true, true, false, false, false);
        }
        if mode.starts_with("rotate") {
            return flags(false, false, true, false, false, false);
        }
        if mode.starts_with("scale") {
            return flags(false, false, false, true, true, true);
        }
        flags(true, true, false, false, false, false)
    }

    /// 🎛️ React's `gumballHandleEnabled` — the handle's own group flag ∩ the drafting-plane subset
    /// (`🧱️elements/🎬️Scene/🟦️.tsx:489-506`, `GUMBALL_PLANE_HANDLES`).
    pub(super) fn admits(&self, handle: GumballHandle) -> bool {
        let group = match handle {
            GumballHandle::MoveX | GumballHandle::MoveY | GumballHandle::MoveZ => self.move_axes,
            GumballHandle::MoveXY | GumballHandle::MoveYZ | GumballHandle::MoveXZ => self.move_planes,
            GumballHandle::RotateX | GumballHandle::RotateY | GumballHandle::RotateZ => self.rotate,
            GumballHandle::ScaleX | GumballHandle::ScaleY | GumballHandle::ScaleZ => self.scale_axes,
        };
        if group == Some(false) {
            return false;
        }
        match self.plane.as_deref() {
            None => true,
            Some("xy") => matches!(handle, GumballHandle::MoveX | GumballHandle::MoveY | GumballHandle::MoveXY | GumballHandle::RotateZ | GumballHandle::ScaleX | GumballHandle::ScaleY),
            Some("yz") => matches!(handle, GumballHandle::MoveY | GumballHandle::MoveZ | GumballHandle::MoveYZ | GumballHandle::RotateX | GumballHandle::ScaleY | GumballHandle::ScaleZ),
            Some("xz") => matches!(handle, GumballHandle::MoveX | GumballHandle::MoveZ | GumballHandle::MoveXZ | GumballHandle::RotateY | GumballHandle::ScaleX | GumballHandle::ScaleZ),
            Some(_) => true,
        }
    }
}

/// 🎛️ The gumball config THIS surface offers: the plugin-authored one when it published one,
/// otherwise the single-mode fallback for its `transformMode`.
///
/// 🩸️ Before ticket 26/09/17 packet W14g nothing on this target read `selection.gumballConfig` at
/// all, and the handle gate was a `transform_mode` string match that admitted the three MOVE axes
/// and the three move planes UNCONDITIONALLY — so a `rotate`-mode gumball still offered translation
/// where React offers only the rings, and a plugin that authored several groups at once (the
/// contract puzzle3d's Move/Rotate relies on) got the single-mode set.
pub(super) fn world3d_gumball_config(state: &World3dState) -> World3dSceneGumballConfigRecord {
    state.gumball_config.clone().unwrap_or_else(|| World3dSceneGumballConfigRecord::for_transform_mode(&state.transform_mode))
}

/// 🎯️ `selection_json.hoveredComponent` — the vertex/edge/face under the pointer, if any.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct World3dSceneHoveredComponent {
    #[serde(default)]
    pub(super) object_id: Option<String>,
    #[serde(default)]
    pub(super) mode: Option<String>,
    #[serde(default)]
    pub(super) id: Option<serde_json::Value>,
}

/// 🔢️ A JSON scalar id (`10`, `"10"`) as the decimal string every component registry keys by.
fn scene_component_id_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(text) => Some(text.clone()),
        serde_json::Value::Number(number) => Some(number.to_string()),
        _ => None,
    }
}

/// 🔏️ FNV-1a over the payload lanes that decide whether a bridge rebuild is needed. Content
/// addressing, not a clock: an app that republishes an identical scene must not cost a rebuild.
pub(super) fn world3d_scene_digest(parts: &[&str]) -> u64 {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value_derive::RetireOwned)]
pub(super) enum World3dSceneBridgePhase {
    Parse,
    ScalarAttributes,
    Meshes,
    Pages,
    Close,
}

/// 🌉️ One in-flight `meshes_json`/`instances_json` → `World3dSnapshotLease` build. Parsing, mesh
/// publication and page admission are three separately budgeted phases so a large preview never
/// blocks a frame.
#[derive(semio_framework_value_derive::RetireOwned)]
pub(super) struct World3dSceneBridgeCursor {
    pub(super) digest: u64,
    pub(super) camera_digest: u64,
    pub(super) meshes_json: String,
    pub(super) instances_json: String,
    pub(super) camera_json: String,
    pub(super) camera_changed: bool,
    pub(super) phase: World3dSceneBridgePhase,
    pub(super) meshes: Vec<World3dSceneMeshEntry>,
    pub(super) instances: Vec<World3dSceneInstanceEntry>,
    pub(super) camera: Option<World3dSceneCameraRecord>,
    pub(super) mesh_cursor: usize,
    pub(super) scalar_mesh_cursor:usize,
    pub(super) scalar_slot_cursor:usize,
    pub(super) retiring_mesh: bool,
    pub(super) mesh_digest:Option<WorldMeshDigestCursor>,
    pub(super) scalar_field: Option<ui_wgpu::wgpu::World3dScalarField>,
}

/// 🎯️ The plain-JSON instance lane may retain at most the objects and identifier bytes the
/// interaction registry can actually address. This is the existing native authority, shared by render
/// ids, topology ids and per-instance granularity overrides; the bridge never invents a second cap.
pub(super) fn world3d_scene_instance_interactions_are_bounded(instances: &[World3dSceneInstanceEntry]) -> bool {
    instances.len() <= WORLD_INTERACTION_OBJECT_CAPACITY
        && instances.iter().all(|instance| {
            instance.id.len() <= WORLD_INTERACTION_ID_BYTE_CAPACITY
                && instance.mesh_id.len() <= WORLD_INTERACTION_ID_BYTE_CAPACITY
                && instance.interaction_id.as_ref().is_none_or(|id| id.len() <= WORLD_INTERACTION_ID_BYTE_CAPACITY)
                && instance.interaction_granularity_id.as_ref().is_none_or(|id| id.len() <= WORLD_INTERACTION_ID_BYTE_CAPACITY)
        })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSceneBridgeStep {
    Idle,
    Pending,
    Complete,
    Fault,
}

/// 🌉️ Notes a scene whose producer published no typed snapshot of its own, so the next
/// [`step_world3d_scene_bridge`] turns its plain-JSON mesh/instance lanes into one. Cheap and
/// idempotent: an unchanged payload re-stages nothing.
pub(super) fn stage_world3d_scene_bridge(state: &mut World3dState, world: &ui_wgpu::wgpu::World3dScene) {
    let camera_digest = world3d_scene_digest(&[&world.camera_json]);
    let field_key = world.scalar_field.as_ref().map(|field| world.lanes.iter().find(|lane| lane.lane == "scalarField").map_or_else(|| ui_wgpu::wgpu::scene_lane_hash(&ui_wgpu::wgpu::world3d_modelling_lane_text(field)), |lane| lane.hash.clone())).unwrap_or_default();
    let digest = world3d_scene_digest(&[&world.meshes_json, &world.instances_json, &world.camera_json, &field_key]);
    if state.scene_bridge_digest == Some(digest) || state.scene_bridge.as_ref().is_some_and(|cursor| cursor.digest == digest) {
        return;
    }
    if state.scene_bridge.is_some() || state.scene_bridge_close.is_some() {
        return;
    }
    state.scene_bridge = Some(World3dSceneBridgeCursor {
        digest,
        camera_digest,
        meshes_json: world.meshes_json.clone(),
        instances_json: world.instances_json.clone(),
        camera_json: world.camera_json.clone(),
        camera_changed: state.scene_camera_digest != Some(camera_digest),
        phase: World3dSceneBridgePhase::Parse,
        meshes: Vec::new(),
        instances: Vec::new(),
        camera: None,
        mesh_cursor: 0,
        scalar_mesh_cursor:0,
        scalar_slot_cursor:0,
        retiring_mesh: false,
        mesh_digest:None,
        scalar_field: world.scalar_field.clone(),
    });
}

/// ♻️ Drains one superseded bridge lease, or the whole staged build, one authority step at a time.
/// Returns `true` when it did work — the retirement ladder polls it before anything else so a
/// world that is being torn down never leaves a snapshot slot reserved.
pub(super) fn step_world3d_scene_bridge_close(state: &mut World3dState,context:&mut semio_framework_job::StepContext<'_>) -> bool {
    if let Some(owner)=state.scene_bridge_close.as_mut(){
        let result=owner.step(context.retained_grant());
        let progress=match result{Ok(step)=>step.progress(),Err(_)=>owner.step_progress()};
        context.consume_retained(progress).expect("original scene cursor retirement fits original remaining grant");
        if owner.terminal_is_empty(){state.scene_bridge_close=None;}
        return true;
    }
    if state.scene_bridge.is_some(){
        let grant=context.retained_grant();
        let bytes=size_of::<World3dSceneBridgeCursor>();
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return true;}
        let original=state.scene_bridge.take().unwrap();
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){
            Ok(owner)=>{state.scene_bridge_close=Some(owner);context.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}).expect("original scene cursor handoff fits admitted typed move");},
            Err((_,original))=>state.scene_bridge=Some(original),
        }
        return true;
    }
    let Some((lease, begun)) = state.scene_bridge_retired else {
        let Some(lease) = state.scene_bridge_lease.take() else { return false };
        state.scene_bridge_retired = Some((lease, false));
        return true;
    };
    if !begun {
        match ui_wgpu::wgpu::world3d_snapshot_begin_close(lease) {
            Ok(()) => state.scene_bridge_retired = Some((lease, true)),
            Err(_) => state.scene_bridge_retired = None,
        }
        return true;
    }
    match ui_wgpu::wgpu::world3d_snapshot_close_step(lease) {
        Ok(true) | Err(_) => state.scene_bridge_retired = None,
        Ok(false) => {}
    }
    true
}

/// 🌉️ One budgeted turn of the mesh-wire → snapshot bridge. Drives the staged payload through
/// parse → retained attribute candidates → per-mesh publication → snapshot page admission, then hands the sealed lease to
/// [`sync_world3d_state`], which feeds it to the very same apply ladder a typed producer's own
/// snapshot rides.
///
/// 📐️ The delivered spec seeds the framing plane only while the pane holds none of its
/// own — React's `cameraState.projectionSpec ?? externalPendingProjectionSpec` precedence
/// (`🌐️World3dHost/🟦️.tsx`), the same rule the snapshot apply keeps for the family.
pub fn step_world3d_scene_bridge(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> World3dSceneBridgeStep {
    if context.should_yield() {
        return World3dSceneBridgeStep::Pending;
    }
    if state.scene_bridge_retired.is_some() && state.snapshot_apply.is_none() && state.scene_bridge_lease == state.snapshot_lease {
        step_world3d_scene_bridge_close(state,context);
        context.consume_fuel(1);
        return World3dSceneBridgeStep::Pending;
    }
    if state.scene_bridge_close.is_some()||state.scene_bridge.as_ref().is_some_and(|cursor|matches!(cursor.phase,World3dSceneBridgePhase::Close)){
        step_world3d_scene_bridge_close(state,context);
        context.consume_fuel(1);
        return World3dSceneBridgeStep::Pending;
    }
    let Some(mut cursor) = state.scene_bridge.take() else {
        return World3dSceneBridgeStep::Idle;
    };
    context.consume_fuel(1);
    match cursor.phase {
        World3dSceneBridgePhase::Parse => {
            if cursor.instances_json.len() > WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.phase=World3dSceneBridgePhase::Close;state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;
            }
            cursor.meshes = semio_framework_pack_json::from_json_str::<Vec<World3dSceneMeshEntry>>(&cursor.meshes_json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default();
            cursor.instances = serde_json::from_str::<Vec<World3dSceneInstanceEntry>>(&cursor.instances_json).unwrap_or_default();
            cursor.camera = serde_json::from_str::<World3dSceneCameraRecord>(&cursor.camera_json).ok();
            if let Some(camera) = cursor.camera.as_ref().filter(|_| !state.projection_selected) {
                state.projection_frame_policy = camera.projection_frame;
                if camera.projection_frame == Viewport3dProjectionFramePolicy::PreserveCamera {
                    state.projection_frame_owed = false;
                    state.projection_frame_key = None;
                    state.projection_frame_zoom = None;
                }
            }
            if let Some(camera) = cursor.camera.as_ref().filter(|_| !state.projection_selected) {
                if let Some(spec) = camera.projection {
                    state.projection_spec = spec;
                    if cursor.camera_changed && camera.projection_frame == Viewport3dProjectionFramePolicy::Content {
                        state.projection_frame_owed = true;
                        state.projection_frame_key = None;
                        state.projection_frame_zoom = None;
                        state.camera_user_moved = false;
                    }
                }
            }
            cursor.meshes.retain(World3dSceneMeshEntry::names_a_mesh);
            for mesh in &mut cursor.meshes {
                if mesh.data.normals.len() != mesh.data.positions.len() && !mesh.data.attributes.values().any(|attribute| attribute.semantic == semio_framework::MeshAttributeSemantic::Normal) {
                    mesh.data.compute_normals();
                }
            }
            if !world3d_scene_instance_interactions_are_bounded(&cursor.instances) {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.phase=World3dSceneBridgePhase::Close;state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;
            }
            cursor.instances.retain(|instance| cursor.meshes.iter().any(|mesh| mesh.id == instance.mesh_id));
            state.scalar_field_status=if cursor.scalar_field.is_some(){World3dScalarFieldStatus::Missing}else{World3dScalarFieldStatus::None};
            cursor.phase = World3dSceneBridgePhase::ScalarAttributes;
            state.scene_bridge = Some(cursor);
            World3dSceneBridgeStep::Pending
        }
        World3dSceneBridgePhase::ScalarAttributes=>{
            let Some(field)=cursor.scalar_field.as_ref()else{cursor.phase=World3dSceneBridgePhase::Meshes;state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending};
            let Some(mesh)=cursor.meshes.get_mut(cursor.scalar_mesh_cursor)else{cursor.phase=World3dSceneBridgePhase::Meshes;state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending};
            if mesh.id!=field.mesh_id{cursor.scalar_mesh_cursor+=1;state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;}
            if !world3d_scalar_field_fits(&mesh.data,field){state.scalar_field_status=World3dScalarFieldStatus::Mismatch;cursor.phase=World3dSceneBridgePhase::Meshes;state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;}
            if cursor.scalar_slot_cursor<mesh.data.attributes.slot_count(){
                let result=mesh.data.attributes.retire_slot_if(cursor.scalar_slot_cursor,|_,attribute|attribute.semantic!=semio_framework::MeshAttributeSemantic::Color,context.retained_grant());
                match result{Ok(progress)=>{context.consume_retained(progress).expect("original attribute move fits supplied caller grant");if progress.copied_items!=0{cursor.scalar_slot_cursor+=1;}},Err(_)=>{state.snapshot_fault=Some(World3dSnapshotFault::Capacity);cursor.phase=World3dSceneBridgePhase::Close;state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;}}
                state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;
            }
            world3d_apply_scalar_field_colors(&mut mesh.data,field);
            state.scalar_field_status=World3dScalarFieldStatus::Applied;
            cursor.phase=World3dSceneBridgePhase::Meshes;
            state.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Pending
        }
        World3dSceneBridgePhase::Close=>unreachable!("original scene cursor close is drained before advancing"),
        World3dSceneBridgePhase::Meshes => {
            if state.dynamic_mesh_close.is_some() || state.dynamic_blocked_mesh.is_some() {
                step_world3d_mesh_close(state,context);
                state.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            if state.placeholder_build.is_some() {
                step_world_placeholder_mesh_batch(state, WORLD3D_BRIDGE_MESH_WRITES_PER_STEP);
                state.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let Some(entry) = cursor.meshes.get(cursor.mesh_cursor) else {
                cursor.phase = World3dSceneBridgePhase::Pages;
                state.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            };
            if !entry.has_inline_geometry() {
                let (id, url, kind) = (entry.id.clone(), entry.url.clone(), entry.kind.clone());
                declare_scene_mesh_source(state, &id, url.as_deref(), kind.as_deref());
                cursor.mesh_cursor += 1;
                cursor.retiring_mesh = false;
                state.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let hashing=cursor.mesh_digest.get_or_insert_with(WorldMeshDigestCursor::default);
            let digest=match hashing.step(&entry.data){Ok(Some(digest))=>digest,Ok(None)=>{state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending},Err(_)=>{state.snapshot_fault=Some(World3dSnapshotFault::Capacity);cursor.phase=World3dSceneBridgePhase::Close;state.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault}};
            if state.scene_mesh_digests.get(&entry.id) == Some(&digest) && state.meshes.contains_key(&entry.id) {
                cursor.mesh_cursor += 1;
                cursor.retiring_mesh = false;
                cursor.mesh_digest=None;
                state.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            if state.meshes.contains_key(&entry.id) && !cursor.retiring_mesh {
                cursor.retiring_mesh = true;
                retire_world_mesh(state, &entry.id.clone());
                state.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let id=entry.id.clone();
            let buffers=std::mem::take(&mut cursor.meshes[cursor.mesh_cursor].data);
            state.placeholder_generation = state.placeholder_generation.wrapping_add(1).max(1);
            match WorldPlaceholderMeshCursor::inline(&id, buffers, state.placeholder_generation, state.interaction_revision) {
                Ok(build) => {
                    state.placeholder_build = Some(build);
                    state.scene_mesh_digests.insert(id, digest);
                    cursor.mesh_digest=None;
                    cursor.mesh_cursor += 1;
                    cursor.retiring_mesh = false;
                    state.scene_bridge = Some(cursor);
                    World3dSceneBridgeStep::Pending
                }
                Err(_) => {
                    state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.phase=World3dSceneBridgePhase::Close;state.scene_bridge=Some(cursor);
                World3dSceneBridgeStep::Fault
                }
            }
        }
        World3dSceneBridgePhase::Pages if !world3d_scene_bridge_has_pages(state, &cursor) => {
            state.scene_bridge_digest = Some(cursor.digest);
            if cursor.camera_changed {
                state.scene_camera_digest = Some(cursor.camera_digest);
            }
            cursor.phase=World3dSceneBridgePhase::Close;state.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Complete
        }
        World3dSceneBridgePhase::Pages => match publish_world3d_scene_bridge_snapshot(state, &cursor) {
            Ok(lease) => {
                if let Some(previous) = state.scene_bridge_lease.replace(lease) {
                    state.scene_bridge_retired = Some((previous, false));
                }
                state.scene_bridge_digest = Some(cursor.digest);
                if cursor.camera_changed {
                    state.scene_camera_digest = Some(cursor.camera_digest);
                }
                cursor.phase=World3dSceneBridgePhase::Close;state.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Complete
            }
            Err(fault) => {
                state.snapshot_fault = Some(fault);
                cursor.phase=World3dSceneBridgePhase::Close;state.scene_bridge=Some(cursor);
                World3dSceneBridgeStep::Fault
            }
        },
    }
}

/// 🔏️ Content digest of one mesh's buffers — what decides whether an already-published mesh key
/// can be reused rather than rewritten through the authority ladder.
#[derive(semio_framework_value_derive::RetireOwned)]
struct WorldMeshDigestCursor {phase:usize,item:usize,hash:u64,metadata:semio_framework::mesh_io::text::MeshMetadataCursor,text:String}
impl Default for WorldMeshDigestCursor {fn default()->Self{Self{phase:0,item:0,hash:0xcbf2_9ce4_8422_2325,metadata:Default::default(),text:String::from("{")}}}
impl WorldMeshDigestCursor {
    pub(super) fn bytes(&mut self,bytes:&[u8]) {for byte in bytes{self.hash^=u64::from(*byte);self.hash=self.hash.wrapping_mul(0x0000_0100_0000_01b3);}}
    pub(super) fn step(&mut self,mesh:&WorldMeshBuffers)->Result<Option<u64>,String>{
        let floats=match self.phase {0=>Some(&mesh.positions),1=>Some(&mesh.normals),2=>Some(&mesh.colors),3=>Some(&mesh.uvs),4=>Some(&mesh.edge_positions),5=>Some(&mesh.edge_uvs),_=>None};
        if let Some(values)=floats {if let Some(value)=values.get(self.item){self.bytes(&value.to_bits().to_le_bytes());self.item+=1;return Ok(None)}}
        let integers=match self.phase {6=>Some(&mesh.indices),7=>Some(&mesh.face_ids),8=>Some(&mesh.vertex_ids),9=>Some(&mesh.edge_ids),_=>None};
        if let Some(values)=integers {if let Some(value)=values.get(self.item){self.bytes(&value.to_le_bytes());self.item+=1;return Ok(None)}}
        if self.phase==10 {if let Some(byte)=mesh.edge_is_seam.get(self.item){self.bytes(&[*byte]);self.item+=1;return Ok(None)}}
        if self.phase==11 {if let Some(byte)=mesh.paint_texture_base64.as_ref().and_then(|text|text.as_bytes().get(self.item)){self.bytes(&[*byte]);self.item+=1;return Ok(None)}}
        if self.phase==12 {
            let start=self.text.len();let done=self.metadata.step(&mesh.attributes,&mesh.materials,&mesh.textures,Some(&mesh.component_references),None,None,&mut self.text)?;
            let bytes=self.text.as_bytes()[start..].to_vec();self.bytes(&bytes);let last=self.text.chars().last().unwrap_or('{');self.text.clear();self.text.push(last);
            if done {self.phase=13;return Ok(Some(self.hash))}return Ok(None)
        }
        if self.phase==13{return Ok(Some(self.hash))}
        self.bytes(&(self.item as u64).to_le_bytes());self.bytes(&[self.phase as u8]);self.phase+=1;self.item=0;Ok(None)
    }
}

/// 🧹️ One turn of the superseded-mesh close ladder — the same steps the full-state retirement runs,
/// reachable outside a teardown so a live scene can replace a mesh key without wedging.
pub(super) fn step_world3d_mesh_close(state:&mut World3dState,context:&mut semio_framework_job::StepContext<'_>){
    if context.retirement_should_yield(){return}
    if state.dynamic_mesh_close.is_none(){if let Some(entry)=state.dynamic_blocked_mesh.take(){state.dynamic_mesh_close=Some(entry);}return}
    let entry=state.dynamic_mesh_close.as_mut().unwrap();let grant=context.retained_grant();
    let result=entry.value.close_step(grant);let Ok((complete,progress))=result else{return};
    if context.consume_retained(progress).is_err(){return}if progress.copied_items!=0{context.consume_fuel(progress.copied_items as u64);return}
    if !complete{return}
    let Ok(step)=world_close_string(&mut entry.id,grant)else{return};if context.consume_retained(step.progress()).is_err(){return}if step.progress().copied_items!=0{context.consume_fuel(1);}if matches!(step,semio_framework_job::InteractiveJobCloseStep::Complete{..}){state.dynamic_mesh_close.take();}
}

/// 🧊️ Drains up to `budget` authority writes of the in-flight mesh publication.
fn step_world_placeholder_mesh_batch(state: &mut World3dState, budget: u32) {
    for _ in 0..budget {
        if state.placeholder_build.is_none() {
            return;
        }
        step_world_placeholder_mesh(state);
    }
}

/// 🌉️ Builds and seals the snapshot the bridge publishes: one `Mesh` page per draw (flags `31`,
/// carrying both this draw's and the whole snapshot's credits) followed by that draw's `Instance`
/// pages, then a `Camera` page when the window's camera measure itself changed. Every page is built
/// and validated BEFORE the store is touched, so admission can only fail on a bug.
/// 🌉️ Whether a staged build has anything a snapshot can carry: at least one published mesh with at
/// least one instance, or a camera page. A snapshot is at least one page by construction
/// (`world3d_snapshot_begin` refuses `page_count == 0`), so a build with neither publishes NOTHING —
/// it is not a capacity fault.
///
/// 🩸️ It used to go straight to [`publish_world3d_scene_bridge_snapshot`], whose `pages.is_empty()`
/// guard answers `World3dSnapshotFault::Capacity`, and the wgpu host quarantines the surface on any
/// bridge fault. Every mesh update that momentarily published no DRAWABLE geometry therefore killed
/// the preview for good: measured on 6118 after the third Form slider edit, where the guest's
/// intermediate wire was `meshes=169b instances=267b` — mesh records the parse phase drops for
/// carrying no triangles — against `3644b`/`4786b` for every settled sample, and the surface
/// quarantined with `fault=Some(Capacity) state-meshes=3`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.3).
/// The first build of a surface survived only by accident: its camera digest is always new, so it
/// always had a camera page.
fn world3d_scene_bridge_has_pages(state: &World3dState, cursor: &World3dSceneBridgeCursor) -> bool {
    if cursor.camera_changed && cursor.camera.is_some() {
        return true;
    }
    cursor.meshes.iter().any(|mesh| (state.meshes.contains_key(&mesh.id) || scene_mesh_awaits_its_asset(state, &mesh.id)) && cursor.instances.iter().any(|instance| instance.mesh_id == mesh.id))
}

/// 🥽️ A url-declared mesh has no geometry yet and still earns its draw: the draw is what
/// `render_world_3d`'s missing-mesh loop reads to reserve the GLB fetch, and what starts
/// painting by itself the frame the decoded mesh lands under this key. Dropping it here was
/// the second half of the dead url lane — the bridge kept the instances and then threw the
/// draw away, so `state-draws=0 state-instances=0` (`📓️w3a-asset-decoder-boot-fault.md` §6).
///
/// 🌫️ Locked dimming rides the authored colour's alpha, so it survives the snapshot
/// page (which carries a colour, not a style) and composes with whatever
/// `MESH_STYLE_PAINT` row the instance later resolves to.
fn publish_world3d_scene_bridge_snapshot(state: &mut World3dState, cursor: &World3dSceneBridgeCursor) -> Result<World3dSnapshotLease, World3dSnapshotFault> {
    let (neutral, neutral_source) = scene_bridge_neutral_color(state);
    state.instance_interaction_ids.clear();
    for instance in cursor.instances.iter().filter(|instance| instance.interaction_id.as_deref().is_some_and(|target| target != instance.id)) {
        state.instance_interaction_ids.insert(instance.id.clone(), instance.interaction_id.clone().expect("instance interaction id filtered above"));
    }
    state.instance_interaction_granularity_ids.clear();
    for instance in cursor.instances.iter().filter(|instance| instance.interaction_granularity_id.is_some()) {
        state.instance_interaction_granularity_ids.insert(instance.id.clone(), instance.interaction_granularity_id.clone().expect("instance interaction granularity filtered above"));
    }
    state.instance_positions.clear();
    for instance in &cursor.instances {
        state.instance_positions.insert(instance.id.clone(), instance.position.unwrap_or([0.0; 3]));
    }
    state.provisional_instance_ids = cursor.instances.iter().filter(|instance| instance.provisional).map(|instance| instance.id.clone()).collect();
    state.highlighted_instance_ids = cursor.instances.iter().filter(|instance| instance.highlighted).map(|instance| instance.id.clone()).collect();
    state.disabled_instance_ids = cursor.instances.iter().filter(|instance| instance.disabled).map(|instance| instance.id.clone()).collect();
    state.celebrating_instance_ids = cursor.instances.iter().filter(|instance| instance.celebrating).map(|instance| instance.id.clone()).collect();
    let mut draws: Vec<(&World3dSceneMeshEntry, Vec<&World3dSceneInstanceEntry>)> = Vec::new();
    for mesh in &cursor.meshes {
        if !state.meshes.contains_key(&mesh.id) && !scene_mesh_awaits_its_asset(state, &mesh.id) {
            continue;
        }
        let instances: Vec<&World3dSceneInstanceEntry> = cursor.instances.iter().filter(|instance| instance.mesh_id == mesh.id).collect();
        if !instances.is_empty() {
            draws.push((mesh, instances));
        }
    }
    let draw_count = u16::try_from(draws.len()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let instance_total = u32::try_from(draws.iter().map(|(_, instances)| instances.len()).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let draw_bytes = u32::try_from(draws.iter().map(|(mesh, instances)| mesh.id.len() + instances.iter().map(|instance| instance.id.len()).sum::<usize>()).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let mut pages: Vec<ui_wgpu::wgpu::World3dSnapshotPage> = Vec::new();
    for (ordinal, (mesh, instances)) in draws.iter().enumerate() {
        let own_instances = u32::try_from(instances.len()).map_err(|_| World3dSnapshotFault::Capacity)?;
        let own_bytes = u32::try_from(mesh.id.len() + instances.iter().map(|instance| instance.id.len()).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
        let mut page = ui_wgpu::wgpu::World3dSnapshotPage::new(World3dSnapshotPageKind::Mesh);
        let span = page.push_string(&mesh.id)?;
        page.push_item(World3dSnapshotItem {
            strings: [Some(span), None, None, None],
            indexes: [own_instances, own_bytes, u32::try_from(ordinal).map_err(|_| World3dSnapshotFault::Capacity)?, u32::from(draw_count), instance_total, draw_bytes, 0, 0],
            index_len: 6,
            flags: 31,
            ..Default::default()
        })?;
        page.seal()?;
        pages.push(page);
        for chunk in instances.chunks(ui_wgpu::wgpu::WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY.min(ui_wgpu::wgpu::WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY / (WORLD_DYNAMIC_ID_BYTE_CAPACITY + 128))) {
            let mut page = ui_wgpu::wgpu::World3dSnapshotPage::new(World3dSnapshotPageKind::Instance);
            for instance in chunk {
                let span = page.push_string(&instance.id)?;
                let source_spans = match instance.component_source.as_ref() {
                    Some(source) => {
                        ComponentSource3d::new(&source.handle, &source.revision).ok_or(World3dSnapshotFault::PageState)?;
                        [Some(page.push_string(&source.handle)?), Some(page.push_string(&source.revision)?)]
                    }
                    None => [None, None],
                };
                let position = instance.position.unwrap_or([0.0; 3]);
                let rotation = instance.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                let scale = instance.scale.unwrap_or([1.0; 3]);
                let (mut color, color_source) = instance.color.as_deref().map_or((neutral, neutral_source), |color| (parse_color(color), SceneColorSource3d::Authored));
                if let Some(opacity) = instance.opacity {
                    color[3] *= (opacity as f32).clamp(0.0, 1.0);
                }
                page.push_item(World3dSnapshotItem {
                    strings: [Some(span), source_spans[0], source_spans[1], None],
                    numbers: [position[0], position[1], position[2], rotation[0], rotation[1], rotation[2], rotation[3], scale[0], scale[1], scale[2], f64::from(color[0]), f64::from(color[1]), f64::from(color[2]), f64::from(color[3]), 0.0, 0.0],
                    number_len: 14,
                    flags: color_source.snapshot_flag(),
                    ..Default::default()
                })?;
            }
            page.seal()?;
            pages.push(page);
        }
    }
    if let Some(camera) = cursor.camera.as_ref().filter(|_| cursor.camera_changed) {
        let position = camera.position.unwrap_or([4.0, 4.0, 4.0]);
        let target = camera.target.unwrap_or([0.0; 3]);
        let up = camera.up.unwrap_or([0.0, 0.0, 1.0]);
        let parallel = camera.projection().is_parallel();
        let mut page = ui_wgpu::wgpu::World3dSnapshotPage::new(World3dSnapshotPageKind::Camera);
        page.push_item(World3dSnapshotItem {
            numbers: [position[0], position[1], position[2], target[0], target[1], target[2], up[0], up[1], up[2], camera.fov_degrees(), camera.zoom.unwrap_or(1.0), if parallel { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0],
            number_len: 12,
            ..Default::default()
        })?;
        page.seal()?;
        pages.push(page);
    }
    if pages.is_empty() {
        return Err(World3dSnapshotFault::Capacity);
    }
    let page_count = u16::try_from(pages.len()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let item_count = u32::try_from(pages.iter().map(ui_wgpu::wgpu::World3dSnapshotPage::item_count).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let byte_count = u32::try_from(pages.iter().map(ui_wgpu::wgpu::World3dSnapshotPage::byte_count).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
    state.scene_bridge_generation = state.scene_bridge_generation.wrapping_add(1).max(1);
    let revision = state.interaction_revision.checked_add(1).ok_or(World3dSnapshotFault::Capacity)?;
    let descriptor =
        ui_wgpu::wgpu::World3dSnapshotDescriptor { revision, generation: state.scene_bridge_generation, page_count, item_count, byte_count, draw_count, draw_instance_count: instance_total, draw_byte_count: draw_bytes };
    let token = ui_wgpu::wgpu::world3d_snapshot_begin(descriptor)?;
    for page in pages {
        if let Err(rejected) = ui_wgpu::wgpu::world3d_snapshot_admit_page(token, page) {
            let fault = rejected.fault;
            let _ = ui_wgpu::wgpu::world3d_snapshot_abort_write(token);
            for _ in 0..usize::from(page_count) + 2 {
                if ui_wgpu::wgpu::world3d_snapshot_abort_write_step(token).unwrap_or(true) {
                    break;
                }
            }
            return Err(fault);
        }
    }
    ui_wgpu::wgpu::world3d_snapshot_seal(token)
}

/// 🎨️ The base color an instance without its own `color` paints with: the scene environment's
/// neutral material override when it declares one, else a theme-independent neutral the shaded pass
/// lights (the React reference resolves the same fallback from its `neutral` mesh-style palette).
fn scene_bridge_neutral_color(state: &World3dState) -> ([f32; 4], SceneColorSource3d) {
    state
        .environment
        .material
        .as_ref()
        .and_then(|material| material.color.as_deref())
        .map_or(([1.0; 4], SceneColorSource3d::SemanticNeutral), |color| (parse_color(color), SceneColorSource3d::Environment))
}

/// 🎯️ Applies the scene's own selection document to this world's live selection/hover channels —
/// only when the document itself changed, so an optimistic local preview between two identical
/// refreshes is never clobbered (the rule the React host follows too).
///
/// 🎛️ Authored config WINS over the mode, exactly as React's `gumballConfig ?? gumballConfigForTransformMode(mode)`.
pub(super) fn sync_world3d_scene_selection(state: &mut World3dState, selection_json: &str) {
    let digest = world3d_scene_digest(&[selection_json]);
    if state.scene_selection_digest == Some(digest) {
        return;
    }
    state.scene_selection_digest = Some(digest);
    let Ok(record) = serde_json::from_str::<World3dSceneSelectionRecord>(selection_json) else { return };
    let component_ids:Vec<String>=record.component_ids.iter().filter_map(scene_component_id_text).collect();
    let (hovered_object,hovered_mode,hovered_id)=record.hovered_component.map_or((None,None,None),|hovered|(hovered.object_id,hovered.mode,hovered.id.as_ref().and_then(scene_component_id_text)));
    let gumball_target=record.gumball_target.filter(|target|target.iter().all(|value|value.is_finite()));
    let changed=state.selected_ids!=record.ids || state.gumball_selection_ids!=record.gumball_selection_ids || state.gumball_target!=gumball_target
        || state.local_hover_id!=record.hovered_id || state.local_hover_granularity_id.is_some() || state.component_ids!=component_ids
        || state.hovered_component_object_id!=hovered_object || state.hovered_component_mode!=hovered_mode || state.hovered_component_id!=hovered_id
        || state.active_object_id!=record.active_object_id || state.gumball_config!=record.gumball_config || state.gumball_live_dispatch!=record.gumball_live_dispatch.unwrap_or(false)
        || record.method.as_ref().is_some_and(|method|state.selection_method!=*method) || record.granularity.as_ref().is_some_and(|granularity|state.granularity!=*granularity)
        || record.targets.as_ref().is_some_and(|targets|state.selection_targets!=*targets) || record.show_edges.is_some_and(|show_edges|state.show_edges!=show_edges)
        || record.transform_mode.as_ref().is_some_and(|mode|state.transform_mode!=*mode);
    if let Some(method) = record.method {
        state.selection_method = method;
    }
    state.selected_ids = record.ids;
    state.gumball_selection_ids = record.gumball_selection_ids;
    state.gumball_target = gumball_target;
    state.local_hover_id = record.hovered_id;
    state.local_hover_granularity_id = None;
    if let Some(granularity) = record.granularity {
        state.granularity = granularity;
    }
    state.component_ids = component_ids;
    state.hovered_component_object_id = hovered_object;
    state.hovered_component_mode = hovered_mode;
    state.hovered_component_id = hovered_id;
    if let Some(targets) = record.targets {
        state.selection_targets = targets;
    }
    state.active_object_id = record.active_object_id;
    if let Some(show_edges) = record.show_edges {
        state.show_edges = show_edges;
    }
    if let Some(transform_mode) = record.transform_mode {
        state.transform_mode = transform_mode;
    }
    state.gumball_config = record.gumball_config;
    state.gumball_live_dispatch = record.gumball_live_dispatch.unwrap_or(false);
    if changed {advance_world3d_view_revision(state);}
}

/// 🎯️ Constrains the live selection channels to the scene's pick granularity filter, the native twin of
/// `world3dSelectionWithPickFilter` (`🌐️World3dHost/📏️modelling`): hover and selection reach only the chosen
/// granularity, components of another granularity are dropped, and no filter leaves the guest-authored channels alone.
/// Idempotent: it advances the view revision only when it changed something.
pub(super) fn apply_world3d_pick_filter(state: &mut World3dState, filter: Option<ui_wgpu::wgpu::World3dPickGranularity>) {
    use ui_wgpu::wgpu::World3dPickGranularity as Granularity;
    let Some(filter) = filter else { return };
    let (granularity, targets) = match filter {
        Granularity::Shape => ("mesh", WorldSelectionTargets { vertex: false, edge: false, face: false }),
        Granularity::Face => ("face", WorldSelectionTargets { vertex: false, edge: false, face: true }),
        Granularity::Edge => ("edge", WorldSelectionTargets { vertex: false, edge: true, face: false }),
        Granularity::Vertex => ("vertex", WorldSelectionTargets { vertex: true, edge: false, face: false }),
    };
    let component_filter = filter != Granularity::Shape;
    let keep_components = component_filter && state.granularity == granularity;
    let keep_hovered = component_filter && state.hovered_component_mode.as_deref() == Some(granularity);
    let mut changed = false;
    if state.granularity != granularity {
        state.granularity = granularity.to_string();
        changed = true;
    }
    if state.selection_targets != targets {
        state.selection_targets = targets;
        changed = true;
    }
    if !keep_components && !state.component_ids.is_empty() {
        state.component_ids.clear();
        changed = true;
    }
    if !keep_hovered && (state.hovered_component_mode.is_some() || state.hovered_component_id.is_some() || state.hovered_component_object_id.is_some()) {
        state.hovered_component_mode = None;
        state.hovered_component_id = None;
        state.hovered_component_object_id = None;
        changed = true;
    }
    if changed {
        advance_world3d_view_revision(state);
    }
}

pub(super) fn srgb_byte_to_linear(byte: u8) -> f32 {
    let value = f32::from(byte) / 255.0;
    if value <= 0.04045 { value / 12.92 } else { ((value + 0.055) / 1.055).powf(2.4) }
}

/// 🌡️ Validates the original field cardinality before any mesh attribute is displaced.
pub(super) fn world3d_scalar_field_fits(data: &WorldMeshBuffers, field: &ui_wgpu::wgpu::World3dScalarField) -> bool {
    use ui_wgpu::wgpu::World3dScalarDomain;
    let expected = match field.domain {
        World3dScalarDomain::Vertex => data.vertex_count(),
        World3dScalarDomain::Face => data.indices.len() / 3,
    };
    field.values.len()==expected
}

pub(super) fn world3d_apply_scalar_field_colors(data:&mut WorldMeshBuffers,field:&ui_wgpu::wgpu::World3dScalarField){
    use ui_wgpu::wgpu::World3dScalarDomain;
    let bytes = field.color_bytes();
    let linear: Vec<[f32; 4]> = bytes.chunks_exact(3).map(|rgb| [srgb_byte_to_linear(rgb[0]), srgb_byte_to_linear(rgb[1]), srgb_byte_to_linear(rgb[2]), 1.0]).collect();
    let colored: Vec<String> = data.attributes.iter().filter(|(_, attribute)| attribute.semantic == semio_framework::MeshAttributeSemantic::Color).map(|(name, _)| name.clone()).collect();
    for name in colored {
        data.attributes.remove(name.as_str());
    }
    match field.domain {
        World3dScalarDomain::Vertex => data.colors = linear.into_iter().flatten().collect(),
        World3dScalarDomain::Face => {
            data.colors.clear();
            let values = linear.into_iter().map(|color| semio_framework_value::DslValue::Array(color.into_iter().map(|channel| semio_framework_value::DslValue::float(f64::from(channel))).collect())).collect();
            data.attributes.insert(
                "scalarField".into(),
                semio_framework::MeshAttribute { domain: semio_framework::MeshAttributeDomain::Face, semantic: semio_framework::MeshAttributeSemantic::Color, interpolation: semio_framework::MeshAttributeInterpolation::Constant, values, indices: None },
            );
        }
    }
}

/// 🖼️ Applies the scene's vortex/attraction/target-volume/reference JSON lanes — the same payloads
/// React's `World3dHost` parses every frame, but which the wgpu host had never copied into
/// `World3dState` (so document-tree references never reached `render_world_3d`'s textured planes).
///
/// 🧲️ Caps applied at PARSE time (see `world_pick_targets_from_json`), so no later reader has to
/// know the bound and the retained set can never exceed it however large the lane arrives.
pub(super) fn sync_world3d_scene_document_lanes(state: &mut World3dState, world: &ui_wgpu::wgpu::World3dScene) {
    let digest = world3d_scene_digest(&[
        world.vortices_json.as_deref().unwrap_or(""),
        world.attractions_json.as_deref().unwrap_or(""),
        world.target_volumes_json.as_deref().unwrap_or(""),
        world.references_json.as_deref().unwrap_or(""),
        world.engagement_preview_json.as_deref().unwrap_or(""),
        world.pick_targets_json.as_deref().unwrap_or(""),
    ]);
    if state.scene_document_lanes_digest == Some(digest) {
        return;
    }
    state.scene_document_lanes_digest = Some(digest);
    state.vortices = world.vortices_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    state.attractions = world.attractions_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    state.target_volumes = world.target_volumes_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    let references: Vec<WorldReferenceRecord> = world.references_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    reconcile_world3d_reference_urls(state, &references);
    state.references = references;
    state.engagement_preview = world.engagement_preview_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    state.pick_targets = world.pick_targets_json.as_deref().map(world_pick_targets_from_json).unwrap_or_default();
    if state.pick_hover_key.as_deref().is_some_and(|key| !state.pick_targets.iter().any(|target| world_pick_keys_match(Some(target.key().as_str()), Some(key)))) {
        state.pick_hover_key = None;
    }
    state.interaction_revision = state.interaction_revision.wrapping_add(1);
}

fn reconcile_world3d_reference_urls(state: &mut World3dState, references: &[WorldReferenceRecord]) {
    let removed: Vec<String> = state.reference_pixels.keys().filter(|url| !references.iter().any(|reference| reference.url.as_deref() == Some(url.as_str()))).cloned().collect();
    for url in removed {
        if !retire_world_pixels(state, &url, false) {
            break;
        }
    }
    state.pending_image_urls.retain(|url| references.iter().any(|reference| reference.url.as_ref() == Some(url)));
    state.asset_io.cancel_reference_image_urls_except(references);
}

/// 🖼️ Whether the active document still owns `url`; a decoded response must answer this in addition
/// to the pane-generation guard because the same live pane may replace its reference while bytes are
/// in flight.
pub fn world3d_reference_url_is_current(state: &World3dState, url: &str) -> bool {
    state.references.iter().any(|reference| reference.url.as_deref() == Some(url))
}
//#endregion 🌉️World3dSceneBridge
