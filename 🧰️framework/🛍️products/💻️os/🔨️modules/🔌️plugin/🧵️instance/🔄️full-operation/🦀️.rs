use super::*;

impl<A:ArtifactApp,M:SpaceMember+MemberFactory+Send+'static> VcsArtifactApp<A,M>{
 pub(crate) fn next_granted_mounted_retirement(&self)->Option<(usize,u64)>{(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset|{let index=(self.close_typed_operation_cursor+offset)%ARTIFACT_LIVE_OUTPUT_SLOTS;self.tool_operations.entry(index).filter(|(_,owner)|self.close_started||owner.stage==MountedTypedCommandFullOperationStage::Retiring||(owner.granted_retirement_ready()&&owner.granted_retirement_pending())).map(|(id,_)|(index,*id))})}
 pub(crate) fn granted_mounted_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let Some((_,id))=self.next_granted_mounted_retirement()else{return Ok(Default::default());};let owner=self.tool_operations.get(id).unwrap();
  if owner.close_fault_pending(){return owner.close_fault_demands(body);}
  if owner.stage==MountedTypedCommandFullOperationStage::Retiring { if let Some(group)=owner.owned_child_group { return if self.private_child_groups.get(group).is_some(){self.private_child_group_operation_close_demands(group,body)}else{Ok(RetirementDemand{depth:1,..Default::default()})}; } }
  if owner.lease_retirement_ready(){return owner.lease_retirement_demands(&self.tool_cancellations,body);}
  if self.close_started{return owner.close_demands(body);}
  if owner.stage==MountedTypedCommandFullOperationStage::Retiring{return owner.retirement_demands(body);}
  owner.granted_retirement_demands(body)
 }
 pub(crate) fn granted_mounted_retirement_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,crate::PluginLifecycleFailure>{
  if self.next_granted_mounted_retirement().is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let grant=cx.retained_grant();let demand=self.granted_mounted_retirement_demands(grant.maximum_copy_bytes)?;
  let step=crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{
   self.granted_mounted_retirement_original(same,actual).map(|step|(PluginLifecycleStep::Progress(step.progress()),false))
  })?;
  Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()))
 }
 fn granted_mounted_retirement_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::PluginLifecycleFailure>{
  let Some((index,id))=self.next_granted_mounted_retirement()else{return Ok(RetainedCloneStep::Complete(Default::default()))};
  let closing=self.close_started;
  if closing||self.tool_operations.get(id).unwrap().stage==MountedTypedCommandFullOperationStage::Retiring{self.request_gesture_cancellation(id);}
  if self.tool_operations.get(id).unwrap().close_fault_pending(){let step=self.tool_operations.get_mut(id).unwrap().close_fault_step(grant)?;self.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;return Ok(step)}
  if self.tool_operations.get(id).unwrap().lease_retirement_ready(){let step=self.tool_operations.get_mut(id).unwrap().lease_retirement_step(&self.tool_cancellations,grant)?;self.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;return Ok(step)}
  if self.tool_operations.get(id).unwrap().stage==MountedTypedCommandFullOperationStage::Retiring{
   if !closing{
    let step=self.retire_typed_operation_unit(id,cx)?;self.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
    return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()))
   }
   if let Some(group)=self.tool_operations.get(id).unwrap().owned_child_group{
    if self.private_child_groups.get(group).is_some(){
     let step=self.close_private_child_group_operation_step(group,grant)?;
     return Ok(RetainedCloneStep::Progress(step.progress()))
    }
    if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
    self.tool_operations.get_mut(id).unwrap().owned_child_group=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
   }
  }
  let owner=self.tool_operations.get_mut(id).unwrap();
  let step=if closing||owner.stage==MountedTypedCommandFullOperationStage::Retiring{
   match if closing{owner.close_step(cx)?}else{owner.retirement_step(cx)?}{
    PluginLifecycleStep::Progress(progress)=>RetainedCloneStep::Progress(progress),
    PluginLifecycleStep::Complete(progress)=>RetainedCloneStep::Complete(progress),
    PluginLifecycleStep::AwaitingInput{..}|PluginLifecycleStep::Blocked{..}=>RetainedCloneStep::Progress(Default::default()),
   }
  }else{owner.granted_retirement_step(grant)?};
  if matches!(step,RetainedCloneStep::Complete(_)){
   self.tool_operations.get(id).filter(|owner|owner.terminal_is_empty()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted removal requires every original owner terminal"))?;
   if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   let original=self.tool_operations.remove(id).unwrap();self.operation_progress_retired|=original.progress.is_some();drop(original);self.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
   return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:step.progress().copied_items.max(1),..step.progress()}))
  }
  self.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;Ok(step)
 }

}
