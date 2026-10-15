//! 🌐️ Application-neutral 3D world canvas: mesh loading, orbit camera, picking, and marquee selection.

#[cfg(test)]
use ui_wgpu::wgpu::{gizmo, pick_closest_mesh_url, ray_pick_instance, ray_pick_mesh_detail};

use crate::framework_surface_terrain::TerrainSessionCore;
use semio_framework_ui_viewport::{Viewport3dProjectionFramePolicy, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
// 🧩️ Every name below is target-neutral (ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS's
// wgpu-tier split): `draw_text`/`WidgetContext`/the paint half of `gizmo` are genuinely GPU-adjacent
// (font/icon atlases) and are imported locally inside `render_world_3d`, the one function that is
// itself `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`-gated instead.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{aabb_intersects_frustum, directional_shadow_frustum_planes, directional_shadow_view_projection, frustum_planes, paint_selection_marquee, transform_aabb, LineDraw3d, SceneLighting3d, SceneMaterial3d, Mat4Math, ProceduralGrid3d, SceneAuthoredMaterial3d, SceneInstanceMaterial3d, SceneMaterialAlpha3d, SceneMaterialDraw3d, SceneMaterialKind3d, ScenePass3d, SceneSectionRole3d, SceneShadow3d, SceneShadowRole3d, SceneViewportMask3d, TexturedDraw3d, TexturedInstance3d, ICON_SHADOW_MAP_SIZE, WORLD_SHADOW_MAP_SIZE};
#[cfg(test)] use ui_wgpu::wgpu::{grid_placement_anchor};
use ui_wgpu::wgpu::axis_rotate_angle;
use ui_wgpu::wgpu::camera_grid_fade_distance;
use ui_wgpu::wgpu::gumball_extent;
use ui_wgpu::wgpu::gumball_eye;
use ui_wgpu::wgpu::gumball_project_ray_onto_axis;
use ui_wgpu::wgpu::interpolate_mesh_uv;
use ui_wgpu::wgpu::lod_from_camera_distance;
use ui_wgpu::wgpu::lod_grid_fade_alpha;
use ui_wgpu::wgpu::lod_grid_step_world;
use ui_wgpu::wgpu::lod_orbit_distance_for_camera;
use ui_wgpu::wgpu::marquee_is_crossing_from_path;
use ui_wgpu::wgpu::mesh3d_abort;
use ui_wgpu::wgpu::mesh3d_abort_step;
use ui_wgpu::wgpu::mesh3d_allocate_step;
use ui_wgpu::wgpu::mesh3d_begin;
use ui_wgpu::wgpu::mesh3d_begin_close;
use ui_wgpu::wgpu::mesh3d_close_step;
use ui_wgpu::wgpu::mesh3d_seal;
use ui_wgpu::wgpu::mesh3d_terminal_is_empty;
use ui_wgpu::wgpu::mesh3d_write_u32;
use ui_wgpu::wgpu::mesh3d_write_vec3;
use ui_wgpu::wgpu::mesh3d_write_vec4;
use ui_wgpu::wgpu::quat_from_basis;
use ui_wgpu::wgpu::ray_aabb_slab;
use ui_wgpu::wgpu::ray_plane_point;
use ui_wgpu::wgpu::ray_segment_distance;
use ui_wgpu::wgpu::rotate_vector;
use ui_wgpu::wgpu::world3d_snapshot_claim_draw_permit;
use ui_wgpu::wgpu::world3d_snapshot_with_page;
use ui_wgpu::wgpu::ActionDescriptor;
use ui_wgpu::wgpu::Camera3d;
use ui_wgpu::wgpu::CameraProjection3d;
use ui_wgpu::wgpu::HitKind;
use ui_wgpu::wgpu::HitTarget;
use ui_wgpu::wgpu::{ComponentSource3d, Instance3d};
use ui_wgpu::wgpu::LineVertex3d;
use semio_framework_ui_locale::LocalizedLabel;
use ui_wgpu::wgpu::Mat4;
use ui_wgpu::wgpu::Mesh3dField;
use ui_wgpu::wgpu::Mesh3dLease;
use ui_wgpu::wgpu::Mesh3dSchema;
use ui_wgpu::wgpu::Mesh3dWriteToken;
use ui_wgpu::wgpu::OrbitController;
use ui_wgpu::wgpu::PointerModifiers;
use ui_wgpu::wgpu::PreparedRasterProducer;
use ui_wgpu::wgpu::PreparedRasterRejected;
use ui_wgpu::wgpu::PreparedRenderEviction;
use ui_wgpu::wgpu::PreparedRenderUpload;
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::SceneColorSource3d;
use ui_wgpu::wgpu::SceneDraw3d;
use ui_wgpu::wgpu::SceneRasterBegin;
use ui_wgpu::wgpu::SceneRasterDescriptor;
use ui_wgpu::wgpu::SceneRasterIdentity;
use ui_wgpu::wgpu::SceneRasterLease;
#[cfg(test)] use ui_wgpu::wgpu::SceneRasterMeshSeal;
use ui_wgpu::wgpu::SceneRasterPool;
use ui_wgpu::wgpu::SceneRasterProfile;
use ui_wgpu::wgpu::SceneRasterWriteMode;
use ui_wgpu::wgpu::UiComponentSceneNode;
use ui_wgpu::wgpu::Vec3;
use ui_wgpu::wgpu::World3dPresentation;
use ui_wgpu::wgpu::World3dPresentationClear;
use ui_wgpu::wgpu::World3dSnapshotDrawPermit;
use ui_wgpu::wgpu::World3dSnapshotFault;
use ui_wgpu::wgpu::World3dSnapshotItem;
use ui_wgpu::wgpu::World3dSnapshotLease;
use ui_wgpu::wgpu::World3dSnapshotPageKind;
#[cfg(test)]
use ui_wgpu::wgpu::{screen_select_components, screen_select_instances};

//#region 📦️PreparedWorldResources
const WORLD3D_FRAME_RESOURCE_CAPACITY: usize = 256;

const WORLD3D_RESOURCE_PAGE: usize = 16;
type WorldSlots<T> = semio_framework_async::OriginalFixedSlotPages<T,WORLD3D_RESOURCE_PAGE,{WORLD3D_FRAME_RESOURCE_CAPACITY/WORLD3D_RESOURCE_PAGE}>;

/// 🧯️ Retirement steps a refused raster admission is given to reach terminal — one page of its source
/// per step, so a 16 MiB item needs 1 024 of them. The bound exists only so a poisoned ledger lock
/// cannot spin forever; reaching it leaks that one credit, which a lock that is already poisoned has
/// lost anyway.
const WORLD_RASTER_REFUSAL_CLOSE_STEPS: u32 = 8_192;

fn world_close_string_demand(key:&String)->semio_framework_value::RetainedCloneGrant{semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:if key.is_empty(){key.capacity()}else{0},maximum_depth:1,..Default::default()}}
fn world_close_string(key:&mut String,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
    use semio_framework_job::InteractiveJobCloseStep as Close;
    if key.is_empty()&&key.capacity()==0{return Ok(Close::Complete{progress:Default::default()})}
    let demand=world_close_string_demand(key);
    if grant.maximum_items==0||grant.maximum_depth<demand.maximum_depth||grant.maximum_release_bytes<demand.maximum_release_bytes{return Ok(Close::Pending{progress:Default::default()})}
    if !key.is_empty(){key.pop();}else{drop(std::mem::take(key));}
    Ok(Close::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:demand.maximum_release_bytes,..Default::default()}})
}

pub enum World3dBuildRejected {
    Upload(PreparedRenderUpload),
    RasterProducer(PreparedRasterProducer),
    RasterAdmission(PreparedRasterRejected),
    Eviction(PreparedRenderEviction),
}

impl World3dBuildRejected {
    pub fn next_close_demands(&self,copy:usize)->Result<semio_framework_value::RetainedCloneGrant,semio_framework_value::ValueError>{
        match self{Self::Upload(owner)=>owner.next_close_demands(copy),Self::RasterProducer(owner)=>owner.next_close_demands(copy),Self::RasterAdmission(owner)=>owner.next_close_demands(copy),Self::Eviction(PreparedRenderEviction::Mesh{key})=>Ok(world_close_string_demand(key))}
    }
    pub fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        match self{Self::Upload(owner)=>owner.close_step(grant),Self::RasterProducer(owner)=>{owner.begin_close();owner.close_step(grant)},Self::RasterAdmission(owner)=>owner.close_step(grant),Self::Eviction(PreparedRenderEviction::Mesh{key})=>world_close_string(key,grant)}
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Upload(owner) => owner.terminal_is_empty(),
            Self::RasterProducer(producer) => producer.terminal_is_empty(),
            Self::RasterAdmission(rejected) => rejected.terminal_is_empty(),
            Self::Eviction(PreparedRenderEviction::Mesh { key }) => key.is_empty()&&key.capacity()==0,
        }
    }
}

/// 📦️ Worker-owned World3d resource requests. It contains only CPU buffers and cache keys; device,
/// queue, texture, and surface authority remain in the prepared renderer presenter.
pub struct World3dBuildContext {
    uploads: WorldSlots<PreparedRenderUpload>,
    upload_len: usize,
    raster_producers: WorldSlots<PreparedRasterProducer>,
    raster_producer_len: usize,
    evictions: WorldSlots<PreparedRenderEviction>,
    eviction_len: usize,
    mesh_requests: WorldSlots<(String, u64)>,
    mesh_request_len: usize,
    raster_requests: WorldSlots<String>,
    raster_request_len: usize,
    rejected: Option<World3dBuildRejected>,
    cursor_wake: std::mem::ManuallyDrop<Option<WorldCursorWakeAuthority>>,
    cursor_wake_retirement: Option<semio_framework_value::retirement::shared::SharedControlledRetirement<Mutex<WorldCursorWakeState>>>,
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

#[derive(Clone,Copy,Default)]
struct WorldCursorWakeState {
    generation: u64,
    acknowledged_generation: u64,
    pending_generation: Option<u64>,
    close_phase: u8,
    closing: bool,
    terminal: bool,
}

semio_framework_value::artifact_retire_leaf!(WorldCursorWakeState);

#[derive(Clone, Default)]
pub struct WorldCursorWakeAuthority(std::sync::Arc<Mutex<WorldCursorWakeState>>);

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
            uploads: WorldSlots::new(),
            upload_len: 0,
            raster_producers: WorldSlots::new(),
            raster_producer_len: 0,
            evictions: WorldSlots::new(),
            eviction_len: 0,
            mesh_requests: WorldSlots::new(),
            mesh_request_len: 0,
            raster_requests: WorldSlots::new(),
            raster_request_len: 0,
            rejected: None,
            cursor_wake:std::mem::ManuallyDrop::new(Some(cursor_wake)),
            cursor_wake_retirement:None,
            cursor_wake_token: None,
            cursor_wake_fault: None,
        }
    }

    pub fn prepared(&self)->bool{self.uploads.prepared()&&self.raster_producers.prepared()&&self.evictions.prepared()&&self.mesh_requests.prepared()&&self.raster_requests.prepared()}
    /// 🎟️ Allocates one physical slot page from the same original context before resource admission.
    pub fn prepare_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if cx.is_cancelled(){return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original world page birth canceled"),cx))}
        if self.prepared(){return Ok(true)}
        let bytes=if !self.uploads.prepared(){self.uploads.birth_bytes()}else if !self.raster_producers.prepared(){self.raster_producers.birth_bytes()}else if !self.evictions.prepared(){self.evictions.birth_bytes()}else if !self.mesh_requests.prepared(){self.mesh_requests.birth_bytes()}else if !self.raster_requests.prepared(){self.raster_requests.birth_bytes()}else{0};
        semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{capacity_bytes:bytes,depth:1,..Default::default()},|grant|{
            if !self.uploads.prepared(){self.uploads.birth(grant)}else if !self.raster_producers.prepared(){self.raster_producers.birth(grant)}else if !self.evictions.prepared(){self.evictions.birth(grant)}else if !self.mesh_requests.prepared(){self.mesh_requests.birth(grant)}else if !self.raster_requests.prepared(){self.raster_requests.birth(grant)}else{Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))}
        })?;
        Ok(self.prepared())
    }

    pub fn ensure_mesh(&mut self, key: &str, version: u64, lease: Mesh3dLease) {
        if self.mesh_requests.iter().take(self.mesh_request_len).flatten().any(|candidate| candidate.0 == key && candidate.1 == version) {
            return;
        }
        let upload = PreparedRenderUpload::Mesh { key: key.to_string(), version, lease };
        if self.mesh_request_len >= WORLD3D_FRAME_RESOURCE_CAPACITY || self.upload_len >= WORLD3D_FRAME_RESOURCE_CAPACITY {
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

    /// 🧷️ Whether this frame's uploads already carry that exact mesh identity.
    ///
    /// The predicate [`retain_ensured_world_draws`] enforces the residency law with.
    pub fn has_mesh_request(&self, key: &str, version: u64) -> bool {
        self.mesh_requests.iter().take(self.mesh_request_len).flatten().any(|candidate| candidate.0 == key && candidate.1 == version)
    }

    /// 🖼️ Admits ONE decoded raster into this frame's world uploads, TAKING its pixels.
    ///
    /// 🩸️ It used to borrow them and `to_vec()` a fresh copy on every call, and `render_world_3d`
    /// calls it once per visible reference image per surface per FRAME — so a two-pane mode re-admitted
    /// the same megabytes twice a frame against the process-wide 32 MiB
    /// `PREPARED_RASTER_PRODUCER_BYTES` ledger. Eight outstanding producers exhausted it, the
    /// admission was refused, and the refusal surfaced as the frame fault `frame world resource
    /// admission exceeded fixed credits`, which kills the page: on the close of a dock window (a
    /// relayout burst) the whole wgpu shell tore down with it (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` step 17). Taking the
    /// pixels means the world state can hand them over exactly ONCE — the same one-shot semantics the
    /// chrome lane's `queue_decoded_raster_upload` has always had — and the producer, not the world,
    /// owns their bounded retirement from then on.
    pub fn ensure_world_plane_texture(&mut self, key: &str, lease: SceneRasterLease) {
        if self.raster_requests.iter().take(self.raster_request_len).flatten().any(|candidate| candidate == key) {
            return;
        }
        let upload = PreparedRenderUpload::SceneRaster { key: key.to_string(), lease };
        if self.raster_request_len >= WORLD3D_FRAME_RESOURCE_CAPACITY || self.upload_len >= WORLD3D_FRAME_RESOURCE_CAPACITY {
            if self.rejected.is_none() {
                self.rejected = Some(World3dBuildRejected::Upload(upload));
            }
            return;
        }
        self.raster_requests[self.raster_request_len] = Some(key.to_string());
        self.raster_request_len += 1;
        self.uploads[self.upload_len] = Some(upload);
        self.upload_len += 1;
    }

    pub fn evict_mesh(&mut self, key: &str) {
        let eviction = PreparedRenderEviction::Mesh { key: key.to_string() };
        if self.eviction_len >= WORLD3D_FRAME_RESOURCE_CAPACITY {
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
        match self.cursor_wake.as_ref().expect("original world authority remains owned").request() {
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

    /// 📤️ Moves ONE admitted world resource into the frame's prepared input per call.
    ///
    /// 🩸️ A raster producer must be BOUND to the input's `preview_generation` before it is pushed, on
    /// this lane exactly as on the chrome lane (`🧊️renderer/🦀️.rs`'s `RasterUploads` phase). An unbound
    /// producer carries `frame_generation: None`, and `PreparedRasterProducer::step` refuses it with
    /// `raster producer generation is stale` — a fault `AppFramePreparation` turned into an EMPTY
    /// `JobFault` and `ActiveFrameBuild` answered by cancelling its own token, with no fault recorded
    /// anywhere. The world reference underlay is the only producer this lane publishes, so the shell
    /// presented exactly ONE frame per boot: every build admitted after the plan image decoded died
    /// silently in preparation (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w7b-presenter-one-frame-per-boot.md` §1).
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path."))]
    pub fn append_step(&mut self, input: &mut ui_wgpu::wgpu::PreparedRenderInput) -> Result<bool, World3dBuildRejected> {
        if let Some(rejected) = self.rejected.take() {
            return Err(rejected);
        }
        if let Some(index) = self.raster_producer_len.checked_sub(1) {
            self.raster_producer_len = index;
            let Some(mut producer) = self.raster_producers[index].take() else { return Ok(false) };
            let Some(next) = input.raster_producers.len().checked_add(input.uploads.len()).and_then(|count| count.checked_add(1)) else {
                return Err(World3dBuildRejected::RasterProducer(producer));
            };
            if next > input.limits.max_upload_items {
                return Err(World3dBuildRejected::RasterProducer(producer));
            }
            if !producer.bind_frame_generation(input.preview_generation) {
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
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.upload_len == 0
            && self.raster_producer_len == 0
            && self.eviction_len == 0
            && self.mesh_request_len == 0
            && self.raster_request_len == 0
            && self.rejected.is_none()
            && self.cursor_wake_token.is_none()
            && self.cursor_wake_fault.is_none()
            && self.uploads.empty()
            && self.raster_producers.empty()
            && self.evictions.empty()
            && self.mesh_requests.empty()
            && self.raster_requests.empty()
            && self.cursor_wake.is_none()
            && self.cursor_wake_retirement.is_none()
    }

    pub fn next_close_demands(&self,copy:usize)->Result<semio_framework_value::RetainedCloneGrant,semio_framework_value::ValueError>{
        if self.cursor_wake_token.is_some()||self.cursor_wake_fault.is_some(){return Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()})}
        if let Some(owner)=self.rejected.as_ref(){return owner.next_close_demands(copy)}
        if let Some(owner)=self.raster_producer_len.checked_sub(1).and_then(|index|self.raster_producers[index].as_ref()){return owner.next_close_demands(copy)}
        if let Some(owner)=self.upload_len.checked_sub(1).and_then(|index|self.uploads[index].as_ref()){return owner.next_close_demands(copy)}
        if let Some(PreparedRenderEviction::Mesh{key})=self.eviction_len.checked_sub(1).and_then(|index|self.evictions[index].as_ref()){return Ok(world_close_string_demand(key))}
        if let Some((key,_))=self.mesh_request_len.checked_sub(1).and_then(|index|self.mesh_requests[index].as_ref()){return Ok(world_close_string_demand(key))}
        if let Some(key)=self.raster_request_len.checked_sub(1).and_then(|index|self.raster_requests[index].as_ref()){return Ok(world_close_string_demand(key))}
        if !self.uploads.empty(){return Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:self.uploads.release_bytes(),maximum_depth:1,..Default::default()})}
        if !self.raster_producers.empty(){return Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:self.raster_producers.release_bytes(),maximum_depth:1,..Default::default()})}
        if !self.evictions.empty(){return Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:self.evictions.release_bytes(),maximum_depth:1,..Default::default()})}
        if !self.mesh_requests.empty(){return Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:self.mesh_requests.release_bytes(),maximum_depth:1,..Default::default()})}
        if !self.raster_requests.empty(){return Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:self.raster_requests.release_bytes(),maximum_depth:1,..Default::default()})}
        if self.cursor_wake.is_some(){return Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()})}
        if let Some(owner)=self.cursor_wake_retirement.as_ref(){return Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:owner.next_copy_byte_demand()?,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy)?,maximum_release_bytes:owner.next_release_byte_demand()?,maximum_depth:owner.next_depth_demand()?})}
        Ok(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()})
    }

    /// 🧹️ Releases one untransferred resource through the same original caller and complete failure receipt.
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Close;
        if self.terminal_is_empty(){return Ok(Close::Complete{progress:Default::default()})}
        let grant=self.next_close_demands(cx.retained_grant().maximum_copy_bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        let demand=semio_framework_value::RetirementDemand{copy_bytes:grant.maximum_copy_bytes,capacity_bytes:grant.maximum_capacity_bytes,release_bytes:grant.maximum_release_bytes,depth:grant.maximum_depth};
        let mut blocked=false;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_granted(grant).map(|step|match step{Close::Pending{progress}=>semio_framework_value::RetainedCloneStep::Progress(progress),Close::Complete{progress}=>semio_framework_value::RetainedCloneStep::Complete(progress),Close::Blocked=>{blocked=true;semio_framework_value::RetainedCloneStep::Progress(Default::default())}}))?;
        if blocked{return Ok(Close::Blocked)}
        if self.terminal_is_empty(){Ok(Close::Complete{progress:step.progress()})}else{Ok(step)}
    }

    fn close_granted(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::InteractiveJobCloseStep as Close;
        if self.cursor_wake_token.take().is_some()||self.cursor_wake_fault.take().is_some(){return Ok(Close::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if let Some(owner)=self.rejected.as_mut(){let step=owner.close_step(grant)?;if owner.terminal_is_empty(){self.rejected=None}return Ok(match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}
        if let Some(index)=self.raster_producer_len.checked_sub(1){if let Some(owner)=self.raster_producers[index].as_mut(){owner.begin_close();let step=owner.close_step(grant)?;if owner.terminal_is_empty(){self.raster_producers[index]=None;self.raster_producer_len=index}return Ok(match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}self.raster_producer_len=index;}
        else if let Some(index)=self.upload_len.checked_sub(1){if let Some(owner)=self.uploads[index].as_mut(){let step=owner.close_step(grant)?;if owner.terminal_is_empty(){self.uploads[index]=None;self.upload_len=index}return Ok(match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}self.upload_len=index;}
        else if let Some(index)=self.eviction_len.checked_sub(1){if let Some(PreparedRenderEviction::Mesh{key})=self.evictions[index].as_mut(){let step=world_close_string(key,grant)?;if key.is_empty()&&key.capacity()==0{self.evictions[index]=None;self.eviction_len=index}return Ok(match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}self.eviction_len=index;}
        else if let Some(index)=self.mesh_request_len.checked_sub(1){if let Some((key,_))=self.mesh_requests[index].as_mut(){let step=world_close_string(key,grant)?;if key.is_empty()&&key.capacity()==0{self.mesh_requests[index]=None;self.mesh_request_len=index}return Ok(match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}self.mesh_request_len=index;}
        else if let Some(index)=self.raster_request_len.checked_sub(1){if let Some(key)=self.raster_requests[index].as_mut(){let step=world_close_string(key,grant)?;if key.is_empty()&&key.capacity()==0{self.raster_requests[index]=None;self.raster_request_len=index}return Ok(match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}self.raster_request_len=index;}
        if self.raster_producer_len==0&&self.upload_len==0&&self.eviction_len==0&&self.mesh_request_len==0&&self.raster_request_len==0{
            if !self.uploads.empty(){let bytes=self.uploads.release_bytes();if grant.maximum_release_bytes<bytes{return Ok(Close::Pending{progress:Default::default()})}let step=self.uploads.release(grant)?;return Ok(Close::Pending{progress:step.progress()})}
            if !self.raster_producers.empty(){let bytes=self.raster_producers.release_bytes();if grant.maximum_release_bytes<bytes{return Ok(Close::Pending{progress:Default::default()})}let step=self.raster_producers.release(grant)?;return Ok(Close::Pending{progress:step.progress()})}
            if !self.evictions.empty(){let bytes=self.evictions.release_bytes();if grant.maximum_release_bytes<bytes{return Ok(Close::Pending{progress:Default::default()})}let step=self.evictions.release(grant)?;return Ok(Close::Pending{progress:step.progress()})}
            if !self.mesh_requests.empty(){let bytes=self.mesh_requests.release_bytes();if grant.maximum_release_bytes<bytes{return Ok(Close::Pending{progress:Default::default()})}let step=self.mesh_requests.release(grant)?;return Ok(Close::Pending{progress:step.progress()})}
            if !self.raster_requests.empty(){let bytes=self.raster_requests.release_bytes();if grant.maximum_release_bytes<bytes{return Ok(Close::Pending{progress:Default::default()})}let step=self.raster_requests.release(grant)?;return Ok(Close::Pending{progress:step.progress()})}

        }
        if let Some(authority)=self.cursor_wake.take(){self.cursor_wake_retirement=Some(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(authority.0));return Ok(Close::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if let Some(owner)=self.cursor_wake_retirement.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.cursor_wake_retirement=None}return Ok(Close::Pending{progress:step.progress()})}
        Ok(Close::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}})
    }

}

impl Drop for World3dBuildContext{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original world resources dropped before paid close");if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.cursor_wake)}}}}
//#endregion 📦️PreparedWorldResources

use semio_framework::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
use serde::Deserialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
use std::sync::{LazyLock, Mutex};

//#region SceneRecords

fn dsl_id_to_string(value: &semio_framework::DslValue) -> Option<String> {
    value.as_str().map(str::to_string).or_else(|| value.as_f64().map(|n| if n.fract() == 0.0 { format!("{}", n as u64) } else { n.to_string() }))
}

fn dsl_string_vec(value: &semio_framework::DslValue) -> Vec<String> {
    value.as_array().map(|items| items.iter().filter_map(dsl_id_to_string).collect()).unwrap_or_default()
}

#[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[cfg(test)]
struct WorldMeshLodEntry {
    lod: f64,
    url: String,
}

#[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldLodRecord {
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    automatic: bool,
    #[serde(default = "default_manual_lod")]
    #[value(default = "default_manual_lod")]
    manual: f64,
    #[serde(default = "default_distance_reference")]
    #[value(default = "default_distance_reference")]
    distance_reference: f64,
    #[serde(default)]
    #[value(default)]
    depth_variable: bool,
    #[serde(default = "default_grid_factor")]
    #[value(default = "default_grid_factor")]
    grid_factor: f64,
    #[serde(default)]
    #[value(default)]
    grid_snap_enabled: bool,
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    show_grid: bool,
    #[serde(default)]
    #[value(default)]
    grid_datum: Option<[f64; 3]>,
}

impl Default for WorldLodRecord {
    fn default() -> Self {
        default_lod_record()
    }
}

#[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldSelectionTargets {
    #[serde(default)]
    #[value(default)]
    vertex: bool,
    #[serde(default)]
    #[value(default)]
    edge: bool,
    #[serde(default)]
    #[value(default)]
    face: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldVortexRecord {
    full_id: String,
    #[value(default)]
    position: Option<[f64; 3]>,
    #[value(default)]
    direction: Option<[f64; 3]>,
    #[value(default)]
    display_direction: Option<String>,
    #[value(default)]
    radius: Option<f64>,
    #[value(default)]
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldAttractionRecord {
    #[value(default)]
    id: Option<String>,
    #[value(default)]
    from: Option<[f64; 3]>,
    #[value(default)]
    to: Option<[f64; 3]>,
    #[value(default)]
    color: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldTutorialGeometry {
    pub point: [f32; 2],
    pub polyline: Option<Vec<[f32; 2]>>,
}

/// 👻️ Projects one tutorial scene point through the live world camera into surface-local pixels.
pub fn world3d_tutorial_scene_point(state: &World3dState, position: [f64; 3]) -> Option<[f32; 2]> {
    let camera = state.orbit.to_camera();
    let view_projection = ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.projection_spec, state.bounds.w.max(1.0), state.bounds.h.max(1.0));
    ui_wgpu::wgpu::projection_spec_project_point(view_projection, state.projection_spec, Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32), state.bounds.w.max(1.0), state.bounds.h.max(1.0))
}

/// 👻️ Resolves the world host's semantic pick vocabulary from its retained live scene geometry.
pub fn world3d_tutorial_entity_geometry(state: &World3dState, domain: &str, entity: &str) -> Option<WorldTutorialGeometry> {
    let center = [state.bounds.w * 0.5, state.bounds.h * 0.5];
    let nearest = |points: Vec<[f64; 3]>| {
        points
            .into_iter()
            .filter_map(|position| world3d_tutorial_scene_point(state, position).map(|point| (point, (point[0] - center[0]).hypot(point[1] - center[1]))))
            .min_by(|left, right| left.1.total_cmp(&right.1))
            .map(|(point, _)| WorldTutorialGeometry { point, polyline: None })
    };
    match domain {
        "vortex" => {
            if entity == "*" {
                return nearest(state.vortices.iter().filter_map(|vortex| vortex.position).collect());
            }
            let position = state.vortices.iter().find(|vortex| vortex.full_id == entity)?.position?;
            world3d_tutorial_scene_point(state, position).map(|point| WorldTutorialGeometry { point, polyline: None })
        }
        "object" => {
            if entity == "*" {
                return nearest(state.instance_positions.values().copied().collect());
            }
            let position = *state.instance_positions.get(entity)?;
            world3d_tutorial_scene_point(state, position).map(|point| WorldTutorialGeometry { point, polyline: None })
        }
        "attraction" => {
            let attraction = if entity == "*" { state.attractions.iter().find(|attraction| attraction.from.is_some() && attraction.to.is_some()) } else { state.attractions.iter().find(|attraction| attraction.id.as_deref() == Some(entity)) }?;
            let from = world3d_tutorial_scene_point(state, attraction.from?)?;
            let to = world3d_tutorial_scene_point(state, attraction.to?)?;
            Some(WorldTutorialGeometry { point: [(from[0] + to[0]) * 0.5, (from[1] + to[1]) * 0.5], polyline: Some(vec![from, to]) })
        }
        _ => None,
    }
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldTargetVolumeRecord {
    #[value(default)]
    origin: Option<[f64; 3]>,
    #[value(default)]
    orientation: Option<[f64; 4]>,
    #[value(default)]
    scale: Option<[f64; 3]>,
    #[value(default)]
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldReferenceRecord {
    #[value(default)]
    id: Option<String>,
    #[value(default)]
    url: Option<String>,
    #[value(default)]
    origin: Option<[f64; 3]>,
    #[value(default)]
    width_world: Option<f64>,
    #[value(default)]
    locked: Option<bool>,
    #[value(default)]
    opacity: Option<f64>,
    #[value(default)]
    hidden: Option<bool>,
}

/// 🖼️ React's authored fallback for a reference without `widthWorld`.
const WORLD_REFERENCE_DEFAULT_WIDTH: f32 = 10.0;

fn world_reference_width(reference: &WorldReferenceRecord) -> f32 {
    reference.width_world.map(|width| width as f32).unwrap_or(WORLD_REFERENCE_DEFAULT_WIDTH)
}

/// 🤝️ One item of the `engagementPreview` lane — the live construction feedback a plugin's
/// interaction statechart publishes for the state it is in (`cad`'s `preview_display_items`). Same
/// four wire kinds React's `World3dHost` parses (`WorldEngagementPreviewItem`, `🟦️.tsx:344-372`):
/// `point`, `segment`, `box-preview`, `linear-handle`.
///
/// 🩸️ The lane had NO wgpu reader at all before ticket 26/09/17 packet W2f, so every rubber-band
/// point, segment, footprint box and height handle of every plugin construction interaction was
/// invisible on this target while React drew them.
#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldEngagementPreviewRecord {
    #[value(default)]
    kind: Option<String>,
    #[value(default)]
    role: Option<String>,
    #[value(default)]
    position: Option<[f64; 3]>,
    #[value(default)]
    from: Option<[f64; 3]>,
    #[value(default)]
    to: Option<[f64; 3]>,
    #[value(default)]
    corner_a: Option<[f64; 3]>,
    #[value(default)]
    corner_b: Option<[f64; 3]>,
    #[value(default)]
    height: Option<f64>,
    #[value(default)]
    axis: Option<[f64; 3]>,
    #[value(default)]
    origin: Option<[f64; 3]>,
}

//#region 🧲️PickTargets
/// 🧲️ How many `pick_targets_json` rows one surface RETAINS. The lane is bounded on both ends —
/// the producer caps what it publishes (`CAD_PICK_OVERLAY_ITEM_BUDGET`) and this caps what a
/// consumer keeps — so a Concrete-Forest pane with thousands of kernel members can never grow an
/// unbounded `Vec` behind a pointer move.
pub const WORLD_PICK_TARGET_CAPACITY: usize = 256;

/// 🧲️ How many points ONE retained pick target keeps. A sampled arc/circle/ellipse edge is 32-64
/// samples on React's side (`edgeSamplePoints`), and the bounds this host hit-tests are unchanged
/// by dropping the tail of a longer run.
pub const WORLD_PICK_TARGET_POINT_CAPACITY: usize = 64;

/// 🧲️ React's `targetRayScore` box padding — `box.expandByScalar(kind === "vertex" ? 0.12 : 0.08)`
/// (`✏️editor/⚙️engine/📺️renderer/🟦️.tsx:2301`). World units, not pixels: the CAD editor's own
/// pick proxies are world-sized spheres/boxes, so this host uses the same numbers rather than a
/// screen-space radius that would disagree with React at every zoom.
const WORLD_PICK_TARGET_VERTEX_PAD: f32 = 0.12;
const WORLD_PICK_TARGET_PAD: f32 = 0.08;

/// 🧲️ React's `spatialPickPriority * 1e-4` tie-break (`🟦️.tsx:2288-2302`) — at equal ray distance
/// the FINEST kind wins (vertex 0 → edge 1 → face 2 → object 3), which is why a click on a corner
/// of a solid selects the vertex and not the solid.
const WORLD_PICK_TARGET_PRIORITY_EPSILON: f32 = 1e-4;

/// 🎨️ One `pick_targets_json` row's resolved highlight material — the wire half of the CAD picking
/// engine's `SpatialTargetStyle`. `color`/`emissive` are theme TOKEN ids (`--foreground`,
/// `--color-changed-selected`, …) or a `#rrggbb` literal, never a resolved colour: the token is
/// what survives an appearance switch.
#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldPickTargetStyleRecord {
    #[value(default)]
    color: Option<String>,
    #[value(default)]
    emissive: Option<String>,
    #[value(default)]
    opacity: Option<f64>,
    #[value(default)]
    line_width: Option<f64>,
}

/// 🧲️ One sub-object pick target — the wire half of the CAD picking engine's `SpatialPickTarget`
/// (`✏️editor/⚙️engine/🧲️picking/🦀️.rs:86`).
///
/// 🩸️ Before ticket 26/09/17 packet W14g this surface had NO pick-target lane: the overlay rode
/// `engagementPreview`, a paint-only lane, so sub-object picking was shown and never hit-tested and
/// every click still resolved to the whole instance through the domain's `object` granularity.
#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldPickTargetRecord {
    #[value(default)]
    kind: String,
    #[value(default)]
    id: String,
    #[value(default)]
    point: Option<[f64; 3]>,
    #[value(default)]
    points: Vec<[f64; 3]>,
    #[value(default)]
    typology: Option<String>,
    /// 👁️ `false` for a hidden or locked entity — it still PAINTS (dimmed, through `style`) and is
    /// never hit-tested, exactly as React's `SpatialPickGeometryLayer` renders such a target through
    /// `SpatialPickTargetNode` while `selectableTargets` leaves out its `SpatialPickHitTarget`.
    #[value(default)]
    selectable: Option<bool>,
    #[value(default)]
    style: Option<WorldPickTargetStyleRecord>,
}

impl WorldPickTargetRecord {
    /// 🪪️ `kind:id`, the key every hover/selection comparison is made on — the twin of the CAD
    /// engine's `spatial_pick_target_key`.
    fn key(&self) -> String {
        format!("{}:{}", self.kind, self.id)
    }

    /// 👁️ See [`WorldPickTargetRecord::selectable`].
    fn is_selectable(&self) -> bool {
        self.selectable.unwrap_or(true)
    }
}

/// 🧲️ React's `spatialPickPriority` (`📺️renderer/🟦️.tsx:2288`). An unknown kind sorts LAST so a
/// producer that grows a fifth kind degrades to "coarsest", never to "wins every tie".
fn world_pick_target_priority(kind: &str) -> u8 {
    match kind {
        "vertex" => 0,
        "edge" => 1,
        "face" => 2,
        "object" => 3,
        _ => 4,
    }
}

/// 🧲️ The padded world AABB React's `targetRayScore` raycasts — `points` when the row carries any,
/// otherwise the single `point`. `None` for a row that carries neither.
fn world_pick_target_bounds(target: &WorldPickTargetRecord) -> Option<([f32; 3], [f32; 3])> {
    let fallback = [target.point?];
    let points: &[[f64; 3]] = if target.points.is_empty() { &fallback } else { &target.points };
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    for point in points {
        for axis in 0..3 {
            min[axis] = min[axis].min(point[axis] as f32);
            max[axis] = max[axis].max(point[axis] as f32);
        }
    }
    let pad = if target.kind == "vertex" { WORLD_PICK_TARGET_VERTEX_PAD } else { WORLD_PICK_TARGET_PAD };
    for axis in 0..3 {
        min[axis] -= pad;
        max[axis] += pad;
    }
    Some((min, max))
}

/// 🧲️ React's `targetRayScore` — ray distance to the padded box plus the kind's tie-break weight.
/// Lower wins, and the same number is comparable with a mesh hit's own ray distance, which is what
/// lets a pick target and the instance under it compete in ONE ordering (r3f raycasts both in the
/// same scene).
fn world_pick_target_ray_score(target: &WorldPickTargetRecord, origin: Vec3, direction: Vec3) -> Option<f32> {
    let (min, max) = world_pick_target_bounds(target)?;
    let distance = ray_aabb_slab(origin, direction, min, max)?;
    Some(distance + f32::from(world_pick_target_priority(&target.kind)) * WORLD_PICK_TARGET_PRIORITY_EPSILON)
}

/// 🪪️ The CAD engine's `spatial_hover_key_aliases` as the framework sees it: a kernel member key
/// (`solid:`/`shell:`/`wire:`/`anchor:`) and the coarser pick key that represents it
/// (`object:`/`face:`/`edge:`/`vertex:`) name the SAME thing, so a guest that echoes one spelling
/// must still match a host that dispatched the other.
fn world_pick_key_alias(key: &str) -> Option<String> {
    let (kind, id) = key.split_once(':')?;
    let mapped = match kind {
        "vertex" | "anchor" => "vertex",
        "edge" | "wire" => "edge",
        "face" | "shell" => "face",
        "solid" => "object",
        "object" => "solid",
        _ => return None,
    };
    Some(format!("{mapped}:{id}"))
}

/// 🪪️ True when two hover/selection keys name the same pick target, aliases included.
fn world_pick_keys_match(left: Option<&str>, right: Option<&str>) -> bool {
    let (Some(left), Some(right)) = (left.filter(|key| !key.is_empty()), right.filter(|key| !key.is_empty())) else {
        return false;
    };
    left == right || world_pick_key_alias(left).as_deref() == Some(right) || world_pick_key_alias(right).as_deref() == Some(left)
}

/// 🚚️ Parses the lane into the bounded retained set — every cap applied here, never at paint time.
fn world_pick_targets_from_json(json: &str) -> Vec<WorldPickTargetRecord> {
    let mut targets: Vec<WorldPickTargetRecord> = serde_json::from_str(json).unwrap_or_default();
    targets.truncate(WORLD_PICK_TARGET_CAPACITY);
    for target in &mut targets {
        target.points.truncate(WORLD_PICK_TARGET_POINT_CAPACITY);
    }
    targets
}
//#endregion 🧲️PickTargets

#[derive(Clone, Debug)]
pub struct WorldCatalogueDropPreviewRecord {
    pub object_kind: String,
    pub mesh_url: Option<String>,
    pub origin: [f64; 3],
}

/// 🌉️ Hand-written, not derived: `scale: Option<serde_json::Value>` has no `ToValue`/`FromValue`
/// for `serde_json::Value` (only the `DslValue <-> serde_json::Value` `From` bridges in
/// `🌱️value/🦀️.rs` exist) — same reason as `semio_framework_graph_manifest::KindDef`.
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

impl semio_framework_value::ToValue for WorldBrushPreviewRecord {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries: Vec<(String, semio_framework_value::DslValue)> = vec![
            ("meshUrl".to_string(), semio_framework_value::ToValue::to_value(&self.mesh_url)),
            ("origin".to_string(), semio_framework_value::ToValue::to_value(&self.origin)),
            ("orientation".to_string(), semio_framework_value::ToValue::to_value(&self.orientation)),
            ("targetVortexFullId".to_string(), semio_framework_value::ToValue::to_value(&self.target_vortex_full_id)),
            ("objectKindId".to_string(), semio_framework_value::ToValue::to_value(&self.object_kind_id)),
            ("sourceVortexIndex".to_string(), semio_framework_value::ToValue::to_value(&self.source_vortex_index)),
            ("color".to_string(), semio_framework_value::ToValue::to_value(&self.color)),
        ];
        entries.push((
            "scale".to_string(),
            match &self.scale {
                Some(scale) => semio_framework_value::DslValue::from(scale),
                None => semio_framework_value::DslValue::Null,
            },
        ));
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for WorldBrushPreviewRecord {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for WorldBrushPreviewRecord, found {value:?}")));
        };
        let mut out = WorldBrushPreviewRecord::default();
        for (key, entry) in fields {
            match key.as_str() {
                "meshUrl" => out.mesh_url = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("meshUrl"))?,
                "origin" => out.origin = <Option<[f64; 3]> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("origin"))?,
                "orientation" => out.orientation = <Option<[f64; 4]> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("orientation"))?,
                "scale" => out.scale = if matches!(entry, semio_framework_value::DslValue::Null) { None } else { Some(serde_json::Value::from(&entry)) },
                "targetVortexFullId" => out.target_vortex_full_id = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("targetVortexFullId"))?,
                "objectKindId" => out.object_kind_id = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("objectKindId"))?,
                "sourceVortexIndex" => out.source_vortex_index = <Option<usize> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("sourceVortexIndex"))?,
                "color" => out.color = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("color"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

//#region Environment
/// ☀️ Directional sun light in the same horizontal coordinate system React uses.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldEnvironmentSunRecord {
    #[value(default)]
    enabled: Option<bool>,
    #[value(default)]
    azimuth: Option<f64>,
    #[value(default)]
    elevation: Option<f64>,
    #[value(default)]
    intensity: Option<f64>,
    #[value(default)]
    color: Option<String>,
}

/// 💡️ Ambient light carried into the scene pass and GPU globals.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldEnvironmentAmbientRecord {
    #[value(default)]
    intensity: Option<f64>,
    #[value(default)]
    color: Option<String>,
}

/// 🌑️ Directional shadow controls shared by World3d and IconRender.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldEnvironmentShadowRecord {
    #[value(default)]
    enabled: Option<bool>,
    #[value(default)]
    opacity: Option<f64>,
    #[value(default)]
    softness: Option<f64>,
}

/// 🎨️ Neutral-instance material override. Selection and hover keep their semantic base/emissive
/// rows while sharing the material's metalness and roughness, as React's `GlbInstanceMesh` does.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldEnvironmentMaterialRecord {
    #[value(default)]
    color: Option<String>,
    #[value(default)]
    metalness: Option<f64>,
    #[value(default)]
    roughness: Option<f64>,
    #[value(default)]
    emissive: Option<String>,
    #[value(default)]
    emissive_intensity: Option<f64>,
    #[value(default)]
    stroke: Option<String>,
}

/// 🌍️ `World3dScene.environmentJson` mirror — see `world-3d-host.tsx`'s `WorldEnvironmentRecord`.
/// Every visible light/material field is resolved into [`ScenePass3d`] before GPU preparation.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldEnvironmentRecord {
    #[value(default)]
    background: Option<String>,
    #[value(default)]
    ambient: Option<WorldEnvironmentAmbientRecord>,
    #[value(default)]
    sun: Option<WorldEnvironmentSunRecord>,
    #[value(default)]
    shadow: Option<WorldEnvironmentShadowRecord>,
    #[value(default)]
    material: Option<WorldEnvironmentMaterialRecord>,
}
//#endregion Environment

//#region TerrainStyle
/// 🌐️⛰️ `World3dScene.terrainJson` mirror — GIS-3D terrain style/source descriptor consumed by
/// `🗺️WorldTerrainLayer` in the React reference. `color_ramp`/`min_zoom`/`max_zoom` are parsed but
/// not branched on, mirroring the React reference (single hardcoded hypsometric ramp; zoom bounds
/// fixed inside `framework_surface_terrain::tiles`) — not a gap, a faithful match of upstream.
#[derive(Clone, Debug, Deserialize, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldTerrainStyle {
    tile_url_template: String,
    #[serde(default)]
    #[value(default)]
    project_origin_lon: f64,
    #[serde(default)]
    #[value(default)]
    project_origin_lat: f64,
    #[serde(default = "default_terrain_exaggeration")]
    #[value(default = "default_terrain_exaggeration")]
    exaggeration: f64,
    #[serde(default = "default_terrain_color_ramp")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_color_ramp")]
    color_ramp: String,
    #[serde(default = "default_terrain_min_zoom")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_min_zoom")]
    min_zoom: u32,
    #[serde(default = "default_terrain_max_zoom")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_max_zoom")]
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
/// 🎟️ Funds one original World mesh turn with independent ownership currencies.
fn world_mesh_ownership_grant(copy: usize, capacity: usize, release: usize, depth: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth }
}

/// 🧾️ Verifies the original owner receipt before World advances its publication or close phase.
fn world_mesh_ownership_complete(grant: semio_framework_value::retained_clone::RetainedCloneGrant, step: ui_wgpu::wgpu::Mesh3dOwnershipStep) -> Result<bool, ui_wgpu::wgpu::Mesh3dFault> {
    if !step.progress.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::Closing); }
    Ok(step.complete)
}

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
    Mesh(ui_wgpu::wgpu::Mesh3dFault),
    Ownership(semio_framework_value::ValueRefusalKind),
    Canceled,
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

pub use semio_framework_ui_contract::UiRevision as WorldNativeUiRevision;
pub use semio_framework_ui_runtime::UiNativeMeshResource as WorldNativeMeshResource;
pub use ui_wgpu::wgpu::kernel_3d_scene::Mesh3dResourceLoan as WorldNativeMeshLoan;

#[derive(Debug)]
enum WorldMeshSource {
    Owned{lease:Option<Mesh3dLease>,closing:bool},
    Native(WorldNativeMeshSource),
}

#[derive(Debug)]
struct WorldNativeMeshSource {
    loan: WorldNativeMeshLoan,
    receipt: semio_framework::kernel::ActorUiPatchReceipt,
    revision: WorldNativeUiRevision,
    resource_ordinal: usize,
    lookup_extent: usize,
    frozen: bool,
    gpu_returned: bool,
}

impl WorldMeshSource {
    fn lease(&self)->Option<&Mesh3dLease>{match self{Self::Owned{lease,..}=>lease.as_ref(),Self::Native(source) if !source.frozen=>source.loan.lease_ref(),Self::Native(_)=>None}}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ui_wgpu::wgpu::Mesh3dFault>{match self{
        Self::Owned{lease,closing}=>{let Some(value)=*lease else{return Ok((true,Default::default()))};if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}if !*closing{match mesh3d_begin_close(value){Ok(())|Err(ui_wgpu::wgpu::Mesh3dFault::Closing)=>*closing=true,Err(ui_wgpu::wgpu::Mesh3dFault::Stale)=>{lease.take();return Ok((true,RetainedCloneProgress{copied_items:1,..Default::default()}))},Err(fault)=>return Err(fault)}return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))}let step=mesh3d_close_step(value,grant)?;if step.complete{lease.take();}Ok((step.complete,step.progress))},
        Self::Native(source)=>{if !source.gpu_returned{return Ok((false,Default::default()))}let step=source.loan.return_step(grant)?;Ok((step.complete,step.progress))}
    }}
}

#[derive(Default)]
struct WorldMeshRegistry(WorldDynamicRegistry<WorldMeshSource,WORLD_DYNAMIC_MESH_CAPACITY>);

impl std::ops::Deref for WorldMeshRegistry{type Target=WorldDynamicRegistry<WorldMeshSource,WORLD_DYNAMIC_MESH_CAPACITY>;fn deref(&self)->&Self::Target{&self.0}}
impl std::ops::DerefMut for WorldMeshRegistry{fn deref_mut(&mut self)->&mut Self::Target{&mut self.0}}
impl WorldMeshRegistry {
    fn get(&self,id:&str)->Option<&Mesh3dLease>{self.0.get(id)?.lease()}
    #[cfg(test)]
    fn iter(&self)->impl Iterator<Item=(&String,&Mesh3dLease)>{self.0.slots.iter().flatten().filter_map(|entry|entry.value.lease().map(|lease|(&entry.id,lease)))}
    #[cfg(test)]
    fn insert(&mut self,id:String,value:Mesh3dLease)->Result<(WorldDynamicToken,Option<WorldDynamicEntry<WorldMeshSource>>),WorldDynamicRejected<Mesh3dLease>>{self.0.insert(id,WorldMeshSource::Owned{lease:Some(value),closing:false}).map_err(|rejected|WorldDynamicRejected{fault:rejected.fault,id:rejected.id,value})}
    fn commit_insert(&mut self,plan:WorldDynamicInsertPlan,id:String,value:Mesh3dLease)->Result<(WorldDynamicToken,Option<WorldDynamicEntry<WorldMeshSource>>),WorldDynamicRejected<Mesh3dLease>>{self.0.commit_insert(plan,id,WorldMeshSource::Owned{lease:Some(value),closing:false}).map_err(|rejected|WorldDynamicRejected{fault:rejected.fault,id:rejected.id,value})}
}

struct WorldNativeMeshAdmission {
    identity:ui_wgpu::wgpu::kernel_3d_scene::Mesh3dResourceIdentity,
    lease:Mesh3dLease,
    receipt:semio_framework::kernel::ActorUiPatchReceipt,
    revision:WorldNativeUiRevision,
    resource_ordinal:usize,
    key:String,
    extent:usize,
    interaction_id:WorldInteractionId,
    interaction_offset:usize,
    allocated:bool,
}

#[derive(Debug)]
pub enum WorldNativeMeshFailure { Binding, Registry(WorldDynamicFault), Mesh(ui_wgpu::wgpu::Mesh3dFault), Gpu(&'static str), Close(semio_framework_value::ValueError) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorldDynamicInsertPlan {
    slot: u16,
    epoch: u64,
    occupied: bool,
}

impl<T, const N: usize> Default for WorldDynamicRegistry<T, N> {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; N], len: 0, closing: false }
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

    #[cfg(test)]
    fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.slots.iter().flatten().map(|entry| (&entry.id, &entry.value))
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        usize::from(self.len)
    }

    #[cfg(test)]
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
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; WORLD_DYNAMIC_DRAW_CAPACITY], len: 0, closing: false }
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
            let Some(next) = bytes.checked_add(instance.id.len()).and_then(|bytes| bytes.checked_add(size_of::<Instance3d>())) else {
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
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldDrawDraft reached Drop before every admitted instance reached its terminal handback");
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
            drafts: semio_framework_async::boxed_fixed_slots(|| None),
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
        let bytes = mesh_key.len().checked_add(size_of::<SceneDraw3d>()).ok_or(WorldDynamicFault::ByteCapacity)?;
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

    fn admit_instance(&mut self, draw: u16, id: &str, model: Mat4, color: [f32; 4], color_source: SceneColorSource3d, selected: bool, hovered: bool, component_source: Option<ComponentSource3d>) -> Result<(), WorldDynamicFault> {
        if self.publishing || self.faulted || id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY { WorldDynamicFault::IdCapacity } else { WorldDynamicFault::Closing });
        }
        let Some(draft) = self.drafts.get_mut(usize::from(draw)).and_then(Option::as_mut) else {
            return Err(WorldDynamicFault::StaleToken);
        };
        let bytes = id.len().saturating_add(size_of::<Instance3d>());
        let next = usize::try_from(self.admitted_bytes).unwrap_or(usize::MAX).saturating_add(bytes);
        if next > usize::try_from(self.descriptor.byte_count).unwrap_or(0) || self.admitted_instances == self.descriptor.instance_count {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        if draft.admitted == draft.slots.len() {
            return Err(WorldDynamicFault::InstanceCapacity);
        }
        let instance = Instance3d { component_source, id: id.to_owned(), model, color, selected, hovered, material: SceneInstanceMaterial3d { color_source, ..Default::default() } };
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
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldDrawRebuildCursor reached Drop before every draft, instance, output, and staged draw reached terminal handback");
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
        assert!(self.is_empty() || std::thread::panicking(), "world draw registry reached Drop before its exact terminal-empty witness");
    }
}

/// 🖼️ Bounded identity and lease metadata; dropping it releases one token while the pool retains pixel storage.
#[derive(Debug)]
struct WorldSceneRaster {
    identity: SceneRasterIdentity,
    pending: Mutex<Option<SceneRasterLease>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct World3dPrimitiveMaterial {
    pub first_index: u32,
    pub index_count: u32,
    pub material: SceneAuthoredMaterial3d,
}

#[derive(Debug)]
pub struct World3dMeshAppearance {
    primitives: Vec<World3dPrimitiveMaterial>,
    textures: Vec<(String, WorldSceneRaster)>,
}

impl World3dMeshAppearance {
    pub fn new(primitives: Vec<World3dPrimitiveMaterial>, textures: Vec<(String, SceneRasterLease)>) -> Result<Self, (Vec<World3dPrimitiveMaterial>, Vec<(String, SceneRasterLease)>)> {
        if primitives.is_empty() || primitives.len() > WORLD_DYNAMIC_MESH_CAPACITY || textures.len() > WORLD_DYNAMIC_PIXEL_CAPACITY || primitives.iter().any(|primitive| primitive.index_count == 0) {
            return Err((primitives, textures));
        }
        if textures.iter().any(|(key, _)| key.is_empty() || key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY) {
            return Err((primitives, textures));
        }
        let textures = textures.into_iter().map(|(key, lease)| (key, WorldSceneRaster { identity: lease.identity(), pending: Mutex::new(Some(lease)) })).collect();
        Ok(Self { primitives, textures })
    }

    pub fn untextured(primitives: Vec<World3dPrimitiveMaterial>) -> Result<Self, Vec<World3dPrimitiveMaterial>> {
        match Self::new(primitives, Vec::new()) {
            Ok(appearance) => Ok(appearance),
            Err((primitives, _)) => Err(primitives),
        }
    }

    fn texture_upload(&self, key: &str, pool: &SceneRasterPool) -> Option<SceneRasterLease> {
        let (_, raster) = self.textures.iter().find(|(candidate, _)| candidate == key)?;
        if pool.gpu_resident(key, raster.identity) {
            raster.pending.lock().ok()?.take();
            return None;
        }
        raster.pending.lock().ok()?.take().or_else(|| pool.acquire(raster.identity).ok())
    }

    pub fn primitives(&self) -> &[World3dPrimitiveMaterial] {
        &self.primitives
    }

    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }

    /// 🧹️ Releases one primitive or pending texture owner per call.
    pub fn close_step(&mut self) -> bool {
        if self.primitives.pop().is_some() {
            return false;
        }
        if self.textures.pop().is_some() {
            return false;
        }
        true
    }

    /// 🧾️ Exact terminal witness for request-owned appearance retirement.
    pub fn terminal_is_empty(&self) -> bool {
        self.primitives.is_empty() && self.textures.is_empty()
    }
}

#[derive(Debug)]
pub struct World3dMeshAsset {
    pub mesh: Mesh3dLease,
    pub appearance: World3dMeshAppearance,
}

impl World3dMeshAsset {
    /// 🧹️ Retires appearance owners before advancing the paged mesh close ladder.
    pub fn close_step(&mut self) -> bool {
        if !self.appearance.close_step() {
            return false;
        }
        match mesh3d_begin_close(self.mesh) {
            Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => {}
            Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => return true,
            Err(_) => return false,
        }
        ui_wgpu::wgpu::mesh3d_close_demands(self.mesh, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_close_step(self.mesh, grant)?) }).is_ok_and(|complete| complete) || mesh3d_terminal_is_empty(self.mesh)
    }

    /// 🧾️ Exact terminal witness for cancellation before World publication.
    pub fn terminal_is_empty(&self) -> bool {
        self.appearance.terminal_is_empty() && mesh3d_terminal_is_empty(self.mesh)
    }
}

#[derive(Debug)]
pub struct World3dMeshAssetRejected {
    pub fault: WorldDynamicFault,
    pub id: String,
    pub value: World3dMeshAsset,
}

enum WorldOpaqueOwner {
    Draw(WorldDynamicEntry<SceneDraw3d>),
}

struct WorldOpaqueQuarantine<const N: usize> {
    slots: Box<[Option<WorldOpaqueOwner>; N]>,
    len: u16,
    saturated: u64,
}

impl<const N: usize> Default for WorldOpaqueQuarantine<N> {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), len: 0, saturated: 0 }
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
static WORLD_SCENE_RASTER_POOL: LazyLock<SceneRasterPool> = LazyLock::new(SceneRasterPool::new);

pub fn world_scene_raster_pool() -> SceneRasterPool {
    WORLD_SCENE_RASTER_POOL.clone()
}

fn quarantine_world_owner(owner: WorldOpaqueOwner) -> Result<WorldDynamicToken, WorldOpaqueOwner> {
    let Ok(mut quarantine) = WORLD_OPAQUE_QUARANTINE.lock() else {
        return Err(owner);
    };
    quarantine.admit(owner)
}

pub fn world3d_opaque_quarantine_status() -> (usize, u64) {
    WORLD_OPAQUE_QUARANTINE.lock().map_or((WORLD_OPAQUE_QUARANTINE_CAPACITY, u64::MAX), |quarantine| (usize::from(quarantine.len), quarantine.saturated))
}

/// 🌡️ What became of the scene's scalar field on the mesh it names, so the legend is shown only for a heatmap that exists
/// (`applied`) and a mismatch is announced instead of a wrong colouring — the React host's `data-world-scalar-field` status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum World3dScalarFieldStatus {
    #[default]
    None,
    Applied,
    Mismatch,
    Missing,
}

/// 🖍️ The scene's sub-element highlight tokens resolved through the live theme; an absent entry keeps the engine's built-in palette.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct WorldSubElementPaint {
    face_select: Option<[f32; 4]>,
    face_hover: Option<[f32; 4]>,
    edge_select: Option<[f32; 4]>,
    edge_hover: Option<[f32; 4]>,
    vertex_select: Option<[f32; 4]>,
    vertex_hover: Option<[f32; 4]>,
    vertex_mark_px: Option<f32>,
}

pub struct World3dState {
    pub surface_id: String,
    pub controller_id: String,
    pub bounds: Rect,
    pub pick_bounds: Rect,
    pub orbit: OrbitController,
    meshes: WorldMeshRegistry,
    mesh_versions: WorldDynamicRegistry<u64, WORLD_DYNAMIC_MESH_CAPACITY>,
    mesh_appearances: WorldDynamicRegistry<World3dMeshAppearance, WORLD_DYNAMIC_MESH_CAPACITY>,
    draws: WorldDrawRegistry,
    pub selection_method: String,
    pub local_hover_id: Option<String>,
    local_hover_granularity_id: Option<String>,
    pub pending_glb_urls: HashSet<String>,
    pub marquee_points: Vec<[f32; 2]>,
    pub marquee_active: bool,
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
    engagement_preview: Vec<WorldEngagementPreviewRecord>,
    /// 🧲️ The bounded sub-object pick targets of `pick_targets_json`, retained so a pointer move or
    /// a click can hit-test them without re-parsing the lane.
    pick_targets: Vec<WorldPickTargetRecord>,
    /// 🧲️ The `kind:id` of the pick target this host last published an `interactionHover` for —
    /// the sub-object twin of `local_hover_id`, deduped through [`world_pick_keys_match`] so an
    /// alias spelling never re-publishes the same hover.
    pick_hover_key: Option<String>,
    brush_preview: Option<WorldBrushPreviewRecord>,
    catalogue_drop_preview: Option<WorldCatalogueDropPreviewRecord>,
    active_utility: String,
    hovered_vortex_id: Option<String>,
    #[cfg(test)]
    drag_object_id: Option<String>,
    #[cfg(test)]
    drag_object_z: f32,
    #[cfg(test)]
    drag_last_position: Option<[f32; 3]>,
    /// 🚚️ The live relocate drag, or `None`. Armed by a primary press under the `worldRelocate`
    /// utility, previewed as a catalogue-drop ghost on every move, and closed by the release (commit)
    /// or by Escape (abort) — React's `relocateSessionRef` (`🌐️World3dHost/🟦️.tsx:6845`).
    relocate: Option<World3dRelocateSession>,
    selected_ids: Vec<String>,
    /// 🎯️ The Guest Scene's exact source-bound component interaction targets.
    gumball_selection_ids: Vec<String>,
    /// 📇️ The action ids the WINDOW KIND hosting this surface declares — the app manifest's
    /// `window_kind_action_refs`, republished by the shell on every engine-surface sync
    /// ([`set_world3d_declared_actions`]). A surface may only mint a verb that appears here.
    ///
    /// 🩸️ Without it the gumball was an unconditional affordance of every World3d surface, so the
    /// procedural VIEWER — whose `procedural-view-preview` kind declares only `exportDocument` and
    /// the camera verbs — offered a translate gizmo and published `translateSelection` into a window
    /// that refuses it, measured verbatim on 6118 as `handle_action promise failed: window kind
    /// procedural-view-preview does not own action translateSelection`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.2).
    /// Empty means "no window kind has declared anything for this surface yet", which offers nothing.
    declared_action_ids: Vec<String>,
    transform_mode: String,
    /// 🎛️ `selection_json.gumballConfig` when the producer authored one — see [`world3d_gumball_config`].
    gumball_config: Option<World3dSceneGumballConfigRecord>,
    /// ⚡️ `selection_json.gumballLiveDispatch`: the guest owns the mid-drag pose — a gesture streams its ticks and the host
    /// draws the guest's answer instead of a local preview (`🌐️World3dHost` `worldGumballStep`, live branch).
    gumball_live_dispatch: bool,
    #[cfg(test)]
    gumball_handle: Option<GumballHandle>,
    #[cfg(test)]
    gumball_pivot: Vec3,
    #[cfg(test)]
    gumball_drag_anchor: f32,
    #[cfg(test)]
    gumball_drag_start_vec: Vec3,
    #[cfg(test)]
    gumball_preview_translate: Vec3,
    #[cfg(test)]
    gumball_preview_angle: f32,
    #[cfg(test)]
    gumball_preview_scale: Vec3,
    pending_image_urls: HashSet<String>,
    reference_pixels: WorldDynamicRegistry<WorldSceneRaster, WORLD_DYNAMIC_PIXEL_CAPACITY>,
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
    #[cfg(test)]
    press_object_id: Option<String>,
    mesh_paint_textures: WorldDynamicRegistry<WorldSceneRaster, WORLD_DYNAMIC_PIXEL_CAPACITY>,
    lod: WorldLodRecord,
    presentation: World3dPresentation,
    chunking: Option<WorldChunkingRecord>,
    visible_chunks: HashSet<(i64, i64, i64)>,
    #[cfg(test)]
    mesh_lod_catalog: HashMap<String, Vec<WorldMeshLodEntry>>,
    #[cfg(test)]
    mesh_url_fallback: HashMap<String, String>,
    instance_positions: HashMap<String, [f64; 3]>,
    /// 🎯️ Rendered instance id → the TOPOLOGY target it stands for, from the scene's own
    /// `instances[].interactionId`. Only the entries that actually differ are kept, so a lookup miss
    /// means "this instance is its own target" — see [`instance_interaction_id`].
    instance_interaction_ids: HashMap<String, String>,
    instance_interaction_granularity_ids: HashMap<String, String>,
    mesh_pool: RefCountPool<String>,
    mesh_source_urls: HashMap<String, String>,
    /// 🥽️ Mesh url → the [`world_brush_mesh_revision`] THIS surface last announced it to the guest
    /// at. The wgpu twin of the `revision` prop React hands each mounted `BrushMeshRegistrar`: a
    /// surface announces an identity once per revision, and a surface that is rebuilt (window
    /// reopen, role switch) starts empty and announces again — see
    /// [`step_world3d_brush_mesh_announce`].
    brush_mesh_announced: HashMap<String, u64>,
    /// 🥽️ The ONE `registerBrushMesh` page run this surface has in flight. One page leaves per
    /// frame, so a user action never queues behind more than a single page — React's
    /// `drainPuzzle3dBrushMeshQueue` back pressure, expressed as a frame cursor.
    brush_mesh_run: Option<WorldBrushMeshRun>,
    brush_mesh_feedback_digest: Option<u64>,
    resolved_lod_pick: Option<f64>,
    environment: WorldEnvironmentRecord,
    terrain_style: Option<WorldTerrainStyle>,
    terrain_applied_signature: Option<(String, f64, f64, f64)>,
    terrain_session: TerrainSessionCore,
    terrain_visible_tiles: HashSet<(u32, u32, u32)>,
    terrain_built_tiles: HashSet<(u32, u32, u32)>,
    pending_terrain_tile_urls: HashMap<String, (u32, u32, u32)>,
    terrain_tile_misses: HashSet<(u32, u32, u32)>,
    asset_url_misses: HashSet<String>,
    #[cfg(test)]
    right_press_point: Option<[f32; 2]>,
    gizmo_hovered_tip: Option<usize>,
    interaction_revision: u64,
    interaction_authority: Option<WorldInteractionAuthority>,
    interaction_meshes: WorldInteractionMeshRegistry,
    interaction_objects: WorldInteractionObjectRegistry,
    snapshot_lease: Option<World3dSnapshotLease>,
    snapshot_apply: Option<World3dSnapshotApplyCursor>,
    snapshot_fault: Option<World3dSnapshotFault>,
    /// 🌉️ The mesh-wire → snapshot bridge (`//#region 🌉️World3dSceneBridge`) that gives a producer
    /// publishing only plain-JSON `meshes_json`/`instances_json` the same typed snapshot a
    /// snapshot-native producer hands over.
    scene_bridge: Option<World3dSceneBridgeCursor>,
    scene_bridge_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<World3dSceneBridgeCursor>>,
    scene_bridge_lease: Option<World3dSnapshotLease>,
    scene_bridge_retired: Option<(World3dSnapshotLease, bool)>,
    scene_bridge_generation: u64,
    scene_bridge_digest: Option<u64>,
    scene_camera_digest: Option<u64>,
    /// 🎯️ React's retained `WorldAutoFit` request, ownership key, applied key, and bounded live-geometry
    /// measurement. A missing published extent stays owed until rendered mesh bounds exist.
    fit_request: Option<World3dCameraFitRequest>,
    fit_owner: Option<World3dCameraFitOwner>,
    fit_applied_key: Option<u64>,
    fit_bounds_cursor: Option<World3dCameraBoundsCursor>,
    /// 📷️ The content extent consumed by this pane's last explicit projection-frame debt.
    projection_frame_key: Option<u64>,
    /// 📷️ The zoom produced by that accepted one-shot frame, retained for diagnostics.
    projection_frame_zoom: Option<f32>,
    /// 📐️ The accepted renderer-neutral projection mode × orientation for this pane.
    projection_spec: Viewport3dProjectionSpec,
    /// 📐️ This pane holds its OWN projection spec — React's `externalPendingProjectionSpec`, which
    /// the delivered `cameraState.projectionSpec` never overrides while it is set.
    projection_selected: bool,
    /// 📐️ A local selection owes exactly ONE content framing, and it is immune to
    /// `camera_user_moved`: the press is not a gesture the latch exists to protect.
    projection_frame_owed: bool,
    /// 📷️ The delivered camera decides whether scene content or its authored pose owns framing.
    projection_frame_policy: Viewport3dProjectionFramePolicy,
    /// 🔒️ The user has moved this camera since the live fit revision arrived — the latch that keeps a
    /// re-evaluation of the SAME document from yanking the view back
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
    camera_user_moved: bool,
    /// 🧭️ This pane's camera-report debt — see [`WorldCameraSync`].
    camera_sync: WorldCameraSync,
    scene_selection_digest: Option<u64>,
    scene_document_lanes_digest: Option<u64>,
    scene_mesh_digests: HashMap<String, u64>,
    prepared_status: [Option<World3dPreparedStatus>; 2],
    dynamic_blocked_owner: Option<WorldOpaqueOwner>,
    dynamic_mesh_close: Option<WorldDynamicEntry<WorldMeshSource>>,
    dynamic_blocked_mesh: Option<WorldDynamicEntry<WorldMeshSource>>,
    native_mesh_admission: Option<WorldNativeMeshAdmission>,
    placeholder_generation: u64,
    placeholder_build: Option<WorldPlaceholderMeshCursor>,
    terrain_build: Option<WorldTerrainMeshCursor>,
    terrain_pending: Option<((u32,u32,u32),TerrainTileMeshPayload,u64,u64,u64)>,
    dynamic_fault: Option<WorldDynamicFault>,
    terrain_revision: u64,
    face_overlay_build: Option<WorldFaceOverlayMeshCursor>,
    face_overlay_generation: Option<u64>,
    face_overlay_retired_generation: Option<u64>,
    face_overlay_colors: [Option<[f32; 4]>; 3],
    modelling_locale: &'static str,
    scalar_field_status: World3dScalarFieldStatus,
    sub_element_paint: WorldSubElementPaint,
    face_overlay_applied_revision: u64,
    face_overlay_applied_draw_generation: u64,
    draw_generation: u64,
    geometry_generation: u64,
    draw_rebuild: Option<WorldDrawRebuildCursor>,
    retired_draws: Option<WorldDrawRegistry>,
    asset_generation: u64,
    asset_io: WorldAssetIoAuthority,
    dynamic_retirement: Option<World3dDynamicRetirement>,
    /// ⏯️ The resident tool run trace fed by `World3dScene.tool_run_trace` — see [`tool_run_trace`].
    pub tool_run_trace: tool_run_trace::ToolRunTraceLayer,
    /// 👁️ The trace legend toggles this window's config exposes.
    pub tool_run_trace_visibility: tool_run_trace::ToolRunTraceVisibility,
    tool_run_trace_meshes_digest: Option<u64>,
    tool_run_trace_mesh_keys: Vec<String>,
    /// 🟩️ Instance ids a running tool placed provisionally; painted with the `provisional` style token.
    /// Filled from the `provisional: true` flag the scene producer stamps on `instances_json` records.
    pub provisional_instance_ids: HashSet<String>,
    /// 🎨️ The instances whose wire rows carry `highlighted`/`disabled`/`celebrating` — the three
    /// `MESH_STYLE_PAINT` rows that had a table entry and NO producer before ticket 26/09/17 packet
    /// W14g (`📓️w9b-projection-pane-framing-grid-materials.md` §7.3).
    pub highlighted_instance_ids: HashSet<String>,
    pub disabled_instance_ids: HashSet<String>,
    pub celebrating_instance_ids: HashSet<String>,
    /// 🪟️ The window instance whose view state echoes this layer's trace cursor as
    /// `toolRunTraceCursorByWindowId[window]`; set by the host that paints the surface into that window.
    pub tool_run_trace_window_id: Option<String>,
}

impl World3dState {
    /// 🩺️ What this surface's mesh ingest actually holds, for the one `[TRACE] ` line the browser
    /// Worker can emit. `draws=0` on a surface whose `meshes_json` carries a solid is ambiguous
    /// between "the bridge never ran", "the bridge ran and produced nothing" and "the draws exist
    /// but no frame has painted them since" — this separates the three
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn ingest_census(&self) -> String {
        format!(
            "state-draws={} state-instances={} state-meshes={} bridge={} bridge-lease={} apply={} apply-page={}/{} apply-item={} apply-faulted={} rebuild={} retired-draws={} retirement={} blocked={} snapshot-lease={} fault={:?}",
            self.draws.len,
            self.draws.iter().map(|draw| draw.instances.len()).sum::<usize>(),
            self.meshes.len,
            self.scene_bridge.is_some(),
            self.scene_bridge_lease.is_some(),
            self.snapshot_apply.is_some(),
            self.snapshot_apply.as_ref().map_or(0, |cursor| cursor.page),
            self.snapshot_apply.as_ref().map_or(0, |cursor| cursor.lease.page_count),
            self.snapshot_apply.as_ref().map_or(0, |cursor| cursor.item),
            self.snapshot_apply.as_ref().is_some_and(|cursor| cursor.faulted),
            self.draw_rebuild.is_some(),
            self.retired_draws.is_some(),
            self.dynamic_retirement.is_some(),
            self.dynamic_blocked_owner.is_some(),
            self.snapshot_lease.is_some(),
            self.snapshot_fault,
        )
    }

    /// 🩺️🧊️ One row per RESIDENT mesh — its key, its published vertex/index counts and its local
    /// bounds. `state-meshes=5` is ambiguous between "five real meshes" and "five empty leases", and a
    /// draw that submits an empty GPU mesh paints exactly nothing while every count above it reads
    /// healthy (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3d-world3d-glb-url-lane.md`).
    pub fn mesh_geometry_census(&self) -> String {
        let mut rows = String::from("meshes=[");
        for (index, key) in self.meshes.keys().enumerate() {
            let Some(mesh) = self.meshes.get(key) else { continue };
            let schema = mesh.schema().ok();
            let bounds = mesh.aabb().ok();
            if index > 0 {
                rows.push(',');
            }
            rows.push_str(&format!(
                "{key}:v{}/i{}{}",
                schema.map_or(0, |schema| schema.vertices),
                schema.map_or(0, |schema| schema.indices),
                bounds.map_or_else(String::new, |(min, max)| format!("@[{:.2},{:.2},{:.2}]..[{:.2},{:.2},{:.2}]", min[0], min[1], min[2], max[0], max[1], max[2]))
            ));
        }
        rows.push(']');
        rows
    }

    /// 🕹️ What this surface's INTERACTION authority actually holds, for the one `[TRACE] ` line the
    /// browser Worker can emit.
    ///
    /// ⚖️ "hover, click and wheel changed nothing" is ambiguous between five different stops — no
    /// intent ever arrived, the intent sat behind a generation the pump never asked for, the ray
    /// found no triangle, the plan was built but never published, or the action was published and
    /// the guest never took it. `ingest_census` separates the MESH half; this separates the INPUT
    /// half (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎥️ The surface's OWN camera, not the guest's `cameraJson`. A wheel or orbit turns the local
    /// rig first (`publish_world3d_plan_step`'s `publish_with`) and only then asks the guest for the
    /// same pose, so the local rig is the honest witness that the gesture was understood — the
    /// scene's `cameraJson` cannot move until the guest answers.
    pub fn interaction_census(&self) -> String {
        let Some(authority) = self.interaction_authority.as_ref() else {
            return "authority=none".into();
        };
        let front = authority.queue.front().map_or_else(
            || "front=none".to_string(),
            |intent| {
                format!(
                    "front={:?}/g{} button={} down={} at={},{} d={},{} wheel={} mods={}{}{}{}",
                    intent.phase,
                    intent.generation,
                    intent.button,
                    intent.down,
                    intent.x,
                    intent.y,
                    intent.dx,
                    intent.dy,
                    intent.delta,
                    if intent.shift { "s" } else { "-" },
                    if intent.ctrl { "c" } else { "-" },
                    if intent.alt { "a" } else { "-" },
                    if intent.meta { "m" } else { "-" },
                )
            },
        );
        let active = authority.active.as_ref().map_or_else(|| "none".to_string(), world_interaction_active_census);
        let eye = self.orbit.to_camera();
        format!(
            "camera=[{:.3},{:.3},{:.3}]->[{:.3},{:.3},{:.3}]/{:.1}deg {front} queued={} next-g={} active={active} blocked={} registry={} marquee={} gumball={} right-press={} faulted={} closing={} sync-owed={} sync-navigating={} sync-hover-owed={} revision={} objects-revision={} objects={} draws={} utility={} granularity={} mode={} bounds={}x{}+{},{} pick={}x{}+{},{} hover={:?} selected={}",
            eye.position.x,
            eye.position.y,
            eye.position.z,
            eye.target.x,
            eye.target.y,
            eye.target.z,
            eye.fov_y.to_degrees(),
            authority.queue.len,
            authority.next_generation,
            authority.blocked.is_some(),
            authority.registry.is_some(),
            authority.marquee.is_some(),
            authority.gumball.is_some(),
            authority.right_press.is_some(),
            authority.faulted,
            authority.closing,
            self.camera_sync.owed,
            self.camera_sync.navigating,
            self.camera_sync.hover_owed,
            self.interaction_revision,
            self.interaction_objects.revision,
            self.interaction_objects.instance_len,
            self.draws.len,
            self.active_utility,
            self.granularity,
            self.interaction_mode,
            self.bounds.w,
            self.bounds.h,
            self.bounds.x,
            self.bounds.y,
            self.pick_bounds.w,
            self.pick_bounds.h,
            self.pick_bounds.x,
            self.pick_bounds.y,
            self.local_hover_id,
            self.selected_ids.len(),
        )
    }

    pub fn new(surface_id: String, controller_id: String) -> Self {
        Self {
            surface_id,
            controller_id,
            bounds: Rect::default(),
            pick_bounds: Rect::default(),
            orbit: OrbitController::default(),
            meshes: WorldMeshRegistry::default(),
            mesh_versions: WorldDynamicRegistry::default(),
            mesh_appearances: WorldDynamicRegistry::default(),
            draws: WorldDrawRegistry::default(),
            selection_method: "rectangle".into(),
            local_hover_id: None,
            local_hover_granularity_id: None,
            pending_glb_urls: HashSet::new(),
            marquee_points: Vec::new(),
            marquee_active: false,
            bound_domain_id: None,
            bound_domain_granularity_id: None,
            vortices: Vec::new(),
            attractions: Vec::new(),
            target_volumes: Vec::new(),
            references: Vec::new(),
            engagement_preview: Vec::new(),
            pick_targets: Vec::new(),
            pick_hover_key: None,
            brush_preview: None,
            catalogue_drop_preview: None,
            active_utility: "select".into(),
            hovered_vortex_id: None,
            #[cfg(test)]
            drag_object_id: None,
            #[cfg(test)]
            drag_object_z: 0.0,
            #[cfg(test)]
            drag_last_position: None,
            relocate: None,
            selected_ids: Vec::new(),
            gumball_selection_ids: Vec::new(),
            declared_action_ids: Vec::new(),
            transform_mode: "translate".into(),
            gumball_config: None,
            gumball_live_dispatch: false,
            #[cfg(test)]
            gumball_handle: None,
            #[cfg(test)]
            gumball_pivot: Vec3::ZERO,
            #[cfg(test)]
            gumball_drag_anchor: 0.0,
            #[cfg(test)]
            gumball_drag_start_vec: Vec3::ZERO,
            #[cfg(test)]
            gumball_preview_translate: Vec3::ZERO,
            #[cfg(test)]
            gumball_preview_angle: 0.0,
            #[cfg(test)]
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
            #[cfg(test)]
            press_object_id: None,
            mesh_paint_textures: WorldDynamicRegistry::default(),
            lod: WorldLodRecord {
                automatic: true,
                manual: default_manual_lod(),
                distance_reference: default_distance_reference(),
                depth_variable: false,
                grid_factor: default_grid_factor(),
                grid_snap_enabled: false,
                show_grid: true,
                grid_datum: Some([0.0, 0.0, 0.0]),
            },
            presentation: World3dPresentation::default(),
            chunking: None,
            visible_chunks: HashSet::new(),
            #[cfg(test)]
            mesh_lod_catalog: HashMap::new(),
            #[cfg(test)]
            mesh_url_fallback: HashMap::new(),
            instance_positions: HashMap::new(),
            instance_interaction_ids: HashMap::new(),
            instance_interaction_granularity_ids: HashMap::new(),
            mesh_pool: RefCountPool::new(),
            mesh_source_urls: HashMap::new(),
            brush_mesh_announced: HashMap::new(),
            brush_mesh_run: None,
            brush_mesh_feedback_digest: None,
            resolved_lod_pick: None,
            environment: WorldEnvironmentRecord::default(),
            terrain_style: None,
            terrain_applied_signature: None,
            terrain_session: TerrainSessionCore::default(),
            terrain_visible_tiles: HashSet::new(),
            terrain_built_tiles: HashSet::new(),
            pending_terrain_tile_urls: HashMap::new(),
            terrain_tile_misses: HashSet::new(),
            asset_url_misses: HashSet::new(),
            #[cfg(test)]
            right_press_point: None,
            gizmo_hovered_tip: None,
            interaction_revision: 0,
            interaction_authority: Some(WorldInteractionAuthority::default()),
            interaction_meshes: WorldInteractionMeshRegistry::default(),
            interaction_objects: WorldInteractionObjectRegistry::default(),
            snapshot_lease: None,
            scene_bridge: None,
            scene_bridge_close: None,
            scene_bridge_lease: None,
            scene_bridge_retired: None,
            scene_bridge_generation: 0,
            scene_bridge_digest: None,
            scene_camera_digest: None,
            fit_request: None,
            fit_owner: None,
            fit_applied_key: None,
            fit_bounds_cursor: None,
            projection_frame_key: None,
            projection_frame_zoom: None,
            projection_spec: ui_wgpu::wgpu::default_projection_spec(),
            projection_selected: false,
            projection_frame_owed: false,
            projection_frame_policy: Viewport3dProjectionFramePolicy::Content,
            camera_user_moved: false,
            camera_sync: WorldCameraSync::default(),
            scene_selection_digest: None,
            scene_document_lanes_digest: None,
            scene_mesh_digests: HashMap::new(),
            snapshot_apply: None,
            snapshot_fault: None,
            prepared_status: [None; 2],
            dynamic_blocked_owner: None,
            dynamic_mesh_close: None,
            native_mesh_admission: None,
            dynamic_blocked_mesh: None,
            placeholder_generation: 0,
            placeholder_build: None,
            terrain_build: None,
            terrain_pending: None,
            dynamic_fault: None,
            terrain_revision: 0,
            face_overlay_build: None,
            face_overlay_generation: None,
            face_overlay_retired_generation: None,
            face_overlay_colors: [None; 3],
            modelling_locale: "en",
            scalar_field_status: World3dScalarFieldStatus::None,
            sub_element_paint: WorldSubElementPaint::default(),
            face_overlay_applied_revision: u64::MAX,
            face_overlay_applied_draw_generation: u64::MAX,
            draw_generation: 0,
            geometry_generation: 0,
            draw_rebuild: None,
            retired_draws: None,
            asset_generation: 0,
            asset_io: WorldAssetIoAuthority::default(),
            dynamic_retirement: None,
            tool_run_trace: tool_run_trace::ToolRunTraceLayer::default(),
            tool_run_trace_visibility: tool_run_trace::ToolRunTraceVisibility::default(),
            tool_run_trace_meshes_digest: None,
            tool_run_trace_mesh_keys: Vec::new(),
            provisional_instance_ids: HashSet::new(),
            highlighted_instance_ids: HashSet::new(),
            disabled_instance_ids: HashSet::new(),
            celebrating_instance_ids: HashSet::new(),
            tool_run_trace_window_id: None,
        }
    }

    /// 📤️ Creates an isolated render owner at the revision of the decoded asset handed to it.
    pub fn request_owned(surface_id: String, controller_id: String, revision: u64) -> Self {
        let mut state = Self::new(surface_id, controller_id);
        state.interaction_revision = revision;
        state
    }

    pub fn dynamic_retirement_is_idle(&self) -> bool {
        self.dynamic_retirement.is_none()
    }

    /// 🌍️ Selects the language the modelling overlays (annotations, legend, their accessible text) are written in: German, else English.
    pub fn set_modelling_locale(&mut self, german: bool) {
        self.modelling_locale = if german { "de" } else { "en" };
    }

    /// 🌡️ Whether the scene's scalar field painted the mesh it names.
    pub fn scalar_field_status(&self) -> World3dScalarFieldStatus {
        self.scalar_field_status
    }

    /// 🚨️ The last snapshot fault this surface recorded, for a host that reports why a world window
    /// went dark instead of guessing. `None` once a snapshot published, and `None` while a staged
    /// scene bridge is still building.
    pub fn snapshot_fault(&self) -> Option<World3dSnapshotFault> {
        self.snapshot_fault
    }

    /// 🔺️ The published geometry one draw's `mesh_key` resolves to — the lease `render_world_3d`
    /// itself hands to `World3dBuildContext::ensure_mesh`, so a host can read back the triangle and
    /// vertex counts it is actually drawing.
    pub fn mesh_lease(&self, mesh_key: &str) -> Option<Mesh3dLease> {
        self.meshes.get(mesh_key).copied()
    }
}

/// 📇️ Republishes the action ids the window kind hosting this surface declares. Idempotent: an
/// unchanged declaration rewrites nothing, so a host may call it on every frame.
///
/// 🔗️ The one source is the app manifest (`window_kind_action_refs` → `WindowKindDefinition.actions`);
/// see [`World3dState::declared_action_ids`] for what a lost declaration cost on 6118.
pub fn set_world3d_declared_actions(state: &mut World3dState, action_ids: &[String]) {
    if state.declared_action_ids.len() == action_ids.len() && state.declared_action_ids.iter().zip(action_ids).all(|(held, declared)| held == declared) {
        return;
    }
    state.declared_action_ids.clear();
    state.declared_action_ids.extend_from_slice(action_ids);
}

/// 📇️ Whether this surface's window kind declares `action_id`, i.e. whether the surface may mint it.
pub fn world3d_declares_action(state: &World3dState, action_id: &str) -> bool {
    state.declared_action_ids.iter().any(|declared| declared == action_id)
}

/// 🎚️ Whether this surface may offer a transform gumball at all — true when its window kind declares
/// at least one of the three verbs a gumball handle commits as.
pub fn world3d_offers_transform_gumball(state: &World3dState) -> bool {
    [TRANSLATE_SELECTION_ACTION_ID, ROTATE_SELECTION_ACTION_ID, SCALE_SELECTION_ACTION_ID].iter().any(|action_id| world3d_declares_action(state, action_id))
}

#[cfg(not(test))]
impl Drop for World3dState {
    fn drop(&mut self) {
        assert!(world3d_dynamic_retirement_terminal_is_empty(self) || std::thread::panicking(), "World3dState reached Drop before retained dynamic owners reached terminal empty");
    }
}
//#endregion World3dState

#[path = "🧹️lifecycle/🦀️.rs"]
mod lifecycle;
pub use lifecycle::*;
#[path = "🧵️interaction/🦀️.rs"]
mod interaction;
pub use interaction::*;
#[path = "🧱️scene/🦀️.rs"]
mod scene;
pub use scene::*;
#[path = "🖌️overlay/🦀️.rs"]
mod overlay;
pub use overlay::*;
#[path = "🌉️bridge/🦀️.rs"]
mod bridge;
pub use bridge::*;
#[path = "🎛️frame/🦀️.rs"]
mod frame;
pub use frame::*;
#[path = "⏯️tool-run-trace/🦀️.rs"]
pub mod tool_run_trace;

#[path = "🖱️pointer/🦀️.rs"]
mod pointer;
pub use pointer::*;
#[path = "📡️assets/🦀️.rs"]
mod assets;
pub use assets::*;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

/// 🖱️ The gesture→action law this surface shares with the React `World3dHost`, read from
/// `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`.
#[cfg(test)]
#[path = "🧪️tests/🖱️pointer-gestures/🦀️.rs"]
mod pointer_gesture_tests;

/// 🧲️ Grid-snap input and active-document reference ownership over one shared neutral fixture.
#[cfg(test)]
#[path = "🧪️tests/🧲️scene-input-residency/🦀️.rs"]
mod scene_input_residency_tests;

/// 📜️ The ORDERED action journal one probe gesture publishes, against React's own measured
/// sequences in `🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json`.
#[cfg(test)]
#[path = "🧪️tests/📜️journal-sequences/🦀️.rs"]
mod journal_sequence_tests;

/// 📇️ Which verbs a surface may emit, read from
/// `🌐️World3dHost/🧫️fixtures/📇️surface-verbs.json` — the oracle its TypeScript twin answers too.
#[cfg(test)]
#[path = "🧪️tests/📇️surface-verbs/🦀️.rs"]
mod surface_verb_tests;

/// 🌉️ A mesh update never quarantines the surface, over
/// `🧫️fixtures/🌉️bridge-empty-build/🔣️.json` applied to the committed `🌉️scene-bridge` payload.
#[cfg(test)]
#[path = "🧪️tests/🌉️bridge-empty-build/🦀️.rs"]
mod bridge_empty_build_tests;

#[cfg(test)]
fn world_test_mesh_query<T>(mut query:impl FnMut()->Result<T,ui_wgpu::wgpu::Mesh3dFault>)->Result<T,ui_wgpu::wgpu::Mesh3dFault>{
    let deadline=std::time::Instant::now()+std::time::Duration::from_millis(5000);
    let cancel=std::sync::atomic::AtomicBool::new(false);
    for _ in 0..4096{
        if cancel.load(std::sync::atomic::Ordering::Acquire)||std::time::Instant::now()>=deadline{return Err(ui_wgpu::wgpu::Mesh3dFault::Busy);}
        match query(){Err(ui_wgpu::wgpu::Mesh3dFault::Busy)=>std::thread::yield_now(),result=>return result}
    }
    Err(ui_wgpu::wgpu::Mesh3dFault::Busy)
}
