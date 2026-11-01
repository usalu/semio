//! 🎟️ Stages a borrowed authority's funded birth and its paid cancellation before the cursor reads it.

use super::super::{RetainedCloneBorrowAuthority,RetainedCloneBorrowAuthorityAdmission,RetainedCloneGrant,RetainedCloneRef,RetainedCloneSourceTake,RetainedCloneStep};
use crate::{RetainedWorkOutcome,RetirementDemand,ValueError,retirement::{RetireOwned,controlled::ControlledRetirement}};

/// 🪪️ One workspace is admitted, borrowed, taken back or retired without ever being unowned.
pub(super) struct StagedLease<W:RetireOwned+Sync>{admission:Option<RetainedCloneBorrowAuthorityAdmission<W>>,lease:Option<RetainedCloneBorrowAuthority<W>>,retiring:Option<ControlledRetirement<W>>}
impl<W:RetireOwned+Sync> StagedLease<W>{
 pub(super) fn new(workspace:W)->Self{Self{admission:Some(RetainedCloneBorrowAuthority::admission(workspace)),lease:None,retiring:None}}
 pub(super) fn ready(&self)->bool{self.lease.is_some()}
 pub(super) fn birth_demand(&self)->Result<RetirementDemand,ValueError>{self.admission.as_ref().map_or_else(||Ok(Default::default()),RetainedCloneBorrowAuthorityAdmission::next_demand)}
 pub(super) fn advance_birth(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let Some(admission)=self.admission.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  let turn=admission.take_output(&mut self.lease,grant);
  let turn=if self.lease.is_some(){turn}else{admission.advance(grant)};
  if admission.terminal_is_empty(){self.admission=None;}
  turn
 }
 pub(super) fn borrow<T:?Sized,F>(&self,discriminator:usize,project:F)->RetainedCloneRef<'_,T> where F:for<'source>FnOnce(&'source W)->&'source T{self.lease.as_ref().expect("staged lease is ready").borrow(discriminator,project)}
 pub(super) fn next_take_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.lease.as_ref().ok_or_else(||ValueError::invariant("staged lease is ready"))?.next_take_demands(body)}
 pub(super) fn take_authority(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneSourceTake<W>>{match self.lease.as_mut(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("staged lease is ready").into()),work:0}}.take_authority(grant)}
 pub(super) fn next_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(retiring)=self.retiring.as_ref(){return retiring.next_demand(body)}
  if let Some(lease)=self.lease.as_ref(){return lease.next_close_demand(body)}
  self.admission.as_ref().map_or_else(||Ok(Default::default()),RetainedCloneBorrowAuthorityAdmission::next_return_demand)
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if let Some(retiring)=self.retiring.as_mut(){let result=retiring.step(grant);let work=retiring.last_turn_work();if retiring.terminal_is_empty(){self.retiring=None;}return RetainedWorkOutcome{result,work}}
  if let Some(lease)=self.lease.as_mut(){return lease.close_step(grant)}
  let Some(admission)=self.admission.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  let published=admission.take_output(&mut self.lease,grant);
  if self.lease.is_some(){if admission.terminal_is_empty(){self.admission=None;}return published}
  let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);
  if let Some((workspace,()))=returned{self.retiring=Some(ControlledRetirement::retain(workspace));}
  if admission.terminal_is_empty(){self.admission=None;}
  turn
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.admission.is_none()&&self.lease.as_ref().is_none_or(RetainedCloneBorrowAuthority::terminal_is_empty)&&self.retiring.is_none()}
}
