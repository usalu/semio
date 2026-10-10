use super::*;
use crate::{list::PagedList, paged::PagedUtf8, retirement::controlled::ControlledRetirement};
use crate::retained_clone::{RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep};
use crate::value::observe_retirement_allocations;

#[derive(Debug, crate::RetireOwned, serde::Deserialize, serde::Serialize)]
struct Row { label: PagedUtf8<{usize::MAX}>, choice: Option<PagedUtf8<{usize::MAX}>> }
#[derive(Debug, crate::RetireOwned, serde::Deserialize, serde::Serialize)]
struct Root { rows: PagedList<Row, {usize::MAX}> }

#[test]
fn paged_native_owned_projection_retains_root_and_closes_one_actual_alias(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();let law=&corpus["ownedProjection"];
 let(root,heap)=observe_retirement_allocations(||serde_json::from_value::<Root>(law["input"].clone()).unwrap());let original=heap.0-heap.1;let pointer=root.rows.get(0).unwrap().label.chunks().next().unwrap().as_ptr();
 let birth=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<Root>::constructor_copy_bytes(),maximum_capacity_bytes:RetainedCloneSource::<Root>::constructor_capacity_bytes::<()>(),maximum_depth:1,..Default::default()};
 let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(root,(),birth));let(mut source,p)=result.unwrap_or_else(|_|panic!("original root birth"));assert_eq!(heap,(p.retained_capacity_bytes,0));let(mut born,mut freed)=(heap.0,0);let before=serde_json::to_value(source.borrow().get()).unwrap();
 for(index,vector)in law["paths"].as_array().unwrap().iter().enumerate(){
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:source.borrow().binding_copy_bytes(),maximum_depth:1,..Default::default()};
  let(result,heap)=observe_retirement_allocations(||source.project_owned(1,|root|&root.rows,grant));let(mut parent,p)=result.unwrap();assert_eq!(heap,(0,0));assert!(p.fits(grant));
  let(result,heap)=observe_retirement_allocations(||parent.project(index,|rows|if index==0{&rows[index].label}else{rows[index].choice.as_ref().unwrap()},grant));let(mut child,p)=result.unwrap();assert_eq!(heap,(0,0));assert!(p.fits(grant));assert!(child.borrow().unwrap().get().eq_str(vector["value"].as_str().unwrap()));
  assert_eq!(child.close_step(Default::default()).unwrap().progress(),Default::default());
  for owner in [&mut parent as &mut dyn ProjectionClose,&mut child as &mut dyn ProjectionClose]{
   for _ in 0..10000{if owner.empty(){break;}let grant=owner.grant();let(step,heap)=observe_retirement_allocations(||owner.step(grant));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}assert!(owner.empty());
  }
  assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),before);
 }
 let root=loop{let copy=source.next_take_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_take_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_take_release_byte_demand().unwrap(),maximum_depth:source.next_take_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||source.take_authority(grant).unwrap());born+=heap.0;freed+=heap.1;match step{crate::retained_clone::RetainedCloneSourceTake::Pending(p)=>assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes)),crate::retained_clone::RetainedCloneSourceTake::Ready(root,p)=>{assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));break root;}}};
 assert_eq!(root.rows.get(0).unwrap().label.chunks().next().unwrap().as_ptr(),pointer);let mut close=ControlledRetirement::new(root).unwrap_or_else(|_|panic!("root closure"));
 for _ in 0..100000{if close.terminal_is_empty(){break;}let copy=close.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||close.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
 assert!(close.terminal_is_empty());assert_eq!(original+born,freed);assert_eq!(observe_retirement_allocations(||{drop(close);drop(source);}).1,(0,0));println!("[DEBUG] owned projections same original paged root original={original} born={born} release={freed}");
}
trait ProjectionClose{fn grant(&self)->RetainedCloneGrant;fn step(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep;fn empty(&self)->bool;}
impl<T:?Sized+Sync> ProjectionClose for RetainedOwnedProjection<T>{fn grant(&self)->RetainedCloneGrant{let copy=self.next_close_copy_byte_demand().unwrap();RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:self.next_close_release_byte_demand().unwrap(),maximum_depth:self.next_close_depth_demand().unwrap()}}fn step(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.close_step(grant).unwrap()}fn empty(&self)->bool{self.terminal_is_empty()}}