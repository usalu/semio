//! 🧬️ Owned row fields share original validation, bounded equality, field preparation and sparse path publication.
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::{EntityRow,RetainedEntityLookup,RetainedSealedField,RetainedBranchProjection}},diff::original::EnergyModelDiff,row::{RowBranchPath,RowTail,tail_step,BranchOutput,RowOutcome},leaf::RowFieldPreparation,custody::{self,Cursor,Held,Bound},scalar::{Quote,birth,invalid,scalar,nested,child,add,receipt,bind}};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,ValueError,RetainedOwnedProjection,shared_issuance::SealedShared,retained_clone::{RetainedClone,RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use semio_framework_value_persistent::{RetainedEquality,RetainedEqualityOwner,list::persistent::PersistentList};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use protocol::{RetainedFieldPreparation,retained_mutation_demand_fits};
use crate::retained::{Done,Flight,Reply,conclude,native_child};
use std::mem::size_of;
/// 🧭️ Authored field policies supply exact original value storage, collection and native validation owner.
pub trait OwnedRowPath:Send+Sync+'static{
 type Value:RetainedClone+RetainedEquality;type Row:EntityRow+SnapshotProjection<Output=Box<Self::Row>>;type Branch:SnapshotProjection<Output=Box<Self::Branch>>+Send+Sync;type Output:BranchOutput<Self::Branch>;type Validation:OwnedRowValidation<Self::Value,Self::Row>;
 fn rows(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>;
 fn branch(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>;
 fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>;
 fn read(row:&Self::Row)->&SealedShared<Self::Value>;fn select(row:&mut Self::Row)->&mut SealedShared<Self::Value>;
 fn output()->Self::Output;fn validation()->Self::Validation;
}
/// 🚦️ Validation inspects original authored fields and genuine target rows before candidate birth.
pub trait OwnedRowValidation<V:Send+Sync+'static,R:EntityRow>:Cursor+Send+'static{
 fn quote(&self,input:RetainedCloneRef<'_,V>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,row:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>;
 fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,V>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,row:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
 fn result(&self)->Option<Option<RowOutcome>>;
 fn begin_close(&mut self);
}
pub struct OwnedBranch<S>(std::marker::PhantomData<fn()->S>);
impl<S:OwnedRowPath> RowBranchPath for OwnedBranch<S>{type Row=S::Row;type Branch=S::Branch;type Output=S::Output;fn rows(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{S::rows(o)}fn branch(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{S::branch(o)}fn rows_mut(b:&mut Self::Branch)->&mut PersistentList<Self::Row>{S::rows_mut(b)}fn output()->Self::Output{S::output()}}
/// 🧳️ Original row and authored text stay bound while each prepared child has its own funded close.
pub struct OwnedRowPreparation<S:OwnedRowPath>{
 original:Bound,key:Bound,authored:Bound,
 lookup:Option<Box<RetainedEntityLookup<S::Row>>>,row:Option<Box<RetainedOwnedProjection<S::Row>>>,validation:Option<Box<S::Validation>>,equality:Option<Box<RetainedEqualityOwner<S::Value>>>,text:Option<Box<RetainedSealedField<S::Value>>>,tail:Option<Box<RowTail<OwnedBranch<S>>>>,inverse_stage:Option<Box<RetainedFieldPreparation<S::Value>>>,
 value:Held<SealedShared<S::Value>>,displaced:Held<SealedShared<S::Value>>,inverse_value:Held<S::Value>,
 rank:Option<usize>,phase:u8,inverse_phase:u8,outcome:Option<RowOutcome>,closing:bool,body:Option<usize>,flight:Flight,
}
impl<S:OwnedRowPath> OwnedRowPreparation<S>{
 pub fn new()->Self{Self{original:Default::default(),key:Default::default(),authored:Default::default(),lookup:None,row:None,validation:None,equality:None,text:None,tail:None,inverse_stage:None,value:Default::default(),displaced:Default::default(),inverse_value:Default::default(),rank:None,phase:0,inverse_phase:0,outcome:None,closing:false,body:None,flight:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==99}
 pub fn outcome(&self)->Option<RowOutcome>{self.outcome}
 pub(crate) fn check(&self,input:RetainedCloneRef<'_,S::Value>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{input.check_binding(&self.authored.0)?;key.check_binding(&self.key.0)?;original.check_binding(&self.original.0)?;Ok(())}
 fn old_of(row:&Option<Box<RetainedOwnedProjection<S::Row>>>)->Result<RetainedCloneRef<'_,S::Value>,ValueError>{Ok(row.as_ref().ok_or_else(invalid)?.borrow()?.project(0,|r|S::read(r).get()))}
 pub fn quote(&self,input:RetainedCloneRef<'_,S::Value>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||!self.flight.idle(){return Err(invalid());}self.check(input,key,original)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}
  let lookup=||self.lookup.as_ref().ok_or_else(invalid);let nb=body.saturating_sub(1);
  match self.phase{
   0=>Ok((nested(scalar(original.binding_copy_bytes()+if self.body.is_none(){size_of::<usize>()}else{0}),1)?,original.binding_work_demand(&self.original.0)?)),
   1=>Ok((nested(scalar(key.binding_copy_bytes()),1)?,key.binding_work_demand(&self.key.0)?)),
   2=>Ok((nested(scalar(input.binding_copy_bytes()),1)?,input.binding_work_demand(&self.authored.0)?)),
   3=>Ok((birth::<RetainedEntityLookup<S::Row>>(),1)),
   4=>Ok((nested(lookup()?.demand(S::rows(original),key)?,1)?,lookup()?.advance_work_demand(S::rows(original),key)?)),
   5=>if lookup()?.result()==Some(None){Ok((scalar(2),1))}else{Ok((nested(lookup()?.take_demand()?,2*size_of::<Option<RetainedOwnedProjection<S::Row>>>()+2*size_of::<Option<usize>>()+1)?,1))},
   6=>Ok((Cursor::demand(&self.lookup,body)?,Cursor::work(&self.lookup,body)?)),
   7=>Ok((birth::<S::Validation>(),1)),
   8=>{let(d,w)=self.validation.as_ref().ok_or_else(invalid)?.quote(input,key,original,self.row.as_ref().ok_or_else(invalid)?.borrow()?,nb,max)?;Ok((nested(d,1)?,w))},
   9=>Ok((Cursor::demand(&self.validation,body)?,Cursor::work(&self.validation,body)?)),
   11=>Ok((birth::<RetainedEqualityOwner<S::Value>>(),1)),
   12=>{let e=self.equality.as_ref().ok_or_else(invalid)?;let old=Self::old_of(&self.row)?;Ok((nested(e.demand(input,old,nb,max)?,1)?,e.work_demand(input,old,nb,max)?))},
   13=>Ok((Cursor::demand(&self.equality,body)?,Cursor::work(&self.equality,body)?)),
   14=>Ok((birth::<RetainedSealedField<S::Value>>(),1)),
   15=>{let t=self.text.as_ref().ok_or_else(invalid)?;Ok((nested(t.demand(input,nb,max)?,1)?,t.advance_work_demand(input,nb,max)?))},
   16=>Ok((nested(self.text.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<SealedShared<S::Value>>>()+1)?,1)),
   17=>Ok((Cursor::demand(&self.text,body)?,Cursor::work(&self.text,body)?)),
   18=>Ok((scalar(1),1)),
   30=>self.tail.as_ref().ok_or_else(invalid)?.quote::<SealedShared<S::Value>>(self.row.as_ref().ok_or_else(invalid)?.borrow()?,original,body,max),
   _=>Err(invalid()),
  }
 }
 pub fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,S::Value>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;let body_birth=if self.body.is_none(){size_of::<usize>()}else{0};
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(input,key,original,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(1);
   match self.phase{
    0=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1+body_birth)),&mut work)?,1+body_birth)?;self.phase=1;},
    1=>{p=add(bind(key,&mut self.key,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{p=add(bind(input,&mut self.authored,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=3;},
    3=>{self.lookup=Some(Box::new(RetainedEntityLookup::new()));self.phase=4;p=receipt(birth::<RetainedEntityLookup<S::Row>>());},
    4=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;p=add(lookup.advance(S::rows(original),key,&mut effect.nested(child(g,1)))?.progress(),1)?;if lookup.result().is_some(){self.phase=5;}},
    5=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;if lookup.result()==Some(None){self.outcome=Some(RowOutcome::Missing);self.phase=99;p=receipt(scalar(2));}else{let extra=2*size_of::<Option<RetainedOwnedProjection<S::Row>>>()+2*size_of::<Option<usize>>()+1;let(row,progress)=lookup.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.rank=lookup.result().ok_or_else(invalid)?;self.row=Some(Box::new(row));lookup.begin_close();self.phase=6;p=add(progress,extra)?;}},
    6=>{let step=custody::finish(&mut self.flight,&mut self.lookup,body,effect,&mut work,recover)?;p=step.progress();if self.lookup.is_none(){self.phase=7;}},
    7=>{self.validation=Some(Box::new(S::validation()));self.phase=8;p=receipt(birth::<S::Validation>());},
    8=>{let row=self.row.as_ref().ok_or_else(invalid)?.borrow()?;let validation=self.validation.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| validation.advance_reply(input,key,original,row,nb,max,&mut effect.nested(child(g,1)),r,false),validation.advance_reply(input,key,original,row,nb,max,&mut effect.nested(child(g,1)),r,true));
     p=add(step?.progress(),1)?;if let Some(outcome)=validation.result(){self.outcome=outcome;validation.begin_close();self.phase=9;}},
    9=>{let step=custody::finish(&mut self.flight,&mut self.validation,body,effect,&mut work,recover)?;p=step.progress();if self.validation.is_none(){self.phase=if self.outcome.is_some(){99}else{11};}},
    11=>{self.equality=Some(Box::new(RetainedEqualityOwner::new()));self.phase=12;p=receipt(birth::<RetainedEqualityOwner<S::Value>>());},
    12=>{let old=Self::old_of(&self.row)?;let equality=self.equality.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,2,body,max,None,work,|r| equality.advance_into(input,old,nb,max,&mut effect.nested(child(g,1)),r),equality.recover_advance_into(input,old,nb,max,&mut effect.nested(child(g,1)),r));
     let step=step?;p=add(step.progress,1)?;if let Some(equal)=step.equal{if equal{self.outcome=Some(RowOutcome::Unchanged);}self.phase=13;}},
    13=>{let step=custody::finish(&mut self.flight,&mut self.equality,body,effect,&mut work,recover)?;p=step.progress();if self.equality.is_none(){self.phase=if self.outcome==Some(RowOutcome::Unchanged){99}else{14};}},
    14=>{self.text=Some(Box::new(RetainedSealedField::new()));self.phase=15;p=receipt(birth::<RetainedSealedField<S::Value>>());},
    15=>{let text=self.text.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,3,body,max,None,work,|r| text.advance_reply(input,nb,max,&mut effect.nested(child(g,1)),r,false),text.advance_reply(input,nb,max,&mut effect.nested(child(g,1)),r,true));
     p=add(step?.progress(),1)?;if text.ready(){self.phase=16;}},
    16=>{let extra=2*size_of::<Option<SealedShared<S::Value>>>()+1;let text=self.text.as_mut().ok_or_else(invalid)?;let(value,progress)=text.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.value.set(value);text.begin_close();self.phase=17;p=add(progress,extra)?;},
    17=>{let step=custody::finish(&mut self.flight,&mut self.text,body,effect,&mut work,recover)?;p=step.progress();if self.text.is_none(){self.phase=18;}},
    18=>{self.tail=Some(Box::new(RowTail::new(self.rank.ok_or_else(invalid)?)));self.phase=30;p=receipt(scalar(1));},
    30=>{let Self{tail,row,value,displaced,..}=self;let tail=tail.as_mut().ok_or_else(invalid)?;let row=row.as_ref().ok_or_else(invalid)?.borrow()?;
     let mut exchange=|stage:&mut RetainedBranchProjection<S::Row>,e:&mut RetainedAllocationEffect<'_>|stage.exchange(S::select,value.slot(),displaced.slot(),e);
     let step=tail_step::<OwnedBranch<S>,SealedShared<S::Value>>(tail,row,original,body,max,effect,&mut work,recover,&mut exchange)?;p=step.progress();if tail.ready(){self.phase=99;self.outcome=Some(RowOutcome::Changed);}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy row name preparation")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.tail.is_none(){return Err(invalid());}self.tail.as_ref().ok_or_else(invalid)?.take_quote()}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{self.take_quote()?;let taken=self.tail.as_mut().ok_or_else(invalid)?.take(effect)?;if taken.is_some(){self.phase=100;}Ok(taken)}
 pub fn inverse_ready(&self)->bool{!self.closing&&self.inverse_phase==2&&self.inverse_value.is_some()}
 pub fn inverse_quote(&self,body:usize,max:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=100||self.inverse_phase>=2||!self.flight.idle(){return Err(invalid());}let nb=body.saturating_sub(1);if let Some(stage)=self.inverse_stage.as_ref(){if stage.value().is_some(){Ok((nested(stage.exchange_demand()?,2*size_of::<Option<S::Value>>()+1)?,1))}else{let old=Self::old_of(&self.row)?;Ok((nested(stage.demand(old,nb,max)?,1)?,stage.advance_work_demand(old,nb,max)?))}}else{Ok((birth::<RetainedFieldPreparation<S::Value>>(),1))}}
 pub fn inverse_reply<'r>(&mut self,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.inverse_quote(body,max))};let g=effect.grant();let nb=body.saturating_sub(1);
  custody::gated(effect,reply,quote,max,|effect,work|{
   let p;
   if self.inverse_stage.is_none(){self.inverse_stage=Some(Box::new(Default::default()));p=receipt(birth::<RetainedFieldPreparation<S::Value>>());}
   else if self.inverse_stage.as_ref().ok_or_else(invalid)?.value().is_some(){let extra=2*size_of::<Option<S::Value>>()+1;let stage=self.inverse_stage.as_mut().ok_or_else(invalid)?;let slot=self.inverse_value.slot();let step=native_child(effect,work,|e,r|stage.take_into(slot,&mut e.nested(child(g,extra)),r))??;self.inverse_phase=2;p=add(step.progress(),extra)?;}
   else{let old=Self::old_of(&self.row)?;let stage=self.inverse_stage.as_mut().ok_or_else(invalid)?;
    let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,*work,|r| stage.advance_into(old,nb,max,&mut effect.nested(child(g,1)),r),stage.recover_advance_into(old,nb,max,&mut effect.nested(child(g,1)),r));
    p=add(step?.progress(),1)?;}
   Ok(if self.inverse_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 pub fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !self.inverse_ready(){return Err(invalid());}Ok((scalar(2*size_of::<Option<S::Value>>()+1),1))}
 pub fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(S::Value,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}self.inverse_phase=3;Ok(Some((self.inverse_value.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(inverse_stage,tail,text,equality,validation,lookup,row,value,displaced,inverse_value,original,authored,key);
}
crate::custody::std_cursor!([S:OwnedRowPath] OwnedRowPreparation<S>,[],|s|s.begin_close());

impl<S:OwnedRowPath> RowFieldPreparation for OwnedRowPreparation<S>{
 type Input=S::Value;type Inverse=S::Value;
 fn new()->Self{Self::new()}
 fn check(&self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{Self::check(self,v,k.ok_or_else(invalid)?,o)}
 fn quote(&self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{Self::quote(self,v,k.ok_or_else(invalid)?,o,b,m)}
 fn advance_reply<'r>(&mut self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{Self::advance_reply(self,v,k.ok_or_else(invalid)?,o,b,m,e,r,recover)}
 fn ready(&self)->bool{Self::ready(self)}fn outcome(&self)->Option<RowOutcome>{Self::outcome(self)}
 fn take_quote(&self)->Result<Quote,ValueError>{Self::take_quote(self)}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{Self::take(self,e)}
 fn inverse_quote(&self,b:usize,m:u64)->Result<Quote,ValueError>{Self::inverse_quote(self,b,m)}
 fn prepare_inverse_reply<'r>(&mut self,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{Self::inverse_reply(self,b,m,e,r,recover)}
 fn inverse_ready(&self)->bool{Self::inverse_ready(self)}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{Self::take_inverse_quote(self)}
 fn take_inverse(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{Self::take_inverse(self,e)}
}

#[path="✅️rule/🦀️.rs"]mod rule;pub use rule::{OriginalValueRule,ValueValidation};

#[path="🪆️leaf/🦀️.rs"]mod leaf;pub use leaf::{OwnedLeafSpec,OwnedLeafPreparation};
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🔀️permutation/🦀️.rs"]mod permutation;pub use permutation::{OriginalPermutationPath,PermutationValidation};

#[path="🌐️plane/🦀️.rs"]mod plane;pub use plane::{OriginalFenestrationPlane,FenestrationPlaneValidation};
