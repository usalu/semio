//! ♻️ Registration cancellation closes the same partial producers, issuers, aliases and physical frames.
use super::*;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeRegistryRelease;
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>(),depth:1,..Default::default()}}
fn clear<T>(owner:&mut Option<T>,cx:&mut StepContext<'_>,release:usize)->Result<(),OriginalTurnFailure>{let d=RetirementDemand{release_bytes:release,..fixed::<Option<T>>()};close_original_retained_step(cx,d,|_|{*owner=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(())}
fn close_value<T:RetireOwned>(source:&mut Option<T>,close:&mut Option<ControlledRetirement<T>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if let Some(owner)=close.as_mut(){if owner.terminal_is_empty(){clear(close,cx,0)?;}else{let d=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|owner.step(g))?;}return Ok(false)}
 if source.is_some(){let d=fixed::<(T,ControlledRetirement<T>)>();close_original_retained_step(cx,d,|_|{match ControlledRetirement::new(source.take().unwrap()){Ok(owner)=>*close=Some(owner),Err((error,original))=>{*source=Some(original);return Err(error)}}Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}Ok(true)
}
fn close_issuer<R:ArtifactEnvelopeRegistryRelease>(source:&mut Option<Arc<R>>,close:&mut Option<ArtifactEnvelopeRegistryOwner<R>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if let Some(owner)=close.as_mut(){if owner.custody_is_closed(){clear(close,cx,0)?;}else{let d=owner.close_demands()?;close_original_retained_step(cx,d,|g|owner.close_step(g))?;}return Ok(false)}
 if source.is_some(){let d=fixed::<(Option<Arc<R>>,Option<ArtifactEnvelopeRegistryOwner<R>>)>();close_original_retained_step(cx,d,|_|{*close=Some(ArtifactEnvelopeRegistryOwner::from_original(source.take().unwrap()));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}Ok(true)
}
impl<A,M,O> OriginalSingleWindowRegistration<A,M,O>
where A:OriginalArtifactAppBirth<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>,M:ReferenceSpaceMember+MemberFactory+'static,O:WindowConfigOwner{
 pub(super) fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;cx.with_original_retirement(|cx|{
  if !self.closing{let d=fixed::<bool>();close_original_retained_step(cx,d,|_|{self.closing=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}
  if let Some(source)=self.tools_source.as_mut(){if source.terminal_is_empty(){clear(&mut self.tools_source,cx,OriginalDeclaredEmptyOperationSource::boxed_release_demands().release_bytes)?;}else{source.close_step(cx)?;}return Ok(false)}
  if let Some(source)=self.ephemeral_source.as_mut(){if source.terminal_is_empty(){clear(&mut self.ephemeral_source,cx,OriginalEmptyEphemeralSource::boxed_release_demands().release_bytes)?;}else{source.close_step(cx)?;}return Ok(false)}
  if let Some(source)=self.window_source.as_mut(){if source.terminal_is_empty(){clear(&mut self.window_source,cx,OriginalWindowOwnerRegistration::<O>::boxed_release_demands().release_bytes)?;}else{source.close_step(self.window.as_mut().ok_or_else(||invalid("original Window insertion lost its retained registry"))?,cx)?;}return Ok(false)}
  if let Some(tools)=self.tools.as_mut(){if tools.terminal_is_empty(){clear(&mut self.tools,cx,0)?;}else{tools.close_step(cx)?;}return Ok(false)}
  if let Some(ephemeral)=self.ephemeral.as_mut(){if ephemeral.terminal_is_empty(){clear(&mut self.ephemeral,cx,0)?;}else{ephemeral.close_step(cx)?;}return Ok(false)}
  if let Some(window)=self.window.as_mut(){if window.terminal_is_empty(){clear(&mut self.window,cx,0)?;}else{let d=window.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();if semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){let before=cx.retained_progress();let step=window.close_step_context(grant,cx).map_err(OriginalTurnFailure::from)?;let p=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_unposted(step.progress().unwrap_or_default(),before,cx.retained_progress())?;cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);}}return Ok(false)}
  if let Some(owner)=self.peer_close.as_mut(){if owner.terminal_is_empty(){clear(&mut self.peer_close,cx,0)?;}else{let d=owner.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|owner.close_step(g).map(|step|RetainedCloneStep::Progress(step.progress().unwrap_or_default())))?;}return Ok(false)}
  if self.peer.is_some(){let d=fixed::<(Arc<PeerPresenceRoot>,PeerPresenceRootRetirement)>();close_original_retained_step(cx,d,|_|{self.peer_close=Some(PeerPresenceRootRetirement::new(self.peer.take().unwrap()));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;return Ok(false)}
  if !close_value(&mut self.scope,&mut self.scope_close,cx)?||!close_issuer(&mut self.fields,&mut self.fields_close,cx)?||!close_issuer(&mut self.records,&mut self.records_close,cx)?{return Ok(false)}
  if let Some(catalog)=self.catalog.as_mut(){if catalog.terminal_is_empty(){clear(&mut self.catalog,cx,0)?;}else{let d=catalog.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|catalog.close_step(g))?;}return Ok(false)}
  Ok(self.sources_empty())
 })}
}
