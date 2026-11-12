//! 🪟️ Captures the original joint presentation under its concrete strong-only issuer.
use semio_framework_tool_run::{ToolRunProgress,ToolRunIdentity,ToolRunState};
use semio_framework_tool_run_entities::{admit_sealed,duplicate_sealed,provenance::ToolRunEntityProvenance};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedAllocationEffect,RetainedCloneProgress,retained_clone_grant_admits}};
use semio_framework_value_shared_issuance::SealedShared;
use std::mem::size_of;
#[path="🧬️update/🦀️.rs"]
pub mod update;
#[path="🌱️seed/🦀️.rs"]
pub mod seed;
#[path="🗃️slot/🦀️.rs"]
pub mod slot;

#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunPresentationBody{pub tool_id:SealedShared<String>,pub progress:ToolRunProgress,pub provenance:ToolRunEntityProvenance,pub payload:Option<SealedShared<semio_framework_tool_run::payload_stream::ToolRunPayloadStream>>}
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunPresentation{body:SealedShared<ToolRunPresentationBody>}
#[derive(Clone,Copy)]
pub struct ToolRunView<'a>{pub tool_id:&'a str,pub identity:ToolRunIdentity,pub state:ToolRunState,pub provisional_entities:&'a semio_framework_tool_run_entities::EntityMap,pub progress:&'a ToolRunProgress,pub payload:Option<semio_framework_tool_run::payload_stream::ToolRunPayloadView<'a>>}
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunCapturedView{presentation:ToolRunPresentation,identity:ToolRunIdentity,state:ToolRunState}
impl ToolRunCapturedView{
 pub fn capture_copy_bytes()->usize{size_of::<Self>().max(ToolRunPresentation::capture_copy_bytes())}
 pub fn borrowed_view(&self)->ToolRunView<'_>{self.presentation.borrowed_view(self.identity,self.state)}
 pub fn source_identity(&self)->usize{self.presentation.identity()}
 pub fn body(&self)->&ToolRunPresentationBody{self.presentation.body()}
}
impl semio_framework_value::retained_clone::OriginalSourceProjection<ToolRunProgress> for ToolRunPresentation{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalProjectionReply<'reply,'source,ToolRunProgress>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'reply,std::ptr::NonNull<ToolRunProgress>,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"presentation progress projection exceeded its original work"))));}Ok(reply.complete(Ok(&self.body().progress)))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalProjectionReply<'reply,'source,ToolRunProgress>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'reply,std::ptr::NonNull<ToolRunProgress>,ValueError>,ValueError>{Ok(reply.pending())}
}
impl ToolRunPresentation{
 pub fn birth_demands()->Result<RetirementDemand,ValueError>{SealedShared::<ToolRunPresentationBody>::source_admission_demand()}
 pub fn capture_copy_bytes()->usize{SealedShared::<ToolRunPresentationBody>::duplicate_copy_bytes()}
 pub fn admit(body:ToolRunPresentationBody,effect:&mut RetainedAllocationEffect<'_>)->Result<(Self,RetainedCloneProgress),(ValueError,ToolRunPresentationBody)>{let quote=match Self::birth_demands(){Ok(quote)=>quote,Err(error)=>return Err((error,body))};if !retained_clone_grant_admits(effect.grant(),quote){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original presentation fixed-body publication is not funded"),body));}admit_sealed(body,effect).map(|(body,mut receipt)|{receipt.copied_bytes=quote.copy_bytes;(Self{body},receipt)})}
 pub fn capture(&self,effect:&mut RetainedAllocationEffect<'_>)->Result<(Self,RetainedCloneProgress),ValueError>{let grant=effect.grant();if grant.maximum_items==0||grant.maximum_copy_bytes<Self::capture_copy_bytes()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original presentation capture is not funded"));}duplicate_sealed(&self.body,effect).map(|(body,mut receipt)|{receipt.copied_bytes=Self::capture_copy_bytes();(Self{body},receipt)})}
}
impl ToolRunPresentation{
 pub fn body(&self)->&ToolRunPresentationBody{self.body.get()}
 pub fn identity(&self)->usize{self.body.identity()}
 pub fn borrowed_view(&self,identity:ToolRunIdentity,state:ToolRunState)->ToolRunView<'_>{let body=self.body();ToolRunView{tool_id:body.tool_id.get(),identity,state,provisional_entities:body.provenance.entities(),progress:&body.progress,payload:body.payload.as_ref().and_then(|payload|payload.get().borrowed_view(identity))}}
 pub fn capture_view(&self,identity:ToolRunIdentity,state:ToolRunState,effect:&mut RetainedAllocationEffect<'_>)->Result<(ToolRunCapturedView,RetainedCloneProgress),ValueError>{if effect.grant().maximum_copy_bytes<ToolRunCapturedView::capture_copy_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"captured presentation and scalar envelope handoff is not funded"));}let(presentation,mut receipt)=self.capture(effect)?;receipt.copied_bytes=ToolRunCapturedView::capture_copy_bytes();Ok((ToolRunCapturedView{presentation,identity,state},receipt))}
}

#[cfg(test)]
#[path="../../🧪️tests/⏱️effect/🦀️.rs"]
mod effect_harness;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
