//! 🎟️ Native original sealed-owner families count the terminal leaf visits of each aggregate demand.
use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement},shared_issuance::{SealedShared,SharedIssuer}};
use std::{cell::Cell,mem::size_of};
thread_local!{static VISITS:Cell<usize>=const{Cell::new(0)};static HINT:Cell<usize>=const{Cell::new(0)};static FOREIGN_HINTS:Cell<usize>=const{Cell::new(0)};}
struct Probe;
struct ProbeCursor{present:bool}
impl RetireOwned for Probe{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(ProbeCursor{present:true})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ProbeCursor>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for ProbeCursor{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{let copy_bytes=self.next_work_byte_demand()?;self.next_birth_bytes(body).unwrap();Ok(RetirementDemand{copy_bytes,depth:usize::from(self.present),..Default::default()})}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{if g.maximum_items==0||g.maximum_copy_bytes<1||g.maximum_depth==0{return RetirementStep::BudgetExhausted;}self.present=false;RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})}
 fn terminal_is_empty(&self)->bool{!self.present}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{VISITS.with(|n|n.set(n.get()+1));Ok(usize::from(self.present))}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{HINT.with(|expected|if body!=expected.get(){FOREIGN_HINTS.with(|n|n.set(n.get()+1));});Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.present))}
 fn terminal_release_bytes(&self)->Option<usize>{(!self.present).then_some(size_of::<Self>())}
}
#[derive(crate::RetireOwned)]
enum Family{Leaf(Probe),Branch(SealedShared<Family>)}
fn grant(d:RetirementDemand,body:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn quote(owner:&ControlledRetirement<Family>,body:usize)->(RetirementDemand,usize,usize){VISITS.with(|n|n.set(0));HINT.with(|n|n.set(body));FOREIGN_HINTS.with(|n|n.set(0));let(d,heap)=crate::observe_retirement_allocations(||ErasedSnapshotRetirement::next_demand(owner,body).unwrap());assert_eq!(heap,(0,0));(d,VISITS.with(Cell::get),FOREIGN_HINTS.with(Cell::get))}
#[test]
fn original_retirement_combined_quote_visits_terminal_leaf_once_with_actual_body_hint(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let mut obeys=true;
 for family in law["families"].as_array().unwrap(){
  let depth=family["sealedDepth"].as_u64().unwrap()as usize;let mut value=Family::Leaf(Probe);let(mut born,mut released)=(0,0);
  for _ in 0..depth{let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:4096};let((alias,p),heap)=crate::observe_retirement_allocations(||SealedShared::admit(value,SharedIssuer::owned(),g).unwrap_or_else(|_|panic!("original family issuance")));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;released+=heap.1;value=Family::Branch(alias);}
  let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original family owner"));let mut reached=false;
  for _ in 0..10000{let(d,visits,_)=quote(&owner,64);if visits>0{reached=true;break;}let g=grant(d,64);let(step,heap)=crate::observe_retirement_allocations(||owner.step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
  assert!(reached,"actual original terminal leaf frontier must be reached");
  for body in law["bodyHints"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize){let(_,visits,foreign)=quote(&owner,body);assert!(!owner.terminal_is_empty());obeys&=visits==law["expected"]["leafVisitsPerQuote"].as_u64().unwrap()as usize&&foreign==0;}
  for turn in 0..10000{if owner.terminal_is_empty(){break;}let(d,_,_)=quote(&owner,64);let g=grant(d,64);let mut denials=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes>0{denials.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g});}if d.capacity_bytes>0{denials.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g});}if d.release_bytes>0{denials.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g});}if d.depth>0{denials.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g});}
   for denied in denials{let(result,heap)=crate::observe_retirement_allocations(||owner.step(denied));assert_eq!(heap,(0,0));if let Ok(step)=result{assert_eq!(step.progress(),Default::default());}assert_eq!(quote(&owner,64).0,d);}
   let(step,heap)=crate::observe_retirement_allocations(||owner.step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;assert!(turn<9999,"original family retirement did not finish");
  }
  assert!(owner.terminal_is_empty());assert_eq!(born,released);assert_eq!(crate::observe_retirement_allocations(||drop(owner)).1,(0,0));
 }
 assert!(obeys,"every combined original retirement quote visits its actual leaf once and propagates its original body hint");
}