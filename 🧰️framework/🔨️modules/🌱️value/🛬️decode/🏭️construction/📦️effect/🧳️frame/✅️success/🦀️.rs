//! ✅️ Acknowledges only the already-settled successful native receipt under its separately funded action.
use super::*;
/// 🪙️ The bytes a recorded successful outcome really holds: its step, its Work and its tags, never the room reserved for a failure it does not carry.
fn success_outcome_extent()->usize{size_of::<RetainedCloneStep>()+size_of::<u128>()+2*size_of::<u8>()}
impl<E:NativeReceivingEffectError> NativeReceivingEffect<E>{
 /// 📐️ Prices the live successful outcome and the live successful caller reply, which is all this acknowledgement moves.
 pub fn acknowledge_success_demand<F>()->Result<RetirementDemand,ValueError>{let outcome=success_outcome_extent();let reply=size_of::<RetainedCloneStep>()+size_of::<u8>();let copy_bytes=outcome.checked_mul(2).and_then(|n|n.checked_add(reply.checked_mul(2)?)).ok_or_else(||ValueError::invariant("native success acknowledgement extent overflow"))?;Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 /// 📬️ Records only this metadata action; the original semantic receipt has already settled.
 pub fn acknowledge_success_into<'reply,F:From<ValueError>>(original:&mut Option<NativeReceivingEffectOutcome<E>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,F>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,F>,ValueError>{
  if effect.invocation().is_none()||!original.as_ref().is_some_and(|outcome|outcome.work==0&&outcome.result.is_ok()){return Ok(reply.complete(Err(ValueError::invariant("native success acknowledgement requires its recorded original result with no unreported work").into())))}
  let demand=Self::acknowledge_success_demand::<F>()?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native success acknowledgement exceeded its original Work"))}
  drop(original.take());Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))
 }
}
