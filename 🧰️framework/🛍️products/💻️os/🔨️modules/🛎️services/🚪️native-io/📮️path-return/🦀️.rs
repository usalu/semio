//! 🧭️ An original selected directory path keeps its genuine supplier alive through downstream borrowed-path reads.
use super::*;
use semio_framework_job::{OriginalCallerSourceLoan,close_original_retained_step};
use semio_framework_value::RetainedCloneStep;
use std::sync::atomic::{AtomicU8,AtomicU64,AtomicUsize,Ordering};
use std::cell::UnsafeCell;
use std::mem::size_of;

const VACANT:u8=0;const CHECKED:u8=1;const RESERVED:u8=2;const ISSUED:u8=3;const RETURNED:u8=4;const EXHAUSTED:u8=5;
struct PathOwners{source:Option<OriginalCallerSourceLoan>,path:Option<PathBuf>}
struct PathSlot{state:AtomicU8,generation:AtomicU64,identity:AtomicUsize,owners:UnsafeCell<PathOwners>}
/// 🔐️ Atomic checkout grants exclusive owner access and publishes the original return before a wake.
unsafe impl Sync for PathSlot{}
impl PathSlot{const fn empty()->Self{Self{state:AtomicU8::new(VACANT),generation:AtomicU64::new(0),identity:AtomicUsize::new(0),owners:UnsafeCell::new(PathOwners{source:None,path:None})}}fn checkout(&self,state:u8)->Option<PathCheckout<'_>>{self.state.compare_exchange(state,CHECKED,Ordering::Acquire,Ordering::Relaxed).ok().map(|_|PathCheckout{slot:self,restore:state})}}
struct PathCheckout<'a>{slot:&'a PathSlot,restore:u8}
impl PathCheckout<'_>{fn owners(&mut self)->&mut PathOwners{unsafe{&mut*self.slot.owners.get()}}}
impl Drop for PathCheckout<'_>{fn drop(&mut self){self.slot.state.store(self.restore,Ordering::Release)}}
trait PathReturnTarget:Sync{fn identity(&self,index:usize)->usize;fn return_path(&self,index:usize,generation:u64,path:PathBuf);}
fn refused(message:&'static str)->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message).into()}
fn metadata(cx:&mut StepContext<'_>,bytes:usize,closing:bool)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if (if closing{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}

/// 🗄️ Fixed startup-owned return slots accept only selected paths from the actual NativeIoJob result.
pub struct OriginalNativePathReturnRegistry<const N:usize>{slots:[PathSlot;N],wake:fn()}
impl<const N:usize> OriginalNativePathReturnRegistry<N>{
 pub const fn with_return_wake(wake:fn())->Self{Self{slots:[const{PathSlot::empty()};N],wake}}
 pub fn reserve(&'static self,cx:&mut StepContext<'_>)->Result<Option<OriginalNativePathReservation<N>>,OriginalTurnFailure>{
  fn require_send<T:Send>(){}require_send::<PathOwners>();let identity=cx.original_source_parent()?.source_identity()?;
  for(index,slot)in self.slots.iter().enumerate(){let Some(mut checked)=slot.checkout(VACANT)else{continue};let Some(generation)=slot.generation.load(Ordering::Relaxed).checked_add(1)else{checked.restore=EXHAUSTED;continue};let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(None)};checked.owners().source=Some(source);slot.generation.store(generation,Ordering::Relaxed);slot.identity.store(identity,Ordering::Relaxed);checked.restore=RESERVED;return Ok(Some(OriginalNativePathReservation{registry:self,index,generation,active:true}))}Ok(None)
 }
 pub fn retains_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|matches!(slot.state.load(Ordering::Acquire),CHECKED|RESERVED|ISSUED|RETURNED)&&slot.identity.load(Ordering::Acquire)==identity)}
 pub fn has_returned_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|slot.state.load(Ordering::Acquire)==RETURNED&&slot.identity.load(Ordering::Acquire)==identity)}
 pub fn close_returned_step(&self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let identity=cx.original_source_parent()?.source_identity()?;
  for slot in &self.slots{if slot.identity.load(Ordering::Acquire)!=identity{continue}let Some(mut checked)=slot.checkout(RETURNED)else{continue};let owners=checked.owners();if let Some(source)=owners.source.as_ref(){source.validate_retirement_context(cx)?;}
   if let Some(path)=owners.path.as_ref(){let demand=RetirementDemand{copy_bytes:size_of::<Option<PathBuf>>(),release_bytes:path.capacity(),depth:1,..Default::default()};return close_original_retained_step(cx,demand,|_|{drop(owners.path.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})}
   if let Some(source)=owners.source.as_mut(){let step=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){owners.source.take();}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}
   if !metadata(cx,size_of::<bool>(),true)?{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}checked.restore=VACANT;return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}})
  }Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }
}
impl<const N:usize> PathReturnTarget for OriginalNativePathReturnRegistry<N>{fn identity(&self,index:usize)->usize{self.slots[index].identity.load(Ordering::Acquire)}fn return_path(&self,index:usize,generation:u64,path:PathBuf){let slot=&self.slots[index];let mut checked=slot.checkout(ISSUED).expect("selected original path retains its exclusive return slot");assert_eq!(slot.generation.load(Ordering::Relaxed),generation);checked.owners().path=Some(path);checked.restore=RETURNED;drop(checked);(self.wake)();}}

/// 🎟️ The reservation retains the original source before the actual selected PathBuf leaves its producer.
pub struct OriginalNativePathReservation<const N:usize>{registry:&'static OriginalNativePathReturnRegistry<N>,index:usize,generation:u64,active:bool}
impl<const N:usize> OriginalNativePathReservation<N>{
 pub(super) fn publish_last(&mut self,paths:&mut NativePathSet,output:&mut Option<OriginalSuppliedNativePath>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.active||paths.is_empty()||output.is_some(){return Err(refused("selected path requires original result, active return and empty output"))}let slot=&self.registry.slots[self.index];let Some(mut checked)=slot.checkout(RESERVED)else{return Ok(false)};if slot.generation.load(Ordering::Relaxed)!=self.generation{return Err(refused("selected native path return generation changed"))}checked.owners().source.as_ref().unwrap().validate_context(cx)?;if !metadata(cx,size_of::<OriginalSuppliedNativePath>()+size_of::<Option<PathBuf>>()+size_of::<bool>(),false)?{return Ok(false)}
  *output=Some(OriginalSuppliedNativePath{target:self.registry,index:self.index,generation:self.generation,path:paths.pop()});self.active=false;checked.restore=ISSUED;Ok(true)
 }
}
impl<const N:usize> Drop for OriginalNativePathReservation<N>{fn drop(&mut self){if !self.active{return}let slot=&self.registry.slots[self.index];let mut checked=slot.checkout(RESERVED).expect("selected native path reservation retains source custody");assert_eq!(slot.generation.load(Ordering::Relaxed),self.generation);checked.restore=RETURNED;drop(checked);(self.registry.wake)();}}
/// 📖️ A consumer borrows the exact selected path; Drop returns its original backing without freeing or cloning it.
pub struct OriginalSuppliedNativePath{target:&'static dyn PathReturnTarget,index:usize,generation:u64,path:Option<PathBuf>}
impl OriginalSuppliedNativePath{pub fn path(&self)->&std::path::Path{self.path.as_ref().unwrap().as_path()}pub fn source_identity(&self)->usize{self.target.identity(self.index)}}
impl std::fmt::Debug for OriginalSuppliedNativePath{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalSuppliedNativePath").field("path",&self.path()).finish()}}
impl Drop for OriginalSuppliedNativePath{fn drop(&mut self){self.target.return_path(self.index,self.generation,self.path.take().unwrap())}}
