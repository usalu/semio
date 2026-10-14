//! 🏗️ One original caller funds each native Session family before its exact owner is born.
use super::*;
use semio_framework_job::{StepContext,OriginalTurnFailure,OperationId,Generation};
use semio_framework_value::RetirementDemand;
use std::sync::atomic::{AtomicU64,AtomicUsize,Ordering};

type KernelShell=ShellArc<RetainedOwnerGate<ManuallyDrop<Brep>>>;
type CacheShell=ShellArc<RetainedOwnerGate<ManuallyDrop<MeshCache>>>;
type ClaimsShell=ShellArc<RetainedOwnerGate<ManuallyDrop<FamilyClaims>>>;
type RetirementShell=ShellArc<RetainedOwnerGate<SessionRetirement>>;

#[derive(Clone,Copy,PartialEq,Eq)]
struct SourceIdentity{operation:OperationId,generation:Generation,parent:usize,recipient:usize,cancel:usize,clock:usize,epoch:Option<u64>}
impl SourceIdentity{
 fn read(cx:&StepContext<'_>)->Result<Self,ValueError>{Ok(Self{operation:cx.operation(),generation:cx.generation(),parent:cx.original_source_parent()?.source_identity()?,recipient:cx.original_recipient_identity(),cancel:cx.original_cancel_token()as*const _ as usize,clock:cx.clock_source()as usize,epoch:cx.original_source_epoch()})}
}

/// 🎟️ Returns the original capture after all native births and its paid handoff.
pub struct SessionConstructionStep{pub ready:Option<SessionCapture>,pub progress:RetainedCloneProgress}

/// 🏗️ Retains only physically born families; cancellation never finishes an absent constructor.
#[must_use="Session construction requires explicit completion or closure"]
pub struct SessionConstructionCursor{
 operations:&'static dyn GeometryOperations,identity:Option<SourceIdentity>,phase:u8,closing:bool,done:bool,
 kernel:ManuallyDrop<Option<KernelShell>>,cache:ManuallyDrop<Option<CacheShell>>,claims:ManuallyDrop<Option<ClaimsShell>>,
 counter:ManuallyDrop<Option<ShellArc<AtomicU64>>>,retirement:ManuallyDrop<Option<RetirementShell>>,
 raw_kernel:ManuallyDrop<Option<ControlledRetirement<Brep>>>,raw_cache:ManuallyDrop<Option<ControlledRetirement<MeshCache>>>,raw_claims:ManuallyDrop<Option<ControlledRetirement<FamilyClaims>>>,
 ready:ManuallyDrop<Option<SessionCapture>>,
}
impl SessionConstructionCursor{
 pub fn new(operations:&'static dyn GeometryOperations)->Self{Self{operations,identity:None,phase:0,closing:false,done:false,kernel:ManuallyDrop::new(None),cache:ManuallyDrop::new(None),claims:ManuallyDrop::new(None),counter:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),raw_kernel:ManuallyDrop::new(None),raw_cache:ManuallyDrop::new(None),raw_claims:ManuallyDrop::new(None),ready:ManuallyDrop::new(None)}}
 pub fn phase(&self)->u8{self.phase}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn terminal_is_empty(&self)->bool{self.done&&self.kernel.is_none()&&self.cache.is_none()&&self.claims.is_none()&&self.counter.is_none()&&self.retirement.is_none()&&self.raw_kernel.is_none()&&self.raw_cache.is_none()&&self.raw_claims.is_none()&&self.ready.is_none()}
 fn validate(&self,cx:&StepContext<'_>)->Result<SourceIdentity,OriginalTurnFailure>{let identity=SourceIdentity::read(cx)?;if self.identity.is_some_and(|original|original!=identity){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Session construction requires its original source and recipient").into())}Ok(identity)}
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}if self.closing{return self.close_demand(body)}
  let mut demand=RetirementDemand{depth:1,..Default::default()};
  match self.phase{
   0|9=>{},
   1=>{let claims=self.raw_claims.as_ref().and_then(ControlledRetirement::original).ok_or_else(ownership_busy)?;demand.capacity_bytes=claims.next_insert_capacity_byte_demand(&1,body)?;demand.depth=claims.next_insert_depth_demand(&1)?;},
   2=>{let claims=self.raw_claims.as_ref().and_then(ControlledRetirement::original).ok_or_else(ownership_busy)?;demand.depth=claims.next_insert_depth_demand(&1)?;},
   3=>demand.capacity_bytes=shell_birth_bytes::<RetainedOwnerGate<ManuallyDrop<Brep>>>(),
   4=>demand.capacity_bytes=shell_birth_bytes::<RetainedOwnerGate<ManuallyDrop<MeshCache>>>(),
   5=>demand.capacity_bytes=shell_birth_bytes::<RetainedOwnerGate<ManuallyDrop<FamilyClaims>>>(),
   6=>demand.capacity_bytes=shell_birth_bytes::<AtomicU64>(),
   7=>demand.capacity_bytes=shell_birth_bytes::<RetainedOwnerGate<SessionRetirement>>(),
   8=>demand.capacity_bytes=shell_birth_bytes::<SessionState>(),
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"invalid original Session birth phase")),
  }Ok(demand)
 }
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<SessionConstructionStep,OriginalTurnFailure>{
  let identity=self.validate(cx)?;if cx.is_cancelled(){self.begin_close()}
  if self.closing{let progress=self.close_step(cx)?.progress();return Ok(SessionConstructionStep{ready:None,progress})}
  if self.terminal_is_empty()||cx.should_yield(){return Ok(SessionConstructionStep{ready:None,progress:Default::default()})}
  let grant=cx.retained_grant();if !fits(self.next_demand(grant.maximum_copy_bytes)?,grant){return Ok(SessionConstructionStep{ready:None,progress:Default::default()})}
  cx.consume_fuel(1);let result=self.birth_event(identity,grant);
  let progress=match result{Ok(progress)=>progress,Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))};
  cx.consume_retained(progress)?;
  let ready=if self.phase==10{self.done=true;self.ready.take()}else{None};Ok(SessionConstructionStep{ready,progress})
 }
 fn birth_event(&mut self,identity:SourceIdentity,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  match self.phase{
   0=>{self.identity=Some(identity);self.raw_claims=ManuallyDrop::new(Some(controlled(FamilyClaims::new())));},
   1=>{let claims=self.raw_claims.as_mut().and_then(ControlledRetirement::original_mut).ok_or_else(ownership_busy)?;progress=claims.reserve_insert_step(&1,grant).map_err(|(error,receipt)|error.with_retained_progress(receipt))?;},
   2=>{let claims=self.raw_claims.as_mut().and_then(ControlledRetirement::original_mut).ok_or_else(ownership_busy)?;let(previous,receipt)=claims.insert_reserved(1,AuthorityClaims::open(),grant).map_err(|(error,_,value)|{assert!(value.handles.terminal_is_empty());error})?;assert!(previous.is_none());progress=receipt;},
   3=>{let(owner,receipt)=birth_shell(||RetainedOwnerGate::new(ManuallyDrop::new(Brep::new())))?;self.kernel=ManuallyDrop::new(Some(owner));progress=receipt;},
   4=>{let(owner,receipt)=birth_shell(||RetainedOwnerGate::new(ManuallyDrop::new(MeshCache::new())))?;self.cache=ManuallyDrop::new(Some(owner));progress=receipt;},
   5=>{let(owner,receipt)=birth_shell(||{let mut raw=self.raw_claims.take().unwrap();RetainedOwnerGate::new(ManuallyDrop::new(raw.take_original().unwrap()))})?;self.claims=ManuallyDrop::new(Some(owner));progress=receipt;},
   6=>{let(owner,receipt)=birth_shell(||AtomicU64::new(2))?;self.counter=ManuallyDrop::new(Some(owner));progress=receipt;},
   7=>{let(owner,receipt)=birth_shell(||RetainedOwnerGate::new(SessionRetirement::default()))?;self.retirement=ManuallyDrop::new(Some(owner));progress=receipt;},
   8=>{let(owner,receipt)=birth_shell(||SessionState{operations:self.operations,kernel:self.kernel.take().unwrap(),mesh_cache:self.cache.take().unwrap(),claims:self.claims.take().unwrap(),next_authority:self.counter.take().unwrap(),authority:1,closed:std::sync::atomic::AtomicBool::new(false),jobs:RetainedOwnerGate::new(controlled(TessellationJobRegistry::default())),retirement:self.retirement.take().unwrap()})?;self.ready=ManuallyDrop::new(Some(SessionCapture::owned(Session{state:owner})));progress=receipt;},
   9=>{},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"invalid original Session birth event")),
  }self.phase+=1;Ok(progress)
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(ready)=self.ready.as_ref(){return Ok(RetirementDemand{copy_bytes:ready.next_close_copy_byte_demand()?,capacity_bytes:ready.next_close_capacity_byte_demand(body)?,release_bytes:ready.next_close_release_byte_demand()?,depth:ready.next_close_depth_demand()?.max(1)})}
  if let Some(owner)=self.kernel.as_ref(){return Ok(shell_close_demand(owner))}if let Some(owner)=self.cache.as_ref(){return Ok(shell_close_demand(owner))}if let Some(owner)=self.claims.as_ref(){return Ok(shell_close_demand(owner))}if let Some(owner)=self.counter.as_ref(){return Ok(shell_close_demand(owner))}if let Some(owner)=self.retirement.as_ref(){return Ok(shell_close_demand(owner))}
  if let Some(owner)=self.raw_kernel.as_ref(){return close_owner_demand(owner,body)}if let Some(owner)=self.raw_cache.as_ref(){return close_owner_demand(owner,body)}if let Some(owner)=self.raw_claims.as_ref(){return close_owner_demand(owner,body)}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{
  self.validate(cx)?;self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let grant=cx.retained_grant();if cx.should_yield()||!fits(self.close_demand(grant.maximum_copy_bytes)?,grant){return Ok(ownership_progress())}
  cx.consume_fuel(1);let progress=match self.close_event(grant){Ok(progress)=>progress,Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))};cx.consume_retained(progress)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 fn close_event(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if let Some(ready)=self.ready.as_mut(){if ready.terminal_is_empty(){self.ready.take();return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}return ready.close_step(grant).map(|step|step.progress())}
  if self.kernel.is_some(){let(value,progress)=take_shell(self.kernel.take().unwrap());self.raw_kernel=ManuallyDrop::new(Some(controlled(ManuallyDrop::into_inner(value.into_inner()))));return Ok(progress)}
  if self.cache.is_some(){let(value,progress)=take_shell(self.cache.take().unwrap());self.raw_cache=ManuallyDrop::new(Some(controlled(ManuallyDrop::into_inner(value.into_inner()))));return Ok(progress)}
  if self.claims.is_some(){let(value,progress)=take_shell(self.claims.take().unwrap());self.raw_claims=ManuallyDrop::new(Some(controlled(ManuallyDrop::into_inner(value.into_inner()))));return Ok(progress)}
  if self.counter.is_some(){let(_,progress)=take_shell(self.counter.take().unwrap());return Ok(progress)}
  if self.retirement.is_some(){let(value,progress)=take_shell(self.retirement.take().unwrap());assert!(value.into_inner().family.is_none());return Ok(progress)}
  if let Some(owner)=self.raw_kernel.as_mut(){if owner.terminal_is_empty(){self.raw_kernel.take();return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}return owner.step(grant).map(|step|step.progress()).map_err(|error|error.with_retained_progress(owner.step_progress()))}
  if let Some(owner)=self.raw_cache.as_mut(){if owner.terminal_is_empty(){self.raw_cache.take();return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}return owner.step(grant).map(|step|step.progress()).map_err(|error|error.with_retained_progress(owner.step_progress()))}
  if let Some(owner)=self.raw_claims.as_mut(){if owner.terminal_is_empty(){self.raw_claims.take();return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}return owner.step(grant).map(|step|step.progress()).map_err(|error|error.with_retained_progress(owner.step_progress()))}
  self.identity=None;self.done=true;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
 }
}
impl Drop for SessionConstructionCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Session construction requires paid original closure before Drop")}}
fn fits(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn close_owner_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{let mut demand=owner.selected_demand(body)?;demand.depth=demand.depth.max(1);Ok(demand)}
fn shell_birth_bytes<T>()->usize{std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<T>()).expect("Arc layout").0.pad_to_align().size()}
fn birth_shell<T>(make:impl FnOnce()->T)->Result<(ShellArc<T>,RetainedCloneProgress),ValueError>{
 let mut allocation=Arc::<T,ShellAllocator>::try_new_uninit_in(ShellAllocator::default()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original Session shell allocation failed"))?;
 let bytes=shell_bytes(&allocation);Arc::get_mut(&mut allocation).unwrap().write(make());let owner=unsafe{allocation.assume_init()};Ok((owner,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}))
}
fn shell_close_demand<T>(owner:&ShellArc<T>)->RetirementDemand{RetirementDemand{release_bytes:shell_bytes(owner),depth:1,..Default::default()}}
fn take_shell<T>(owner:ShellArc<T>)->(T,RetainedCloneProgress){let receipt=AtomicUsize::new(0);record_shell(&owner,&receipt);let value=Arc::into_inner(owner).expect("constructor shell remains its unique original");(value,RetainedCloneProgress{copied_items:1,released_bytes:receipt.load(Ordering::Relaxed),..Default::default()})}
