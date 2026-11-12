//! 🪡️ Native authored fields become sealed immutable values through original retained preparation.
use protocol::{RetainedFieldPreparation,retained_mutation_demand_fits,retained::{Done,Flight,Reply,conclude,native_child}};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedClone,RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{size_of,ManuallyDrop};

/// ♻️ Generates the four reply-shaped close entries over fresh or recovered invocations with ordinary or original-loan control.
macro_rules! close_entries{()=>{
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(None,body,effect,reply,false)}
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(None,body,effect,reply,true)}
 pub fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
 pub fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}
}}
/// 🧳️ The actual authored source, prepared field and unpublished sealed value close independently after every prefix.
pub struct RetainedSealedField<T:RetainedClone+Send+Sync>{binding:Option<RetainedCloneBinding>,preparation:Option<Box<RetainedFieldPreparation<T>>>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<SealedShared<T>>>,retirement:RetainedCloneClose,flight:Flight,phase:u8,closing:bool}
impl<T:RetainedClone+Send+Sync> Default for RetainedSealedField<T>{fn default()->Self{Self::new()}}
impl<T:RetainedClone+Send+Sync> RetainedSealedField<T>{
 pub fn new()->Self{Self{binding:None,preparation:None,raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),flight:Default::default(),phase:0,closing:false}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==6&&self.output.is_some()}
 fn stage(&self)->u8{if self.phase==2&&self.preparation.as_ref().is_some_and(|p|p.value().is_some()){3}else{self.phase}}
 fn prepared(&self)->Result<&RetainedFieldPreparation<T>,ValueError>{self.preparation.as_deref().ok_or_else(invalid)}
 fn prepared_mut(&mut self)->Result<&mut RetainedFieldPreparation<T>,ValueError>{self.preparation.as_deref_mut().ok_or_else(invalid)}
 pub fn demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=6||!self.flight.idle(){return Err(invalid());}source.check_binding(&self.binding)?;
  match self.stage(){
   0=>nested(source.binding_demand(&self.binding)?.ok_or_else(invalid)?,1),
   1=>Ok(RetirementDemand{copy_bytes:2*size_of::<RetainedFieldPreparation<T>>()+2*size_of::<Option<Box<RetainedFieldPreparation<T>>>>()+1,capacity_bytes:size_of::<RetainedFieldPreparation<T>>(),depth:1,..Default::default()}),
   2=>nested(self.prepared()?.demand(source,body.saturating_sub(1),maximum_work_units)?,1),
   3=>nested(self.prepared()?.exchange_demand()?,2*size_of::<Option<T>>()+1),
   5=>{let mut d=SealedShared::<T>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<T>>()+1).ok_or_else(invalid)?;Ok(d)},
   _=>Err(invalid()),
  }
 }
 pub fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
  self.demand(source,body,maximum_work_units)?;
  match self.stage(){0=>source.binding_work_demand(&self.binding),1|3=>Ok(1),2=>self.prepared()?.advance_work_demand(source,body.saturating_sub(1),maximum_work_units),5=>Ok(SealedShared::<T>::source_admission_work_demand()),_=>Err(invalid())}
 }
 pub fn advance_reply<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut demand=RetirementDemand::default();
   if recover{self.flight.route().ok_or_else(invalid)?;}else{
    if !self.flight.idle(){return Err(invalid().into())}
    demand=self.demand(source,body,maximum_work_units)?;let units=self.advance_work_demand(source,body,maximum_work_units)?;
    if maximum_work_units==0||units==0||effect.remaining_work_units()?<units||!retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    match self.stage(){
     0=>{let turn=source.bind(&mut self.binding,effect);work+=turn.work;let progress=turn.result?.ok_or_else(invalid)?;self.phase=1;return Ok(RetainedCloneStep::Progress(add(progress,1)?))},
     1=>{work=1;self.preparation=Some(Box::new(RetainedFieldPreparation::default()));self.phase=2;return Ok(RetainedCloneStep::Progress(receipt(demand)))},
     3=>{let prepared=self.preparation.as_deref_mut().ok_or_else(invalid)?;let raw=&mut *self.raw;let step=native_child(effect,&mut work,|e,r|prepared.take_into(raw,e,r))??;self.phase=5;return Ok(RetainedCloneStep::Progress(add(step.progress(),2*size_of::<Option<T>>()+1)?))},
     5=>{let turn=SealedShared::admit_from(&mut self.raw,SharedIssuer::owned(),&mut self.output,effect);work+=turn.work;let step=turn.result?;if self.output.is_none(){return Ok(RetainedCloneStep::Progress(step.progress()))}self.phase=6;return Ok(RetainedCloneStep::Complete(add(step.progress(),2*size_of::<Option<T>>()+1)?))},
     _=>{}
    }
   }
   let nested_body=body.saturating_sub(1);let before=work;
   let step=protocol::retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| self.prepared_mut()?.advance_into(source,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r),self.prepared_mut()?.recover_advance_into(source,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r));
   let step=step.map_err(|error|tagged(error,work!=before))?;
   Ok(RetainedCloneStep::Progress(add(step.progress(),1)?))
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(RetirementDemand{copy_bytes:2*size_of::<Option<SealedShared<T>>>()+1,depth:1,..Default::default()})}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(SealedShared<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Energy authored field take exceeded its action"));}let value=self.output.take().ok_or_else(invalid)?;self.phase=7;Ok(Some((value,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if let Some(p)=self.preparation.as_deref(){return if !p.close_started(){nested(p.begin_close_demand(),1)}else if p.terminal_is_empty(){Ok(preparation_release::<T>())}else{nested(p.close_demand(body.saturating_sub(1))?,1)};}
  if !self.retirement.is_empty()||self.raw.is_some(){return self.retirement.next_owner_demands::<T>(self.raw.is_some(),&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body);}RetainedCloneBinding::close_demand(&self.binding,body)
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demand(body)?;
  if let Some(p)=self.preparation.as_deref(){return if !p.close_started(){Ok(p.begin_close_work_demand())}else if p.terminal_is_empty(){Ok(1)}else{p.close_work_demand(body.saturating_sub(1))};}
  if !self.retirement.is_empty()||self.raw.is_some(){return self.retirement.next_owner_work::<T>(self.raw.is_some(),&None,body);}if self.output.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body);}RetainedCloneBinding::close_work_demand(&self.binding,body)
 }
 pub fn requires_original_birth(&self)->bool{
  if !self.closing||!self.flight.idle(){return false}
  if let Some(p)=self.preparation.as_deref(){return !p.terminal_is_empty()&&p.close_started()&&p.requires_original_birth()}
  if !self.retirement.is_empty(){return self.retirement.requires_original_birth()}
  if self.raw.is_some()||self.output.is_some(){return false}RetainedCloneBinding::requires_original_birth(&self.binding)
 }
 pub fn requires_original_control(&self)->bool{
  if let Some(loaned)=self.flight.loaned(){return loaned}
  if let Some(p)=self.preparation.as_deref(){return !p.terminal_is_empty()&&p.close_started()&&p.requires_original_control()}
  if !self.retirement.is_empty(){return self.retirement.requires_original_control()}
  if self.raw.is_some()||self.output.is_some(){return false}RetainedCloneBinding::requires_original_control(&self.binding)
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if let Some(p)=self.preparation.as_deref_mut(){return p.close_original_birth_step(original,grant)}
  if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,grant)}
  RetainedCloneBinding::close_original_birth_step(&mut self.binding,original,grant)
 }
 close_entries!();
 pub fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut route:u8=0;
   if recover{route=self.flight.route().ok_or_else(invalid)?;}else{
    if !self.flight.idle(){return Err(invalid().into())}
    let demand=self.close_demand(body)?;if !retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if self.requires_original_birth(){return Err(invalid().into())}
    if self.requires_original_control()&&original.is_none(){return Err(invalid().into())}
    if let Some(p)=self.preparation.as_deref_mut(){
     if !p.close_started(){let turn=p.begin_close_paid(&mut effect.nested(child(g,1)));work+=turn.work;return Ok(RetainedCloneStep::Progress(add(turn.result?,1)?))}
     if p.terminal_is_empty(){work=1;drop(self.preparation.take());return Ok(RetainedCloneStep::Progress(receipt(demand)))}
     route=if p.requires_original_control(){2}else{1};
    }else if !self.retirement.is_empty(){route=if self.retirement.requires_original_control(){4}else{3};}
    else if self.raw.is_some(){let turn=self.retirement.begin_granted(&mut self.raw,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}return Err(invalid().into())}
    else if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}return Err(invalid().into())}
    else if self.binding.is_some(){route=if RetainedCloneBinding::requires_original_control(&self.binding){6}else{5};}
    else{return Ok(RetainedCloneStep::Complete(Default::default()))}
   }
   let nested_body=body.saturating_sub(1);let loan=if matches!(route,2|4|6){original.as_deref_mut()}else{None};let before=work;
   let missing=||ValueError::invariant("original Energy authored field close lacks its original controller");
   let step=protocol::retained_routed!(self.flight,effect,recover,route,body,0,loan,work,|r| match route{
    1=>self.preparation.as_deref_mut().ok_or_else(invalid)?.close_into(nested_body,&mut effect.nested(child(g,1)),r),
    2=>self.preparation.as_deref_mut().ok_or_else(invalid)?.close_original_into(original.as_deref_mut().ok_or_else(missing)?,nested_body,&mut effect.nested(child(g,1)),r),
    3=>self.retirement.step_into(body,effect,r),
    4=>self.retirement.step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    5=>RetainedCloneBinding::close_into(&mut self.binding,body,effect,r),
    6=>RetainedCloneBinding::close_original_into(&mut self.binding,original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    _=>Err(invalid())},
    match route{
    1=>self.preparation.as_deref_mut().ok_or_else(invalid)?.recover_close_into(nested_body,&mut effect.nested(child(g,1)),r),
    2=>self.preparation.as_deref_mut().ok_or_else(invalid)?.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,nested_body,&mut effect.nested(child(g,1)),r),
    3=>self.retirement.recover_step_into(body,effect,r),
    4=>self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    5=>RetainedCloneBinding::recover_close_into(&mut self.binding,body,effect,r),
    6=>RetainedCloneBinding::recover_close_original_into(&mut self.binding,original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    _=>Err(invalid())});
   let step=step.map_err(|error|tagged(error,work!=before))?;
   Ok(RetainedCloneStep::Progress(step.progress()))
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.preparation.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.flight.idle()}
}
impl<T:RetainedClone+Send+Sync> Drop for RetainedSealedField<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original Energy authored field retains funded preparation and source custody");if empty{unsafe{ManuallyDrop::drop(&mut self.raw);ManuallyDrop::drop(&mut self.output);}}}}
fn tagged(error:OriginalRetirementFailure,paid:bool)->OriginalRetirementFailure{if !paid{return error}let progress=error.retained_progress();error.with_retained_progress(add(progress,1).unwrap_or(progress))}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Energy authored field lost its required source or phase")}
fn preparation_release<T:RetainedClone>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Box<RetainedFieldPreparation<T>>>>()+1,release_bytes:size_of::<RetainedFieldPreparation<T>>(),depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(extra),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
fn add(mut p:RetainedCloneProgress,extra:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(extra).ok_or_else(invalid)?;Ok(p)}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}

#[path="🌳️branch/🦀️.rs"]mod branch;
pub use branch::{RetainedBranchProjection,BranchProjectionOutput};

#[path="🔎️key/🦀️.rs"]mod entity_lookup;
pub use entity_lookup::{EntityRow,RetainedEntityLookup};
