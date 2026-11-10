//! 📤️ Native registration donates its actual tools, ephemeral Stores and installed Store factory aliases together.
use super::*;
use semio_plugin_ownership::{bounded_ownership::{ArtifactFixedRegistry,original_registry_backing},store_lifecycle::ArtifactDocumentStoreDisposer};
impl<A,M,D> OriginalNativeRegistration<A,M,D>
where A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+'static,D:OriginalInstanceDeclarations<A>{
 /// 📏️ The final private donation includes every actual fixed service birth and transferred source frame.
 pub fn take_demands()->Result<RetirementDemand,ValueError>{
  let backing=original_registry_backing::<OriginalInstanceLiveStores<A>,_>(|owner|&owner.retired_window_transient,64)?;
  let capacity_bytes=backing+size_of::<ArtifactDocumentStoreDisposer<A::Snapshot,A::Mutation>>()+size_of::<ArtifactDocumentStoreDisposer<A::Config,A::ConfigMutation>>()+size_of::<ArtifactDocumentStoreDisposer<A::Draft,A::DraftMutation>>()+size_of::<ArtifactDocumentStoreDisposer<protocol::InteractionState,semio_plugin_interaction::mutations::InteractionConfigMutation>>();
  let copy_bytes=2*(size_of::<OriginalInstanceRegistrationParts<A,M>>()+size_of::<OriginalEphemeralStores<A>>()+size_of::<OriginalDeclaredOperationTools>()+size_of::<OriginalInstanceLiveStores<A>>());
  Ok(RetirementDemand{copy_bytes,capacity_bytes,depth:2,..Default::default()})
 }
 pub(super) fn take_original(&mut self,lanes:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>,output:&mut Option<OriginalInstanceRegistrationParts<A,M>>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if !self.ready()||output.is_some()||!lanes.matches(cx){return Err(invalid("original Window registration requires its ready original lanes and empty recipient").into())}
  let artifact_factory=lanes.document.parts_ref().owners.one_item_preparation_factory.as_ref();
  let config_factory=lanes.config.parts_ref().owners.one_item_preparation_factory.as_ref();
  let draft_factory=lanes.draft.parts_ref().owners.one_item_preparation_factory.as_ref();
  let d=Self::take_demands()?;let grant=cx.retained_grant();
  self.birth.built(cx,output,construction(d),||{
   let(registry,declared,bus,owner)=self.tools.take().unwrap().into_parts();
   let tools=semio_plugin_instance_jobs::jobs_components::OriginalToolOperationsInitialization::from_original(OriginalToolOperationsSources{registry,declared,bus,owner});
   let(presence,transient,presence_disposer,transient_disposer,presence_retirement,transient_retirement,presence_factory,transient_factory)=self.ephemeral.take().unwrap().into_parts();
   let close=CloseCursors::from_original(Some(Box::new(ArtifactDocumentStoreDisposer::<A::Snapshot,A::Mutation>::new())),Some(Box::new(ArtifactDocumentStoreDisposer::<A::Config,A::ConfigMutation>::new())),Some(Box::new(ArtifactDocumentStoreDisposer::<A::Draft,A::DraftMutation>::new())),Some(presence_disposer),Some(transient_disposer),Some(Box::new(ArtifactDocumentStoreDisposer::<protocol::InteractionState,semio_plugin_interaction::mutations::InteractionConfigMutation>::new())));
   let services=OriginalInstanceServiceSources{policy:self.policy,peer_presence:self.peer.take(),peer_scope:self.scope.take(),field_decoders:self.fields.take(),completed_records:self.records.take(),tools:Some(tools),close:Some(close),dispatch_report:Some(store::DispatchReport{policy:store::MergePolicy::default(),worst:None,messages:Vec::new()})};
   let live=OriginalInstanceLiveStores{retired_window_transient:ArtifactFixedRegistry::new(),presence,transient,window_config:self.window.take().unwrap(),window_transient:self.window_transient.take().unwrap(),artifact_factory:artifact_factory.map(Arc::clone),config_factory:config_factory.map(Arc::clone),draft_factory:draft_factory.map(Arc::clone),presence_factory,transient_factory,presence_retirement:Some(presence_retirement),transient_retirement:Some(transient_retirement),unsupported:BTreeMap::new()};
   self.closing=true;Ok((OriginalInstanceRegistrationParts{live,services},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))
  })?;Ok(output.is_some())
 }
}
