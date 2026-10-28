//! 🌱️ Native surface declarations retain actual authored genesis sources and the paid controller identity.
use super::*;
use semio_framework_plugin_ownership::original_store_genesis::OriginalStoreGenesisOwner;
use semio_framework_plugin_decl_app::OriginalArtifactAppBirth;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use std::mem::size_of;
/// 🌱️ The authored editor explicitly names its native document, config and draft source vocabularies.
pub trait OriginalArtifactEditorBirth:ArtifactEditor{
 type DocumentGenesis:OriginalStoreGenesisOwner<State=Self::Snapshot,Mutation=Self::Mutation>;
 type ConfigGenesis:OriginalStoreGenesisOwner<State=Self::Config,Mutation=Self::ConfigMutation>;
 type DraftGenesis:OriginalStoreGenesisOwner<State=Self::Draft,Mutation=Self::DraftMutation>;
}
impl<E:ArtifactEditor> RetireOwned for EditorApp<E>{fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{self.id.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.id.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl<E:OriginalArtifactEditorBirth> OriginalArtifactAppBirth for EditorApp<E>{
 type DocumentGenesis=E::DocumentGenesis;type ConfigGenesis=E::ConfigGenesis;type DraftGenesis=E::DraftGenesis;type InteractionGenesis=semio_framework_plugin_interaction::original_genesis::InteractionGenesis;
 fn controller_birth_demands(app_id:&str)->Result<RetirementDemand,ValueError>{if app_id.is_empty()||app_id.len()>4096{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native surface identity is empty or exceeds its declared bound"))}Ok(RetirementDemand{copy_bytes:2*size_of::<Self>()+app_id.len(),capacity_bytes:app_id.len(),depth:1,..Default::default()})}
 fn admit_controller(app_id:&str,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::controller_birth_demands(app_id)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native surface identity birth is not admitted"))}let mut id=String::with_capacity(app_id.len());id.push_str(app_id);Ok((Self::from_admitted_identity(id),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))}
 fn controller_retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(semio_framework_plugin_ownership::bounded_ownership::plugin_text_retirement_demand(&self.id))}
 fn controller_close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=self.controller_retirement_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,d,|grant|{let p=semio_framework_plugin_ownership::bounded_ownership::plugin_text_close(&mut self.id,grant).unwrap_or_default();Ok(if self.id.capacity()==0{semio_framework_value::RetainedCloneStep::Complete(p)}else{semio_framework_value::RetainedCloneStep::Progress(p)})}))?;Ok(self.controller_terminal_is_empty())}
 fn controller_terminal_is_empty(&self)->bool{self.id.capacity()==0}
}
/// 🌱️ The authored viewer explicitly names its native document, config and draft source vocabularies.
pub trait OriginalArtifactViewerBirth:ArtifactViewer{
 type DocumentGenesis:OriginalStoreGenesisOwner<State=Self::Snapshot,Mutation=Self::Mutation>;
 type ConfigGenesis:OriginalStoreGenesisOwner<State=Self::Config,Mutation=Self::ConfigMutation>;
 type DraftGenesis:OriginalStoreGenesisOwner<State=NoDraft,Mutation=NoDraftMutation>;
}
impl<V:ArtifactViewer> RetireOwned for ViewerApp<V>{fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{self.id.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.id.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl<V:OriginalArtifactViewerBirth> OriginalArtifactAppBirth for ViewerApp<V>{
 type DocumentGenesis=V::DocumentGenesis;type ConfigGenesis=V::ConfigGenesis;type DraftGenesis=V::DraftGenesis;type InteractionGenesis=semio_framework_plugin_interaction::original_genesis::InteractionGenesis;
 fn controller_birth_demands(app_id:&str)->Result<RetirementDemand,ValueError>{if app_id.is_empty()||app_id.len()>4096{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native surface identity is empty or exceeds its declared bound"))}Ok(RetirementDemand{copy_bytes:2*size_of::<Self>()+app_id.len(),capacity_bytes:app_id.len(),depth:1,..Default::default()})}
 fn admit_controller(app_id:&str,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::controller_birth_demands(app_id)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native surface identity birth is not admitted"))}let mut id=String::with_capacity(app_id.len());id.push_str(app_id);Ok((Self::from_admitted_identity(id),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))}
 fn controller_retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(semio_framework_plugin_ownership::bounded_ownership::plugin_text_retirement_demand(&self.id))}
 fn controller_close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=self.controller_retirement_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,d,|grant|{let p=semio_framework_plugin_ownership::bounded_ownership::plugin_text_close(&mut self.id,grant).unwrap_or_default();Ok(if self.id.capacity()==0{semio_framework_value::RetainedCloneStep::Complete(p)}else{semio_framework_value::RetainedCloneStep::Progress(p)})}))?;Ok(self.controller_terminal_is_empty())}
 fn controller_terminal_is_empty(&self)->bool{self.id.capacity()==0}
}
