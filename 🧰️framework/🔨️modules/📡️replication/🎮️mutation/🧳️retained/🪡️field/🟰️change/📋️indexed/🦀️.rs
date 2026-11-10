//! 📋️ Exact whole replacements compare original typed vectors and move each funded payload into ascending sparse rows.
use super::RetainedFieldChange;
use crate::field::{Done,Flight,Reply,slot,native,unit};
use semio_framework_value::{OriginalRetirementCloseLoan, OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned, retained_clone::{RetainedClone, RetainedCloneRef, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits}};
use semio_framework_value_persistent::retained_equality::RetainedEquality;
use semio_framework_value::retained_clone::allocation::backing::{allocate_exact_vector,exact_vector_bytes};
use std::{collections::VecDeque,mem::{size_of,ManuallyDrop}};
const META:usize=size_of::<usize>()*8+3;
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn capacity<T>(len:usize)->Result<usize,ValueError>{exact_vector_bytes::<T>(len)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=META;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-META,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=META;p}
/// 🎯️ Original base ordinals are removed; authored rows are added at exact ascending final positions.
#[derive(semio_framework_value::RetireOwned)]
pub struct IndexedReplacement<A:RetireOwned>{pub removed:Vec<usize>,pub added:Vec<A>}
/// 🎯️ An implementation moves one admitted header without allocation, traversal, destruction or panic, within its declared work.
pub trait RetainedIndexedMap<T,A:RetireOwned>:Send+'static{const WORK:u64;fn map(index:usize,value:T)->A;}
/// 🧳️ Each source row remains owned until its declared mapper transfers it into the result.
pub struct RetainedIndexedReplacement<T:RetainedEquality+RetainedClone,A:RetireOwned,M:RetainedIndexedMap<T,A>>{field:RetainedFieldChange<Vec<T>>,source:ManuallyDrop<Option<VecDeque<T>>>,removed:ManuallyDrop<Option<Vec<usize>>>,added:ManuallyDrop<Option<Vec<A>>>,retirement:RetainedCloneClose,flight:Flight,mapper:std::marker::PhantomData<fn(M)>,old_len:usize,new_len:usize,index:usize,changed:bool,phase:u8,closing:bool}
impl<T:RetainedEquality+RetainedClone,A:RetireOwned,M:RetainedIndexedMap<T,A>> RetainedIndexedReplacement<T,A,M>{
 pub fn new()->Self{Self{field:Default::default(),source:ManuallyDrop::new(None),removed:ManuallyDrop::new(None),added:ManuallyDrop::new(None),retirement:Default::default(),flight:Default::default(),mapper:Default::default(),old_len:0,new_len:0,index:0,changed:false,phase:0,closing:false}}
 pub fn ready(&self)->bool{self.phase==8&&!self.closing}
 pub fn demand(&self,a:RetainedCloneRef<'_,Vec<T>>,o:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>8{return Err(invalid("indexed original decision settled or closing"));}self.field.verify_sources(a,o)?;Ok(match self.phase{0=>nested(self.field.decision_demand(a,o,body.saturating_sub(META),maximum_work_units.saturating_sub(1))?),1=>{let mut d=self.field.change_demand()?;d.copy_bytes+=META+size_of::<VecDeque<T>>()*2+size_of::<Vec<T>>()*2+size_of::<usize>()*3;d.depth+=1;d},2=>RetirementDemand{capacity_bytes:capacity::<usize>(self.old_len)?,..scalar(META+size_of::<Vec<usize>>()*2)},3=>RetirementDemand{capacity_bytes:capacity::<A>(self.new_len)?,..scalar(META+size_of::<Vec<A>>()*2)},4=>scalar(META+size_of::<usize>()*3),5=>scalar(META+size_of::<Option<T>>()*2+size_of::<A>()*2),6=>RetirementDemand{release_bytes:capacity::<T>(self.source.as_ref().ok_or_else(||invalid("indexed source rows are absent"))?.capacity())?,..scalar(META+size_of::<Option<VecDeque<T>>>()*2)},8=>self.take_demand()?,_=>return Err(invalid("indexed original phase is invalid"))})}
 pub fn work_demand(&self,a:RetainedCloneRef<'_,Vec<T>>,o:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.demand(a,o,body,maximum_work_units)?;if maximum_work_units==0||self.phase==8{return Ok(0)}let child=match self.phase{0=>{let n=self.field.decision_work_demand(a,o,body.saturating_sub(META),maximum_work_units-1)?;if n==0{return Ok(0)}n},1=>self.field.take_change_work_demand()?,2|3|6=>1,4=>0,5=>if self.source.as_ref().ok_or_else(||invalid("indexed source rows are absent"))?.is_empty(){0}else{M::WORK},_=>return Err(invalid("indexed original phase is invalid"))};let work=child.checked_add(1).ok_or_else(||invalid("indexed decision work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}
/// 📥️ Records one decision turn of the indexed replacement or its field in the caller's reply.
 pub fn advance_into<'r>(&mut self,a:RetainedCloneRef<'_,Vec<T>>,o:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.advance_reply(a,o,body,maximum_work_units,effect,reply,false)}
 /// 🔁️ Recovers the same field decision action.
 pub fn recover_advance_into<'r>(&mut self,a:RetainedCloneRef<'_,Vec<T>>,o:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.advance_reply(a,o,body,maximum_work_units,effect,reply,true)}
 fn advance_reply<'r>(&mut self,a:RetainedCloneRef<'_,Vec<T>>,o:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{ let mut child_step=None;
  let g=effect.grant();
  if recover{self.flight.resume(effect,body,maximum_work_units,None)?;}else{
   if !self.flight.idle(){return Err(invalid("indexed replacement requires recovery"))}
   if effect.actual().is_some(){return Ok(reply.complete(Err(invalid("indexed replacement requires an empty allocation marker").into())))}
   if self.ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
   if maximum_work_units==0||g.maximum_items==0||g.maximum_depth==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   let work=match self.work_demand(a,o,body,maximum_work_units){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if work==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   let d=match self.demand(a,o,body,maximum_work_units){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if !retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   charge!(effect,reply,1,"indexed replacement exceeded original Work");
   if self.phase!=0{
    let settled:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
     let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
     match self.phase{
      1=>{let fixed=META+size_of::<VecDeque<T>>()*2+size_of::<Vec<T>>()*2+size_of::<usize>()*3;let mut value=None;let mut inner=effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});let taken=native(|slot|self.field.take_change_into(&mut value,&mut inner,slot))?;drop(inner);let RetainedCloneStep::Complete(mut progress)=taken? else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.old_len=o.get().len();if let Some(value)=value{self.new_len=value.len();*self.source=Some(VecDeque::from(value));self.changed=true;self.phase=2;}else{self.phase=8;}progress.copied_bytes+=fixed;p=progress;},
      2=>{unit(effect,1,"indexed removed rows exceeded original Work")?;let v=allocate_exact_vector::<usize>(self.old_len).map_err(|error|{if error.kind==ValueRefusalKind::AllocationFailed{*effect.actual()=Some(0)}error})?;p.retained_capacity_bytes=capacity::<usize>(v.capacity())?;if p.retained_capacity_bytes>0{*effect.actual()=Some(p.retained_capacity_bytes)}*self.removed=Some(v);self.phase=3;},
      3=>{unit(effect,1,"indexed added rows exceeded original Work")?;let v=allocate_exact_vector::<A>(self.new_len).map_err(|error|{if error.kind==ValueRefusalKind::AllocationFailed{*effect.actual()=Some(0)}error})?;p.retained_capacity_bytes=capacity::<A>(v.capacity())?;if p.retained_capacity_bytes>0{*effect.actual()=Some(p.retained_capacity_bytes)}*self.added=Some(v);self.index=0;self.phase=4;},
      4=>{if self.index==self.old_len{self.index=0;self.phase=5;}else{self.removed.as_mut().ok_or_else(||invalid("indexed removed rows are absent"))?.push(self.index);self.index+=1;}},
      5=>{let source=self.source.as_mut().ok_or_else(||invalid("indexed source rows are absent"))?;if let Some(value)=source.pop_front(){self.added.as_mut().ok_or_else(||invalid("indexed added rows are absent"))?.push(M::map(self.index,value));unit(effect,M::WORK,"indexed mapping exceeded original Work")?;self.index+=1;}else{self.phase=6;}},
      6=>{unit(effect,1,"indexed release exceeded original Work")?;if !self.source.as_ref().is_some_and(VecDeque::is_empty){return Err(invalid("indexed source rows remain at release").into())}drop(self.source.take());p.released_bytes=d.release_bytes;self.phase=8;},
      _=>return Err(invalid("indexed original phase is invalid").into())
     }Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
    })();
    return Ok(reply.complete(settled))
   }
   self.flight.start(effect,1,body,maximum_work_units,None)?;
  }
  let mut inner=effect.nested(child(g));let nested=body.saturating_sub(META);let rest=maximum_work_units.saturating_sub(1);
  if recover{let done=self.field.recover_decide_into(a,o,nested,rest,&mut inner,slot(&mut child_step)?)?;drop(done)}else{let done=self.field.decide_into(a,o,nested,rest,&mut inner,slot(&mut child_step)?)?;drop(done)}
  drop(inner);
  let Some(result)=self.flight.settle(&mut child_step)else{return Ok(reply.pending())};
  let settled=result.map_err(|error|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)}).map(|step|{if self.field.ready(){self.phase=1;}let progress=receipt(step.progress());if self.ready(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}});
  Ok(reply.complete(settled))
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("indexed original replacement is pending"));}Ok(scalar(META+size_of::<Option<IndexedReplacement<A>>>()*2))}
/// 📤️ Moves the finished replacement into the caller slot; Complete with an empty slot means unchanged.
 pub fn take_into<'r>(&mut self,output:&mut Option<IndexedReplacement<A>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Done<'r,RetainedCloneStep,ValueError>,ValueError>{
  if output.is_some(){return Err(invalid("indexed replacement output is occupied"))}
  let d=match self.take_demand(){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error)))};
  let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error)))};
  if remaining<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"indexed replacement result exceeded original Work")))),Err(error)=>return Ok(reply.complete(Err(error)))}
  self.phase=9;
  if self.changed{match(self.removed.take(),self.added.take()){(Some(removed),Some(added))=>*output=Some(IndexedReplacement{removed,added}),_=>return Ok(reply.complete(Err(invalid("indexed changed rows are absent"))))}}
  Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))
 }
 pub fn begin_close(&mut self){self.closing=true;self.field.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("indexed original close not begun"));}let body=body.saturating_sub(META);if !self.field.terminal_is_empty(){return Ok(nested(self.field.close_demand(body)?));}if !self.retirement.is_empty()||self.source.is_some(){return Ok(nested(self.retirement.next_owner_demands::<VecDeque<T>>(self.source.is_some(),&None,body)?));}if self.removed.is_some(){return Ok(nested(self.retirement.next_owner_demands::<Vec<usize>>(true,&None,body)?));}if self.added.is_some(){return Ok(nested(self.retirement.next_owner_demands::<Vec<A>>(true,&None,body)?));}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if !self.closing{return Err(invalid("indexed original close not begun"))}if self.terminal_is_empty(){return Ok(0)}let body=body.saturating_sub(META);
  let child=if !self.field.terminal_is_empty(){self.field.close_work_demand(body)?}else if !self.retirement.is_empty()||self.source.is_some(){self.retirement.next_owner_work::<VecDeque<T>>(self.source.is_some(),&None,body)?}else if self.removed.is_some(){self.retirement.next_owner_work::<Vec<usize>>(true,&None,body)?}else{self.retirement.next_owner_work::<Vec<A>>(true,&None,body)?};
  child.checked_add(1).ok_or_else(||invalid("indexed close work overflow"))
 }
/// 👶️ Selects the child whose original Birth must precede an ordinary effect.
 pub fn requires_original_birth(&self)->bool{if !self.closing||!self.flight.idle(){return false}if !self.field.terminal_is_empty(){return self.field.requires_original_birth()}if !self.retirement.is_empty(){return self.retirement.requires_original_birth()}false}
 /// 🪢️ Reports whether the selected child close runs under its original controller.
 pub fn requires_original_control(&self)->bool{if let Some(loaned)=self.flight.loaned(){return loaned}if !self.closing{return false}if !self.field.terminal_is_empty(){return self.field.requires_original_control()}if !self.retirement.is_empty(){return self.retirement.requires_original_control()}false}
 /// 👶️ Continues the selected child Birth on the original controller.
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if !self.requires_original_birth(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("indexed original Birth is not selected").into()),work:0})}if !self.field.terminal_is_empty(){return self.field.close_original_birth_step(original,grant)}self.retirement.step_original_birth(original,grant)}
 fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{ let mut child_step=None;
  if !self.closing{return Err(invalid("indexed original close not begun"))}
  let g=effect.grant();let nested=body.saturating_sub(META);
  let route=if recover{self.flight.resume(effect,body,0,original.as_deref_mut())?}else{
   if !self.flight.idle(){return Err(invalid("indexed close requires recovery"))}
   if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
   if self.requires_original_birth(){return Err(invalid("indexed Birth must settle before effect"))}
   if self.requires_original_control()&&original.is_none(){return Err(invalid("indexed close requires original controller"))}
   let d=match self.close_demand(body){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   let work=match self.close_work_demand(body){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if remaining<work||!retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   charge!(effect,reply,1,"indexed close exceeded original Work");
   let route=if !self.field.terminal_is_empty(){1}else if !self.retirement.is_empty(){2}else{
    let mut inner=effect.nested(child(g));
    let turn=self.retirement.begin_granted(&mut self.source,&mut inner);
    let turn=match turn.result{Ok(None)=>self.retirement.begin_granted(&mut self.removed,&mut inner),other=>RetainedWorkOutcome{result:other,work:turn.work}};
    let turn=match turn.result{Ok(None)=>self.retirement.begin_granted(&mut self.added,&mut inner),other=>RetainedWorkOutcome{result:other,work:turn.work}};
    return Ok(reply.complete(match turn.result{Ok(Some(step))=>Ok(RetainedCloneStep::Progress(receipt(step.progress()))),Ok(None)=>Ok(RetainedCloneStep::Complete(Default::default())),Err(error)=>{let error:OriginalRetirementFailure=error.into();let progress=receipt(error.retained_progress());Err(error.with_retained_progress(progress))}}))
   };
   self.flight.start(effect,route,body,0,original.as_deref_mut())?;route
  };
  let mut inner=effect.nested(child(g));
  match(route,recover,original.as_deref_mut()){
   (1,false,None)=>{let done=self.field.close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,true,None)=>{let done=self.field.recover_close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,false,Some(loan))=>{let done=self.field.close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,true,Some(loan))=>{let done=self.field.recover_close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,false,None)=>{let done=self.retirement.step_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,true,None)=>{let done=self.retirement.recover_step_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,false,Some(loan))=>{let done=self.retirement.step_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,true,Some(loan))=>{let done=self.retirement.recover_step_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   _=>return Err(invalid("indexed close route changed")),
  }
  drop(inner);
  match self.flight.settle(&mut child_step){None=>Ok(reply.pending()),Some(result)=>{let settled=result.map_err(|error|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)}).map(|step|{let progress=receipt(step.progress());if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}});Ok(reply.complete(settled))}}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.flight.idle()&&self.field.terminal_is_empty()&&self.source.is_none()&&self.removed.is_none()&&self.added.is_none()&&self.retirement.is_empty()}
}
retained_close!([T:RetainedEquality+RetainedClone,A:RetireOwned,M:RetainedIndexedMap<T,A>] RetainedIndexedReplacement<T,A,M>,close_reply);
impl<T:RetainedEquality+RetainedClone,A:RetireOwned,M:RetainedIndexedMap<T,A>> Drop for RetainedIndexedReplacement<T,A,M>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"indexed original source/deque/output custody requires explicit closure");if empty{unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.removed);ManuallyDrop::drop(&mut self.added);}}}}
#[cfg(test)] #[path="🧪️tests/🦀️.rs"] mod tests;
