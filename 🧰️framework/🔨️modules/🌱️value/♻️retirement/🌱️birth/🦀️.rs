//! 🌱️ Concrete inline cursors are allocated before their original owners leave retained slots.
use super::{RetireOwned,RetirementCursor,RetirementStep};
use crate::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use std::{alloc::{alloc,Layout},mem::size_of,ptr::NonNull};
pub fn retirement_cursor_birth_demand<T:RetireOwned>()->Result<RetirementDemand,ValueError>{
 let copy_bytes=size_of::<T::Retirement>().checked_add(size_of::<Option<T>>()).and_then(|value|value.checked_add(size_of::<Option<Box<dyn RetirementCursor>>>())).ok_or_else(||ValueError::invariant("retirement cursor placement extent overflow"))?;
 Ok(RetirementDemand{copy_bytes,capacity_bytes:Layout::new::<T::Retirement>().size(),depth:1,..Default::default()})
}
pub fn retirement_cursor_birth_work()->u64{4}
pub fn admit_retirement_cursor<T:RetireOwned>(source:&mut Option<T>,target:&mut Option<Box<dyn RetirementCursor>>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome{
 if source.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
 if !source.as_ref().is_some_and(RetireOwned::retirement_ready){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"retirement original is not ready for transfer")),work:0}}
 admit_retirement_cursor_from(target,grant,actual,||source.take().expect("original retirement slot was checked before allocation"))
}
pub fn admit_retirement_cursor_from<T:RetireOwned>(target:&mut Option<Box<dyn RetirementCursor>>,grant:RetainedCloneGrant,actual:&mut Option<usize>,take:impl FnOnce()->T)->RetainedWorkOutcome{
 let demand=match retirement_cursor_birth_demand::<T>(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
 if target.is_some()||actual.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("retirement birth requires its empty actual destination and marker")),work:0}}
 if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
 let layout=Layout::new::<T::Retirement>();
 let pointer=if layout.size()==0{NonNull::<T::Retirement>::dangling()}else{*actual=Some(0);let Some(pointer)=NonNull::new(unsafe{alloc(layout)}.cast::<T::Retirement>())else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"retirement cursor allocation refused")),work:1}};*actual=Some(layout.size());pointer};
 let value=take();
 unsafe{pointer.as_ptr().write(value.retirement());*target=Some(Box::from_raw(pointer.as_ptr()));}
 RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),released_bytes:0})),work:retirement_cursor_birth_work()}
}

pub trait RetirementFields:Send+'static{
 fn field_count(&self)->usize;
 fn field_demand(&self,index:usize)->Result<RetirementDemand,ValueError>;
 fn admit_field(&mut self,index:usize,target:&mut Option<Box<dyn RetirementCursor>>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome;
}
pub struct FieldsRetirementCursor<F:RetirementFields>{fields:F,index:usize,pending:Option<Box<dyn RetirementCursor>>}
impl<F:RetirementFields> FieldsRetirementCursor<F>{pub fn new(fields:F)->Self{Self{fields,index:0,pending:None}}}
impl<F:RetirementFields> RetirementCursor for FieldsRetirementCursor<F>{
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(if self.terminal_is_empty(){0}else if self.pending.is_some(){1}else{4})}
 fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if self.pending.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<dyn RetirementCursor>>>()+size_of::<usize>(),depth:1,..Default::default()})}self.fields.field_demand(self.index)}
 fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete(0)}
  let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}
  if self.pending.is_some(){let child=self.pending.take().expect("funded retirement child was retained");self.index+=1;return RetirementStep::ChildProgress(child,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1)}
  let turn=self.fields.admit_field(self.index,&mut self.pending,grant,actual);
  match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error.into(),turn.work)}
 }
 fn terminal_is_empty(&self)->bool{self.index==self.fields.field_count()&&self.pending.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<F:RetirementFields> Drop for FieldsRetirementCursor<F>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"retirement fields retain their original selected owners");}}

#[cfg(test)]mod tests{
 use super::*;
 struct Probe(u64);
 struct ProbeCursor(Option<u64>);
 impl RetireOwned for Probe{type Retirement=ProbeCursor;fn retirement(self)->Self::Retirement{ProbeCursor(Some(self.0))}fn controlled_retirement_supported()->bool{true}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::Retirement>())}}
 impl RetirementCursor for ProbeCursor{
  fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(u64::from(self.0.is_some()))}
  fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Option<u64>>(),depth:1,..Default::default()})}
  fn close_step(&mut self,grant:RetainedCloneGrant,_actual:&mut Option<usize>)->RetirementStep{if self.0.is_none(){return RetirementStep::Complete(0)}let demand=self.next_demand(0).unwrap();if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}self.0=None;RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1)}
  fn terminal_is_empty(&self)->bool{self.0.is_none()}
  fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
 }
 #[test]fn funded_inline_cursor_birth_preserves_originals_and_actual_heap_on_every_refusal(){
  let law:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🌱️birth/📐️.json")).unwrap();
  let demand=retirement_cursor_birth_demand::<Probe>().unwrap();
  for row in law["cases"].as_array().unwrap(){
   let axis=row["axis"].as_str().unwrap();let mut source=Some(Probe(73));let mut target=None;let mut actual=None;
   let mut grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.capacity_bytes,maximum_depth:1};
   match axis{"items"=>grant.maximum_items=0,"copy"=>grant.maximum_copy_bytes=demand.copy_bytes-1,"capacity"=>grant.maximum_capacity_bytes=demand.capacity_bytes-1,"depth"=>grant.maximum_depth=0,_=>{}}
   let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||if axis=="allocator"{let(turn,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||admit_retirement_cursor(&mut source,&mut target,grant,&mut actual));assert_eq!(attempts,1);turn}else{admit_retirement_cursor(&mut source,&mut target,grant,&mut actual)});
   assert_eq!(source.is_some(),row["retained"].as_bool().unwrap());assert_eq!(target.is_some(),!row["retained"].as_bool().unwrap());
   assert_eq!(match actual{None=>"none",Some(0)=>"zero",Some(_)=>"extent"},row["marker"].as_str().unwrap());
   if axis=="ready"{let progress=turn.result.unwrap().progress();assert_eq!(turn.work,4);assert_eq!(heap,(demand.capacity_bytes,0));assert_eq!(progress.retained_capacity_bytes,actual.unwrap());assert_eq!(progress.copied_bytes,demand.copy_bytes);let mut cursor=target.take().unwrap();assert!(matches!(cursor.close_step(RetainedCloneGrant{maximum_copy_bytes:usize::MAX,..grant},&mut None),RetirementStep::Progress(_,1)));assert!(cursor.terminal_is_empty());let(_,released)=semio_framework_value_testing_allocator::observe_retirement_allocations(||drop(cursor));assert_eq!(released,(0,demand.capacity_bytes));}else{assert_eq!(heap,(0,0));assert_eq!(source.as_ref().unwrap().0,73);if axis=="allocator"{assert!(turn.result.is_err());assert_eq!(turn.work,1)}else{assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default())}}
  }
 }
}
