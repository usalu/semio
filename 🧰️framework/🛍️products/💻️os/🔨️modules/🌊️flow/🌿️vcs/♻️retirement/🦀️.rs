//! 🌿️ Full original VCS custody forwards every actual retained receipt.

use super::{FlowVcsAction,FlowVcsFault,FlowHostSnapshot,WidgetLayout,LayoutUpdate};
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close},retirement::controlled::ControlledRetirement};

#[derive(semio_framework_value::RetireOwned)]
pub(super) enum FlowVcsClosingOwner{Action(FlowVcsAction),LayoutUpdate(LayoutUpdate<WidgetLayout>),Snapshot(FlowHostSnapshot)}
pub(super) type FlowVcsRetirement=Option<ControlledRetirement<FlowVcsClosingOwner>>;

/// 📏️ Four independent authorities for the original next VCS close turn.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct FlowVcsCloseDemands{pub copy_bytes:usize,pub capacity_bytes:usize,pub release_bytes:usize,pub depth:usize}

/// 🧾️ A failed original VCS close retains every actual allocator effect in its receipt.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FlowVcsCloseFailure{pub fault:FlowVcsFault,pub retained_progress:RetainedCloneProgress}
impl From<FlowVcsFault> for FlowVcsCloseFailure{fn from(fault:FlowVcsFault)->Self{Self{fault,retained_progress:Default::default()}}}
impl From<ValueError> for FlowVcsCloseFailure{fn from(error:ValueError)->Self{Self{fault:FlowVcsFault::ClosePending,retained_progress:error.retained_progress()}}}

pub(super) fn owner_demands(owner:&FlowVcsRetirement,copy:usize)->Result<FlowVcsCloseDemands,FlowVcsCloseFailure>{
 let Some(owner)=owner.as_ref().filter(|owner|!owner.terminal_is_empty())else{return Ok(FlowVcsCloseDemands{depth:1,..Default::default()})};
 Ok(FlowVcsCloseDemands{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or(FlowVcsFault::Depth)?})
}

pub(super) fn admit_turn(grant:RetainedCloneGrant,demands:FlowVcsCloseDemands)->Result<bool,FlowVcsCloseFailure>{
 if !<FlowVcsClosingOwner as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original VCS owner lacks declared controlled retirement authority").into())}
 if grant.maximum_items==0{return Ok(false)}
 if grant.maximum_depth<demands.depth{return Err(FlowVcsFault::Depth.into())}
 Ok(grant.maximum_copy_bytes>=demands.copy_bytes&&grant.maximum_capacity_bytes>=demands.capacity_bytes&&grant.maximum_release_bytes>=demands.release_bytes)
}

pub(super) fn owner_step(owner:&mut FlowVcsRetirement,grant:RetainedCloneGrant)->Result<RetainedCloneStep,FlowVcsCloseFailure>{
 if owner.as_ref().is_some_and(|owner|!owner.terminal_is_empty()){
  let demands=owner_demands(owner,grant.maximum_copy_bytes)?;
  if !admit_turn(grant,FlowVcsCloseDemands{copy_bytes:0,..demands})?{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let owner=owner.as_mut().unwrap();let step=owner.step(child)?;
  let step=admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original VCS retained owner")?;
  return Ok(RetainedCloneStep::Progress(step.progress()))
 }
 if !admit_turn(grant,FlowVcsCloseDemands{depth:1,..Default::default()})?{return Ok(RetainedCloneStep::Progress(Default::default()))}
 *owner=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
}

pub(super) fn retain_owner(owner:&mut FlowVcsRetirement,value:FlowVcsClosingOwner){
 assert!(owner.is_none(),"original VCS inline custody is reserved before handoff");*owner=Some(ControlledRetirement::new(value).unwrap_or_else(|_|unreachable!("original VCS retirement support is checked before source handoff")));
}
