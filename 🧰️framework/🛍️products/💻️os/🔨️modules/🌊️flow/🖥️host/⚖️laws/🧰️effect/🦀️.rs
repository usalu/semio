//! 🧰️ Test-only original effect lending: every host law drives the current effect-shaped producers through one genuine driven action of the shared effect bank.
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkPoll,ValueError};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext};
use semio_framework_value_testing_admission as admission;
thread_local!{static BANK:std::cell::RefCell<Option<admission::EffectBank>>=const{std::cell::RefCell::new(None)};}
fn bank()->admission::EffectBank{admission::EffectBank::new(admission::BankWallet::from_profile(&serde_json::json!({"items":1u64<<30,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})))}
/// 🎟️ Lends one real original effect of `grant`, driven inside a producer action so nested retirement finds its invocation.
pub(crate) fn fx<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 BANK.with(|slot|{let mut bank=slot.borrow_mut().take().unwrap_or_else(bank);let(mut run,mut result)=(Some(run),None);
  let(poll,_)=admission::driven::<ValueError>(&mut bank,grant,1<<20,|effect,reply,_|{result=Some((run.take().expect("single driven action"))(effect));Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))});
  match poll{RetainedWorkPoll::Complete(turn)=>turn.result.expect("driven action"),RetainedWorkPoll::Pending{..}=>panic!("driven action stayed pending")};bank.take_admitted();*slot.borrow_mut()=Some(bank);result.expect("effect call ran")})
}
/// 🔬️ Observes only the producer's own heap traffic inside the lent effect.
pub(crate) fn observed<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,semio_framework_trace::HeapAllocationObservation){fx(grant,|effect|semio_framework_trace::observe_heap_allocations_on_this_thread(||run(effect)))}
/// 🔬️ Like [`observed`], shaped as `(result, born, freed)`.
pub(crate) fn observe_fx<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,usize,usize){let(value,heap)=observed(grant,run);(value,heap.requested_bytes,heap.released_bytes)}
/// 🏁️ Requires a finished poll and returns its result.
pub(crate) fn done<T,E>(poll:RetainedWorkPoll<T,E>)->Result<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn.result,RetainedWorkPoll::Pending{..}=>panic!("synchronous original turn stayed pending")}}
/// 🎟️ A Source-parented original caller that issues every turn exactly one unit of fuel and an unbounded finite stock, for laws whose producers only need their own grant.
pub(crate) fn source_caller(operation:u64)->OriginalJobFixtureCaller{let mut caller=OriginalJobFixtureCaller::admit(OperationId(operation),Generation(9),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<26,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:128},||Some(1),u64::MAX,1);caller.renew_each_turn(1,u64::MAX);caller}
/// 🫧️ Runs one turn under exactly `grant` that pays no origin refresh and answers the callback's value with the physical receipt of the turn.
pub(crate) fn bare<R>(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->R)->(R,RetainedCloneProgress){let mut value=None;caller.bare_turn(grant,|cx|{value=Some(run(cx));Ok(())}).unwrap_or_else(|failure|panic!("original bare turn refused: {failure:?}"));(value.expect("bare turn callback ran"),caller.receipt())}
