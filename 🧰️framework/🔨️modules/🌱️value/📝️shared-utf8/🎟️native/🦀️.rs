//! 🎟️ Native shared text stores its exact owner before reporting any original receipt refusal.
use super::*;
impl SharedUtf8{
 pub fn receive_original(input:&mut Option<String>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneProgress,ValueError>{
  let grant=effect.grant();
  if input.is_none()||output.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native shared text requires its input and an empty output")),work:0}}
  if !crate::retained_clone::retained_clone_grant_admits(grant,Self::admission_demand())||effect.maximum_work_units()==0{return crate::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let before=effect.performed_work();let performed=effect.performed(1);let work=effect.performed_work().saturating_sub(before);
  match performed{Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native shared text exceeded its original work")),work},Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work}}
  let Some(text)=input.take()else{return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("native shared text input absent")),work}};
  let turn=Self::admit(text,grant,effect.actual());
  let result=match turn.result{Ok((owner,progress))=>{*output=Some(owner);Ok(progress)},Err((error,original))=>{*input=Some(original);Err(error)}};
  crate::RetainedWorkOutcome{result,work}
 }
}
