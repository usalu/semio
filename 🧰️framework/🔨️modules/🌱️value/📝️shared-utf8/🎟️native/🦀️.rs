//! 🎟️ Native shared text stores its exact owner before reporting any original receipt refusal.
use super::*;
use semio_framework_value_native_owned::OriginalReceivingPrimary;
impl SharedUtf8{
 pub fn receive_original(input:&mut Option<String>,output:&mut Option<Self>,native:&mut crate::NativeDecodeControl<'_>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let admission=(||{if input.is_none()||output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native shared text requires its input and an empty output"))}let d=Self::admission_demand();if !crate::retained_clone::retained_clone_grant_admits(grant,d){return Ok(false)}if native.owned_bytes().checked_add(d.capacity_bytes).is_none_or(|n|n>native.maximum_bytes()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native shared text exceeds its original decoder ceiling"))}native.checkpoint()?;native.prepare_work(1)})();
  match admission{Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Ok(Default::default()),work:0},Err(e)=>return crate::RetainedWorkOutcome{result:Err(e.into()),work:0}}
  let turn=Self::admit(input.take().unwrap(),grant);let result=match turn.result{Ok((owner,p))=>{*output=Some(owner);Ok(crate::RetainedCloneStep::Complete(p))},Err((error,original))=>{*input=Some(original);Err(error)}};
  native.commit_work(turn.work).expect("one concrete shared text birth consumes only its held native work");let turn=native.receive_retained_work(grant,crate::RetainedWorkOutcome{result,work:turn.work});crate::RetainedWorkOutcome{result:turn.result.map(|s|s.progress()),work:turn.work}
 }
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
