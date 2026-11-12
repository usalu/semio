//! 🔲️ Adjacency events retain two original surface references and splice only the actual ranked pair collection.
use crate::{snapshot::{original::EnergyModelSnapshot,state::{StateAdjacencyPair,ModelGeometryState}},diff::original::EnergyModelDiff,row::{RowBranchPath,RowBranchPreparation,RowOutcome},reference::{ReferenceLookup,ReferenceCollection},leaf::RowFieldPreparation,custody::{self,Cursor,Held,Bound,Sourced},scalar::{Quote,birth,invalid,scalar,nested,child,add,receipt,bind}};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,ValueError,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,StagedSourceAdmission,admit_retained_clone_progress}};
use protocol::retained_mutation_demand_fits;
use crate::retained::{Done,Flight,Reply,conclude};
use std::mem::size_of;
/// 🔲️ The authored pair event lends both actual identifier sources and its original optional insertion rank.
pub trait AdjacencyPath:RowBranchPath<Row=StateAdjacencyPair,Branch=ModelGeometryState>{type Payload:Send+Sync+'static;const INSERT:bool;fn surface(p:RetainedCloneRef<'_,Self::Payload>,index:usize)->RetainedCloneRef<'_,u32>;fn index(p:&Self::Payload)->Option<usize>;fn row(p:RetainedCloneRef<'_,Self::Payload>)->StateAdjacencyPair;}
/// ↩️ Removing a pair retains its actual orientation and original rank for the inverse connection.
#[derive(Clone,Copy,semio_framework_value::RetireOwned)]pub struct AdjacencyInverse{pub a:u32,pub b:u32,pub rank:u32}
pub struct AdjacencyPreparation<S:AdjacencyPath>{input:Bound,original:Bound,body:Option<usize>,reference:Option<Box<ReferenceLookup>>,reference_index:usize,reference_found:bool,offset:usize,rank:Option<usize>,inverse:Option<AdjacencyInverse>,raw:Option<StateAdjacencyPair>,row:Held<SealedShared<StateAdjacencyPair>>,source:Sourced<SealedShared<StateAdjacencyPair>>,row_admit:Held<StagedSourceAdmission<SealedShared<StateAdjacencyPair>,SealedShared<StateAdjacencyPair>>>,publication:Option<Box<RowBranchPreparation<S>>>,decision:Held<EnergyModelDiff>,phase:u8,closing:bool,outcome:Option<RowOutcome>,flight:Flight}
impl<S:AdjacencyPath> AdjacencyPreparation<S>{
 fn check_original(&self,p:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;Ok(())}
 fn pair(p:RetainedCloneRef<'_,S::Payload>)->(u32,u32){(*S::surface(p,0).get(),*S::surface(p,1).get())}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(publication,source,row_admit,decision,row,reference,original,input);
}
crate::custody::std_cursor!([S:AdjacencyPath] AdjacencyPreparation<S>,[],|s|s.begin_close());
impl<S:AdjacencyPath> RowFieldPreparation for AdjacencyPreparation<S>{
 type Input=S::Payload;type Inverse=AdjacencyInverse;
 fn new()->Self{Self{input:Default::default(),original:Default::default(),body:None,reference:None,reference_index:0,reference_found:false,offset:0,rank:None,inverse:None,raw:None,row:Default::default(),source:Default::default(),row_admit:Default::default(),publication:None,decision:Default::default(),phase:0,closing:false,outcome:None,flight:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{self.check_original(p,o)}
 fn quote(&self,p:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{if self.closing||self.phase>=99||self.body.is_some_and(|v|v!=body)||!self.flight.idle(){return Err(invalid());}self.check_original(p,o)?;match self.phase{
 0=>Ok((nested(scalar(p.binding_copy_bytes()+size_of::<usize>()),1)?,p.binding_work_demand(&self.input.0)?)),
 1=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),
 2=>Ok((scalar(2*size_of::<(u32,u32)>()+2*size_of::<Option<usize>>()+1),1)),
 3=>Ok((birth::<ReferenceLookup>(),1)),
 4=>{let(d,w)=self.reference.as_ref().ok_or_else(invalid)?.quote(o,S::surface(p,self.reference_index))?;Ok((nested(d,2)?,w))},
 5=>Ok((nested(Cursor::demand(&self.reference,body.saturating_sub(1))?,2*size_of::<usize>()+1)?,Cursor::work(&self.reference,body.saturating_sub(1))?)),
 6=>Ok((scalar(2*size_of::<StateAdjacencyPair>()+4*size_of::<usize>()+2*size_of::<Option<AdjacencyInverse>>()+2*size_of::<Option<usize>>()+1),1)),
 7=>{let mut d=SealedShared::<StateAdjacencyPair>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<StateAdjacencyPair>>()+1).ok_or_else(invalid)?;Ok((d,SealedShared::<StateAdjacencyPair>::source_admission_work_demand()))},
 8=>{let extra=2*size_of::<Option<SealedShared<StateAdjacencyPair>>>()+2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<SealedShared<StateAdjacencyPair>>>>()+2*size_of::<Option<StagedSourceAdmission<SealedShared<StateAdjacencyPair>,SealedShared<StateAdjacencyPair>>>>()+1;match self.row_admit.get(){Some(a)=>{let d=a.next_demand()?;Ok((RetirementDemand{copy_bytes:d.copy_bytes+extra,..d},a.next_work_demand()?))},None=>Ok((RetirementDemand{copy_bytes:2*size_of::<StagedSourceAdmission<SealedShared<StateAdjacencyPair>,SealedShared<StateAdjacencyPair>>>()+extra,depth:1,..Default::default()},1))}},
 9=>Ok((birth::<RowBranchPreparation<S>>(),1)),
 10=>{let publication=self.publication.as_ref().ok_or_else(invalid)?;let(d,w)=if S::INSERT{publication.quote(self.source.get().ok_or_else(invalid)?.borrow(),o,body.saturating_sub(1),max)?}else{publication.quote_remove(o,body.saturating_sub(1),max)?};Ok((nested(d,1)?,w))},
 11=>{let(d,w)=self.publication.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
 12=>Ok((Cursor::demand(&self.publication,body)?,Cursor::work(&self.publication,body)?)),
 _=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(p,None,o,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut progress=RetainedCloneProgress::default();let nested_body=body.saturating_sub(1);
   match self.phase{
 0=>{progress=add(bind(p,&mut self.input,&mut effect.nested(child(g,1+size_of::<usize>())),&mut work)?,1+size_of::<usize>())?;self.body=Some(body);self.phase=1;},
 1=>{progress=add(bind(o,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
 2=>{progress=receipt(scalar(1));if S::INSERT&&Self::pair(p).0==Self::pair(p).1{self.outcome=Some(RowOutcome::Invalid);self.phase=99;}else{self.rank=S::index(p.get());self.phase=if S::INSERT{3}else{6};}},
 3=>{self.reference=Some(Box::new(ReferenceLookup::new(ReferenceCollection::Surfaces)));self.phase=4;progress=receipt(birth::<ReferenceLookup>());},
 4=>{let r=self.reference.as_mut().ok_or_else(invalid)?;progress=add(r.advance(o,S::surface(p,self.reference_index),&mut effect.nested(child(g,2)))?.progress(),2)?;if let Some(found)=r.result(){self.reference_found=found;r.begin_close();self.phase=5;}},
 5=>{let step=custody::finish(&mut self.flight,&mut self.reference,body,effect,&mut work,recover)?;progress=step.progress();if self.reference.is_none(){if !self.reference_found{self.outcome=Some(RowOutcome::InvalidReference);self.phase=99;}else if self.reference_index==0{self.reference_index=1;self.phase=3;}else{self.phase=6;}}},
 6=>{progress=receipt(scalar(1));let rows=S::rows(o);let(a,b)=Self::pair(p);if self.offset==rows.get().len(){if !S::INSERT{self.outcome=Some(RowOutcome::Missing);self.phase=99;}else{let rank=self.rank.unwrap_or(rows.get().len());if rank>rows.get().len(){self.outcome=Some(RowOutcome::Missing);self.phase=99;}else{self.rank=Some(rank);self.inverse=Some(AdjacencyInverse{a,b,rank:u32::try_from(rank).map_err(|_|invalid())?});self.raw=Some(S::row(p));self.phase=7;}}}else{let row=rows.get().get(self.offset).ok_or_else(invalid)?;let pair=(row.surface_a_id.0,row.surface_b_id.0);if pair==(a,b)||pair==(b,a){if S::INSERT{self.outcome=Some(RowOutcome::Duplicate);self.phase=99;}else{self.rank=Some(self.offset);self.inverse=Some(AdjacencyInverse{a:pair.0,b:pair.1,rank:u32::try_from(self.offset).map_err(|_|invalid())?});self.phase=9;}}else{if S::INSERT&&S::index(p.get()).is_none()&&self.rank.is_none()&&pair>(a,b){self.rank=Some(self.offset);}self.offset+=1;}}},
 7=>{let turn=SealedShared::admit_from(&mut self.raw,SharedIssuer::owned(),self.row.slot(),&mut effect.nested(g));work+=turn.work;progress=turn.result?.progress();if self.row.is_some(){self.phase=8;}},
 8=>{let extra=2*size_of::<Option<SealedShared<StateAdjacencyPair>>>()+1;if self.row_admit.is_none(){let row=self.row.take().ok_or_else(invalid)?;self.row_admit.set(StagedSourceAdmission::owned(row));progress=receipt(scalar(extra));}else{let a=self.row_admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.source.slot(),effect)};work+=turn.work;progress=add(turn.result?.progress(),extra)?;if a.terminal_is_empty(){drop(self.row_admit.take());}}if self.source.get().is_some(){self.phase=9;}},
 9=>{let rank=self.rank.ok_or_else(invalid)?;self.publication=Some(Box::new(if S::INSERT{RowBranchPreparation::insert(rank)}else{RowBranchPreparation::remove(rank)}));self.phase=10;progress=receipt(birth::<RowBranchPreparation<S>>());},
 10=>{let publication=self.publication.as_mut().ok_or_else(invalid)?;
  let step=if S::INSERT{let source=self.source.get().ok_or_else(invalid)?;protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| publication.advance_reply(source.borrow(),o,nested_body,max,&mut effect.nested(child(g,1)),r,false),publication.advance_reply(source.borrow(),o,nested_body,max,&mut effect.nested(child(g,1)),r,true))}else{protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| publication.advance_remove_reply(o,nested_body,max,&mut effect.nested(child(g,1)),r,false),publication.advance_remove_reply(o,nested_body,max,&mut effect.nested(child(g,1)),r,true))};
  progress=add(step?.progress(),1)?;if publication.ready(){self.phase=11;}},
 11=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let publication=self.publication.as_mut().ok_or_else(invalid)?;let(diff,paid)=publication.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(diff);publication.begin_close();self.phase=12;progress=add(paid,extra)?;},
 12=>{let step=custody::finish(&mut self.flight,&mut self.publication,body,effect,&mut work,recover)?;progress=step.progress();if self.publication.is_none(){self.outcome=Some(RowOutcome::Changed);self.phase=99;}},
 _=>return Err(invalid().into())}
   admit_retained_clone_progress(g,progress,"original Energy adjacency event")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn ready(&self)->bool{!self.closing&&self.phase==99}fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_quote(&self,_:usize,_:u64)->Result<Quote,ValueError>{if self.phase!=100||self.inverse.is_none(){return Err(invalid());}Ok((scalar(1),1))}
 fn inverse_ready(&self)->bool{self.phase==100&&self.inverse.is_some()}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !self.inverse_ready(){return Err(invalid());}Ok((scalar(2*size_of::<Option<AdjacencyInverse>>()),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}Ok(Some((self.inverse.take().ok_or_else(invalid)?,receipt(d))))}
}
