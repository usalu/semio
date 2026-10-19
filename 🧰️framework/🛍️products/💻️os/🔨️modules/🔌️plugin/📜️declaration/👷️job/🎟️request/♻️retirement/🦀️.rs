//! 🧳️ Pending original requests retain all nineteen owned sources through paid cancellation and terminal closure.
use super::ArtifactOwnedToolJobRequest;
use crate::ArtifactApp;
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,RetirementCursor,RetirementStep,sequence,sequence_birth_bytes,deferred,deferred_birth_bytes,deferred_birth_bytes_for,shared::SharedControlledRetirement}};
use std::{mem::size_of,sync::Arc};

struct OriginalRequestLease<T:RetireOwned+Sync>(Arc<T>);
impl<T:RetireOwned+Sync> RetireOwned for OriginalRequestLease<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SharedControlledRetirement::lease(self.0))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedControlledRetirement<T>>())}
 fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
fn lease_field_birth<T:RetireOwned+Sync>(_:&Arc<T>)->usize{deferred_birth_bytes::<OriginalRequestLease<T>>()}

struct OriginalRequestWire(RetainedToolWireInput);
struct OriginalRequestWireClose{original:Option<RetainedToolWireInput>}
impl RetireOwned for OriginalRequestWire{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(OriginalRequestWireClose{original:Some(self.0)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalRequestWireClose>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for OriginalRequestWireClose{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let Some(original)=self.original.as_ref()else{return Ok(Default::default())};
  if original.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  Ok(RetirementDemand{copy_bytes:original.next_close_copy_byte_demand()?,capacity_bytes:original.next_close_capacity_byte_demand(body)?,release_bytes:original.next_close_release_byte_demand()?,depth:original.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original request wire retirement depth overflow"))?})
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error)};
  if self.original.is_none(){return RetirementStep::Complete;}
  if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return RetirementStep::BudgetExhausted;}
  let original=self.original.as_mut().expect("original wire slot remains owned");
  if original.terminal_is_empty(){self.original.take();return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});}
  match original.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}){Ok(InteractiveJobCloseStep::Complete{progress}|InteractiveJobCloseStep::Pending{progress})=>RetirementStep::Progress(progress),Ok(InteractiveJobCloseStep::Blocked)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error)}
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

impl<A:ArtifactApp> RetireOwned for ArtifactOwnedToolJobRequest<A>{
 fn retirement(self)->Box<dyn RetirementCursor>{
  let ArtifactOwnedToolJobRequest{command,input,checkpoint,controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,snapshot,snapshot_read,config,window_config,history,interaction_state,interaction_hover,context,instance_operation_owner,output_chunks,completion,..}=self;
  let input=input.map(OriginalRequestWire);let checkpoint=checkpoint.map(OriginalRequestWire);
  let snapshot=OriginalRequestLease(snapshot);let config=OriginalRequestLease(config);let history=OriginalRequestLease(history);let interaction_state=OriginalRequestLease(interaction_state);let interaction_hover=OriginalRequestLease(interaction_hover);
  sequence(vec![deferred(command),deferred(input),deferred(checkpoint),deferred(controller_id),deferred(tool_id),deferred(payload_schema_id),deferred(parent_document_id),deferred(authoring_seed),deferred(snapshot),deferred(snapshot_read),deferred(config),deferred(window_config),deferred(history),deferred(interaction_state),deferred(interaction_hover),deferred(context),deferred(instance_operation_owner),deferred(output_chunks),deferred(completion)])
 }
 fn retirement_birth_bytes(&self)->Option<usize>{
  sequence_birth_bytes(&[deferred_birth_bytes_for(&self.command),deferred_birth_bytes::<Option<OriginalRequestWire>>(),deferred_birth_bytes::<Option<OriginalRequestWire>>(),deferred_birth_bytes_for(&self.controller_id),deferred_birth_bytes_for(&self.tool_id),deferred_birth_bytes_for(&self.payload_schema_id),deferred_birth_bytes_for(&self.parent_document_id),deferred_birth_bytes_for(&self.authoring_seed),lease_field_birth(&self.snapshot),deferred_birth_bytes_for(&self.snapshot_read),lease_field_birth(&self.config),deferred_birth_bytes_for(&self.window_config),lease_field_birth(&self.history),lease_field_birth(&self.interaction_state),lease_field_birth(&self.interaction_hover),deferred_birth_bytes_for(&self.context),deferred_birth_bytes_for(&self.instance_operation_owner),deferred_birth_bytes_for(&self.output_chunks),deferred_birth_bytes_for(&self.completion)])
 }
 fn controlled_retirement_supported()->bool{<A::Command as RetireOwned>::controlled_retirement_supported()&&<A::Snapshot as RetireOwned>::controlled_retirement_supported()&&<A::Config as RetireOwned>::controlled_retirement_supported()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
