//! 🧳️ Paid original retirement failure custody.
use crate::{ValueError,ValueRefusalKind,NativeAllocationRefusal,RetainedCloneProgress,RetainedWorkOutcome};
use std::mem::MaybeUninit;
#[derive(Debug,PartialEq)]
pub struct OriginalRetirementFailure{primary:Option<ValueError>,work_refusal:Option<ValueError>,receipt_refusal:Option<ValueError>,allocation_refusal:Option<NativeAllocationRefusal>,child:Option<Box<OriginalRetirementFailure>>,selected:RetainedCloneProgress,override_progress:Option<RetainedCloneProgress>}
impl OriginalRetirementFailure{
 fn select(primary:&Option<ValueError>,work:&Option<ValueError>,receipt:&Option<ValueError>,allocation:&Option<NativeAllocationRefusal>,child:&Option<Box<Self>>)->RetainedCloneProgress{child.as_ref().map_or_else(||primary.as_ref().or(work.as_ref()).or(receipt.as_ref()).or_else(||allocation.as_ref().and_then(NativeAllocationRefusal::owned)).map_or(Default::default(),ValueError::retained_progress),|child|child.retained_progress())}
 pub fn from_primary(primary:ValueError)->Self{Self::from_causes(Some(primary),None,None,None)}
 pub fn from_work_refusal(refusal:ValueError)->Self{Self::from_causes(None,Some(refusal),None,None)}
 pub fn from_allocation_refusal(refusal:NativeAllocationRefusal)->Self{Self::from_causes(None,None,None,Some(refusal))}
 pub fn from_primary_and_work(primary:ValueError,refusal:ValueError)->Self{Self::from_causes(Some(primary),Some(refusal),None,None)}
 pub fn from_primary_and_receipt(primary:ValueError,refusal:ValueError)->Self{Self::from_causes(Some(primary),None,Some(refusal),None)}
 pub fn from_causes(primary:Option<ValueError>,work_refusal:Option<ValueError>,receipt_refusal:Option<ValueError>,allocation_refusal:Option<NativeAllocationRefusal>)->Self{Self::from_owned_parts(primary,work_refusal,receipt_refusal,allocation_refusal,None,None)}
 /// 🧳️ Transfers every original cause, nested owner and explicit enclosing progress unchanged.
 pub fn from_owned_parts(primary:Option<ValueError>,work_refusal:Option<ValueError>,receipt_refusal:Option<ValueError>,allocation_refusal:Option<NativeAllocationRefusal>,child:Option<Box<Self>>,override_progress:Option<RetainedCloneProgress>)->Self{let selected=Self::select(&primary,&work_refusal,&receipt_refusal,&allocation_refusal,&child);Self{primary,work_refusal,receipt_refusal,allocation_refusal,child,selected,override_progress}}
 pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref()}
 pub fn allocation_refusal(&self)->Option<&NativeAllocationRefusal>{self.allocation_refusal.as_ref()}
 pub fn primary(&self)->Option<&ValueError>{self.primary.as_ref()}
 pub fn work_refusal(&self)->Option<&ValueError>{self.work_refusal.as_ref()}
 pub fn child(&self)->Option<&Self>{self.child.as_deref()}
 pub fn with_work_refusal(mut self,refusal:ValueError)->Result<Self,(Self,ValueError)>{if self.work_refusal.is_some(){return Err((self,refusal))}if self.child.is_none()&&self.primary.is_none(){self.selected=refusal.retained_progress()}self.work_refusal=Some(refusal);Ok(self)}
 pub fn into_parts(self)->(Option<ValueError>,Option<ValueError>,Option<ValueError>,Option<NativeAllocationRefusal>,Option<Box<Self>>,Option<RetainedCloneProgress>){(self.primary,self.work_refusal,self.receipt_refusal,self.allocation_refusal,self.child,self.override_progress)}
 pub fn with_retained_progress(mut self,progress:RetainedCloneProgress)->Self{self.override_progress=Some(progress);self}
 /// 📏️ Borrows the enclosing span only when it was explicitly recorded.
 pub fn progress_override(&self)->Option<RetainedCloneProgress>{self.override_progress}
 pub fn retained_progress(&self)->RetainedCloneProgress{self.override_progress.unwrap_or(self.selected)}
 /// 🧯️ Collapses to the single most fundamental ValueError (primary, work, receipt, allocation, then child) for callers whose contract is a plain ValueError; the retained progress travels with it.
 pub fn into_value_error(self)->ValueError{let progress=self.retained_progress();let (primary,work,receipt,allocation,child,_)=self.into_parts();primary.or(work).or(receipt).or_else(||child.map(|child|child.into_value_error())).unwrap_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,if allocation.is_some(){"original retirement allocation was refused"}else{"original retirement failed without a cause"})).with_retained_progress(progress)}
}
impl From<ValueError> for OriginalRetirementFailure{fn from(error:ValueError)->Self{Self::from_primary(error)}}
impl From<NativeAllocationRefusal> for OriginalRetirementFailure{fn from(error:NativeAllocationRefusal)->Self{Self::from_allocation_refusal(error)}}
impl std::fmt::Display for OriginalRetirementFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{if let Some(error)=self.primary.as_ref().or(self.work_refusal.as_ref()).or(self.receipt_refusal.as_ref()){return std::fmt::Display::fmt(error,f)}if let Some(refusal)=&self.allocation_refusal{return match refusal{NativeAllocationRefusal::Owned(error)=>std::fmt::Display::fmt(error,f),NativeAllocationRefusal::Retained(_)=>f.write_str("original allocation refusal remains with its owner")}}match &self.child{Some(child)=>std::fmt::Display::fmt(child,f),None=>f.write_str("original retirement failed without a cause")}}}
impl std::error::Error for OriginalRetirementFailure{}
/// 🎟️ An admitted node is retained until it is attached or explicitly released.
#[derive(Debug)]
pub struct OriginalFailureStorage(std::mem::ManuallyDrop<Box<MaybeUninit<OriginalRetirementFailure>>>);
impl OriginalFailureStorage{
 pub const fn birth_bytes()->usize{std::mem::size_of::<OriginalRetirementFailure>()}
 /// 📏️ Prices the physical marker writes and the returned node header.
 pub const fn admission_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<usize>>()+std::mem::size_of::<Self>(),capacity_bytes:Self::birth_bytes(),release_bytes:0,depth:1}}
 /// 🧺️ Prices allocation and publication into the original intrusive spare pool.
 pub const fn birth_demand()->crate::RetirementDemand{let mut demand=Self::admission_demand();demand.copy_bytes+=2*std::mem::size_of::<Option<Box<MaybeUninit<OriginalRetirementFailure>>>>();demand}
 /// 🧵️ Uses the caller's actual sealed receipt before the system allocation leaf.
 pub fn admit(effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<(Self,RetainedCloneProgress)>{
  if effect.actual().is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original failure node still owns a physical receipt")),work:0}}
  let demand=Self::admission_demand();if effect.maximum_work_units()==0||!crate::retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original failure node exceeds its admitted physical grant")),work:0}}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original failure allocation exceeds its original work receipt")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}}
  let layout=std::alloc::Layout::new::<MaybeUninit<OriginalRetirementFailure>>();*effect.actual()=Some(0);
  let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<MaybeUninit<OriginalRetirementFailure>>())else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original failure node allocation failed").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<usize>>(),..Default::default()})),work:1}};
  *effect.actual()=Some(demand.capacity_bytes);let node=unsafe{Box::from_raw(pointer.as_ptr())};RetainedWorkOutcome{result:Ok((Self(std::mem::ManuallyDrop::new(node)),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})),work:1}
 }

 pub fn attach(self,child:OriginalRetirementFailure,refusal:ValueError)->OriginalRetirementFailure{self.attach_owned_parts(child,None,Some(refusal),None,None)}
 /// 🪢️ Installs the original child into already-admitted storage while retaining every independent parent cause without allocation.
 /// 🧳️ Consumes one prepaid node and transfers the complete parent and child causes.
 pub fn attach_owned_parts(self,child:OriginalRetirementFailure,primary:Option<ValueError>,work_refusal:Option<ValueError>,receipt_refusal:Option<ValueError>,allocation_refusal:Option<NativeAllocationRefusal>)->OriginalRetirementFailure{let selected=child.retained_progress();let mut storage=std::mem::ManuallyDrop::new(self);let mut node=unsafe{std::mem::ManuallyDrop::take(&mut storage.0)};node.write(child);let child=unsafe{node.assume_init()};OriginalRetirementFailure{primary,work_refusal,receipt_refusal,allocation_refusal,child:Some(child),selected,override_progress:None}}
 pub fn release(self,release:usize)->Result<RetainedCloneProgress,Self>{let bytes=Self::birth_bytes();if release<bytes{return Err(self)}let mut storage=std::mem::ManuallyDrop::new(self);unsafe{std::mem::ManuallyDrop::drop(&mut storage.0)};Ok(RetainedCloneProgress{released_bytes:bytes,..Default::default()})}
}
impl Drop for OriginalFailureStorage{fn drop(&mut self){assert!(std::thread::panicking(),"original failure storage requires paid release or transfer");unsafe{std::mem::ManuallyDrop::drop(&mut self.0)}}}

/// 🧺️ One preborn spare remains in its actual recipient slot through success and unwinding.
#[derive(Default)]
pub struct OriginalFailureStoragePool{slot:Option<OriginalFailureStorage>}
impl OriginalFailureStoragePool{
 pub fn is_empty(&self)->bool{self.slot.is_none()}
 pub fn spare_slot(&mut self)->&mut Option<OriginalFailureStorage>{&mut self.slot}
 pub fn spare_destination(&self)->(usize,usize){(std::ptr::from_ref(&self.slot)as usize,std::mem::size_of::<Option<OriginalFailureStorage>>())}
 pub fn spare_payload_destination(&self)->Option<(usize,usize)>{self.slot.as_ref().map(|node|(node.0.as_ptr()as usize,OriginalFailureStorage::birth_bytes()))}
 pub fn put(&mut self,storage:OriginalFailureStorage){assert!(self.slot.is_none(),"original failure spare recipient is already occupied");self.slot=Some(storage)}
 pub fn take(&mut self)->Option<OriginalFailureStorage>{self.slot.take()}
 pub fn release_demand(&self)->crate::RetirementDemand{if self.slot.is_none(){Default::default()}else{crate::RetirementDemand{copy_bytes:std::mem::size_of::<Option<OriginalFailureStorage>>()+std::mem::size_of::<OriginalFailureStorage>(),release_bytes:OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()}}}
 pub fn release_work_demand(&self)->u64{if self.slot.is_none(){0}else{3}}
 pub fn release_step(&mut self,grant:crate::RetainedCloneGrant,maximum:u64)->RetainedWorkOutcome{let d=self.release_demand();if self.slot.is_none(){return RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0}}if maximum<3||!crate::retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0}}let storage=self.slot.take().expect("funded original spare remains in its slot");let mut storage=std::mem::ManuallyDrop::new(storage);unsafe{std::mem::ManuallyDrop::drop(&mut storage.0)};RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:3}}
 pub fn release_one(&mut self,grant:usize)->Result<RetainedCloneProgress,ValueError>{if grant<OriginalFailureStorage::birth_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"failure node release exceeds admitted grant"))}match self.slot.take(){Some(storage)=>match storage.release(grant){Ok(progress)=>Ok(progress),Err(storage)=>{self.slot=Some(storage);Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"failure node release exceeds admitted grant"))}},None=>Ok(Default::default())}}
}
impl Drop for OriginalFailureStoragePool{fn drop(&mut self){assert!(std::thread::panicking()||self.slot.is_none(),"original failure pool retains an unpaid spare");}}
/// 🧾️ Reserves failure custody before one effect and its actual work settlement.
pub fn run_with_failure_storage_transaction<T:Copy>(storage:&mut Option<OriginalFailureStorage>,action:impl FnOnce()->(RetainedWorkOutcome<T,OriginalRetirementFailure>,Result<(),ValueError>),effects:impl FnOnce(&T)->RetainedCloneProgress)->Option<RetainedWorkOutcome<T,OriginalRetirementFailure>>{
 if storage.is_none(){return None}let(turn,settlement)=action();let result=match(turn.result,settlement){
  (result,Ok(()))=>result,
  (Ok(value),Err(refusal))=>Err(OriginalRetirementFailure::from_work_refusal(refusal.with_retained_progress(effects(&value)))),
  (Err(child),Err(refusal))=>match child.with_work_refusal(refusal){Ok(failure)=>Err(failure),Err((child,refusal))=>Err(storage.take().expect("disjoint original failure slot remains available").attach(child,refusal))},
 };Some(RetainedWorkOutcome{result,work:turn.work})
}
/// 🧮️ Runs a child and its settlement using the same reserved failure custody.
pub fn run_with_failure_storage<T:Copy>(storage:&mut Option<OriginalFailureStorage>,child:impl FnOnce()->RetainedWorkOutcome<T,OriginalRetirementFailure>,settle:impl FnOnce(u128)->Result<(),ValueError>,effects:impl FnOnce(&T)->RetainedCloneProgress)->Option<RetainedWorkOutcome<T,OriginalRetirementFailure>>{
 run_with_failure_storage_transaction(storage,||{let turn=child();let settlement=settle(turn.work);(turn,settlement)},effects)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod custody_tests;

#[path="👶️birth/🦀️.rs"]
mod birth;
pub use birth::{OriginalFailureStorageBirth,OriginalFailureStorageBirthPhase,OriginalFailurePerformedScope,OriginalFailureStorageStep,OriginalFailureFunding,refill_original_failure_storage};
