//! 📊️ Reporting registrations retain their actual composite string identity and original ranked collection.
use crate::{snapshot::{original::EnergyModelSnapshot,state::{StateOutputVariableSpec,ModelOutputState,fields::RetainedSealedField}},diff::original::EnergyModelDiff,row::{RowBranchPath,RowBranchPreparation,RowOutcome},leaf::RowFieldPreparation,custody::{self,Cursor,Held,Bound,Sourced},scalar::{Quote,birth,invalid,scalar,nested,child,add,receipt,bind},model::OutputReportFrequency,retained::{Done,Flight,Reply,conclude,native_child}};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,ValueError,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,StagedSourceAdmission,admit_retained_clone_progress}};
use protocol::{retained_mutation_demand_fits,RetainedFieldPreparation};
use std::mem::size_of;

type Spec=StateOutputVariableSpec;
type Created=SealedShared<Spec>;
/// 🧭️ The authored event supplies its genuine two-string identity and optional insertion rank.
pub trait OutputVariablePath:RowBranchPath<Row=StateOutputVariableSpec,Branch=ModelOutputState>{type Payload:Send+Sync+'static;const INSERT:bool;fn text(p:RetainedCloneRef<'_,Self::Payload>,field:usize)->RetainedCloneRef<'_,String>;fn frequency(p:&Self::Payload)->Option<OutputReportFrequency>;fn index(p:&Self::Payload)->Option<usize>;}
/// ↩️ Both directions preserve complete registration data without manufacturing an entity id.
#[derive(semio_framework_value::RetireOwned)]pub struct OutputVariableInverse{pub name:String,pub key:String,pub reporting_frequency:OutputReportFrequency,pub index:u32}
pub struct OutputVariablePreparation<P:OutputVariablePath>{
 authored:Bound,original:Bound,inverse_freq:Option<OutputReportFrequency>,body:Option<usize>,rank:usize,position:usize,field:usize,offset:usize,nonblank:bool,phase:u8,outcome:Option<RowOutcome>,closing:bool,
 seal:Option<Box<RetainedSealedField<String>>>,name:Held<SealedShared<String>>,key:Held<SealedShared<String>>,frequency_raw:Option<OutputReportFrequency>,frequency:Held<SealedShared<OutputReportFrequency>>,spec_raw:Held<Spec>,created:Held<Created>,
 source:Sourced<Created>,admit:Held<StagedSourceAdmission<Created,Created>>,publication:Option<Box<RowBranchPreparation<P>>>,decision:Held<EnergyModelDiff>,
 inverse_stage:Option<Box<RetainedFieldPreparation<String>>>,inverse_name:Held<String>,inverse_key:Held<String>,inverse_field:usize,flight:Flight,
}
impl<P:OutputVariablePath> OutputVariablePreparation<P>{
 fn text(row:RetainedCloneRef<'_,Spec>,field:usize)->RetainedCloneRef<'_,String>{row.project(field,move|r|if field==0{r.name.get()}else{r.key.get()})}
 fn check_original(&self,p:RetainedCloneRef<'_,P::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.authored.0)?;o.check_binding(&self.original.0)?;Ok(())}
 fn row_at<'a>(&self,o:RetainedCloneRef<'a,EnergyModelSnapshot>)->RetainedCloneRef<'a,Spec>{let position=self.position;P::rows(o).project(1,move|l|l.get(position).expect("original output registration rank is in range"))}
 fn scan_bytes(&self,p:RetainedCloneRef<'_,P::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<usize,ValueError>{let rows=P::rows(o);let row=rows.get().get(self.position).ok_or_else(invalid)?;let a=P::text(p,self.field).get();let b=if self.field==0{row.name.get()}else{row.key.get()};Ok(if a.len()==b.len(){(a.len()-self.offset).min(64)}else{0})}
 fn old_text<'a>(&self,p:RetainedCloneRef<'a,P::Payload>,o:RetainedCloneRef<'a,EnergyModelSnapshot>)->RetainedCloneRef<'a,String>{if P::INSERT{P::text(p,self.inverse_field)}else{Self::text(self.row_at(o),self.inverse_field)}}
 fn sealed_slot(&mut self)->&mut Held<SealedShared<String>>{if self.field==0{&mut self.name}else{&mut self.key}}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 fn admit_extra()->usize{2*size_of::<Option<Created>>()+2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<Created>>>()+2*size_of::<Option<StagedSourceAdmission<Created,Created>>>()+1}
 fn admission<T:Send+Sync+'static>(extra:usize)->Result<RetirementDemand,ValueError> where SealedShared<T>:Sized{let mut d=SealedShared::<T>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(invalid)?;Ok(d)}
 pub fn quote_phase(&self,p:RetainedCloneRef<'_,P::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||self.body.is_some_and(|b|b!=body)||!self.flight.idle(){return Err(invalid())}self.check_original(p,o)?;let nb=body.saturating_sub(1);
  match self.phase{
   0=>Ok((nested(scalar(p.binding_copy_bytes()),1+size_of::<usize>())?,p.binding_work_demand(&self.authored.0)?)),
   1=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),
   2=>Ok((scalar(64+3*size_of::<usize>()+2),1)),
   3=>Ok((scalar(4*size_of::<usize>()+2),1)),
   4=>Ok((scalar(2*self.scan_bytes(p,o)?+4*size_of::<usize>()+3),1)),
   6=>Ok((birth::<RetainedSealedField<String>>(),1)),
   7=>{let s=self.seal.as_ref().ok_or_else(invalid)?;let t=P::text(p,self.field);Ok((nested(s.demand(t,nb,max)?,1)?,s.advance_work_demand(t,nb,max)?))},
   8=>Ok((nested(self.seal.as_ref().ok_or_else(invalid)?.take_demand()?,size_of::<Option<SealedShared<String>>>()+1)?,1)),
   9=>Ok((Cursor::demand(&self.seal,body)?,Cursor::work(&self.seal,body)?)),
   10=>Ok((scalar(2*size_of::<OutputReportFrequency>()+2*size_of::<Option<OutputReportFrequency>>()+1),1)),
   11=>Ok((Self::admission::<OutputReportFrequency>(2*size_of::<Option<OutputReportFrequency>>()+1)?,SealedShared::<OutputReportFrequency>::source_admission_work_demand())),
   12=>Ok((scalar(3*size_of::<Spec>()+size_of::<Option<Spec>>()+1),1)),
   13=>Ok((Self::admission::<Spec>(2*size_of::<Option<Spec>>()+1)?,SealedShared::<Spec>::source_admission_work_demand())),
   14=>{let extra=Self::admit_extra();match self.admit.get(){Some(a)=>{let d=a.next_demand()?;Ok((RetirementDemand{copy_bytes:d.copy_bytes+extra,..d},a.next_work_demand()?))},None=>Ok((RetirementDemand{copy_bytes:2*size_of::<StagedSourceAdmission<Created,Created>>()+extra,depth:1,..Default::default()},1))}},
   15=>Ok((birth::<RowBranchPreparation<P>>(),1)),
   20=>Ok((birth::<RetainedFieldPreparation<String>>(),1)),
   21=>{let s=self.inverse_stage.as_ref().ok_or_else(invalid)?;let old=self.old_text(p,o);Ok((nested(s.demand(old,nb,max)?,1)?,s.advance_work_demand(old,nb,max)?))},
   22=>Ok((nested(self.inverse_stage.as_ref().ok_or_else(invalid)?.exchange_demand()?,2*size_of::<Option<String>>()+1)?,1)),
   23=>Ok((Cursor::demand(&self.inverse_stage,body)?,Cursor::work(&self.inverse_stage,body)?)),
   16=>{let publication=self.publication.as_ref().ok_or_else(invalid)?;let(d,w)=if P::INSERT{publication.quote(self.source.get().ok_or_else(invalid)?.borrow(),o,nb,max)?}else{publication.quote_remove(o,nb,max)?};Ok((nested(d,1)?,w))},
   17=>{let(d,w)=self.publication.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
   18=>Ok((Cursor::demand(&self.publication,body)?,Cursor::work(&self.publication,body)?)),
   _=>Err(invalid()),
  }
 }
}
impl<P:OutputVariablePath> OutputVariablePreparation<P>{
 semio_s_artifact_energy_model_native_mutations::custody_impl!(inverse_stage,publication,seal,source,admit,decision,created,spec_raw,frequency,name,key,inverse_name,inverse_key,original,authored);
 fn advance_phase<'r>(&mut self,p:RetainedCloneRef<'_,P::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote_phase(p,o,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut progress=RetainedCloneProgress::default();let nb=body.saturating_sub(1);
   match self.phase{
    0=>{progress=add(bind(p,&mut self.authored,&mut effect.nested(child(g,1+size_of::<usize>())),&mut work)?,1+size_of::<usize>())?;self.body=Some(body);self.phase=1;},
    1=>{progress=add(bind(o,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=if P::INSERT{2}else{3};},
    2=>{let text=P::text(p,0).get();let mut end=(self.offset+64).min(text.len());while !text.is_char_boundary(end){end-=1}self.nonblank|=text[self.offset..end].chars().any(|c|!c.is_whitespace());self.offset=end;if end==text.len(){self.offset=0;if self.nonblank{self.phase=3}else{self.outcome=Some(RowOutcome::Invalid);self.phase=99}}progress=receipt(scalar(1));},
    3=>{progress=receipt(scalar(1));let len=P::rows(o).get().len();if self.position==len{if P::INSERT{self.rank=P::index(p.get()).unwrap_or(self.position);if self.rank>self.position{self.outcome=Some(RowOutcome::Missing);self.phase=99}else{u32::try_from(self.rank).map_err(|_|invalid())?;self.inverse_freq=Some(P::frequency(p.get()).ok_or_else(invalid)?);self.inverse_field=0;self.phase=20}}else{self.outcome=Some(RowOutcome::Missing);self.phase=99}}else{self.field=0;self.offset=0;self.phase=4}},
    4=>{progress=receipt(scalar(1));let rows=P::rows(o);let row=rows.get().get(self.position).ok_or_else(invalid)?;let a=P::text(p,self.field).get();let b=if self.field==0{row.name.get()}else{row.key.get()};let count=self.scan_bytes(p,o)?;
     if a.len()!=b.len()||a.as_bytes()[self.offset..self.offset+count]!=b.as_bytes()[self.offset..self.offset+count]{self.offset=0;self.position+=1;self.phase=3}else{self.offset+=count;if self.offset==a.len(){self.offset=0;if self.field==0{self.field=1}else if P::INSERT{self.outcome=Some(RowOutcome::Duplicate);self.phase=99}else{self.rank=self.position;u32::try_from(self.rank).map_err(|_|invalid())?;self.inverse_freq=Some(*row.reporting_frequency.get());self.inverse_field=0;self.phase=20}}}},
    20=>{self.inverse_stage=Some(Box::new(Default::default()));self.phase=21;progress=receipt(birth::<RetainedFieldPreparation<String>>());},
    21=>{let old=self.old_text(p,o);let stage=self.inverse_stage.as_deref_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| stage.advance_into(old,nb,max,&mut effect.nested(child(g,1)),r),stage.recover_advance_into(old,nb,max,&mut effect.nested(child(g,1)),r));
     progress=add(step?.progress(),1)?;if stage.value().is_some(){self.phase=22;}},
    22=>{let extra=2*size_of::<Option<String>>()+1;let Self{inverse_stage,inverse_name,inverse_key,inverse_field,..}=self;let stage=inverse_stage.as_deref_mut().ok_or_else(invalid)?;let slot=if *inverse_field==0{inverse_name.slot()}else{inverse_key.slot()};let step=native_child(effect,&mut work,|e,r|stage.take_into(slot,&mut e.nested(child(g,extra)),r))??;progress=add(step.progress(),extra)?;self.phase=23;},
    23=>{let step=custody::finish(&mut self.flight,&mut self.inverse_stage,body,effect,&mut work,recover)?;progress=step.progress();if self.inverse_stage.is_none(){self.inverse_field+=1;if self.inverse_field==2{self.field=0;self.phase=if P::INSERT{6}else{15};}else{self.phase=20;}}},
    6=>{self.seal=Some(Box::new(Default::default()));self.phase=7;progress=receipt(birth::<RetainedSealedField<String>>());},
    7=>{let seal=self.seal.as_mut().ok_or_else(invalid)?;let t=P::text(p,self.field);
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| seal.advance_reply(t,nb,max,&mut effect.nested(child(g,1)),r,false),seal.advance_reply(t,nb,max,&mut effect.nested(child(g,1)),r,true));
     progress=add(step?.progress(),1)?;if seal.ready(){self.phase=8;}},
    8=>{let extra=size_of::<Option<SealedShared<String>>>()+1;let seal=self.seal.as_mut().ok_or_else(invalid)?;let(value,paid)=seal.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;seal.begin_close();self.sealed_slot().set(value);self.phase=9;progress=add(paid,extra)?;},
    9=>{let step=custody::finish(&mut self.flight,&mut self.seal,body,effect,&mut work,recover)?;progress=step.progress();if self.seal.is_none(){self.field+=1;self.phase=if self.field==2{10}else{6};}},
    10=>{self.frequency_raw=Some(P::frequency(p.get()).ok_or_else(invalid)?);self.phase=11;progress=receipt(scalar(1));},
    11=>{let turn=SealedShared::admit_from(&mut self.frequency_raw,SharedIssuer::owned(),self.frequency.slot(),&mut effect.nested(g));work+=turn.work;progress=turn.result?.progress();if self.frequency.is_some(){self.phase=12;}},
    12=>{let spec=Spec{name:self.name.take().ok_or_else(invalid)?,key:self.key.take().ok_or_else(invalid)?,reporting_frequency:self.frequency.take().ok_or_else(invalid)?};self.spec_raw.set(spec);self.phase=13;progress=receipt(scalar(1));},
    13=>{let turn=SealedShared::admit_from(self.spec_raw.slot(),SharedIssuer::owned(),self.created.slot(),&mut effect.nested(g));work+=turn.work;progress=turn.result?.progress();if self.created.is_some(){self.phase=14;}},
    14=>{let extra=Self::admit_extra();if self.admit.is_none()&&self.source.get().is_none(){let value=self.created.take().ok_or_else(invalid)?;self.admit.set(StagedSourceAdmission::owned(value));progress=receipt(scalar(extra));}else{let a=self.admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.source.slot(),effect)};work+=turn.work;progress=add(turn.result?.progress(),extra)?;if a.terminal_is_empty(){drop(self.admit.take());}}if self.source.get().is_some(){self.phase=15;}},
    15=>{self.publication=Some(Box::new(if P::INSERT{RowBranchPreparation::insert(self.rank)}else{RowBranchPreparation::remove(self.rank)}));self.phase=16;progress=receipt(birth::<RowBranchPreparation<P>>());},
    16=>{let publication=self.publication.as_mut().ok_or_else(invalid)?;
     let step=if P::INSERT{let source=self.source.get().ok_or_else(invalid)?;protocol::retained_routed!(self.flight,effect,recover,2,body,max,None,work,|r| publication.advance_reply(source.borrow(),o,nb,max,&mut effect.nested(child(g,1)),r,false),publication.advance_reply(source.borrow(),o,nb,max,&mut effect.nested(child(g,1)),r,true))}else{protocol::retained_routed!(self.flight,effect,recover,2,body,max,None,work,|r| publication.advance_remove_reply(o,nb,max,&mut effect.nested(child(g,1)),r,false),publication.advance_remove_reply(o,nb,max,&mut effect.nested(child(g,1)),r,true))};
     progress=add(step?.progress(),1)?;if publication.ready(){self.phase=17;}},
    17=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let publication=self.publication.as_mut().ok_or_else(invalid)?;let(diff,paid)=publication.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(diff);publication.begin_close();self.phase=18;progress=add(paid,extra)?;},
    18=>{let step=custody::finish(&mut self.flight,&mut self.publication,body,effect,&mut work,recover)?;progress=step.progress();if self.publication.is_none(){self.outcome=Some(RowOutcome::Changed);self.phase=99;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,progress,"original Energy reporting registration")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==99{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==100&&self.inverse_name.is_some()&&self.inverse_key.is_some()}
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([P:OutputVariablePath] OutputVariablePreparation<P>,[],|s|s.begin_close());
impl<P:OutputVariablePath> RowFieldPreparation for OutputVariablePreparation<P>{
 type Input=P::Payload;type Inverse=OutputVariableInverse;
 fn new()->Self{Self{authored:Default::default(),original:Default::default(),inverse_freq:None,body:None,rank:0,position:0,field:0,offset:0,nonblank:false,phase:0,outcome:None,closing:false,seal:None,name:Default::default(),key:Default::default(),frequency_raw:None,frequency:Default::default(),spec_raw:Default::default(),created:Default::default(),source:Default::default(),admit:Default::default(),publication:None,decision:Default::default(),inverse_stage:None,inverse_name:Default::default(),inverse_key:Default::default(),inverse_field:0,flight:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if key.is_some(){return Err(invalid())}self.check_original(p,o)}
 fn quote(&self,p:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{self.quote_phase(p,o,b,m)}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.advance_phase(p,o,b,m,e,r,recover)}
 fn ready(&self)->bool{!self.closing&&self.phase==99}fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid())}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None)}if !effect.performed(1)?{return Err(invalid())}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_quote(&self,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=100||!Self::inverse_ready(self){return Err(invalid())}Ok((scalar(1),1))}
 fn inverse_ready(&self)->bool{Self::inverse_ready(self)}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !Self::inverse_ready(self){return Err(invalid())}Ok((scalar(3*size_of::<OutputVariableInverse>()+1),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None)}let reporting_frequency=self.inverse_freq.ok_or_else(invalid)?;let value=OutputVariableInverse{name:self.inverse_name.take().ok_or_else(invalid)?,key:self.inverse_key.take().ok_or_else(invalid)?,reporting_frequency,index:u32::try_from(self.rank).map_err(|_|invalid())?};self.phase=101;Ok(Some((value,receipt(d))))}
}
#[cfg(test)]#[path="tests.rs"]mod tests;
