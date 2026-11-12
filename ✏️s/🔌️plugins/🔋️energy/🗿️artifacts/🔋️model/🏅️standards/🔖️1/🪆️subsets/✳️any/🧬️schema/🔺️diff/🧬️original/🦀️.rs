//! 🔺️ Native document deltas retain sparse prepared branch and link authorities until central adoption.
use crate::native_snapshot::EnergyModelSnapshot;
use crate::native_state::{EnergyModelState,diff::{EnergyModelStatePatch,branches::PreparedEnergyBranchesApply}};
use semio_framework_value::{NativeDecodeControl,OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,shared_issuance::SealedShared,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep}};
use protocol::{ApplyCapability,MutationMessage,DiffAlgebra,MutationDiff,RetainedDiff,RetainedDiffApply,RetainedDiffPreflightAdvance,retained_mutation_demand_fits,RetainedRowPatchApply,RetainedPatchEmpty,RetainedDone as Done,RetainedFlight as Flight,RetainedReply as Reply,retained_conclude as conclude,retained_settle as settle};

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
impl RetainedPatchEmpty for EnergyModelDiff{fn empty_patch_work_demand()->u64{10}fn empty_patch()->(Self,u64){(Self::default(),10)}}
impl MutationDiff<EnergyModelSnapshot> for EnergyModelDiff{}
impl DiffAlgebra<EnergyModelSnapshot> for EnergyModelDiff{fn is_empty(&self)->bool{self.model.is_empty()&&self.referenced_model.is_none()&&self.weather_link.is_none()}}
impl RetainedDiff<EnergyModelSnapshot> for EnergyModelDiff{type Apply=EnergyDocumentApply;}

/// 🪡️ Every field is prepared before the first central document write; both original child plans close independently.
pub struct EnergyDocumentApply{model:PreparedEnergyBranchesApply,links:RetainedRowPatchApply<EnergyModelSnapshot,EnergyModelDiff>,flight:Flight,phase:u8}
fn model_diff<'a>(d:RetainedCloneRef<'a,EnergyModelDiff>)->RetainedCloneRef<'a,EnergyModelStatePatch>{d.project(0,|d|&d.model)}
fn model_of<'a>(s:RetainedCloneRef<'a,EnergyModelSnapshot>)->RetainedCloneRef<'a,EnergyModelState>{s.project(0,|s|&s.model)}
impl RetainedDiffApply<EnergyModelSnapshot,EnergyModelDiff> for EnergyDocumentApply{
 fn new()->Self{Self{model:PreparedEnergyBranchesApply::new(),links:Default::default(),flight:Default::default(),phase:0}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{let body=body.saturating_sub(1);match self.phase{0=>nested(self.model.preflight_demand(model_diff(d),model_of(s),model_of(p),body,max)?),1=>nested(self.links.stage_demand(d,s,body,max)?),_=>Err(invalid())}}
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{self.preflight_demand(d,s,p,body,max)?;let body=body.saturating_sub(1);if self.phase==0{self.model.preflight_work_demand(model_diff(d),model_of(s),model_of(p),body,max)}else{self.links.stage_work_demand(d,s,body,max)}}
 fn preflight_reply<'r>(&mut self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedDiffPreflightAdvance,RetirementPollExit>=(||{
   let route:u8=if recover{self.flight.route().ok_or_else(invalid)?}else{
    if !self.flight.idle(){return Err(invalid().into())}
    if self.preflight_work_demand(d,s,p,body,max)?==0{return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()))}
    let q=self.preflight_demand(d,s,p,body,max)?;if !retained_mutation_demand_fits(q,g){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()))}
    self.phase+1
   };
   let nested_body=body.saturating_sub(1);let before=work;
   if route==1{
    let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| self.model.preflight_reply(model_diff(d),model_of(s),model_of(p),nested_body,max,&mut effect.nested(child(g)),receiving,r,false),self.model.preflight_reply(model_diff(d),model_of(s),model_of(p),nested_body,max,&mut effect.nested(child(g)),receiving,r,true));
    let step=step.map_err(|error|tagged(error,work!=before))?;let mut progress=step.progress();progress.copied_bytes+=1;
    if let RetainedDiffPreflightAdvance::Ready(_,admitted)=step{if !admitted{return Ok(RetainedDiffPreflightAdvance::Ready(progress,false))}self.phase=1;}
    return Ok(RetainedDiffPreflightAdvance::Progress(progress));
   }
   let step=protocol::retained_routed!(self.flight,effect,recover,2,body,max,None,work,|r| self.links.stage_into(d,s,nested_body,max,&mut effect.nested(child(g)),r),self.links.recover_stage_into(d,s,nested_body,max,&mut effect.nested(child(g)),r));
   let step=step.map_err(|error|tagged(error,work!=before))?;let mut progress=step.progress();progress.copied_bytes+=1;
   if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;Ok(RetainedDiffPreflightAdvance::Ready(progress,true))}else{Ok(RetainedDiffPreflightAdvance::Progress(progress))}
  })();conclude(effect,base,result,work,RetainedDiffPreflightAdvance::progress,reply)
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{Vec::new()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:&EnergyModelSnapshot,body:usize)->Result<RetirementDemand,ValueError>{match self.phase{2=>nested(self.model.commit_demand(model_diff(d),model_of(s),&p.model,body.saturating_sub(1))?),3=>nested(self.links.exchange_demand()?),_=>Err(invalid())}}
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:&EnergyModelSnapshot,body:usize)->Result<u64,ValueError>{self.commit_demand(d,s,p,body)?;if self.phase==2{self.model.commit_work_demand(model_diff(d),model_of(s),&p.model,body.saturating_sub(1))}else{self.links.exchange_work_demand()}}
 fn commit(&mut self,d:RetainedCloneRef<'_,EnergyModelDiff>,s:RetainedCloneRef<'_,EnergyModelSnapshot>,p:&mut EnergyModelSnapshot,effect:&mut RetainedAllocationEffect<'_>,c:ApplyCapability)->OriginalRetirementOutcome{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
   let q=self.commit_demand(d,s,p,g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(q,g){return Ok(RetainedCloneStep::Progress(Default::default()));}
   let turn=if self.phase==2{self.model.commit(model_diff(d),model_of(s),&mut p.model,&mut effect.nested(child(g)),c)}else{self.links.exchange(p,c,child(g))};
   work=turn.work;let step=turn.result.map_err(|e|tagged(e,work!=0))?;let mut progress=step.progress();progress.copied_bytes+=1;
   if matches!(step,RetainedCloneStep::Complete(_)){self.phase+=1;}
   Ok(if self.phase==4{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
  })();settle(effect,base,result,work)
 }
 fn begin_close(&mut self){self.model.begin_close();self.links.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.model.terminal_is_empty(){self.model.close_demand(body)}else{self.links.close_demand(body)}}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.model.terminal_is_empty(){self.model.close_work_demand(body)}else{self.links.close_work_demand(body)}}
 fn requires_original_birth(&self)->bool{if !self.flight.idle(){return false}if !self.model.terminal_is_empty(){self.model.requires_original_birth()}else{self.links.requires_original_birth()}}
 fn requires_original_control(&self)->bool{if let Some(loaned)=self.flight.loaned(){return loaned}if !self.model.terminal_is_empty(){self.model.requires_original_control()}else{self.links.requires_original_control()}}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if !self.model.terminal_is_empty(){self.model.close_original_birth_step(original,grant)}else{self.links.close_original_birth_step(original,grant)}}
 fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let route:u8;let loaned:bool;
   if recover{route=self.flight.route().ok_or_else(invalid)?;loaned=self.flight.loaned().unwrap_or(false);}else{
    if !self.flight.idle(){return Err(invalid().into())}
    let demand=self.close_demand(body)?;if !retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if self.requires_original_birth(){return Err(invalid().into())}
    loaned=self.requires_original_control();if loaned&&original.is_none(){return Err(invalid().into())}
    route=if !self.model.terminal_is_empty(){1}else if !self.links.terminal_is_empty(){2}else{return Ok(RetainedCloneStep::Complete(Default::default()))};
   }
   let nested_body=body.saturating_sub(1);let before=work;let loan=if loaned{original.as_deref_mut()}else{None};
   let step=protocol::retained_routed!(self.flight,effect,recover,route,body,0,loan,work,|r| if route==1{self.model.close_reply(if loaned{original.as_deref_mut()}else{None},nested_body,&mut effect.nested(child(g)),r,false)}else{self.links.close_reply(if loaned{original.as_deref_mut()}else{None},nested_body,&mut effect.nested(child(g)),r,false)},if route==1{self.model.close_reply(if loaned{original.as_deref_mut()}else{None},nested_body,&mut effect.nested(child(g)),r,true)}else{self.links.close_reply(if loaned{original.as_deref_mut()}else{None},nested_body,&mut effect.nested(child(g)),r,true)});
   let step=step.map_err(|error|tagged(error,work!=before))?;let mut progress=step.progress();progress.copied_bytes+=1;
   Ok(RetainedCloneStep::Progress(progress))
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn terminal_is_empty(&self)->bool{self.model.terminal_is_empty()&&self.links.terminal_is_empty()&&self.flight.idle()}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native Energy document delta lost its original staged phase")}
fn tagged(error:OriginalRetirementFailure,paid:bool)->OriginalRetirementFailure{if !paid{return error}let mut progress=error.retained_progress();progress.copied_bytes=progress.copied_bytes.saturating_add(1);error.with_retained_progress(progress)}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(1).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(1),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
