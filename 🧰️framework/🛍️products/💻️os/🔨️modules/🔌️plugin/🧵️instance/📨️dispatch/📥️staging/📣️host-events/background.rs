//! 🌐️ A stationary base change advances one exact window event through the real original typed scheduler.
use super::*;
use store::os_store::ArtifactStoreNativeVcsTurn as _;
/// 📏️ An unpublished exact reservation returns before any source payload is retired.
/// ♻️ Only the source's actual still-unpublished reservation can be returned by its original caller.
impl<'ctx,A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static,X:InstanceExtension> DispatchCtx<'ctx,A,M,X>{
 /// 🪟️ The caller has exclusively selected this source after active actions and task-resume owners yielded it.
 pub async fn background_host_event_step(&mut self,source:&mut OriginalTypedCommandSource<A>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
  if cx.is_cancelled(){if source.pending(){close_original_typed_source(source,&mut self.tools.typed_operation_reservations,&mut self.host_events.host_event_operations,cx)?;return Ok(())}if !self.host_events.routes.close_delivery(cx)?{return Ok(())}self.host_events.routes.finish_cancelled(cx)?;return Ok(())}
  if !self.host_events.routes.settle_capture(cx)?{return Ok(())}if !self.host_events.routes.start(cx)?{return Ok(())}
  if self.host_events.routes.completed||self.host_events.routes.outcome.is_some(){if source.pending(){close_original_typed_source(source,&mut self.tools.typed_operation_reservations,&mut self.host_events.host_event_operations,cx)?;return Ok(())}if !self.host_events.routes.close_delivery(cx)?{return Ok(())}self.host_events.routes.next_window(cx)?;return Ok(())}
  if self.host_events.routes.cursor==self.host_events.routes.extent(){self.host_events.routes.finish(cx)?;return Ok(())}
  if self.host_events.routes.event.is_none(){let scope=self.host_events.routes.route(self.host_events.routes.cursor).is_some_and(|meta|Some(meta.instance_id)==self.tools.live_runtime_instance_id&&meta.actor.as_str()==self.stores.store.local_actor_id().0.as_str());if !scope{self.host_events.routes.next_window(cx)?;return Ok(())}match self.host_events.routes.advance_liveness(cx)?{None=>return Ok(()),Some(false)=>{self.host_events.routes.next_window(cx)?;return Ok(())},Some(true)=>{}}if !self.host_events.routes.prepare_metadata(cx)?{return Ok(())}self.host_events.routes.prepare_event(cx)?;return Ok(())}
  let event=self.host_events.routes.event.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"background host event lost its admitted fact"))?;
  if let Some(operation)=source.operation.filter(|_|source.receipt.is_none()){let revision=self.stores.store.content_revision_now();if operation.generation.0!=self.stores.store.generation_now()||operation.base_revision.0!=u64::from_be_bytes(revision[..8].try_into().expect("fixed revision prefix")){close_original_typed_source(source,&mut self.tools.typed_operation_reservations,&mut self.host_events.host_event_operations,cx)?;return Ok(())}}
  match self.prepare_host_event_typed_command(source,event,cx).await?{None=>return Ok(()),Some(false)=>{self.host_events.routes.complete_current(cx)?;return Ok(())},Some(true)=>{}}
  if source.metadata.is_none()&&source.receipt.is_none(){source.capture_metadata(&mut self.host_events.routes.metadata,cx)?;return Ok(())}
  if let Some(outcome)=self.dispatch_typed_command_inner(source,&mut None,&mut None,identity,cx).await?{*self.host_events.routes.outcome=Some(outcome);self.host_events.routes.complete_current(cx)?;}
  Ok(())
 }
}
