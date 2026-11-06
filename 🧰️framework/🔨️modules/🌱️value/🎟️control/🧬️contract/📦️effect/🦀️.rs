//! 🎟️ Nested producers borrow the caller's same effect marker and unchanged authority.
use super::{OriginalWorkSink,RetainedCloneGrant,ValueError,ValueRefusalKind};
/// ⏱️ Opaque observation of one original performed-work receipt.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkPerformedBoundary{identity:super::OriginalWorkPerformedIdentity,observed:u128}
/// 🪢️ One effect frame owns the allocation attempt while nested producers only borrow its marker.
pub struct RetainedAllocationEffect<'a>{grant:RetainedCloneGrant,maximum_work_units:u64,actual:&'a mut Option<usize>,work:&'a mut dyn OriginalWorkSink,work_start:u128}
impl<'a> RetainedAllocationEffect<'a>{
 /// 🫴️ Borrows the genuine receiving callback's marker before the selected producer runs.
 pub fn new(grant:RetainedCloneGrant,maximum_work_units:u64,actual:&'a mut Option<usize>,work:&'a mut dyn OriginalWorkSink)->Self{let work_start=work.observed_work();Self{grant,maximum_work_units,actual,work,work_start}}
 /// 📏️ Reads the caller's unchanged physical grant.
 pub fn grant(&self)->RetainedCloneGrant{self.grant}
 /// ⛽️ Reads the caller's authored computational ceiling.
 pub fn maximum_work_units(&self)->u64{self.maximum_work_units}
 /// 🧬️ Borrows the same original marker; no nested attempt slot is created.
 pub fn actual(&mut self)->&mut Option<usize>{self.actual}
 /// 🪪️ Observes the identity of the same original performed-work receipt.
 pub fn work_identity(&self)->super::OriginalWorkPerformedIdentity{self.work.identity()}
 /// 🪪️ Observes the actual original Work Source turn when one is bound.
 pub fn work_source_turn(&self)->Option<super::OriginalSourceTurnIdentity>{self.work.source_turn()}
 /// ⏱️ Observes work staged since this original effect boundary.
 pub fn performed_work(&self)->u128{self.work.observed_work()-self.work_start}
 /// 📍️ Captures the current absolute extent of this actual receipt.
 pub fn performed_boundary(&self)->OriginalWorkPerformedBoundary{OriginalWorkPerformedBoundary{identity:self.work.identity(),observed:self.work.observed_work()}}
 /// 📏️ Measures actual work only against a boundary of this same receipt.
 pub fn performed_since(&self,boundary:OriginalWorkPerformedBoundary)->Result<u128,ValueError>{if boundary.identity!=self.work.identity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original performed work receipt changed"))}self.work.observed_work().checked_sub(boundary.observed).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original performed work moved backwards"))}
 /// ⛽️ Reads the exact remaining work of this effect and its original receipt.
 pub fn remaining_work_units(&self)->Result<u64,ValueError>{let original=self.work.remaining_work_units()?;let spent=self.work.observed_work().checked_sub(self.work_start).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original performed work moved backwards"))?;let local=u128::from(self.maximum_work_units).checked_sub(spent).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original effect already exceeded its work grant"))?;Ok(original.min(u64::try_from(local).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"original effect work does not fit its grant"))?))}
 /// ⏱️ Records actual producer work before any validation or callback.
 pub fn performed(&mut self,delta:u64)->Result<bool,ValueError>{let admitted=self.work.performed(delta)?;let local=self.work.observed_work().checked_sub(self.work_start).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original performed work moved backwards"))?;Ok(admitted&&local<=u128::from(self.maximum_work_units))}
 /// 🔗️ Narrows authority while preserving the original allocation attempt recipient.
 pub fn reborrow(&mut self,grant:RetainedCloneGrant,maximum_work_units:u64)->Result<RetainedAllocationEffect<'_>,ValueError>{
  let remaining=self.remaining_work_units()?;let work_start=self.work.observed_work();if grant.restricted_by(self.grant)!=grant||maximum_work_units>remaining{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"nested allocation effect changes its original authority"))}
  Ok(RetainedAllocationEffect{grant,maximum_work_units,actual:self.actual,work:&mut *self.work,work_start})
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
