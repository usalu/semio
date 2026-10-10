//! 🪟️ Captures the original joint presentation under its concrete strong-only issuer.
use super::{ToolRunProgress,ToolRunIdentity,ToolRunState};
use super::entities::provenance::ToolRunEntityProvenance;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::shared::sealed::{SealedShared,SharedIssuer}};
use std::mem::size_of;
#[path="🧬️update/🦀️.rs"]
pub mod update;
#[path="🌱️seed/🦀️.rs"]
pub mod seed;
#[path="🗃️slot/🦀️.rs"]
pub mod slot;

#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunPresentationBody{pub tool_id:SealedShared<String>,pub progress:ToolRunProgress,pub provenance:ToolRunEntityProvenance,pub payload:Option<SealedShared<Vec<u8>>>}
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunPresentation{body:SealedShared<ToolRunPresentationBody>}
#[derive(Clone,Copy)]
pub struct ToolRunView<'a>{pub tool_id:&'a str,pub identity:ToolRunIdentity,pub state:ToolRunState,pub provisional_entities:&'a super::entities::EntityMap,pub progress:&'a ToolRunProgress,pub payload:Option<&'a [u8]>}
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunCapturedView{presentation:ToolRunPresentation,identity:ToolRunIdentity,state:ToolRunState}
impl ToolRunCapturedView{
 pub fn capture_copy_bytes()->usize{size_of::<Self>()}
 pub fn borrowed_view(&self)->ToolRunView<'_>{self.presentation.borrowed_view(self.identity,self.state)}
 pub fn source_identity(&self)->usize{self.presentation.identity()}
 pub fn body(&self)->&ToolRunPresentationBody{self.presentation.body()}
}
impl ToolRunPresentation{
 pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<ToolRunPresentationBody>(),capacity_bytes:SealedShared::<ToolRunPresentationBody>::birth_bytes(),depth:1,..Default::default()}}
 pub fn capture_copy_bytes()->usize{size_of::<Self>()}
 pub fn admit(body:ToolRunPresentationBody,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,ToolRunPresentationBody)>{if grant.maximum_copy_bytes<Self::birth_demands().copy_bytes{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original presentation fixed-body publication is not funded"),body));}SealedShared::admit(body,SharedIssuer::owned(),grant).map(|(body,mut receipt)|{receipt.copied_bytes=Self::birth_demands().copy_bytes;(Self{body},receipt)})}
 pub fn capture(&self,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{self.body.try_duplicate(grant).map(|(body,mut receipt)|{receipt.copied_bytes=Self::capture_copy_bytes();(Self{body},receipt)})}
 pub fn body(&self)->&ToolRunPresentationBody{self.body.get()}
 pub fn identity(&self)->usize{self.body.identity()}
 pub fn borrowed_view(&self,identity:ToolRunIdentity,state:ToolRunState)->ToolRunView<'_>{let body=self.body();ToolRunView{tool_id:body.tool_id.get(),identity,state,provisional_entities:body.provenance.entities(),progress:&body.progress,payload:body.payload.as_ref().map(|payload|payload.get().as_slice())}}
 pub fn capture_view(&self,identity:ToolRunIdentity,state:ToolRunState,grant:RetainedCloneGrant)->Result<(ToolRunCapturedView,RetainedCloneProgress),ValueError>{if grant.maximum_copy_bytes<ToolRunCapturedView::capture_copy_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"captured presentation and scalar envelope handoff is not funded"));}let(presentation,mut receipt)=self.capture(grant)?;receipt.copied_bytes=ToolRunCapturedView::capture_copy_bytes();Ok((ToolRunCapturedView{presentation,identity,state},receipt))}
}
