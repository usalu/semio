//! 📤️ An actual output publishes into its caller-resident slot under the same original effect.
use super::*;
use std::mem::size_of;
/// 📏️ Prices both native option headers and the moved original payload header.
pub fn retained_clone_take_demand<T>()->Result<RetirementDemand,ValueError>{let copy_bytes=std::mem::size_of::<Option<T>>().checked_mul(2).and_then(|bytes|bytes.checked_add(std::mem::size_of::<T>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original output header extent overflow"))?;Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
/// 🧳️ Publishes the exact original once before returning any producer status.
pub fn take_retained_clone_output<T>(original:&mut Option<T>,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{
 if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("original output destination occupied")),work:0}}if original.is_none(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let demand=match retained_clone_take_demand::<T>(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}match effect.remaining_work_units(){Ok(0)=>return RetainedWorkOutcome{result:Ok(Default::default()),work:0},Ok(_)=>{},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}}let boundary=effect.performed_boundary();let performed=effect.performed(1);let work=effect.performed_since(boundary).expect("output publication retains its original receipt");match performed{Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"output publication exceeded original Work")),work},Err(error)=>return RetainedWorkOutcome{result:Err(error),work}}*output=original.take();RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),work}
}

#[cfg(test)]
mod tests{use super::*;#[test]fn resident_output_requires_original_work_and_complete_header_copy(){let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../📤️output/🧪️tests/fixture.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){let mut original=Some(fixture["value"].as_u64().unwrap());let mut output=None;let copy=2*std::mem::size_of::<Option<u64>>()+std::mem::size_of::<u64>();let grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:if row["copy"].as_bool().unwrap(){copy}else{0},maximum_depth:row["depth"].as_u64().unwrap()as usize,..Default::default()};let(turn,performed)=crate::close_start::tests::with_effect(grant,row["maximumWork"].as_u64().unwrap(),|effect|take_retained_clone_output(&mut original,&mut output,effect));assert_eq!(turn.work,performed);assert_eq!(output.is_some(),row["moved"].as_bool().unwrap());assert_eq!(original.is_none(),row["moved"].as_bool().unwrap());assert_eq!(turn.work,u128::from(row["work"].as_u64().unwrap()));if let Some(value)=output{assert_eq!(serde_json::to_value(value).unwrap(),fixture["value"]);}}}}

/// 📏️ Prices the original value publication together with its fixed native cursor state write.
pub fn retained_clone_cursor_take_demand<T,F:Copy>()->Result<RetirementDemand,ValueError>{let mut demand=retained_clone_take_demand::<T>()?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<F>()).ok_or_else(||ValueError::invariant("cursor output phase header overflow"))?;Ok(demand)}
/// 📤️ Places the original output and fixed cursor phase before recording its actual reply.
#[macro_export]
macro_rules! take_retained_clone_cursor_output{
 ($ready:expr,$original:expr,$output:expr,$flag:expr,$next:expr,$effect:expr,$reply:expr,$payload:ty,$phase:ty)=>{{
  let ready=$ready;let output=$output;let effect=$effect;let reply=$reply;
  let _: &Option<$payload>=&$original;let _: &$phase=&$flag;
  if !ready{Ok(reply.complete(Ok($crate::RetainedCloneStep::Progress(Default::default()))))}else{
   let demand=$crate::retained_clone_cursor_take_demand::<$payload,$phase>()?;
   if !$crate::retained_clone_grant_admits(effect.grant(),demand){Ok(reply.complete(Ok($crate::RetainedCloneStep::Progress(Default::default()))))}else{
    let turn=$crate::take_retained_clone_output(&mut $original,output,effect);
    let result=turn.result.map(|mut progress|{if output.is_some(){$flag=$next;progress.copied_bytes+=std::mem::size_of::<$phase>();$crate::RetainedCloneStep::Complete(progress)}else{$crate::RetainedCloneStep::Progress(progress)}});
    Ok(reply.complete(result))
   }
  }
 }};
}
