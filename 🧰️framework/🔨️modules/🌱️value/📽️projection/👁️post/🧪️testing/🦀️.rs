//! 🧪️ Native central tests lend the actual post and restore it with the same original funded custody owner.
use semio_framework_value::{RetirementDemand,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneRef,RetainedCloneGrant}};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{OriginalPostRead,PagedOneItemPostMode};
use std::mem::ManuallyDrop;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub struct PostFixture<S:RetireOwned+Sync>{held_post:ManuallyDrop<Option<S>>,original_post_read:OriginalPostRead<S>}
impl<S:RetireOwned+Sync> PostFixture<S>{
 pub fn projected<O:semio_framework_value_snapshot_projection::snapshot_projection::ProjectionOutput<S>>(output:O)->Self{let d=O::placement_demand();let mut target=std::mem::MaybeUninit::<S>::uninit();let p=output.place(&mut target,grant(d)).result.unwrap_or_else(|_|panic!("test projected output original placement"));assert!(p.fits(grant(d)));Self::new(unsafe{target.assume_init()})}
 pub fn new(value:S)->Self{let mut owner=Self{held_post:ManuallyDrop::new(Some(value)),original_post_read:Default::default()};owner.prepare(PagedOneItemPostMode::Original);owner}
 fn prepare(&mut self,mode:PagedOneItemPostMode){for _ in 0..100000{let Some(d)=self.original_post_read.demand(&self.held_post,mode,4096).unwrap()else{return;};let p=eff::with_effect(grant(d),|e|self.original_post_read.prepare(&mut self.held_post,mode,4096,e)).result.unwrap().expect("selected original post frontier");assert!(p.fits(grant(d)));}panic!("test post original handback stalled");}
 pub fn turn_demand(&self,mode:PagedOneItemPostMode,body:usize)->Result<Option<RetirementDemand>,semio_framework_value::ValueError>{self.original_post_read.demand(&self.held_post,mode,body)}
 pub fn read_mode(&self,mode:PagedOneItemPostMode)->Result<semio_framework_value_snapshot_projection::snapshot_projection::post::PagedOneItemPostRead<'_,S>,semio_framework_value::ValueError>{self.original_post_read.read(&self.held_post,mode)}
 pub fn prepare_turn(&mut self,mode:PagedOneItemPostMode,body:usize,grant:RetainedCloneGrant)->Result<Option<semio_framework_value::retained_clone::RetainedCloneProgress>,semio_framework_value::ValueError>{eff::with_effect(grant,|e|self.original_post_read.prepare(&mut self.held_post,mode,body,e)).result.map_err(|failure|failure.primary().or(failure.work_refusal()).or(failure.receipt_refusal()).cloned().unwrap_or_else(||semio_framework_value::ValueError::invariant("test post preparation failed without a value cause")))}
 pub fn access(&mut self,mode:PagedOneItemPostMode)->Result<semio_framework_value_snapshot_projection::snapshot_projection::post::PagedOneItemPost<'_,S>,semio_framework_value::ValueError>{self.original_post_read.access(&mut self.held_post,mode)}
 pub fn borrow(&self)->RetainedCloneRef<'_,S>{self.original_post_read.read(&self.held_post,PagedOneItemPostMode::Original).unwrap().original().unwrap()}
 pub fn mutable_read(&mut self)->&S{self.prepare(PagedOneItemPostMode::Mutable);self.held_post.as_ref().unwrap()}
 pub fn into_inner(mut self)->S{self.prepare(PagedOneItemPostMode::Mutable);self.held_post.take().unwrap()}
}
impl<S:RetireOwned+Sync>std::ops::Deref for PostFixture<S>{type Target=S;fn deref(&self)->&S{if self.original_post_read.terminal_is_empty(){self.held_post.as_ref().unwrap()}else{self.borrow().get()}}}
impl<S:RetireOwned+Sync>std::ops::DerefMut for PostFixture<S>{fn deref_mut(&mut self)->&mut S{assert!(self.original_post_read.terminal_is_empty(),"mutable test post requires explicit funded handback before observed commit");self.held_post.as_mut().unwrap()}}
impl<S:RetireOwned+Sync+std::fmt::Debug>std::fmt::Debug for PostFixture<S>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Debug::fmt(&**self,f)}}
impl<S:RetireOwned+Sync+PartialEq>PartialEq<S>for PostFixture<S>{fn eq(&self,other:&S)->bool{&**self==other}}
impl<S:RetireOwned+Sync>Drop for PostFixture<S>{fn drop(&mut self){if std::thread::panicking(){return;}if self.held_post.is_none()&&self.original_post_read.terminal_is_empty(){return;}self.prepare(PagedOneItemPostMode::Mutable);if let Some(value)=self.held_post.take(){let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("test post original typed retirement"));for _ in 0..100000{if close.terminal_is_empty(){return;}let d=close.next_demand(4096).unwrap();let p=eff::settled(eff::run(grant(d),|e|close.step(e))).result.unwrap().progress();assert!(p.fits(grant(d)));}panic!("test post original retirement stalled");}}}
/// ⏱️ Local fixture effect: one actual original Work action around each post preparation turn.
mod eff {
 use semio_framework_value::retained_clone::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink,RetainedCloneGrant};
 use semio_framework_value::{RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
 const CPU:u64=1<<20;
 struct Producer;
 impl OriginalWorkRecipient for Producer{
  type Context<'turn,'source>=() where 'source:'turn;
  fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
  fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
  fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=CPU)))}
  fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
  fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 }
 pub fn run<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
  let mut authority=OriginalWorkAuthority::new(Producer);let mut actual=None;
  let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(CPU,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,CPU,&mut actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().expect("original fixture Work partial outcome");assert_eq!(partial.total(),performed);assert!(partial.source_refusal().is_none());assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}output}).unwrap();
  assert!(!authority.has_live_actions());result
 }
 pub fn with_effect<T,E>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,E>{run(grant,operation)}
 pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
}
