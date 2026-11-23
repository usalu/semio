//! 🪙️ Original retirement turns of encoder and controlled owners, each funded by one explicit authoring wallet.
use semio_framework_value::retained_clone::OriginalEffectReply;
use semio_framework_value::{AuthoringWallet,NativeEncodeControl,OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};

pub(crate) type Close=RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>;

/// 🪙️ Runs one closure inside one invoked original effect funded by a wallet of exactly this grant.
pub(crate) fn paid<R>(grant:RetainedCloneGrant,run:impl FnOnce(usize,&mut RetainedAllocationEffect<'_>)->R)->R{
 let body=grant.maximum_copy_bytes;
 AuthoringWallet::new(grant,1<<20).run_invoked(grant,1<<10,|effect|run(body,effect)).expect("authoring effect").expect("work quote admitted").output
}

/// 🚫️ Runs one plain original effect of a grant the owner cannot spend; the step must refuse by reporting no progress.
pub(crate) fn denied<R>(grant:RetainedCloneGrant,run:impl FnOnce(usize,&mut RetainedAllocationEffect<'_>)->R)->R{
 AuthoringWallet::new(grant,1<<20).run(grant,1<<10,|effect|run(grant.maximum_copy_bytes,effect)).expect("authoring effect").expect("work quote admitted").output
}

/// ♻️ Closes one turn of the encoder's selected retirement recipient into a local reply slot.
pub(crate) fn close_encode(native:&mut NativeEncodeControl<'_>,grant:RetainedCloneGrant)->Close{
 paid(grant,|body,effect|{let mut slot=None;{let reply=OriginalEffectReply::borrow(&mut slot).unwrap();drop(native.close_retirement_recipient_into(body,effect,reply).unwrap());}RetainedWorkOutcome{result:slot.expect("encode close recorded its reply"),work:effect.performed_work()}})
}

/// 🏁️ Settles one synchronous retirement poll to its completed step; a pending poll is a refused fixture.
pub(crate) fn settled(poll:RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>)->Result<RetainedCloneStep,OriginalRetirementFailure>{
 match poll{RetainedWorkPoll::Complete(turn)=>turn.result,RetainedWorkPoll::Pending{..}=>panic!("synchronous fixture retirement stayed pending")}
}
