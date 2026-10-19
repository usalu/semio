/// 🏭️ Mounted tool production retains one original request or its actual preborn interactive job.
impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{
 fn validate_original_producer_source(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
  if self.producer_source.is_some_and(|binding|!binding.matches(cx)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original tool producer requires the same source roots, clock and deadline").into())}Ok(())
 }
 fn original_reserved_producer_pending(&self)->bool{self.reserved_producer.is_some()&&!self.terminal_seen&&self.session.is_none()&&self.session_rejected.is_none()}
 fn original_producer_pending(&self)->bool{self.producer_request.is_some()||self.producer_request_retirement.is_some()||self.original_reserved_producer_pending()}
 fn original_producer_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.original_reserved_producer_pending(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.producer_request,&self.producer_request_retirement,body)
 }
 fn retire_original_producer(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
  let reserved=self.original_reserved_producer_pending();self.user_cancel_requested=true;self.terminal_seen=true;
  if reserved{self.stage=MountedTypedCommandFullOperationStage::Publishing;if let Some(lease)=self.cancellation_lease.as_ref(){lease.cancel();}return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  let step=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.producer_request,&mut self.producer_request_retirement,grant)?;
  if !self.original_producer_pending(){self.stage=MountedTypedCommandFullOperationStage::Publishing;if let Some(lease)=self.cancellation_lease.as_ref(){lease.cancel();}}
  Ok(PluginLifecycleStep::Progress(step.progress()))
 }
 fn drive_original_producer(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
  let before=cx.retained_progress();let mut candidate=None;
  let result=if self.producer_request.is_some(){A::build_tool_job(&mut self.producer_request,&mut candidate,cx)}else{semio_framework_plugin_job::reserved_jobs::admit_original_reserved_producer(&mut self.reserved_producer,&mut candidate,&self.operation,cx)};
  if let Some(job)=candidate{self.session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(job,(self.source_identity.operation(),self.source_identity.generation())));}
  if result.is_err(){self.user_cancel_requested=true;self.terminal_seen=true;}
  match result{
   Ok(true)if self.session_rejected.is_none()=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original producer completed without retaining its job").into()),
   Err(failure)=>return Err(failure),
   _=>{}
  }
  Ok(PluginLifecycleStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))
 }
}
