//! 🌱️ Typed row construction admits each original field before the unique immutable row birth.
use crate::scalar::{invalid,scalar,nested,child,add,tag,receipt,birth,cursor_demand,close_cursor,CursorClose};
use semio_framework_value::{RetirementDemand,ValueError,retirement::RetireOwned,shared_issuance::{SealedShared,SealedSharedBirth,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use protocol::retained_mutation_demand_fits;
use std::mem::{size_of,ManuallyDrop};
/// 🪡️ Authored schema fields provide their genuine typed preparation and transfer contract.
pub trait RowFieldsPreparation<P:Send+Sync+'static>:CursorClose+Send+'static{
 type Fields:RetireOwned+Send+Sync;
 fn new()->Self;
 fn demand(&self,payload:RetainedCloneRef<'_,P>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,payload:RetainedCloneRef<'_,P>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
}
/// 🧩️ The final constructor only moves prepared sealed fields and copies authored fixed scalars.
pub trait RowBirthSpec:Send+Sync+'static{
 type Payload:Send+Sync+'static;type Row:RetireOwned+Send+Sync;type Context:Copy+Send+Sync+'static;type Fields:RowFieldsPreparation<Self::Payload>;
 fn validate_source(_: &Self::Payload)->Result<(),ValueError>{Ok(())}
 fn validate_context(_:Self::Context)->Result<(),ValueError>{Ok(())}
 fn finish(payload:RetainedCloneRef<'_,Self::Payload>,fields:<Self::Fields as RowFieldsPreparation<Self::Payload>>::Fields,context:Self::Context)->Result<Self::Row,ValueError>;
}
/// 🧳️ Source binding, partial fields, unique result and their original retirement remain explicit owners.
pub struct RowBirthPreparation<S:RowBirthSpec,F:RowBirthForm<S::Row>=SealedRowForm>{source:Option<RetainedCloneBinding>,fields:ManuallyDrop<Option<<S::Fields as RowFieldsPreparation<S::Payload>>::Fields>>,cursor:Option<Box<S::Fields>>,output:ManuallyDrop<Option<F::Output>>,retirement:RetainedCloneClose,body:Option<usize>,context:S::Context,phase:u8,closing:bool}
impl<S:RowBirthSpec,F:RowBirthForm<S::Row>> RowBirthPreparation<S,F>{
 pub fn with_context(context:S::Context)->Self{Self{source:None,fields:ManuallyDrop::new(None),cursor:None,output:ManuallyDrop::new(None),retirement:Default::default(),body:None,context,phase:0,closing:false}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==6}
 pub fn demand(&self,payload:RetainedCloneRef<'_,S::Payload>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=6{return Err(invalid());}payload.check_binding(&self.source)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}S::validate_context(self.context)?;S::validate_source(payload.get())?;match self.phase{0=>nested(scalar(payload.binding_copy_bytes()+if self.body.is_none(){size_of::<usize>()}else{0}),1),1=>Ok(birth::<S::Fields>()),2=>nested(self.cursor.as_ref().ok_or_else(invalid)?.demand(payload,body.saturating_sub(1))?,1),3=>nested(self.cursor.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<<S::Fields as RowFieldsPreparation<S::Payload>>::Fields>>()+1),4=>cursor_demand(&self.cursor,body),5=>{let mut d=F::demand();d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<<S::Fields as RowFieldsPreparation<S::Payload>>::Fields>>()+2*size_of::<S::Context>()).ok_or_else(invalid)?;Ok(d)},_=>Err(invalid())}}
 pub fn advance(&mut self,payload:RetainedCloneRef<'_,S::Payload>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(payload,body)?;let body_birth=if self.body.is_none(){size_of::<usize>()}else{0};if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}self.body=Some(body);let mut p=receipt(d);match self.phase{0=>{p=add(payload.bind(&mut self.source,child(g,1+body_birth))?.ok_or_else(invalid)?,1+body_birth)?;self.phase=1;},1=>{self.cursor=Some(Box::new(S::Fields::new()));self.phase=2;},2=>{let c=self.cursor.as_mut().ok_or_else(invalid)?;p=add(c.advance(payload,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if c.ready(){self.phase=3;}},3=>{let extra=2*size_of::<Option<<S::Fields as RowFieldsPreparation<S::Payload>>::Fields>>()+1;let c=self.cursor.as_mut().ok_or_else(invalid)?;let(fields,progress)=c.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.fields=Some(fields);c.begin_close();self.phase=4;p=add(progress,extra)?;},4=>{p=close_cursor(&mut self.cursor,g)?;if self.cursor.is_none(){self.phase=5;}},5=>{let birth=F::prepare(g)?;let fields=self.fields.take().ok_or_else(invalid)?;let value=S::finish(payload,fields,self.context)?;let(value,progress)=F::materialize(birth,value);*self.output=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes;self.phase=6;},_=>return Err(invalid())}semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original typed Energy row birth")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.output.is_none(){return Err(invalid());}Ok(scalar(2*size_of::<Option<F::Output>>()+1))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(F::Output,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}self.phase=7;Ok(Some((self.output.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(c)=self.cursor.as_mut(){c.begin_close();}}
}
impl<S:RowBirthSpec,F:RowBirthForm<S::Row>> CursorClose for RowBirthPreparation<S,F>{
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}let body=self.body.unwrap_or(body);if self.cursor.is_some(){return cursor_demand(&self.cursor,body);}if !self.retirement.is_empty()||self.fields.is_some(){return self.retirement.next_owner_demands::<<S::Fields as RowFieldsPreparation<S::Payload>>::Fields>(self.fields.is_some(),&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<F::Output>(true,&None,body);}RetainedCloneBinding::close_demand(&self.source,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(self.body.unwrap_or(g.maximum_copy_bytes))?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let g=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,..g};if self.cursor.is_some(){return close_cursor(&mut self.cursor,g).map(RetainedCloneStep::Progress);}if !self.retirement.is_empty(){return self.retirement.select(self.body.unwrap_or(g.maximum_copy_bytes))?.step(g);}if let Some(s)=self.retirement.begin_granted(&mut self.fields,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.output,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}let s=RetainedCloneBinding::close_one(&mut self.source,g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})}
 fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.is_none()&&self.fields.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.source.is_none()}
}
impl<S:RowBirthSpec,F:RowBirthForm<S::Row>> Drop for RowBirthPreparation<S,F>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original row birth retains typed fields or source custody");if empty{unsafe{ManuallyDrop::drop(&mut self.fields);ManuallyDrop::drop(&mut self.output);}}}}

#[path="🪡️fields/🦀️.rs"]mod fields;pub use fields::{RowFieldSource,SealedRowField,RowFieldsPair,FixedRowFields,CopyRowField,OwnedRowField};

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🧱️create/🦀️.rs"]mod create;pub use create::{RowCreationValidation,CreateRowPath,CreateRowPreparation,NamedCreationRule,NamedRowCreationValidation,FixedCreationRule,FixedRowCreationValidation,CreationRule,CreationRuleSource,RuleRowCreationValidation};

impl<S:RowBirthSpec<Context=()>,F:RowBirthForm<S::Row>> RowBirthPreparation<S,F>{pub fn new()->Self{Self::with_context(())}}
#[path="📦️form/🦀️.rs"]mod form;pub use form::{RowBirthForm,SealedRowForm,OwnedRecordForm,OwnedFixedVectorForm};

/// ↩️ A declared create payload supplies its exact original row reconstruction for semantic deletion inverses.
pub trait RowRecreation:Send+Sync+Sized+'static{type Spec:RowBirthSpec<Row=Self,Context=usize>;}
