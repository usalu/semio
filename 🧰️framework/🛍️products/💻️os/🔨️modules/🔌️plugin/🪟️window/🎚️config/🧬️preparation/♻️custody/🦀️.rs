//! ♻️ Retains original domain preparation fields until their independently admitted typed closure.
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,controlled::ControlledRetirement}};
use std::mem::{ManuallyDrop,size_of};
pub(crate) struct WindowEditCustody<T:RetireOwned>{original:ManuallyDrop<Option<T>>,close:Option<ControlledRetirement<T>>,closing:bool}
impl<T:RetireOwned> WindowEditCustody<T>{
 pub fn new(original:T)->Self{Self{original:ManuallyDrop::new(Some(original)),close:None,closing:false}}
 pub fn original(&self)->&T{self.original.as_ref().expect("original domain fields retained")}
 pub fn original_mut(&mut self)->&mut T{self.original.as_mut().expect("original domain fields remain in their preparation frame")}
 pub fn is_closing(&self)->bool{self.closing}
 pub fn begin_close(&mut self)->bool{let started=!self.closing;self.closing=true;started}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.original.is_none()&&self.close.is_none()}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original domain field close depth overflow"))?});}
  Ok(if self.original.is_some(){RetirementDemand{copy_bytes:size_of::<T>()+size_of::<ControlledRetirement<T>>(),depth:1,..Default::default()}}else{Default::default()})
 }
 pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let demand=self.demands(grant.maximum_copy_bytes)?;
  if !self.closing||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original domain fields exceed admitted close depth"));}
  if let Some(owner)=self.close.as_mut(){if owner.terminal_is_empty(){self.close=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}return owner.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()));}
  let original=self.original.take().expect("quoted original domain fields remain");match ControlledRetirement::new(original){Ok(owner)=>{self.close=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))},Err((error,original))=>{*self.original=Some(original);Err(error)}}
 }
}
impl<T:RetireOwned> Drop for WindowEditCustody<T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"window domain original fields require complete admitted closure");}}
