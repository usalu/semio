//! 🎒️ World3d prepared frame resources, build context and the cursor wake authority.

use super::*;

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
