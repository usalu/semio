//! 🌱️ Typed row construction admits each original field before the unique immutable row birth.
use crate::scalar::{Quote,invalid,scalar,nested,child,add,receipt,birth,bind};
use crate::custody::{self,Cursor,Held,Bound,Sourced};
use crate::retained::{Done,Flight,Reply,conclude,native_child};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,RetainedWorkOutcome,ValueError,retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use protocol::retained_mutation_demand_fits;
use std::mem::size_of;
/// 🪡️ Authored schema fields provide their genuine typed preparation and transfer contract.
pub trait RowFieldsPreparation<P:Send+Sync+'static>:Cursor+Send+'static{
 type Fields:RetireOwned+Send+Sync;
 fn new()->Self;
 fn quote(&self,payload:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>;
 fn advance_reply<'r>(&mut self,payload:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
 fn ready(&self)->bool;
 fn take_quote(&self)->Result<Quote,ValueError>;
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self){Cursor::mark(self)}
}
/// 🧩️ The final constructor only moves prepared sealed fields and copies authored fixed scalars.
pub trait RowBirthSpec:Send+Sync+'static{
 type Payload:Send+Sync+'static;type Row:RetireOwned+Send+Sync;type Context:Copy+Send+Sync+'static;type Fields:RowFieldsPreparation<Self::Payload>;
 fn validate_source(_: &Self::Payload)->Result<(),ValueError>{Ok(())}
 fn validate_context(_:Self::Context)->Result<(),ValueError>{Ok(())}
 fn finish(payload:RetainedCloneRef<'_,Self::Payload>,fields:<Self::Fields as RowFieldsPreparation<Self::Payload>>::Fields,context:Self::Context)->Result<Self::Row,ValueError>;
}
type BirthFields<S>=<<S as RowBirthSpec>::Fields as RowFieldsPreparation<<S as RowBirthSpec>::Payload>>::Fields;
/// 🧳️ Source binding, partial fields, unique result and their original retirement remain explicit owners.
pub struct RowBirthPreparation<S:RowBirthSpec,F:RowBirthForm<S::Row>=SealedRowForm>{source:Bound,fields:Held<BirthFields<S>>,cursor:Option<Box<S::Fields>>,raw:Held<S::Row>,output:Held<F::Output>,body:Option<usize>,context:S::Context,phase:u8,closing:bool,flight:Flight}
impl<S:RowBirthSpec,F:RowBirthForm<S::Row>> RowBirthPreparation<S,F>{
 pub fn with_context(context:S::Context)->Self{Self{source:Default::default(),fields:Default::default(),cursor:None,raw:Default::default(),output:Default::default(),body:None,context,phase:0,closing:false,flight:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==7}
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(cursor,output,raw,fields,source);
 pub fn quote(&self,payload:RetainedCloneRef<'_,S::Payload>,body:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=7||!self.flight.idle(){return Err(invalid());}payload.check_binding(&self.source.0)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}S::validate_context(self.context)?;S::validate_source(payload.get())?;let nb=body.saturating_sub(1);
  match self.phase{
   0=>Ok((nested(scalar(payload.binding_copy_bytes()+if self.body.is_none(){size_of::<usize>()}else{0}),1)?,payload.binding_work_demand(&self.source.0)?)),
   1=>Ok((birth::<S::Fields>(),1)),
   2=>{let(d,w)=self.cursor.as_ref().ok_or_else(invalid)?.quote(payload,nb,max)?;Ok((nested(d,1)?,w))},
   3=>{let(d,w)=self.cursor.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<BirthFields<S>>>()+1)?,w))},
   4=>Ok((Cursor::demand(&self.cursor,body)?,Cursor::work(&self.cursor,body)?)),
   5=>Ok((scalar(2*size_of::<Option<BirthFields<S>>>()+2*size_of::<S::Context>()+4*size_of::<S::Row>()+2*size_of::<Option<S::Row>>()+1),1)),
   6=>F::quote(),
   _=>Err(invalid()),
  }
 }
 pub fn advance_reply<'r>(&mut self,payload:RetainedCloneRef<'_,S::Payload>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;let body_birth=if self.body.is_none(){size_of::<usize>()}else{0};
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(payload,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(1);
   match self.phase{
    0=>{p=add(bind(payload,&mut self.source,&mut effect.nested(child(g,1+body_birth)),&mut work)?,1+body_birth)?;self.phase=1;},
    1=>{self.cursor=Some(Box::new(S::Fields::new()));self.phase=2;p=receipt(birth::<S::Fields>());},
    2=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(payload,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(payload,nb,max,&mut effect.nested(child(g,1)),r,true));
     p=add(step?.progress(),1)?;if c.ready(){self.phase=3;}},
    3=>{let extra=2*size_of::<Option<BirthFields<S>>>()+1;let c=self.cursor.as_mut().ok_or_else(invalid)?;let(fields,progress)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.fields.set(fields);c.begin_close();self.phase=4;p=add(progress,extra)?;},
    4=>{let step=custody::finish(&mut self.flight,&mut self.cursor,body,effect,&mut work,recover)?;p=step.progress();if self.cursor.is_none(){self.phase=5;}},
    5=>{let fields=self.fields.take().ok_or_else(invalid)?;let value=S::finish(payload,fields,self.context)?;self.raw.set(value);self.phase=6;p=receipt(scalar(1));},
    6=>{let turn=F::materialize(self.raw.slot(),self.output.slot(),&mut effect.nested(g));work+=turn.work;p=turn.result?.progress();if self.output.is_some(){self.phase=7;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original typed Energy row birth")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.output.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<F::Output>>()+1),1))}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(F::Output,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=8;Ok(Some((self.output.take().ok_or_else(invalid)?,receipt(d))))}
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:RowBirthSpec,F:RowBirthForm<S::Row>] RowBirthPreparation<S,F>,[],|s|s.begin_close());

#[path="🪡️fields/🦀️.rs"]mod fields;pub use fields::{RowFieldSource,SealedRowField,RowFieldsPair,FixedRowFields,CopyRowField,OwnedRowField};

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🧱️create/🦀️.rs"]mod create;pub use create::{RowCreationValidation,CreateRowPath,CreateRowPreparation,NamedCreationRule,NamedRowCreationValidation,FixedCreationRule,FixedRowCreationValidation,CreationRule,CreationRuleSource,RuleRowCreationValidation};

impl<S:RowBirthSpec<Context=()>,F:RowBirthForm<S::Row>> RowBirthPreparation<S,F>{pub fn new()->Self{Self::with_context(())}}
#[path="📦️form/🦀️.rs"]mod form;pub use form::{RowBirthForm,SealedRowForm,OwnedRecordForm,OwnedFixedVectorForm};

/// ↩️ A declared create payload supplies its exact original row reconstruction for semantic deletion inverses.
pub trait RowRecreation:Send+Sync+Sized+'static{type Spec:RowBirthSpec<Row=Self,Context=usize>;}
