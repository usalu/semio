impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{
 fn granted_retirement_ready(&self)->bool{self.result_page.is_none()&&(self.stage==MountedTypedCommandFullOperationStage::Retiring||self.pending_artifact_publication.as_ref().is_some_and(PendingArtifactStorePublication::is_closing))}
 fn granted_retirement_pending(&self)->bool{self.pending_artifact_publication.as_ref().is_some_and(PendingArtifactStorePublication::is_closing)||(self.stage==MountedTypedCommandFullOperationStage::Retiring&&(self.actor_capture.is_some()||self.reserved_producer.is_some()||self.raw_input.is_some()||self.pending_artifact_publication.is_some()||self.publication.is_some()||self.publication_retirement.is_some()||self.completion.is_some()||self.completion_retirement.is_some()||self.output_chunks.is_some()||self.output_retirement.is_some()))}
 fn granted_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.actor_capture.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
  if let Some(producer)=self.reserved_producer.as_ref(){
   if producer.terminal_is_empty(){return Ok(if std::sync::Arc::strong_count(&producer.state)==1&&std::sync::Arc::weak_count(&producer.state)==0{RetirementDemand{release_bytes:original_reserved_arc_frame_bytes::<std::sync::Mutex<ArtifactReservedToolJobState>>(),depth:1,..Default::default()}}else{Default::default()});}
   return Ok(RetirementDemand{copy_bytes:semio_framework_job::InteractiveJob::next_close_copy_byte_demand(producer)?,capacity_bytes:semio_framework_job::InteractiveJob::next_close_capacity_byte_demand(producer,body)?,release_bytes:semio_framework_job::InteractiveJob::next_close_release_byte_demand(producer)?,depth:semio_framework_job::InteractiveJob::next_close_depth_demand(producer)?});
  }
  if let Some(raw)=self.raw_input.as_ref(){
   if raw.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:original_reserved_arc_frame_bytes::<std::sync::Mutex<Option<semio_framework::action_bus::RetainedToolWireInput>>>(),depth:1,..Default::default()});}
   return raw.retirement_demands();
  }
  if let Some(owner)=self.pending_artifact_publication.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:owner.frame_release_bytes(),depth:1,..Default::default()});}return owner.retirement_demands(body);}
  if let Some(owner)=self.publication_retirement.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:std::mem::size_of_val(&**owner),depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
  if let Some(original)=self.publication.as_ref(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of_val(original),capacity_bytes:A::completion_retirement_birth_bytes(original).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"mounted completion payload has no original app retirement issuer"))?,depth:1,..Default::default()});}
  if let Some(owner)=self.completion_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
  if self.completion.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  if let Some(owner)=self.output_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
  Ok(RetirementDemand{depth:usize::from(self.output_chunks.is_some()),..Default::default()})
 }
 fn granted_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>{
  use semio_framework_value::retained_clone::RetainedCloneStep;
  let demand=self.granted_retirement_demands(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(owner)=self.actor_capture.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.actor_capture.take();}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(producer)=self.reserved_producer.as_mut(){
   if producer.terminal_is_empty(){
    if std::sync::Arc::strong_count(&producer.state)!=1||std::sync::Arc::weak_count(&producer.state)!=0{return Ok(RetainedCloneStep::Progress(Default::default()));}
    let bytes=original_reserved_arc_frame_bytes::<std::sync::Mutex<ArtifactReservedToolJobState>>();drop(self.reserved_producer.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}));
   }
   producer.begin_close();
   let step=semio_framework_job::InteractiveJob::close_step(producer,grant);
   return match step{
    semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>{if !progress.fits(grant){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved producer exceeded its original retirement grant"));}Ok(RetainedCloneStep::Progress(progress))},
    semio_framework_job::InteractiveJobCloseStep::Blocked=>Ok(RetainedCloneStep::Progress(Default::default())),
    semio_framework_job::InteractiveJobCloseStep::Refused{kind,progress}=>Err(ValueError::literal(kind,"reserved producer refused its original retirement grant").with_retained_progress(progress)),
   };
  }
  if let Some(raw)=self.raw_input.as_ref(){
   if raw.terminal_is_empty(){let bytes=original_reserved_arc_frame_bytes::<std::sync::Mutex<Option<semio_framework::action_bus::RetainedToolWireInput>>>();drop(self.raw_input.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}));}
   let step=raw.close_step(grant).map_err(|error|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original raw input close failed").with_retained_progress(error.retained_progress()))?;
   return match step{PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress)=>{if !progress.fits(grant){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original raw input exceeded full retirement grant").with_retained_progress(progress));}Ok(RetainedCloneStep::Progress(progress))},PluginLifecycleStep::AwaitingInput{..}|PluginLifecycleStep::Blocked{..}=>Ok(RetainedCloneStep::Progress(Default::default()))};
  }
  if let Some(owner)=self.pending_artifact_publication.as_mut(){if owner.terminal_is_empty(){let bytes=owner.frame_release_bytes();self.pending_artifact_publication.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}));}owner.begin_close();let step=owner.close_step(grant).map_err(|error|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted publication retirement failed").with_retained_progress(error.retained_progress()))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.publication_retirement.is_some(){return semio_framework_value::close_factory_ticket(&mut self.publication_retirement,grant);}
  if self.publication.is_some(){return match A::admit_completion_retirement(&mut self.publication,grant)?{Some((owner,progress))=>{self.publication_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},None=>Ok(RetainedCloneStep::Progress(Default::default()))};}
  if let Some(owner)=self.completion_retirement.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.completion_retirement.take();}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(original)=self.completion.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.completion_retirement=Some(owner),Err((error,original))=>{self.completion=Some(original);return Err(error);}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
  if let Some(owner)=self.output_retirement.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.output_retirement.take();}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(original)=self.output_chunks.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.output_retirement=Some(owner),Err((error,original))=>{self.output_chunks=Some(original);return Err(error);}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
}
impl<A:ArtifactApp,M:SpaceMember+MemberFactory+Send+'static> VcsArtifactApp<A,M>{
 fn next_granted_mounted_retirement(&self)->Option<(usize,u64)>{(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset|{let index=(self.close_typed_operation_cursor+offset)%ARTIFACT_LIVE_OUTPUT_SLOTS;self.tool_operations.entry(index).filter(|(_,owner)|owner.granted_retirement_ready()&&owner.granted_retirement_pending()).map(|(id,_)|(index,*id))})}
 fn granted_mounted_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.next_granted_mounted_retirement().map_or(Ok(Default::default()),|(_,id)|self.tool_operations.get(id).unwrap().granted_retirement_demands(body))}
 fn granted_mounted_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>{let Some((index,id))=self.next_granted_mounted_retirement()else{return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()));};let step=self.tool_operations.get_mut(id).unwrap().granted_retirement_step(grant)?;self.close_typed_operation_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;Ok(step)}
}
