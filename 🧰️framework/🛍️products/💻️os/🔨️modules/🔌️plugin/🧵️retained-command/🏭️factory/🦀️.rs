//! 🏭️ Retained command factories quote physical frame and raw-buffer births before original custody transfer.
use super::{ArtifactApp,ArtifactRetainedCommandJob,ArtifactRetainedCommandPayload,RetainedToolWireInput,StepContext,ValueError,ValueRefusalKind,RetainedCloneProgress};
use semio_framework::action_bus::{ToolJobAdmissionDemand,admit_original_tool_job_measured};
use semio_framework_job::{Operation,OriginalTurnFailure};
use std::mem::size_of;

pub(super) fn quote<P,J>(input:Option<usize>,checkpoint:Option<usize>,maximum_raw_bytes:usize,resume:bool,payload_maximum:usize,payload_admitted:bool)->Result<ToolJobAdmissionDemand,ValueError>{
 if input.is_some_and(|bytes|bytes>maximum_raw_bytes)||checkpoint.is_some_and(|bytes|!resume||input.is_none()||bytes>super::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"retained command factory rejects its wire size or checkpoint policy"));}
 let raw=input.filter(|bytes|payload_admitted&&*bytes<=payload_maximum).unwrap_or(0);
 let mut demand=ToolJobAdmissionDemand::boxed_job::<P,J>();
 demand.capacity_bytes=demand.capacity_bytes.checked_add(raw).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"retained command frame and raw allocation extent overflow"))?;
 Ok(demand)
}

/// 📐️ The owner supplies its explicit wire ceiling and checkpoint policy for the same borrowed payload.
pub fn admission_demands<A:ArtifactApp>(payload:&ArtifactRetainedCommandPayload<A>,input:Option<&RetainedToolWireInput>,checkpoint:Option<&RetainedToolWireInput>,maximum_raw_bytes:usize,resume:bool)->Result<ToolJobAdmissionDemand,ValueError>{
 quote::<ArtifactRetainedCommandPayload<A>,ArtifactRetainedCommandJob<A>>(input.map(RetainedToolWireInput::declared_bytes),checkpoint.map(RetainedToolWireInput::declared_bytes),maximum_raw_bytes,resume,payload.maximum_raw_bytes,payload.admission_refusal.is_none())
}

/// 🧳️ The measured job retains original sources before any post-construction receipt refusal.
pub fn admit_job<A:ArtifactApp>(context:&mut StepContext<'_>,operation:&Operation,payload:&mut Option<Box<ArtifactRetainedCommandPayload<A>>>,input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<ArtifactRetainedCommandJob<A>>>,maximum_raw_bytes:usize,resume:bool)->Result<bool,OriginalTurnFailure>{
 let demand=admission_demands(payload.as_deref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained command factory requires its original payload"))?,input.as_ref(),checkpoint.as_ref(),maximum_raw_bytes,resume)?;
 admit_original_tool_job_measured(context,operation,payload,input,checkpoint,output,demand,|identity,payload,input,checkpoint|{
  let mut job=match(input,checkpoint){(Some(input),Some(checkpoint))=>ArtifactRetainedCommandJob::from_wire_with_checkpoint(*payload,input,checkpoint),(Some(input),None)=>ArtifactRetainedCommandJob::from_wire(*payload,input),(None,None)=>ArtifactRetainedCommandJob::new(*payload),(None,Some(_))=>unreachable!("original checkpoint policy preflight")};
  job.source_binding=Some(identity.source_binding());
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:size_of::<ArtifactRetainedCommandJob<A>>()+job.raw.capacity(),released_bytes:size_of::<ArtifactRetainedCommandPayload<A>>()};
  (Box::new(job),progress)
 })
}

/// 🪢️ Concrete factories declare both required admission methods with their own original wire policy.
#[macro_export]
macro_rules! retained_command_factory_admission{
 (maximum_raw_bytes:|$payload:ident|$maximum:expr,checkpoint_resume:$resume:expr$(,)?)=>{
  fn admission_demands(&self,_operation:&$crate::__job::Operation,payload:&Self::Payload,input:Option<&$crate::action_bus::RetainedToolWireInput>,checkpoint:Option<&$crate::action_bus::RetainedToolWireInput>)->Result<$crate::action_bus::ToolJobAdmissionDemand,$crate::__value::ValueError>{
   let maximum={let $payload=payload;$maximum};
   $crate::retained_command::factory::admission_demands(payload,input,checkpoint,maximum,$resume)
  }
  fn admit_job(&mut self,context:&mut $crate::StepContext<'_>,operation:&$crate::__job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<$crate::action_bus::RetainedToolWireInput>,checkpoint:&mut Option<$crate::action_bus::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,$crate::__job::OriginalTurnFailure>{
   let maximum={let $payload=payload.as_deref().ok_or_else(||$crate::__value::ValueError::literal($crate::__value::ValueRefusalKind::InvariantViolated,"retained command factory requires its original payload"))?;$maximum};
   $crate::retained_command::factory::admit_job(context,operation,payload,input,checkpoint,output,maximum,$resume)
  }
 };
 (maximum_raw_bytes:$maximum:expr,checkpoint_resume:$resume:expr$(,)?)=>{
  $crate::retained_command_factory_admission!(maximum_raw_bytes:|__original_payload|$maximum,checkpoint_resume:$resume);
 };
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
