//! ♻️ Registration cancellation closes the same partial producers, issuers, aliases and physical frames.
use super::*;
use semio_framework_job::{close_original_effect_step,close_original_retained_step,OriginalTurnPrimary};
use crate::services::synchronous;
use semio_framework_value::retirement::RetireOwned;
use semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeRegistryRelease;
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>(),depth:1,..Default::default()}}
fn effected<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,E>)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{close_original_effect_step(cx,d,|effect|{let result=run(effect);semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result,work:effect.performed_work().max(1)})})}
fn clear<T>(owner:&mut Option<T>,cx:&mut StepContext<'_>,release:usize)->Result<(),OriginalTurnFailure>{let d=RetirementDemand{release_bytes:release,..fixed::<Option<T>>()};synchronous(cx,d,||{*owner=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(())}
fn close_value<T:RetireOwned>(source:&mut Option<T>,close:&mut Option<ControlledRetirement<T>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if let Some(owner)=close.as_mut(){if owner.terminal_is_empty(){clear(close,cx,0)?;}else{let body=cx.retained_grant().maximum_copy_bytes;let d=owner.selected_demand(body)?;close_original_effect_step(cx,d,|effect|owner.step(body,effect))?;}return Ok(false)}
 if source.is_some(){let d=fixed::<(T,ControlledRetirement<T>)>();synchronous(cx,d,||{match ControlledRetirement::new(source.take().unwrap()){Ok(owner)=>*close=Some(owner),Err((error,original))=>{*source=Some(original);return Err(error)}}Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}Ok(true)
}
fn close_issuer<R:ArtifactEnvelopeRegistryRelease>(source:&mut Option<Arc<R>>,close:&mut Option<ArtifactEnvelopeRegistryOwner<R>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if let Some(owner)=close.as_mut(){if owner.custody_is_closed(){clear(close,cx,0)?;}else{let d=owner.close_demands()?;close_original_retained_step(cx,d,|g|{let result=(||owner.close_step(g))();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;}return Ok(false)}
 if source.is_some(){let d=fixed::<(Option<Arc<R>>,Option<ArtifactEnvelopeRegistryOwner<R>>)>();synchronous(cx,d,||{*close=Some(ArtifactEnvelopeRegistryOwner::from_original(source.take().unwrap()));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}Ok(true)
}
fn close_set<S:OriginalWindowSet>(source:&mut Option<Box<S>>,registry:Option<&mut S::Registry>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let Some(owner)=source.as_mut()else{return Ok(true)};if owner.terminal_is_empty(){clear(source,cx,S::boxed_release_demands().release_bytes)?;}else{owner.close_step(registry,cx)?;}Ok(false)}
impl<A,M,D> OriginalNativeRegistration<A,M,D>
where A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+'static,D:OriginalInstanceDeclarations<A>{
 pub(super) fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;cx.with_original_retirement(|cx|{
  if !self.closing{let d=fixed::<bool>();synchronous(cx,d,||{self.closing=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}
  if let Some(source)=self.tools_source.as_mut(){if source.terminal_is_empty(){clear(&mut self.tools_source,cx,OriginalDeclaredEmptyOperationSource::boxed_release_demands().release_bytes)?;}else{source.close_step(cx)?;}return Ok(false)}
  if let Some(source)=self.ephemeral_source.as_mut(){if source.terminal_is_empty(){clear(&mut self.ephemeral_source,cx,OriginalEphemeralSource::<A,D::Ephemeral>::boxed_release_demands().release_bytes)?;}else{source.close_step(cx)?;}return Ok(false)}
  if !close_set(&mut self.config_windows,self.window.as_mut(),cx)?||!close_set(&mut self.transient_windows,self.window_transient.as_mut(),cx)?{return Ok(false)}
  if let Some(window)=self.window_transient.as_mut(){if window.terminal_is_empty(){clear(&mut self.window_transient,cx,0)?;}else{let d=window.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;effected(cx,d,|effect|window.close_step(effect).map(|step|RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(OriginalTurnPrimary::Fault))?;}return Ok(false)}
  if let Some(tools)=self.tools.as_mut(){if tools.terminal_is_empty(){clear(&mut self.tools,cx,0)?;}else{tools.close_step(cx)?;}return Ok(false)}
  if let Some(ephemeral)=self.ephemeral.as_mut(){if ephemeral.terminal_is_empty(){clear(&mut self.ephemeral,cx,0)?;}else{ephemeral.close_step(cx)?;}return Ok(false)}
  if let Some(window)=self.window.as_mut(){if window.terminal_is_empty(){clear(&mut self.window,cx,0)?;}else{let d=window.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();if semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){let before=cx.retained_progress();let step=window.close_step_context(grant,cx).map_err(OriginalTurnFailure::from)?;let p=semio_plugin_ownership::bounded_ownership::original_lifecycle_unposted(step.progress().unwrap_or_default(),before,cx.retained_progress())?;cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);}}return Ok(false)}
  if let Some(owner)=self.peer_close.as_mut(){if owner.terminal_is_empty(){clear(&mut self.peer_close,cx,0)?;}else{let d=owner.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;effected(cx,d,|effect|owner.close_step(effect).map(|step|RetainedCloneStep::Progress(step.progress().unwrap_or_default())))?;}return Ok(false)}
  if self.peer.is_some(){let d=fixed::<(Arc<PeerPresenceRoot>,PeerPresenceRootRetirement)>();synchronous(cx,d,||{self.peer_close=Some(PeerPresenceRootRetirement::new(self.peer.take().unwrap()));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}
  if !close_value(&mut self.scope,&mut self.scope_close,cx)?||!close_issuer(&mut self.fields,&mut self.fields_close,cx)?||!close_issuer(&mut self.records,&mut self.records_close,cx)?{return Ok(false)}
  if let Some(catalog)=self.catalog.as_mut(){if catalog.terminal_is_empty(){clear(&mut self.catalog,cx,0)?;}else{let d=catalog.close_demands(cx.retained_grant().maximum_copy_bytes)?;effected(cx,d,|effect|catalog.close_step(effect))?;}return Ok(false)}
  if !self.birth.terminal_is_empty(){self.birth.close_step(cx)?;return Ok(false)}
  Ok(self.sources_empty())
 })}
}
