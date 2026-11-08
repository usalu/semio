//! 🪑️ Concrete primitive cursors preserve the standard allocator's original release conservation.
use semio_framework_value_retirement::{RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value_retirement::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
pub use semio_framework_value_retirement::{RetirementDemand,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
pub use semio_framework_value::retained_clone;
pub(crate) use semio_framework_value_testing_allocator as allocation_testing;
#[path="../⏱️effect/🦀️.rs"]
mod effect;
struct InlineFields{text:String,numbers:Vec<u64>}
semio_framework_value_retirement::artifact_retire_struct!(InlineFields{text,numbers});
struct Bank{grant:RetainedCloneGrant,work:u128,turns:usize,maximum:usize,born:usize,released:usize,actual:Option<usize>}
impl Bank{
 fn close(&mut self,cursor:&mut dyn RetirementCursor){
  loop{if self.actual.is_some(){assert!(self.work>0);self.work-=1;self.actual=None;self.post(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<usize>>(),..Default::default()});continue}if cursor.terminal_is_empty(){break}
   assert!(self.turns<self.maximum);self.turns+=1;
   let d=cursor.next_demand(4096).unwrap();let q=cursor.next_computational_work(4096).unwrap();let q=u128::from(q);assert!(q<=self.work);let g=self.grant;assert!(d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth);
   let(step,heap)=effect::with_effect(RetainedCloneGrant{maximum_items:1,..g},effect::WORK,&mut self.actual,|e|observe_retirement_allocations(||cursor.close_step(e)));assert_eq!(self.actual.unwrap_or(0),heap.0);
   let work=step.work();assert!(work<=q);self.work-=work;self.born+=heap.0;self.released+=heap.1;
   let(progress,child)=match step{RetirementStep::Progress(p,_)=>(p,None),RetirementStep::ChildProgress(child,p,_)=>(p,Some(child)),RetirementStep::Child(child)=>(Default::default(),Some(child)),RetirementStep::Bytes(n)=>(RetainedCloneProgress{copied_items:1,released_bytes:n,..Default::default()},None),RetirementStep::ProcessedBytes(n)=>(RetainedCloneProgress{copied_items:1,copied_bytes:n,..Default::default()},None),RetirementStep::Advanced=>(RetainedCloneProgress{copied_items:1,..Default::default()},None),RetirementStep::Complete(_)=>(Default::default(),None),RetirementStep::Failure(error,_)=>panic!("original inline retirement refused: {error:?}"),RetirementStep::BudgetExhausted=>panic!("quoted original inline retirement did not advance"),RetirementStep::Pending(..)=>panic!("original inline retirement stayed pending")};
   assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);self.post(progress);
   if let Some(mut child)=child{self.close(child.as_mut());let release=child.terminal_release_bytes().unwrap();assert!(release<=self.grant.maximum_release_bytes);assert!(self.work>0);self.work-=1;let(_,heap)=observe_retirement_allocations(||drop(child));assert_eq!(heap,(0,release));self.released+=release;self.post(RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()});}
  }
 }
 fn post(&mut self,p:RetainedCloneProgress){assert!(p.fits(self.grant));self.grant.maximum_items-=p.copied_items;self.grant.maximum_copy_bytes-=p.copied_bytes;self.grant.maximum_capacity_bytes-=p.retained_capacity_bytes;self.grant.maximum_release_bytes-=p.released_bytes;}
}
fn check<T:RetireOwned>(factory:impl Fn()->T,law:&serde_json::Value){
 let oracle=factory();let(_,expected)=observe_retirement_allocations(||drop(oracle));assert_eq!(expected.0,0);
 let original=factory();let(mut cursor,heap)=observe_retirement_allocations(||original.retirement_seed());assert_eq!(heap,(0,0));
 let stock=&law["stock"];let n=|k|stock[k].as_u64().unwrap()as usize;let mut bank=Bank{grant:RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copyBytes"),maximum_capacity_bytes:n("capacityBytes"),maximum_release_bytes:n("releaseBytes"),maximum_depth:n("depth")},work:n("work")as u128,turns:0,maximum:law["maximumTurns"].as_u64().unwrap()as usize,born:0,released:0,actual:None};
 bank.close(&mut cursor);assert_eq!(bank.released.checked_sub(bank.born).unwrap(),expected.1);assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
}
#[test]fn primitive_inline_cursors_allocate_only_on_funded_original_child_turns(){
 let law:serde_json::Value=serde_json::from_str(include_str!("📐️.json")).unwrap();assert_eq!(law["text"].as_str().unwrap().len(),law["expected"]["utf8Bytes"].as_u64().unwrap()as usize);
 for kind in law["kinds"].as_array().unwrap(){match kind.as_str().unwrap(){"scalar"=>check(||73_u64,&law),"utf8"=>check(||law["text"].as_str().unwrap().to_owned(),&law),"vector"=>check(||serde_json::from_value::<Vec<u64>>(law["numbers"].clone()).unwrap(),&law),"tuple"=>check(||(law["text"].as_str().unwrap().to_owned(),73_u64),&law),"array"=>check(||[3_u64,5,8],&law),"struct"=>check(||InlineFields{text:law["text"].as_str().unwrap().to_owned(),numbers:serde_json::from_value(law["numbers"].clone()).unwrap()},&law),_=>unreachable!()}}
}
