//! 📤️ Native registration donates its actual tools, ephemeral Stores and installed Store factory aliases together.
use super::*;
use semio_framework_plugin_ownership::{bounded_ownership::{ArtifactFixedRegistry,original_registry_backing},store_lifecycle::ArtifactDocumentStoreDisposer};
impl<A,M,O> OriginalSingleWindowRegistration<A,M,O>
where A:OriginalArtifactAppBirth<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>,M:SpaceMember+MemberFactory+'static,O:WindowConfigOwner{
 /// 📏️ The final private donation includes every actual fixed service birth and transferred source frame.
 pub fn take_demands()->Result<RetirementDemand,ValueError>{
  let service=OriginalInstanceServices::<A,M>::birth_demands()?;let tools=ToolOperations::<A>::original_birth_demands()?;
  let backing=original_registry_backing::<OriginalInstanceLiveStores<A>,_>(|owner|&owner.retired_window_transient,64)?;
  let capacity_bytes=service.capacity_bytes.checked_add(tools.capacity_bytes).and_then(|n|n.checked_add(backing)).ok_or_else(||invalid("original Window registration backing extent overflow"))?;
  let copy_bytes=service.copy_bytes.checked_add(tools.copy_bytes).and_then(|n|n.checked_add(2*size_of::<OriginalInstanceRegistrationParts<A,M>>() +2*size_of::<OriginalEmptyEphemeralStores>()+2*size_of::<OriginalDeclaredOperationTools>()+2*size_of::<OriginalInstanceLiveStores<A>>())).ok_or_else(||invalid("original Window registration copy extent overflow"))?;
  Ok(RetirementDemand{copy_bytes,capacity_bytes,depth:service.depth.max(tools.depth).max(2),..Default::default()})
 }
 pub(super) fn take_original(&mut self,lanes:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>,output:&mut Option<OriginalInstanceRegistrationParts<A,M>>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if !self.ready()||output.is_some()||!lanes.matches(cx){return Err(invalid("original Window registration requires its ready original lanes and empty recipient").into())}
  let artifact_factory=lanes.document.parts_ref().owners.one_item_preparation_factory.as_ref();
  let config_factory=lanes.config.parts_ref().owners.one_item_preparation_factory.as_ref();
  let draft_factory=lanes.draft.parts_ref().owners.one_item_preparation_factory.as_ref();
  let d=Self::take_demands()?;let grant=cx.retained_grant();
  admit_original_construction(cx,output,construction(d),|_|{
   let(registry,declared,bus,owner)=self.tools.take().unwrap().into_parts();
   let(tools,_)=ToolOperations::<A>::build_original(OriginalToolOperationsSources{registry,declared,bus,owner},grant).unwrap_or_else(|_|panic!("original ToolOperations donation was fully preflighted"));
   let(presence,transient,presence_disposer,transient_disposer,presence_retirement,transient_retirement)=self.ephemeral.take().unwrap().into_parts();
   let close=CloseCursors::from_original(Some(Box::new(ArtifactDocumentStoreDisposer::<A::Snapshot,A::Mutation>::new())),Some(Box::new(ArtifactDocumentStoreDisposer::<A::Config,A::ConfigMutation>::new())),Some(Box::new(ArtifactDocumentStoreDisposer::<A::Draft,A::DraftMutation>::new())),Some(presence_disposer),Some(transient_disposer),Some(Box::new(ArtifactDocumentStoreDisposer::<protocol::InteractionState,semio_framework_plugin_interaction::mutations::InteractionConfigMutation>::new())));
   let services=OriginalInstanceServices::from_original(OriginalInstanceServiceSources{policy:self.policy,peer_presence:self.peer.take().unwrap(),peer_scope:self.scope.take().unwrap(),field_decoders:self.fields.take().unwrap(),completed_records:self.records.take().unwrap(),tools,close,dispatch_report:store::DispatchReport::default()});
   let live=OriginalInstanceLiveStores{retired_window_transient:ArtifactFixedRegistry::new(),presence,transient,window_config:self.window.take().unwrap(),window_transient:WindowTransientOwnerRegistry::for_document_generation(lanes.document.generation()),artifact_factory:artifact_factory.map(Arc::clone),config_factory:config_factory.map(Arc::clone),draft_factory:draft_factory.map(Arc::clone),presence_factory:None,transient_factory:None,presence_retirement:Some(presence_retirement),transient_retirement:Some(transient_retirement),unsupported:BTreeMap::new()};
   self.closing=true;Ok((OriginalInstanceRegistrationParts{live,services},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))
  })?;Ok(output.is_some())
 }
}
