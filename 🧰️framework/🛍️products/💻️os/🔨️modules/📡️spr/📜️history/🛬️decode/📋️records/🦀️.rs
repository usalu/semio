//! 📋️ Original typed history records keep payload identities while their contiguous index births and releases remain granted.
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use std::mem::{ManuallyDrop,size_of};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedHistoryRecordsStep{Blocked,Reserved,Published,Returned,Closed}
pub struct RetainedHistoryRecords<T>{records:ManuallyDrop<Vec<T>>,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,closing:bool,closed:bool}
impl<T> RetainedHistoryRecords<T>{
 /// 🎟️ Creates one actual empty typed recipient under the original metadata grant.
 pub fn admit(grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}Ok(Some(Self{records:ManuallyDrop::new(Vec::new()),receipt:Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()})),closing:false,closed:false}))}
 /// 🫴️ Captures the same published index from its real receiving slot under metadata admission.
 pub fn admit_original_index(original:&mut Option<Vec<T>>,grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{if original.is_none()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}Ok(Some(Self{records:ManuallyDrop::new(original.take().unwrap()),receipt:Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()})),closing:false,closed:false}))}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 pub fn records(&self)->&[T]{self.records.as_slice()}
 fn bytes(capacity:usize)->Result<usize,ValueError>{capacity.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original history record layout overflow"))}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_depth>=demand.depth&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes}
 /// 📐️ Quotes the whole genuine next allocation and its replaced original backing before touching either owner.
 pub fn demands(&self)->Result<RetirementDemand,ValueError>{
  if self.closed||self.receipt.is_some(){return Ok(Default::default())}
  if self.closing{return Ok(RetirementDemand{release_bytes:if self.records.is_empty(){Self::bytes(self.records.capacity())?}else{0},depth:1,..Default::default()})}
  if self.records.len()<self.records.capacity(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  let capacity=self.records.len().checked_add(self.records.len().max(1)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original history record capacity overflow"))?;
  Ok(RetirementDemand{capacity_bytes:Self::bytes(capacity)?,release_bytes:Self::bytes(self.records.capacity())?,depth:1,..Default::default()})
 }
 fn record_receipt(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.receipt=Some((grant,progress));if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original history record native backing exceeded its original grant").with_retained_progress(progress))}Ok(())}
 /// 🧳️ Reserves first, then moves one exact original payload into its admitted slot on another turn.
 pub fn append_original(&mut self,original:&mut Option<T>,grant:RetainedCloneGrant)->Result<RetainedHistoryRecordsStep,ValueError>{
  if original.is_none()||self.receipt.is_some()||self.closing||self.closed{return Ok(RetainedHistoryRecordsStep::Blocked)}
  let demand=self.demands()?;if !Self::permits(grant,demand){return Ok(RetainedHistoryRecordsStep::Blocked)}
  if self.records.len()==self.records.capacity(){
   let old=Self::bytes(self.records.capacity())?;let additional=self.records.len().max(1);self.records.try_reserve_exact(additional).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original history record allocation failed"))?;let actual=Self::bytes(self.records.capacity())?;
   self.record_receipt(grant,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:actual,released_bytes:old,..Default::default()})?;return Ok(RetainedHistoryRecordsStep::Reserved)
  }
  self.records.push(original.take().unwrap());self.record_receipt(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(RetainedHistoryRecordsStep::Published)
 }
 /// 🫴️ Returns one original record into the caller's real empty slot before any container release.
 pub fn take_original_into(&mut self,recipient:&mut Option<T>,grant:RetainedCloneGrant)->Result<RetainedHistoryRecordsStep,ValueError>{
  if !self.closing||self.closed||self.receipt.is_some()||recipient.is_some()||self.records.is_empty()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedHistoryRecordsStep::Blocked)}
  *recipient=self.records.pop();self.record_receipt(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(RetainedHistoryRecordsStep::Returned)
 }
 /// 📤️ Transfers the unchanged completed contiguous index into its actual original typed receiver.
 pub fn take_ready_into(&mut self,recipient:&mut Option<Vec<T>>,grant:RetainedCloneGrant)->Result<RetainedHistoryRecordsStep,ValueError>{
  if self.closing||self.closed||self.receipt.is_some()||recipient.is_some()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedHistoryRecordsStep::Blocked)}
  *recipient=Some(std::mem::take(&mut*self.records));self.closed=true;self.record_receipt(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(RetainedHistoryRecordsStep::Closed)
 }
 pub fn cancel(&mut self){self.closing=true}
 /// ♻️ Releases only the empty original index, after every record has its paid retained recipient.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedHistoryRecordsStep,ValueError>{
  if self.receipt.is_some(){return Ok(RetainedHistoryRecordsStep::Blocked)}if self.closed{return Ok(RetainedHistoryRecordsStep::Closed)}
  if !self.closing||!self.records.is_empty(){return Ok(RetainedHistoryRecordsStep::Blocked)}let demand=self.demands()?;if !Self::permits(grant,demand){return Ok(RetainedHistoryRecordsStep::Blocked)}
  drop(std::mem::take(&mut*self.records));self.closed=true;self.record_receipt(grant,RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})?;Ok(RetainedHistoryRecordsStep::Closed)
 }
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.receipt.is_none()&&self.records.is_empty()&&(self.records.capacity()==0||size_of::<T>()==0)}
}
impl<T> Drop for RetainedHistoryRecords<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original history records abandoned typed payload or native index custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.records)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
