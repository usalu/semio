//! 🧭️ Original directory paths retain their supplier through current staged loans and caller-owned publication.
use super::*;
use semio_framework_job::{OriginalCallerSourceLoan,OriginalCallerReturnPlace,OriginalSourceBinding,close_original_retained_step,receive_original_retained_work};
use std::sync::atomic::{AtomicU8,AtomicU64,AtomicUsize,Ordering};
use std::cell::UnsafeCell;
const VACANT:u8=0;const CHECKED:u8=1;const RESERVED:u8=2;const ISSUED:u8=3;const RETURNED:u8=4;const EXHAUSTED:u8=5;const CLAIMING:u8=6;
struct PathOwners{source:Option<OriginalCallerSourceLoan>,path:Option<PathBuf>,binding:Option<OriginalSourceBinding>,credit:OriginalCallerReturnPlace}
struct PathSlot{state:AtomicU8,generation:AtomicU64,identity:AtomicUsize,owners:UnsafeCell<PathOwners>}
/// 🔐️ Atomic checkout grants exclusive access to original path and source owners.
unsafe impl Sync for PathSlot{}
impl PathSlot{const fn empty()->Self{Self{state:AtomicU8::new(VACANT),generation:AtomicU64::new(0),identity:AtomicUsize::new(0),owners:UnsafeCell::new(PathOwners{source:None,path:None,binding:None,credit:OriginalCallerReturnPlace::empty()})}}fn checkout(&self,state:u8)->Option<PathCheckout<'_>>{self.state.compare_exchange(state,CHECKED,Ordering::Acquire,Ordering::Relaxed).ok().map(|_|PathCheckout{slot:self,restore:state})}}
struct PathCheckout<'a>{slot:&'a PathSlot,restore:u8}
impl PathCheckout<'_>{fn owners(&mut self)->&mut PathOwners{unsafe{&mut*self.slot.owners.get()}}}
impl Drop for PathCheckout<'_>{fn drop(&mut self){self.slot.state.store(self.restore,Ordering::Release)}}
trait PathReturnTarget:Sync{fn identity(&self,index:usize)->usize;fn return_path(&self,index:usize,generation:u64,path:PathBuf);}
fn refused(message:&'static str)->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message).into()}
fn control()->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
fn current(cx:&StepContext<'_>)->bool{!cx.is_cancelled()&&!cx.should_yield()&&cx.retained_work_can_enter(RetirementDemand{depth:1,..Default::default()})}
/// 🗄️ Startup-owned return places keep every path and genuine source until explicit paid closure.
pub struct OriginalNativePathReturnRegistry<const N:usize>{slots:[PathSlot;N],cursor:AtomicUsize,wake:fn()}
impl<const N:usize> OriginalNativePathReturnRegistry<N>{
 pub const fn with_return_wake(wake:fn())->Self{Self{slots:[const{PathSlot::empty()};N],cursor:AtomicUsize::new(0),wake}}
 pub fn reserve(&'static self,output:&mut Option<OriginalNativePathReservation<N>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;let identity=parent.source_identity()?;
  if let Some(reservation)=output.as_mut(){
   if !std::ptr::eq(reservation.registry,self)||!reservation.active{return Err(refused("original native path reservation changed its registry or lifecycle"))}
   let slot=&self.slots[reservation.index];if slot.identity.load(Ordering::Acquire)!=identity||slot.generation.load(Ordering::Acquire)!=reservation.generation{return Err(refused("original native path reservation changed its source or generation"))}
   let Some(mut checked)=slot.checkout(if reservation.ready{RESERVED}else{CLAIMING})else{return Ok(false)};
   if !checked.owners().binding.is_some_and(|binding|binding.matches(cx)){return Err(refused("native path reservation changed its original caller context"))}
   if reservation.ready{checked.owners().source.as_ref().unwrap().validate_context(cx)?;return Ok(true)}
   if checked.owners().source.is_none(){parent.admit_loan(&mut checked.owners().source,cx)?;return Ok(false)}
   checked.owners().source.as_ref().unwrap().validate_context(cx)?;if !current(cx){return Ok(false)}
   reservation.ready=true;checked.restore=RESERVED;receive_original_retained_work(cx,control())?;return Ok(true)
  }
  if N==0{return Err(refused("native path return requires a configured original place"))}if !current(cx){return Ok(false)}
  let index=self.cursor.load(Ordering::Relaxed)%N;let slot=&self.slots[index];let Some(mut checked)=slot.checkout(VACANT)else{self.cursor.fetch_add(1,Ordering::Relaxed);receive_original_retained_work(cx,control())?;return Ok(false)};
  let Some(generation)=slot.generation.load(Ordering::Relaxed).checked_add(1)else{checked.restore=EXHAUSTED;self.cursor.fetch_add(1,Ordering::Relaxed);receive_original_retained_work(cx,control())?;return Ok(false)};
  let admission=checked.owners().credit.admit(cx);
  if checked.owners().credit.is_admitted(){slot.generation.store(generation,Ordering::Relaxed);slot.identity.store(identity,Ordering::Relaxed);checked.owners().binding=Some(OriginalSourceBinding::borrow(cx));*output=Some(OriginalNativePathReservation{registry:self,index,generation,active:true,ready:false});checked.restore=CLAIMING;self.cursor.fetch_add(1,Ordering::Relaxed);}
  admission?;Ok(false)
 }
 pub fn retains_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|matches!(slot.state.load(Ordering::Acquire),CHECKED|CLAIMING|RESERVED|ISSUED|RETURNED)&&slot.identity.load(Ordering::Acquire)==identity)}
 pub fn has_returned_source(&self,identity:usize)->bool{self.slots.iter().any(|slot|slot.state.load(Ordering::Acquire)==RETURNED&&slot.identity.load(Ordering::Acquire)==identity)}
 pub fn close_returned_step(&self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let identity=cx.original_source_parent()?.source_identity()?;
  for slot in &self.slots{
   if slot.identity.load(Ordering::Acquire)!=identity{continue}let Some(mut checked)=slot.checkout(RETURNED)else{continue};let owners=checked.owners();
   if !owners.binding.is_some_and(|binding|binding.matches(cx)){return Err(refused("native path return changed its original caller context"))}
   if let Some(source)=owners.source.as_ref(){if !source.terminal_is_empty(){source.validate_retirement_context(cx)?}}
   if owners.credit.is_admitted(){let before=cx.retained_progress();owners.credit.settle_return(cx)?;let after=cx.retained_progress();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}})}
   if let Some(path)=owners.path.as_ref(){let d=RetirementDemand{release_bytes:path.capacity(),depth:1,..Default::default()};return close_original_retained_step(cx,d,|_|{drop(owners.path.take());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}})}
   if let Some(source)=owners.source.as_mut(){if !source.terminal_is_empty(){let step=source.close_step(cx.original_source_parent()?,cx)?;return Ok(match step{InteractiveJobCloseStep::Blocked=>InteractiveJobCloseStep::Blocked,_=>InteractiveJobCloseStep::Pending{progress:step.progress()}})}return close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{owners.source.take();control()})}
   return close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{checked.owners().binding=None;checked.restore=VACANT;control()})
  }
  Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }
}
impl<const N:usize> PathReturnTarget for OriginalNativePathReturnRegistry<N>{
 fn identity(&self,index:usize)->usize{self.slots[index].identity.load(Ordering::Acquire)}
 fn return_path(&self,index:usize,generation:u64,path:PathBuf){let slot=&self.slots[index];let mut checked=slot.checkout(ISSUED).expect("selected original path retains its exclusive return place");assert_eq!(slot.generation.load(Ordering::Relaxed),generation);checked.owners().path=Some(path);checked.owners().credit.record_return().expect("selected path retains its genuine one-shot source return credit");checked.restore=RETURNED;drop(checked);(self.wake)();}
}
pub struct OriginalNativePathReservation<const N:usize>{registry:&'static OriginalNativePathReturnRegistry<N>,index:usize,generation:u64,active:bool,ready:bool}
impl<const N:usize> OriginalNativePathReservation<N>{
 pub fn ready(&self)->bool{self.active&&self.ready}
 pub(super) fn publish_last(&mut self,paths:&mut NativePathSet,output:&mut Option<OriginalSuppliedNativePath>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;let slot=&self.registry.slots[self.index];if slot.generation.load(Ordering::Acquire)!=self.generation||slot.identity.load(Ordering::Acquire)!=parent.source_identity()?{return Err(refused("native selected path changed its original source or generation"))}
  if let Some(owner)=output.as_ref(){
   if owner.target as*const dyn PathReturnTarget as*const()!=self.registry as*const _ as*const()||owner.index!=self.index||owner.generation!=self.generation||self.active{return Err(refused("native selected path output belongs to another original place"))}
   let Some(mut checked)=slot.checkout(ISSUED)else{return Ok(false)};if !checked.owners().binding.is_some_and(|binding|binding.matches(cx)){return Err(refused("cached selected path changed its original caller context"))}checked.owners().source.as_ref().unwrap().validate_context(cx)?;return Ok(true)
  }
  if !self.ready(){return Err(refused("native selected path requires its ready original reservation"))}
  let Some(mut checked)=slot.checkout(RESERVED)else{return Ok(false)};checked.owners().source.as_ref().unwrap().validate_context(cx)?;
  if checked.owners().path.is_none(){if paths.is_empty(){return Ok(false)}if !current(cx){return Ok(false)}let turn=paths.pop_into(&mut checked.owners().path,cx.retained_grant());receive_original_retained_work(cx,turn)?;return Ok(false)}
  if !current(cx){return Ok(false)}*output=Some(OriginalSuppliedNativePath{target:self.registry,index:self.index,generation:self.generation,path:checked.owners().path.take()});self.active=false;checked.restore=ISSUED;receive_original_retained_work(cx,control())?;Ok(true)
 }
}
impl<const N:usize> Drop for OriginalNativePathReservation<N>{
 fn drop(&mut self){if !self.active{return}let slot=&self.registry.slots[self.index];let mut checked=slot.checkout(if self.ready{RESERVED}else{CLAIMING}).expect("native path reservation retains exclusive original custody");assert_eq!(slot.generation.load(Ordering::Relaxed),self.generation);checked.owners().credit.record_return().expect("cancelled path reservation retains its genuine source return credit");checked.restore=RETURNED;drop(checked);(self.registry.wake)();}
}
/// 📖️ Only immutable original path data is borrowed; Drop returns its exact backing to the retained supplier.
pub struct OriginalSuppliedNativePath{target:&'static dyn PathReturnTarget,index:usize,generation:u64,path:Option<PathBuf>}
impl OriginalSuppliedNativePath{pub fn path(&self)->&std::path::Path{self.path.as_ref().unwrap().as_path()}pub fn source_identity(&self)->usize{self.target.identity(self.index)}}
impl std::fmt::Debug for OriginalSuppliedNativePath{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalSuppliedNativePath").field("path",&self.path()).finish()}}
impl Drop for OriginalSuppliedNativePath{fn drop(&mut self){self.target.return_path(self.index,self.generation,self.path.take().unwrap())}}
