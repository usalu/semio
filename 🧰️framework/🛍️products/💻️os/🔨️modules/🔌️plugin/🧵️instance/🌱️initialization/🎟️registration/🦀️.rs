//! 🎟️ One authored Window and the declared native tools retain their original producers through mounted admission.
use super::*;
pub use crate::services::OriginalInstanceRegistration;
#[path="🪟️windows/🦀️.rs"]
pub mod windows;
pub use windows::{OriginalWindowAdmission,OriginalWindowSet,OriginalWindowSequence,OriginalNoWindows};
use crate::ephemeral::{OriginalEmptyEphemeralDeclaration,OriginalEphemeralDeclaration,OriginalEphemeralSource,OriginalEphemeralStores};
use semio_plugin_instance_jobs::jobs_components::{OriginalDeclaredEmptyOperationSource,OriginalDeclaredOperationTools,OriginalDeclaredTools,DeclaredToolCatalogAlias,OriginalToolOperationsSources};
use semio_plugin_decl_views::{NoPresence,NoPresenceMutation,NoTransient,NoTransientMutation,PeerPresenceRoot};
use semio_plugin_window_config::{WindowConfigOwner,OriginalWindowOwnerRegistration};
use semio_plugin_reactor_app::OriginalPluginAppRegistration;
use semio_plugin_window_kits::RegisteredDefinitionSource;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_os_kernel::os_store::owned_schema_decode::{ArtifactEnvelopeFieldDecoderRegistry,ArtifactEnvelopeCompletedRecordRegistry,ArtifactEnvelopeRegistryOwner};
use semio_plugin_peer::PeerPresenceRootRetirement;
use semio_plugin_ownership::original_store_genesis::OriginalStoreBirth;
use std::marker::PhantomData;
type Fields<A>=ArtifactEnvelopeFieldDecoderRegistry<<A as ArtifactApp>::Snapshot,<A as ArtifactApp>::Mutation>;
type Records<A>=ArtifactEnvelopeCompletedRecordRegistry<<A as ArtifactApp>::Snapshot,<A as ArtifactApp>::Mutation>;
type Lifetime=semio_framework_job::CancelToken;
/// 🧩️ Each app authors its actual ephemeral lanes and complete persisted/transient Window type sequences.
pub trait OriginalInstanceDeclarations<A:OriginalArtifactAppBirth>:Send+'static{
 type Ephemeral:OriginalEphemeralDeclaration<A>;
 type ConfigWindows:OriginalWindowSet<Registry=WindowConfigOwnerRegistry>;
 type TransientWindows:OriginalWindowSet<Registry=WindowTransientOwnerRegistry>;
}
/// 🪟️ The explicit one-window empty-lane declaration uses the same general registration lifecycle.
pub struct OriginalSingleWindowDeclaration<O:WindowConfigOwner>(PhantomData<fn()->O>);
impl<A,O> OriginalInstanceDeclarations<A> for OriginalSingleWindowDeclaration<O>
where A:OriginalArtifactAppBirth<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>,O:WindowConfigOwner{
 type Ephemeral=OriginalEmptyEphemeralDeclaration;
 type ConfigWindows=OriginalWindowSequence<OriginalWindowOwnerRegistration<O>,OriginalNoWindows<WindowConfigOwnerRegistry>>;
 type TransientWindows=OriginalNoWindows<WindowTransientOwnerRegistry>;
}
/// 🎟️ The single-window declaration is a named selection of the common original registrar.
pub type OriginalSingleWindowRegistration<A,M,O>=OriginalNativeRegistration<A,M,OriginalSingleWindowDeclaration<O>>;
/// 🧺️ One original registrar retains every declared native owner through mounted admission.
pub struct OriginalNativeRegistration<A,M,D>
where A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+'static,D:OriginalInstanceDeclarations<A>{
 binding:OriginalSourceBinding,definition:(usize,usize),policy:semio_plugin_mounted_owner::MountedOwnerPolicyV1,
 catalog:ManuallyDrop<Option<DeclaredToolCatalogAlias>>,tools_source:Option<Box<OriginalDeclaredEmptyOperationSource>>,tools:ManuallyDrop<Option<OriginalDeclaredOperationTools>>,
 ephemeral_source:Option<Box<OriginalEphemeralSource<A,D::Ephemeral>>>,ephemeral:ManuallyDrop<Option<OriginalEphemeralStores<A>>>,window:ManuallyDrop<Option<WindowConfigOwnerRegistry>>,config_windows:Option<Box<D::ConfigWindows>>,window_transient:ManuallyDrop<Option<WindowTransientOwnerRegistry>>,transient_windows:Option<Box<D::TransientWindows>>,
 peer:ManuallyDrop<Option<Arc<PeerPresenceRoot>>>,peer_close:Option<PeerPresenceRootRetirement>,scope:ManuallyDrop<Option<Lifetime>>,scope_close:Option<ControlledRetirement<Lifetime>>,
 fields:ManuallyDrop<Option<Arc<Fields<A>>>>,fields_close:Option<ArtifactEnvelopeRegistryOwner<Fields<A>>>,records:ManuallyDrop<Option<Arc<Records<A>>>>,records_close:Option<ArtifactEnvelopeRegistryOwner<Records<A>>>,
 birth:OriginalStoreBirth,unit:Option<()>,phase:u8,closing:bool,marker:PhantomData<fn()->(M,D)>,
}
impl<A,M,D> OriginalNativeRegistration<A,M,D>
where A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+'static,D:OriginalInstanceDeclarations<A>{
 /// 📏️ The producer quotes its actual selected-source frame and genuine declaration and actor aliases.
 pub fn birth_demands()->RetirementDemand{let mut demand=DeclaredToolCatalogAlias::birth_demands();demand.copy_bytes+=2*size_of::<Self>()+size_of::<protocol::ActorId>();demand.capacity_bytes+=size_of::<Self>();demand}
 fn ready(&self)->bool{self.phase==9&&!self.closing}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if !self.binding.matches(cx){return Err(invalid("original Window registration changed its initiating caller"))}Ok(())}
 fn phase(&mut self,next:u8,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let d=RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()};let ready=self.birth.built(cx,&mut self.unit,construction(d),||Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))?;if ready{self.unit=None;self.phase=next;}Ok(())}
 fn sources_empty(&self)->bool{self.birth.terminal_is_empty()&&self.unit.is_none()&&self.catalog.is_none()&&self.tools_source.is_none()&&self.tools.is_none()&&self.ephemeral_source.is_none()&&self.ephemeral.is_none()&&self.window.is_none()&&self.config_windows.is_none()&&self.transient_windows.is_none()&&self.window_transient.is_none()&&self.peer.is_none()&&self.peer_close.is_none()&&self.scope.is_none()&&self.scope_close.is_none()&&self.fields.is_none()&&self.fields_close.is_none()&&self.records.is_none()&&self.records_close.is_none()}
}
impl<A,M,D> OriginalInstanceRegistration<A,M> for OriginalNativeRegistration<A,M,D>
where A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+'static,D:OriginalInstanceDeclarations<A>{
 fn admit(source:RegisteredDefinitionSource<'_>,registration:&dyn OriginalPluginAppRegistration,actor:&protocol::ActorId,policy:semio_plugin_mounted_owner::MountedOwnerPolicyV1,cx:&mut StepContext<'_>,birth:&mut semio_plugin_ownership::original_store_genesis::OriginalStoreBirth,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(invalid("original Window registration recipient is occupied").into())}
  let catalog=registration.as_any().downcast_ref::<OriginalDeclaredTools>().and_then(OriginalDeclaredTools::source).ok_or_else(||invalid("native Window registration requires its actual committed declared tool producer"))?;
  if source.id!=catalog.controller_id{return Err(invalid("original Window registration changed its declared controller").into())}
  let d=Self::birth_demands();let binding=OriginalSourceBinding::borrow(cx);let grant=cx.retained_grant();
  if !birth.stash_actor(actor,cx)?{return Ok(false)}
  let ready=birth.built_actor(cx,output,construction(d),|actor|{let(alias,_)=DeclaredToolCatalogAlias::admit(catalog,grant)?.ok_or_else(||invalid("funded declaration alias admission refused"))?;Ok((Box::new(Self{binding,definition:(source.id.as_ptr()as usize,source.id.len()),policy,catalog:ManuallyDrop::new(Some(alias)),tools_source:None,tools:ManuallyDrop::new(None),ephemeral_source:None,ephemeral:ManuallyDrop::new(None),window:ManuallyDrop::new(Some(WindowConfigOwnerRegistry::new(protocol::ActorId(actor)))),config_windows:None,window_transient:ManuallyDrop::new(None),transient_windows:None,peer:ManuallyDrop::new(None),peer_close:None,scope:ManuallyDrop::new(None),scope_close:None,fields:ManuallyDrop::new(None),fields_close:None,records:ManuallyDrop::new(None),records_close:None,birth:OriginalStoreBirth::default(),unit:None,phase:0,closing:false,marker:PhantomData}),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))})?;Ok(ready&&output.is_some())
 }
 fn advance(&mut self,source:RegisteredDefinitionSource<'_>,lanes:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing||self.definition!=(source.id.as_ptr()as usize,source.id.len())||!lanes.matches(cx){return Err(invalid("original native registration changed its source or lifecycle").into())}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}self.advance_source(source,lanes,cx)?;Ok(self.ready())}
 fn take(&mut self,lanes:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>,output:&mut Option<OriginalInstanceRegistrationParts<A,M>>)->Result<bool,OriginalTurnFailure>{self.take_original(lanes,cx,output)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.close_original(cx)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.sources_empty()}
 fn boxed_release_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<A,M,D> Drop for OriginalNativeRegistration<A,M,D>
where A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+'static,D:OriginalInstanceDeclarations<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.closing&&self.sources_empty(),"native Window registration retains all original owners until donation or funded closure")}}
#[path="▶️advance/🦀️.rs"]mod advance;
#[path="📤️take/🦀️.rs"]mod take;
#[path="♻️close/🦀️.rs"]mod close;
