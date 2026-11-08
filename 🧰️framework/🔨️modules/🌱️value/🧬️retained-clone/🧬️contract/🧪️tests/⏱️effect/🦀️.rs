//! ⏱️ Test-only adapters drive one actual cursor turn inside one original effect and keep every law's grant, work and receipt assertions. See [`semio_framework_value_clone_source_testing::with_effect`].
use super::*;
use semio_framework_value_retirement::OriginalRetirementOutcome;
pub use semio_framework_value_clone_source_testing::{settled,with_effect};

pub const CPU:u64=1048576;
pub const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:33554432,maximum_depth:256};

/// 🎟️ Runs one immutable-source advance under `grant` and the same work limit the cursor quotes.
pub fn advance<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,grant:RetainedCloneGrant)->OriginalRetirementOutcome{let mut actual=None;with_effect(grant,maximum,&mut actual,|effect|cursor.advance(source,body,maximum,effect)).0}

/// 📤️ Runs one ready-owner transfer and returns the moved owner with its receipt, `None` for a denied or cached turn.
pub fn take<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>,ValueError>{
 let mut actual=None;let mut output=None;let mut slot=None;
 let(called,work)=with_effect(grant,CPU,&mut actual,|effect|{let reply=OriginalEffectReply::borrow(&mut slot)?;cursor.take(&mut output,effect,reply).map(|_|())});
 let result=called.and_then(|()|slot.take().expect("fixture take stayed pending")).map(|step|output.map(|value|(value,step.progress())));
 RetainedWorkOutcome{result,work}
}

/// 🚦️ Runs the admitted close-start action.
pub fn begin_close_paid<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{let mut actual=None;let(turn,_)=with_effect(grant,CPU,&mut actual,|effect|cursor.begin_close_paid(effect));turn}

/// ♻️ Runs one actual close step to completion.
pub fn close_step<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,grant:RetainedCloneGrant)->OriginalRetirementOutcome{let mut actual=None;settled(with_effect(grant,CPU,&mut actual,|effect|cursor.close_step(effect)).0)}

/// 🎯️ Runs one body-selected close to completion.
pub fn close_selected<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,body:usize,grant:RetainedCloneGrant)->OriginalRetirementOutcome{let mut actual=None;settled(with_effect(grant,CPU,&mut actual,|effect|cursor.close_selected(body,effect)).0)}

/// 🚦️ Runs the admitted close-start of one close frame over its owner.
pub fn begin_granted<T:semio_framework_value_retirement::retirement::RetireOwned>(close:&mut RetainedCloneClose,value:&mut Option<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneStep>>{let mut actual=None;with_effect(grant,CPU,&mut actual,|effect|close.begin_granted(value,effect)).0}

/// ♻️ Runs one close frame step to completion.
pub fn step_granted(close:&mut RetainedCloneClose,grant:RetainedCloneGrant)->OriginalRetirementOutcome{let mut actual=None;settled(with_effect(grant,CPU,&mut actual,|effect|close.step_granted(effect)).0)}

/// 🪢️ Runs any operation inside one actual original effect over a fresh allocation marker.
pub fn run<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{let mut actual=None;with_effect(grant,CPU,&mut actual,operation).0}

/// 🏁️ Runs a suspendable retirement turn to completion inside one actual original effect.
pub fn settle<T,E>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{settled(run(grant,operation))}

/// 🚦️ Quotes then runs the close-start of one cursor under exactly its own quote and returns the actual work.
pub fn begin_close<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C)->u128{let d=cursor.begin_close_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let turn=begin_close_paid(cursor,grant);turn.result.unwrap();turn.work}

/// 📄️ Reports whether one funded list reservation made progress.
pub fn progressed(outcome:&semio_framework_value_paged_list::list::PagedListReservationOutcome)->bool{matches!(outcome,semio_framework_value_paged_list::list::PagedListReservationOutcome::Ready(progress) if progress.progressed)}
