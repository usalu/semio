//! 🧠️ Session core: the shell allocator, `Session`, its capture and retirement state, and the geometry port.

use super::*;
use semio_framework_3d_brep_engine::{AnalyticPublication, ConicFacePublication, PolygonFacePublication, PolygonSectionSet, PolygonWirePublication, PolygonWireRecipe, PrismDraft, SECTION_FACE_LIMIT, SolidPrimitive, SolidPublication, VertexPublication};
use semio_framework_value::{RetainedCloneGrant, RetirementDemand};
use semio_framework_3d_brep_engine::{CompoundPublication, TopologyAlias, TopologyList, TopologySelection};
use semio_framework_3d_brep_engine::{SolidPatternPublication, SolidPatternRecipe};
use semio_framework_3d_brep_engine::{MeshDecode, MeshPublication};

trait GeometryPublication {
    fn handle(&self) -> &str;
    fn demand(&self, kernel: &Brep, copy: usize) -> Result<RetirementDemand, ValueError>;
    fn reserve(&self, kernel: &mut Brep, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError>;
    fn publish(&mut self, kernel: &mut Brep, key: String, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, (ValueError, String)>;
}

macro_rules! geometry_publication {
    ($($publication:ty => $demand:ident, $reserve:ident, $publish:ident;)+) => {
        $(impl GeometryPublication for $publication {
            fn handle(&self) -> &str { self.handle() }
            fn demand(&self, kernel: &Brep, copy: usize) -> Result<RetirementDemand, ValueError> {
                kernel.$demand(self, copy)
            }
            fn reserve(&self, kernel: &mut Brep, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
                kernel.$reserve(self, grant)
            }
            fn publish(&mut self, kernel: &mut Brep, key: String, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, (ValueError, String)> {
                kernel.$publish(self, key, grant)
            }
        })+
    };
}

geometry_publication! {
    AnalyticPublication => analytic_publication_demand, reserve_analytic_publication, publish_analytic_geometry;
    CompoundPublication => compound_publication_demand, reserve_compound_publication, publish_compound;
    SolidPatternPublication => solid_pattern_publication_demand, reserve_solid_pattern, publish_solid_pattern;
    SolidPublication => solid_primitive_demand, reserve_solid_primitive, publish_solid_primitive;
    PolygonFacePublication => polygon_face_demand, reserve_polygon_face, publish_polygon_face;
    VertexPublication => vertex_publication_demand, reserve_vertex_publication, publish_vertex;
    PolygonWirePublication => polygon_wire_demand, reserve_polygon_wire, publish_polygon_wire;
    ConicFacePublication => conic_face_demand, reserve_conic_face, publish_conic_face;
}

impl GeometryPublication for MeshPublication {
    fn handle(&self) -> &str { self.handle().expect("prepared original mesh handle") }
    fn demand(&self, kernel: &Brep, copy: usize) -> Result<RetirementDemand, ValueError> { kernel.mesh_publication_demand(self, copy) }
    fn reserve(&self, kernel: &mut Brep, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> { kernel.reserve_mesh_publication(self, grant) }
    fn publish(&mut self, kernel: &mut Brep, key: String, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, (ValueError, String)> { kernel.publish_mesh(self, key, grant) }
}

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
    pub fn close_original_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        let grant=effect.grant();let result=(||->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
        let state=self.session.as_ref().map(|session|&*session.state).or_else(||self.orphan.as_ref());
        if state.is_some_and(|state|state.retirement.try_lock().map(|owner|owner.paused).unwrap_or(true)) {return Ok(ownership_progress());}
        if let Some(session)=self.session.as_ref() {
            spend_event(effect)?;let allocation=Arc::downgrade(&session.state);
            self.allocation_bytes=shell_bytes(&session.state);
            *self.allocation=Some(allocation);
            let Session {state}=self.session.take().unwrap();
            *self.orphan=Arc::into_inner(state);
            return Ok(ownership_advance());
        }
        if let Some(state)=self.orphan.as_ref() {
            if !state.terminal_is_empty() {return settled_step(state.close_original_step(effect)).map(|step|RetainedCloneStep::Progress(step.progress()));}
            if grant.maximum_release_bytes<self.shell_byte_requirement() {return Ok(ownership_progress());}
            spend_event(effect)?;let receipt=std::sync::atomic::AtomicUsize::new(0);
            record_shell(&state.kernel,&receipt);record_shell(&state.mesh_cache,&receipt);record_shell(&state.claims,&receipt);record_shell(&state.next_authority,&receipt);record_shell(&state.retirement,&receipt);
            drop(self.orphan.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed),..Default::default()}));
        }
        if grant.maximum_release_bytes<self.allocation_bytes {return Ok(ownership_progress());}
        spend_event(effect)?;let receipt=std::sync::atomic::AtomicUsize::new(0);
        let allocation=self.allocation.take().unwrap();
        Weak::allocator(&allocation).receipt.store(std::ptr::from_ref(&receipt).cast_mut(),std::sync::atomic::Ordering::Relaxed);
        drop(allocation);
        self.allocation_bytes=0;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed),..Default::default()}))
    })();settled_poll(effect,result)
    }
    /// 🧾️ Borrows the same close result when only physical retirement is requested.
    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{settled_step(self.close_original_step(effect))}
    /// 🧊️ Grants each current original currency exactly at an explicit cold boundary.
    pub fn retire_cold(&mut self) {
        while !self.terminal_is_empty() {
            let copy=self.next_close_copy_byte_demand().expect("cold capture copy demand");
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_close_capacity_byte_demand(copy).expect("cold capture capacity demand"),maximum_release_bytes:self.next_close_release_byte_demand().expect("cold capture release demand"),maximum_depth:self.next_close_depth_demand().expect("cold capture depth demand")};
            let step=neural_engine::retirement::cold_effect(grant,|effect|self.close_step(effect)).expect("cold geometry capture effect").expect("cold geometry capture retirement");
            assert!(step.progress()!=RetainedCloneProgress::default() || self.terminal_is_empty(),"cold geometry capture waits on an original owner");
        }
    }
}

impl Drop for SessionCapture {
    fn drop(&mut self){assert!(self.terminal_is_empty(),"geometry capture requires explicit retirement before drop");}
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
/// 🔁️ Reads one settled retirement poll as its step; a suspended effect keeps its state and retries next turn.
pub(super) fn settled_step(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError> {match poll {RetainedWorkPoll::Pending{..}=>Ok(ownership_progress()),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(OriginalRetirementFailure::into_value_error)}}
/// 🪙️ Settles a native close result as the complete poll of the effect that funded it.
pub(super) fn settled_poll(effect:&RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>)->OriginalRetirementPoll {RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:result.map_err(Into::into),work:effect.performed_work()})}
/// ⛽️ Records one unit of original work for a native event that runs no nested producer.
pub(super) fn spend_event(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError> {if effect.performed(1)? {Ok(())} else {Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original geometry close exceeds its work ceiling"))}}
pub(super) fn controlled<T:semio_framework_value::retirement::RetireOwned>(value:T)->ControlledRetirement<T> {ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original geometry owner has no retirement authority"))}
impl SessionState {
    pub(super) fn begin_close(&self){self.try_begin_close().expect("geometry original close admission");}
    fn try_begin_close(&self)->Result<(),ValueError>{
        let mut claims=self.claims.try_lock().map_err(|_|ownership_busy())?;let mut retirement=self.retirement.try_lock().map_err(|_|ownership_busy())?;
        if self.closed.swap(true,std::sync::atomic::Ordering::AcqRel){return Ok(())}
        if let Some(claim)=claims.get_mut(&self.authority){claim.active=false;retirement.active-=1;self.next_authority.fetch_add(1,std::sync::atomic::Ordering::AcqRel);}Ok(())
    }
    pub(super) fn terminal_is_empty(&self)->bool {
        let Ok(jobs)=self.jobs.try_lock() else {return false};
        let Ok(retirement)=self.retirement.try_lock() else {return false};
        self.closed.load(std::sync::atomic::Ordering::Acquire) && jobs.terminal_is_empty() && (retirement.active!=0 || Arc::strong_count(&self.retirement)>1 || retirement.family.as_ref().is_some_and(ControlledRetirement::terminal_is_empty))
    }
    pub(super) fn close_original_step(&self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        let grant=effect.grant();let result=(||->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
        if self.retirement.try_lock().map_err(|_|ownership_busy())?.paused {return Ok(ownership_progress());}
        if !self.closed.load(std::sync::atomic::Ordering::Acquire){spend_event(effect)?;self.try_begin_close()?;return Ok(ownership_advance())}
        let mut jobs=self.jobs.try_lock().map_err(|_|ownership_busy())?;
        if !jobs.terminal_is_empty() {return settled_step(jobs.step(grant.maximum_copy_bytes,effect)).map(|step|RetainedCloneStep::Progress(step.progress()));}
        drop(jobs);
        let mut retirement=self.retirement.try_lock().map_err(|_|ownership_busy())?;
        if retirement.active!=0 || Arc::strong_count(&self.retirement)>1 {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if retirement.family.is_none() {
            spend_event(effect)?;let mut kernel=self.kernel.try_lock().map_err(|_|ownership_busy())?;
            let mut cache=self.mesh_cache.try_lock().map_err(|_|ownership_busy())?;
            let mut claims=self.claims.try_lock().map_err(|_|ownership_busy())?;
            let original=(std::mem::replace(&mut **kernel,Brep::new()),std::mem::take(&mut **cache),std::mem::take(&mut **claims));
            retirement.family=Some(controlled(original));
            return Ok(ownership_advance());
        }
        let family=retirement.family.as_mut().unwrap();
        if family.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        settled_step(family.step(grant.maximum_copy_bytes,effect)).map(|step|RetainedCloneStep::Progress(step.progress()))
    })();settled_poll(effect,result)
    }
    pub(super) fn close_step(&self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{settled_step(self.close_original_step(effect))}
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
    pub fn port(&self) -> Box<dyn semio_os_flow::geometry::GeometryPort> {Box::new(self.admitted_port())}
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
    pub fn close_step(&self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {self.state.close_step(effect)}
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
            let progress=neural_engine::retirement::cold_effect(grant,|effect|self.close_step(effect)).expect("cold geometry effect").expect("cold geometry retirement").progress();
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
/// 🎯️ Plans an analytic curve without scanning or copying session handles.
pub fn plan_analytic_curve(&self, parameters: semio_framework_3d_brep_engine::AnalyticCurveParameters) -> Result<semio_framework_3d_brep_engine::AnalyticPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_analytic_curve(parameters)
}

/// ➿️ Plans original curve interpolation from the live ordered point input size.
pub fn plan_interpolated_curve(&self, count: usize, degree: usize) -> Result<AnalyticPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_interpolated_curve(count, degree)
}

/// 📉️ Admits an original endpoint-constrained curve fit.
pub fn plan_approximated_curve(&self, count: usize, degree: usize, controls: usize) -> Result<AnalyticPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_approximated_curve(count, degree, controls)
}

/// 🌈️ Plans an exact four-boundary Coons surface under the original session owner.
pub fn plan_coons_surface(&self, boundaries: [usize; 4]) -> Result<AnalyticPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_coons_surface(boundaries)
}

/// 🪡️ Plans a bounded interpolating surface from original grid dimensions.
pub fn plan_interpolated_grid(&self, rows: usize, columns: usize, degree_u: usize, degree_v: usize) -> Result<AnalyticPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_interpolated_grid(rows, columns, degree_u, degree_v)
}

/// 📄️ Plans a fixed analytic plane in the original session.
pub fn plan_analytic_plane(&self, origin: Vec3, normal: Vec3) -> Result<AnalyticPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_analytic_plane(origin, normal)
}

/// 🌛️ Opens a budgeted rational circular arc in the original kernel.
pub fn plan_analytic_arc(&self, center:Vec3,normal:Vec3,radius:f64,start:f64,end:f64) -> Result<AnalyticPublication,ValueError> {
    if self.is_closed() {return Err(ownership_busy());}
    self.kernel().try_lock().map_err(|_|ownership_busy())?.plan_analytic_arc(center,normal,radius,start,end)
}

/// 📐️ Stages the analytic affine image in its original kernel.
pub fn plan_analytic_transform(&self, source: &str, transform: semio_framework_3d_brep_engine::AnalyticTransform) -> Result<AnalyticPublication,ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_analytic_transform(source, transform)
}

/// 🎫️ Captures one original certified Boolean operand with no topology copy.
pub fn capture_boolean_operand(&self, handle: &str) -> Result<semio_framework_3d_brep_engine::BooleanOperand, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.capture_boolean_operand(handle)
}

/// 🔍️ Starts acquisition of the general Boolean's source faces.
pub fn plan_boolean_preparation(&self, a: &str, b: &str) -> Result<semio_framework_3d_brep_engine::BooleanPreparation, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_boolean_preparation(a,b)
}

/// 📏️ Quotes one original source face lookup and transfer.
pub fn boolean_preparation_demand(&self, a: &str, b: &str, preparation: &semio_framework_3d_brep_engine::BooleanPreparation)
    -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.boolean_preparation_demand(a,b,preparation)
}

/// 🧩️ Acquires one original source face for general Boolean preparation.
pub fn step_boolean_preparation(&self, a: &str, b: &str, preparation: &mut semio_framework_3d_brep_engine::BooleanPreparation,
    effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_boolean_preparation(a,b,preparation,effect)
}

/// ⚖️ Admits an exact certified Boolean result without copying any live session rows.
pub fn plan_certified_boolean(&self, a: &str, b: &str, first: &semio_framework_3d_brep_engine::BooleanOperand,
    operation: semio_framework_3d_brep_engine::BooleanOperation) -> Result<semio_framework_3d_brep_engine::CertifiedBooleanResult<SolidPatternPublication>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_certified_boolean(a, b, first, operation)
}

/// 🛢️ Stages the affine image of a live certified solid recipe if it remains exact.
pub fn plan_certified_solid_transform(&self, source: &str, transform: semio_framework_3d_brep_engine::AnalyticTransform) -> Result<Option<semio_framework_3d_brep_engine::SolidPublication>,ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_certified_solid_transform(source, transform)
}

/// ✂️ Captures one certified box and plane for a retained section.
pub fn plan_section(&self,source:&str,origin:Vec3,normal:Vec3)->Result<semio_framework_3d_brep_engine::SectionPlan,ValueError>{
    if self.is_closed(){return Err(ownership_busy());}
    self.kernel().try_lock().map_err(|_|ownership_busy())?.plan_section(source,origin,normal)
}

/// 🎟️ Quotes original native reads for the analytic bounding interval.
pub fn analytic_bounds_depth(&self, handle: &str) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.analytic_bounds_depth(handle))
}

/// 🔳️ Stages a solid for the exact world bounds of one live analytic source.
pub fn plan_analytic_bounds(&self, handle: &str) -> Result<semio_framework_3d_brep_engine::SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_analytic_bounds(handle)
}

/// 🎟️ Quotes the original certified solid lookup and arena read.
pub fn certified_solid_depth(&self, handle: &str) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.certified_solid_depth(handle))
}

/// 📐 Borrows one live certified solid and its original native topology under the session lock.
pub fn with_original_certified_solid<T>(&self, handle: &str, read: impl FnOnce(
    &semio_framework_3d_brep_representation::topology::Body,
    semio_framework_3d_brep_representation::arena::SolidId, u64, usize,
) -> Result<T, ValueError>) -> Result<T, ValueError> {
    use semio_framework_3d_brep_representation::topology::EntityRef;
    self.with_original_geometry_query(|kernel| {
        if let Some(EntityRef::Solid(id)) = kernel.preview_validation_root(handle, 0)? {
            if kernel.original_validation_is_certified(handle)? {
                return read(kernel.representation(), id, kernel.preview_validation_revision(), kernel.certified_solid_depth(handle)?);
            }
        }
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original exchange requires a certified solid"))
    })
}

/// 🧊️ Borrows the exact box certificate and captures its fixed world-space interval.
pub fn certified_box_bounds(&self, handle: &str) -> Result<[[f64; 3]; 2], ValueError> {
    self.with_original_geometry_query(|kernel| {
        let primitive = kernel.live_solid_primitive(handle).filter(|shape| matches!(shape, SolidPrimitive::Box { .. }))
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Compound Cut requires certified boxes"))?;
        Ok(primitive.axis_aligned_bounds().map(|point| point.to_array()))
    })
}

/// 🪓️ Builds one native transaction for the bounded box arrangement.
pub fn plan_compound_box_cut(&self, stock: [[f64; 3]; 2], tools: &[[[f64; 3]; 2]]) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_compound_box_cut(stock, tools)
}

/// 📌️ Classifies a point in the live certified solid's world placement.
pub fn original_solid_classification(&self, handle: &str, point: Vec3) -> Result<PointClassification, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_solid_classification(handle, point))
}

/// 🧭️ Opens the retained distance cursor for an original vertex and planar face pair.
pub fn plan_face_distance(&self, a: &str, b: &str) -> Result<Option<(semio_framework_3d_brep_engine::PlanarClosestJob, bool)>, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.plan_face_distance(a, b))
}

/// 📏️ Computes original vertex–curve and certified box distances.
pub fn original_measure_distance(&self, a: &str, b: &str) -> Result<f64, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_measure_distance(a, b))
}

/// 🪜️ Quotes the source rows for a retained measurement in one session read.
pub fn original_measure_source_depth(&self, sources: &[&str]) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| sources.iter().try_fold(1, |depth, source| {
        Ok(depth.max(kernel.original_measure_source_depth(source)?))
    }))
}

/// 🥞️ Quotes the paired original Loft section roots.
pub fn loft_depth(&self, handles: [&str; 2]) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.loft_depth(handles))
}

/// 🧱️ Stages a compatible two-profile exact Loft.
pub fn plan_exact_loft(&self, handles: [&str; 2]) -> Result<semio_framework_3d_brep_engine::LoftPlan, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_exact_loft(handles)
}

/// 📏️ Quotes one funded check of corresponding Loft lines.
pub fn loft_draft_demand(&self, handles: [&str; 2], draft: &semio_framework_3d_brep_engine::LoftDraft) -> Result<RetirementDemand, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.loft_draft_demand(handles, draft))
}

/// 🛠️ Advances one original Loft pair without copying either whole profile.
pub fn step_loft_draft(&self, handles: [&str; 2], draft: &mut semio_framework_3d_brep_engine::LoftDraft, effect: &mut RetainedAllocationEffect<'_>)
    -> Result<(Option<PrismDraft>, RetainedCloneProgress), ValueError> {
    self.with_original_geometry_query(|kernel| kernel.step_loft_draft(handles, draft, effect))
}

/// 🎟️ Quotes the face and line path before a straight sweep's original read.
pub fn straight_sweep_depth(&self, profile: &str, path: &str) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.straight_sweep_depth(profile, path))
}

/// 🛤️ Stages an exact straight sweep with the certified solid topology cursors.
pub fn plan_straight_sweep(&self, profile: &str, path: &str) -> Result<semio_framework_3d_brep_engine::PlanarExtrusion, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_straight_sweep(profile, path)
}

/// 📦️ Stages the closed convex hull through the exact original solid publisher.
pub fn finish_point_hull(&self, draft: &mut semio_framework_3d_brep_engine::HullDraft) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.finish_point_hull(draft)
}

/// 🐚️ Captures a certified box and positive thickness in the original session.
pub fn plan_shell(&self, handle: &str, thickness: f64) -> Result<semio_framework_3d_brep_engine::ShellDraft, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.plan_shell(handle, thickness))
}

/// 🎟️ Quotes one original selection before adding a shell opening.
pub fn shell_face_depth(&self, draft: &semio_framework_3d_brep_engine::ShellDraft, handle: &str) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.shell_face_depth(draft, handle))
}

/// 🪟️ Admits a selected original box face without creating topology.
pub fn add_shell_face(&self, draft: &mut semio_framework_3d_brep_engine::ShellDraft, handle: &str) -> Result<(), ValueError> {
    self.with_original_geometry_query(|kernel| kernel.add_shell_face(draft, handle))
}

/// 🪚️ Stages the exact closed boundary from the selected box opening sides.
pub fn finish_shell(&self, draft: &semio_framework_3d_brep_engine::ShellDraft, handle: &str) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.finish_shell(draft, handle)
}

/// ↔️ Stages a signed sharp box offset through the original solid publisher.
pub fn plan_certified_solid_offset(&self, handle: &str, distance: f64) -> Result<semio_framework_3d_brep_engine::SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_certified_solid_offset(handle, distance)
}

/// 📍️ Reads an analytic point or tangent from the original session.
pub fn original_curve_sample(&self, handle: &str, parameter: f64, tangent: bool) -> Result<Vec3, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_curve_sample(handle, parameter, tangent))
}

/// 📏️ Reads the parameter span from the original session.
pub fn original_curve_span(&self, handle: &str) -> Result<f64, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_curve_span(handle))
}

/// 🪝️ Reads analytic curve curvature from the original session.
pub fn original_curve_curvature(&self, handle: &str, parameter: f64) -> Result<f64, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_curve_curvature(handle, parameter))
}

/// 🧭️ Reads an analytic surface point or normal from the original session.
pub fn original_surface_sample(&self, handle: &str, u: f64, v: f64, normal: bool) -> Result<Vec3, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_surface_sample(handle, u, v, normal))
}

/// 📏️ Starts a funded wire length query when the source is a wire.
pub fn plan_wire_length(&self, handle: &str) -> Result<Option<semio_framework_3d_brep_engine::WireLengthJob>, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.plan_wire_length(handle))
}

/// 🪙️ Quotes the current original wire measurement turn.
pub fn wire_length_demand(&self, handle: &str, query: &semio_framework_3d_brep_engine::WireLengthJob) -> Result<RetirementDemand, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.wire_length_demand(handle, query))
}

/// 📐️ Advances the original edge measurement.
pub fn step_wire_length(&self, handle: &str, query: &mut semio_framework_3d_brep_engine::WireLengthJob, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<f64>, RetainedCloneProgress), ValueError> {
    self.with_original_geometry_query(|kernel| kernel.step_wire_length(handle, query, effect))
}

/// 📎️ Measures a fixed analytic entity from the original session.
pub fn original_geometry_length(&self, handle: &str) -> Result<f64, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_geometry_length(handle))
}

/// 📍️ Projects a point onto original analytic geometry.
pub fn original_geometry_closest_point(&self, handle: &str, target: Vec3) -> Result<Vec3, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_geometry_closest_point(handle, target))
}

/// 🎟️ Plans a closest-point cursor over the original line-trimmed face.
pub fn plan_planar_closest(&self, handle: &str, target: Vec3) -> Result<Option<semio_framework_3d_brep_engine::PlanarClosestJob>, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.plan_planar_closest(handle, target))
}

/// 📏️ Quotes one original planar face cursor step.
pub fn planar_closest_demand(&self, handle: &str, job: &semio_framework_3d_brep_engine::PlanarClosestJob) -> Result<RetirementDemand, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.planar_closest_demand(handle, job))
}

/// ⏱️ Projects through one granted original planar face cursor step.
pub fn step_planar_closest(&self, handle: &str, job: &mut semio_framework_3d_brep_engine::PlanarClosestJob, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<semio_framework_3d_brep_engine::PlanarClosestOutput>, RetainedCloneProgress), ValueError> {
    self.with_original_geometry_query(|kernel| kernel.step_planar_closest(handle, job, effect))
}

/// 🪜️ Quotes the exact native rows needed for one original closest-point query.
pub fn original_closest_point_depth_demand(&self,handle:&str)->Result<usize,ValueError>{
    self.with_original_geometry_query(|kernel|kernel.original_closest_point_depth_demand(handle))
}

/// 🎲️ Projects onto a native analytic curve with its parameter and distance.
pub fn original_curve_closest_parameter(&self,handle:&str,target:Vec3)->Result<(f64,Vec3,f64),ValueError>{
    self.with_original_geometry_query(|kernel|kernel.original_curve_closest_parameter(handle,target))
}

/// 🏄️ Projects onto a native analytic surface with UV and distance.
pub fn original_surface_closest_uv(&self,handle:&str,target:Vec3)->Result<(f64,f64,Vec3,f64),ValueError>{
    self.with_original_geometry_query(|kernel|kernel.original_surface_closest_uv(handle,target))
}

/// 🏷️ Reads a persistent label from the original session.
pub fn original_geometry_label(&self, handle: &str) -> Result<u64, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_geometry_label(handle))
}

/// 🪜️ Quotes the path to one borrowed native geometry handle.
pub fn original_query_depth_demand(&self, handle: &str) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.tessellation_source_depth_demand(handle)).map(|depth| depth.max(1))
}

pub(super) fn with_original_geometry_query<T>(&self, query: impl FnOnce(&Brep) -> Result<T, ValueError>) -> Result<T, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    let kernel = self.kernel().try_lock().map_err(|_| ownership_busy())?;
    query(&kernel)
}

/// 💠️ Borrows live primitive provenance for an exact bounded volume when available.
pub fn original_primitive_volume(&self, handle: &str) -> Result<Option<f64>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    let kernel = self.kernel().try_lock().map_err(|_| ownership_busy())?;
    kernel.live_certified_volume(handle).map(|_| kernel.original_primitive_volume(handle)).transpose()
}

/// 📐️ Reads exact area from a live certified solid or planar conic face.
pub fn original_analytic_area(&self, handle: &str) -> Result<Option<f64>,ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_analytic_area(handle))
}

/// ⚖️ Reads exact center of mass from a live constructor-certified solid.
pub fn original_center_of_mass(&self, handle: &str) -> Result<Vec3,ValueError> {
    self.with_original_geometry_query(|kernel| {
        let point = kernel.live_curved_solid_properties(handle).map(|(_, center)| center)
            .or_else(|| kernel.live_solid_primitive(handle).map(|primitive| primitive.center_of_mass()))
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original center of mass requires a constructor-certified solid"))?;
        let center = point.to_array();
        if center.into_iter().all(f64::is_finite) { Ok(center) }
        else { Err(ValueError::literal(ValueRefusalKind::InvalidValue, "certified center of mass exceeds finite range")) }
    })
}

/// 🧊️ Starts exact polygon-face integration from an original live solid.
pub fn plan_planar_measure(&self, handle: &str, measure_area: bool) -> Result<semio_framework_3d_brep_engine::PlanarMeasureJob, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_planar_measure(handle,measure_area)
}

/// ⚖️ Starts exact retained center-of-mass integration for a planar solid or compound.
pub fn plan_planar_center_of_mass(&self,handle:&str)->Result<semio_framework_3d_brep_engine::PlanarMeasureJob,ValueError>{
    self.with_original_geometry_query(|kernel|kernel.plan_planar_center_of_mass(handle))
}

/// 📦️ Opens the retained cursor for an exact planar axis range.
pub fn plan_planar_bounds(&self,handle:&str)->Result<semio_framework_3d_brep_engine::PlanarMeasureJob,ValueError>{
    self.with_original_geometry_query(|kernel|kernel.plan_planar_bounds(handle))
}

/// 📐️ Quotes one original polygon-volume integration.
pub fn planar_measure_demand(&self, handle: &str, job: &semio_framework_3d_brep_engine::PlanarMeasureJob) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.planar_measure_demand(handle, job)
}

/// 🧮️ Advances exact polygon-volume integration with the original work and copy grant.
pub fn step_planar_measure(
    &self, handle: &str, job: &mut semio_framework_3d_brep_engine::PlanarMeasureJob, effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(Option<f64>, RetainedCloneProgress), ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_planar_measure(handle, job, effect)
}

/// 📐️ Quotes the next original index growth or commit for a fixed analytic entity.
pub fn analytic_publication_demand(&self, curve: &AnalyticPublication, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    self.geometry_publication_demand(curve, claim_staged, copy)
}

/// ⛲️ Publishes a fixed analytic entity under original session authority.
pub fn step_analytic_publication(
    &self,
    curve: &mut AnalyticPublication,
    live_key: &mut String,
    claim_key: &mut String,
    claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(bool, RetainedCloneProgress), ValueError> {
    self.step_geometry_publication(curve, live_key, claim_key, claim_staged, effect)
}

/// 🧺️ Starts an unpublished group of original native solid roots.
pub fn plan_compound(&self,count:usize)->Result<CompoundPublication,ValueError>{
    if self.is_closed(){return Err(ownership_busy());}
    self.kernel().try_lock().map_err(|_|ownership_busy())?.plan_compound(count)
}

pub fn compound_preparation_demand(&self,plan:&CompoundPublication,source:Option<&str>)->Result<RetirementDemand,ValueError>{
    self.with_original_geometry_query(|kernel|kernel.compound_preparation_demand(plan,source))
}

pub fn step_compound_preparation(&self,plan:&mut CompoundPublication,source:Option<&str>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
    self.with_original_geometry_query(|kernel|kernel.step_compound_preparation(plan,source,effect))
}

pub fn compound_publication_demand(&self,plan:&CompoundPublication,claim_staged:bool,copy:usize)->Result<RetirementDemand,ValueError>{
    self.geometry_publication_demand(plan,claim_staged,copy)
}

pub fn step_compound_publication(&self,plan:&mut CompoundPublication,live_key:&mut String,claim_key:&mut String,claim_staged:&mut bool,
    effect:&mut RetainedAllocationEffect<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{
    self.step_geometry_publication(plan,live_key,claim_key,claim_staged,effect)
}

/// 🎟️ Starts a private solid pattern against one live certified source.
pub fn plan_solid_pattern(&self, source: &str, recipe: SolidPatternRecipe) -> Result<SolidPatternPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_solid_pattern(source, recipe)
}

pub fn solid_pattern_preparation_demand(&self, plan: &SolidPatternPublication) -> Result<RetirementDemand, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.solid_pattern_preparation_demand(plan))
}

pub fn step_solid_pattern_preparation(&self, plan: &mut SolidPatternPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_solid_pattern_preparation(plan, effect)
}

pub fn solid_pattern_publication_demand(&self, plan: &SolidPatternPublication, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    self.geometry_publication_demand(plan, claim_staged, copy)
}

pub fn step_solid_pattern_publication(&self, plan: &mut SolidPatternPublication, live_key: &mut String, claim_key: &mut String, claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
    self.step_geometry_publication(plan, live_key, claim_key, claim_staged, effect)
}

pub fn solid_pattern_close_demand(&self, plan: &SolidPatternPublication) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.solid_pattern_close_demand(plan)
}

pub fn close_solid_pattern(&self, plan: &mut SolidPatternPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<semio_framework_value::RetainedCloneStep, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.close_solid_pattern(plan, effect)
}

pub fn plan_mesh_publication(&self, decoder: &mut MeshDecode) -> Result<MeshPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_mesh_publication(decoder)
}

pub fn mesh_preparation_demand(&self, plan: &MeshPublication) -> Result<RetirementDemand, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.mesh_preparation_demand(plan))
}

pub fn step_mesh_preparation(&self, plan: &mut MeshPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_mesh_preparation(plan, effect)
}

pub fn mesh_publication_demand(&self, plan: &MeshPublication, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    self.geometry_publication_demand(plan, claim_staged, copy)
}

pub fn step_mesh_publication(&self, plan: &mut MeshPublication, live_key: &mut String, claim_key: &mut String, claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
    self.step_geometry_publication(plan, live_key, claim_key, claim_staged, effect)
}

pub fn mesh_publication_close_demand(&self, plan: &MeshPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.mesh_publication_close_demand(plan, copy)
}

pub fn close_mesh_publication(&self, plan: &mut MeshPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<semio_framework_value::RetainedCloneStep, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.close_mesh_publication(plan, effect)
}

/// 🗃️ Stages an exact orthogonal cell arrangement.
pub fn plan_orthogonal_cells(&self, axes: [[f64;4];3], cells: u32) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_orthogonal_cells(axes,cells)
}
/// 📥️ Stages one exact solid boundary from a funded exchange cursor.
pub fn plan_exchange_boundary(&self, boundary: semio_framework_3d_brep_engine::SolidExchangeBoundary) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_exchange_boundary(boundary)
}

/// 〰️ Plans exact NURBS storage for a certified original cuboid.
pub fn plan_nurbs_cuboid(&self, source: &str) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_nurbs_cuboid(source)
}

/// 📥️ Opens an independent spline cuboid from its certified exchange geometry.
pub fn plan_nurbs_cuboid_primitive(&self, primitive: SolidPrimitive) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_nurbs_cuboid_primitive(primitive)
}

/// 📦️ Plans an unpublished original analytic solid without scanning session handles.
pub fn plan_solid_primitive(&self, primitive: SolidPrimitive) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_solid_primitive(primitive)
}

/// 🧱️ Starts an original solid cursor from a published circular profile and axial direction.
pub fn plan_circular_extrusion(&self, handle: &str, vector: Vec3) -> Result<Option<SolidPublication>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_circular_extrusion(handle, vector)
}

/// 📦️ Admits exact original extrusion of one horizontal rectangular profile.
pub fn plan_rectangular_extrusion(&self, handle: &str, vector: Vec3) -> Result<Option<SolidPublication>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_rectangular_extrusion(handle, vector)
}

/// 🎟️ Starts extraction of one original planar face or closed line wire.
pub fn plan_prism(&self, handle: &str, vector: Vec3) -> Result<Option<PrismDraft>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_prism(handle, vector)
}

/// 📏️ Quotes one original prism source turn or transfer.
pub fn prism_draft_demand(&self, source: &str, draft: &PrismDraft) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.prism_draft_demand(source, draft)
}

/// 📐️ Visits one admitted polygon corner or settles its exact solid recipe.
pub fn step_prism_draft(&self, source: &str, draft: &mut PrismDraft, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<SolidPublication>, RetainedCloneProgress), ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_prism_draft(source, draft, effect)
}

/// 📏️ Quotes the live operand lookups for retained planar sweeps.
pub fn planar_face_boundary_depth(&self, handle: &str) -> Result<usize, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.planar_face_boundary_depth(handle)
}

/// 🔄️ Plans the original native torus from a meridional circular face.
pub fn plan_circular_revolution(&self, handle: &str, origin: Vec3, direction: Vec3, angle: f64)
    -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_circular_revolution(handle, origin, direction, angle)
}

/// 🌀️ Starts the exact closed circular specialization of a helical sweep.
pub fn plan_closed_helical_sweep(&self, handle:&str, origin:Vec3, direction:Vec3, parameters:[f64;3]) -> Result<SolidPublication,ValueError> {
    if self.is_closed(){return Err(ownership_busy());}
    self.kernel().try_lock().map_err(|_|ownership_busy())?.plan_closed_helical_sweep(handle,origin,direction,parameters)
}

/// 🧱️ Starts the exact cylinder or prism for the original planar face thickness.
pub fn plan_planar_thickening(&self, handle: &str, thickness: f64) -> Result<semio_framework_3d_brep_engine::PlanarExtrusion, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_planar_thickening(handle, thickness)
}

/// 🧮️ Quotes the next arena action or transfer of the completed solid.
pub fn solid_primitive_demand(&self, solid: &SolidPublication, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    if solid.topology_ready() { return self.geometry_publication_demand(solid, claim_staged, copy); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.solid_primitive_demand(solid, copy)
}

/// 🏗️ Advances the unpublished solid by one original topology action.
pub fn step_solid_primitive_topology(&self, solid: &mut SolidPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_solid_primitive_topology(solid, effect)
}

/// 🪪️ Publishes the completed original analytic solid and its authority claim.
pub fn step_solid_primitive(
    &self,
    solid: &mut SolidPublication,
    live_key: &mut String,
    claim_key: &mut String,
    claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(bool, RetainedCloneProgress), ValueError> {
    self.step_geometry_publication(solid, live_key, claim_key, claim_staged, effect)
}

/// 🧾️ Quotes rollback of the next unpublished solid entity.
pub fn solid_primitive_close_demand(&self, solid: &SolidPublication) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.solid_primitive_close_demand(solid)
}

/// 🧹️ Releases one original ownership from the cancelled solid.
pub fn close_solid_primitive(&self, solid: &mut SolidPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.close_solid_primitive(solid, effect)
}

/// 🪷️ Plans original conic face topology from a complete planar boundary.
pub fn plan_conic_face(&self, handle: &str, xy_only: bool) -> Result<ConicFacePublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_conic_face(handle, xy_only)
}

/// ↔️ Plans an original conic face offset through the native publisher.
pub fn plan_conic_face_offset(&self, handle: &str, distance: f64) -> Result<ConicFacePublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_conic_face_offset(handle, distance)
}

/// 📏️ Quotes the next funded topology or index action for a conic face.
pub fn conic_face_demand(&self, face: &ConicFacePublication, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    if face.topology_ready() { return self.geometry_publication_demand(face, claim_staged, copy); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.conic_face_demand(face, copy)
}

/// 🧱️ Builds the next admitted conic topology record or link.
pub fn step_conic_face_topology(&self, face: &mut ConicFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_conic_face_topology(face, effect)
}

/// 🌱️ Transfers an admitted conic face under the exact original live and authority keys.
pub fn step_conic_face_publication(
    &self,
    face: &mut ConicFacePublication,
    live_key: &mut String,
    claim_key: &mut String,
    claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(bool, RetainedCloneProgress), ValueError> {
    self.step_geometry_publication(face, live_key, claim_key, claim_staged, effect)
}

/// 🧹️ Quotes removal of an unpublished conic face's next topology record.
pub fn conic_face_close_demand(&self, face: &ConicFacePublication) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.conic_face_close_demand(face)
}

/// ♻️ Retires one actual arena record of a cancelled conic face.
pub fn close_conic_face(&self, face: &mut ConicFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.close_conic_face(face, effect)
}

/// ✂️ Resolves each bounded original curve intersection under the live session lock.
pub fn original_curve_curve_contacts(&self, a: &str, b: &str, tolerance: f64) -> Result<[Option<Vec3>; 4], ValueError> {
    self.with_original_geometry_query(|kernel| kernel.original_curve_curve_contacts(a, b, tolerance))
}

/// 🪜️ Quotes both registered original curve or edge sources before their contact event.
pub fn original_intersection_curve_source_depth(&self, sources: &[&str]) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| sources.iter().try_fold(1, |depth, source| {
        Ok(depth.max(kernel.original_intersection_curve_source_depth(source)?))
    }))
}

/// 📐️ Quotes one exact source face or halfedge for cuboid sewing.
pub fn cuboid_sew_demand(&self, draft: &semio_framework_3d_brep_engine::CuboidSew, source: &str) -> Result<RetirementDemand, ValueError> {
    self.with_original_geometry_query(|kernel| {
        let (face, revision, depth) = kernel.cuboid_sew_source(source)?;
        if draft.next_face().is_none() {
            return Ok(RetirementDemand { depth, copy_bytes: 6 * std::mem::size_of::<f64>(), ..Default::default() });
        }
        draft.demand(kernel.representation(), face, revision, depth)
    })
}

/// 🪡️ Advances one source record or stages the fresh canonical solid under the same session lock.
pub fn step_cuboid_sew(&self, draft: &mut semio_framework_3d_brep_engine::CuboidSew, source: &str) -> Result<Option<SolidPublication>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    let mut kernel = self.kernel().try_lock().map_err(|_| ownership_busy())?;
    let (face, revision, _) = kernel.cuboid_sew_source(source)?;
    if draft.next_face().is_none() { return kernel.plan_solid_primitive(draft.cuboid(revision)?).map(Some); }
    draft.step(kernel.representation(), face, revision)?;
    Ok(None)
}

/// 🪟️ Resolves each bounded line or conic contact with a live analytic plane.
pub fn original_curve_plane_contacts(&self,curve:&str,surface:&str,tolerance:f64)->Result<[Option<Vec3>;4],ValueError>{
    self.with_original_geometry_query(|kernel|kernel.original_curve_plane_contacts(curve,surface,tolerance))
}

/// 🌐️ Plans the exact continuous intersection of two original analytic supports.
pub fn plan_original_surface_intersection(&self, a: &str, b: &str, tolerance: f64) -> Result<AnalyticPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_original_surface_intersection(a, b, tolerance)
}

/// 🪚️ Plans one exact finishing prism from a selected box member.
pub fn plan_box_finish(&self, shape: &str, member: &[&str], finish: semio_framework_3d_brep_engine::BoxFinish) -> Result<SolidPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_box_finish(shape,member,finish)
}

/// 📏️ Quotes the borrowed box and member paths.
pub fn box_finish_depth(&self, shape: &str, member: &[&str]) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.box_finish_depth(shape,member))
}

/// 🐚️ Captures the source of an ordered original topology list.
pub fn plan_topology_selection(&self, handle: &str, kind: TopologyList) -> Result<TopologySelection, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.plan_topology_selection(handle, kind))
}

/// 📐️ Quotes the next original topology lookup.
pub fn topology_member_depth(&self, plan: &TopologySelection, index: usize) -> Result<usize, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.topology_member_depth(plan, index))
}

/// 🏷️ Reads one original topology identity.
pub fn topology_member(&self, plan: &TopologySelection, index: usize) -> Result<TopologyAlias, ValueError> {
    self.with_original_geometry_query(|kernel| kernel.topology_member(plan, index))
}

fn topology_alias_claim_demand(own: &AuthorityClaims, kernel: &Brep, alias: &TopologyAlias, staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    let key = alias.handle();
    let claim_depth = own.handles.next_contains_depth_demand(key)?;
    match (staged, own.handles.get(key).copied()) {
        (false, None) => Self::claim_insertion_demand(own, key, copy),
        (false, Some(false)) => Ok(RetirementDemand { depth: claim_depth, ..Default::default() }),
        _ => {
            let live = kernel.topology_alias_demand(alias, copy)?;
            Ok(RetirementDemand { depth: live.depth.max(claim_depth), ..live })
        }
    }
}

/// 📋️ Quotes one view registration, reusing any existing live claim.
pub fn topology_alias_publication_demand(&self, alias: &TopologyAlias, staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    let claims = self.state.claims.try_lock().map_err(|_| ownership_busy())?;
    let kernel = self.kernel().try_lock().map_err(|_| ownership_busy())?;
    let own = claims.get(&self.state.authority).ok_or_else(ownership_busy)?;
    Self::topology_alias_claim_demand(own, &kernel, alias, staged, copy)
}

/// 🌱️ Publishes an original topology alias under the current geometry authority.
pub fn step_topology_alias_publication(&self, alias: &TopologyAlias, live_key: &mut String, claim_key: &mut String, staged: &mut bool, effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
    use semio_framework_value::retained_clone::retained_clone_grant_admits;
    if self.is_closed() { return Err(ownership_busy()); }
    let grant = effect.grant();
    let key = alias.handle();
    let empty = RetainedCloneProgress::default();
    let mut claims = self.state.claims.try_lock().map_err(|_| ownership_busy())?;
    let mut kernel = self.kernel().try_lock().map_err(|_| ownership_busy())?;
    let own = claims.get_mut(&self.state.authority).ok_or_else(ownership_busy)?;
    let previous = own.handles.get(key).copied();
    if live_key.as_str() != key || (!*staged && claim_key.as_str() != key) || *staged && previous != Some(false) {
        return Err(ValueError::invariant("original topology has inconsistent publication keys"));
    }
    if !*staged && previous == Some(false) { return Ok((false, empty)); }
    let demand = Self::topology_alias_claim_demand(own, &kernel, alias, *staged, grant.maximum_copy_bytes)?;
    if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((false, empty)); }
    if previous.is_none() {
        if demand.capacity_bytes != 0 {
            let receipt = own.handles.reserve_insert_step(key, grant).map_err(|(error, progress)| error.with_retained_progress(progress))?;
            return Ok((false, receipt));
        }
        let (old, receipt) = own.handles.insert_reserved(std::mem::take(claim_key), false, grant)
            .map_err(|(error, key, _)| { *claim_key = key; error })?;
        assert!(old.is_none());
        *staged = true;
        return Ok((false, receipt));
    }
    if demand.capacity_bytes != 0 { return kernel.reserve_topology_alias(alias, grant).map(|receipt| (false, receipt)); }
    let receipt = kernel.publish_topology_alias(alias, live_key, grant)?;
    if *staged {
        *own.handles.get_mut(key).expect("staged original topology claim") = true;
        *staged = false;
        self.claims_changed();
    }
    Ok((true, receipt))
}

/// 📍️ Plans a native vertex in this geometry authority.
pub fn plan_vertex_publication(&self, point: Vec3) -> Result<VertexPublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_vertex_publication(point)
}

/// 📐️ Quotes the next vertex topology or publication step.
pub fn vertex_publication_demand(&self, vertex: &VertexPublication, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    if vertex.topology_ready() { return self.geometry_publication_demand(vertex, claim_staged, copy); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.vertex_publication_demand(vertex, copy)
}

/// 🧱️ Stages the native vertex slot.
pub fn step_vertex_topology(&self, vertex: &mut VertexPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_vertex_topology(vertex, effect)
}

/// 🌱️ Publishes the vertex when its staged claim and live index are funded.
pub fn step_vertex_publication(
    &self,
    vertex: &mut VertexPublication,
    live_key: &mut String,
    claim_key: &mut String,
    claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(bool, RetainedCloneProgress), ValueError> {
    self.step_geometry_publication(vertex, live_key, claim_key, claim_staged, effect)
}

/// 🧹️ Quotes release of an unpublished vertex slot.
pub fn vertex_close_demand(&self, vertex: &VertexPublication) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.vertex_close_demand(vertex)
}

/// ♻️ Closes the vertex plan and rolls back any unpublished slot.
pub fn close_vertex(&self, vertex: &mut VertexPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.close_vertex(vertex, effect)
}

/// 🛤️ Plans exact line topology from one original polygonal source.
pub fn plan_polygon_wire(&self, recipe: PolygonWireRecipe) -> Result<PolygonWirePublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_polygon_wire(recipe)
}

/// 📐️ Quotes the next wire topology or index event.
pub fn polygon_wire_demand(&self, wire: &PolygonWirePublication, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    if wire.topology_ready() { return self.geometry_publication_demand(wire, claim_staged, copy); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.polygon_wire_demand(wire, copy)
}

/// 🧱️ Advances the admitted wire owner with at most one original polyline point.
pub fn step_polygon_wire_topology(&self, wire: &mut PolygonWirePublication, point: Option<Vec3>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_polygon_wire_topology(wire, point, effect)
}

/// 🌱️ Transfers a completed wire under the original live and claim keys.
pub fn step_polygon_wire_publication(
    &self,
    wire: &mut PolygonWirePublication,
    live_key: &mut String,
    claim_key: &mut String,
    claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(bool, RetainedCloneProgress), ValueError> {
    self.step_geometry_publication(wire, live_key, claim_key, claim_staged, effect)
}

/// 🧹️ Quotes retirement of the next unpublished wire owner.
pub fn polygon_wire_close_demand(&self, wire: &PolygonWirePublication) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.polygon_wire_close_demand(wire)
}

/// ♻️ Closes one unpublished wire record or buffer.
pub fn close_polygon_wire(&self, wire: &mut PolygonWirePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.close_polygon_wire(wire, effect)
}

/// 🎟️ Plans a polygon face from a complete bound original wire.
pub fn plan_polygon_face(&self, source: &str, xy_only: bool) -> Result<Option<PolygonFacePublication>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_polygon_face(source, xy_only)
}

/// 🔒️ Admits a planar point-list face with a private closed polygon boundary.
pub fn plan_polygon_face_points(&self, vertices: usize) -> Result<PolygonFacePublication, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_polygon_face_points(vertices)
}

/// ↔️ Admits an original polygon offset under the session authority.
pub fn plan_polygon_face_offset(&self, source: &str, distance: f64) -> Result<Option<PolygonFacePublication>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_polygon_face_offset(source, distance)
}

/// 🔁️ Admits a retained affine image of the original planar line ring.
pub fn plan_polygon_face_transform(&self, source: &str, transform: semio_framework_3d_brep_engine::AnalyticTransform) -> Result<Option<PolygonFacePublication>, ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_polygon_face_transform(source, transform)
}

/// 📐️ Quotes the next polygon face validation, topology or index turn.
pub fn polygon_face_demand(&self, face: &PolygonFacePublication, source: Option<&str>, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    if face.topology_ready() { return self.geometry_publication_demand(face, claim_staged, copy); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.polygon_face_topology_demand(face, source)
}

/// 🧱️ Advances one admitted polygon face validation or topology action.
pub fn step_polygon_face_topology(&self, face: &mut PolygonFacePublication, source: Option<&str>, point: Option<[f64; 3]>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.step_polygon_face_topology(face, source, point, effect)
}

/// 🌱️ Transfers a completed polygon face through the original session authority.
pub fn step_polygon_face_publication(
    &self,
    face: &mut PolygonFacePublication,
    live_key: &mut String,
    claim_key: &mut String,
    claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(bool, RetainedCloneProgress), ValueError> {
    self.step_geometry_publication(face, live_key, claim_key, claim_staged, effect)
}

/// 🧹️ Quotes the next unpublished record or original rollback journal release.
pub fn polygon_face_close_demand(&self, face: &PolygonFacePublication) -> Result<RetirementDemand, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.polygon_face_close_demand(face)
}

/// ♻️ Closes an unpublished face or releases the journal of the committed topology.
pub fn close_polygon_face(&self, face: &mut PolygonFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
    self.kernel().try_lock().map_err(|_| ownership_busy())?.close_polygon_face(face, effect)
}

fn claim_insertion_demand(
    own: &AuthorityClaims,
    key: &str,
    copy: usize,
) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand {
        capacity_bytes: own.handles.next_insert_capacity_byte_demand(key, copy)?,
        depth: own.handles.next_insert_depth_demand(key)?,
        ..Default::default()
    })
}

fn geometry_publication_demand(&self, publication: &impl GeometryPublication, claim_staged: bool, copy: usize) -> Result<RetirementDemand, ValueError> {
    let claims = self.state.claims.try_lock().map_err(|_| ownership_busy())?;
    let kernel = self.kernel().try_lock().map_err(|_| ownership_busy())?;
    let own = claims.get(&self.state.authority).ok_or_else(ownership_busy)?;
    if !claim_staged { return Self::claim_insertion_demand(own, publication.handle(), copy); }
    let live = publication.demand(&kernel, copy)?;
    let claim_depth = own.handles.next_contains_depth_demand(publication.handle())?;
    Ok(RetirementDemand { depth: live.depth.max(claim_depth), ..live })
}

fn step_geometry_publication(
    &self,
    publication: &mut impl GeometryPublication,
    live_key: &mut String,
    claim_key: &mut String,
    claim_staged: &mut bool,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(bool, RetainedCloneProgress), ValueError> {
    use semio_framework_value::retained_clone::retained_clone_grant_admits;
    let grant = effect.grant();
    let empty = RetainedCloneProgress::default();
    if self.is_closed() { return Err(ownership_busy()); }
    let mut claims = self.state.claims.try_lock().map_err(|_| ownership_busy())?;
    let mut kernel = self.kernel().try_lock().map_err(|_| ownership_busy())?;
    let own = claims.get_mut(&self.state.authority).ok_or_else(ownership_busy)?;
    let key = publication.handle();
    if live_key.as_str() != key || (!*claim_staged && claim_key.as_str() != key) || *claim_staged != own.handles.contains_key(key) {
        return Err(ValueError::invariant("original geometry has inconsistent publication keys"));
    }
    if !*claim_staged {
        let demand = Self::claim_insertion_demand(own, key, grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((false, empty)); }
        if demand.capacity_bytes != 0 {
            let receipt = own.handles.reserve_insert_step(key, grant).map_err(|(error, progress)| error.with_retained_progress(progress))?;
            return Ok((false, receipt));
        }
        let (previous, receipt) = own.handles.insert_reserved(std::mem::take(claim_key), false, grant).map_err(|(error, key, _)| {
            *claim_key = key;
            error
        })?;
        assert!(previous.is_none());
        *claim_staged = true;
        return Ok((false, receipt));
    }
    let mut live = publication.demand(&kernel, grant.maximum_copy_bytes)?;
    live.depth = live.depth.max(own.handles.next_contains_depth_demand(key)?);
    if !retained_clone_grant_admits(grant, live) || !effect.performed(1)? { return Ok((false, empty)); }
    if live.capacity_bytes > 0 { return publication.reserve(&mut kernel, grant).map(|receipt| (false, receipt)); }
    let claim = own.handles.get_mut(key).expect("original staged geometry claim");
    let receipt = publication.publish(&mut kernel, std::mem::take(live_key), grant).map_err(|(error, key)| {
        *live_key = key;
        error
    })?;
    *claim = true;
    *claim_staged = false;
    self.claims_changed();
    Ok((true, receipt))
}

/// 🪓️ Admits two unpublished solids in signed plane order.
pub fn plan_plane_split(&self, source: &str, origin: [f64; 3], normal: [f64; 3]) -> Result<[SolidPublication; 2], ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    self.kernel().try_lock().map_err(|_| ownership_busy())?.plan_plane_split(source, origin, normal)
}

pub fn solid_pair_demand(&self, pair: &[SolidPublication; 2], staged: [bool; 2], copy: usize) -> Result<RetirementDemand, ValueError> {
    if let Some(i) = staged.iter().position(|staged| !staged) { return self.geometry_publication_demand(&pair[i], false, copy); }
    self.geometry_batch_demand(pair.iter().map(SolidPublication::handle))
}

/// 🤝️ Stages claims independently, then commits both certified roots under the same locks.
pub fn step_solid_pair(&self, pair: &mut [SolidPublication; 2], live: &mut [String; 2], claim: &mut [String; 2], staged: &mut [bool; 2], effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
    if let Some(i) = staged.iter().position(|staged| !staged) { return self.step_geometry_publication(&mut pair[i], &mut live[i], &mut claim[i], &mut staged[i], effect); }
    self.step_geometry_batch(pair, live, staged, 2, |pair, i| pair[i].handle(), Brep::publish_solid_pair, effect)
}

pub fn polygon_section_demand(&self, section: &PolygonSectionSet, staged: [bool; SECTION_FACE_LIMIT], copy: usize) -> Result<RetirementDemand, ValueError> {
    let count = section.count();
    if let Some(i) = staged[..count].iter().position(|staged| !staged) { return self.geometry_publication_demand(&section.face(i).0, false, copy); }
    self.geometry_batch_demand((0..count).map(|i| section.face(i).0.handle()))
}

/// 🪷️ Stages the private Section claims and commits the complete face list.
pub fn step_polygon_section(&self, section: &mut PolygonSectionSet, live: &mut [String; SECTION_FACE_LIMIT], claim: &mut [String; SECTION_FACE_LIMIT], staged: &mut [bool; SECTION_FACE_LIMIT], effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
    let count = section.count();
    if let Some(i) = staged[..count].iter().position(|staged| !staged) { return self.step_geometry_publication(&mut section.face_mut(i).0, &mut live[i], &mut claim[i], &mut staged[i], effect); }
    self.step_geometry_batch(section, live, staged, count, |section, i| section.face(i).0.handle(), Brep::publish_polygon_section, effect)
}

fn geometry_batch_demand<'a>(&self, handles: impl ExactSizeIterator<Item = &'a str>) -> Result<RetirementDemand, ValueError> {
    let claims = self.state.claims.try_lock().map_err(|_| ownership_busy())?;
    let own = claims.get(&self.state.authority).ok_or_else(ownership_busy)?;
    let mut demand = self.kernel().try_lock().map_err(|_| ownership_busy())?.geometry_batch_demand(handles.len())?;
    for key in handles { demand.depth = demand.depth.max(own.handles.next_contains_depth_demand(key)?); }
    Ok(demand)
}

fn step_geometry_batch<T, const N: usize>(
    &self, batch: &mut T, keys: &mut [String; N], staged: &mut [bool; N], count: usize,
    handle: fn(&T, usize) -> &str, publish: fn(&mut Brep, &mut T, &mut [String; N], RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError>,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<(bool, RetainedCloneProgress), ValueError> {
    if self.is_closed() { return Err(ownership_busy()); }
    let mut claims = self.state.claims.try_lock().map_err(|_| ownership_busy())?;
    let mut kernel = self.kernel().try_lock().map_err(|_| ownership_busy())?;
    let own = claims.get_mut(&self.state.authority).ok_or_else(ownership_busy)?;
    let mut demand = kernel.geometry_batch_demand(count)?;
    for i in 0..count {
        let key = handle(batch, i);
        demand.depth = demand.depth.max(own.handles.next_contains_depth_demand(key)?);
        if own.handles.get(key) != Some(&false) { return Err(ValueError::invariant("original geometry batch claims changed before commit")); }
    }
    let grant = effect.grant();
    if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok((false, Default::default())); }
    if let Some(receipt) = kernel.reserve_geometry_batch(count, effect)? { return Ok((false, receipt)); }
    if effect.remaining_work_units()? == 0 || !effect.performed(1)? { return Ok((false, Default::default())); }
    let receipt = publish(&mut kernel, batch, keys, grant)?;
    for (i, staged) in staged.iter_mut().enumerate().take(count) {
        *own.handles.get_mut(handle(batch, i)).expect("admitted geometry batch claim") = true;
        *staged = false;
    }
    self.claims_changed();
    Ok((true, receipt))
}

fn staged_claim_demand(own: &AuthorityClaims, key: &str) -> Result<RetirementDemand, ValueError> {
    let stored = own.handles.get_key_value(key).ok_or_else(|| ValueError::invariant("original staged geometry claim disappeared"))?.0;
    Ok(RetirementDemand {
        depth: own.handles.next_insert_depth_demand(key)?,
        release_bytes: stored.capacity(),
        ..Default::default()
    })
}

/// 🎟️ Quotes the exact stored key and index path of an unpublished geometry claim.
pub fn staged_geometry_claim_close_demand(&self, key: &str) -> Result<RetirementDemand, ValueError> {
    let claims = self.state.claims.try_lock().map_err(|_| ownership_busy())?;
    let own = claims.get(&self.state.authority).ok_or_else(ownership_busy)?;
    Self::staged_claim_demand(own, key)
}

/// 🧹️ Releases one unpublished claim without allocating or touching other session keys.
pub fn close_staged_geometry_claim(&self, key: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    use semio_framework_value::retained_clone::retained_clone_grant_admits;
    let mut claims = self.state.claims.try_lock().map_err(|_| ownership_busy())?;
    let own = claims.get_mut(&self.state.authority).ok_or_else(ownership_busy)?;
    let demand = Self::staged_claim_demand(own, key)?;
    if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(Default::default()); }
    let original = own.handles.remove_entry(key).expect("original staged geometry claim");
    drop(original);
    self.claims_changed();
    Ok(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })
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
/// 🧾️ Keeps the actual import cleanup work on lock refusal and producer failure.
pub fn close_mesh_import_original(&self,cursor:&mut semio_framework_3d_brep_engine::MeshImportCursor,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome{
 let mut guard=match self.kernel().try_lock(){Ok(guard)=>guard,Err(_)=>return semio_framework_value::RetainedWorkOutcome{result:Err(ownership_busy()),work:0}};guard.close_mesh_import_original(cursor,effect)
}
pub fn close_mesh_import(&self,cursor:&mut semio_framework_3d_brep_engine::MeshImportCursor,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{self.close_mesh_import_original(cursor,effect).result}
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
pub fn cancel_step(&self,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<RetainedCloneStep,ValueError>{
    *receipt=Default::default();let result=self.cancel_step_retained(effect,receipt);if let Ok(step)=&result{*receipt=step.progress();}result
}
fn cancel_step_retained(&self,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<RetainedCloneStep,ValueError>{
    let grant=effect.grant();
    let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original_mut().ok_or_else(ownership_busy)?;
    if let Some(closing)=&mut registry.admission_closing{let result=settled_step(closing.step(grant.maximum_copy_bytes,effect));*receipt=closing.step_progress();let step=result?;if closing.terminal_is_empty(){registry.admission_closing=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
    if let Some(closing)=&mut registry.closing{let result=settled_step(closing.step(grant.maximum_copy_bytes,effect));*receipt=closing.step_progress();let step=result?;if closing.terminal_is_empty(){registry.closing=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
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
pub fn close_retired_tessellation_step(&self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {
    let grant=effect.grant();
    let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
    let Some(registry)=owner.original_mut() else {return Err(ownership_busy())};
    if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
    if let Some(closing)=registry.closing.as_mut() {
        if closing.terminal_is_empty() {registry.closing=None;return Ok(ownership_advance());}
        return settled_step(closing.step(grant.maximum_copy_bytes,effect)).map(|step|RetainedCloneStep::Progress(step.progress()));
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
        if matches!(neural_engine::retirement::cold_effect(grant,|effect|self.close_retired_tessellation_step(effect)).map_err(|error|error.to_string())?.map_err(|error|error.to_string())?,RetainedCloneStep::Complete(_)) {return Ok(());}
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
pub(super) fn tessellation_work_units(&self,identity:semio_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64)->usize {
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
struct SessionPort {pub(super) authority:SessionCapture,anchor:SessionCapture,retaining:RetainedOwnerGate<Option<SessionRetention>>,retain_receipt:RetainedCloneProgress,cancel_receipt:RetainedCloneProgress}
impl SessionPort {fn retained_demand(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{let owner=self.retaining.try_lock().map_err(|_|ownership_busy())?;owner.as_ref().map_or(Ok(Default::default()),|owner|owner.demand(self.authority.session.as_ref().ok_or_else(ownership_busy)?,copy))}}
impl Drop for SessionPort {fn drop(&mut self) {if !std::thread::panicking() {assert!(self.retaining.get_mut().is_none()&&self.authority.terminal_is_empty() && self.anchor.terminal_is_empty(),"geometry port requires terminal-empty retirement");}}}
impl semio_os_flow::geometry::GeometryPort for SessionPort {
    fn begin_retain(&mut self,handles:Vec<String>)->Result<(),(ValueError,Vec<String>)>{if self.retaining.get_mut().is_some(){return Err((ownership_busy(),handles));}let Some(session)=self.authority.session.as_ref()else{return Err((ownership_busy(),handles));};*self.retaining.get_mut()=Some(SessionRetention::new(session,handles)?);Ok(())}
    fn retain_terminal_is_empty(&self)->bool{self.retaining.try_lock().is_ok_and(|owner|owner.is_none())}
    fn next_retain_copy_byte_demand(&self)->Result<usize,ValueError>{self.retained_demand(0).map(|demand|demand.copy_bytes)}
    fn next_retain_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.retained_demand(copy).map(|demand|demand.capacity_bytes)}
    fn next_retain_release_byte_demand(&self)->Result<usize,ValueError>{self.retained_demand(0).map(|demand|demand.release_bytes)}
    fn next_retain_depth_demand(&self)->Result<usize,ValueError>{self.retained_demand(0).map(|demand|demand.depth)}
    fn retain_step_progress(&self)->RetainedCloneProgress{self.retain_receipt}
    fn retain_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{self.retain_receipt=Default::default();let Some(owner)=self.retaining.get_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};let result=owner.step(self.authority.session.as_ref().ok_or_else(ownership_busy)?,effect);self.retain_receipt=owner.normal_step_progress();let step=result?;if owner.terminal_is_empty(){*self.retaining.get_mut()=None;Ok(RetainedCloneStep::Complete(step.progress()))}else{Ok(RetainedCloneStep::Progress(step.progress()))}}
    fn tessellation_demands(&self,identity:semio_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.authority.session.as_ref().ok_or_else(ownership_busy)?.tessellation_demands(identity,handle,tolerance,copy)}
    fn tessellate_step(&self,identity:semio_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(semio_os_flow::geometry::GeometryStep,semio_os_flow::geometry::GeometryWorkProgress),semio_os_flow::geometry::GeometryWorkFailure>{self.authority.session.as_ref().ok_or_else(ownership_busy).map_err(|primary|semio_os_flow::geometry::GeometryWorkFailure::new(primary,0))?.tessellate_step(identity,handle,tolerance,cx)}
    fn cancel_tessellation_demands(&self,identity:semio_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.authority.session.as_ref().ok_or_else(ownership_busy)?.cancel_tessellation_demands(identity,handle,tolerance,copy)}
    fn cancel_tessellation_step(&self,identity:semio_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(RetainedCloneStep,semio_os_flow::geometry::GeometryWorkProgress),semio_os_flow::geometry::GeometryWorkFailure>{self.authority.session.as_ref().ok_or_else(ownership_busy).map_err(|primary|semio_os_flow::geometry::GeometryWorkFailure::new(primary,0))?.cancel_tessellation_step(identity,handle,tolerance,cx)}
    fn dispose(&self,handle:&str)->Result<(),String> {self.authority.session.as_ref().ok_or("geometry session is closed")?.dispose_geometry(handle)}
    fn begin_cancel(&mut self)->Result<(),ValueError>{self.authority.session.as_ref().ok_or_else(ownership_busy)?.begin_cancel()?;if let Some(owner)=self.retaining.get_mut(){owner.cancel();}Ok(())}
    fn cancel_terminal_is_empty(&self)->bool{self.retain_terminal_is_empty()&&self.authority.session.as_ref().is_none_or(Session::cancel_terminal_is_empty)}
    fn next_cancel_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_copy_byte_demand();}self.authority.session.as_ref().map_or(Ok(0),Session::next_cancel_copy_byte_demand)}
    fn next_cancel_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_capacity_byte_demand(copy);}self.authority.session.as_ref().map_or(Ok(0),|session|session.next_cancel_capacity_byte_demand(copy))}
    fn next_cancel_release_byte_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_release_byte_demand();}self.authority.session.as_ref().map_or(Ok(0),Session::next_cancel_release_byte_demand)}
    fn next_cancel_depth_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_depth_demand();}self.authority.session.as_ref().map_or(Ok(0),Session::next_cancel_depth_demand)}
    fn cancel_step_progress(&self)->RetainedCloneProgress{self.cancel_receipt}
    fn cancel_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{self.cancel_receipt=Default::default();if !self.retain_terminal_is_empty(){let result=self.retain_step(effect);self.cancel_receipt=self.retain_receipt;return result.map(|step|RetainedCloneStep::Progress(step.progress()));}let result=self.authority.session.as_ref().map_or(Ok(RetainedCloneStep::Complete(Default::default())),|session|session.cancel_step(effect,&mut self.cancel_receipt));if let Ok(step)=&result{self.cancel_receipt=step.progress();}result}
    fn begin_close(&self) {if let Ok(mut retaining)=self.retaining.try_lock(){if let Some(owner)=retaining.as_mut(){owner.cancel();}}if let Some(authority)=self.authority.session.as_ref() {authority.begin_close();}}
    fn terminal_is_empty(&self)->bool {self.retain_terminal_is_empty()&&self.authority.terminal_is_empty() && self.anchor.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_retain_copy_byte_demand();}if self.authority.terminal_is_empty() {self.anchor.next_close_copy_byte_demand()}else {self.authority.next_close_copy_byte_demand()}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_retain_capacity_byte_demand(copy);}if self.authority.terminal_is_empty() {self.anchor.next_close_capacity_byte_demand(copy)}else {self.authority.next_close_capacity_byte_demand(copy)}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_retain_release_byte_demand();}if self.authority.terminal_is_empty() {self.anchor.next_close_release_byte_demand()}else {self.authority.next_close_release_byte_demand()}}
    fn next_depth_demand(&self)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_retain_depth_demand();}if self.authority.terminal_is_empty() {self.anchor.next_close_depth_demand()}else {self.authority.next_close_depth_demand()}}
    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.retain_terminal_is_empty(){return self.retain_step(effect).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if !self.authority.terminal_is_empty() {return self.authority.close_step(effect).map(|step|RetainedCloneStep::Progress(step.progress()));}
        self.anchor.close_step(effect)
    }
}
