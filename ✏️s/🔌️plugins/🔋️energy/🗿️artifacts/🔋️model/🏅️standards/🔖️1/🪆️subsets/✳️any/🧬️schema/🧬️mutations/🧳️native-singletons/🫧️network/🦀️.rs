//! 🫧️ Original network replacement pairs one authored zone/node per admitted item and publishes only the systems branch.
use crate::{snapshot::{original::EnergyModelSnapshot,state::{StateAirflowNetworkDefinition as Network,ModelSystemState as Systems,fields::RetainedSealedField}},diff::original::EnergyModelDiff,row::{DirectBranchOutput,BranchOutput,RowOutcome,Staged,staged_step},leaf::RowFieldPreparation,custody::{self,Cursor,Held,Bound,Sourced},scalar::{Quote,birth,invalid,scalar,nested,child,add,receipt,bind},retained::{Done,Flight,Reply,conclude,native_child}};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,ValueError,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,StagedSourceAdmission,admit_retained_clone_progress}};
use protocol::{RetainedFieldPreparation,retained_mutation_demand_fits};
use std::mem::size_of;
type Pair=(crate::model::EntityId,u32);type OptionalNetwork=Option<SealedShared<Network>>;
/// 🧭️ The original authored parallel arrays remain the only input payload.
pub trait NetworkPath:Send+Sync+'static{type Payload:Send+Sync+'static;fn present(p:&Self::Payload)->bool;fn zones(p:&Self::Payload)->&Vec<u32>;fn nodes(p:&Self::Payload)->&Vec<u32>;fn links(p:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,Vec<u32>>;fn outdoor(p:&Self::Payload)->u32;}
/// ↩️ Exact prior presence, pair order, outdoor node and links form the authored parallel-array inverse.
#[derive(semio_framework_value::RetireOwned)]pub struct NetworkInverse{pub present:bool,pub zone_ids:Vec<u32>,pub node_ids:Vec<u32>,pub outdoor_node_id:u32,pub link_ids:Vec<u32>}
/// 🧳️ Every partial vector, immutable child, displaced singleton and output keeps its own funded closure.
pub struct NetworkPreparation<S:NetworkPath>{
 authored:Bound,original:Bound,body:Option<usize>,phase:u8,index:usize,outcome:Option<RowOutcome>,closing:bool,flight:Flight,
 raw_pairs:Held<Vec<Pair>>,pairs:Held<SealedShared<Vec<Pair>>>,links:Held<SealedShared<Vec<u32>>>,raw_network:Held<Network>,network:Held<SealedShared<Network>>,value:Held<OptionalNetwork>,old:Held<OptionalNetwork>,decision:Held<EnergyModelDiff>,
 link_cursor:Option<Box<RetainedSealedField<Vec<u32>>>>,staged:Staged<Systems>,output:Option<Box<DirectBranchOutput<Systems>>>,
 old_source:Sourced<OptionalNetwork>,old_admit:Held<StagedSourceAdmission<OptionalNetwork,OptionalNetwork>>,inverse:Held<NetworkInverse>,inverse_cursor:Option<Box<RetainedFieldPreparation<Vec<u32>>>>,inverse_links:Held<Vec<u32>>,inverse_phase:u8,inverse_index:usize,marker:std::marker::PhantomData<fn()->S>,
}
fn systems(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Systems>{o.project(0,|s|s.model.systems.get())}
fn admission<T:Send+Sync+'static>(extra:usize)->Result<RetirementDemand,ValueError>{let mut d=SealedShared::<T>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(invalid)?;Ok(d)}
impl<S:NetworkPath> NetworkPreparation<S>{
 fn check_sources(&self,p:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.authored.0)?;o.check_binding(&self.original.0)?;Ok(())}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(inverse_cursor,output,staged,link_cursor,old_admit,old_source,inverse_links,inverse,decision,network,raw_network,links,pairs,raw_pairs,value,old,original,authored);
 fn quote(&self,p:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||self.body.is_some_and(|v|v!=b)||!self.flight.idle(){return Err(invalid())}self.check_sources(p,o)?;let nb=b.saturating_sub(1);
  match self.phase{
   0=>Ok((nested(scalar(p.binding_copy_bytes()),1+size_of::<usize>())?,p.binding_work_demand(&self.authored.0)?)),
   1=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),
   2|3|4=>Ok((scalar(2*size_of::<S::Payload>()+2*size_of::<Pair>()+4*size_of::<usize>()+4),1)),
   5=>Ok((RetirementDemand{copy_bytes:2*size_of::<Vec<Pair>>()+size_of::<usize>()+1,capacity_bytes:S::zones(p.get()).len().checked_mul(size_of::<Pair>()).ok_or_else(invalid)?,depth:1,..Default::default()},1)),
   6=>Ok((scalar(2*size_of::<Pair>()+2*size_of::<usize>()+1),1)),
   7=>Ok((admission::<Vec<Pair>>(2*size_of::<Option<Vec<Pair>>>()+1)?,SealedShared::<Vec<Pair>>::source_admission_work_demand())),
   8=>Ok((birth::<RetainedSealedField<Vec<u32>>>(),1)),
   9=>{let s=self.link_cursor.as_ref().ok_or_else(invalid)?;let l=S::links(p);Ok((nested(s.demand(l,nb,max)?,1)?,s.advance_work_demand(l,nb,max)?))},
   10=>Ok((nested(self.link_cursor.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<SealedShared<Vec<u32>>>>()+1)?,1)),
   11=>Ok((scalar(3*size_of::<Network>()+2*size_of::<Option<Network>>()+1),1)),
   12=>Ok((admission::<Network>(2*size_of::<Option<Network>>()+2*size_of::<Option<OptionalNetwork>>()+1)?,SealedShared::<Network>::source_admission_work_demand())),
   13=>self.staged.quote::<OptionalNetwork>(systems(o),b,max),
   14=>Ok((birth::<DirectBranchOutput<Systems>>(),1)),
   15=>self.output.as_ref().ok_or_else(invalid)?.quote(self.staged.sealed.get_opt(),o,nb,max),
   16=>{let(d,w)=self.output.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
   17=>Ok((Cursor::demand(&self.output,b)?,Cursor::work(&self.output,b)?)),
   _=>Err(invalid()),
  }
 }
 fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(input,o,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let v=input.get();let nb=body.saturating_sub(1);let mut p=RetainedCloneProgress::default();
   match self.phase{
    0=>{p=add(bind(input,&mut self.authored,&mut effect.nested(child(g,1+size_of::<usize>())),&mut work)?,1+size_of::<usize>())?;self.phase=1;},
    1=>{p=add(bind(o,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{p=receipt(scalar(1));let before=&systems(o).get().airflow_network;let present=S::present(v);if S::zones(v).len()!=S::nodes(v).len()||!present&&(!S::zones(v).is_empty()||!S::links(input).get().is_empty()){self.outcome=Some(RowOutcome::Invalid);self.phase=99;}else if !present&&before.is_none(){self.outcome=Some(RowOutcome::Unchanged);self.phase=99;}else if !present{self.value.set(None);self.phase=13;}else if before.as_ref().is_some_and(|old|old.get().outdoor_node_id==S::outdoor(v)&&old.get().zone_node_ids.get().len()==S::zones(v).len()&&old.get().link_ids.get().len()==S::links(input).get().len()){self.index=0;self.phase=3;}else{self.phase=5;}},
    3=>{p=receipt(scalar(1));let old=systems(o).get().airflow_network.as_ref().ok_or_else(invalid)?.get();if self.index==S::zones(v).len(){self.index=0;self.phase=4;}else{let pair=old.zone_node_ids.get()[self.index];if pair.0.0!=S::zones(v)[self.index]||pair.1!=S::nodes(v)[self.index]{self.phase=5;}else{self.index+=1;}}},
    4=>{p=receipt(scalar(1));let old=systems(o).get().airflow_network.as_ref().ok_or_else(invalid)?.get();if self.index==S::links(input).get().len(){self.outcome=Some(RowOutcome::Unchanged);self.phase=99;}else if old.link_ids.get()[self.index]!=S::links(input).get()[self.index]{self.phase=5;}else{self.index+=1;}},
    5=>{let(values,bytes)=custody::exact_vec::<Pair>(S::zones(v).len(),effect)?;p=receipt(scalar(1));p.retained_capacity_bytes=bytes;self.raw_pairs.set(values);self.index=0;self.phase=6;},
    6=>{p=receipt(scalar(1));if self.index==S::zones(v).len(){self.phase=7;}else{self.raw_pairs.get_mut().ok_or_else(invalid)?.push((crate::model::EntityId(S::zones(v)[self.index]),S::nodes(v)[self.index]));self.index+=1;}},
    7=>{let turn=SealedShared::admit_from(self.raw_pairs.slot(),SharedIssuer::owned(),self.pairs.slot(),&mut effect.nested(g));work+=turn.work;p=turn.result?.progress();if self.pairs.is_some(){self.phase=8;}},
    8=>{self.link_cursor=Some(Box::new(RetainedSealedField::new()));self.phase=9;p=receipt(birth::<RetainedSealedField<Vec<u32>>>());},
    9=>{let c=self.link_cursor.as_mut().ok_or_else(invalid)?;let l=S::links(input);
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(l,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(l,nb,max,&mut effect.nested(child(g,1)),r,true));
     p=add(step?.progress(),1)?;if c.ready(){self.phase=10;}},
    10=>{let extra=2*size_of::<Option<SealedShared<Vec<u32>>>>()+1;let c=self.link_cursor.as_mut().ok_or_else(invalid)?;let(value,r)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;c.begin_close();self.links.set(value);p=add(r,extra)?;self.phase=11;},
    11=>{let net=Network{zone_node_ids:self.pairs.take().ok_or_else(invalid)?,outdoor_node_id:S::outdoor(v),link_ids:self.links.take().ok_or_else(invalid)?};self.raw_network.set(net);self.phase=12;p=receipt(scalar(1));},
    12=>{let turn=SealedShared::admit_from(self.raw_network.slot(),SharedIssuer::owned(),self.network.slot(),&mut effect.nested(g));work+=turn.work;p=turn.result?.progress();if self.network.is_some(){self.value.set(Some(self.network.take().ok_or_else(invalid)?));self.phase=13;}},
    13=>{let Self{staged,value,old,..}=self;let mut exchange=|stage:&mut crate::snapshot::state::fields::RetainedBranchProjection<Systems>,e:&mut RetainedAllocationEffect<'_>|stage.exchange(|s:&mut Systems|&mut s.airflow_network,value.slot(),old.slot(),e);
     let step=staged_step::<_,OptionalNetwork>(staged,systems(o),body,max,effect,&mut work,recover,&mut exchange,&mut units)?;p=step.progress();if staged.ready(){self.phase=14;}},
    14=>{self.output=Some(Box::new(DirectBranchOutput::new(|b|{let mut diff=EnergyModelDiff::default();diff.model.systems=Some(b);diff})));self.phase=15;p=receipt(birth::<DirectBranchOutput<Systems>>());},
    15=>{let c=self.output.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,2,body,max,None,work,|r| c.advance_reply(self.staged.sealed.slot(),o,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(self.staged.sealed.slot(),o,nb,max,&mut effect.nested(child(g,1)),r,true));
     p=add(step?.progress(),1)?;if c.ready(){self.phase=16;}},
    16=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let c=self.output.as_mut().ok_or_else(invalid)?;let(value,r)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(value);c.begin_close();self.phase=17;p=add(r,extra)?;},
    17=>{let step=custody::finish(&mut self.flight,&mut self.output,body,effect,&mut work,recover)?;p=step.progress();if self.output.is_none(){self.outcome=Some(RowOutcome::Changed);self.phase=99;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original native airflow network")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:NetworkPath] NetworkPreparation<S>,[],|s|s.begin_close());
#[path="↩️inverse/🦀️.rs"]mod inverse;

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
