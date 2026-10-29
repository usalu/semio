//! 🪪️ Original model identity mutations prepare immutable typed deltas without numerical model materialization.
use crate::{snapshot::{original::EnergyModelSnapshot,state::{ModelIdentityState,fields::{RetainedSealedField,RetainedBranchProjection}}},diff::original::EnergyModelDiff};
use protocol::{RetainedFieldPreparation,retained_mutation_demand_fits};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,shared_issuance::SealedShared,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_persistent::RetainedEqualityOwner;
use std::mem::{size_of,ManuallyDrop};

/// 🏷️ Both identity fields share original validation and retain distinct schema addresses.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum IdentityField{Name,Version}
impl IdentityField{
 pub fn text(self,identity:&ModelIdentityState)->&String{match self{Self::Name=>identity.name.get(),Self::Version=>identity.version.get()}}
 fn select(self)->for<'a>fn(&'a mut ModelIdentityState)->&'a mut SealedShared<String>{match self{Self::Name=>|identity|&mut identity.name,Self::Version=>|identity|&mut identity.version}}
 fn source(self,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,String>{match self{Self::Name=>original.project(0,|s|s.model.identity.get().name.get()),Self::Version=>original.project(1,|s|s.model.identity.get().version.get())}}
}
/// 🚦️ Unchanged and refused values yield no branch and no inverse operation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum IdentityOutcome{Changed,Unchanged,Refused}
impl IdentityOutcome{pub fn as_str(self)->&'static str{match self{Self::Changed=>"changed",Self::Unchanged=>"no-op",Self::Refused=>"refused"}}}

/// 🧳️ Original authored text, immutable branch candidate and inverse text remain independently closeable.
pub struct IdentityChangePreparation{
 field:IdentityField,payload:Option<RetainedCloneBinding>,original:Option<RetainedCloneBinding>,offset:usize,phase:u8,outcome:Option<IdentityOutcome>,closing:bool,
 equality:RetainedEqualityOwner<String>,text:RetainedSealedField<String>,branch:RetainedBranchProjection<ModelIdentityState>,
 value:ManuallyDrop<Option<SealedShared<String>>>,displaced:ManuallyDrop<Option<SealedShared<String>>>,decision:ManuallyDrop<Option<EnergyModelDiff>>,
 inverse:Option<Box<RetainedFieldPreparation<String>>>,inverse_value:ManuallyDrop<Option<String>>,inverse_phase:u8,retirement:RetainedCloneClose,
}
impl IdentityChangePreparation{
 pub fn new(field:IdentityField)->Self{Self{field,payload:None,original:None,offset:0,phase:0,outcome:None,closing:false,equality:RetainedEqualityOwner::new(),text:RetainedSealedField::new(),branch:RetainedBranchProjection::new(),value:ManuallyDrop::new(None),displaced:ManuallyDrop::new(None),decision:ManuallyDrop::new(None),inverse:None,inverse_value:ManuallyDrop::new(None),inverse_phase:0,retirement:Default::default()}}
 pub fn decision_ready(&self)->bool{!self.closing&&self.phase==90}
 pub fn outcome(&self)->Option<IdentityOutcome>{self.outcome}
 fn verify(&self,payload:RetainedCloneRef<'_,String>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{payload.check_binding(&self.payload)?;original.check_binding(&self.original)?;Ok(())}
 fn scan_end(&self,text:&str,body:usize)->usize{let fixed=2*size_of::<usize>()+3;let count=(body.saturating_sub(fixed)/2).min(text.len().saturating_sub(self.offset));let mut end=self.offset+count;while end>self.offset&&!text.is_char_boundary(end){end-=1;}end}
 pub fn decision_demand(&self,payload:RetainedCloneRef<'_,String>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=90{return Err(invalid());}self.verify(payload,original)?;match self.phase{
   0=>nested(scalar(payload.binding_copy_bytes()),1),1=>nested(scalar(original.binding_copy_bytes()),1),
   2=>Ok(scalar(2*size_of::<usize>()+3+2*(self.scan_end(payload.get(),body)-self.offset))),
   3=>nested(self.equality.demand(payload,self.field.source(original),body.saturating_sub(1))?,1),
   4=>nested(self.text.demand(payload,body.saturating_sub(1))?,1),
   5=>nested(self.text.take_demand()?,2*size_of::<Option<SealedShared<String>>>()+1),
   6=>nested(self.branch.demand(original.project(2,|s|s.model.identity.get()),body.saturating_sub(1))?,1),
   7=>nested(RetainedBranchProjection::<ModelIdentityState>::exchange_demand::<SealedShared<String>>(),1),
   8=>nested(self.branch.demand(original.project(2,|s|s.model.identity.get()),body.saturating_sub(1))?,1),
   9=>nested(self.branch.take_demand()?,2*size_of::<Option<EnergyModelDiff>>()+size_of::<EnergyModelDiff>()+1),
   _=>Err(invalid()),
  }
 }
 pub fn decide(&mut self,payload:RetainedCloneRef<'_,String>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.decision_demand(payload,original,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut p=receipt(d);match self.phase{
   0=>{p=add(payload.bind(&mut self.payload,child(g,1))?.ok_or_else(invalid)?,1)?;self.phase=1;},
   1=>{p=add(original.bind(&mut self.original,child(g,1))?.ok_or_else(invalid)?,1)?;self.phase=2;},
   2=>{let end=self.scan_end(payload.get(),body);if payload.get()[self.offset..end].chars().any(|c|!c.is_whitespace()){self.phase=3;}else if end==payload.get().len(){self.outcome=Some(IdentityOutcome::Refused);self.phase=90;}self.offset=end;},
   3=>{let step=self.equality.advance(payload,self.field.source(original),child(g,1)).map_err(|e|tag(e,1))?;p=add(step.progress,1)?;if let Some(equal)=step.equal{if equal{self.outcome=Some(IdentityOutcome::Unchanged);self.phase=90;}else{self.phase=4;}}},
   4=>{p=add(self.text.advance(payload,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if self.text.ready(){self.phase=5;}},
   5=>{let extra=2*size_of::<Option<SealedShared<String>>>()+1;let(value,progress)=self.text.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.value=Some(value);p=add(progress,extra)?;self.phase=6;},
   6=>{p=add(self.branch.advance(original.project(2,|s|s.model.identity.get()),body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if self.branch.raw_ready(){self.phase=7;}},
   7=>{let(_,progress)=self.branch.exchange(self.field.select(),&mut self.value,&mut self.displaced,child(g,1)).map_err(|e|tag(e,1))?;self.branch.begin_seal()?;p=add(progress,1)?;self.phase=8;},
   8=>{p=add(self.branch.advance(original.project(2,|s|s.model.identity.get()),body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if self.branch.ready(){self.phase=9;}},
   9=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+size_of::<EnergyModelDiff>()+1;let(identity,progress)=self.branch.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;let mut diff=EnergyModelDiff::default();diff.model.identity=Some(identity);*self.decision=Some(diff);self.outcome=Some(IdentityOutcome::Changed);self.phase=90;p=add(progress,extra)?;},
   _=>return Err(invalid()),
  }semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original Energy identity mutation")?;Ok(if self.decision_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn take_decision_demand(&self)->Result<RetirementDemand,ValueError>{if !self.decision_ready()||self.decision.is_none(){return Err(invalid());}Ok(scalar(2*size_of::<Option<EnergyModelDiff>>()+1))}
 pub fn take_decision(&mut self,g:RetainedCloneGrant)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let d=self.take_decision_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}let value=self.decision.take().ok_or_else(invalid)?;self.phase=91;Ok(Some((value,receipt(d))))}
 pub fn inverse_ready(&self)->bool{!self.closing&&self.inverse_phase==2&&self.inverse_value.is_some()}
 pub fn inverse_demand(&self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=91||self.inverse_phase>=2{return Err(invalid());}original.check_binding(&self.original)?;let source=self.field.source(original);self.equality.verify_original(source)?;if let Some(inverse)=self.inverse.as_ref(){if inverse.value().is_some(){nested(inverse.exchange_demand()?,2*size_of::<Option<String>>()+1)}else{nested(inverse.demand(source,body.saturating_sub(1))?,1)}}else{Ok(RetirementDemand{copy_bytes:2*size_of::<RetainedFieldPreparation<String>>()+2*size_of::<Option<Box<RetainedFieldPreparation<String>>>>(),capacity_bytes:size_of::<RetainedFieldPreparation<String>>(),depth:1,..Default::default()})}}
 pub fn inverse(&mut self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.inverse_demand(original,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.inverse.is_none(){self.inverse=Some(Box::new(Default::default()));return Ok(RetainedCloneStep::Progress(receipt(d)));}let p=if self.inverse.as_ref().ok_or_else(invalid)?.value().is_some(){let extra=2*size_of::<Option<String>>()+1;let(value,p)=self.inverse.as_mut().ok_or_else(invalid)?.take(child(g,extra)).map_err(|e|tag(e,extra))?;*self.inverse_value=value;self.inverse_phase=2;add(p,extra)?}else{add(self.inverse.as_mut().ok_or_else(invalid)?.advance(self.field.source(original),body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?};Ok(if self.inverse_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn take_inverse_demand(&self)->Result<RetirementDemand,ValueError>{if !self.inverse_ready(){return Err(invalid());}Ok(scalar(2*size_of::<Option<String>>()+1))}
 pub fn take_inverse(&mut self,g:RetainedCloneGrant)->Result<Option<(String,RetainedCloneProgress)>,ValueError>{let d=self.take_inverse_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}let value=self.inverse_value.take().ok_or_else(invalid)?;self.inverse_phase=3;Ok(Some((value,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;self.equality.begin_close();self.text.begin_close();self.branch.begin_close();if let Some(inverse)=self.inverse.as_mut(){inverse.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if !self.branch.terminal_is_empty(){return self.branch.close_demand(body);}if !self.text.terminal_is_empty(){return self.text.close_demand(body);}if let Some(inverse)=self.inverse.as_ref(){return if inverse.terminal_is_empty(){Ok(inverse_release())}else{inverse.close_demand(body)};}if !self.equality.terminal_is_empty(){return self.equality.close_demand(body);}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<EnergyModelDiff>(false,&None,body);}if self.decision.is_some(){return self.retirement.next_owner_demands::<EnergyModelDiff>(true,&None,body);}if self.value.is_some()||self.displaced.is_some(){return self.retirement.next_owner_demands::<SealedShared<String>>(true,&None,body);}if self.inverse_value.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body);}RetainedCloneBinding::close_demand(if self.original.is_some(){&self.original}else{&self.payload},body)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.branch.terminal_is_empty(){return self.branch.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.text.terminal_is_empty(){return self.text.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(inverse)=self.inverse.as_mut(){if inverse.terminal_is_empty(){drop(self.inverse.take());return Ok(RetainedCloneStep::Progress(receipt(d)));}return inverse.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.equality.terminal_is_empty(){return self.equality.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.retirement.is_empty(){return self.retirement.step_granted(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.decision,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.value,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.displaced,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.inverse_value,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}let s=RetainedCloneBinding::close_one(if self.original.is_some(){&mut self.original}else{&mut self.payload},g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.branch.terminal_is_empty()&&self.text.terminal_is_empty()&&self.inverse.is_none()&&self.equality.terminal_is_empty()&&self.decision.is_none()&&self.value.is_none()&&self.displaced.is_none()&&self.inverse_value.is_none()&&self.retirement.is_empty()&&self.payload.is_none()&&self.original.is_none()}
}
impl Drop for IdentityChangePreparation{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original identity mutation retains original prepared source custody");if empty{unsafe{ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.displaced);ManuallyDrop::drop(&mut self.decision);ManuallyDrop::drop(&mut self.inverse_value);}}}}
fn inverse_release()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Box<RetainedFieldPreparation<String>>>>(),release_bytes:size_of::<RetainedFieldPreparation<String>>(),depth:1,..Default::default()}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Energy identity mutation lost its source or native phase")}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g}}
fn add(mut p:RetainedCloneProgress,extra:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(extra).ok_or_else(invalid)?;Ok(p)}
fn tag(error:ValueError,extra:usize)->ValueError{let p=error.retained_progress();match add(p,extra){Ok(p)=>error.with_retained_progress(p),Err(failure)=>failure.with_retained_progress(p)}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🪆️leaf/🦀️.rs"]mod leaf;
pub use leaf::{IdentityPayload,IdentityLeafPreparation};
