//! 🏦️ Grant-shaped views over every effect-driven retirement action keep each grant-level law of the child-group laws.
//! @see [`semio_framework_value_clone_source_testing::drive::drive`]
use semio_framework_value::{ErasedSnapshotRetirement,OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use semio_framework_value_clone_source_testing::drive::drive;
use semio_os_kernel::os_spr::operation_bytes::{OperationByteCloseStep,OwnedOperationBytes};

/// 🎟️ One item, the physical 4096-byte copy and release currencies and the maintained depth.
pub(crate) const PAGE:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64};
/// 🎟️ No item at all, so every action must yield without spending anything.
pub(crate) const NONE:RetainedCloneGrant=RetainedCloneGrant{maximum_items:0,..PAGE};
const UNITS:u64=1<<10;
fn settle(turn:RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>)->Result<RetainedCloneStep,ValueError>{turn.result.map_err(OriginalRetirementFailure::into_value_error)}

/// 🔁️ Runs one erased owner close under the grant through the real original Work authority.
pub(crate) fn step(owner:&mut dyn ErasedSnapshotRetirement,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{settle(drive(grant,UNITS,|effect|owner.close_step(4096,effect)))}

/// 🔁️ Closes one scalar or backing of an owned operation source under the grant.
pub(crate) fn bytes(source:&mut OwnedOperationBytes,grant:RetainedCloneGrant)->Result<OperationByteCloseStep,ValueError>{
    let mut closed=None;
    settle(drive(grant,UNITS,|effect|{let turn=source.close_one(effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|{let reply=match &step{OperationByteCloseStep::Pending{released_items,released_bytes}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:*released_items,released_bytes:*released_bytes,..Default::default()}),OperationByteCloseStep::Complete=>RetainedCloneStep::Complete(Default::default())};closed=Some(step);reply}).map_err(Into::into),work:turn.work})}))?;
    closed.ok_or_else(||ValueError::invariant("original operation bytes close recorded no step"))
}

/// 🧹️ Drains an owned operation source with the physical page grant.
pub(crate) fn drain_bytes(source:&mut OwnedOperationBytes){for _ in 0..source.len()+256{if matches!(bytes(source,PAGE).unwrap(),OperationByteCloseStep::Complete){break}}assert!(source.terminal_is_empty());}

/// ♻️ Retires one scalar or one backing of a borrowed capsule and reports its exact `(more, items, released bytes)` receipt.
pub(crate) fn retire(grant:RetainedCloneGrant,mut operation:impl FnMut(&mut RetainedAllocationEffect<'_>)->Result<(bool,usize,usize),ValueError>)->Result<(bool,usize,usize),ValueError>{
    let mut last=None;
    settle(drive(grant,UNITS,|effect|{let result=operation(effect).map(|(more,items,released)|{last=Some((more,items,released));RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:items,released_bytes:released,..Default::default()})}).map_err(Into::into);RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:0})}))?;
    last.ok_or_else(||ValueError::invariant("original capsule retirement recorded no receipt"))
}

/// 🧹️ Drains one erased retained owner with the physical page grant and returns its released bytes.
pub(crate) fn drain_retained(owner:&mut dyn semio_framework_value::ErasedSnapshotRetirement)->usize{
    let mut released=0;for _ in 0..100000{match step(owner,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(){RetainedCloneStep::Complete(_)=>break,RetainedCloneStep::Progress(progress)=>{let released_items=progress.copied_items;let released_bytes=progress.released_bytes;assert!(progress.copied_bytes<=4096);assert_eq!(progress.retained_capacity_bytes,0);assert!(released_items<=1);assert!(released_bytes<=4096);released+=released_bytes;},step=>panic!("retained child owner refused its genuine physical page grant: {step:?}")}}assert!(owner.terminal_is_empty());released
}
