use super::*;
use crate::{effect::{self,CPU},paged::PagedUtf8, retirement::controlled::ControlledRetirement};
use semio_framework_value_retirement::OriginalRetirementFailure as Failure;
use semio_framework_value_paged_list::list::PagedList;
use crate::retained_clone::{RetainedCloneGrant, RetainedCloneRef, RetainedCloneStep, RetirementDemand};
use semio_framework_value_testing_allocator::observe_retirement_allocations;

#[derive(Debug, crate::RetireOwned, serde::Deserialize, serde::Serialize)]#[value(crate="crate")]
struct Row { label: PagedUtf8<{usize::MAX}>, choice: Option<PagedUtf8<{usize::MAX}>> }
#[derive(Debug, crate::RetireOwned, serde::Deserialize, serde::Serialize)]#[value(crate="crate")]
struct Root { rows: PagedList<Row, {usize::MAX}> }

fn grant_of(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🪢️ Retains one issued reference as an owned projection through its two separately funded original turns and reports their physical heap.
fn retain<U:?Sized+Sync+'static>(row:RetainedCloneRef<'_,U>)->(RetainedOwnedProjection<U>,(usize,usize)){
 let mut admission=RetainedOwnedProjectionAdmission::<U>::default();let mut output=None;
 let g=grant_of(admission.demand(row).unwrap());let(first,first_heap)=observe_retirement_allocations(||effect::turn::<crate::ValueError>(g,CPU,&mut |e,r|admission.admit(row,e,r)));let p=first.result.unwrap().progress();assert!(p.fits(g));assert_eq!(first_heap,(p.retained_capacity_bytes,p.released_bytes));
 let g=grant_of(admission.take_demand().unwrap());let(second,second_heap)=observe_retirement_allocations(||effect::turn::<crate::ValueError>(g,CPU,&mut |e,r|admission.take(&mut output,e,r)));let p=second.result.unwrap().progress();assert!(p.fits(g));assert_eq!(second_heap,(p.retained_capacity_bytes,p.released_bytes));
 assert!(admission.terminal_is_empty());(output.expect("owned projection published"),(first_heap.0+second_heap.0,first_heap.1+second_heap.1))
}
/// 🧵️ Runs one funded alias close turn through the real original action.
fn close_turn<T:?Sized+Sync>(owner:&mut RetainedOwnedProjection<T>,grant:RetainedCloneGrant)->RetainedCloneStep{effect::turn::<Failure>(grant,CPU,&mut |e,r|owner.close_into(4096,e,r)).result.unwrap()}

#[test]
fn paged_native_owned_projection_retains_root_and_closes_one_actual_alias(){
 let corpus:serde_json::Value=serde_json::from_str(crate::text("📦️paged/🧪️tests/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();let law=&corpus["ownedProjection"];
 let(root,heap)=observe_retirement_allocations(||serde_json::from_value::<Root>(law["input"].clone()).unwrap());let original=heap.0-heap.1;let pointer=root.rows.get(0).unwrap().label.chunks().next().unwrap().as_ptr();
  let((mut source,p,_),heap)=semio_framework_value_clone_source_testing::observe_fixture_source_allocations(||effect::admit(root));assert_eq!(heap,(p.retained_capacity_bytes,0));let(mut born,mut freed)=(heap.0,0);let before=serde_json::to_value(source.borrow().get()).unwrap();
 for(index,vector)in law["paths"].as_array().unwrap().iter().enumerate(){
  let(mut parent,heap)=retain(source.borrow().project(1,|root|&root.rows));assert_eq!(heap,(0,0));
  let(mut child,heap)=retain(parent.borrow().unwrap().project(index,|rows|if index==0{&rows[index].label}else{rows[index].choice.as_ref().unwrap()}));assert_eq!(heap,(0,0));assert!(child.borrow().unwrap().get().eq_str(vector["value"].as_str().unwrap()));
  assert_eq!(close_turn(&mut child,Default::default()).progress(),Default::default());
  for owner in [&mut parent as &mut dyn ProjectionClose,&mut child as &mut dyn ProjectionClose]{
   for _ in 0..10000{if owner.empty(){break;}let grant=owner.grant();let(step,heap)=observe_retirement_allocations(||owner.step(grant));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}assert!(owner.empty());
  }
  assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),before);
 }
 let mut handback_turns=0;let mut returned=None;let root=loop{handback_turns+=1;assert!(handback_turns<100000,"original projection handback did not settle");let d=source.next_take_demand(4096).unwrap();let grant=grant_of(d);assert_eq!(source.next_take_demand(grant.maximum_copy_bytes).unwrap(),d);let(turn,heap)=observe_retirement_allocations(||effect::turn::<Failure>(grant,CPU,&mut |e,r|source.take_authority_into(&mut returned,grant.maximum_copy_bytes,e,r)));let step=turn.result.unwrap();let p=step.progress();born+=heap.0;freed+=heap.1;assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break returned.take().expect("source handed its original root back");}};
 assert_eq!(root.rows.get(0).unwrap().label.chunks().next().unwrap().as_ptr(),pointer);let mut close=ControlledRetirement::new(root).unwrap_or_else(|_|panic!("root closure"));
 for _ in 0..100000{if close.terminal_is_empty(){break;}let d=close.selected_demand(4096).unwrap();let grant=grant_of(d);assert_eq!(close.selected_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||effect::turn::<Failure>(grant,CPU,&mut |e,r|close.step_into(4096,e,r)).result.unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
 assert!(close.terminal_is_empty());assert_eq!(original+born,freed);assert_eq!(observe_retirement_allocations(||{drop(close);drop(source);}).1,(0,0));
}
trait ProjectionClose{fn grant(&self)->RetainedCloneGrant;fn step(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep;fn empty(&self)->bool;}
impl<T:?Sized+Sync> ProjectionClose for RetainedOwnedProjection<T>{fn grant(&self)->RetainedCloneGrant{let d=self.next_close_demand(4096).unwrap();let g=grant_of(d);assert_eq!(self.next_close_demand(g.maximum_copy_bytes).unwrap(),d);g}fn step(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{close_turn(self,grant)}fn empty(&self)->bool{self.terminal_is_empty()}}