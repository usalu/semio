//! 🧱️ Row creation composes original identity admission, authored validation, typed birth and one persistent insertion.
use super::*;
use crate::reference::{ReferenceCollection,ReferenceLookup};
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::EntityRow},diff::original::EnergyModelDiff,row::{RowBranchPath,RowBranchPreparation,RowAdmissionScan,RowAdmission,RowOutcome},leaf::RowFieldPreparation};
use semio_framework_value::retained_clone::StagedSourceAdmission;
/// ✅️ Domain rules retain their exact original input and any foreign-key or name cursors.
pub trait RowCreationValidation<P:Send+Sync+'static>:Cursor+Send+'static{
 fn quote(&self,input:RetainedCloneRef<'_,P>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>;
 fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,P>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
 fn result(&self)->Option<Option<RowOutcome>>;
 fn reference_index(&self)->Option<usize>{None}
 fn begin_close(&mut self){Cursor::mark(self)}
}
/// 🧭️ The authored create recipe supplies its actual identity, rank and typed semantic sources.
pub trait CreateRowPath:RowBranchPath{
 type Payload:Send+Sync+'static;type Birth:RowBirthSpec<Payload=Self::Payload,Row=Self::Row,Context=()>;type Validation:RowCreationValidation<Self::Payload>;
 const VALIDATE_BEFORE_RANK:bool;
 fn key(input:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,u32>;
 fn index(input:RetainedCloneRef<'_,Self::Payload>)->Option<usize>;
 fn original_index(input:RetainedCloneRef<'_,Self::Payload>,_:RetainedCloneRef<'_,EnergyModelSnapshot>)->Option<usize>{Self::index(input)}
 fn identity_collections()->&'static[ReferenceCollection]{&[]}
 fn validation()->Self::Validation;
}
type Created<S>=SealedShared<<S as RowBranchPath>::Row>;
/// 🧳️ Every created row and private list path remains owned until the sparse branch diff transfers.
pub struct CreateRowPreparation<S:CreateRowPath> where S::Row:EntityRow{
 identity:Option<Box<ReferenceLookup>>,identity_index:usize,
 input:Bound,original:Bound,scan:Option<Box<RowAdmissionScan<S::Row>>>,validation:Option<Box<S::Validation>>,birth:Option<Box<RowBirthPreparation<S::Birth>>>,publication:Option<Box<RowBranchPreparation<S>>>,
 row:Held<Created<S>>,source:Sourced<Created<S>>,row_admit:Held<StagedSourceAdmission<Created<S>,Created<S>>>,decision:Held<EnergyModelDiff>,body:Option<usize>,rank:Option<usize>,outcome:Option<RowOutcome>,invalid_rank:bool,invalid_reference:Option<usize>,phase:u8,inverse:u8,closing:bool,flight:Flight,
}
impl<S:CreateRowPath> CreateRowPreparation<S> where S::Row:EntityRow{
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(identity,publication,birth,validation,scan,source,row_admit,decision,row,input,original);
 fn admit_extra()->usize{2*size_of::<Option<Created<S>>>()+2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<Created<S>>>>()+2*size_of::<Option<StagedSourceAdmission<Created<S>,Created<S>>>>()+1}
 fn create_quote(&self,input:RetainedCloneRef<'_,S::Payload>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||!self.flight.idle(){return Err(invalid());}input.check_binding(&self.input.0)?;original.check_binding(&self.original.0)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}let nb=body.saturating_sub(1);
  match self.phase{
   0=>Ok((nested(scalar(input.binding_copy_bytes()+if self.body.is_none(){size_of::<usize>()}else{0}),1)?,input.binding_work_demand(&self.input.0)?)),
   1=>Ok((nested(scalar(original.binding_copy_bytes()),1)?,original.binding_work_demand(&self.original.0)?)),
   2=>Ok((birth::<RowAdmissionScan<S::Row>>(),1)),
   3=>{let(d,w)=self.scan.as_ref().ok_or_else(invalid)?.quote(S::rows(original),S::key(input))?;Ok((nested(d,2*size_of::<Option<usize>>()+2*size_of::<Option<RowOutcome>>()+1)?,w))},
   4=>Ok((Cursor::demand(&self.scan,body)?,Cursor::work(&self.scan,body)?)),
   5=>Ok((birth::<S::Validation>(),1)),
   6=>{let extra=1+2*size_of::<Option<RowOutcome>>()+2*size_of::<Option<usize>>();let(d,w)=self.validation.as_ref().ok_or_else(invalid)?.quote(input,original,body.saturating_sub(extra),max)?;Ok((nested(d,extra)?,w))},
   7=>Ok((Cursor::demand(&self.validation,body)?,Cursor::work(&self.validation,body)?)),
   8=>Ok((birth::<RowBirthPreparation<S::Birth>>(),1)),
   9=>{let(d,w)=self.birth.as_ref().ok_or_else(invalid)?.quote(input,nb,max)?;Ok((nested(d,1)?,w))},
   10=>{let(d,w)=self.birth.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<Created<S>>>()+1)?,w))},
   11=>Ok((Cursor::demand(&self.birth,body)?,Cursor::work(&self.birth,body)?)),
   12=>{let extra=Self::admit_extra();match self.row_admit.get(){Some(a)=>{let d=a.next_demand()?;Ok((RetirementDemand{copy_bytes:d.copy_bytes+extra,depth:d.depth+1,..d},a.next_work_demand()?))},None=>Ok((RetirementDemand{copy_bytes:2*size_of::<StagedSourceAdmission<Created<S>,Created<S>>>()+extra,depth:2,..Default::default()},1))}},
   13=>Ok((birth::<RowBranchPreparation<S>>(),1)),
   14=>{let(d,w)=self.publication.as_ref().ok_or_else(invalid)?.quote(self.source.get().ok_or_else(invalid)?.borrow(),original,nb,max)?;Ok((nested(d,1)?,w))},
   15=>{let(d,w)=self.publication.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<EnergyModelDiff>>()+1)?,w))},
   16=>Ok((Cursor::demand(&self.publication,body)?,Cursor::work(&self.publication,body)?)),
   17=>Ok((birth::<ReferenceLookup>(),1)),
   18=>{let(d,w)=self.identity.as_ref().ok_or_else(invalid)?.quote(original,S::key(input))?;Ok((nested(d,2*size_of::<Option<RowOutcome>>()+1)?,w))},
   19=>Ok((nested(Cursor::demand(&self.identity,body.saturating_sub(2*size_of::<usize>()+1))?,2*size_of::<usize>()+1)?,Cursor::work(&self.identity,body.saturating_sub(2*size_of::<usize>()+1))?)),
   _=>Err(invalid()),
  }
 }
 fn create_advance<'r>(&mut self,input:RetainedCloneRef<'_,S::Payload>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;let body_birth=if self.body.is_none(){size_of::<usize>()}else{0};
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.create_quote(input,original,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(1);
   match self.phase{
    0=>{p=add(bind(input,&mut self.input,&mut effect.nested(child(g,1+body_birth)),&mut work)?,1+body_birth)?;self.phase=1;},
    1=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=if S::identity_collections().is_empty(){2}else{17};},
    2=>{self.scan=Some(Box::new(RowAdmissionScan::new(S::original_index(input,original))));self.phase=3;p=receipt(birth::<RowAdmissionScan<S::Row>>());},
    3=>{let extra=2*size_of::<Option<usize>>()+2*size_of::<Option<RowOutcome>>()+1;let c=self.scan.as_mut().ok_or_else(invalid)?;p=add(c.advance(S::rows(original),S::key(input),&mut effect.nested(child(g,extra)),&mut work)?.progress(),extra)?;if let Some(result)=c.result(){match result{RowAdmission::Rank(rank)=>self.rank=Some(rank),RowAdmission::Duplicate=>self.outcome=Some(RowOutcome::Duplicate),RowAdmission::Missing=>self.invalid_rank=true}c.begin_close();self.phase=4;}},
    4=>{let step=custody::finish(&mut self.flight,&mut self.scan,body,effect,&mut work,recover)?;p=step.progress();if self.scan.is_none(){self.phase=if self.outcome.is_some(){99}else if self.invalid_rank&&!S::VALIDATE_BEFORE_RANK{self.outcome=Some(RowOutcome::Missing);99}else{5};}},
    5=>{self.validation=Some(Box::new(S::validation()));self.phase=6;p=receipt(birth::<S::Validation>());},
    6=>{let extra=1+2*size_of::<Option<RowOutcome>>()+2*size_of::<Option<usize>>();let c=self.validation.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(input,original,body.saturating_sub(extra),max,&mut effect.nested(child(g,extra)),r,false),c.advance_reply(input,original,body.saturating_sub(extra),max,&mut effect.nested(child(g,extra)),r,true));
     p=add(step?.progress(),extra)?;if let Some(result)=c.result(){self.outcome=result;self.invalid_reference=c.reference_index();c.begin_close();self.phase=7;}},
    7=>{let step=custody::finish(&mut self.flight,&mut self.validation,body,effect,&mut work,recover)?;p=step.progress();if self.validation.is_none(){self.phase=if self.outcome.is_some(){99}else if self.invalid_rank{self.outcome=Some(RowOutcome::Missing);99}else{8};}},
    8=>{self.birth=Some(Box::new(RowBirthPreparation::new()));self.phase=9;p=receipt(birth::<RowBirthPreparation<S::Birth>>());},
    9=>{let c=self.birth.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(input,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(input,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){self.phase=10;}},
    10=>{let extra=2*size_of::<Option<Created<S>>>()+1;let c=self.birth.as_mut().ok_or_else(invalid)?;let(row,progress)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.row.set(row);c.begin_close();self.phase=11;p=add(progress,extra)?;},
    11=>{let step=custody::finish(&mut self.flight,&mut self.birth,body,effect,&mut work,recover)?;p=step.progress();if self.birth.is_none(){self.phase=12;}},
    12=>{let extra=Self::admit_extra();if self.row_admit.is_none()&&self.source.get().is_none(){let row=self.row.take().ok_or_else(invalid)?;self.row_admit.set(StagedSourceAdmission::owned(row));p=receipt(scalar(extra));}else{let a=self.row_admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.source.slot(),effect)};work+=turn.work;p=add(turn.result?.progress(),extra)?;if a.terminal_is_empty(){drop(self.row_admit.take());}}if self.source.get().is_some(){self.phase=13;}},
    13=>{self.publication=Some(Box::new(RowBranchPreparation::insert(self.rank.ok_or_else(invalid)?)));self.phase=14;p=receipt(birth::<RowBranchPreparation<S>>());},
    14=>{let c=self.publication.as_mut().ok_or_else(invalid)?;let src=self.source.get().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(src.borrow(),original,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(src.borrow(),original,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){self.phase=15;}},
    15=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let c=self.publication.as_mut().ok_or_else(invalid)?;let(diff,progress)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.decision.set(diff);c.begin_close();self.phase=16;p=add(progress,extra)?;},
    16=>{let step=custody::finish(&mut self.flight,&mut self.publication,body,effect,&mut work,recover)?;p=step.progress();if self.publication.is_none(){self.phase=99;self.outcome=Some(RowOutcome::Changed);}},
    17=>{self.identity=Some(Box::new(ReferenceLookup::new(*S::identity_collections().get(self.identity_index).ok_or_else(invalid)?)));self.phase=18;p=receipt(birth::<ReferenceLookup>());},
    18=>{let extra=2*size_of::<Option<RowOutcome>>()+1;let c=self.identity.as_mut().ok_or_else(invalid)?;p=add(c.advance(original,S::key(input),&mut effect.nested(child(g,extra)))?.progress(),extra)?;if let Some(found)=c.result(){if found{self.outcome=Some(RowOutcome::Duplicate);}c.begin_close();self.phase=19;}},
    19=>{let step=custody::finish(&mut self.flight,&mut self.identity,body.saturating_sub(2*size_of::<usize>()+1),effect,&mut work,recover)?;p=add(step.progress(),2*size_of::<usize>()+1)?;if self.identity.is_none(){if self.outcome.is_some(){self.phase=99;}else{self.identity_index+=1;self.phase=if self.identity_index<S::identity_collections().len(){17}else{2};}}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original native Energy create row")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:CreateRowPath] CreateRowPreparation<S>,[S::Row:EntityRow],|s|s.begin_close());
impl<S:CreateRowPath> RowFieldPreparation for CreateRowPreparation<S> where S::Row:EntityRow{
 type Input=S::Payload;type Inverse=();
 fn new()->Self{Self{identity:None,identity_index:0,input:Default::default(),original:Default::default(),scan:None,validation:None,birth:None,publication:None,row:Default::default(),source:Default::default(),row_admit:Default::default(),decision:Default::default(),body:None,rank:None,outcome:None,invalid_rank:false,invalid_reference:None,phase:0,inverse:0,closing:false,flight:Default::default()}}
 fn check(&self,input:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{input.check_binding(&self.input.0)?;original.check_binding(&self.original.0)?;Ok(())}
 fn quote(&self,input:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{self.create_quote(input,original,body,max)}
 fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.create_advance(input,original,body,max,effect,reply,recover)}
 fn ready(&self)->bool{!self.closing&&self.phase==99}
 fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn reference_index(&self)->Option<usize>{self.invalid_reference}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_quote(&self,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=100||self.inverse!=0{return Err(invalid());}Ok((scalar(1),1))}
 fn prepare_inverse_reply<'r>(&mut self,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{let q=if recover{None}else{Some(self.inverse_quote(b,m))};custody::gated(e,r,q,m,|_,_|{self.inverse=1;Ok(RetainedCloneStep::Complete(receipt(scalar(1))))})}
 fn inverse_ready(&self)->bool{!self.closing&&self.inverse==1}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !self.inverse_ready(){return Err(invalid());}Ok((scalar(1),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<((),RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}self.inverse=2;Ok(Some(((),receipt(d))))}
}

#[path="✅️validation/🦀️.rs"]mod validation;pub use validation::{NamedCreationRule,NamedRowCreationValidation,FixedCreationRule,FixedRowCreationValidation,CreationRule,CreationRuleSource,RuleRowCreationValidation};
