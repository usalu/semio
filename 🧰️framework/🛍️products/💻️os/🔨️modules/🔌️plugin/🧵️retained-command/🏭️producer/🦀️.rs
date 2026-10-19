//! 🏭️ Original retained producers transfer every source only inside their actual native construction turn.
use super::{ArtifactCommandWork,ArtifactRetainedCommandInputs,ArtifactRetainedCommandPayload,ArtifactRetainedCommandJob};
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobRequest;
use semio_framework::{ErasedToolJob,action_bus::{OriginalToolJobIdentity,OriginalToolConstructionScope,ToolJobAdmissionDemand,admit_original_tool_construction_scoped}};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::RetainedCloneProgress};
use std::mem::size_of;
/// 🪙️ Heapless typed work and the actual interactive job are boxed under the initiating source turn.
pub fn admit_retained_tool_producer<A:super::ArtifactApp,W:ArtifactCommandWork<A>+'static>(request:&mut Option<ArtifactOwnedToolJobRequest<A>>,output:&mut Option<ErasedToolJob>,cx:&mut StepContext<'_>,command_id:fn(&A::Command)->&'static str,maximum_raw_bytes:usize,maximum_work_items:usize,checkpoint_resume:bool,work:impl FnOnce(&ArtifactOwnedToolJobRequest<A>)->W)->Result<bool,OriginalTurnFailure>{
 admit_retained_tool_producer_measured(request,output,cx,command_id,maximum_raw_bytes,maximum_work_items,checkpoint_resume,Default::default(),|request,_|Ok((work(request),Default::default())))
}
/// 🧮️ Additional work-owned allocation and copying carry an explicit quote and actual physical receipt.
pub fn admit_retained_tool_producer_measured<A:super::ArtifactApp,W:ArtifactCommandWork<A>+'static>(request:&mut Option<ArtifactOwnedToolJobRequest<A>>,output:&mut Option<ErasedToolJob>,cx:&mut StepContext<'_>,command_id:fn(&A::Command)->&'static str,maximum_raw_bytes:usize,maximum_work_items:usize,checkpoint_resume:bool,work_demand:RetirementDemand,work:impl FnOnce(&ArtifactOwnedToolJobRequest<A>,OriginalToolJobIdentity)->Result<(W,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{
 admit_retained_tool_producer_measured_scoped(request,output,cx,command_id,maximum_raw_bytes,maximum_work_items,checkpoint_resume,work_demand,|request,scope|work(request,scope.identity()))
}
/// 🧰️ A single original construction scope reserves nested work owners before their physical birth.
pub fn admit_retained_tool_producer_measured_scoped<A:super::ArtifactApp,W:ArtifactCommandWork<A>+'static>(request:&mut Option<ArtifactOwnedToolJobRequest<A>>,output:&mut Option<ErasedToolJob>,cx:&mut StepContext<'_>,command_id:fn(&A::Command)->&'static str,maximum_raw_bytes:usize,maximum_work_items:usize,checkpoint_resume:bool,work_demand:RetirementDemand,work:impl FnOnce(&ArtifactOwnedToolJobRequest<A>,&mut OriginalToolConstructionScope<'_>)->Result<(W,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{
 let original=request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained producer request is absent"))?;
 if !original.source_binding.matches(cx){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained producer refused different source roots or clock").into())}
 let input=original.input.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original command wire is absent"))?;
 original.command.admitted_encoded_bytes(input,cx)?;
 let layout=super::factory::quote::<A::Command,ArtifactRetainedCommandJob<A>>(original.input.as_ref().map(|input|input.declared_bytes()),original.checkpoint.as_ref().map(|input|input.declared_bytes()),maximum_raw_bytes,checkpoint_resume,maximum_raw_bytes,super::admission::admit_limits(maximum_raw_bytes,maximum_work_items).is_none())?;
 let fixed_capacity=size_of::<W>().checked_add(size_of::<ArtifactRetainedCommandJob<A>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original retained producer layout overflow"))?;
 let demand=ToolJobAdmissionDemand{copy_bytes:work_demand.copy_bytes,capacity_bytes:layout.capacity_bytes.checked_add(size_of::<W>()).and_then(|bytes|bytes.checked_add(work_demand.capacity_bytes)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original retained work layout overflow"))?,release_bytes:layout.release_bytes.checked_add(work_demand.release_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original retained source release overflow"))?,depth:layout.depth.max(work_demand.depth),..layout};
 admit_original_tool_construction_scoped(cx,output,demand,|scope|{
  scope.admit_owner(RetirementDemand{copy_bytes:0,capacity_bytes:layout.capacity_bytes.checked_add(size_of::<W>()).expect("quoted original work layout"),release_bytes:layout.release_bytes,depth:layout.depth})?;
  let (work,extra)=work(request.as_ref().expect("admitted original retained request"),scope)?;
  let original=request.take().expect("admitted original retained request");
  let ArtifactOwnedToolJobRequest{source_binding:_,retained,command,input,checkpoint,operation,controller_id,tool_id,payload_schema_id,contract:_,decoded_items:_,app_instance_id,parent_document_id,canonical_base_revision,authoring_seed,snapshot,snapshot_read,config,window_config,history,interaction_state,interaction_hover,context,instance_operation_owner,output_chunks,completion}=original;
  let operation_context=semio_framework_plugin_decl_views::AppOperationContext{app_instance_id,parent_document_id,operation_id:operation.operation.0,generation:operation.generation.0,canonical_base_revision,retained,authoring_seed};
  let mut payload=ArtifactRetainedCommandPayload::new(ArtifactRetainedCommandInputs{command:*command.into_box(),snapshot,snapshot_read,config,history,interaction_state,interaction_hover,context:Some(context),operation:operation_context,completion},command_id,maximum_raw_bytes,maximum_work_items,Box::new(work));
  payload.producer_window_config=window_config;payload.producer_instance_operation_owner=Some(instance_operation_owner);payload.producer_output_chunks=Some(output_chunks);payload.producer_controller_id=Some(controller_id);payload.producer_tool_id=Some(tool_id);payload.producer_payload_schema_id=Some(payload_schema_id);
  let mut job=ArtifactRetainedCommandJob::from_payload(payload,input,checkpoint);
  job.source_binding=Some(scope.identity().source_binding());
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:extra.copied_bytes,retained_capacity_bytes:fixed_capacity.checked_add(job.raw.capacity()).and_then(|bytes|bytes.checked_add(extra.retained_capacity_bytes)).expect("actual native allocation fits address space"),released_bytes:layout.release_bytes.checked_add(extra.released_bytes).expect("actual native release fits address space")};
  Ok((ErasedToolJob::from_owned(Box::new(job)),progress))
 })
}
