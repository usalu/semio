//! ⏱️ Replay fixtures run each original turn inside one actual original Work effect.
use semio_framework_value::retained_clone::{RetainedCloneStep,OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value_testing_admission::{EffectBank,Wallet,driven};
struct Producer{maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🏦️ Opens the test's one funded Source bank; every turn of the test runs inside it.
pub(crate) fn bank()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<40,maximum_capacity_bytes:1<<40,maximum_release_bytes:1<<40,maximum_depth:1<<10},1<<40))}
/// 🔒️ Closes the test's bank for real: a live action, partial Work check or unsettled verdict fails the test.
pub(crate) fn finish(bank:EffectBank){if bank.into_wallet().is_err(){panic!("fixture bank closed with a live original action")}}
/// 🪢️ Runs one operation as a genuine original producer invocation inside the bank's Source and returns the performed work the effect recorded.
pub(crate) fn with_effect<R>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,u128){
 let(mut operation,mut output,mut work)=(Some(operation),None,0u128);
 let(poll,_)=driven::<ValueError>(bank,grant,maximum,|effect,reply,_|{let before=effect.performed_work();let run=operation.take().ok_or_else(||ValueError::invariant("fixture operation runs once"))?;output=Some(run(effect));work=effect.performed_work()-before;Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))});
 if !matches!(poll,RetainedWorkPoll::Complete(ref turn)if turn.result.is_ok()){panic!("fixture effect turn did not complete")}
 (output.take().expect("fixture operation ran"),work)
}
/// 🏁️ A fixture drives every retirement turn to completion, so a suspended poll is a fixture fault.
pub(crate) fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// ♻️ Settles one granted retirement turn inside its own original effect.
pub(crate) fn drive(bank:&mut EffectBank,grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<RetainedCloneStep,OriginalRetirementFailure>{settled(with_effect(bank,grant,1<<20,run).0).result}
/// 🔎️ Settles one granted retirement turn and reports its exact physical allocations.
pub(crate) fn drive_observed(bank:&mut EffectBank,grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->(Result<RetainedCloneStep,OriginalRetirementFailure>,semio_framework_trace::HeapAllocationObservation){let((poll,heap),_)=with_effect(bank,grant,1<<20,|effect|semio_framework_trace::observe_heap_allocations_on_this_thread(||run(effect)));(settled(poll).result,heap)}
/// 🎟️ Lends the decoder one independent original work loan for the duration of the operation.
pub(crate) fn with_work<R>(maximum:u64,control:&mut NativeDecodeControl<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>)->R)->R{
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 authority.with_loan(&mut (),|loan|control.with_work_recipient(loan,|control|Ok::<_,ValueError>(run(control)))).unwrap().unwrap()
}
