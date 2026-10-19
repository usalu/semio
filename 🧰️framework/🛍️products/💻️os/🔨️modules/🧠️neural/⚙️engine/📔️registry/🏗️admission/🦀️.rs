//! 🏗️ The original caller funds both registry shells before the native source moves.
use super::*;
use semio_framework_job::{StepContext,OriginalTurnFailure,OperationId,Generation};
use semio_framework_value::RetirementDemand;
use std::mem::MaybeUninit;
type UninitRegistry=Arc<MaybeUninit<Registry>,RegistryAllocator>;

#[derive(Clone,Copy,PartialEq,Eq)]
struct SourceIdentity{operation:OperationId,generation:Generation,source:usize,recipient:usize,cancel:usize,clock:usize,epoch:Option<u64>,slot:usize}
impl SourceIdentity{
 fn read(source:&Option<Registry>,cx:&StepContext<'_>)->Result<Self,ValueError>{Ok(Self{operation:cx.operation(),generation:cx.generation(),source:cx.original_source_parent()?.source_identity()?,recipient:cx.original_recipient_identity(),cancel:cx.original_cancel_token()as*const _ as usize,clock:cx.clock_source()as usize,epoch:cx.original_source_epoch(),slot:source as*const _ as usize})}
}
/// 🎟️ Transfers the original reader and its separately funded final authority together.
pub struct SharedRegistryAdmissionStep{pub ready:Option<(SharedRegistry,RegistryRetirement)>,pub progress:RetainedCloneProgress}
/// 🏗️ Reserves original physical shells without taking or cloning the caller's registry on denial.
#[must_use="registry admission requires completion or paid source-return closure"]
pub struct SharedRegistryIssuer{
 identity:Option<SourceIdentity>,phase:u8,closing:bool,done:bool,
 final_root:ManuallyDrop<Option<FinalRoot>>,uninit:ManuallyDrop<Option<UninitRegistry>>,
 root:ManuallyDrop<Option<Arc<Registry,RegistryAllocator>>>,retirement:ManuallyDrop<Option<RegistryRetirement>>,
 returned:ManuallyDrop<Option<Registry>>,
}
impl SharedRegistryIssuer{
 pub fn new()->Self{Self{identity:None,phase:0,closing:false,done:false,final_root:ManuallyDrop::new(None),uninit:ManuallyDrop::new(None),root:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),returned:ManuallyDrop::new(None)}}
 pub fn phase(&self)->u8{self.phase}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn terminal_is_empty(&self)->bool{self.done&&self.final_root.is_none()&&self.uninit.is_none()&&self.root.is_none()&&self.retirement.is_none()&&self.returned.is_none()}
 fn validate(&self,source:&Option<Registry>,cx:&StepContext<'_>)->Result<SourceIdentity,OriginalTurnFailure>{let identity=SourceIdentity::read(source,cx)?;if self.identity.is_some_and(|original|original!=identity){return Err(refused("registry admission requires its original source slot and caller").into())}Ok(identity)}
 pub fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}if self.closing{return self.close_demand()}
  Ok(RetirementDemand{capacity_bytes:match self.phase{1=>shell_bytes::<FinalRootSlot>(),2=>shell_bytes::<Registry>(),0|3|4|5=>0,_=>return Err(refused("invalid original registry admission phase"))},depth:1,..Default::default()})
 }
 pub fn step(&mut self,source:&mut Option<Registry>,cx:&mut StepContext<'_>)->Result<SharedRegistryAdmissionStep,OriginalTurnFailure>{
  let identity=self.validate(source,cx)?;if cx.is_cancelled(){self.begin_close()}
  if self.closing{return Ok(SharedRegistryAdmissionStep{ready:None,progress:self.close_step(source,cx)?.progress()})}
  if self.terminal_is_empty()||cx.should_yield(){return Ok(SharedRegistryAdmissionStep{ready:None,progress:Default::default()})}
  if self.phase<=3&&source.is_none(){return Err(refused("original registry source is absent before donation").into())}
  let grant=cx.retained_grant();if !fits(self.next_demand(grant.maximum_copy_bytes)?,grant){return Ok(SharedRegistryAdmissionStep{ready:None,progress:Default::default()})}
  cx.consume_fuel(1);let progress=match self.birth_event(source,identity){Ok(progress)=>progress,Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))};cx.consume_retained(progress)?;
  let ready=if self.phase==6{self.done=true;Some((SharedRegistry{root:self.root.take(),final_root:self.final_root.take().unwrap()},self.retirement.take().unwrap()))}else{None};Ok(SharedRegistryAdmissionStep{ready,progress})
 }
 fn birth_event(&mut self,source:&mut Option<Registry>,identity:SourceIdentity)->Result<RetainedCloneProgress,ValueError>{
  let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
  match self.phase{
   0=>self.identity=Some(identity),
   1=>{let mut root=Arc::<FinalRootSlot,ShellAllocator>::try_new_uninit_in(ShellAllocator::default()).map_err(|_|allocation())?;let bytes=Arc::allocator(&root).0.load(Ordering::Acquire);Arc::get_mut(&mut root).unwrap().write(FinalRootSlot{registry:OnceLock::new(),shell:OnceLock::new(),registry_bytes:AtomicUsize::new(0)});self.final_root=ManuallyDrop::new(Some(unsafe{root.assume_init()}));receipt.retained_capacity_bytes=bytes;},
   2=>{let final_root=self.final_root.as_ref().unwrap();let root=Arc::<Registry,RegistryAllocator>::try_new_uninit_in(RegistryAllocator{final_root:Arc::as_ptr(final_root)as usize}).map_err(|_|allocation())?;self.uninit=ManuallyDrop::new(Some(root));receipt.retained_capacity_bytes=final_root.registry_bytes.load(Ordering::Acquire);},
   3=>{let mut root=self.uninit.take().unwrap();Arc::get_mut(&mut root).unwrap().write(source.take().unwrap());self.root=ManuallyDrop::new(Some(unsafe{root.assume_init()}));},
   4=>{self.retirement=ManuallyDrop::new(Some(RegistryRetirement{final_root:ManuallyDrop::new(Some(Arc::clone(self.final_root.as_ref().unwrap()))),registry:None}));},
   5=>{},
   _=>return Err(refused("invalid original registry birth event")),
  }self.phase+=1;Ok(receipt)
 }
 fn close_demand(&self)->Result<RetirementDemand,ValueError>{
  let mut demand=RetirementDemand{depth:1,..Default::default()};
  if self.retirement.is_some()||self.root.is_some()||self.uninit.is_some(){return Ok(demand)}
  if let Some(root)=self.final_root.as_ref(){demand.release_bytes=root.shell.get().map_or_else(||Arc::allocator(root).0.load(Ordering::Acquire),|shell|shell.layout.size());}
  Ok(demand)
 }
 pub fn close_step(&mut self,source:&mut Option<Registry>,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{
  self.validate(source,cx)?;self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let grant=cx.retained_grant();if cx.should_yield()||!fits(self.close_demand()?,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.returned.is_some()&&self.final_root.is_none()&&source.is_some(){return Err(refused("original registry return slot is occupied").into())}
  cx.consume_fuel(1);let progress=match self.close_event(source){Ok(progress)=>progress,Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))};cx.consume_retained(progress)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 fn close_event(&mut self,source:&mut Option<Registry>)->Result<RetainedCloneProgress,ValueError>{
  let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
  if let Some(mut retirement)=self.retirement.take(){assert!(retirement.registry.is_none());drop(retirement.final_root.take());return Ok(receipt)}
  if self.root.as_mut().is_some_and(|root|Arc::get_mut(root).is_none()){return Err(refused("original registry shell escaped before handoff"))}
  if self.uninit.as_mut().is_some_and(|root|Arc::get_mut(root).is_none()){return Err(refused("original uninitialized registry shell escaped before handoff"))}
  if let Some(root)=self.root.take(){self.returned=ManuallyDrop::new(Some(Arc::into_inner(root).ok_or_else(||refused("original registry shell escaped before handoff"))?));return Ok(receipt)}
  if let Some(root)=self.uninit.take(){drop(root);return Ok(receipt)}
  if let Some(root)=self.final_root.as_mut(){let slot=Arc::get_mut(root).ok_or_else(||refused("original final-root shell escaped before handoff"))?;if let Some(shell)=slot.shell.take(){unsafe{Global.deallocate(NonNull::new_unchecked(shell.address as*mut u8),shell.layout)};receipt.released_bytes=shell.layout.size();return Ok(receipt)}assert!(slot.registry.get().is_none());let bytes=Arc::allocator(root).0.load(Ordering::Acquire);drop(self.final_root.take());receipt.released_bytes=bytes;return Ok(receipt)}
  if self.returned.is_some(){*source=self.returned.take();return Ok(receipt)}
  self.identity=None;self.done=true;Ok(receipt)
 }
}
impl Default for SharedRegistryIssuer{fn default()->Self{Self::new()}}
impl Drop for SharedRegistryIssuer{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original registry issuer requires paid source-return closure before Drop")}}
fn shell_bytes<T>()->usize{semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<T>()}
fn fits(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"original registry shell allocation failed")}
