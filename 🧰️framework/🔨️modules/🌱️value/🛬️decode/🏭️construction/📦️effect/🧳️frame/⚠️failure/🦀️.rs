//! 🧳️ Complete native failure data moves independently of its metadata acknowledgement.
use super::*;
use semio_framework_value_native_outcome::{NativeReceivingFailure,OriginalRetirementFailure};
fn demand<E:NativeReceivingEffectError,F>()->Result<RetirementDemand,ValueError>{let copy_bytes=[size_of::<Option<NativeReceivingEffectOutcome<E>>>(),size_of::<NativeReceivingEffectOutcome<E>>(),size_of::<Option<NativeReceivingFailure>>(),size_of::<NativeReceivingFailure>(),size_of::<Option<Result<RetainedCloneStep,F>>>(),size_of::<Result<RetainedCloneStep,F>>()].into_iter().try_fold(0usize,|sum,bytes|sum.checked_add(bytes)).ok_or_else(||ValueError::invariant("native failure data handback extent overflow"))?;Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
macro_rules! failure_data{
 ($domain:ty,$variant:ident)=>{
  impl NativeReceivingEffect<$domain>{
   /// 📐️ Prices the complete original outcome, canonical destination and actual metadata reply.
   pub fn failure_transfer_demand<F>()->Result<RetirementDemand,ValueError>{demand::<$domain,F>()}
   /// 📥️ Keeps full failure data separate from every refusal settling its new metadata action.
   pub fn take_failure_into<'reply,F:From<ValueError>>(original:&mut Option<NativeReceivingEffectOutcome<$domain>>,output:&mut Option<NativeReceivingFailure>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,F>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,F>,ValueError>{
    if effect.invocation().is_none()||output.is_some()||!original.as_ref().is_some_and(|outcome|outcome.work==0&&outcome.result.is_err()){return Ok(reply.complete(Err(ValueError::invariant("native failure handback requires its recorded observed failure and empty caller slot").into())))}
    let demand=Self::failure_transfer_demand::<F>()?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
    if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native failure handback exceeded its original Work"))}
    let RetainedWorkOutcome{result:Err(failure),work:0}=original.take().expect("original native failure remains resident")else{unreachable!("checked native failure remains unchanged before its closed handback")};*output=Some(NativeReceivingFailure::$variant(failure));Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))
   }
  }
 };
}
failure_data!(ValueError,ValueEffect);
failure_data!(OriginalRetirementFailure,RetirementEffect);
failure_data!(OriginalReceivingPrimary,ReceivingEffect);
