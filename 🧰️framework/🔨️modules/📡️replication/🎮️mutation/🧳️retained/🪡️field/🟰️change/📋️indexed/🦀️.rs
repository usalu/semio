//! 📋️ Exact whole replacements compare original typed vectors and move each funded payload into ascending sparse rows.
use super::RetainedFieldChange;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned, retained_clone::{RetainedClone, RetainedCloneRef, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits}};
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
pub struct RetainedIndexedReplacement<T:RetainedEquality+RetainedClone,A:RetireOwned,M:RetainedIndexedMap<T,A>>{field:RetainedFieldChange<Vec<T>>,source:ManuallyDrop<Option<VecDeque<T>>>,removed:ManuallyDrop<Option<Vec<usize>>>,added:ManuallyDrop<Option<Vec<A>>>,retirement:RetainedCloneClose,mapper:std::marker::PhantomData<fn(M)>,old_len:usize,new_len:usize,index:usize,changed:bool,phase:u8,closing:bool}
impl<T:RetainedEquality+RetainedClone,A:RetireOwned,M:RetainedIndexedMap<T,A>> RetainedIndexedReplacement<T,A,M>{
 pub fn new()->Self{Self{field:Default::default(),source:ManuallyDrop::new(None),removed:ManuallyDrop::new(None),added:ManuallyDrop::new(None),retirement:Default::default(),mapper:Default::default(),old_len:0,new_len:0,index:0,changed:false,phase:0,closing:false}}
 pub fn ready(&self)->bool{self.phase==8&&!self.closing}
 pub fn demand(&self,a:RetainedCloneRef<'_,Vec<T>>,o:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>8{return Err(invalid("indexed original decision settled or closing"));}self.field.verify_sources(a,o)?;Ok(match self.phase{0=>nested(self.field.decision_demand(a,o,body.saturating_sub(META),maximum_work_units.saturating_sub(1))?),1=>{let mut d=self.field.change_demand()?;d.copy_bytes+=META+size_of::<VecDeque<T>>()*2+size_of::<Vec<T>>()*2+size_of::<usize>()*3;d.depth+=1;d},2=>RetirementDemand{capacity_bytes:capacity::<usize>(self.old_len)?,..scalar(META+size_of::<Vec<usize>>()*2)},3=>RetirementDemand{capacity_bytes:capacity::<A>(self.new_len)?,..scalar(META+size_of::<Vec<A>>()*2)},4=>scalar(META+size_of::<usize>()*3),5=>scalar(META+size_of::<Option<T>>()*2+size_of::<A>()*2),6=>RetirementDemand{release_bytes:capacity::<T>(self.source.as_ref().unwrap().capacity())?,..scalar(META+size_of::<Option<VecDeque<T>>>()*2)},8=>self.take_demand()?,_=>return Err(invalid("indexed original phase is invalid"))})}
 pub fn work_demand(&self,a:RetainedCloneRef<'_,Vec<T>>,o:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.demand(a,o,body,maximum_work_units)?;if maximum_work_units==0||self.phase==8{return Ok(0)}let child=match self.phase{0=>{let n=self.field.decision_work_demand(a,o,body.saturating_sub(META),maximum_work_units-1)?;if n==0{return Ok(0)}n},1=>self.field.take_change_work_demand()?,2|3|6=>1,4=>0,5=>if self.source.as_ref().unwrap().is_empty(){0}else{M::WORK},_=>return Err(invalid("indexed original phase is invalid"))};let work=child.checked_add(1).ok_or_else(||invalid("indexed decision work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}
 pub fn advance(&mut self,a:RetainedCloneRef<'_,Vec<T>>,o:RetainedCloneRef<'_,Vec<T>>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome{
  let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
   if actual.is_some(){return Err(invalid("indexed replacement requires an empty allocation marker").into())}if self.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}if maximum_work_units==0||g.maximum_items==0||g.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.work_demand(a,o,body,maximum_work_units)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   let d=self.demand(a,o,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};work=1;
   match self.phase{
    0=>{let turn=self.field.decide(a,o,body.saturating_sub(META),maximum_work_units-1,child(g));work+=turn.work;let step=turn.result.map_err(|error|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)})?;p=receipt(step.progress());if self.field.ready(){self.phase=1;}},
    1=>{let fixed=META+size_of::<VecDeque<T>>()*2+size_of::<Vec<T>>()*2+size_of::<usize>()*3;let turn=self.field.take_change(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});work+=turn.work;let Some((value,mut progress))=turn.result? else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.old_len=o.get().len();if let Some(value)=value{self.new_len=value.len();*self.source=Some(VecDeque::from(value));self.changed=true;self.phase=2;}else{self.phase=8;}progress.copied_bytes+=fixed;p=progress;},
    2=>{work+=1;let v=allocate_exact_vector::<usize>(self.old_len).map_err(|error|{if error.kind==ValueRefusalKind::AllocationFailed{*actual=Some(0)}error})?;p.retained_capacity_bytes=capacity::<usize>(v.capacity())?;if p.retained_capacity_bytes>0{*actual=Some(p.retained_capacity_bytes)}*self.removed=Some(v);self.phase=3;},
    3=>{work+=1;let v=allocate_exact_vector::<A>(self.new_len).map_err(|error|{if error.kind==ValueRefusalKind::AllocationFailed{*actual=Some(0)}error})?;p.retained_capacity_bytes=capacity::<A>(v.capacity())?;if p.retained_capacity_bytes>0{*actual=Some(p.retained_capacity_bytes)}*self.added=Some(v);self.index=0;self.phase=4;},
    4=>{if self.index==self.old_len{self.index=0;self.phase=5;}else{self.removed.as_mut().unwrap().push(self.index);self.index+=1;}},
    5=>{if let Some(value)=self.source.as_mut().unwrap().pop_front(){self.added.as_mut().unwrap().push(M::map(self.index,value));work+=M::WORK;self.index+=1;}else{self.phase=6;}},
    6=>{work+=1;assert!(self.source.as_ref().unwrap().is_empty());drop(self.source.take());p.released_bytes=d.release_bytes;self.phase=8;},
    _=>return Err(invalid("indexed original phase is invalid").into())
   }Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();semio_framework_value::RetainedWorkOutcome{result,work}
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("indexed original replacement is pending"));}Ok(scalar(META+size_of::<Option<IndexedReplacement<A>>>()*2))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(Option<IndexedReplacement<A>>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}self.phase=9;let result=if self.changed{Some(IndexedReplacement{removed:self.removed.take().unwrap(),added:self.added.take().unwrap()})}else{None};Ok(Some((result,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;self.field.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("indexed original close not begun"));}let body=body.saturating_sub(META);if !self.field.terminal_is_empty(){return Ok(nested(self.field.close_demand(body)?));}if !self.retirement.is_empty()||self.source.is_some(){return Ok(nested(self.retirement.next_owner_demands::<VecDeque<T>>(self.source.is_some(),&None,body)?));}if self.removed.is_some(){return Ok(nested(self.retirement.next_owner_demands::<Vec<usize>>(true,&None,body)?));}if self.added.is_some(){return Ok(nested(self.retirement.next_owner_demands::<Vec<A>>(true,&None,body)?));}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if !self.closing{return Err(invalid("indexed original close not begun"))}if self.terminal_is_empty(){return Ok(0)}let body=body.saturating_sub(META);
  let child=if !self.field.terminal_is_empty(){self.field.close_work_demand(body)?}else if !self.retirement.is_empty()||self.source.is_some(){self.retirement.next_owner_work::<VecDeque<T>>(self.source.is_some(),&None,body)?}else if self.removed.is_some(){self.retirement.next_owner_work::<Vec<usize>>(true,&None,body)?}else{self.retirement.next_owner_work::<Vec<A>>(true,&None,body)?};
  child.checked_add(1).ok_or_else(||invalid("indexed close work overflow"))
 }
 pub fn close_step(&mut self,g:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome{
  let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
   let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}work=1;
   if !self.field.terminal_is_empty(){let turn=self.field.close_step(child(g),actual);work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(receipt(s.progress()))).map_err(|error|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)})}
   if !self.retirement.is_empty(){let turn=self.retirement.step_granted(child(g));work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(receipt(s.progress()))).map_err(|error|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)})}
   let turn=self.retirement.begin_granted(&mut self.source,child(g));work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())))}
   let turn=self.retirement.begin_granted(&mut self.removed,child(g));work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())))}
   let turn=self.retirement.begin_granted(&mut self.added,child(g));work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())))}
   Ok(RetainedCloneStep::Complete(Default::default()))
  })();semio_framework_value::RetainedWorkOutcome{result,work}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.field.terminal_is_empty()&&self.source.is_none()&&self.removed.is_none()&&self.added.is_none()&&self.retirement.is_empty()}
}
impl<T:RetainedEquality+RetainedClone,A:RetireOwned,M:RetainedIndexedMap<T,A>> Drop for RetainedIndexedReplacement<T,A,M>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"indexed original source/deque/output custody requires explicit closure");if empty{unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.removed);ManuallyDrop::drop(&mut self.added);}}}}
#[cfg(test)] #[path="🧪️tests/🦀️.rs"] mod tests;
