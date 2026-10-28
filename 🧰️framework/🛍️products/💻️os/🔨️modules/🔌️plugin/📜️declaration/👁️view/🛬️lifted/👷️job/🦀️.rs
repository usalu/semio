use semio_framework_value::retained_clone::RetainedCloneProgress;
use std::mem::ManuallyDrop;
use semio_framework_job::StepContext;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure;

/// 🧳️ Object-safe app-instance operation state retained and closed by [VcsArtifactApp].
pub trait ArtifactInstanceOperationOwner: std::any::Any + Send {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    /// 📏️ Exact per-axis grant the owner's next maintenance or close turn needs.
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn maintenance_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>;
    fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct EmptyArtifactInstanceOperationOwner;

impl ArtifactInstanceOperationOwner for EmptyArtifactInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn maintenance_step(&mut self,_cx:&mut StepContext<'_>,_loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        Ok(PluginLifecycleStep::Complete(Default::default()))
    }

    fn close_step(&mut self,_cx:&mut StepContext<'_>,_loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        Ok(PluginLifecycleStep::Complete(Default::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        true
    }
}

/// 🧬️ Mandatory concrete session retirement retains original typed custody until full paid close.
trait OriginalInferenceSessionOwner:Send {
 fn original_mut(&mut self)->Option<&mut dyn std::any::Any>;
 fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}

struct OriginalInferenceSession<T:semio_framework_value::retirement::RetireOwned>{body:semio_framework_value::retirement::controlled::ControlledRetirement<T>}

impl<T:semio_framework_value::retirement::RetireOwned> OriginalInferenceSessionOwner for OriginalInferenceSession<T>{
 fn original_mut(&mut self)->Option<&mut dyn std::any::Any>{self.body.original_mut().map(|value|value as &mut dyn std::any::Any)}
 fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.body.selected_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>{self.body.step(grant)}
 fn terminal_is_empty(&self)->bool{self.body.terminal_is_empty()}
}

/// 🔮️ Original mounted sessions carry their mandatory typed close producer beside the same value.
#[derive(Default)]
pub struct InstanceInferenceSessions {
 sessions:semio_framework_value_persistent::list::PagedList<(&'static str,Box<dyn OriginalInferenceSessionOwner>),{usize::MAX}>,
 closed:bool,
 closing:bool,
}

impl InstanceInferenceSessions {
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.sessions.terminal_is_empty()}
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some((_,session))=self.sessions.last(){return if session.terminal_is_empty(){Ok(RetirementDemand{release_bytes:std::mem::size_of_val(session.as_ref()),depth:self.sessions.next_pop_depth_demand().map_err(ValueError::from)?,..Default::default()})}else{session.retirement_demands(body)}}
  Ok(RetirementDemand{release_bytes:self.sessions.next_release_allocation_bytes().map_err(ValueError::from)?,depth:if self.sessions.terminal_is_empty(){usize::from(!self.closed)}else{self.sessions.next_release_depth_demand().map_err(ValueError::from)?},..Default::default()})
 }
 /// 🎟️ Advances only the same original data retirement after all independent currencies admit it.
 pub fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>{
  use semio_framework_value::RetainedCloneStep;let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.closing=true;
  if let Some((_,session))=self.sessions.last_mut(){
   if session.terminal_is_empty(){let bytes=std::mem::size_of_val(session.as_ref());drop(self.sessions.pop());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}))}
   return session.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))
  }
  if !self.sessions.terminal_is_empty(){let released=self.sessions.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(released.progressed),released_bytes:released.released_allocation_bytes,..Default::default()}))}
  self.closed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
  let demand=self.retirement_demands(loan.maximum_copy_bytes)?;
  semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|_,grant|{let step=self.close_granted(grant)?;let terminal=self.terminal_is_empty();Ok((match step{semio_framework_value::RetainedCloneStep::Complete(progress)=>PluginLifecycleStep::Complete(progress),semio_framework_value::RetainedCloneStep::Progress(progress)=>PluginLifecycleStep::Progress(progress)},terminal))})
 }

}

/// 📏️ Quotes the original shared container without reconstructing its mounted Handle.
pub fn original_inference_close_demands(original:&std::sync::Arc<std::sync::Mutex<InstanceInferenceSessions>>,body:usize)->Result<RetirementDemand,ValueError>{match original.try_lock(){Ok(sessions)=>sessions.retirement_demands(body),Err(std::sync::TryLockError::WouldBlock)=>Ok(Default::default()),Err(std::sync::TryLockError::Poisoned(_))=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred session owner is poisoned"))}}

/// 🎟️ Closes the same shared inference owner with the actual original context.
pub fn original_inference_close_step(original:&std::sync::Arc<std::sync::Mutex<InstanceInferenceSessions>>,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{match original.try_lock(){Ok(mut sessions)=>sessions.close_step(cx,loan),Err(std::sync::TryLockError::WouldBlock)=>Ok(PluginLifecycleStep::Blocked{reason:"original inferred session remains borrowed"}),Err(std::sync::TryLockError::Poisoned(_))=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred session owner is poisoned").into())}}

/// 🧳️ Cloneable capability for one concrete app instance's retained operation owner and its inference sessions.
#[derive(Clone)]
pub struct ArtifactInstanceOperationOwnerHandle {
    inner: std::sync::Arc<semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellV1<dyn ArtifactInstanceOperationOwner>>,
    inference: std::sync::Arc<std::sync::Mutex<InstanceInferenceSessions>>,
}

/// 🧭️ Retains one original caller's mounted lookup and donated typed session between funded turns.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct ArtifactInferenceSessionAccess<T:semio_framework_value::retirement::RetireOwned>{
 original:Option<T>,
 source:usize,
 operation:u64,
 generation:u64,
 name_pointer:usize,
 name_length:usize,
 index:usize,
 byte:usize,
 phase:u8,
}

impl<T:semio_framework_value::retirement::RetireOwned> Default for ArtifactInferenceSessionAccess<T>{
 fn default()->Self{Self{original:None,source:0,operation:0,generation:0,name_pointer:0,name_length:0,index:0,byte:0,phase:0}}
}

impl<T:semio_framework_value::retirement::RetireOwned> ArtifactInferenceSessionAccess<T>{
 /// 👁️ Borrows the defining original source while its typed frame awaits admission.
 pub fn original(&self)->Option<&T>{self.original.as_ref()}
 /// 🪹️ Pending typed ownership must close before the access frontier can retire.
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
}

fn original_inference_metadata(cx:&mut StepContext<'_>,copy:usize,depth:usize)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 let grant=cx.retained_grant();
 if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth<depth{return Ok(false)}
 cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;Ok(true)
}

impl ArtifactInstanceOperationOwnerHandle {
    /// 📏️ Quotes the defining two shared original owners without constructing either source early.
    pub fn original_birth_demands()->Result<RetirementDemand,ValueError>{let capacity_bytes=semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellV1<dyn ArtifactInstanceOperationOwner>>().checked_add(semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<std::sync::Mutex<InstanceInferenceSessions>>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original operation owner shared extent overflow"))?;Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<Self>()+2*std::mem::size_of::<Box<dyn ArtifactInstanceOperationOwner>>(),capacity_bytes,depth:2,..Default::default()})}
    /// 🌱️ An explicitly authored empty operation owner still admits its actual inference and mutable-owner authorities.
    pub fn admit_original_empty(cx:&mut StepContext<'_>,output:&mut Option<Self>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if output.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original empty operation owner requires an empty recipient").into())}let demand=Self::original_birth_demands()?;semio_framework_job::admit_original_construction(cx,output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:0,depth:demand.depth},|_|Ok((Self::new(Box::new(EmptyArtifactInstanceOperationOwner)),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})))?;Ok(output.is_some())}
    /// 🧳️ Wraps an owner the CALLER built. It confers no authority of its own — `with_mut`
    /// downcasts to the concrete type the caller passed in — so an app's own laws can build the
    /// handle their retained work expects instead of reaching into a live instance
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn new(owner: Box<dyn ArtifactInstanceOperationOwner>) -> Self {
        Self { inner: std::sync::Arc::new(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellV1::new(owner)), inference: Default::default() }
    }

    /// 🔮️ Explicit synchronous semantic facade of the same registry and typed close owner.
    pub fn with_inference_session_cold<T: semio_framework_value::retirement::RetireOwned, R>(&self, name: &'static str, init: impl FnOnce() -> T, apply: impl FnOnce(&mut T) -> Result<R, Fault>) -> Result<R, Fault> {
        if !T::controlled_retirement_supported(){return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.instance-inference-retirement"),"mounted inference session requires its defining controlled retirement producer"))}
        let mut sessions = self.inference.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if sessions.closed || sessions.closing {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-inference-closed"), "app-instance inference sessions were already closed"));
        }
        let index=match sessions.sessions.iter().position(|(known,_)|*known==name){Some(index)=>index,None=>{let body=semio_framework_value::retirement::controlled::ControlledRetirement::new(init()).unwrap_or_else(|_|panic!("mounted inference session must declare its typed retirement"));sessions.sessions.push((name,Box::new(OriginalInferenceSession{body})));sessions.sessions.len()-1}};
        let session = sessions.sessions[index].1.original_mut().and_then(|value|value.downcast_mut::<T>()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-inference-type"), "app-instance inference session type does not match its name"))?;
        apply(session)
    }

    /// 🎟️ Looks up and admits the same defining typed session with the actual caller and registry.
    pub fn with_inference_session<T:semio_framework_value::retirement::RetireOwned,R>(&self,name:&'static str,access:&mut ArtifactInferenceSessionAccess<T>,cx:&mut StepContext<'_>,init:impl FnOnce(&mut StepContext<'_>)->Result<Option<T>,semio_framework_job::OriginalTurnFailure>,apply:impl FnOnce(&mut T,&mut StepContext<'_>)->R)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
     use semio_framework_job::OriginalTurnFailure;
     use semio_framework_value::ValueRefusalKind;
     cx.original_source_parent()?;
     let refused=|message|ValueError::literal(ValueRefusalKind::InvariantViolated,message);
     let source=std::sync::Arc::as_ptr(&self.inference)as usize;
     if access.phase!=0&&(access.source!=source||access.operation!=cx.operation().0||access.generation!=cx.generation().0||access.name_pointer!=name.as_ptr()as usize||access.name_length!=name.len()){return Err(refused("original mounted inference access source or caller changed").into())}
     if cx.is_cancelled()||cx.should_yield(){return Ok(None)}
     if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"mounted inference source requires its defining controlled retirement producer").into())}
     let mut sessions=match self.inference.try_lock(){Ok(sessions)=>sessions,Err(std::sync::TryLockError::WouldBlock)=>return Ok(None),Err(std::sync::TryLockError::Poisoned(_))=>return Err(refused("original mounted inference registry is poisoned").into())};
     if sessions.closed||sessions.closing{return Err(refused("original mounted inference registry is closing").into())}
     match access.phase{
      0=>{if original_inference_metadata(cx,0,1)?{access.source=source;access.operation=cx.operation().0;access.generation=cx.generation().0;access.name_pointer=name.as_ptr()as usize;access.name_length=name.len();access.phase=1}Ok(None)},
      1=>{
       let depth=sessions.sessions.next_get_depth_demand(access.index).map_err(ValueError::from)?.max(1);
       let Some((known,_))=sessions.sessions.get(access.index)else{if original_inference_metadata(cx,0,1)?{access.phase=2}return Ok(None)};
       if access.byte<known.len().min(name.len()){
        if !original_inference_metadata(cx,1,depth)?{return Ok(None)}
        if known.as_bytes()[access.byte]==name.as_bytes()[access.byte]{access.byte+=1}else{access.index+=1;access.byte=0}return Ok(None)
       }
       if original_inference_metadata(cx,0,depth)?{if known.len()==name.len(){access.phase=5}else{access.index+=1;access.byte=0}}Ok(None)
      },
      2=>{
       if sessions.sessions.has_reserved_slot(){if original_inference_metadata(cx,0,1)?{access.phase=if access.original.is_some(){4}else{3}}return Ok(None)}
       if cx.should_yield(){return Ok(None)}
       let grant=cx.retained_grant();
       match sessions.sessions.reserve_one_funded(grant){
        Ok(progress)=>{if progress.copied_items!=0{cx.consume_fuel(1);cx.consume_retained(progress)?}Ok(None)},
        Err(error)=>{cx.consume_fuel(1);Err(OriginalTurnFailure::debit_original(ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}),cx))}
       }
      },
      3=>{let before=cx.fuel_remaining();if let Some(original)=init(cx)?{access.original=Some(original);access.phase=4;if cx.fuel_remaining()==before{return Err(refused("defining mounted inference constructor omitted original work receipt").into())}}Ok(None)},
      4=>{
       if !sessions.sessions.has_reserved_slot(){if original_inference_metadata(cx,0,1)?{access.phase=2}return Ok(None)}
       let grant=cx.retained_grant();let layout=std::alloc::Layout::new::<OriginalInferenceSession<T>>();
       if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth<sessions.sessions.next_push_depth_demand().map_err(ValueError::from)?.max(1)||grant.maximum_capacity_bytes<layout.size(){return Ok(None)}
       cx.consume_fuel(1);
       let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<OriginalInferenceSession<T>>())else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original mounted inference typed frame allocation refused").into())};
       let body=match semio_framework_value::retirement::controlled::ControlledRetirement::new(access.original.take().expect("original inference source retained before typed birth")){Ok(body)=>body,Err((error,original))=>{access.original=Some(original);unsafe{std::alloc::dealloc(pointer.as_ptr().cast(),layout)};return Err(OriginalTurnFailure::debit_original(error.with_retained_progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),released_bytes:layout.size(),..Default::default()}),cx))}};
       let owner=unsafe{pointer.as_ptr().write(OriginalInferenceSession{body});Box::from_raw(pointer.as_ptr())};
       access.index=sessions.sessions.len();sessions.sessions.push_reserved((name,owner)).unwrap_or_else(|_|panic!("original mounted inference row was admitted"));access.phase=5;
       cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()})?;Ok(None)
      },
      5=>{let depth=sessions.sessions.next_get_depth_demand(access.index).map_err(ValueError::from)?;let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_depth<=depth{return Ok(None)}let session=sessions.sessions.get_mut(access.index).and_then(|(_,owner)|owner.original_mut()).and_then(|original|original.downcast_mut::<T>()).ok_or_else(||refused("original mounted inference source type changed"))?;let mut output=None;cx.with_retained_loan(RetainedCloneGrant{maximum_depth:grant.maximum_depth-depth,..grant},|cx|{output=Some(apply(session,cx));Ok(())})?;Ok(output)},
      _=>Err(refused("original mounted inference admission phase is invalid").into())
     }
    }

    /// 📏️ Quotes this actual registry lookup, page and frame event before its defining constructor or child.
    pub fn inference_admission_demands<T:semio_framework_value::retirement::RetireOwned>(&self,access:&ArtifactInferenceSessionAccess<T>)->Result<RetirementDemand,ValueError>{
     let sessions=match self.inference.try_lock(){Ok(sessions)=>sessions,Err(std::sync::TryLockError::WouldBlock)=>return Ok(Default::default()),Err(std::sync::TryLockError::Poisoned(_))=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original mounted inference registry is poisoned"))};
     Ok(match access.phase{
      0|3=>RetirementDemand{depth:1,..Default::default()},
      1=>RetirementDemand{copy_bytes:sessions.sessions.get(access.index).filter(|(name,_)|access.byte<name.len().min(access.name_length)).map_or(0,|_|1),depth:sessions.sessions.next_get_depth_demand(access.index).map_err(ValueError::from)?.max(1),..Default::default()},
      2=>if sessions.sessions.has_reserved_slot(){RetirementDemand{depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:sessions.sessions.next_reserve_copy_byte_demand().map_err(ValueError::from)?,capacity_bytes:sessions.sessions.next_allocation_bytes().map_err(ValueError::from)?,depth:sessions.sessions.next_reserve_depth_demand().map_err(ValueError::from)?,..Default::default()}},
      4=>if sessions.sessions.has_reserved_slot(){RetirementDemand{capacity_bytes:std::mem::size_of::<OriginalInferenceSession<T>>(),depth:sessions.sessions.next_push_depth_demand().map_err(ValueError::from)?.max(1),..Default::default()}}else{RetirementDemand{depth:1,..Default::default()}},
      5=>RetirementDemand{depth:sessions.sessions.next_get_depth_demand(access.index).map_err(ValueError::from)?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original inference child depth overflow"))?,..Default::default()},
      _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inference admission phase is invalid"))
     })
    }

    pub fn inference_is_empty(&self) -> bool {
        self.inference.try_lock().is_ok_and(|sessions|sessions.terminal_is_empty())
    }

    /// 📏️ Borrows the original concrete session close producer under its existing Arc.
    pub fn inference_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{original_inference_close_demands(&self.inference,body)}
    /// 🎟️ Advances one actual caller-funded step of the same original session owner.
    pub fn close_inference_step(&self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{original_inference_close_step(&self.inference,cx,loan)}

    pub fn with_mut<T: std::any::Any, R>(&self, apply: impl FnOnce(&mut T) -> Result<R, Fault>) -> Result<R, Fault> {
        let mut owner = self.inner.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-owner-busy"), "app-instance operation owner is busy or poisoned"))?;
        let owner = owner.as_any_mut().downcast_mut::<T>().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-owner-type"), "app-instance operation owner type does not match its concrete app"))?;
        apply(owner)
    }

    /// 🔎️ Quotes the same original concrete owner without manufacturing a diagnostic on contention.
    pub fn inspect_original<T:std::any::Any,R>(&self,read:impl FnOnce(&T)->R)->Result<Option<R>,ValueError>{
        let mut owner=match self.inner.try_lock(){Ok(owner)=>owner,Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy)=>return Ok(None),Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned)=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original instance owner is poisoned"))};
        let source=owner.as_any_mut().downcast_ref::<T>().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original instance owner type changed"))?;
        Ok(Some(read(source)))
    }

    /// ⏳️ A WouldBlock is not a fault. A retained command job runs its `step` on a process-pool
    /// worker thread and holds this lock while it does, so the reactor's own maintenance turn on
    /// the main thread legitimately meets a busy owner — reporting that as
    /// `interactive-job.instance-owner-busy` failed the whole in-flight dispatch every time the
    /// two raced. Only a POISONED owner (a panic inside `with_mut`) is a fault; a busy one is
    /// simply no progress this turn (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn instance_owner_poisoned_fault() -> Fault {
        Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.instance-owner-poisoned"), "app-instance operation owner is poisoned")
    }

    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.inner.try_lock() {
            Ok(owner) => if owner.terminal_is_empty(){self.inference_retirement_demands(body)}else{owner.retirement_demands(body)},
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy) => Ok(RetirementDemand::default()),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned) => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "app-instance operation owner is poisoned")),
        }
    }

    pub fn maintenance_step(&self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        match self.inner.try_lock() {
            Ok(mut owner) => {let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,loan);let demand=owner.retirement_demands(grant.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|cx,grant|owner.maintenance_step(cx,grant).map(|step|(step,true)))},
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy) => Ok(PluginLifecycleStep::Progress(Default::default())),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned) => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"app-instance operation owner is poisoned").into()),
        }
    }

    pub fn close_step(&self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{
        match self.inner.try_lock() {
            Ok(mut owner) => {if owner.terminal_is_empty(){drop(owner);return self.close_inference_step(cx,loan)}let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,loan);let demand=owner.retirement_demands(grant.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|cx,grant|{let step=owner.close_step(cx,grant)?;let terminal=owner.terminal_is_empty();Ok((match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step},terminal))})},
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy) => Ok(PluginLifecycleStep::Progress(Default::default())),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned) => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"app-instance operation owner is poisoned").into()),
        }
    }

    pub fn terminal_is_empty(&self) -> Result<bool, Fault> {
        match self.inner.try_lock() {
            Ok(owner) => Ok(owner.terminal_is_empty()),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Busy) => Ok(false),
            Err(semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerCellErrorV1::Poisoned) => Err(Self::instance_owner_poisoned_fault()),
        }
    }

    /// 🧹️ The close witness: the operation owner is empty AND the instance holds no inference session.
    pub fn close_terminal_is_empty(&self) -> Result<bool, Fault> {
        Ok(self.terminal_is_empty()? && self.inference_is_empty())
    }
}

/// 🔗️ One original command alias keeps final instance payload custody until each physical backing closes.
pub struct ArtifactInstanceOperationOwnerAliasRetirement{
    original:semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerAliasRetirementV1<dyn ArtifactInstanceOperationOwner>,
    inference:ManuallyDrop<Option<std::sync::Arc<std::sync::Mutex<InstanceInferenceSessions>>>>,
    inference_owned:ManuallyDrop<Option<std::sync::Mutex<InstanceInferenceSessions>>>,
}

impl ArtifactInstanceOperationOwnerAliasRetirement {
    /// 📥️ Moves the caller's exact existing handle into an inline frontier without a new lease or frame.
    pub fn new(source: ArtifactInstanceOperationOwnerHandle) -> Self {
        Self{original:semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerAliasRetirementV1::new(source.inner),inference:ManuallyDrop::new(Some(source.inference)),inference_owned:ManuallyDrop::new(None)}
    }
    /// 📏️ Initial custody transfer needs one item and depth while all heap currencies remain zero.
    pub fn constructor_demands() -> RetirementDemand {
        semio_framework_plugin_mounted_owner::alias_retirement::MountedOwnerAliasRetirementV1::<dyn ArtifactInstanceOperationOwner>::constructor_demands()
    }
    /// 🔎️ Queries the same retained payload or actual backing without creating authority.
    pub fn demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner)=self.original.unique() {
            if !owner.terminal_is_empty() {
                let mut demand=owner.retirement_demands(copy)?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"instance alias payload depth overflow"))?;
                return Ok(demand);
            }
        }
        if !self.original.terminal_is_empty(){return Ok(self.original.demands())}
        if let Some(original)=self.inference_owned.as_ref(){let sessions=original.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred alias retains its source"))?;return if sessions.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{sessions.retirement_demands(copy)}}
        self.alias_return_demands(copy)
    }
    /// 🔗️ Quotes only the original alias or terminal box shell without authorizing active payload work.
    pub fn alias_return_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.original.terminal_is_empty(){return Ok(self.original.demands())}
        if self.inference.is_some(){return Ok(RetirementDemand{release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<std::sync::Mutex<InstanceInferenceSessions>>(),depth:1,..Default::default()})}
        if let Some(original)=self.inference_owned.as_ref(){let sessions=original.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred alias retains its exclusive source"))?;return if sessions.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{sessions.retirement_demands(copy)}}
        Ok(Default::default())
    }
    /// 📤️ Atomically returns the actual alias and retains any final original inferred payload.
    pub fn alias_return_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let demand=self.alias_return_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !self.original.terminal_is_empty(){return self.original.step(grant,ArtifactInstanceOperationOwner::terminal_is_empty).map(|step|RetainedCloneStep::Progress(step.progress()))}
        if let Some(original)=self.inference.as_ref(){if std::sync::Arc::weak_count(original)!=0{return Ok(RetainedCloneStep::Progress(Default::default()))}let released=if let Some(value)=std::sync::Arc::into_inner(self.inference.take().unwrap()){*self.inference_owned=Some(value);demand.release_bytes}else{0};return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}))}
        if let Some(original)=self.inference_owned.as_mut(){let sessions=original.get_mut().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inferred alias source is poisoned"))?;if !sessions.terminal_is_empty(){return sessions.close_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}drop(self.inference_owned.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    /// 🧾️ Closes shared backing, exact final owner and dynamic box in separate funded turns.
    pub fn step(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PluginLifecycleFailure>{
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(owner)=self.original.unique_mut() {
            if !owner.terminal_is_empty() {
                let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
                let step=owner.close_step(cx,child)?;
                let progress=step.progress().unwrap_or_default();
                let outcome=if matches!(step,PluginLifecycleStep::Complete(_)){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)};
                let accepted=semio_framework_value::retained_clone::admit_retained_clone_close(child,outcome,owner.terminal_is_empty(),"original instance alias final payload")?;
                return Ok(RetainedCloneStep::Progress(accepted.progress()));
            }
        }
        self.alias_return_step(grant).map_err(PluginLifecycleFailure::from)
    }
    /// 🏁️ The original alias, final concrete payload and its dynamic box shell are all physically gone.
    pub fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()&&self.inference.is_none()&&self.inference_owned.is_none()}
}

impl Drop for ArtifactInstanceOperationOwnerAliasRetirement{
    fn drop(&mut self){
        assert!(std::thread::panicking()||self.terminal_is_empty(),"original instance alias abandoned inferred owner custody");
        if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.inference);ManuallyDrop::drop(&mut self.inference_owned);}}
    }
}

include!("../../../👷️job/♻️alias/🦀️.rs");

#[cfg(test)]
#[path = "../../../👷️job/🧪️inference-close/🦀️.rs"]
mod original_inference_close_tests;

#[cfg(test)]
#[path = "../../../👷️job/🧪️inference-admission/🦀️.rs"]
mod original_inference_admission_tests;
