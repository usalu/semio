//! 📣️ Field-borrowed original host-event routing and retirement.
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_emission::{ActionMeta, HostEvent};
use semio_plugin_instance_history::history_components::HostEvents;
use semio_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_plugin_instance_stores::stores_components::InstanceStores;
/// 🎟️ The original source binding and its decoder and retirement recipient identities.
pub type SourceScope = Option<(semio_framework_job::OriginalSourceBinding, (semio_framework_value::retained_clone::NativeAllocationIdentity, u64))>;
fn claim_scope(scope:&mut SourceScope,pending:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if (*scope).is_some_and(|(scope,decoder)|scope.matches(cx)&&cx.native_receiving_identity()==Some(decoder)){return Ok(true)}if pending{return Ok(false)}let binding=semio_framework_job::OriginalSourceBinding::borrow(cx);let bytes=2*std::mem::size_of::<Option<(semio_framework_job::OriginalSourceBinding,(semio_framework_value::retained_clone::NativeAllocationIdentity,u64))>>();let p=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};if !cx.is_cancelled()&&!cx.should_yield()&&cx.fuel_remaining()>=1&&p.fits(cx.retained_grant()){cx.consume_retained(p)?;cx.consume_fuel(1);let decoder=cx.native_receiving_identity().ok_or_else(||semio_framework_value::ValueError::invariant("original command scope requires its actual decoder recipient"))?;*scope=Some((binding,decoder));}Ok(false)}
/// 📣️ Borrows the original routing owners without transferring their authority.
pub struct HostEventRoutingCtx<'a, A: ArtifactApp> {
 pub host_events: &'a mut HostEvents,
 pub tools: &'a mut ToolOperations<A>,
 pub stores: &'a mut InstanceStores<A>,
 pub original_host_event_command: &'a mut semio_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource<A>,
 pub original_host_event_scope: &'a mut SourceScope,
 pub original_command_scope: &'a mut SourceScope,
 pub shared_pending: bool,
}
impl<A: ArtifactApp> HostEventRoutingCtx<'_, A> {
 pub fn action_host_event_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if !self.host_events.action_delivery.terminal_is_empty(){return self.host_events.action_delivery.close_demands(body)}Ok(semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<HostEvent>>()+4*std::mem::size_of::<bool>(),depth:1,..Default::default()})}
 pub fn close_action_host_events(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{if !self.host_events.action_delivery.terminal_is_empty(){self.host_events.action_delivery.close_step(cx)?;return Ok(())}let d=self.action_host_event_close_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|_|{if self.host_events.queue.is_empty(){self.host_events.delivering=false;self.host_events.reply=false;self.host_events.action_event_complete=false;self.host_events.action_event_removed=false;}else{*self.host_events.action_delivery.event=self.host_events.queue.pop_front();}semio_framework_value::RetainedWorkOutcome{result:Ok::<_,semio_framework_job::OriginalTurnPrimary>(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})?;Ok(())})}
 pub fn claim_original_host_event_scope(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let pending=self.original_host_event_command.pending()||self.host_events.routes.delivering||!self.host_events.routes.delivery_is_empty();claim_scope(&mut self.original_host_event_scope,pending,cx)}
 pub fn close_original_host_event_source(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_value::RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{if self.original_host_event_command.pending(){return crate::dispatch_runtime_command::close_original_typed_source(&mut self.original_host_event_command,&mut self.tools.typed_operation_reservations,&mut self.host_events.host_event_operations,cx)}let before=cx.retained_progress();if !self.host_events.routes.delivery_is_empty(){self.host_events.routes.close_delivery(cx)?;}else{self.host_events.routes.finish_cancelled(cx)?;}Ok(semio_framework_value::RetainedCloneStep::Progress(semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?))}
 pub fn claim_original_command_scope(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let pending=self.shared_pending;claim_scope(&mut self.original_command_scope,pending,cx)}
 pub fn capture_original_host_event_route(&mut self,meta:Option<&ActionMeta>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if !self.claim_original_command_scope(cx)?{return Ok(false)}
  if self.host_events.routes.delivering{return Ok(true)}let Some(meta)=meta.filter(|meta|meta.view_state.is_some())else{return Ok(true)};if self.tools.live_runtime_instance_id!=Some(meta.instance_id)||meta.actor.as_str()!=self.stores.store.local_actor_id().0.as_str(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"host event routing requires its actual mounted actor and instance").into())}self.host_events.routes.capture(meta,cx)
 }
}

/// 🔎 Borrows original host-event state for retirement quotation.
pub struct HostEventRoutingRef<'a, A: ArtifactApp> { pub host_events: &'a HostEvents, pub original_host_event_command: &'a semio_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource<A>, pub original_host_event_scope: &'a SourceScope, pub original_command_scope: &'a SourceScope }
impl<A: ArtifactApp> HostEventRoutingRef<'_, A> {
 pub fn action_host_event_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if !self.host_events.action_delivery.terminal_is_empty(){return self.host_events.action_delivery.close_demands(body)}Ok(semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<HostEvent>>()+4*std::mem::size_of::<bool>(),depth:1,..Default::default()})}
 pub fn original_host_event_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if self.original_host_event_command.pending(){return crate::dispatch_runtime_command::original_typed_source_close_demands(&self.original_host_event_command,body)}if !self.host_events.routes.delivery_is_empty(){return self.host_events.routes.delivery_close_demands(body)}Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<(bool,bool,u64)>(),depth:1,..Default::default()})}
 pub fn owns_original_command_scope(&self,cx:&semio_framework_job::StepContext<'_>)->bool{self.original_command_scope.is_some_and(|(scope,decoder)|scope.matches(cx)&&cx.native_receiving_identity()==Some(decoder))}
 pub fn owns_original_host_event_source(&self,cx:&semio_framework_job::StepContext<'_>)->bool{self.original_host_event_scope.is_some_and(|(scope,decoder)|scope.matches(cx)&&cx.native_receiving_identity()==Some(decoder))&&(self.original_host_event_command.pending()||self.host_events.routes.delivering||!self.host_events.routes.delivery_is_empty())}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
