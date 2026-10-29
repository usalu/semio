//! 🔺️ Native document deltas retain sparse prepared branch and link authorities until central adoption.
use crate::native_snapshot::EnergyModelSnapshot;
use crate::native_state::{EnergyModelState,diff::{EnergyModelStatePatch,branches::PreparedEnergyBranchesApply}};
use semio_framework_value::{ToValue,RetirementDemand,ValueError,ValueRefusalKind,shared_issuance::SealedShared,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep}};
use protocol::{ApplyCapability,MutationMessage,DiffAlgebra,MutationDiff,mutation::retained::{RetainedDiff,RetainedDiffApply,RetainedDiffPreflightAdvance,retained_mutation_demand_fits,patch::RetainedRowPatchApply}};

/// 🌳️ The semantic sparse path stage owns prepared immutable branches; unchanged fields remain absent.
#[derive(Default,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::RetireOwned,framework_schema::ArtifactSchema)]
#[artifact_schema(id="s.energy.model.diff")]
#[value(rename_all="camelCase")]
pub struct EnergyModelDiff{
 #[state(artifact)]pub model:EnergyModelStatePatch,
 #[value(skip_serializing_if="Option::is_none")]#[state(artifact)]pub referenced_model:Option<SealedShared<Option<store::ArtifactLink>>>,
 #[value(skip_serializing_if="Option::is_none")]#[state(artifact)]pub weather_link:Option<SealedShared<Option<store::ArtifactLink>>>,
}
protocol::retained_row_patch!(pub link_fields for EnergyModelDiff=>EnergyModelSnapshot{referenced_model:SealedShared<Option<store::ArtifactLink>>,weather_link:SealedShared<Option<store::ArtifactLink>>});
impl MutationDiff<EnergyModelSnapshot> for EnergyModelDiff{}
impl DiffAlgebra<EnergyModelSnapshot> for EnergyModelDiff{fn is_empty(&self)->bool{self.model.is_empty()&&self.referenced_model.is_none()&&self.weather_link.is_none()}}
impl RetainedDiff<EnergyModelSnapshot> for EnergyModelDiff{type Apply=EnergyDocumentApply;}

/// 🪡️ Every field is prepared before the first central document write; both original child plans close independently.
pub struct EnergyDocumentApply{model:PreparedEnergyBranchesApply,links:RetainedRowPatchApply<EnergyModelSnapshot,EnergyModelDiff>,phase:u8}
impl RetainedDiffApply<EnergyModelSnapshot,EnergyModelDiff> for EnergyDocumentApply{
 fn new()->Self{Self{model:PreparedEnergyBranchesApply::new(),links:Default::default(),phase:0}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{let body=body.saturating_sub(1);match self.phase{0=>nested(self.model.preflight_demand(d.project(0,|d|&d.model),s.project(0,|s|&s.model),p.project(0,|p|&p.model),body)?),1=>nested(self.links.stage_demand(d,s,body)?),_=>Err(invalid())}}
 fn preflight(&mut self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<RetainedDiffPreflightAdvance,ValueError>{let q=self.preflight_demand(d,s,p,body)?;if !retained_mutation_demand_fits(q,g){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}let body=body.saturating_sub(1);if self.phase==0{let step=self.model.preflight(d.project(0,|d|&d.model),s.project(0,|s|&s.model),p.project(0,|p|&p.model),body,child(g),receiving)?;let mut progress=step.progress();progress.copied_bytes+=1;if let RetainedDiffPreflightAdvance::Ready(_,admitted)=step{if !admitted{return Ok(RetainedDiffPreflightAdvance::Ready(progress,false));}self.phase=1;}return Ok(RetainedDiffPreflightAdvance::Progress(progress));}let step=self.links.stage(d,s,body,child(g))?;let mut progress=step.progress();progress.copied_bytes+=1;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;Ok(RetainedDiffPreflightAdvance::Ready(progress,true))}else{Ok(RetainedDiffPreflightAdvance::Progress(progress))}}
 fn take_messages(&mut self)->Vec<MutationMessage>{Vec::new()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:&EnergyModelSnapshot,body:usize)->Result<RetirementDemand,ValueError>{match self.phase{2=>nested(self.model.commit_demand(d.project(0,|d|&d.model),s.project(0,|s|&s.model),&p.model,body.saturating_sub(1))?),3=>nested(self.links.exchange_demand()?),_=>Err(invalid())}}
 fn commit(&mut self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:&mut EnergyModelSnapshot,g:RetainedCloneGrant,c:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let q=self.commit_demand(d,s,p,g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(q,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let step=if self.phase==2{self.model.commit(d.project(0,|d|&d.model),s.project(0,|s|&s.model),&mut p.model,child(g),c)?}else{self.links.exchange(p,c,child(g))?};let mut progress=step.progress();progress.copied_bytes+=1;if matches!(step,RetainedCloneStep::Complete(_)){self.phase+=1;}Ok(if self.phase==4{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})}
 fn begin_close(&mut self){self.model.begin_close();self.links.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.model.terminal_is_empty(){self.model.close_demand(body)}else{self.links.close_demand(body)}}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.model.terminal_is_empty(){self.model.close_step(g)}else{self.links.close_step(g)}}
 fn terminal_is_empty(&self)->bool{self.model.terminal_is_empty()&&self.links.terminal_is_empty()}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native Energy document delta lost its original staged phase")}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(1).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-1,maximum_depth:g.maximum_depth-1,..g}}
