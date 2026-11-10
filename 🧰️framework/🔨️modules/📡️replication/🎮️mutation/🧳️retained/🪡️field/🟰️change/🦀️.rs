//! 🎚️ Typed fields share original typed equality, authored cloning, and exact inverse ownership.
use crate::{field::{Done,Flight,Reply,RetainedFieldPreparation,native,slot,unit},retained_mutation_demand_fits};
use semio_framework_value::{OriginalRetirementCloseLoan, OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedClone, RetainedCloneRef, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value_persistent::{retained_equality::RetainedEquality, retained_equality::RetainedEqualityOwner};
use std::mem::size_of;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}

/// 🪡️ None means unchanged; Some(value) transfers the original funded authored field without a synthetic source wrapper.
pub struct RetainedFieldChange<T:RetainedEquality+RetainedClone>{equality:RetainedEqualityOwner<T>,authored:RetainedFieldPreparation<T>,original:RetainedFieldPreparation<T>,equal:Option<bool>,flight:Flight,phase:u8,closing:bool}
impl<T:RetainedEquality+RetainedClone> Default for RetainedFieldChange<T>{fn default()->Self{Self{equality:RetainedEqualityOwner::new(),authored:Default::default(),original:Default::default(),equal:None,flight:Default::default(),phase:0,closing:false}}}
impl<T:RetainedEquality+RetainedClone> RetainedFieldChange<T>{
 /// ⛽️ Quotes the equality owner, two lazy field preparations and seven direct fields.
 pub fn birth_work()->Result<u64,ValueError>{let equality=RetainedEqualityOwner::<T>::birth_work()?;let field=RetainedFieldPreparation::<T>::embedded_birth_work()?;equality.checked_add(field.checked_mul(2).ok_or_else(||invalid("field birth work overflow"))?).and_then(|v|v.checked_add(7)).ok_or_else(||invalid("field birth work overflow"))}
 /// 🧭️ Typed wrappers keep the same original source pair after child ownership transfer.
 pub fn verify_sources(&self,a:RetainedCloneRef<'_,T>,o:RetainedCloneRef<'_,T>)->Result<(),ValueError>{self.equality.verify(a,o)}
 fn fixed()->usize{size_of::<u8>()*2}
 fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=Self::fixed();d.depth+=1;d}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::fixed(),maximum_depth:g.maximum_depth-1,..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=Self::fixed();p}
 fn refuse(error:OriginalRetirementFailure)->OriginalRetirementFailure{let progress=Self::receipt(error.retained_progress());error.with_retained_progress(progress)}
 pub fn ready(&self)->bool{self.phase==2&&!self.closing}
 pub fn inverse_ready(&self)->bool{self.phase==4&&!self.closing}
 pub fn decision_demand(&self,authored:RetainedCloneRef<'_,T>,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid("typed field decision is closing"))}let body=body.saturating_sub(Self::fixed());let maximum=maximum_work_units.saturating_sub(2);Ok(match self.phase{0=>Self::nested(self.equality.demand(authored,original,body,maximum)?),1=>Self::nested(self.authored.demand(authored,body,maximum)?),2=>self.change_demand()?,_=>return Err(invalid("typed field decision is spent"))})}
 pub fn decision_work_demand(&self,authored:RetainedCloneRef<'_,T>,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.decision_demand(authored,original,body,maximum_work_units)?;if maximum_work_units==0||self.phase==2{return Ok(0)}let body=body.saturating_sub(Self::fixed());let maximum=maximum_work_units.saturating_sub(2);let child=match self.phase{0=>self.equality.work_demand(authored,original,body,maximum)?,1=>self.authored.advance_work_demand(authored,body,maximum)?,_=>return Err(invalid("typed field decision is spent"))};if child==0{return Ok(0)}let work=child.checked_add(2).ok_or_else(||invalid("typed field work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}
 /// 📥️ Records one decision turn of the equality or authored preparation child in the caller's reply.
 pub fn decide_into<'r>(&mut self,authored:RetainedCloneRef<'_,T>,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.decide_reply(authored,original,body,maximum_work_units,effect,reply,false)}
 /// 🔁️ Recovers the same decision child action.
 pub fn recover_decide_into<'r>(&mut self,authored:RetainedCloneRef<'_,T>,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.decide_reply(authored,original,body,maximum_work_units,effect,reply,true)}
 fn decide_reply<'r>(&mut self,authored:RetainedCloneRef<'_,T>,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant(); let mut child_equal=None;let mut child_step=None;
  let route=if recover{self.flight.resume(effect,body,maximum_work_units,None)?}else{
   if !self.flight.idle(){return Err(invalid("typed field decision requires recovery"))}
   if effect.actual().is_some(){return Ok(reply.complete(Err(invalid("field decision needs an empty allocation marker").into())))}
   if self.ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
   let work=match self.decision_work_demand(authored,original,body,maximum_work_units){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if work==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   let demand=match self.decision_demand(authored,original,body,maximum_work_units){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if !retained_mutation_demand_fits(demand,g){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   charge!(effect,reply,1,"typed field decision exceeded original Work");
   let route=self.phase+1;self.flight.start(effect,route,body,maximum_work_units,None)?;route
  };
  let nested=body.saturating_sub(Self::fixed());let rest=maximum_work_units.saturating_sub(2);let mut child=effect.nested(Self::child(g));
  match(route,recover){
   (1,false)=>{let done=self.equality.advance_into(authored,original,nested,rest,&mut child,slot(&mut child_equal)?)?;drop(done)},
   (1,true)=>{let done=self.equality.recover_advance_into(authored,original,nested,rest,&mut child,slot(&mut child_equal)?)?;drop(done)},
   (2,false)=>{let done=self.authored.advance_into(authored,nested,rest,&mut child,slot(&mut child_step)?)?;drop(done)},
   (2,true)=>{let done=self.authored.recover_advance_into(authored,nested,rest,&mut child,slot(&mut child_step)?)?;drop(done)},
   _=>return Err(invalid("typed field decision route changed")),
  }
  drop(child);
  let settled:Result<RetainedCloneStep,OriginalRetirementFailure>=if route==1{match self.flight.settle(&mut child_equal){None=>return Ok(reply.pending()),Some(result)=>(||{let step=result.map_err(Self::refuse)?;let mut progress=step.progress;if let Some(equal)=step.equal{self.equal=Some(equal);self.phase=if equal{2}else{1};unit(effect,1,"typed field decision exceeded original Work")?;progress.copied_items=1;progress.copied_bytes+=Self::fixed();}Ok(if self.ready(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})})()}}else{match self.flight.settle(&mut child_step){None=>return Ok(reply.pending()),Some(result)=>(||{let step=result.map_err(Self::refuse)?;let complete=matches!(step,RetainedCloneStep::Complete(_));let mut progress=step.progress();if complete{self.phase=2;unit(effect,1,"typed field decision exceeded original Work")?;progress.copied_items=1;progress.copied_bytes+=Self::fixed();}Ok(if self.ready(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})})()}};
  Ok(reply.complete(settled))
 }
 /// 📤️ Quotes both the original authored child transfer and the exact outer tri-state result header.
 pub fn change_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("typed field authored change is not ready"));}let fixed=size_of::<Option<T>>()*2+Self::fixed();if self.equal==Some(true){return Ok(scalar(fixed));}let mut d=self.authored.exchange_demand()?;d.copy_bytes+=fixed;d.depth+=1;Ok(d)}
 pub fn take_change_work_demand(&self)->Result<u64,ValueError>{self.change_demand()?;Ok(1)}
 /// 📤️ Moves the authored field into the caller slot; Complete with an empty slot means unchanged.
 pub fn take_change_into<'r>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Done<'r,RetainedCloneStep,ValueError>,ValueError>{
  if output.is_some(){return Err(invalid("typed field change output is occupied"))}
  let demand=match self.change_demand(){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error)))};
  let g=effect.grant();let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error)))};
  if remaining<1||!retained_mutation_demand_fits(demand,g){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let fixed=size_of::<Option<T>>()*2+Self::fixed();
  if self.equal==Some(true){match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"typed field change exceeded original Work")))),Err(error)=>return Ok(reply.complete(Err(error)))}self.phase=3;return Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:fixed,..Default::default()}))))}
  let mut child=effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});
  let result=native(|inner|self.authored.take_into(output,&mut child,inner))?;
  Ok(reply.complete(result.map(|step|match step{RetainedCloneStep::Complete(mut progress)=>{self.phase=3;progress.copied_bytes+=fixed;RetainedCloneStep::Complete(progress)},progress=>progress})))
 }
 pub fn inverse_demand(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=3{return Err(invalid("typed field inverse lacks original decision transfer"))}Ok(Self::nested(self.original.demand(original,body.saturating_sub(Self::fixed()),maximum_work_units.saturating_sub(self.equality.verify_original_work().saturating_add(1)))?))}
 pub fn inverse_work_demand(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.inverse_demand(original,body,maximum_work_units)?;let child=self.original.advance_work_demand(original,body.saturating_sub(Self::fixed()),maximum_work_units.saturating_sub(self.equality.verify_original_work().saturating_add(1)))?;if child==0{return Ok(0)}let work=child.checked_add(self.equality.verify_original_work()).and_then(|v|v.checked_add(1)).ok_or_else(||invalid("typed field inverse work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}
 /// 📥️ Records one inverse turn of the original preparation child in the caller's reply.
 pub fn inverse_into<'r>(&mut self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.inverse_reply(original,body,maximum_work_units,effect,reply,false)}
 /// 🔁️ Recovers the same inverse child action.
 pub fn recover_inverse_into<'r>(&mut self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.inverse_reply(original,body,maximum_work_units,effect,reply,true)}
 fn inverse_reply<'r>(&mut self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let overhead=self.equality.verify_original_work().saturating_add(1); let mut child_step=None;
  if recover{self.flight.resume(effect,body,maximum_work_units,None)?;}else{
   if !self.flight.idle(){return Err(invalid("typed field inverse requires recovery"))}
   if effect.actual().is_some(){return Ok(reply.complete(Err(invalid("field inverse needs an empty allocation marker").into())))}
   let work=match self.inverse_work_demand(original,body,maximum_work_units){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if work==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   let demand=match self.inverse_demand(original,body,maximum_work_units){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if !retained_mutation_demand_fits(demand,g){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   match self.equality.verify_original_paid(original,effect).result{Ok(Some(true))=>{},Ok(_)=>return Ok(reply.complete(Err(invalid("field original binding changed").into()))),Err(error)=>return Ok(reply.complete(Err(error.into())))}
   self.flight.start(effect,3,body,maximum_work_units,None)?;
  }
  let nested=body.saturating_sub(Self::fixed());let rest=maximum_work_units.saturating_sub(overhead);let mut child=effect.nested(Self::child(g));
  if recover{let done=self.original.recover_advance_into(original,nested,rest,&mut child,slot(&mut child_step)?)?;drop(done)}else{let done=self.original.advance_into(original,nested,rest,&mut child,slot(&mut child_step)?)?;drop(done)}
  drop(child);
  let settled:Result<RetainedCloneStep,OriginalRetirementFailure>=match self.flight.settle(&mut child_step){None=>return Ok(reply.pending()),Some(result)=>(||{let step=result.map_err(Self::refuse)?;let complete=matches!(step,RetainedCloneStep::Complete(_));let mut progress=step.progress();if complete{self.phase=4;unit(effect,1,"typed field inverse exceeded original Work")?;progress.copied_items=1;progress.copied_bytes+=Self::fixed();}Ok(if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})})()};
  Ok(reply.complete(settled))
 }
 pub fn inverse_take_demand(&self)->Result<RetirementDemand,ValueError>{if self.phase!=4||self.closing{return Err(invalid("typed field inverse is not ready"));}Ok(Self::nested(self.original.exchange_demand()?))}
 pub fn take_inverse_work_demand(&self)->Result<u64,ValueError>{self.inverse_take_demand()?;Ok(1)}
 /// 📤️ Moves the displaced original field into the caller slot.
 pub fn take_inverse_into<'r>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Done<'r,RetainedCloneStep,ValueError>,ValueError>{
  if output.is_some(){return Err(invalid("typed field inverse output is occupied"))}
  let demand=match self.inverse_take_demand(){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error)))};
  let g=effect.grant();let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error)))};
  if remaining<1||!retained_mutation_demand_fits(demand,g){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let mut child=effect.nested(Self::child(g));
  let result=native(|inner|self.original.take_into(output,&mut child,inner))?;
  Ok(reply.complete(result.map(|step|match step{RetainedCloneStep::Complete(progress)=>{self.phase=5;RetainedCloneStep::Complete(Self::receipt(progress))},progress=>progress})))
 }
 /// 🚦️ Marks this field closing; each child starts its own paid close inside the close entries.
 pub fn begin_close(&mut self){self.closing=true;}
 fn closing_child(&self)->u8{if !self.authored.terminal_is_empty(){1}else if !self.original.terminal_is_empty(){2}else if !self.equality.terminal_is_empty(){3}else{0}}
 fn started(&self,child:u8)->bool{match child{1=>self.authored.close_started(),2=>self.original.close_started(),_=>self.equality.close_started()}}
 fn child_close(&self,child:u8,body:usize)->Result<(RetirementDemand,u64),ValueError>{Ok(match(child,self.started(child)){(0,_)=>(Default::default(),0),(1,false)=>(self.authored.begin_close_demand(),self.authored.begin_close_work_demand()),(2,false)=>(self.original.begin_close_demand(),self.original.begin_close_work_demand()),(3,false)=>(self.equality.begin_close_demand(),1),(1,true)=>(self.authored.close_demand(body)?,self.authored.close_work_demand(body)?),(2,true)=>(self.original.close_demand(body)?,self.original.close_work_demand(body)?),_=>(self.equality.close_demand(body)?,self.equality.close_work_demand(body)?)})}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("typed field field close has not begun"));}Ok(Self::nested(self.child_close(self.closing_child(),body.saturating_sub(Self::fixed()))?.0))}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid("typed field field close has not begun"));}let child=self.child_close(self.closing_child(),body.saturating_sub(Self::fixed()))?.1;if child==0{return Ok(0)}child.checked_add(1).ok_or_else(||invalid("typed field close work overflow"))}
 /// 👶️ Selects the started child whose original Birth must precede an ordinary effect.
 pub fn requires_original_birth(&self)->bool{if !self.closing||!self.flight.idle(){return false}let child=self.closing_child();child!=0&&self.started(child)&&match child{1=>self.authored.requires_original_birth(),2=>self.original.requires_original_birth(),_=>self.equality.requires_original_birth()}}
 /// 🪢️ Reports whether the selected child close runs under its original controller.
 pub fn requires_original_control(&self)->bool{if let Some(loaned)=self.flight.loaned(){return loaned}let child=self.closing_child();child!=0&&self.started(child)&&match child{1=>self.authored.requires_original_control(),2=>self.original.requires_original_control(),_=>self.equality.requires_original_control()}}
 /// 👶️ Continues the selected child Birth on the original controller.
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if !self.requires_original_birth(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("typed field original Birth is not selected").into()),work:0})}match self.closing_child(){1=>self.authored.close_original_birth_step(original,grant),2=>self.original.close_original_birth_step(original,grant),_=>self.equality.close_original_birth_step(original,grant)}}
 fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  if !self.closing{return Err(invalid("typed field close has not begun"))}
  let g=effect.grant();let nested=body.saturating_sub(Self::fixed());
  let mut child_step=None;
  let child=if recover{self.flight.resume(effect,body,0,original.as_deref_mut())?}else{
   if !self.flight.idle(){return Err(invalid("typed field close requires recovery"))}
   if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
   if self.requires_original_birth(){return Err(invalid("typed field Birth must settle before effect"))}
   if self.requires_original_control()&&original.is_none(){return Err(invalid("typed field close requires original controller"))}
   let demand=match self.close_demand(body){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   let work=match self.close_work_demand(body){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if remaining<work||!retained_mutation_demand_fits(demand,g){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   charge!(effect,reply,1,"typed field close exceeded original Work");
   let child=self.closing_child();
   if !self.started(child){let mut inner=effect.nested(Self::child(g));let result=match child{3=>native(|slot|self.equality.begin_close_into(&mut inner,slot))?,_=>{let turn=if child==1{self.authored.begin_close_paid(&mut inner)}else{self.original.begin_close_paid(&mut inner)};turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}};return Ok(reply.complete(result.map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress()))).map_err(Self::refuse)))}
   self.flight.start(effect,child,body,0,original.as_deref_mut())?;child
  };
  let mut inner=effect.nested(Self::child(g));
  match(child,recover,original.as_deref_mut()){
   (1,false,None)=>{let done=self.authored.close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,true,None)=>{let done=self.authored.recover_close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,false,Some(loan))=>{let done=self.authored.close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,true,Some(loan))=>{let done=self.authored.recover_close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,false,None)=>{let done=self.original.close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,true,None)=>{let done=self.original.recover_close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,false,Some(loan))=>{let done=self.original.close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,true,Some(loan))=>{let done=self.original.recover_close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (3,false,None)=>{let done=self.equality.close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (3,true,None)=>{let done=self.equality.recover_close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (3,false,Some(loan))=>{let done=self.equality.close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (3,true,Some(loan))=>{let done=self.equality.recover_close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   _=>return Err(invalid("typed field close route changed")),
  }
  drop(inner);
  match self.flight.settle(&mut child_step){None=>Ok(reply.pending()),Some(result)=>{let settled=result.map_err(Self::refuse).map(|step|{let progress=Self::receipt(step.progress());if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}});Ok(reply.complete(settled))}}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.flight.idle()&&self.authored.terminal_is_empty()&&self.original.terminal_is_empty()&&self.equality.terminal_is_empty()}
}
retained_close!([T:RetainedEquality+RetainedClone] RetainedFieldChange<T>,close_reply);

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🎚️record/🦀️.rs"] pub mod record;

#[path="📋️indexed/🦀️.rs"] pub mod indexed;
