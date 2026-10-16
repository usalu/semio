//! 🧠️ Session core: the shell allocator, `Session`, its capture and retirement state, and the geometry port.

use super::*;

/// 🌉️ Dispatches one `BrepKernel` method by name over `os_pack::json` args (see `handle_result`
/// and friends above for the response shapes); the sole bridge every `SemioBrepKernel` TS method
/// (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts`) calls into. Every arm is declared, argument for
/// argument, by the verb catalog `🔣️.json` beside this file (schema `🧬️schema/🔣️.json`); the bridge law
/// `🌊️flow/🧪️tests/📐️brep-invoke` holds the two to each other.

/// 🌐️ `brep_invoke` implementation shared by the wasm export and native callers/tests: returns
/// the result JSON or `{"error": "..."}` — never panics on malformed input.
/// 🧾️ Records real heap layouts; cloned handles clear the synchronous consuming-call receipt.
#[derive(Default)]
pub(super) struct ShellAllocator {
    layout_bytes: std::sync::atomic::AtomicUsize,
    pub(super) receipt: std::sync::atomic::AtomicPtr<std::sync::atomic::AtomicUsize>,
}
impl Clone for ShellAllocator {
    fn clone(&self) -> Self { Self { layout_bytes:std::sync::atomic::AtomicUsize::new(self.layout_bytes.load(std::sync::atomic::Ordering::Relaxed)),receipt:std::sync::atomic::AtomicPtr::default() } }
}
unsafe impl std::alloc::Allocator for ShellAllocator {
    fn allocate(&self,layout:std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>,std::alloc::AllocError> {
        let allocation = std::alloc::Allocator::allocate(&std::alloc::Global,layout)?;
        self.layout_bytes.store(layout.size(),std::sync::atomic::Ordering::Relaxed);
        Ok(allocation)
    }
    unsafe fn deallocate(&self,pointer:std::ptr::NonNull<u8>,layout:std::alloc::Layout) {
        unsafe { std::alloc::Allocator::deallocate(&std::alloc::Global,pointer,layout); }
        let receipt = self.receipt.load(std::sync::atomic::Ordering::Relaxed);
        if !receipt.is_null() { unsafe { (*receipt).fetch_add(layout.size(),std::sync::atomic::Ordering::Relaxed); } }
    }
}
pub(super) type ShellArc<T> = Arc<T,ShellAllocator>;
fn shell<T>(value:T) -> ShellArc<T> { Arc::new_in(value,ShellAllocator::default()) }
pub(super) fn shell_bytes<T>(owner:&ShellArc<T>) -> usize { Arc::allocator(owner).layout_bytes.load(std::sync::atomic::Ordering::Relaxed) }
pub(super) fn record_shell<T>(owner:&ShellArc<T>,receipt:&std::sync::atomic::AtomicUsize) { Arc::allocator(owner).receipt.store(std::ptr::from_ref(receipt).cast_mut(),std::sync::atomic::Ordering::Relaxed); }

#[derive(Clone)]
pub struct Session { pub(super) state: ShellArc<SessionState> }

/// 🎟️ Owns one exact authority reader until reference release or real final-family retirement.
#[must_use = "session captures require explicit retirement"]
pub struct SessionCapture {
    pub(super) session:ManuallyDrop<Option<Session>>,
    pub(super) orphan:ManuallyDrop<Option<SessionState>>,
    pub(super) allocation:ManuallyDrop<Option<Weak<SessionState,ShellAllocator>>>,
    allocation_bytes:usize,
}
impl std::ops::Deref for SessionCapture {
    type Target=Session;
    fn deref(&self)->&Session {self.session.as_ref().expect("open geometry capture")}
}
impl SessionCapture {
    /// 🪺️ Takes sole ownership of a fresh session, as the browser module's `BrowserSession` does.
    pub fn owned(session:Session)->Self {Self {session:ManuallyDrop::new(Some(session)),orphan:ManuallyDrop::new(None),allocation:ManuallyDrop::new(None),allocation_bytes:0}}
    /// 🔍️ The open session, or `None` once close has released it.
    pub fn session(&self)->Option<&Session> {self.session.as_ref()}
    pub fn terminal_is_empty(&self)->bool {self.session.is_none() && self.orphan.is_none() && self.allocation.is_none()}
    fn retained_state(&self)->Option<&SessionState> {self.session.as_ref().map(|session|&*session.state).or_else(||self.orphan.as_ref())}
    pub fn begin_close(&self) {if let Some(state)=self.retained_state() {state.begin_close();}}
    pub fn cancel_close(&self) {if let Some(state)=self.retained_state() {state.retirement.try_lock().expect("geometry retirement").paused=true;}}
    pub fn resume_close(&self) {if let Some(state)=self.retained_state() {state.retirement.try_lock().expect("geometry retirement").paused=false;}}

    pub fn is_closed(&self)->bool {self.session.as_ref().map_or_else(||self.orphan.as_ref().is_none_or(|state|state.closed.load(std::sync::atomic::Ordering::Acquire)),Session::is_closed)}
    pub fn native_terminal_is_empty(&self)->bool {self.session.as_ref().map_or_else(||self.orphan.as_ref().is_none_or(SessionState::terminal_is_empty),Session::terminal_is_empty)}
    pub fn shell_byte_requirement(&self)->usize {
        if let Some(state)=self.orphan.as_ref() {shell_bytes(&state.kernel)+shell_bytes(&state.mesh_cache)+shell_bytes(&state.claims)+shell_bytes(&state.next_authority)+shell_bytes(&state.retirement)} else if self.allocation.is_some() {self.allocation_bytes} else {0}
    }
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {match self.orphan.as_ref() {Some(state) if !state.terminal_is_empty()=>state.next_copy_byte_demand(),_=>Ok(0)}}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {match self.orphan.as_ref() {Some(state) if !state.terminal_is_empty()=>state.next_capacity_byte_demand(copy),_=>Ok(0)}}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {match self.orphan.as_ref() {Some(state) if !state.terminal_is_empty()=>state.next_release_byte_demand(),_=>Ok(self.shell_byte_requirement())}}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError> {match self.orphan.as_ref() {Some(state) if !state.terminal_is_empty()=>state.next_depth_demand(),_=>Ok(usize::from(!self.terminal_is_empty()))}}
    /// ⚓️ Pins the actual allocation while the final original state transfers inline without rebirth.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
        let state=self.session.as_ref().map(|session|&*session.state).or_else(||self.orphan.as_ref());
        if state.is_some_and(|state|state.retirement.try_lock().map(|owner|owner.paused).unwrap_or(true)) {return Ok(ownership_progress());}
        if let Some(session)=self.session.as_ref() {
            let allocation=Arc::downgrade(&session.state);
            self.allocation_bytes=shell_bytes(&session.state);
            *self.allocation=Some(allocation);
            let Session {state}=self.session.take().unwrap();
            *self.orphan=Arc::into_inner(state);
            return Ok(ownership_advance());
        }
        if let Some(state)=self.orphan.as_ref() {
            if !state.terminal_is_empty() {return state.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
            if grant.maximum_release_bytes<self.shell_byte_requirement() {return Ok(ownership_progress());}
            let receipt=std::sync::atomic::AtomicUsize::new(0);
            record_shell(&state.kernel,&receipt);record_shell(&state.mesh_cache,&receipt);record_shell(&state.claims,&receipt);record_shell(&state.next_authority,&receipt);record_shell(&state.retirement,&receipt);
            drop(self.orphan.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed),..Default::default()}));
        }
        if grant.maximum_release_bytes<self.allocation_bytes {return Ok(ownership_progress());}
        let receipt=std::sync::atomic::AtomicUsize::new(0);
        let allocation=self.allocation.take().unwrap();
        Weak::allocator(&allocation).receipt.store(std::ptr::from_ref(&receipt).cast_mut(),std::sync::atomic::Ordering::Relaxed);
        drop(allocation);
        self.allocation_bytes=0;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed),..Default::default()}))
    }
    /// 🧊️ Grants each current original currency exactly at an explicit cold boundary.
    pub fn retire_cold(&mut self) {
        while !self.terminal_is_empty() {
            let copy=self.next_close_copy_byte_demand().expect("cold capture copy demand");
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_close_capacity_byte_demand(copy).expect("cold capture capacity demand"),maximum_release_bytes:self.next_close_release_byte_demand().expect("cold capture release demand"),maximum_depth:self.next_close_depth_demand().expect("cold capture depth demand")};
            let step=self.close_step(grant).expect("cold geometry capture retirement");
            assert!(step.progress()!=RetainedCloneProgress::default() || self.terminal_is_empty(),"cold geometry capture waits on an original owner");
        }
    }
}

impl Drop for SessionCapture {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(),"geometry capture requires explicit retirement before drop"); } }
}

/// 🔌️ Owner-supplied geometry operations; the retained session owns all authority and retirement.
pub trait GeometryOperations: Send + Sync {
    fn export(&self, _: &Brep, format: &str, _: &[GeometryHandle], _: f64) -> Result<(String,bool),BrepModuleError> { Err(BrepModuleError::UnsupportedExportFormat(format.into())) }
    fn import(&self, _: &mut Brep, format: &str, _: &str, _: f64) -> Result<Vec<GeometryHandle>,BrepModuleError> { Err(BrepModuleError::UnsupportedImportFormat(format.into())) }
    fn invoke(&self, _: &mut Brep, method: &str, _: &semio_framework_pack_json::Value) -> Result<semio_framework_pack_json::Value,BrepModuleError> { Err(BrepModuleError::UnknownMethod(method.into())) }
}
impl GeometryOperations for () {}

pub(super) struct SessionState {
    pub(super) operations:&'static dyn GeometryOperations,
    pub(super) kernel:ShellArc<RetainedOwnerGate<ManuallyDrop<Brep>>>,
    pub(super) mesh_cache:ShellArc<RetainedOwnerGate<ManuallyDrop<MeshCache>>>,
    pub(super) claims:ShellArc<RetainedOwnerGate<ManuallyDrop<FamilyClaims>>>,
    pub(super) next_authority:ShellArc<std::sync::atomic::AtomicU64>,
    pub(super) authority:u64,
    pub(super) closed:std::sync::atomic::AtomicBool,
    pub(super) jobs:RetainedOwnerGate<ControlledRetirement<TessellationJobRegistry>>,
    pub(super) retirement:ShellArc<RetainedOwnerGate<SessionRetirement>>,
}
pub(super) struct SessionRetirement {pub(super) family:Option<FamilyRetirement>,pub(super) active:usize,paused:bool}
impl Default for SessionRetirement {fn default()->Self {Self {family:None,active:1,paused:false}}}
/// 📏️ The retained extent of one mesh's payload vectors — the dominant allocation a tessellation turn hands out.
pub fn mesh_payload_bytes(mesh:&semio_framework::MeshData)->usize {
    let floats=mesh.positions.capacity()+mesh.normals.capacity()+mesh.colors.capacity()+mesh.uvs.capacity()+mesh.edge_positions.capacity()+mesh.edge_uvs.capacity();
    let words=mesh.indices.capacity()+mesh.face_ids.capacity()+mesh.vertex_ids.capacity()+mesh.edge_ids.capacity();
    (floats+words).saturating_mul(size_of::<u32>()).saturating_add(mesh.edge_is_seam.capacity())
}
pub(super) fn ownership_busy()->ValueError {ValueError::literal(ValueRefusalKind::WorkLimit,"geometry owner is busy")}
pub(super) fn ownership_progress()->RetainedCloneStep {RetainedCloneStep::Progress(RetainedCloneProgress::default())}
pub(super) fn ownership_advance()->RetainedCloneStep {RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}
pub(super) fn controlled<T:semio_framework_value::retirement::RetireOwned>(value:T)->ControlledRetirement<T> {ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original geometry owner has no retirement authority"))}
impl SessionState {
    pub(super) fn begin_close(&self) {
        let mut claims=self.claims.try_lock().expect("geometry claims owner");
        let mut retirement=self.retirement.try_lock().expect("geometry retirement owner");
        if self.closed.swap(true,std::sync::atomic::Ordering::AcqRel) {return;}
        if let Some(claim)=claims.get_mut(&self.authority) {claim.active=false;retirement.active-=1;self.next_authority.fetch_add(1,std::sync::atomic::Ordering::AcqRel);}
    }
    pub(super) fn terminal_is_empty(&self)->bool {
        let Ok(jobs)=self.jobs.try_lock() else {return false};
        let Ok(retirement)=self.retirement.try_lock() else {return false};
        self.closed.load(std::sync::atomic::Ordering::Acquire) && jobs.terminal_is_empty() && (retirement.active!=0 || Arc::strong_count(&self.retirement)>1 || retirement.family.as_ref().is_some_and(ControlledRetirement::terminal_is_empty))
    }
    pub(super) fn close_step(&self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
        if self.retirement.try_lock().map_err(|_|ownership_busy())?.paused {return Ok(ownership_progress());}
        self.begin_close();
        let mut jobs=self.jobs.try_lock().map_err(|_|ownership_busy())?;
        if !jobs.terminal_is_empty() {return jobs.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        drop(jobs);
        let mut retirement=self.retirement.try_lock().map_err(|_|ownership_busy())?;
        if retirement.active!=0 || Arc::strong_count(&self.retirement)>1 {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if retirement.family.is_none() {
            let mut kernel=self.kernel.try_lock().map_err(|_|ownership_busy())?;
            let mut cache=self.mesh_cache.try_lock().map_err(|_|ownership_busy())?;
            let mut claims=self.claims.try_lock().map_err(|_|ownership_busy())?;
            let original=(std::mem::replace(&mut **kernel,Brep::new()),std::mem::take(&mut **cache),std::mem::take(&mut **claims));
            retirement.family=Some(controlled(original));
            return Ok(ownership_advance());
        }
        let family=retirement.family.as_mut().unwrap();
        if family.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        family.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))
    }
    pub(super) fn demand<R>(&self,query:impl FnOnce(&dyn semio_framework_value::ErasedSnapshotRetirement)->Result<R,ValueError>,transition:R)->Result<R,ValueError> {
        let jobs=self.jobs.try_lock().map_err(|_|ownership_busy())?;
        if !jobs.terminal_is_empty() {return query(&*jobs);}
        let retirement=self.retirement.try_lock().map_err(|_|ownership_busy())?;
        match retirement.family.as_ref() {Some(family) if retirement.active==0=>query(family),_=>Ok(transition)}
    }
    pub(super) fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.demand(|owner|owner.next_copy_byte_demand(),0)}
    pub(super) fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.demand(|owner|owner.next_capacity_byte_demand(copy),0)}
    pub(super) fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.demand(|owner|owner.next_release_byte_demand(),0)}
    pub(super) fn next_depth_demand(&self)->Result<usize,ValueError> {self.demand(|owner|owner.next_depth_demand(),usize::from(!self.terminal_is_empty()))}
}
impl Drop for SessionState {fn drop(&mut self) {if !std::thread::panicking() {assert!(self.closed.load(std::sync::atomic::Ordering::Acquire),"geometry authority requires explicit close before drop");assert!(self.jobs.get_mut().terminal_is_empty(),"geometry jobs require terminal-empty retirement");}}}
impl Drop for SessionRetirement {fn drop(&mut self) {if !std::thread::panicking() {assert!(self.active==0 && self.family.as_ref().is_some_and(ControlledRetirement::terminal_is_empty),"geometry family requires terminal-empty retirement");}}}
impl Default for Session { fn default() -> Self { Self::new() } }
impl Session {
    pub(super) fn claims_changed(&self){self.state.next_authority.fetch_add(1,std::sync::atomic::Ordering::AcqRel);}
    /// 🔗️ Captures this exact authority for a supplied operator or retained composition reader.
    pub fn capture(&self) -> SessionCapture { SessionCapture::owned(self.clone()) }
    /// 🔌️ Supplies one separately admitted retained-job authority over this kernel family.
    pub fn port(&self) -> Box<dyn semio_framework_os_flow::geometry::GeometryPort> {Box::new(self.admitted_port())}
    pub(super) fn admitted_port(&self)->SessionPort{
        let mut claims = self.state.claims.try_lock().expect("geometry claims");
        let authority = self.state.next_authority.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let closed = self.is_closed();
        if !closed {claims.insert(authority,AuthorityClaims::open());self.state.retirement.try_lock().expect("geometry retirement").active+=1;}
        let authority = Session { state: shell(SessionState {
            operations:self.state.operations, kernel: self.state.kernel.clone(), mesh_cache: self.state.mesh_cache.clone(),
            claims: self.state.claims.clone(), next_authority: self.state.next_authority.clone(), authority,
            closed: std::sync::atomic::AtomicBool::new(closed), jobs:RetainedOwnerGate::new(controlled(TessellationJobRegistry::default())), retirement:self.state.retirement.clone(),
        }) };
        SessionPort {authority:SessionCapture::owned(authority),anchor:self.capture(),retaining:RetainedOwnerGate::new(None),retain_receipt:Default::default(),cancel_receipt:Default::default()}
    }
    /// 🚪️ Seals this original authority without extracting claims, jobs or family resources.
    pub fn begin_close(&self) {self.state.begin_close();}
    pub fn cancel_close(&self) {self.state.retirement.try_lock().expect("geometry retirement").paused=true;}
    pub fn resume_close(&self) {self.state.retirement.try_lock().expect("geometry retirement").paused=false;}
    /// 🎟️ Forwards all independent currencies to the actual retained authority.
    pub fn close_step(&self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {self.state.close_step(grant)}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {self.state.next_copy_byte_demand()}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.state.next_capacity_byte_demand(copy)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {self.state.next_release_byte_demand()}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError> {self.state.next_depth_demand()}
    pub fn terminal_is_empty(&self)->bool {self.state.terminal_is_empty() && self.state.retirement.try_lock().is_ok_and(|owner|owner.active==0 && owner.family.as_ref().is_some_and(ControlledRetirement::terminal_is_empty))}
    /// 🧊️ Explicit cold callers admit the next exact original demands independently.
    pub fn close(&self) {
        self.begin_close();
        while !self.terminal_is_empty() {
            let copy=self.next_close_copy_byte_demand().expect("cold geometry copy demand");
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_close_capacity_byte_demand(copy).expect("cold geometry capacity demand"),maximum_release_bytes:self.next_close_release_byte_demand().expect("cold geometry release demand"),maximum_depth:self.next_close_depth_demand().expect("cold geometry depth demand")};
            let progress=self.close_step(grant).expect("cold geometry retirement").progress();
            if progress==RetainedCloneProgress::default() {return;}
        }
    }
    pub fn is_closed(&self) -> bool { self.state.closed.load(std::sync::atomic::Ordering::Acquire) }
    pub fn new() -> Self { Self::with_operations(&()) }
    /// 🔌️ Admits an explicit static owner operation table without retaining extra payload state.
    pub fn with_operations(operations: &'static dyn GeometryOperations) -> Self {
        Self { state: shell(SessionState { operations, kernel:shell(RetainedOwnerGate::new(ManuallyDrop::new(Brep::new()))), mesh_cache:shell(RetainedOwnerGate::new(ManuallyDrop::new(MeshCache::new()))), claims:shell(RetainedOwnerGate::new(ManuallyDrop::new(FamilyClaims::from_iter([(1,AuthorityClaims::open())])))), next_authority: shell(std::sync::atomic::AtomicU64::new(2)), authority:1,closed:std::sync::atomic::AtomicBool::new(false),jobs:RetainedOwnerGate::new(controlled(TessellationJobRegistry::default())),retirement:shell(RetainedOwnerGate::new(SessionRetirement::default())) }) }
    }
pub(super) fn kernel(&self) -> &RetainedOwnerGate<ManuallyDrop<Brep>> { &self.state.kernel }
pub(super) fn mesh_cache(&self) -> &RetainedOwnerGate<ManuallyDrop<MeshCache>> { &self.state.mesh_cache }
pub fn evict_mesh_cache_for_handles(&self,handles:&[String]) {
    if self.is_closed() {return;}
    if let Ok(mut cache)=self.mesh_cache().try_lock() {for (key,cached) in cache.slot_entries_mut() {if !handles.iter().any(|handle|handle==&key.0) {cached.valid=false;}}}
}
pub fn evict_mesh_cache_for_handle(&self,handle:&str) {
    if self.is_closed() {return;}
    if let Ok(mut cache)=self.mesh_cache().try_lock() {for (key,cached) in cache.slot_entries_mut() {if key.0==handle {cached.valid=false;}}}
}
pub fn with_kernel<T>(&self, f: impl FnOnce(&mut Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut claims = self.state.claims.try_lock().map_err(|_| EvalError::InvalidInput("geometry claims owner busy".into()))?;
    let mut guard = self.kernel().try_lock().map_err(|_| EvalError::InvalidInput("brep kernel owner busy".into()))?;
    if self.is_closed() { return Err(EvalError::InvalidInput("geometry.session-closed".into())); }
    let before = guard.live_handles();
    let result = f(&mut guard);
    claims.entry(self.state.authority).or_default().extend(guard.live_handles().difference(&before).cloned());self.claims_changed();
    result
}
/// 🎟️ Drives the genuine retained Boolean job under the same original context without whole registry snapshots.
pub fn step_boolean_job(&self,job:&mut semio_framework_3d_brep_engine::BrepBooleanJob,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_3d_brep_engine::BrepBooleanStep,semio_framework_job::OriginalTurnFailure>{
    if self.is_closed(){return Err(ownership_busy().into())}
    let mut kernel=self.kernel().try_lock().map_err(|_|ownership_busy())?;kernel.step_boolean_job_sync(job,cx)
}
pub fn with_kernel_read<T>(&self, f: impl FnOnce(&Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let guard = self.kernel().try_lock().map_err(|_| EvalError::InvalidInput("brep kernel owner busy".into()))?;
    if self.is_closed() { return Err(EvalError::InvalidInput("geometry.session-closed".into())); }
    f(&guard)
}
/// 📦 Advances an owned mesh import and admits exactly its final published handle.
pub fn step_mesh_import(&self,cursor:&mut semio_framework_3d_brep_engine::MeshImportCursor,budget:usize)->Result<Option<GeometryHandle>,EvalError> {
    if self.is_closed() {return Err(EvalError::InvalidInput("geometry.session-closed".into()));}
    let mut claims=self.state.claims.try_lock().map_err(|_|EvalError::InvalidInput("geometry claims owner busy".into()))?;let mut guard=self.kernel().try_lock().map_err(|_|EvalError::InvalidInput("brep kernel owner busy".into()))?;
    let result=guard.step_mesh_import_sync(cursor,budget).map_err(|error|map_kernel_error(&error))?;
    if let Some(handle)=&result {claims.entry(self.state.authority).or_default().insert(handle.as_str().to_string());self.claims_changed();}Ok(result)
}
pub fn close_mesh_import(&self,cursor:&mut semio_framework_3d_brep_engine::MeshImportCursor,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
    let mut guard=self.kernel().try_lock().map_err(|_|ownership_busy())?;guard.close_mesh_import_sync(cursor,grant)
}
pub fn next_mesh_import_copy_byte_demand(&self,cursor:&semio_framework_3d_brep_engine::MeshImportCursor)->Result<usize,ValueError> {let guard=self.kernel().try_lock().map_err(|_|ownership_busy())?;cursor.next_close_copy_byte_demand(guard.tessellation_body())}
pub fn next_mesh_import_capacity_byte_demand(&self,cursor:&semio_framework_3d_brep_engine::MeshImportCursor,copy:usize)->Result<usize,ValueError> {let guard=self.kernel().try_lock().map_err(|_|ownership_busy())?;cursor.next_close_capacity_byte_demand(guard.tessellation_body(),copy)}
pub fn next_mesh_import_release_byte_demand(&self,cursor:&semio_framework_3d_brep_engine::MeshImportCursor)->Result<usize,ValueError> {let guard=self.kernel().try_lock().map_err(|_|ownership_busy())?;cursor.next_close_release_byte_demand(guard.tessellation_body())}
pub fn next_mesh_import_depth_demand(&self,cursor:&semio_framework_3d_brep_engine::MeshImportCursor)->Result<usize,ValueError> {let guard=self.kernel().try_lock().map_err(|_|ownership_busy())?;cursor.next_close_depth_demand(guard.tessellation_body())}

pub fn retain_geometry_handles(&self, live: &[String]) {
    let mut claims = self.state.claims.try_lock().expect("geometry claims");
    if self.is_closed() { return; }
    claims.entry(self.state.authority).or_default().retain(live);self.claims_changed();
    let merged = claims.values().flat_map(|handles| handles.iter().cloned()).collect::<HashSet<_>>();
    if let Ok(mut guard) = self.kernel().try_lock() { guard.retain(&merged); }
    self.evict_mesh_cache_for_handles(&merged.into_iter().collect::<Vec<_>>());
    self.retain_tessellation_jobs(live);
}
pub(super) fn tessellation_jobs(&self)->&RetainedOwnerGate<ControlledRetirement<TessellationJobRegistry>> {&self.state.jobs}
pub fn retain_tessellation_jobs(&self,live:&[String]) {
    if self.is_closed() {return;}
    let Ok(mut owner)=self.tessellation_jobs().try_lock() else {return};
    let Some(registry)=owner.original_mut() else {return};
    for (key,retained) in registry.jobs.slot_entries_mut() {if !retained.retired && !live.iter().any(|handle|handle==&key.0) {retained.job.cancel();retained.retired=true;}}
    registry.retire_slot=0;
}
pub fn cancel_tessellation(&self,handle:&str,tolerance:f64)->bool {
    let Ok(mut owner)=self.tessellation_jobs().try_lock() else {return false};
    let Some(registry)=owner.original_mut() else {return false};
    for (key,retained) in registry.jobs.slot_entries_mut() {if key.0==handle && key.1==tolerance.to_bits() && !retained.retired {retained.job.cancel();retained.retired=true;registry.retire_slot=0;return true;}}
    false
}
pub fn cancel_all_tessellations(&self)->usize {
    let Ok(mut owner)=self.tessellation_jobs().try_lock() else {return 0};
    let Some(registry)=owner.original_mut() else {return 0};
    let mut count=0;
    for retained in registry.jobs.slot_values_mut() {if !retained.retired {retained.job.cancel();retained.retired=true;count+=1;}}
    registry.retire_slot=0;count
}
/// 🛑️ Flags one original authority for cancellation without scanning, extracting or releasing any job.
pub fn begin_cancel(&self)->Result<(),ValueError>{
    let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original_mut().ok_or_else(ownership_busy)?;
    if registry.cancellation.is_none(){registry.cancellation=Some(TessellationCancellation {slot:0});}Ok(())
}
pub fn cancel_terminal_is_empty(&self)->bool{self.tessellation_jobs().try_lock().is_ok_and(|owner|owner.original().is_some_and(|registry|registry.cancellation.is_none()&&registry.closing.is_none()&&registry.admission_closing.is_none())||owner.terminal_is_empty())}
fn cancellation_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
    let owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original().ok_or_else(ownership_busy)?;
    Self::cancellation_registry_demands(registry,copy)
}
fn cancellation_registry_demands(registry:&TessellationJobRegistry,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
    if let Some(closing)=&registry.admission_closing{return closing.selected_demand(copy);}
    if let Some(closing)=&registry.closing{return closing.selected_demand(copy);}
    let Some(cancellation)=&registry.cancellation else{return Ok(Default::default());};
    if registry.admission.is_some(){return Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()});}
    Ok(semio_framework_value::RetirementDemand {depth:registry.jobs.next_extract_slot_depth_demand(cancellation.slot)?.max(1),..Default::default()})
}
pub fn next_cancel_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.cancellation_demands(0)?.copy_bytes)}
pub fn next_cancel_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.cancellation_demands(copy)?.capacity_bytes)}
pub fn next_cancel_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.cancellation_demands(0)?.release_bytes)}
pub fn next_cancel_depth_demand(&self)->Result<usize,ValueError>{Ok(self.cancellation_demands(0)?.depth)}
/// 🎟️ Cancels and transfers one original registry row, then closes its exact typed payload before another row.
pub fn cancel_step(&self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<RetainedCloneStep,ValueError>{
    *receipt=Default::default();let result=self.cancel_step_retained(grant,receipt);if let Ok(step)=&result{*receipt=step.progress();}result
}
fn cancel_step_retained(&self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<RetainedCloneStep,ValueError>{
    let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original_mut().ok_or_else(ownership_busy)?;
    if let Some(closing)=&mut registry.admission_closing{let result=closing.step(grant);*receipt=closing.step_progress();let step=result?;if closing.terminal_is_empty(){registry.admission_closing=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
    if let Some(closing)=&mut registry.closing{let result=closing.step(grant);*receipt=closing.step_progress();let step=result?;if closing.terminal_is_empty(){registry.closing=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
    let demand=Self::cancellation_registry_demands(registry,grant.maximum_copy_bytes)?;
    if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(ownership_progress());}
    let Some(cancellation)=&mut registry.cancellation else{return Ok(RetainedCloneStep::Complete(Default::default()));};
    if let Some(mut original)=registry.admission.take(){original.cancelled=true;match ControlledRetirement::new(original){Ok(closing)=>registry.admission_closing=Some(closing),Err((error,original))=>{registry.admission=Some(original);return Err(error);}}return Ok(ownership_advance());}
    if cancellation.slot>=registry.jobs.slot_count(){registry.cancellation=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..Default::default()}));}
    let slot=cancellation.slot;cancellation.slot+=1;
    if let Some((_,retained))=registry.jobs.slot_entry_mut(slot){retained.job.cancel();retained.retired=true;}
    if let Some(original)=registry.jobs.extract_slot_if(slot,|_,_|false){registry.closing=Some(controlled(original));}
    Ok(ownership_advance())
}
fn retire_jobs_for_handle(&self,handle:&str) {
    let Ok(mut owner)=self.tessellation_jobs().try_lock() else {return};
    let Some(registry)=owner.original_mut() else {return};
    for (key,retained) in registry.jobs.slot_entries_mut() {if key.0==handle && !retained.retired {retained.job.cancel();retained.retired=true;}}
    registry.retire_slot=0;
}
/// 🎟️ Transfers one original cancelled row inline, then pays its exact typed retirement demands.
pub fn close_retired_tessellation_step(&self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
    let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
    let Some(registry)=owner.original_mut() else {return Err(ownership_busy())};
    if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
    if let Some(closing)=registry.closing.as_mut() {
        if closing.terminal_is_empty() {registry.closing=None;return Ok(ownership_advance());}
        return closing.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));
    }
    if registry.retire_slot>=registry.jobs.slot_count() {return Ok(RetainedCloneStep::Complete(Default::default()));}
    let slot=registry.retire_slot;registry.retire_slot+=1;
    if let Some(original)=registry.jobs.extract_slot_if(slot,|_,row|!row.retired) {registry.closing=Some(controlled(original));}
    Ok(ownership_advance())
}
fn retired_tessellation_demand<R>(&self,query:impl FnOnce(&dyn semio_framework_value::ErasedSnapshotRetirement)->Result<R,ValueError>,scan:R)->Result<R,ValueError> {
    let owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
    let Some(registry)=owner.original() else {return Err(ownership_busy())};
    match registry.closing.as_ref() {Some(closing)=>query(closing),None=>Ok(scan)}
}
pub fn next_retired_tessellation_copy_byte_demand(&self)->Result<usize,ValueError> {self.retired_tessellation_demand(|owner|owner.next_copy_byte_demand(),0)}
pub fn next_retired_tessellation_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.retired_tessellation_demand(|owner|owner.next_capacity_byte_demand(copy),0)}
pub fn next_retired_tessellation_release_byte_demand(&self)->Result<usize,ValueError> {self.retired_tessellation_demand(|owner|owner.next_release_byte_demand(),0)}
pub fn next_retired_tessellation_depth_demand(&self)->Result<usize,ValueError> {self.retired_tessellation_demand(|owner|owner.next_depth_demand(),1)}
fn retire_tessellations_cold(&self)->Result<(),String> {
    loop {
        let copy=self.next_retired_tessellation_copy_byte_demand().map_err(|error|error.to_string())?;
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_retired_tessellation_capacity_byte_demand(copy).map_err(|error|error.to_string())?,maximum_release_bytes:self.next_retired_tessellation_release_byte_demand().map_err(|error|error.to_string())?,maximum_depth:self.next_retired_tessellation_depth_demand().map_err(|error|error.to_string())?};
        if matches!(self.close_retired_tessellation_step(grant).map_err(|error|error.to_string())?,RetainedCloneStep::Complete(_)) {return Ok(());}
    }
}
pub fn tessellation_progress(&self,handle:&str,tolerance:f64)->Option<(usize,usize,&'static str)> {
    let owner=self.tessellation_jobs().try_lock().ok()?;
    let registry=owner.original()?;
    let retained=registry.jobs.iter().filter(|(key,_)|key.0==handle&&key.1==tolerance.to_bits()).max_by_key(|(_,row)|row.last_step).map(|(_,row)|row)?;
    if retained.retired {return None;}
    let progress=retained.job.progress();
    Some((progress.units_done.saturating_add(if retained.validated {retained.validation_units}else {0}),progress.units_total.saturating_add(retained.validation_units),if retained.validated {progress.phase.tag()}else {"validating"}))
}
fn with_cached_mesh_at_or_finer<R>(&self, handle: &str, tolerance: f64, read: impl FnOnce(&semio_framework::MeshData) -> R) -> Option<R> {
    if self.is_closed() { return None; }
    let cache = self.mesh_cache().try_lock().ok()?;
    if let Some(exact) = cache.iter().find(|(key,_)|key.0==handle&&key.1==tolerance.to_bits()).map(|(_,mesh)|mesh) {
        if exact.valid {return Some(read(&exact.mesh));}
    }
    let mut best: Option<(f64, &semio_framework::MeshData)> = None;
    for ((cached_handle,bits),cached) in cache.iter() {
        if !cached.valid {continue;}
        let mesh=&cached.mesh;
        if cached_handle != handle {
            continue;
        }
        let cached_tolerance = f64::from_bits(*bits);
        if cached_tolerance > tolerance {
            continue;
        }
        if best.is_none_or(|(current, _)| cached_tolerance > current) {
            best = Some((cached_tolerance, mesh));
        }
    }
    best.map(|(_, mesh)| read(mesh))
}
pub fn cached_mesh_at_or_finer(&self, handle: &str, tolerance: f64) -> Option<semio_framework::MeshData> {
    self.with_cached_mesh_at_or_finer(handle, tolerance, semio_framework::MeshData::clone)
}
/// 📏️ Exact retained payload extent of the cached mesh the next tessellation turn would clone out; `0` when none is cached.
pub fn cached_mesh_payload_bytes(&self, handle: &str, tolerance: f64) -> usize {
    self.with_cached_mesh_at_or_finer(handle, tolerance, mesh_payload_bytes).unwrap_or(0)
}
pub(super) fn tessellation_work_units(&self,identity:semio_framework_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64)->usize {
    self.tessellation_jobs().try_lock().ok().and_then(|owner|owner.original().and_then(|registry|registry.jobs.iter().find(|(key,_)|key.0==handle&&key.1==tolerance.to_bits()&&key.2==identity.operation&&key.3==identity.generation).map(|(_,row)|row.source_units+row.job.progress().units_done+if row.validated{row.validation_units}else{0}))).unwrap_or(0)
}

pub(super) fn step_tessellation_row(&self,guard:&Brep,key:&TessellationJobKey,retained:&mut RetainedTessellation,geometry:Option<&GeometryHandle>,budget:usize)->TessellationStepOutcome{
    let Some(geometry)=geometry.or(retained.geometry.as_ref())else{return TessellationStepOutcome::Failed{message:"original tessellation row has no retained geometry source".into()};};
    let mut remaining=budget;
    if !retained.validated {
        if remaining==0 {return TessellationStepOutcome::Working {units_done:retained.source_units,units_total:retained.job.progress().units_total.saturating_add(retained.source_units).saturating_add(1),faces_done:0,faces_total:0,phase:"validating"};}
        if let Err(issues)=guard.validate_gate_sync(geometry) {
            retained.job.cancel();retained.retired=true;
            return TessellationStepOutcome::Invalid {issues:issues.into_iter().map(|issue|PreviewDiagnostic {entity:issue.entity,code:issue.code.to_string(),message:issue.message}).collect()};
        }
        retained.validated=true;remaining-=1;
    }
    let validation_units=retained.validation_units;
    let step = guard.step_tessellation_job_sync(geometry,&mut retained.job,remaining);
    match step {
        Err(error) => {
            retained.job.cancel();retained.retired=true;
            TessellationStepOutcome::Failed { message: error.to_string() }
        }
        Ok(TessellationStep::Working(progress)) => TessellationStepOutcome::Working { units_done:progress.units_done.saturating_add(validation_units).saturating_add(retained.source_units),units_total:progress.units_total.saturating_add(validation_units).saturating_add(retained.source_units), faces_done: progress.faces_done, faces_total: progress.faces_total, phase: progress.phase.tag() },
        Ok(TessellationStep::Cancelled(_)) => {
            retained.job.cancel();retained.retired=true;
            TessellationStepOutcome::Cancelled
        }
        Ok(TessellationStep::Done(progress)) => {
            retained.retired=true;
            let Some((transfer,_report))=retained.job.take_mesh() else {return TessellationStepOutcome::Failed {message:"finished tessellation job produced no mesh".into()};};
            retained.output=Some(transfer);
            let mesh=match semio_framework_3d_brep_engine::mesh_data_from_mesh_transfer(retained.output.as_ref().unwrap()) {Ok(mesh)=>mesh,Err(error)=>return TessellationStepOutcome::Failed {message:error.to_string()}};
            if let Ok(mut cache)=self.mesh_cache().try_lock() {retained.previous_mesh=cache.insert((key.0.clone(),key.1),CachedMesh {mesh:mesh.clone(),valid:true});}
            TessellationStepOutcome::Ready { mesh, units_total:progress.units_total.saturating_add(validation_units).saturating_add(retained.source_units),faces_total:progress.faces_total }
        }
    }
}

pub fn dispose_geometry(&self, handle: &str) -> Result<(), String> {
    let mut claims = self.state.claims.try_lock().map_err(|_| "geometry claims owner busy")?;
    if self.is_closed() { return Err("geometry.session-closed".into()); }
    if claims.iter().any(|(authority, handles)| *authority != self.state.authority && handles.contains(handle)) { return Err("geometry.handle-retained-by-other-authority".into()); }
    let mut kernel = self.kernel().try_lock().map_err(|_| "geometry kernel owner busy")?;
    kernel.dispose(&GeometryHandle(handle.into()));
    if let Some(own) = claims.get_mut(&self.state.authority) { own.remove(handle); self.claims_changed();}
    self.evict_mesh_cache_for_handle(handle);
    self.retire_jobs_for_handle(handle);
    Ok(())
}
pub fn export_solid_json(&self, handles: &[String], format: &str, deflection: f64) -> String {
    let shapes: Vec<GeometryHandle> = handles.iter().cloned().map(GeometryHandle).collect();
    let outcome: Result<(String, bool), BrepModuleError> = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy).and_then(|guard| {
        if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
        let guard = &**guard;
        match format {

            "obj" => guard.export_obj(&shapes, deflection).map(|text| (text, false)).map_err(BrepModuleError::from),
            "stl" => guard.export_stl(&shapes, deflection).map(|data| (encode_base64(&data), true)).map_err(BrepModuleError::from),
            "glb" => export_glb_via_tessellation(guard, &shapes, deflection).map(|data| (encode_base64(&data), true)),
            other => self.state.operations.export(guard,other,&shapes,deflection),
        }
    });
    match outcome {
        Ok((data, binary)) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("data".to_string(), semio_framework_pack_json::Value::String(data)),
            ("binary".to_string(), semio_framework_pack_json::Value::Bool(binary)),
            ("format".to_string(), semio_framework_pack_json::Value::String(format.to_string())),
        ])),
        Err(error) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(error.to_string()))])),
    }
}
pub fn import_solid_json(&self, format: &str, data: &str, tolerance: f64) -> String {
    let mut claims = self.state.claims.try_lock().expect("geometry claims");
    let outcome: Result<Vec<String>, BrepModuleError> = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy).and_then(|mut guard| {
        if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
        let guard = &mut **guard;
        match format {

            "obj" => guard.import_obj(data, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from),
            "stl" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| guard.import_stl(&bytes, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)),
            "glb" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| import_glb_via_tessellation(guard, &bytes, tolerance)),
            other => self.state.operations.import(guard,other,data,tolerance).map(|handles| handles.into_iter().map(|handle| handle.0).collect()),
        }
    });
    if let Ok(handles) = &outcome { claims.entry(self.state.authority).or_default().extend(handles.iter().cloned()); self.claims_changed();}
    match outcome {
        Ok(handles) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("handles".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&handles)))])),
        Err(error) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(error.to_string()))])),
    }
}
fn brep_invoke_inner(&self, method: &str, args_json: &str) -> Result<semio_framework_pack_json::Value, BrepModuleError> {
    let mut claims = self.state.claims.try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
    if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
    let args = invoke_args(args_json)?;
    let result = match method {
        "box" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.box_prim(arg_f64(&args, "width")?, arg_f64(&args, "depth")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sphere" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.sphere_prim(arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cylinder" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.cylinder_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cone" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.cone_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "lineCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.line_curve(arg_vec3(&args, "start")?, arg_vec3(&args, "end")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "circleCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.circle_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "arcCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .arc_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?, arg_f64(&args, "startAngle")?, arg_f64(&args, "endAngle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "ellipseCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .ellipse_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "semiMajor")?, arg_f64(&args, "semiMinor")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "interpolateCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.interpolate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "approximateCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .approximate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?, arg_usize(&args, "controlPoints")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "polylineWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.polyline_wire(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rectangleWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.rectangle_wire(arg_f64(&args, "width")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromPoints" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.planar_face_from_points(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.planar_face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrudeWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.extrude_wire(&arg_handle(&args, "wire")?, arg_vec3(&args, "vector")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrude" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.extrude(&arg_handle(&args, "face")?, arg_vec3(&args, "direction")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "revolve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .revolve(&arg_handle(&args, "face")?, arg_vec3(&args, "axisOrigin")?, arg_vec3(&args, "axisDirection")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "loft" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.loft(&arg_handles(&args, "profiles")?, arg_bool_or(&args, "smooth", false)).map(handle_result).map_err(BrepModuleError::from)
        }
        "sweep" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.sweep(&arg_handle(&args, "profile")?, &arg_handle(&args, "path")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "thickenFace" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.thicken_face(&arg_handle(&args, "face")?, arg_f64(&args, "thickness")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "offsetFace" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.offset_face(&arg_handle(&args, "face")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "fuse" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.fuse(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cut" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.cut(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "intersect" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.intersect(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "translate" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.translate(&arg_handle(&args, "shape")?, arg_vec3(&args, "offset")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rotate" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.rotate(&arg_handle(&args, "shape")?, arg_vec3(&args, "axis")?, arg_f64(&args, "angle")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rotateAbout" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .rotate_about(&arg_handle(&args, "shape")?, arg_vec3(&args, "origin")?, arg_vec3(&args, "axis")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "scale" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.scale(&arg_handle(&args, "shape")?, arg_f64(&args, "factor")?, arg_vec3(&args, "center")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "mirror" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.mirror(&arg_handle(&args, "shape")?, arg_vec3(&args, "origin")?, arg_vec3(&args, "normal")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sewFaces" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.sew_faces(&arg_handles(&args, "faces")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "faceFromWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "healSolid" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.heal_solid(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "volume" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.volume(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "area" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.area(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "length" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.length(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "centerOfMass" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.center_of_mass(&arg_handle(&args, "shape")?).map(vec3_result).map_err(BrepModuleError::from)
        }
        "distance" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.distance(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(number_result).map_err(BrepModuleError::from)
        }
        "deconstruct" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.deconstruct(&arg_handle(&args, "shape")?).map(topology_result).map_err(BrepModuleError::from)
        }
        "tessellate" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.tessellate(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-3)).map(|mesh| mesh_result(&mesh)).map_err(BrepModuleError::from)
        }
        "dispose" => {
            let handle = arg_handle(&args, "handle")?;
            if claims.iter().any(|(authority, handles)| *authority != self.state.authority && handles.contains(&handle.0)) { return Err(BrepModuleError::InvalidArgs("geometry.handle-retained-by-other-authority".into())); }
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.dispose(&handle);
            if let Some(own) = claims.get_mut(&self.state.authority) { own.remove(&handle.0); self.claims_changed();}
            self.evict_mesh_cache_for_handle(&handle.0);
            self.retire_jobs_for_handle(&handle.0);
            Ok(unit_result())
        }
        "retain" => {
            let live = arg_handles(&args, "handles")?.into_iter().map(|handle| handle.0).collect::<Vec<_>>();
            claims.entry(self.state.authority).or_default().retain(&live);self.claims_changed();
            let merged = claims.values().flat_map(|handles| handles.iter().cloned()).collect::<HashSet<_>>();
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.retain(&merged);
            self.evict_mesh_cache_for_handles(&merged.into_iter().collect::<Vec<_>>());
            self.retain_tessellation_jobs(&live);
            Ok(unit_result())
        }
        other => {
            let mut guard = self.kernel().try_lock().map_err(|_|BrepModuleError::OwnerBusy)?;
            self.state.operations.invoke(&mut guard,other,&args)
        },
    };
    if let Ok(value) = &result {
        fn gather(value: &semio_framework_pack_json::Value, handles:&mut AuthorityClaims) {
            if let Some(handle) = value.get("handle").and_then(|value| value.as_str()) { handles.insert(handle.into()); }
            for key in ["handles", "vertices", "edges", "faces", "shells"] {
                if let Some(values) = value.get(key).and_then(|value| value.as_array()) { handles.extend(values.iter().filter_map(|value| value.as_str().map(str::to_string))); }
            }
        }
        gather(value, claims.entry(self.state.authority).or_default());self.claims_changed();
    }
    result
}
pub fn brep_invoke_json(&self, method: &str, args_json: &str) -> String {
    match self.brep_invoke_inner(method, args_json) {
        Ok(value) => semio_framework_pack_json::to_string(&value),
        Err(error) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(error.to_string()))])),
    }
}
}

/// ⚓️ Retains the producing family while retiring a separately admitted job authority.
struct SessionPort {authority:SessionCapture,anchor:SessionCapture,retaining:RetainedOwnerGate<Option<SessionRetention>>,retain_receipt:RetainedCloneProgress,cancel_receipt:RetainedCloneProgress}
impl SessionPort {fn retained_demand(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{let owner=self.retaining.try_lock().map_err(|_|ownership_busy())?;owner.as_ref().map_or(Ok(Default::default()),|owner|owner.demand(self.authority.session.as_ref().ok_or_else(ownership_busy)?,copy))}}
impl Drop for SessionPort {fn drop(&mut self) {if !std::thread::panicking() {assert!(self.retaining.get_mut().is_none()&&self.authority.terminal_is_empty() && self.anchor.terminal_is_empty(),"geometry port requires terminal-empty retirement");}}}
impl semio_framework_os_flow::geometry::GeometryPort for SessionPort {
    fn begin_retain(&mut self,handles:Vec<String>)->Result<(),(ValueError,Vec<String>)>{if self.retaining.get_mut().is_some(){return Err((ownership_busy(),handles));}let Some(session)=self.authority.session.as_ref()else{return Err((ownership_busy(),handles));};*self.retaining.get_mut()=Some(SessionRetention::new(session,handles)?);Ok(())}
    fn retain_terminal_is_empty(&self)->bool{self.retaining.try_lock().is_ok_and(|owner|owner.is_none())}
    fn next_retain_copy_byte_demand(&self)->Result<usize,ValueError>{self.retained_demand(0).map(|demand|demand.copy_bytes)}
    fn next_retain_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.retained_demand(copy).map(|demand|demand.capacity_bytes)}
    fn next_retain_release_byte_demand(&self)->Result<usize,ValueError>{self.retained_demand(0).map(|demand|demand.release_bytes)}
    fn next_retain_depth_demand(&self)->Result<usize,ValueError>{self.retained_demand(0).map(|demand|demand.depth)}
    fn retain_step_progress(&self)->RetainedCloneProgress{self.retain_receipt}
    fn retain_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.retain_receipt=Default::default();let Some(owner)=self.retaining.get_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};let result=owner.step(self.authority.session.as_ref().ok_or_else(ownership_busy)?,grant);self.retain_receipt=owner.normal_step_progress();let step=result?;if owner.terminal_is_empty(){*self.retaining.get_mut()=None;Ok(RetainedCloneStep::Complete(step.progress()))}else{Ok(RetainedCloneStep::Progress(step.progress()))}}
    fn tessellation_demands(&self,identity:semio_framework_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.authority.session.as_ref().ok_or_else(ownership_busy)?.tessellation_demands(identity,handle,tolerance,copy)}
    fn tessellate_step(&self,identity:semio_framework_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(semio_framework_os_flow::geometry::GeometryStep,semio_framework_os_flow::geometry::GeometryWorkProgress),semio_framework_os_flow::geometry::GeometryWorkFailure>{self.authority.session.as_ref().ok_or_else(ownership_busy).map_err(|primary|semio_framework_os_flow::geometry::GeometryWorkFailure::new(primary,0))?.tessellate_step(identity,handle,tolerance,cx)}
    fn cancel_tessellation_demands(&self,identity:semio_framework_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.authority.session.as_ref().ok_or_else(ownership_busy)?.cancel_tessellation_demands(identity,handle,tolerance,copy)}
    fn cancel_tessellation_step(&self,identity:semio_framework_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(RetainedCloneStep,semio_framework_os_flow::geometry::GeometryWorkProgress),semio_framework_os_flow::geometry::GeometryWorkFailure>{self.authority.session.as_ref().ok_or_else(ownership_busy).map_err(|primary|semio_framework_os_flow::geometry::GeometryWorkFailure::new(primary,0))?.cancel_tessellation_step(identity,handle,tolerance,cx)}
    fn dispose(&self,handle:&str)->Result<(),String> {self.authority.session.as_ref().ok_or("geometry session is closed")?.dispose_geometry(handle)}
    fn begin_cancel(&mut self)->Result<(),ValueError>{self.authority.session.as_ref().ok_or_else(ownership_busy)?.begin_cancel()?;if let Some(owner)=self.retaining.get_mut(){owner.cancel();}Ok(())}
    fn cancel_terminal_is_empty(&self)->bool{self.retain_terminal_is_empty()&&self.authority.session.as_ref().is_none_or(Session::cancel_terminal_is_empty)}
    fn next_cancel_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_copy_byte_demand();}self.authority.session.as_ref().map_or(Ok(0),Session::next_cancel_copy_byte_demand)}
    fn next_cancel_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_capacity_byte_demand(copy);}self.authority.session.as_ref().map_or(Ok(0),|session|session.next_cancel_capacity_byte_demand(copy))}
    fn next_cancel_release_byte_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_release_byte_demand();}self.authority.session.as_ref().map_or(Ok(0),Session::next_cancel_release_byte_demand)}
    fn next_cancel_depth_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_depth_demand();}self.authority.session.as_ref().map_or(Ok(0),Session::next_cancel_depth_demand)}
    fn cancel_step_progress(&self)->RetainedCloneProgress{self.cancel_receipt}
    fn cancel_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.cancel_receipt=Default::default();if !self.retain_terminal_is_empty(){let result=self.retain_step(grant);self.cancel_receipt=self.retain_receipt;return result.map(|step|RetainedCloneStep::Progress(step.progress()));}let result=self.authority.session.as_ref().map_or(Ok(RetainedCloneStep::Complete(Default::default())),|session|session.cancel_step(grant,&mut self.cancel_receipt));if let Ok(step)=&result{self.cancel_receipt=step.progress();}result}
    fn begin_close(&self) {if let Ok(mut retaining)=self.retaining.try_lock(){if let Some(owner)=retaining.as_mut(){owner.cancel();}}if let Some(authority)=self.authority.session.as_ref() {authority.begin_close();}}
    fn terminal_is_empty(&self)->bool {self.retain_terminal_is_empty()&&self.authority.terminal_is_empty() && self.anchor.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_retain_copy_byte_demand();}if self.authority.terminal_is_empty() {self.anchor.next_close_copy_byte_demand()}else {self.authority.next_close_copy_byte_demand()}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_retain_capacity_byte_demand(copy);}if self.authority.terminal_is_empty() {self.anchor.next_close_capacity_byte_demand(copy)}else {self.authority.next_close_capacity_byte_demand(copy)}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_retain_release_byte_demand();}if self.authority.terminal_is_empty() {self.anchor.next_close_release_byte_demand()}else {self.authority.next_close_release_byte_demand()}}
    fn next_depth_demand(&self)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_retain_depth_demand();}if self.authority.terminal_is_empty() {self.anchor.next_close_depth_demand()}else {self.authority.next_close_depth_demand()}}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.retain_terminal_is_empty(){return self.retain_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if !self.authority.terminal_is_empty() {return self.authority.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        self.anchor.close_step(grant)
    }
}
