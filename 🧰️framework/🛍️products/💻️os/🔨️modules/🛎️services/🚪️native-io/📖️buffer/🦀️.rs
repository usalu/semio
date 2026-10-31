//! 📖️ Native reads borrow only byte prefixes initialized under the actual current copy grant.
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
use std::mem::MaybeUninit;
pub(super) struct NativeReadBuffer{bytes:[MaybeUninit<u8>;JOB_PAYLOAD_PAGE_BYTES],initialized:usize}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"native read prefix exceeds its original fixed storage")}
impl NativeReadBuffer{
 pub(super) fn birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:std::mem::size_of::<Self>(),depth:1,..Default::default()}}
 pub(super) fn admit(output:&mut Option<Box<Self>>,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let d=Self::birth_demand();if g.maximum_items==0||g.maximum_capacity_bytes<d.capacity_bytes||g.maximum_depth<d.depth{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let layout=std::alloc::Layout::new::<Self>();let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<Self>())else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"native read backing allocation failed")),work:1}};
  unsafe{pointer.as_ptr().write(Self::new());*output=Some(Box::from_raw(pointer.as_ptr()));}RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:1}
 }
 pub(super) fn release_demand(output:&Option<Box<Self>>)->RetirementDemand{if output.is_some(){RetirementDemand{release_bytes:std::mem::size_of::<Self>(),depth:1,..Default::default()}}else{Default::default()}}
 pub(super) fn release(output:&mut Option<Box<Self>>,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if output.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let d=Self::release_demand(output);if g.maximum_items==0||g.maximum_release_bytes<d.release_bytes||g.maximum_depth<d.depth{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  drop(output.take());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}
 }
 pub(super) fn new()->Self{Self{bytes:[MaybeUninit::uninit();JOB_PAYLOAD_PAGE_BYTES],initialized:0}}
 pub(super) fn initialized_len(&self)->usize{self.initialized}
 pub(super) fn initialization_demand(&self,length:usize,body:usize)->Result<RetirementDemand,ValueError>{
  if length>JOB_PAYLOAD_PAGE_BYTES{return Err(invalid())}
  if self.initialized>=length{return Ok(Default::default())}
  Ok(RetirementDemand{copy_bytes:(length-self.initialized).min(body.max(1)),depth:1,..Default::default()})
 }
 pub(super) fn initialize_prefix(&mut self,length:usize,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome{
  let d=match self.initialization_demand(length,body){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if self.initialized>=length{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if g.maximum_items==0||g.maximum_copy_bytes<d.copy_bytes||g.maximum_depth<d.depth{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  unsafe{std::ptr::write_bytes(self.bytes.as_mut_ptr().add(self.initialized).cast::<u8>(),0,d.copy_bytes)}self.initialized+=d.copy_bytes;
  let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};RetainedWorkOutcome{result:Ok(if self.initialized>=length{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}),work:1}
 }
 pub(super) fn initialized_prefix(&self,length:usize)->Result<&[u8],ValueError>{if length>self.initialized{return Err(invalid())}Ok(unsafe{std::slice::from_raw_parts(self.bytes.as_ptr().cast::<u8>(),length)})}
 pub(super) fn initialized_prefix_mut(&mut self,length:usize)->Result<&mut[u8],ValueError>{if length>self.initialized{return Err(invalid())}Ok(unsafe{std::slice::from_raw_parts_mut(self.bytes.as_mut_ptr().cast::<u8>(),length)})}
}
#[cfg(test)]
#[path="tests.rs"]
mod tests;
