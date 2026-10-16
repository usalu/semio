//! 🗃️ World3d data model: scene records, resource slots and registries, interaction ids, and the tool run trace layer.
//!
//! Crate `semio-framework-os-infinite-world-model`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use std::collections::{BTreeMap, HashMap};
use ui_wgpu::wgpu::{ComponentSource3d, Instance3d, Mat4, mesh3d_begin_close, mesh3d_close_step, mesh3d_terminal_is_empty, Mesh3dLease, Mesh3dSchema, PointerModifiers, PreparedRasterProducer, PreparedRasterRejected, PreparedRenderEviction, PreparedRenderUpload, ray_aabb_slab, Rect, Rgba, SceneColorSource3d, SceneDraw3d, SceneRasterIdentity, SceneRasterLease, SceneRasterPool, Theme, Vec3, World3dPresentation, World3dSnapshotFault};
use serde::Deserialize;
use std::ops::{Index, IndexMut};
use std::sync::{LazyLock, Mutex};
use std::mem::MaybeUninit;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{SceneAuthoredMaterial3d, SceneInstanceMaterial3d};
use semio_framework_tool_run::{TOOL_RUN_TRACE_PAGE_BYTES_MAX, ToolRunCodecError, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict};
use ui_wgpu::wgpu::kernel_3d_scene::Mesh3dResourceLoan as WorldNativeMeshLoan;
use semio_framework_ui_contract::UiRevision as WorldNativeUiRevision;
use serde_json::json;
use ui_styling::metrics::tool_run;

#[path = "⏯️tool-run-trace/🦀️.rs"]
pub mod tool_run_trace;

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

pub fn world_close_string(key:&mut String,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
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
    pub uploads: WorldSlots<PreparedRenderUpload>,
    upload_len: usize,
    pub raster_producers: WorldSlots<PreparedRasterProducer>,
    raster_producer_len: usize,
    pub evictions: WorldSlots<PreparedRenderEviction>,
    eviction_len: usize,
    pub mesh_requests: WorldSlots<(String, u64)>,
    pub mesh_request_len: usize,
    raster_requests: WorldSlots<String>,
    raster_request_len: usize,
    pub rejected: Option<World3dBuildRejected>,
    pub cursor_wake: std::mem::ManuallyDrop<Option<WorldCursorWakeAuthority>>,
    cursor_wake_retirement: Option<semio_framework_value::retirement::shared::SharedControlledRetirement<Mutex<WorldCursorWakeState>>>,
    cursor_wake_token: Option<WorldCursorWakeToken>,
    cursor_wake_fault: Option<WorldCursorWakeFault>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WorldCursorWakeToken {
    pub generation: u64,
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
    pub generation: u64,
    acknowledged_generation: u64,
    pub pending_generation: Option<u64>,
    close_phase: u8,
    pub closing: bool,
    pub terminal: bool,
}

semio_framework_value::artifact_retire_leaf!(WorldCursorWakeState);

#[derive(Clone, Default)]
pub struct WorldCursorWakeAuthority(pub std::sync::Arc<Mutex<WorldCursorWakeState>>);

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

    #[cfg(any(test, feature = "testkit"))]
    pub fn pending_generation(&self) -> Option<u64> {
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

    pub fn request_cursor_wake(&mut self) {
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
#[cfg(any(test, feature = "testkit"))]
pub struct WorldMeshLodEntry {
    pub lod: f64,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldLodRecord {
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    pub automatic: bool,
    #[serde(default = "default_manual_lod")]
    #[value(default = "default_manual_lod")]
    pub manual: f64,
    #[serde(default = "default_distance_reference")]
    #[value(default = "default_distance_reference")]
    pub distance_reference: f64,
    #[serde(default)]
    #[value(default)]
    pub depth_variable: bool,
    #[serde(default = "default_grid_factor")]
    #[value(default = "default_grid_factor")]
    pub grid_factor: f64,
    #[serde(default)]
    #[value(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    pub show_grid: bool,
    #[serde(default)]
    #[value(default)]
    pub grid_datum: Option<[f64; 3]>,
}

impl Default for WorldLodRecord {
    fn default() -> Self {
        default_lod_record()
    }
}

#[derive(Clone, Debug, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldChunkingRecord {
    pub chunk_size: f64,
    pub max_distance: f64,
}

pub fn default_manual_lod() -> f64 {
    100.0
}

pub fn default_distance_reference() -> f64 {
    100.0
}

pub fn default_grid_factor() -> f64 {
    10.0
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldSelectionTargets {
    #[serde(default)]
    #[value(default)]
    pub vertex: bool,
    #[serde(default)]
    #[value(default)]
    pub edge: bool,
    #[serde(default)]
    #[value(default)]
    pub face: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldVortexRecord {
    pub full_id: String,
    #[value(default)]
    pub position: Option<[f64; 3]>,
    #[value(default)]
    pub direction: Option<[f64; 3]>,
    #[value(default)]
    pub display_direction: Option<String>,
    #[value(default)]
    pub radius: Option<f64>,
    #[value(default)]
    pub color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldAttractionRecord {
    #[value(default)]
    pub id: Option<String>,
    #[value(default)]
    pub from: Option<[f64; 3]>,
    #[value(default)]
    pub to: Option<[f64; 3]>,
    #[value(default)]
    pub color: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldTutorialGeometry {
    pub point: [f32; 2],
    pub polyline: Option<Vec<[f32; 2]>>,
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldTargetVolumeRecord {
    #[value(default)]
    pub origin: Option<[f64; 3]>,
    #[value(default)]
    pub orientation: Option<[f64; 4]>,
    #[value(default)]
    pub scale: Option<[f64; 3]>,
    #[value(default)]
    pub color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldReferenceRecord {
    #[value(default)]
    pub id: Option<String>,
    #[value(default)]
    pub url: Option<String>,
    #[value(default)]
    pub origin: Option<[f64; 3]>,
    #[value(default)]
    pub width_world: Option<f64>,
    #[value(default)]
    pub locked: Option<bool>,
    #[value(default)]
    pub opacity: Option<f64>,
    #[value(default)]
    pub hidden: Option<bool>,
}

/// 🖼️ React's authored fallback for a reference without `widthWorld`.
const WORLD_REFERENCE_DEFAULT_WIDTH: f32 = 10.0;

pub fn world_reference_width(reference: &WorldReferenceRecord) -> f32 {
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
pub struct WorldEngagementPreviewRecord {
    #[value(default)]
    pub kind: Option<String>,
    #[value(default)]
    pub role: Option<String>,
    #[value(default)]
    pub position: Option<[f64; 3]>,
    #[value(default)]
    pub from: Option<[f64; 3]>,
    #[value(default)]
    pub to: Option<[f64; 3]>,
    #[value(default)]
    pub corner_a: Option<[f64; 3]>,
    #[value(default)]
    pub corner_b: Option<[f64; 3]>,
    #[value(default)]
    pub height: Option<f64>,
    #[value(default)]
    pub axis: Option<[f64; 3]>,
    #[value(default)]
    pub origin: Option<[f64; 3]>,
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
    pub color: Option<String>,
    #[value(default)]
    pub emissive: Option<String>,
    #[value(default)]
    pub opacity: Option<f64>,
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
pub struct WorldPickTargetRecord {
    #[value(default)]
    pub kind: String,
    #[value(default)]
    pub id: String,
    #[value(default)]
    pub point: Option<[f64; 3]>,
    #[value(default)]
    pub points: Vec<[f64; 3]>,
    #[value(default)]
    typology: Option<String>,
    /// 👁️ `false` for a hidden or locked entity — it still PAINTS (dimmed, through `style`) and is
    /// never hit-tested, exactly as React's `SpatialPickGeometryLayer` renders such a target through
    /// `SpatialPickTargetNode` while `selectableTargets` leaves out its `SpatialPickHitTarget`.
    #[value(default)]
    selectable: Option<bool>,
    #[value(default)]
    pub style: Option<WorldPickTargetStyleRecord>,
}

impl WorldPickTargetRecord {
    /// 🪪️ `kind:id`, the key every hover/selection comparison is made on — the twin of the CAD
    /// engine's `spatial_pick_target_key`.
    pub fn key(&self) -> String {
        format!("{}:{}", self.kind, self.id)
    }

    /// 👁️ See [`WorldPickTargetRecord::selectable`].
    pub fn is_selectable(&self) -> bool {
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
pub fn world_pick_target_bounds(target: &WorldPickTargetRecord) -> Option<([f32; 3], [f32; 3])> {
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
pub fn world_pick_target_ray_score(target: &WorldPickTargetRecord, origin: Vec3, direction: Vec3) -> Option<f32> {
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
pub fn world_pick_keys_match(left: Option<&str>, right: Option<&str>) -> bool {
    let (Some(left), Some(right)) = (left.filter(|key| !key.is_empty()), right.filter(|key| !key.is_empty())) else {
        return false;
    };
    left == right || world_pick_key_alias(left).as_deref() == Some(right) || world_pick_key_alias(right).as_deref() == Some(left)
}

/// 🚚️ Parses the lane into the bounded retained set — every cap applied here, never at paint time.
pub fn world_pick_targets_from_json(json: &str) -> Vec<WorldPickTargetRecord> {
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
pub struct WorldBrushPreviewRecord {
    pub mesh_url: Option<String>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<serde_json::Value>,
    pub target_vortex_full_id: Option<String>,
    pub object_kind_id: Option<String>,
    pub source_vortex_index: Option<usize>,
    pub color: Option<String>,
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
pub struct WorldEnvironmentSunRecord {
    #[value(default)]
    pub enabled: Option<bool>,
    #[value(default)]
    pub azimuth: Option<f64>,
    #[value(default)]
    pub elevation: Option<f64>,
    #[value(default)]
    pub intensity: Option<f64>,
    #[value(default)]
    pub color: Option<String>,
}

/// 💡️ Ambient light carried into the scene pass and GPU globals.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldEnvironmentAmbientRecord {
    #[value(default)]
    pub intensity: Option<f64>,
    #[value(default)]
    pub color: Option<String>,
}

/// 🌑️ Directional shadow controls shared by World3d and IconRender.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldEnvironmentShadowRecord {
    #[value(default)]
    pub enabled: Option<bool>,
    #[value(default)]
    pub opacity: Option<f64>,
    #[value(default)]
    pub softness: Option<f64>,
}

/// 🎨️ Neutral-instance material override. Selection and hover keep their semantic base/emissive
/// rows while sharing the material's metalness and roughness, as React's `GlbInstanceMesh` does.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct WorldEnvironmentMaterialRecord {
    #[value(default)]
    pub color: Option<String>,
    #[value(default)]
    pub metalness: Option<f64>,
    #[value(default)]
    pub roughness: Option<f64>,
    #[value(default)]
    pub emissive: Option<String>,
    #[value(default)]
    pub emissive_intensity: Option<f64>,
    #[value(default)]
    pub stroke: Option<String>,
}

/// 🌍️ `World3dScene.environmentJson` mirror — see `world-3d-host.tsx`'s `WorldEnvironmentRecord`.
/// Every visible light/material field is resolved into [`ScenePass3d`] before GPU preparation.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldEnvironmentRecord {
    #[value(default)]
    pub background: Option<String>,
    #[value(default)]
    pub ambient: Option<WorldEnvironmentAmbientRecord>,
    #[value(default)]
    pub sun: Option<WorldEnvironmentSunRecord>,
    #[value(default)]
    pub shadow: Option<WorldEnvironmentShadowRecord>,
    #[value(default)]
    pub material: Option<WorldEnvironmentMaterialRecord>,
}
//#endregion Environment

//#region TerrainStyle
/// 🌐️⛰️ `World3dScene.terrainJson` mirror — GIS-3D terrain style/source descriptor consumed by
/// `🗺️WorldTerrainLayer` in the React reference. `color_ramp`/`min_zoom`/`max_zoom` are parsed but
/// not branched on, mirroring the React reference (single hardcoded hypsometric ramp; zoom bounds
/// fixed inside `semio_framework_surface_terrain::tiles`) — not a gap, a faithful match of upstream.
#[derive(Clone, Debug, Deserialize, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldTerrainStyle {
    pub tile_url_template: String,
    #[serde(default)]
    #[value(default)]
    pub project_origin_lon: f64,
    #[serde(default)]
    #[value(default)]
    pub project_origin_lat: f64,
    #[serde(default = "default_terrain_exaggeration")]
    #[value(default = "default_terrain_exaggeration")]
    pub exaggeration: f64,
    #[serde(default = "default_terrain_color_ramp")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_color_ramp")]
    pub color_ramp: String,
    #[serde(default = "default_terrain_min_zoom")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_min_zoom")]
    pub min_zoom: u32,
    #[serde(default = "default_terrain_max_zoom")]
    #[allow(dead_code)]
    #[value(default = "default_terrain_max_zoom")]
    pub max_zoom: u32,
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
pub fn world_mesh_ownership_grant(copy: usize, capacity: usize, release: usize, depth: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth }
}

/// 🧾️ Verifies the original owner receipt before World advances its publication or close phase.
pub fn world_mesh_ownership_complete(grant: semio_framework_value::retained_clone::RetainedCloneGrant, step: ui_wgpu::wgpu::Mesh3dOwnershipStep) -> Result<bool, ui_wgpu::wgpu::Mesh3dFault> {
    if !step.progress.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::Closing); }
    Ok(step.complete)
}

pub const WORLD_DYNAMIC_MESH_CAPACITY: usize = 256;

const WORLD_DYNAMIC_DRAW_CAPACITY: usize = 256;

pub const WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY: usize = 4_096;

pub const WORLD_DYNAMIC_DRAW_BYTE_CAPACITY: usize = 16 * 1024 * 1024;

pub const WORLD_DYNAMIC_PIXEL_CAPACITY: usize = 256;

const WORLD_OPAQUE_QUARANTINE_CAPACITY: usize = 1024;

pub const WORLD_DYNAMIC_ID_BYTE_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldDynamicToken {
    pub slot: u16,
    pub epoch: u64,
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
pub struct WorldDynamicEntry<T> {
    pub id: String,
    pub epoch: u64,
    pub value: T,
}

#[derive(Debug)]
pub struct WorldDynamicRejected<T> {
    pub fault: WorldDynamicFault,
    pub id: String,
    pub value: T,
}

pub struct WorldDynamicRegistry<T, const N: usize> {
    pub slots: Box<[Option<WorldDynamicEntry<T>>; N]>,
    pub epochs: [u64; N],
    pub len: u16,
    pub closing: bool,
}

#[derive(Debug)]
pub enum WorldMeshSource {
    Owned{lease:Option<Mesh3dLease>,closing:bool},
    Native(WorldNativeMeshSource),
}

#[derive(Debug)]
pub struct WorldNativeMeshSource {
    pub loan: WorldNativeMeshLoan,
    pub receipt: semio_framework::kernel::ActorUiPatchReceipt,
    pub revision: WorldNativeUiRevision,
    pub resource_ordinal: usize,
    pub lookup_extent: usize,
    pub frozen: bool,
    pub gpu_returned: bool,
}

impl WorldMeshSource {
    pub fn lease(&self)->Option<&Mesh3dLease>{match self{Self::Owned{lease,..}=>lease.as_ref(),Self::Native(source) if !source.frozen=>source.loan.lease_ref(),Self::Native(_)=>None}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ui_wgpu::wgpu::Mesh3dFault>{match self{
        Self::Owned{lease,closing}=>{let Some(value)=*lease else{return Ok((true,Default::default()))};if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}if !*closing{match mesh3d_begin_close(value){Ok(())|Err(ui_wgpu::wgpu::Mesh3dFault::Closing)=>*closing=true,Err(ui_wgpu::wgpu::Mesh3dFault::Stale)=>{lease.take();return Ok((true,RetainedCloneProgress{copied_items:1,..Default::default()}))},Err(fault)=>return Err(fault)}return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))}let step=mesh3d_close_step(value,grant)?;if step.complete{lease.take();}Ok((step.complete,step.progress))},
        Self::Native(source)=>{if !source.gpu_returned{return Ok((false,Default::default()))}let step=source.loan.return_step(grant)?;Ok((step.complete,step.progress))}
    }}
}

#[derive(Default)]
pub struct WorldMeshRegistry(pub WorldDynamicRegistry<WorldMeshSource,WORLD_DYNAMIC_MESH_CAPACITY>);

impl std::ops::Deref for WorldMeshRegistry{type Target=WorldDynamicRegistry<WorldMeshSource,WORLD_DYNAMIC_MESH_CAPACITY>;fn deref(&self)->&Self::Target{&self.0}}

impl std::ops::DerefMut for WorldMeshRegistry{fn deref_mut(&mut self)->&mut Self::Target{&mut self.0}}

impl WorldMeshRegistry {
    pub fn get(&self,id:&str)->Option<&Mesh3dLease>{self.0.get(id)?.lease()}
    #[cfg(any(test, feature = "testkit"))]
    pub fn iter(&self)->impl Iterator<Item=(&String,&Mesh3dLease)>{self.0.slots.iter().flatten().filter_map(|entry|entry.value.lease().map(|lease|(&entry.id,lease)))}
    #[cfg(any(test, feature = "testkit"))]
    pub fn insert(&mut self,id:String,value:Mesh3dLease)->Result<(WorldDynamicToken,Option<WorldDynamicEntry<WorldMeshSource>>),WorldDynamicRejected<Mesh3dLease>>{self.0.insert(id,WorldMeshSource::Owned{lease:Some(value),closing:false}).map_err(|rejected|WorldDynamicRejected{fault:rejected.fault,id:rejected.id,value})}
    pub fn commit_insert(&mut self,plan:WorldDynamicInsertPlan,id:String,value:Mesh3dLease)->Result<(WorldDynamicToken,Option<WorldDynamicEntry<WorldMeshSource>>),WorldDynamicRejected<Mesh3dLease>>{self.0.commit_insert(plan,id,WorldMeshSource::Owned{lease:Some(value),closing:false}).map_err(|rejected|WorldDynamicRejected{fault:rejected.fault,id:rejected.id,value})}
}

pub struct WorldNativeMeshAdmission {
    pub identity:ui_wgpu::wgpu::kernel_3d_scene::Mesh3dResourceIdentity,
    pub lease:Mesh3dLease,
    pub receipt:semio_framework::kernel::ActorUiPatchReceipt,
    pub revision:WorldNativeUiRevision,
    pub resource_ordinal:usize,
    pub key:String,
    pub extent:usize,
    pub interaction_id:WorldInteractionId,
    pub interaction_offset:usize,
    pub allocated:bool,
}

#[derive(Debug)]
pub enum WorldNativeMeshFailure { Binding, Registry(WorldDynamicFault), Mesh(ui_wgpu::wgpu::Mesh3dFault), Gpu(&'static str), Close(semio_framework_value::ValueError) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorldDynamicInsertPlan {
    pub slot: u16,
    pub epoch: u64,
    pub occupied: bool,
}

impl<T, const N: usize> Default for WorldDynamicRegistry<T, N> {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; N], len: 0, closing: false }
    }
}

impl<T, const N: usize> WorldDynamicRegistry<T, N> {
    pub fn get(&self, id: &str) -> Option<&T> {
        self.slots.iter().flatten().find(|entry| entry.id == id).map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        self.slots.iter_mut().flatten().find(|entry| entry.id == id).map(|entry| &mut entry.value)
    }

    pub fn contains_key(&self, id: &str) -> bool {
        self.get(id).is_some()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.slots.iter().flatten().map(|entry| &entry.id)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.slots.iter().flatten().map(|entry| (&entry.id, &entry.value))
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn token(&self, id: &str) -> Option<WorldDynamicToken> {
        self.slots.iter().enumerate().find_map(|(slot, entry)| entry.as_ref().filter(|entry| entry.id == id).map(|entry| WorldDynamicToken { slot: slot as u16, epoch: entry.epoch }))
    }

    pub fn insert(&mut self, id: String, value: T) -> Result<(WorldDynamicToken, Option<WorldDynamicEntry<T>>), WorldDynamicRejected<T>> {
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

    pub fn plan_insert(&self, id: &str) -> Result<WorldDynamicInsertPlan, WorldDynamicFault> {
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

    pub fn commit_insert(&mut self, plan: WorldDynamicInsertPlan, id: String, value: T) -> Result<(WorldDynamicToken, Option<WorldDynamicEntry<T>>), WorldDynamicRejected<T>> {
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

    pub fn remove(&mut self, id: &str) -> Option<WorldDynamicEntry<T>> {
        let slot = self.slots.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.id == id))?;
        self.len -= 1;
        self.slots[slot].take()
    }

    pub fn remove_token(&mut self, token: WorldDynamicToken) -> Result<WorldDynamicEntry<T>, WorldDynamicFault> {
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

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn take_one(&mut self) -> Option<WorldDynamicEntry<T>> {
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

#[cfg(not(any(test, feature = "testkit")))]
impl<T, const N: usize> Drop for WorldDynamicRegistry<T, N> {
    fn drop(&mut self) {
        assert!(self.is_empty(), "world dynamic registry reached Drop before its exact terminal-empty witness");
    }
}

pub struct WorldDrawRegistry {
    pub slots: Box<[Option<WorldDynamicEntry<SceneDraw3d>>; WORLD_DYNAMIC_DRAW_CAPACITY]>,
    pub epochs: [u64; WORLD_DYNAMIC_DRAW_CAPACITY],
    pub len: u16,
    pub closing: bool,
}

impl Default for WorldDrawRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; WORLD_DYNAMIC_DRAW_CAPACITY], len: 0, closing: false }
    }
}

impl WorldDrawRegistry {
    pub fn get(&self, index: usize) -> Option<&SceneDraw3d> {
        self.slots.get(index).and_then(Option::as_ref).map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut SceneDraw3d> {
        self.slots.get_mut(index).and_then(Option::as_mut).map(|entry| &mut entry.value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SceneDraw3d> {
        self.slots[..usize::from(self.len)].iter().filter_map(|entry| entry.as_ref().map(|entry| &entry.value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SceneDraw3d> {
        self.slots[..usize::from(self.len)].iter_mut().filter_map(|entry| entry.as_mut().map(|entry| &mut entry.value))
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn push(&mut self, value: SceneDraw3d) -> Result<WorldDynamicToken, WorldDynamicRejected<SceneDraw3d>> {
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

    pub fn push_prevalidated(&mut self, value: SceneDraw3d) -> Result<WorldDynamicToken, WorldDynamicRejected<SceneDraw3d>> {
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

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(any(test, feature = "testkit"))]
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

    pub fn take_last(&mut self) -> Option<WorldDynamicEntry<SceneDraw3d>> {
        let len = usize::from(self.len);
        if len == 0 {
            return None;
        }
        self.len -= 1;
        self.slots[len - 1].take()
    }

    pub fn restore_last(&mut self, entry: WorldDynamicEntry<SceneDraw3d>) {
        let slot = usize::from(self.len);
        self.slots[slot] = Some(entry);
        self.len += 1;
    }

    pub fn begin_close(&mut self) {
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
    pub mesh_key: String,
    pub mesh_version: u64,
    pub slots: Box<[MaybeUninit<Instance3d>]>,
    pub admitted: usize,
    taken: usize,
}

impl WorldDrawDraft {
    pub fn new(mesh_key: String, mesh_version: u64, instance_count: usize) -> Self {
        Self { mesh_key, mesh_version, slots: Box::<[Instance3d]>::new_uninit_slice(instance_count), admitted: 0, taken: 0 }
    }

    pub fn push(&mut self, instance: Instance3d) -> Result<(), Instance3d> {
        if self.admitted == self.slots.len() {
            return Err(instance);
        }
        self.slots[self.admitted].write(instance);
        self.admitted += 1;
        Ok(())
    }

    pub fn take_next(&mut self) -> Option<Instance3d> {
        if self.taken == self.admitted {
            return None;
        }
        let instance = unsafe { self.slots[self.taken].assume_init_read() };
        self.taken += 1;
        Some(instance)
    }

    pub fn complete(&self) -> bool {
        self.admitted == self.slots.len()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken == self.admitted
    }
}

impl Drop for WorldDrawDraft {
    fn drop(&mut self) {
        #[cfg(not(any(test, feature = "testkit")))]
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldDrawDraft reached Drop before every admitted instance reached its terminal handback");
        #[cfg(any(test, feature = "testkit"))]
        while self.take_next().is_some() {}
    }
}

pub struct WorldDrawRebuildCursor {
    pub descriptor: WorldDrawRebuildDescriptor,
    pub drafts: Box<[Option<WorldDrawDraft>; WORLD_DYNAMIC_DRAW_CAPACITY]>,
    admitted_draws: u16,
    admitted_instances: u32,
    admitted_bytes: u32,
    pub publishing: bool,
    pub publish_draw: u16,
    pub output: Option<SceneDraw3d>,
    pub staged: WorldDrawRegistry,
    pub faulted: bool,
}

impl WorldDrawRebuildCursor {
    pub fn new(descriptor: WorldDrawRebuildDescriptor) -> Result<Self, WorldDynamicFault> {
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

    pub fn admit_draw(&mut self, mesh_key: &str, mesh_version: u64, instance_count: u16) -> Result<(), WorldDynamicFault> {
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

    pub fn admit_instance(&mut self, draw: u16, id: &str, model: Mat4, color: [f32; 4], color_source: SceneColorSource3d, selected: bool, hovered: bool, component_source: Option<ComponentSource3d>) -> Result<(), WorldDynamicFault> {
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

    pub fn seal(&mut self) -> Result<(), WorldDynamicFault> {
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

    pub fn terminal_is_empty(&self) -> bool {
        self.output.as_ref().is_none_or(|draw| draw.instances.is_empty()) && self.drafts.iter().flatten().all(WorldDrawDraft::terminal_is_empty) && self.staged.is_empty()
    }
}

#[cfg(not(any(test, feature = "testkit")))]
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

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldDrawRegistry {
    fn drop(&mut self) {
        assert!(self.is_empty() || std::thread::panicking(), "world draw registry reached Drop before its exact terminal-empty witness");
    }
}

/// 🖼️ Bounded identity and lease metadata; dropping it releases one token while the pool retains pixel storage.
#[derive(Debug)]
pub struct WorldSceneRaster {
    pub identity: SceneRasterIdentity,
    pub pending: Mutex<Option<SceneRasterLease>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct World3dPrimitiveMaterial {
    pub first_index: u32,
    pub index_count: u32,
    pub material: SceneAuthoredMaterial3d,
}

#[derive(Debug)]
pub struct World3dMeshAppearance {
    pub primitives: Vec<World3dPrimitiveMaterial>,
    pub textures: Vec<(String, WorldSceneRaster)>,
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

    pub fn texture_upload(&self, key: &str, pool: &SceneRasterPool) -> Option<SceneRasterLease> {
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

pub enum WorldOpaqueOwner {
    Draw(WorldDynamicEntry<SceneDraw3d>),
}

pub struct WorldOpaqueQuarantine<const N: usize> {
    pub slots: Box<[Option<WorldOpaqueOwner>; N]>,
    pub len: u16,
    pub saturated: u64,
}

impl<const N: usize> Default for WorldOpaqueQuarantine<N> {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), len: 0, saturated: 0 }
    }
}

impl<const N: usize> WorldOpaqueQuarantine<N> {
    pub fn admit(&mut self, owner: WorldOpaqueOwner) -> Result<WorldDynamicToken, WorldOpaqueOwner> {
        let slot = usize::from(self.len);
        if slot == N {
            self.saturated = self.saturated.saturating_add(1);
            return Err(owner);
        }
        self.slots[slot] = Some(owner);
        self.len += 1;
        Ok(WorldDynamicToken { slot: slot as u16, epoch: 1 })
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn take_one(&mut self) -> Option<WorldOpaqueOwner> {
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

pub fn quarantine_world_owner(owner: WorldOpaqueOwner) -> Result<WorldDynamicToken, WorldOpaqueOwner> {
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
pub struct WorldSubElementPaint {
    pub face_select: Option<[f32; 4]>,
    pub face_hover: Option<[f32; 4]>,
    pub edge_select: Option<[f32; 4]>,
    pub edge_hover: Option<[f32; 4]>,
    pub vertex_select: Option<[f32; 4]>,
    pub vertex_hover: Option<[f32; 4]>,
    pub vertex_mark_px: Option<f32>,
}

/// 🧩️ World3d session component: surface identity, bounds, domain binding, utilities and presentation of one 3D surface.
pub struct WorldSessionState {

    pub surface_id: String,

    pub controller_id: String,

    pub bounds: Rect,

    pub pick_bounds: Rect,

    /// 🪟️ The app-bound `InteractionDefinition` id for this window (see `World3dScene.domain_id`'s
    /// doc comment) — `None` means this window binds no app domain, so plain picks/hover target the
    /// OS's own shared `world` board domain (see `resolved_domain_id`).
    pub bound_domain_id: Option<String>,

    /// 🎯️ The bound domain's granularity id for a plain (non-component) pick/hover hit.
    pub bound_domain_granularity_id: Option<String>,

    pub active_utility: String,

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
    pub declared_action_ids: Vec<String>,

    pub presentation: World3dPresentation,

    pub interaction_revision: u64,

    pub snapshot_fault: Option<World3dSnapshotFault>,

    pub modelling_locale: &'static str,
}

/// 🧩️ World3d interaction component: interaction mesh/object registries and instance interaction ids.
pub struct WorldInteractionState {

    /// 🎯️ Rendered instance id → the TOPOLOGY target it stands for, from the scene's own
    /// `instances[].interactionId`. Only the entries that actually differ are kept, so a lookup miss
    /// means "this instance is its own target" — see [`instance_interaction_id`].
    pub instance_interaction_ids: HashMap<String, String>,

    pub instance_interaction_granularity_ids: HashMap<String, String>,

    pub interaction_meshes: WorldInteractionMeshRegistry,

    pub interaction_objects: WorldInteractionObjectRegistry,
}

impl WorldSessionState {
    pub fn new(surface_id: String, controller_id: String) -> Self {
        Self {
            surface_id,
            controller_id,
            bounds: Rect::default(),
            pick_bounds: Rect::default(),
            bound_domain_id: None,
            bound_domain_granularity_id: None,
            active_utility: "select".into(),
            declared_action_ids: Vec::new(),
            presentation: World3dPresentation::default(),
            interaction_revision: 0,
            snapshot_fault: None,
            modelling_locale: "en",
        }
    }
}

impl WorldInteractionState {
    pub fn new() -> Self {
        Self {
            instance_interaction_ids: HashMap::new(),
            instance_interaction_granularity_ids: HashMap::new(),
            interaction_meshes: WorldInteractionMeshRegistry::default(),
            interaction_objects: WorldInteractionObjectRegistry::default(),
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn world_test_mesh_query<T>(mut query:impl FnMut()->Result<T,ui_wgpu::wgpu::Mesh3dFault>)->Result<T,ui_wgpu::wgpu::Mesh3dFault>{
    let deadline=std::time::Instant::now()+std::time::Duration::from_millis(5000);
    let cancel=std::sync::atomic::AtomicBool::new(false);
    for _ in 0..4096{
        if cancel.load(std::sync::atomic::Ordering::Acquire)||std::time::Instant::now()>=deadline{return Err(ui_wgpu::wgpu::Mesh3dFault::Busy);}
        match query(){Err(ui_wgpu::wgpu::Mesh3dFault::Busy)=>std::thread::yield_now(),result=>return result}
    }
    Err(ui_wgpu::wgpu::Mesh3dFault::Busy)
}

//#region 🔖️Limits
/// 📏️ Longest `World3dScene.tool_run_trace` text the render-plan validator admits: a delta of
/// sixteen full pages, base64url-expanded.
pub const TOOL_RUN_TRACE_LANE_BYTES_MAX: usize = (TOOL_RUN_TRACE_PAGE_BYTES_MAX * 16).div_ceil(3) * 4;
//#endregion 🔖️Limits

//#region 🔖️Store
/// 🧺️ The batch one record draws in: its subject family, the mesh or shape index, and its verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToolRunTraceBatchKey {
    pub family: ToolRunTraceFamily,
    pub index: u32,
    pub verdict: u8,
}

impl ToolRunTraceBatchKey {
    /// 🔎️ The batch a subject with `verdict` belongs to.
    pub fn of(subject: ToolRunTraceSubject, verdict: ToolRunVerdict) -> Self {
        let (family, index) = match subject {
            ToolRunTraceSubject::Instance3d { mesh, .. } => (ToolRunTraceFamily::Instance3d, mesh),
            ToolRunTraceSubject::Placement2d { shape, .. } => (ToolRunTraceFamily::Placement2d, shape),
            ToolRunTraceSubject::Entity { .. } => (ToolRunTraceFamily::Entity, 0),
        };
        Self { family, index, verdict: verdict.ordinal() }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        ToolRunVerdict::from_ordinal(self.verdict).unwrap_or(ToolRunVerdict::Testing)
    }
}

/// 👪️ Subject family of a batch, in draw order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ToolRunTraceFamily {
    Instance3d,
    Placement2d,
    Entity,
}

/// 📦️ One batch: parallel columns, swap-removed so upsert and retire stay O(1).
#[derive(Clone, Debug, Default)]
pub struct ToolRunTraceBatch {
    pub keys: Vec<u64>,
    pub subjects: Vec<ToolRunTraceSubject>,
    pub stamps: Vec<u64>,
}

#[derive(Clone, Copy, Debug)]
struct ToolRunTraceSlot {
    pub batch: ToolRunTraceBatchKey,
    pub index: usize,
}

/// 🚨️ Why a lane could not be applied; the layer keeps its previous records.
#[derive(Clone, Debug, PartialEq)]
pub enum ToolRunTraceLaneFault {
    Base64,
    Codec(ToolRunCodecError),
}

/// 📋️ What one lane application changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolRunTraceLayerApply {
    pub cleared: bool,
    pub pages: u32,
    pub ops: u32,
}

/// ⏯️ The per-window resident trace — see this module's header.
#[derive(Clone, Debug, Default)]
pub struct ToolRunTraceLayer {
    pub cursor: Option<ToolRunTraceCursor>,
    lane_digest: Option<u64>,
    records: HashMap<u64, ToolRunTraceSlot>,
    batches: BTreeMap<ToolRunTraceBatchKey, ToolRunTraceBatch>,
    next_stamp: u64,
    newest_testing: Option<(u64, u64)>,
}

impl ToolRunTraceLayer {
    /// 📥️ Applies one `tool_run_trace` lane text. `None` (no run tracing) keeps the resident records,
    /// and an unchanged text is skipped without decoding, so republishing a scene costs nothing.
    pub fn apply_lane(&mut self, lane: Option<&str>) -> Result<ToolRunTraceLayerApply, ToolRunTraceLaneFault> {
        let Some(lane) = lane else { return Ok(ToolRunTraceLayerApply::default()) };
        let digest = lane_digest(lane);
        if self.lane_digest == Some(digest) {
            return Ok(ToolRunTraceLayerApply::default());
        }
        let bytes = base64_codec::base64_url_decode(lane).map_err(|_| ToolRunTraceLaneFault::Base64)?;
        let delta = ToolRunTraceDelta::decode(&bytes).map_err(ToolRunTraceLaneFault::Codec)?;
        self.lane_digest = Some(digest);
        Ok(self.apply_delta(&delta))
    }

    /// 📬️ Applies one decoded delta. A `clear`, or a run/generation other than the resident one,
    /// drops every record first; pages below the expected cursor page were already applied and are
    /// skipped, so a redelivered delta is idempotent.
    pub fn apply_delta(&mut self, delta: &ToolRunTraceDelta) -> ToolRunTraceLayerApply {
        let run = delta.identity.id.run;
        let generation = delta.identity.generation;
        let rebound = self.cursor.is_none_or(|cursor| cursor.run != run || cursor.generation != generation);
        let mut outcome = ToolRunTraceLayerApply { cleared: delta.clear || rebound, ..Default::default() };
        if outcome.cleared {
            self.clear();
        }
        let expected = if outcome.cleared { 0 } else { self.cursor.map_or(0, |cursor| cursor.page) };
        for page in delta.pages.iter().filter(|page| outcome.cleared || page.page >= expected) {
            outcome.ops += self.apply_page(page);
            outcome.pages += 1;
        }
        let page = if outcome.cleared { delta.next } else { delta.next.max(expected) };
        self.cursor = Some(ToolRunTraceCursor { run, generation, page });
        outcome
    }

    fn apply_page(&mut self, page: &ToolRunTracePage) -> u32 {
        for op in &page.ops {
            match *op {
                ToolRunTraceOp::Clear => self.clear(),
                ToolRunTraceOp::Retire { key } => self.retire(key),
                ToolRunTraceOp::Upsert { key, verdict, subject, .. } => self.upsert(key, verdict, subject),
            }
        }
        page.ops.len() as u32
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.batches.clear();
        self.newest_testing = None;
    }

    fn upsert(&mut self, key: u64, verdict: ToolRunVerdict, subject: ToolRunTraceSubject) {
        self.retire(key);
        let stamp = self.next_stamp;
        self.next_stamp += 1;
        let batch_key = ToolRunTraceBatchKey::of(subject, verdict);
        let batch = self.batches.entry(batch_key).or_default();
        batch.keys.push(key);
        batch.subjects.push(subject);
        batch.stamps.push(stamp);
        self.records.insert(key, ToolRunTraceSlot { batch: batch_key, index: batch.keys.len() - 1 });
        if verdict == ToolRunVerdict::Testing {
            self.newest_testing = Some((key, stamp));
        }
    }

    pub fn retire(&mut self, key: u64) {
        let Some(slot) = self.records.remove(&key) else { return };
        let Some(batch) = self.batches.get_mut(&slot.batch) else { return };
        batch.keys.swap_remove(slot.index);
        batch.subjects.swap_remove(slot.index);
        batch.stamps.swap_remove(slot.index);
        if let Some(&moved) = batch.keys.get(slot.index) {
            if let Some(moved_slot) = self.records.get_mut(&moved) {
                moved_slot.index = slot.index;
            }
        }
        if batch.keys.is_empty() {
            self.batches.remove(&slot.batch);
        }
        if self.newest_testing.is_some_and(|(newest, _)| newest == key) {
            self.newest_testing = self.batches.iter().filter(|(batch_key, _)| batch_key.verdict() == ToolRunVerdict::Testing).flat_map(|(_, batch)| batch.keys.iter().copied().zip(batch.stamps.iter().copied())).max_by_key(|(_, stamp)| *stamp);
        }
    }

    /// 🧭️ What the renderer echoes as `toolRunTraceCursor` in its window instance view state.
    pub fn cursor(&self) -> Option<ToolRunTraceCursor> {
        self.cursor
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// 🔢️ Resident records with `verdict`, over every subject family.
    pub fn count(&self, verdict: ToolRunVerdict) -> usize {
        self.batches.iter().filter(|(key, _)| key.verdict() == verdict).map(|(_, batch)| batch.keys.len()).sum()
    }

    pub fn contains(&self, key: u64) -> bool {
        self.records.contains_key(&key)
    }

    /// 🧺️ Every non-empty batch in draw order.
    pub fn batches(&self) -> impl Iterator<Item = (&ToolRunTraceBatchKey, &ToolRunTraceBatch)> {
        self.batches.iter()
    }

    /// 🔦️ The most recently upserted record still `testing` — the one drawn with the highlight.
    pub fn newest_testing(&self) -> Option<u64> {
        self.newest_testing.map(|(key, _)| key)
    }

    /// 🎨️ One instanced draw per `(mesh, verdict)` for every visible 3d batch — see [`ToolRunTraceDraw`].
    pub fn draws(&self, palette: &ToolRunTracePalette, visibility: ToolRunTraceVisibility) -> Vec<ToolRunTraceDraw> {
        let newest_stamp = self.next_stamp.saturating_sub(1);
        let newest_testing = self.newest_testing();
        self.batches
            .iter()
            .filter(|(key, _)| key.family == ToolRunTraceFamily::Instance3d && visibility.shows(key.verdict()))
            .map(|(key, batch)| {
                let verdict = key.verdict();
                let base = palette.color(verdict);
                let instances = batch
                    .keys
                    .iter()
                    .zip(&batch.subjects)
                    .zip(&batch.stamps)
                    .filter_map(|((record, subject), stamp)| {
                        let ToolRunTraceSubject::Instance3d { position, rotation, scale, .. } = *subject else { return None };
                        let newest = newest_testing == Some(*record);
                        let alpha = if newest { base.a } else { base.a * tool_run_trace_fade(newest_stamp - stamp) };
                        Some(Instance3d { component_source: None, id: format!("toolRunTrace:{record}"), model: Instance3d::model_from_trs(position, rotation, [scale; 3]), color: [base.r, base.g, base.b, alpha], selected: newest, hovered: false, material: Default::default() })
                    })
                    .collect();
                ToolRunTraceDraw { mesh: key.index, verdict, instances }
            })
            .collect()
    }

    /// 🎨️ One filled footprint per visible placement2d record, oldest first within each batch, faded by age; the newest
    /// `testing` record carries `newest` so the host strokes it with the highlight.
    pub fn placement_draws(&self, palette: &ToolRunTracePalette, visibility: ToolRunTraceVisibility) -> Vec<ToolRunTrace2dDraw> {
        let newest_stamp = self.next_stamp.saturating_sub(1);
        let newest_testing = self.newest_testing();
        self.batches
            .iter()
            .filter(|(key, _)| key.family == ToolRunTraceFamily::Placement2d && visibility.shows(key.verdict()))
            .flat_map(|(key, batch)| {
                let base = palette.color(key.verdict());
                batch.keys.iter().zip(&batch.subjects).zip(&batch.stamps).filter_map(move |((record, subject), stamp)| {
                    let ToolRunTraceSubject::Placement2d { shape, position, rotation } = *subject else { return None };
                    let newest = newest_testing == Some(*record);
                    let alpha = if newest { base.a } else { base.a * tool_run_trace_fade(newest_stamp - stamp) };
                    Some(ToolRunTrace2dDraw { shape, position, rotation, color: base.with_alpha(alpha), newest })
                })
            })
            .collect()
    }
}

/// 🔷️ The world-unit footprint a board draws for one kind of its placement2d subjects.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolRunTraceShape2d {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

/// 📏️ World-unit edge of a scale-1 kind footprint, the size a board fill run captures its kinds at.
pub const BOARD2D_TOOL_RUN_TRACE_KIND_SIZE: f64 = 96.0;

/// 🔷️ The footprint of every `glyphCatalogsJson.nodeKinds` row, in row order — the index a placement2d subject's
/// `shape` names: `kindSize × scale` (a scale that is absent, non-finite or not positive reads 1), a square for
/// shape `rectangle`, else a circle of that diameter. A malformed document has no footprints. Pinned by
/// `🖱️ui/🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` `traceShapes`, mirrored by React's `board2dToolRunTraceShapes`.
pub fn board2d_tool_run_trace_shapes(glyph_catalogs_json: &str) -> Vec<ToolRunTraceShape2d> {
    let Ok(catalogs) = serde_json::from_str::<serde_json::Value>(glyph_catalogs_json) else { return Vec::new() };
    let Some(rows) = catalogs.get("nodeKinds").and_then(serde_json::Value::as_array) else { return Vec::new() };
    rows.iter()
        .map(|row| {
            let scale = row.get("scale").and_then(serde_json::Value::as_f64).filter(|scale| scale.is_finite() && *scale > 0.0).unwrap_or(1.0);
            let size = BOARD2D_TOOL_RUN_TRACE_KIND_SIZE * scale;
            match row.get("shape").and_then(serde_json::Value::as_str) {
                Some("rectangle") => ToolRunTraceShape2d::Rectangle { width: size, height: size },
                _ => ToolRunTraceShape2d::Circle { radius: size * 0.5 },
            }
        })
        .collect()
}

/// 🌫️ Age fade by sequence distance: 1 for the newest record, down to the `toolRun.fadeFloorOpacity`
/// token after `toolRun.fadeRecords` newer records.
pub fn tool_run_trace_fade(age: u64) -> f32 {
    let floor = tool_run::FADE_FLOOR_OPACITY as f32;
    (1.0 - age as f32 / tool_run::FADE_RECORDS as f32 * (1.0 - floor)).max(floor)
}

fn lane_digest(lane: &str) -> u64 {
    lane.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)) ^ lane.len() as u64
}
//#endregion 🔖️Store

//#region 🔖️Paint
/// 🎨️ Verdict paint from the theme's semantic outcome tokens: `testing` is the progress tone at the
/// `toolRun.testingOpacity` token, `success`/`warning`/`danger` are the theme's success, warning and
/// error tones. No literal colors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolRunTracePalette {
    pub testing: Rgba,
    pub success: Rgba,
    pub warning: Rgba,
    pub danger: Rgba,
}

impl ToolRunTracePalette {
    pub fn from_theme(theme: &Theme) -> Self {
        Self { testing: theme.progress.with_alpha(tool_run::TESTING_OPACITY as f32), success: theme.success, warning: theme.warning, danger: theme.error }
    }

    pub fn color(&self, verdict: ToolRunVerdict) -> Rgba {
        match verdict {
            ToolRunVerdict::Testing => self.testing,
            ToolRunVerdict::Success => self.success,
            ToolRunVerdict::Warning => self.warning,
            ToolRunVerdict::Danger => self.danger,
        }
    }
}

/// 👁️ The legend toggles a window config exposes (show testing / accepted / rejected).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolRunTraceVisibility {
    pub testing: bool,
    pub accepted: bool,
    pub rejected: bool,
}

impl Default for ToolRunTraceVisibility {
    fn default() -> Self {
        Self { testing: true, accepted: true, rejected: true }
    }
}

impl ToolRunTraceVisibility {
    pub fn shows(self, verdict: ToolRunVerdict) -> bool {
        match verdict {
            ToolRunVerdict::Testing => self.testing,
            ToolRunVerdict::Success => self.accepted,
            ToolRunVerdict::Warning | ToolRunVerdict::Danger => self.rejected,
        }
    }
}

/// 🖌️ One placement2d footprint fill: the subject's shape index, world center and rotation, its faded verdict color,
/// and whether it is the newest `testing` record.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolRunTrace2dDraw {
    pub shape: u32,
    pub position: [f32; 2],
    pub rotation: f32,
    pub color: Rgba,
    pub newest: bool,
}

/// 🖌️ One instanced draw: every visible record of one mesh index and verdict.
#[derive(Clone, Debug)]
pub struct ToolRunTraceDraw {
    pub mesh: u32,
    pub verdict: ToolRunVerdict,
    pub instances: Vec<Instance3d>,
}

/// 🟩️ The `provisional` style token for a document instance a running tool placed: the highlighted mesh tone at the
/// `toolRun.provisionalOpacity` token, so the placements a run has made read as highlighted objects while the success,
/// danger and testing verdict colours stay reserved for the ONE candidate under test. The wgpu theme has no `secondary`
/// tone (React's highlighted mesh paint), so its accent stands in. wgpu paints it static (no dash animation), which is
/// also what `prefers-reduced-motion` asks of the React target.
pub fn tool_run_provisional_color(theme: &Theme) -> [f32; 4] {
    [theme.accent.r, theme.accent.g, theme.accent.b, tool_run::PROVISIONAL_OPACITY as f32]
}
//#endregion 🔖️Paint

//#region Pool
pub struct RefCountPool<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    pub counts: HashMap<K, u32>,
}

impl<K> RefCountPool<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    pub fn new() -> Self {
        Self { counts: HashMap::new() }
    }

    pub fn acquire(&mut self, key: K) {
        *self.counts.entry(key).or_insert(0) += 1;
    }

    pub fn release(&mut self, key: &K) -> bool {
        match self.counts.get_mut(key) {
            Some(count) if *count > 1 => {
                *count -= 1;
                false
            }
            Some(_) => {
                self.counts.remove(key);
                true
            }
            None => false,
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.counts.contains_key(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.counts.keys()
    }
}
//#endregion Pool

pub(crate) fn default_lod_record() -> WorldLodRecord {
    WorldLodRecord {
        automatic: true,
        manual: default_manual_lod(),
        distance_reference: default_distance_reference(),
        depth_variable: false,
        grid_factor: default_grid_factor(),
        grid_snap_enabled: false,
        show_grid: true,
        grid_datum: Some([0.0, 0.0, 0.0]),
    }
}

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
pub struct WorldInteractionId {
    pub bytes: [u8; WORLD_INTERACTION_ID_BYTE_CAPACITY],
    pub len: u16,
}

impl WorldInteractionId {
    pub fn new(value: &str) -> Option<Self> {
        if value.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY {
            return None;
        }
        let mut bytes = [0; WORLD_INTERACTION_ID_BYTE_CAPACITY];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Some(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("world registry id originates from UTF-8")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldInteractionMeshToken {
    pub slot: u16,
    pub generation: u64,
}

#[derive(Clone, Copy)]
struct WorldInteractionMeshSlot {
    pub id: WorldInteractionId,
    pub generation: u64,
    pub version: u64,
    pub vertices: u32,
    pub triangles: u32,
    pub edges: u32,
    topology_bytes: u32,
}

pub struct WorldInteractionMeshRegistry {
    pub slots: Box<[Option<WorldInteractionMeshSlot>; WORLD_INTERACTION_MESH_CAPACITY]>,
    pub epochs: Box<[u64; WORLD_INTERACTION_MESH_CAPACITY]>,
    pub faulted: bool,
}

#[derive(Clone, Copy)]
struct WorldInteractionMeshPlan {
    pub slot: u16,
    pub epoch: u64,
    existing_generation: Option<u64>,
    pub value: WorldInteractionMeshSlot,
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
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: semio_framework_async::boxed_fixed_slots(|| 0), faulted: false }
    }
}

impl WorldInteractionMeshRegistry {
    pub fn hash(id: &str) -> usize {
        (id.as_bytes().iter().fold(2_166_136_261u64, |hash, byte| hash.wrapping_mul(16_777_619) ^ u64::from(*byte)) % WORLD_INTERACTION_MESH_CAPACITY as u64) as usize
    }

    pub fn plan_admit(&self, id: &str, version: u64, mesh: Mesh3dLease) -> Result<WorldInteractionMeshPlan, WorldDynamicFault> {
        let Some(id) = WorldInteractionId::new(id) else {
            return Err(WorldDynamicFault::IdCapacity);
        };
        self.plan_admit_id(id,version,mesh)
    }

    pub fn plan_admit_id(&self, id: WorldInteractionId, version: u64, mesh: Mesh3dLease) -> Result<WorldInteractionMeshPlan, WorldDynamicFault> {
        let Ok(schema) = mesh.schema() else {
            return Err(WorldDynamicFault::StaleToken);
        };
        let vertices = schema.vertices;
        let triangles = schema.indices / 3;
        let edges = if schema.edge_ids == schema.edges { schema.edges } else { 0 };
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

    pub fn commit_admit(&mut self, plan: WorldInteractionMeshPlan) -> Result<WorldInteractionMeshToken, WorldDynamicFault> {
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

    #[cfg(any(test, feature = "testkit"))]
    pub fn admit(&mut self, id: &str, version: u64, mesh: Mesh3dLease) -> Option<WorldInteractionMeshToken> {
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

    pub fn resolve(&self, token: WorldInteractionMeshToken) -> Option<&WorldInteractionMeshSlot> {
        let slot = self.slots.get(usize::from(token.slot))?.as_ref()?;
        (slot.generation == token.generation).then_some(slot)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionObjectKind {
    Instance,
    Vortex,
    Reference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldInteractionObjectToken {
    pub slot: u16,
    pub generation: u64,
    pub revision: u64,
}

#[derive(Clone, Copy)]
pub struct WorldInteractionObjectSlot {
    pub id: WorldInteractionId,
    pub kind: WorldInteractionObjectKind,
    pub generation: u64,
    pub revision: u64,
    pub mesh: Option<WorldInteractionMeshToken>,
    pub component_source: Option<ComponentSource3d>,
    pub model: Mat4,
    pub values: [f32; 8],
}

pub struct WorldInteractionObjectRegistry {
    pub slots: Box<[Option<WorldInteractionObjectSlot>; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub occupied: [u64; WORLD_INTERACTION_OBJECT_CAPACITY.div_ceil(64)],
    pub epochs: Box<[u64; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub instance_order: Box<[Option<WorldInteractionObjectToken>; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub instance_len: u16,
    pub revision: u64,
    pub faulted: bool,
}

impl Default for WorldInteractionObjectRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), occupied: [0; WORLD_INTERACTION_OBJECT_CAPACITY.div_ceil(64)], epochs: semio_framework_async::boxed_fixed_slots(|| 0), instance_order: semio_framework_async::boxed_fixed_slots(|| None), instance_len: 0, revision: u64::MAX, faulted: false }
    }
}

impl WorldInteractionObjectRegistry {
    pub fn hash(kind: WorldInteractionObjectKind, id: &str) -> usize {
        let seed = match kind {
            WorldInteractionObjectKind::Instance => 0x11u64,
            WorldInteractionObjectKind::Vortex => 0x22,
            WorldInteractionObjectKind::Reference => 0x33,
        };
        (id.as_bytes().iter().fold(2_166_136_261u64 ^ seed, |hash, byte| hash.wrapping_mul(16_777_619) ^ u64::from(*byte)) % WORLD_INTERACTION_OBJECT_CAPACITY as u64) as usize
    }

    /// 🪪️ Admits one object for one interaction revision, reusing the slot its own `(kind, id)`
    /// already holds — even from an EARLIER revision — before it takes a free one.
    ///
    /// 🩸️ A stale-revision slot of the same id only seeded `reusable` and the probe walked on to the
    /// next EMPTY slot, so re-admitting an id at a new revision left the previous revision's entry
    /// intact AND resolvable: `resolve` matches on generation and revision, both of which the old
    /// token still carried. Every revision therefore leaked a slot until the fixed table faulted, and
    /// a marquee cursor holding the previous token kept picking the object it had already replaced.
    pub fn admit(&mut self, revision: u64, kind: WorldInteractionObjectKind, id: &str, mesh: Option<WorldInteractionMeshToken>, model: Mat4, values: [f32; 8], component_source: Option<ComponentSource3d>) -> Option<WorldInteractionObjectToken> {
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
                    if existing.mesh == mesh && existing.component_source == component_source && existing.model.cols == model.cols && existing.values == values {
                        return Some(WorldInteractionObjectToken { slot: slot as u16, generation: existing.generation, revision });
                    }
                    reusable = Some(slot);
                    break;
                }
                Some(existing) if existing.revision != revision => {
                    if existing.kind == kind && existing.id == id {
                        reusable = Some(slot);
                        break;
                    }
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
        self.slots[slot] = Some(WorldInteractionObjectSlot { id, kind, generation, revision, mesh, component_source, model, values });
        self.occupied[slot / 64] |= 1u64 << (slot % 64);
        Some(WorldInteractionObjectToken { slot: slot as u16, generation, revision })
    }

    /// 🧭️ Finds the next occupied slot in at most sixteen bitmap words, preserving stable slot order.
    pub fn next_occupied(&self, start: u16) -> Option<(usize, &WorldInteractionObjectSlot)> {
        let mut word = usize::from(start) / 64;
        let mut bits = *self.occupied.get(word)? & (u64::MAX << (usize::from(start) % 64));
        loop {
            if bits != 0 {
                let slot = word * 64 + bits.trailing_zeros() as usize;
                return self.slots.get(slot)?.as_ref().map(|entry| (slot, entry));
            }
            word += 1;
            bits = *self.occupied.get(word)?;
        }
    }

    pub fn resolve(&self, token: WorldInteractionObjectToken) -> Option<&WorldInteractionObjectSlot> {
        let slot = self.slots.get(usize::from(token.slot))?.as_ref()?;
        (slot.generation == token.generation && slot.revision == token.revision).then_some(slot)
    }

    pub fn begin_instance_order(&mut self) {
        self.instance_len = 0;
    }

    pub fn push_instance_order(&mut self, token: WorldInteractionObjectToken) -> bool {
        let index = usize::from(self.instance_len);
        if index == WORLD_INTERACTION_OBJECT_CAPACITY {
            self.faulted = true;
            return false;
        }
        self.instance_order[index] = Some(token);
        self.instance_len += 1;
        true
    }

    pub fn terminal_for_revision(&self, revision: u64) -> bool {
        self.revision == revision && !self.faulted
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionRegistryBuildPhase {
    Instances,
    Vortices,
    References,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionPhase {
    PointerMove,
    PointerButton,
    PointerDrag,
    PointerLeave,
    Wheel,
    /// 🧯️ A host cancel of the gesture in flight (window blur, capture lost); its [`WorldCancelReason`] rides in `button`.
    Cancel,
    Close,
}

/// 🧯️ Why a host cancels a World3d gesture — the tool protocol's abort `reason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldCancelReason {
    Blur,
    CaptureLost,
}

impl WorldCancelReason {
    /// 🏷️ The abort `reason` the guest's tool machine parses (`ToolAbortReason::parse`).
    pub fn code(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::CaptureLost => "captureLost",
        }
    }

    pub fn from_button(button: i16) -> Self {
        if button == Self::CaptureLost as i16 {
            Self::CaptureLost
        } else {
            Self::Blur
        }
    }
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

    /// 🚪️ The DOM's `pointerout`: the pointer is no longer over THIS surface — it left its rect, or a
    /// chrome layer painted above it owns the point now. Carries no gesture, only the point it left
    /// at, and answers exactly one published hover with its clear. See
    /// [`plan_world3d_pointer_leave`].
    pub fn pointer_leave(x: f32, y: f32) -> Self {
        Self { phase: WorldInteractionPhase::PointerLeave, generation: 0, x, y, dx: 0.0, dy: 0.0, delta: 0.0, button: 0, down: false, shift: false, ctrl: false, alt: false, meta: false }
    }

    /// 🧯️ A host cancel (window blur, capture lost before the surface closes): a live gumball gesture that streamed aborts
    /// with zero trace and swallows the rest of its drag, an open paint stroke aborts; anything else is untouched.
    pub fn cancel(reason: WorldCancelReason) -> Self {
        Self { phase: WorldInteractionPhase::Cancel, generation: 0, x: 0.0, y: 0.0, dx: 0.0, dy: 0.0, delta: 0.0, button: reason as i16, down: false, shift: false, ctrl: false, alt: false, meta: false }
    }
}

pub struct WorldInteractionIntentQueue {
    pub slots: Box<[Option<WorldInteractionIntent>; WORLD_INTERACTION_INTENT_CAPACITY]>,
    head: u8,
    pub len: u8,
    pub closing: bool,
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

    /// 🚪️ Whether a [`WorldInteractionPhase::PointerLeave`] is already waiting to be answered.
    pub fn holds_pointer_leave(&self) -> bool {
        self.slots.iter().flatten().any(|intent| intent.phase == WorldInteractionPhase::PointerLeave)
    }

    /// 🫧 Folds a fresh pointer MOVE onto the one already waiting at the tail, answering the
    /// generation it keeps — the retained twin of React's own rAF batching.
    ///
    /// React never sees a move per DOM event either: `🚚️browser-frame-transport/🟦️.ts`'s
    /// `enqueueReplaceable` keeps ONE sample per pointer identity per animation frame, and
    /// `🌐️World3dHost/🟦️.tsx`'s hover dispatcher "keeps at most one outstanding and coalesces the
    /// rest onto the latest target". This queue had no such rung: every move the host drained became
    /// its own intent, so the probe's eight-move drag left `queued=10` in front of the authority and
    /// every one of them was answered with a full pick/plan turn
    /// (`🗑️generated/w12c-parity-run-19/wgpu/console.txt`, ticket 26/09/17 packet W13c §3).
    ///
    /// 🧭️ The fold is EXACT for gesture classification, which is the one thing it may not move:
    /// `dx`/`dy` accumulate (a drag plan reads the delta, so eight folded moves steer the camera by
    /// the same total as eight separate ones), `x`/`y` take the newest sample (a pick reads the
    /// point, and the newest point is the one React would have raycast), and the `down`/button/
    /// modifier state must MATCH — a change in any of them is a different gesture and starts a new
    /// intent. The FRONT is never folded into: it may already be half-stepped by a live cursor.
    pub fn coalesce_tail_pointer_move(&mut self, intent: &WorldInteractionIntent) -> Option<u64> {
        if self.closing || intent.phase != WorldInteractionPhase::PointerMove || self.len < 2 {
            return None;
        }
        let index = (usize::from(self.head) + usize::from(self.len) - 1) % WORLD_INTERACTION_INTENT_CAPACITY;
        let tail = self.slots[index].as_mut()?;
        if tail.phase != WorldInteractionPhase::PointerMove || tail.down != intent.down || tail.button != intent.button || tail.shift != intent.shift || tail.ctrl != intent.ctrl || tail.alt != intent.alt || tail.meta != intent.meta {
            return None;
        }
        tail.x = intent.x;
        tail.y = intent.y;
        tail.dx += intent.dx;
        tail.dy += intent.dy;
        Some(tail.generation)
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
pub enum WorldFlatActionKind {
    PaintAt,
    PaintCommit,
    /// 🧯️ `paintAt{phase:"abort", reason}` — a host cancel drops the open paint stroke with zero trace; `numbers[0]` is
    /// the [`WorldCancelReason`].
    PaintAbort,
    VortexSelect,
    VortexHover,
    SurfacePlace,
    Select,
    Hover,
    Camera,
    /// 🧭️ A navigation step that moves this pane's orbit and publishes NOTHING — see
    /// [`WorldCameraSync`]. The wire report is one debounced [`WorldFlatActionKind::Camera`] at the
    /// settle, never one per pointer move.
    CameraLocal,
    /// 🧭️ `noteWorldNavigation` — the classified gesture set of a completed navigation, published
    /// ahead of the settle's `setCamera` exactly as React's un-debounced `onNavigationGestures` lands
    /// ahead of its debounced `reportCamera`.
    Navigation,
    /// 🚚️ `worldRelocate` — the ONE absolute `{objectId, position}` a finished relocate drag commits.
    Relocate,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldInteractionSpan {
    pub start: u16,
    pub len: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldFlatAction {
    pub kind: WorldFlatActionKind,
    pub strings: [Option<WorldInteractionSpan>; 8],
    pub numbers: [f64; 10],
}

pub struct WorldInteractionPlan {
    pub revision: u64,
    pub generation: u64,
    pub bytes: Box<[u8; WORLD_INTERACTION_BYTE_CAPACITY]>,
    pub byte_len: u16,
    pub actions: Box<[Option<WorldFlatAction>; WORLD_INTERACTION_ITEM_CAPACITY]>,
    pub action_len: u16,
    pub cursor: u16,
    pub faulted: bool,
}

pub const TRANSLATE_SELECTION_ACTION_ID: &str = "translateSelection";

pub const ROTATE_SELECTION_ACTION_ID: &str = "rotateSelection";

pub const SCALE_SELECTION_ACTION_ID: &str = "scaleSelection";

impl WorldInteractionPlan {
    pub fn new(revision: u64, generation: u64) -> Self {
        Self { revision, generation, bytes: Box::new([0; WORLD_INTERACTION_BYTE_CAPACITY]), byte_len: 0, actions: Box::new([None; WORLD_INTERACTION_ITEM_CAPACITY]), action_len: 0, cursor: 0, faulted: false }
    }

    pub fn push_string(&mut self, value: &str) -> Option<WorldInteractionSpan> {
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

    pub fn push_action(&mut self, action: WorldFlatAction) -> bool {
        let index = usize::from(self.action_len);
        if index == WORLD_INTERACTION_ITEM_CAPACITY {
            self.faulted = true;
            return false;
        }
        self.actions[index] = Some(action);
        self.action_len += 1;
        true
    }

    pub fn string(&self, span: WorldInteractionSpan) -> &str {
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
