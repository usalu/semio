//! 👁️ Lift file: the original child read retirement and its `RetireOwned for ChildContentView` impl, of `🧩️composition/♻️root/👁️read/🦀️.rs`, that belong to crate `decl-views` (WP2).
//!
//! WP3 owns the source file and cuts it here: the impl names `ChildContentView` (a `decl-views` type) and `RetireOwned` is foreign, so the orphan rule keeps the impl with the type; the plan listed the file under `composition-emission`, whose crate could not host it. The parent `📜️declaration/🪆️child/🦀️.rs` mounted the file by `include!`; it now mounts this lift as a child module. Not in Appendix J.
use super::*;

use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::{RetainedAllocationEffect,RetirementPollExit};
use crate::child_view::pay_work;
use std::mem::ManuallyDrop;
/// 👁️ Returns exact child reads to their original registries before releasing final view backing.
pub struct ChildContentReadRetirement {
 view:ManuallyDrop<ChildContentView>,
 pending:ManuallyDrop<Option<ChildContentEntry>>,
 returned:ManuallyDrop<Option<ControlledRetirement<store::os_store::snapshot_read::SnapshotReadReturn>>>,
 metadata:ManuallyDrop<Option<ControlledRetirement<ChildContentReadMetadata>>>,
}

#[derive(semio_framework_value::RetireOwned)]
struct ChildContentReadMetadata {owner:String,slot:String,child_id:String,artifact_id:String,dialect:ArtifactDialect,revision:[u8;32]}

/// 🪢️ Aliases another live holder keeps (the app's own children view) are returned by dropping this alias, never by waiting for sole ownership; only the last owner pays the deep release.
///
/// 🧵️ Sound because every holder of these roots belongs to one app instance and is only touched on that instance's actor turn thread (the instance owns its children view and the captured context under `&mut self`; the worker session thread never holds a child view), so no other thread can drop the other holder between the `strong_count` check and the alias drop. Wasm is single-threaded and the native fixture drives one thread. If a holder ever crosses threads, replace the count check with an atomic `Arc::into_inner` ownership transfer.
struct ChildContentReadAliases;
impl ChildContentRetainers for ChildContentReadAliases {
 fn retains_root(&self,view:&ChildContentView)->bool{view.root.as_ref().is_some_and(|root|std::sync::Arc::strong_count(root)>1)}
 fn retains_page(&self,_index:usize,page:&std::sync::Arc<ChildContentPage>)->bool{std::sync::Arc::strong_count(page)>1}
 fn retains_entry(&self,entry:&std::sync::Arc<ChildContentEntry>)->bool{std::sync::Arc::strong_count(entry)>1}
}

impl ChildContentReadRetirement {
 pub fn new(view:ChildContentView)->Self{Self{view:ManuallyDrop::new(view),pending:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),metadata:ManuallyDrop::new(None)}}
 fn child_demands<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,copy:usize)->Result<RetirementDemand,ValueError>{if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}let mut demand=owner.next_demand(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original child read depth overflow"))?;Ok(demand)}
 pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.returned.as_ref(){return Self::child_demands(owner,copy)}
  if let Some(owner)=self.metadata.as_ref(){return Self::child_demands(owner,copy)}
  if self.pending.is_some(){return Ok(store::os_store::snapshot_read::ErasedSnapshotRead::prepared_return_demands())}
  Ok(RetirementDemand{release_bytes:self.view.next_take_release_bytes(),depth:usize::from(!self.terminal_is_empty()),..Default::default()})
 }
 pub fn work_demand(&self,body:usize)->Result<u64,ValueError>{
  if self.terminal_is_empty(){return Ok(0)}
  for owner in [self.returned.as_ref().map(|owner|(owner.terminal_is_empty(),owner.next_computational_work(body))),self.metadata.as_ref().map(|owner|(owner.terminal_is_empty(),owner.next_computational_work(body)))].into_iter().flatten(){return if owner.0{Ok(1)}else{owner.1}}
  Ok(1)
 }
 pub fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  let grant=effect.grant();let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}let d=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(RetainedCloneStep::Progress(empty))}
  if let Some(owner)=self.returned.as_mut(){if owner.terminal_is_empty(){pay_work(effect,1)?;self.returned.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}return RetirementPollExit::resume(owner.step(body,&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}))).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(owner)=self.metadata.as_mut(){if owner.terminal_is_empty(){pay_work(effect,1)?;self.metadata.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}return RetirementPollExit::resume(owner.step(body,&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}))).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(entry)=self.pending.take(){let ChildContentEntry{owner,slot,child_id,artifact_id,dialect,revision,snapshot}=entry;match snapshot.try_return_to_registry_witness(effect){Ok((returned,progress))=>{*self.returned=Some(ControlledRetirement::new(returned).unwrap_or_else(|_|unreachable!("original returned read defines full closure")));*self.metadata=Some(ControlledRetirement::new(ChildContentReadMetadata{owner,slot,child_id,artifact_id,dialect,revision}).unwrap_or_else(|_|unreachable!("original child read metadata defines full closure")));return Ok(RetainedCloneStep::Progress(progress))},Err((error,snapshot))=>{*self.pending=Some(ChildContentEntry{owner,slot,child_id,artifact_id,dialect,revision,snapshot});return Err(error.into())}}}
  pay_work(effect,1)?;
  match self.view.take_one(&ChildContentView::EMPTY,&ChildContentReadAliases){ChildContentTake::Blocked=>Ok(RetainedCloneStep::Progress(empty)),ChildContentTake::Snapshot(entry,released_bytes)=>{*self.pending=Some(entry);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..empty}))},ChildContentTake::Released(released_bytes)=>Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..empty})),ChildContentTake::Complete(released_bytes)=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes,..empty}))}
 }
 pub fn terminal_is_empty(&self)->bool{self.view.root.is_none()&&self.pending.is_none()&&self.returned.is_none()&&self.metadata.is_none()}
}
impl semio_framework_value::retirement::RetirementCursor for ChildContentReadRetirement {
 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.work_demand(body)}
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(maximum_body_bytes)}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl ChildContentReadRetirement {
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value::retirement::RetirementStep{
  use semio_framework_value::retirement::RetirementStep;
  match self.step(body,effect){Err(RetirementPollExit::Pending(work))=>RetirementStep::Pending(work),Err(RetirementPollExit::Failed(failure))=>RetirementStep::Failure(failure,effect.performed_work()),Ok(RetainedCloneStep::Progress(p)|RetainedCloneStep::Complete(p))if p!=Default::default()||effect.performed_work()!=0=>RetirementStep::Progress(p,effect.performed_work()),Ok(RetainedCloneStep::Complete(_))=>RetirementStep::Complete(effect.performed_work()),Ok(_)=>RetirementStep::BudgetExhausted}
 }
}
impl semio_framework_value::retirement::RetireOwned for ChildContentView {
 type RetirementCursor=ChildContentReadRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{ChildContentReadRetirement::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ChildContentReadRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl Drop for ChildContentReadRetirement{fn drop(&mut self){assert!(self.terminal_is_empty(),"original child view abandoned exact read registry custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.view);ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.returned);ManuallyDrop::drop(&mut self.metadata);}}}}

#[cfg(test)]
mod shared_root_laws{
 use super::*;
 /// 👁️ LAW: a child view that merely aliases a root another live holder keeps retires by returning its alias in bounded turns; only the last owner pays the deep release, and the live holder keeps its root.
 #[test]
 fn a_view_aliasing_a_live_root_retires_by_dropping_its_alias_in_bounded_turns(){
  let root=std::sync::Arc::new(ChildContentRoot::default());
  let live=ChildContentView{root:Some(root.clone())};
  let mut retirement=ChildContentReadRetirement::new(ChildContentView{root:Some(root.clone())});
  assert_eq!(std::sync::Arc::strong_count(&root),3);
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:1<<20,maximum_depth:4};
  for _ in 0..8{if retirement.terminal_is_empty(){break}retirement.step(grant).expect("bounded alias return");}
  assert!(retirement.terminal_is_empty());
  assert_eq!(std::sync::Arc::strong_count(&root),2);
  assert!(live.root.is_some());
 }
}
