//! 🎟️ Nested producers borrow the caller's same effect marker and unchanged authority.
use super::{OriginalWorkSink,RetainedCloneGrant,ValueError,ValueRefusalKind};
/// ⏱️ Opaque observation of one original performed-work receipt.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkPerformedBoundary{identity:super::OriginalWorkPerformedIdentity,observed:u128}
/// 🧵️ Opaque cumulative observation tied to one original Work action.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkActionBoundary{action:super::OriginalWorkActionIdentity,observed:u128}
/// 🪢️ One effect frame owns the allocation attempt while nested producers only borrow its marker.
pub struct RetainedAllocationEffect<'a>{grant:RetainedCloneGrant,maximum_work_units:u64,actual:&'a mut Option<usize>,work:&'a mut dyn OriginalWorkSink,work_start:u128,invocation:Option<super::OriginalEffectInvocation<'a>>,forward_progress:Option<&'a mut super::RetainedCloneProgress>,construction_action:Option<super::OriginalConstructionActionIdentity>}
impl<'a> RetainedAllocationEffect<'a>{
 /// 🫴️ Borrows the genuine receiving callback's marker before the selected producer runs.
 pub fn new(grant:RetainedCloneGrant,maximum_work_units:u64,actual:&'a mut Option<usize>,work:&'a mut dyn OriginalWorkSink)->Self{let work_start=work.observed_work();Self{grant,maximum_work_units,actual,work,work_start,invocation:None,forward_progress:None,construction_action:None}}
 /// 🪪️ Borrows only the invocation supplied by the actual live producer action.
 pub fn invocation(&self)->Option<&super::OriginalEffectInvocation<'_>>{self.invocation.as_ref()}
 pub(crate) fn with_invocation<'b>(&'b mut self,identity:&'b super::OriginalEffectInvocationIdentity,mode:super::OriginalEffectCallbackMode,progress:&'b mut super::RetainedCloneProgress)->Result<RetainedAllocationEffect<'b>,ValueError>{let grant=self.grant;let maximum=self.remaining_work_units()?;let mut effect=self.reborrow(grant,maximum)?;effect.invocation=Some(super::OriginalEffectInvocation::new(identity,mode));if effect.forward_progress.is_none(){effect.forward_progress=Some(progress)}Ok(effect)}
 /// 🪪️ Observes only the construction action that lent its actual marker.
 pub fn construction_action(&self)->Option<super::OriginalConstructionActionIdentity>{self.construction_action}
 pub(crate) fn bind_construction_action(&mut self,identity:super::OriginalConstructionActionIdentity){self.construction_action=Some(identity)}
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
 /// 🧵️ Captures the actual cumulative total of this same original Work action.
 pub fn action_boundary(&self)->OriginalWorkActionBoundary{OriginalWorkActionBoundary{action:self.work.identity().action_identity(),observed:self.work.observed_work()}}
 /// 📏️ Measures actual cumulative Work across receipt loans of the same action.
 pub fn performed_since_action(&self,boundary:OriginalWorkActionBoundary)->Result<u128,ValueError>{if boundary.action!=self.work.identity().action_identity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Work action changed"))}self.work.observed_work().checked_sub(boundary.observed).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Work action moved backwards"))}
 /// 🔁️ Admits a recovery turn from whatever grant it has: the recorded quote must fit this grant and the unspent part of its Work must fit what remains, otherwise the close stays Pending without charge.
 pub fn admits_recovery(&self,quote:super::RetirementDemand,work:u64,boundary:OriginalWorkActionBoundary)->Result<bool,ValueError>{let unspent=u128::from(work).saturating_sub(self.performed_since_action(boundary)?);Ok(super::retained_clone_grant_admits(self.grant,quote)&&u128::from(self.remaining_work_units()?)>=unspent)}
 /// 📏️ Measures actual work only against a boundary of this same receipt.
 pub fn performed_since(&self,boundary:OriginalWorkPerformedBoundary)->Result<u128,ValueError>{if boundary.identity!=self.work.identity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original performed work receipt changed"))}self.work.observed_work().checked_sub(boundary.observed).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original performed work moved backwards"))}
 /// ⛽️ Reads the exact remaining work of this effect and its original receipt.
 pub fn remaining_work_units(&self)->Result<u64,ValueError>{let original=self.work.remaining_work_units()?;let spent=self.work.observed_work().checked_sub(self.work_start).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original performed work moved backwards"))?;let local=u128::from(self.maximum_work_units).checked_sub(spent).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original effect already exceeded its work grant"))?;Ok(original.min(u64::try_from(local).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"original effect work does not fit its grant"))?))}
 /// ⏱️ Records actual producer work before any validation or callback.
 pub fn performed(&mut self,delta:u64)->Result<bool,ValueError>{let admitted=self.work.performed(delta)?;let local=self.work.observed_work().checked_sub(self.work_start).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original performed work moved backwards"))?;Ok(admitted&&local<=u128::from(self.maximum_work_units))}
 pub fn record_forward_metadata(&mut self,bytes:usize)->Result<bool,ValueError>{let progress=self.forward_progress.as_deref().ok_or_else(||ValueError::invariant("original producer progress is absent"))?;let copy_bytes=progress.copied_bytes.checked_add(bytes).ok_or_else(||ValueError::invariant("original producer progress overflow"))?;let demand=super::RetirementDemand{copy_bytes,depth:1,..Default::default()};if !super::retained_clone_grant_admits(self.grant,demand)||self.remaining_work_units()?<1{return Ok(false)}if !self.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original forwarding metadata exceeded Work"))}let progress=self.forward_progress.as_deref_mut().expect("original producer progress retained");progress.copied_bytes=copy_bytes;progress.copied_items=1;Ok(true)}
 /// 🔗️ Narrows authority while preserving the original allocation attempt recipient.
 pub fn reborrow(&mut self,grant:RetainedCloneGrant,maximum_work_units:u64)->Result<RetainedAllocationEffect<'_>,ValueError>{
  let remaining=self.remaining_work_units()?;let work_start=self.work.observed_work();if grant.restricted_by(self.grant)!=grant||maximum_work_units>remaining{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"nested allocation effect changes its original authority"))}
  Ok(RetainedAllocationEffect{grant,maximum_work_units,actual:self.actual,work:&mut *self.work,work_start,invocation:self.invocation.as_ref().map(super::OriginalEffectInvocation::reborrow),forward_progress:self.forward_progress.as_deref_mut(),construction_action:self.construction_action})
 }
 /// 🪆️ Lends a nested producer exactly what is left: the grant intersected with this one and the remaining work of this effect and its receipt, so no authority can widen.
 pub fn nested(&mut self,grant:RetainedCloneGrant)->RetainedAllocationEffect<'_>{self.nested_within(grant,u64::MAX)}
 /// 🪆️ Like `nested` but with a tighter authored ceiling; an exhausted or inconsistent receipt yields a zero ceiling that refuses on the first recorded unit.
 pub fn nested_within(&mut self,grant:RetainedCloneGrant,maximum_work_units:u64)->RetainedAllocationEffect<'_>{
  let maximum_work_units=self.remaining_work_units().unwrap_or(0).min(maximum_work_units);let grant=grant.restricted_by(self.grant);let work_start=self.work.observed_work();
  RetainedAllocationEffect{grant,maximum_work_units,actual:self.actual,work:&mut *self.work,work_start,invocation:self.invocation.as_ref().map(super::OriginalEffectInvocation::reborrow),forward_progress:self.forward_progress.as_deref_mut(),construction_action:self.construction_action}
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
