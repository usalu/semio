//! 🏺️ Staged slice of the `full-operation` band of the mounted artifact instance: its `instance-ladders` members (WP6 step S0 of the instance family).
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use crate::ladder_bundle::{LadderCtx, LadderRef};
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_ui_contract::*;

impl<'ctx, A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+Send+'static, X: InstanceExtension> LadderCtx<'ctx, A, M, X> {
 pub fn granted_mounted_retirement_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
  if self.shared().next_granted_mounted_retirement().is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let grant=cx.retained_grant();let demand=self.shared().granted_mounted_retirement_demands(grant.maximum_copy_bytes)?;
  let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{
   self.granted_mounted_retirement_original(same,actual).map(|step|(PluginLifecycleStep::Progress(step.progress()),false))
  })?;
  Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()))
 }

 fn granted_mounted_retirement_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
  let Some((index,id))=self.shared().next_granted_mounted_retirement()else{return Ok(RetainedCloneStep::Complete(Default::default()))};
  let closing=self.close.close_started;
  if self.tools.tool_operations.get(id).unwrap().terminal_is_empty(){let d=self.tools.tool_operations.get(id).unwrap().original_frame_release_demands();if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}let original=self.tools.tool_operations.remove(id).unwrap();self.tools.operation_progress_retired|=original.progress.is_some();drop(original);self.close.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}

  if closing||self.tools.tool_operations.get(id).unwrap().stage==MountedTypedCommandFullOperationStage::Retiring{self.as_render().request_gesture_cancellation(id);}
  if self.tools.tool_operations.get(id).unwrap().close_fault_pending(){let step=self.tools.tool_operations.get_mut(id).unwrap().close_fault_step(grant)?;self.close.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;return Ok(step)}
  if self.tools.tool_operations.get(id).unwrap().lease_retirement_ready(){let step=self.tools.tool_operations.get_mut(id).unwrap().lease_retirement_step(&self.tools.tool_cancellations,grant)?;self.close.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;return Ok(step)}
  if self.tools.tool_operations.get(id).unwrap().stage==MountedTypedCommandFullOperationStage::Retiring{
   if !closing{
    let step=self.as_typed().retire_typed_operation_unit(id,cx)?;self.close.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
    return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()))
   }
   if let Some(group)=self.tools.tool_operations.get(id).unwrap().owned_child_group{
    if self.child_composition.private_child_groups.get(group).is_some(){
     let step=self.child_composition.close_private_child_group_operation_step(&mut *self.stores, group, cx, grant)?;
     return Ok(RetainedCloneStep::Progress(step.progress()))
    }
    if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
    self.tools.tool_operations.get_mut(id).unwrap().owned_child_group=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
   }
  }
  let owner=self.tools.tool_operations.get_mut(id).unwrap();
  let step=if closing||owner.stage==MountedTypedCommandFullOperationStage::Retiring{
   match if closing{owner.close_step(cx)?}else{owner.retirement_step(cx)?}{
    PluginLifecycleStep::Progress(progress)=>RetainedCloneStep::Progress(progress),
    PluginLifecycleStep::Complete(progress)=>RetainedCloneStep::Complete(progress),
    PluginLifecycleStep::AwaitingInput{..}|PluginLifecycleStep::Blocked{..}=>RetainedCloneStep::Progress(Default::default()),
   }
  }else{owner.granted_retirement_step(cx,grant)?};
  if matches!(step,RetainedCloneStep::Complete(_)){
   self.tools.tool_operations.get(id).filter(|owner|owner.terminal_is_empty()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted removal requires every original owner terminal"))?;
   if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   return Ok(RetainedCloneStep::Progress(step.progress()))
  }
  self.close.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;Ok(step)
 }
}

impl<'v, A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+Send+'static, X: InstanceExtension> LadderRef<'v, A, M, X> {
 pub(crate) fn next_granted_mounted_retirement(&self)->Option<(usize,u64)>{(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset|{let index=(self.close.close_typed_operation_cursor+offset)%ARTIFACT_LIVE_OUTPUT_SLOTS;self.tools.tool_operations.entry(index).filter(|(_,owner)|self.close.close_started||owner.stage==MountedTypedCommandFullOperationStage::Retiring||(owner.granted_retirement_ready()&&owner.granted_retirement_pending())).map(|(id,_)|(index,*id))})}

 pub fn granted_mounted_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let Some((_,id))=self.next_granted_mounted_retirement()else{return Ok(Default::default());};let owner=self.tools.tool_operations.get(id).unwrap();
  if owner.terminal_is_empty(){return Ok(owner.original_frame_release_demands())}
  if owner.close_fault_pending(){return owner.close_fault_demands(body);}
  if owner.stage==MountedTypedCommandFullOperationStage::Retiring { if let Some(group)=owner.owned_child_group { return if self.child_composition.private_child_groups.get(group).is_some(){self.child_composition.private_child_group_operation_close_demands(group, body)}else{Ok(RetirementDemand{depth:1,..Default::default()})}; } }
  if owner.lease_retirement_ready(){return owner.lease_retirement_demands(&self.tools.tool_cancellations,body);}
  if self.close.close_started{return owner.close_demands(body);}
  if owner.stage==MountedTypedCommandFullOperationStage::Retiring{return owner.retirement_demands(body);}
  owner.granted_retirement_demands(body)
 }
}
