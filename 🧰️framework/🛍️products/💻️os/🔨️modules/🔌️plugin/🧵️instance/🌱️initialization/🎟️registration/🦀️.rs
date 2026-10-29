//! 🎟️ One authored Window and the declared native tools retain their original producers through mounted admission.
use super::*;
use crate::ephemeral::{OriginalEmptyEphemeralSource,OriginalEmptyEphemeralStores};
use semio_framework_plugin_instance_jobs::jobs_components::{OriginalDeclaredEmptyOperationSource,OriginalDeclaredOperationTools,OriginalDeclaredTools,DeclaredToolCatalogAlias,OriginalToolOperationsSources};
use semio_framework_plugin_decl_views::{NoPresence,NoPresenceMutation,NoTransient,NoTransientMutation,PeerPresenceRoot};
use semio_framework_plugin_window_config::{WindowConfigOwner,OriginalWindowOwnerRegistration};
use semio_framework_plugin_reactor_app::OriginalPluginAppRegistration;
use semio_framework_plugin_window_kits::RegisteredDefinitionSource;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_os_kernel::os_store::owned_schema_decode::{ArtifactEnvelopeFieldDecoderRegistry,ArtifactEnvelopeCompletedRecordRegistry,ArtifactEnvelopeRegistryOwner};
use semio_framework_plugin_peer::PeerPresenceRootRetirement;
use std::marker::PhantomData;
type Fields<A>=ArtifactEnvelopeFieldDecoderRegistry<<A as ArtifactApp>::Snapshot,<A as ArtifactApp>::Mutation>;
type Records<A>=ArtifactEnvelopeCompletedRecordRegistry<<A as ArtifactApp>::Snapshot,<A as ArtifactApp>::Mutation>;
type Lifetime=semio_framework_job::CancelToken;
/// 🧺️ This explicit source supplies empty ephemeral lanes and exactly one authored persisted-local Window owner.
pub struct OriginalSingleWindowRegistration<A,M,O>
where A:OriginalArtifactAppBirth<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>,M:ReferenceSpaceMember+MemberFactory+'static,O:WindowConfigOwner{
 binding:OriginalSourceBinding,definition:(usize,usize),policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
 catalog:ManuallyDrop<Option<DeclaredToolCatalogAlias>>,tools_source:Option<Box<OriginalDeclaredEmptyOperationSource>>,tools:ManuallyDrop<Option<OriginalDeclaredOperationTools>>,
 ephemeral_source:Option<Box<OriginalEmptyEphemeralSource>>,ephemeral:ManuallyDrop<Option<OriginalEmptyEphemeralStores>>,window:ManuallyDrop<Option<WindowConfigOwnerRegistry>>,window_source:Option<Box<OriginalWindowOwnerRegistration<O>>>,
 peer:ManuallyDrop<Option<Arc<PeerPresenceRoot>>>,peer_close:Option<PeerPresenceRootRetirement>,scope:ManuallyDrop<Option<Lifetime>>,scope_close:Option<ControlledRetirement<Lifetime>>,
 fields:ManuallyDrop<Option<Arc<Fields<A>>>>,fields_close:Option<ArtifactEnvelopeRegistryOwner<Fields<A>>>,records:ManuallyDrop<Option<Arc<Records<A>>>>,records_close:Option<ArtifactEnvelopeRegistryOwner<Records<A>>>,
 phase:u8,closing:bool,marker:PhantomData<fn()->M>,
}
impl<A,M,O> OriginalSingleWindowRegistration<A,M,O>
where A:OriginalArtifactAppBirth<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>,M:ReferenceSpaceMember+MemberFactory+'static,O:WindowConfigOwner{
 /// 📏️ The producer quotes its actual selected-source frame and genuine declaration and actor aliases.
 pub fn birth_demands()->RetirementDemand{let mut demand=DeclaredToolCatalogAlias::birth_demands();demand.copy_bytes+=2*size_of::<Self>()+size_of::<protocol::ActorId>();demand.capacity_bytes+=size_of::<Self>();demand}
 fn ready(&self)->bool{self.phase==7&&!self.closing}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if !self.binding.matches(cx){return Err(invalid("original Window registration changed its initiating caller"))}Ok(())}
 fn phase(&mut self,next:u8,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let d=RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()};let mut done=None;admit_original_construction(cx,&mut done,construction(d),|_|{self.phase=next;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;Ok(())}
 fn sources_empty(&self)->bool{self.catalog.is_none()&&self.tools_source.is_none()&&self.tools.is_none()&&self.ephemeral_source.is_none()&&self.ephemeral.is_none()&&self.window.is_none()&&self.window_source.is_none()&&self.peer.is_none()&&self.peer_close.is_none()&&self.scope.is_none()&&self.scope_close.is_none()&&self.fields.is_none()&&self.fields_close.is_none()&&self.records.is_none()&&self.records_close.is_none()}
}
impl<A,M,O> OriginalInstanceRegistration<A,M> for OriginalSingleWindowRegistration<A,M,O>
where A:OriginalArtifactAppBirth<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>,M:ReferenceSpaceMember+MemberFactory+'static,O:WindowConfigOwner{
 fn admit(source:RegisteredDefinitionSource<'_>,registration:&dyn OriginalPluginAppRegistration,actor:&protocol::ActorId,policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(invalid("original Window registration recipient is occupied").into())}
  let catalog=registration.as_any().downcast_ref::<OriginalDeclaredTools>().and_then(OriginalDeclaredTools::source).ok_or_else(||invalid("native Window registration requires its actual committed declared tool producer"))?;
  if source.id!=catalog.controller_id{return Err(invalid("original Window registration changed its declared controller").into())}
  let d=Self::birth_demands();let binding=OriginalSourceBinding::borrow(cx);let grant=cx.retained_grant();
  admit_original_construction(cx,output,construction(d),|_|{let(alias,_)=DeclaredToolCatalogAlias::admit(catalog,grant)?.ok_or_else(||invalid("funded declaration alias admission refused"))?;let(actor,_)=actor.0.admit_clone(grant)?;Ok((Box::new(Self{binding,definition:(source.id.as_ptr()as usize,source.id.len()),policy,catalog:ManuallyDrop::new(Some(alias)),tools_source:None,tools:ManuallyDrop::new(None),ephemeral_source:None,ephemeral:ManuallyDrop::new(None),window:ManuallyDrop::new(Some(WindowConfigOwnerRegistry::new(protocol::ActorId(actor)))),window_source:None,peer:ManuallyDrop::new(None),peer_close:None,scope:ManuallyDrop::new(None),scope_close:None,fields:ManuallyDrop::new(None),fields_close:None,records:ManuallyDrop::new(None),records_close:None,phase:0,closing:false,marker:PhantomData}),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))})?;Ok(output.is_some())
 }
 fn advance(&mut self,source:RegisteredDefinitionSource<'_>,_:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing||self.definition!=(source.id.as_ptr()as usize,source.id.len()){return Err(invalid("original Window registration changed its source or lifecycle").into())}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}self.advance_source(source,cx)?;Ok(self.ready())}
 fn take(&mut self,lanes:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>,output:&mut Option<OriginalInstanceRegistrationParts<A,M>>)->Result<bool,OriginalTurnFailure>{self.take_original(lanes,cx,output)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.close_original(cx)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.sources_empty()}
 fn boxed_release_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<A,M,O> Drop for OriginalSingleWindowRegistration<A,M,O>
where A:OriginalArtifactAppBirth<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>,M:ReferenceSpaceMember+MemberFactory+'static,O:WindowConfigOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.closing&&self.sources_empty(),"native Window registration retains all original owners until donation or funded closure")}}
#[path="▶️advance/🦀️.rs"]mod advance;
#[path="📤️take/🦀️.rs"]mod take;
#[path="♻️close/🦀️.rs"]mod close;
