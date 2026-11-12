//! 🪪️ Original model identity mutations prepare immutable typed deltas without numerical model materialization.
use crate::{snapshot::{original::EnergyModelSnapshot,state::{ModelIdentityState,fields::{RetainedSealedField,RetainedBranchProjection}}},diff::original::EnergyModelDiff};
use protocol::{RetainedFieldPreparation,retained_mutation_demand_fits};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,ValueError,shared_issuance::SealedShared,retained_clone::{RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use semio_framework_value_persistent::RetainedEqualityOwner;
use semio_s_artifact_energy_model_native_mutations::{custody::{self,Cursor,Held,Bound},scalar::{Quote,invalid,scalar,nested,child,add,receipt,birth,bind},retained::{Done,Flight,Reply,conclude,native_child},row::{Staged,staged_step,RowOutcome},leaf::RowFieldPreparation};
use std::mem::size_of;

/// 🏷️ Both identity fields share original validation and retain distinct schema addresses.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum IdentityField{Name,Version}
impl IdentityField{
 pub fn text(self,identity:&ModelIdentityState)->&String{match self{Self::Name=>identity.name.get(),Self::Version=>identity.version.get()}}
 fn select(self)->for<'a>fn(&'a mut ModelIdentityState)->&'a mut SealedShared<String>{match self{Self::Name=>|identity|&mut identity.name,Self::Version=>|identity|&mut identity.version}}
 fn source(self,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,String>{match self{Self::Name=>original.project(0,|s|s.model.identity.get().name.get()),Self::Version=>original.project(1,|s|s.model.identity.get().version.get())}}
}
/// 🧳️ Original authored text, immutable branch candidate and inverse text remain independently closeable.
pub struct IdentityChangePreparation<L:IdentityPayload<M>,M:Send+'static>{
 payload:Bound,original:Bound,offset:usize,phase:u8,outcome:Option<RowOutcome>,closing:bool,body:Option<usize>,
 equality:Option<Box<RetainedEqualityOwner<String>>>,text:RetainedSealedField<String>,staged:Staged<ModelIdentityState>,inverse_stage:Option<Box<RetainedFieldPreparation<String>>>,
 value:Held<SealedShared<String>>,displaced:Held<SealedShared<String>>,decision:Held<EnergyModelDiff>,inverse_value:Held<String>,flight:Flight,marker:std::marker::PhantomData<fn()->(L,M)>,
}
impl<L:IdentityPayload<M>,M:Send+'static> IdentityChangePreparation<L,M>{
 fn verify(&self,payload:RetainedCloneRef<'_,String>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{payload.check_binding(&self.payload.0)?;original.check_binding(&self.original.0)?;Ok(())}
 fn scan_end(&self,text:&str,body:usize)->usize{let fixed=2*size_of::<usize>()+3;let count=(body.saturating_sub(fixed)/2).min(text.len().saturating_sub(self.offset));let mut end=self.offset+count;while end>self.offset&&!text.is_char_boundary(end){end-=1;}end}
 fn identity(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,ModelIdentityState>{original.project(2,|s|s.model.identity.get())}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(inverse_stage,staged,text,equality,decision,value,displaced,inverse_value,original,payload);
 fn identity_quote(&self,payload:RetainedCloneRef<'_,String>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=90||self.body.is_some_and(|b|b!=body)||!self.flight.idle(){return Err(invalid());}self.verify(payload,original)?;let nb=body.saturating_sub(1);let source=L::FIELD.source(original);
  match self.phase{
   0=>Ok((nested(scalar(payload.binding_copy_bytes()),1)?,payload.binding_work_demand(&self.payload.0)?)),
   1=>Ok((nested(scalar(original.binding_copy_bytes()),1)?,original.binding_work_demand(&self.original.0)?)),
   2=>Ok((scalar(2*size_of::<usize>()+3+2*(self.scan_end(payload.get(),body)-self.offset)),1)),
   3=>Ok((birth::<RetainedEqualityOwner<String>>(),1)),
   4=>{let e=self.equality.as_ref().ok_or_else(invalid)?;Ok((nested(e.demand(payload,source,nb,max)?,1)?,e.work_demand(payload,source,nb,max)?))},
   5=>Ok((Cursor::demand(&self.equality,body)?,Cursor::work(&self.equality,body)?)),
   6=>Ok((birth::<RetainedFieldPreparation<String>>(),1)),
   7=>{let s=self.inverse_stage.as_ref().ok_or_else(invalid)?;Ok((nested(s.demand(source,nb,max)?,1)?,s.advance_work_demand(source,nb,max)?))},
   8=>Ok((nested(self.inverse_stage.as_ref().ok_or_else(invalid)?.exchange_demand()?,2*size_of::<Option<String>>()+1)?,1)),
   9=>Ok((Cursor::demand(&self.inverse_stage,body)?,Cursor::work(&self.inverse_stage,body)?)),
   10=>Ok((nested(self.text.demand(payload,nb,max)?,1)?,self.text.advance_work_demand(payload,nb,max)?)),
   11=>Ok((nested(self.text.take_demand()?,2*size_of::<Option<SealedShared<String>>>()+1)?,1)),
   12=>self.staged.quote::<SealedShared<String>>(Self::identity(original),body,max),
   13=>Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+size_of::<EnergyModelDiff>()+1),1)),
   _=>Err(invalid()),
  }
 }
 fn identity_advance<'r>(&mut self,payload:RetainedCloneRef<'_,String>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.identity_quote(payload,original,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(1);let source=L::FIELD.source(original);
   match self.phase{
    0=>{p=add(bind(payload,&mut self.payload,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=1;},
    1=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{p=receipt(scalar(1));let end=self.scan_end(payload.get(),body);if payload.get()[self.offset..end].chars().any(|c|!c.is_whitespace()){self.phase=3;}else if end==payload.get().len(){self.outcome=Some(RowOutcome::Invalid);self.phase=90;}self.offset=end;},
    3=>{self.equality=Some(Box::new(RetainedEqualityOwner::new()));self.phase=4;p=receipt(birth::<RetainedEqualityOwner<String>>());},
    4=>{let e=self.equality.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,2,body,max,None,work,|r| e.advance_into(payload,source,nb,max,&mut effect.nested(child(g,1)),r),e.recover_advance_into(payload,source,nb,max,&mut effect.nested(child(g,1)),r));let step=step?;p=add(step.progress,1)?;if let Some(equal)=step.equal{if equal{self.outcome=Some(RowOutcome::Unchanged);}self.phase=5;}},
    5=>{let step=custody::finish(&mut self.flight,&mut self.equality,body,effect,&mut work,recover)?;p=step.progress();if self.equality.is_none(){self.phase=if self.outcome==Some(RowOutcome::Unchanged){90}else{6};}},
    6=>{self.inverse_stage=Some(Box::default());self.phase=7;p=receipt(birth::<RetainedFieldPreparation<String>>());},
    7=>{let s=self.inverse_stage.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| s.advance_into(source,nb,max,&mut effect.nested(child(g,1)),r),s.recover_advance_into(source,nb,max,&mut effect.nested(child(g,1)),r));p=add(step?.progress(),1)?;if s.value().is_some(){self.phase=8;}},
    8=>{let extra=2*size_of::<Option<String>>()+1;let Self{inverse_stage,inverse_value,..}=self;let s=inverse_stage.as_deref_mut().ok_or_else(invalid)?;let slot=inverse_value.slot();let step=native_child(effect,&mut work,|e,r|s.take_into(slot,&mut e.nested(child(g,extra)),r))??;p=add(step.progress(),extra)?;self.phase=9;},
    9=>{let step=custody::finish(&mut self.flight,&mut self.inverse_stage,body,effect,&mut work,recover)?;p=step.progress();if self.inverse_stage.is_none(){self.phase=10;}},
    10=>{let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| self.text.advance_reply(payload,nb,max,&mut effect.nested(child(g,1)),r,false),self.text.advance_reply(payload,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if self.text.ready(){self.phase=11;}},
    11=>{let extra=2*size_of::<Option<SealedShared<String>>>()+1;let(value,progress)=self.text.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.value.set(value);p=add(progress,extra)?;self.phase=12;},
    12=>{let Self{staged,value,displaced,..}=self;let select=L::FIELD.select();let mut exchange=|stage:&mut RetainedBranchProjection<ModelIdentityState>,e:&mut RetainedAllocationEffect<'_>|stage.exchange(select,value.slot(),displaced.slot(),e);
     let step=staged_step::<_,SealedShared<String>>(staged,Self::identity(original),body,max,effect,&mut work,recover,&mut exchange,&mut units)?;p=step.progress();if staged.ready(){self.phase=13;}},
    13=>{let identity=self.staged.sealed.take().ok_or_else(invalid)?;let mut diff=EnergyModelDiff::default();diff.model.identity=Some(identity);self.decision.set(diff);self.outcome=Some(RowOutcome::Changed);self.phase=90;p=receipt(scalar(1));},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy identity mutation")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==90{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([L:IdentityPayload<M>,M:Send+'static] IdentityChangePreparation<L,M>,[],|s|s.begin_close());
impl<L:IdentityPayload<M>,M:Send+'static> RowFieldPreparation for IdentityChangePreparation<L,M>{
 type Input=String;type Inverse=String;
 fn new()->Self{Self{payload:Default::default(),original:Default::default(),offset:0,phase:0,outcome:None,closing:false,body:None,equality:None,text:RetainedSealedField::new(),staged:Default::default(),inverse_stage:None,value:Default::default(),displaced:Default::default(),decision:Default::default(),inverse_value:Default::default(),flight:Default::default(),marker:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,String>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if key.is_some(){return Err(invalid());}self.verify(p,o)}
 fn quote(&self,p:RetainedCloneRef<'_,String>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{self.identity_quote(p,o,b,m)}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,String>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.identity_advance(p,o,b,m,e,r,recover)}
 fn ready(&self)->bool{!self.closing&&self.phase==90}fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=91;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_quote(&self,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=91||self.inverse_value.is_none(){return Err(invalid());}Ok((scalar(1),1))}
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==91&&self.inverse_value.is_some()}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !self.inverse_ready(){return Err(invalid());}Ok((scalar(2*size_of::<Option<String>>()+1),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(String,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}let value=self.inverse_value.take().ok_or_else(invalid)?;self.phase=92;Ok(Some((value,receipt(d))))}
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🪆️leaf/🦀️.rs"]mod leaf;
pub use leaf::{IdentityPayload,IdentityLeafPreparation};
