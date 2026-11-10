//! 🧪️ Native central tests capture the actual post once and lend or return it through the same original funded custody owner.
use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,ValueError,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
use semio_framework_value_clone_source_testing::drive::{drive,drive_reply};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{OriginalPostRead,PagedOneItemPost,PagedOneItemPostMode,PagedOneItemPostRead};
const UNITS:u64=1<<20;
const BODY:usize=4096;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
trait Failure{fn progress(&self)->RetainedCloneProgress;}
impl Failure for ValueError{fn progress(&self)->RetainedCloneProgress{self.retained_progress()}}
impl Failure for OriginalRetirementFailure{fn progress(&self)->RetainedCloneProgress{self.retained_progress()}}
fn progress_of<E:Failure>(answer:&Result<RetainedCloneStep,E>)->RetainedCloneProgress{match answer{Ok(step)=>step.progress(),Err(failure)=>failure.progress()}}
fn fault(reason:&'static str)->ValueError{ValueError::invariant(reason)}
type Reply<'r,E>=OriginalEffectReply<'r,RetainedCloneStep,E>;
type Done<'r,E>=Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>;
pub struct PostFixture<S:RetireOwned+Sync+'static>{original_post_read:OriginalPostRead<S>}
impl<S:RetireOwned+Sync+'static> PostFixture<S>{
 pub fn new(value:S)->Self{
  let mut owner=Self{original_post_read:Default::default()};let mut input=Some(value);
  let capture=drive_reply::<RetainedCloneStep,ValueError>(grant(OriginalPostRead::<S>::capture_demand()),UNITS,progress_of,|effect,reply:Reply<'_,ValueError>,recover|->Done<'_,ValueError>{if recover{return Err(fault("test post capture has no unrecorded callback"))}owner.original_post_read.capture_into(&mut input,effect,reply)});
  capture.result.unwrap_or_else(|_|panic!("test post original capture"));assert!(input.is_none(),"test post capture kept its input");
  owner.prepare(PagedOneItemPostMode::Original);owner
 }
 fn prepare_once(&mut self,mode:PagedOneItemPostMode,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalRetirementFailure>{
  drive_reply::<RetainedCloneStep,OriginalRetirementFailure>(g,UNITS,progress_of,|effect,reply:Reply<'_,OriginalRetirementFailure>,recover|->Done<'_,OriginalRetirementFailure>{if recover{self.original_post_read.recover_prepare_into(mode,body,effect,reply)}else{self.original_post_read.prepare_into(mode,body,effect,reply)}}).result
 }
 fn prepare(&mut self,mode:PagedOneItemPostMode){for _ in 0..100000{let Some(d)=self.original_post_read.demand(mode,BODY).unwrap()else{return};let step=self.prepare_once(mode,BODY,grant(d)).unwrap_or_else(|_|panic!("test post original preparation"));assert!(step.progress().fits(grant(d)));}panic!("test post original handback stalled");}
 pub fn turn_demand(&self,mode:PagedOneItemPostMode,body:usize)->Result<Option<RetirementDemand>,ValueError>{self.original_post_read.demand(mode,body)}
 pub fn read_mode(&self,mode:PagedOneItemPostMode)->Result<PagedOneItemPostRead<'_,S>,ValueError>{self.original_post_read.read(mode)}
 pub fn prepare_turn(&mut self,mode:PagedOneItemPostMode,body:usize,g:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  if self.original_post_read.demand(mode,body)?.is_none(){return Ok(None)}
  self.prepare_once(mode,body,g).map(|step|Some(step.progress())).map_err(|failure|failure.primary().or(failure.work_refusal()).or(failure.receipt_refusal()).cloned().unwrap_or_else(||fault("test post preparation failed without a value cause")))
 }
 pub fn access(&mut self,mode:PagedOneItemPostMode)->Result<PagedOneItemPost<'_,S>,ValueError>{self.original_post_read.access(mode)}
 pub fn borrow(&self)->RetainedCloneRef<'_,S>{self.original_post_read.read(PagedOneItemPostMode::Original).unwrap().original().unwrap()}
 pub fn mutable_read(&mut self)->&S{self.prepare(PagedOneItemPostMode::Mutable);self.original_post_read.read(PagedOneItemPostMode::Mutable).unwrap().get()}
 fn give_back(&mut self)->Option<S>{
  if self.original_post_read.terminal_is_empty(){return None}
  self.prepare(PagedOneItemPostMode::Mutable);let mut output=None;
  let back=drive_reply::<RetainedCloneStep,ValueError>(grant(self.original_post_read.return_demand()),UNITS,progress_of,|effect,reply:Reply<'_,ValueError>,recover|->Done<'_,ValueError>{if recover{return Err(fault("test post return has no unrecorded callback"))}self.original_post_read.return_into(&mut output,effect,reply)});
  back.result.unwrap_or_else(|_|panic!("test post original return"));output
 }
 pub fn into_inner(mut self)->S{self.give_back().expect("test post is resident")}
}
impl<S:RetireOwned+Sync+'static> std::ops::Deref for PostFixture<S>{type Target=S;fn deref(&self)->&S{self.original_post_read.read(PagedOneItemPostMode::Mutable).or_else(|_|self.original_post_read.read(PagedOneItemPostMode::Original)).expect("test post is resident").get()}}
impl<S:RetireOwned+Sync+'static> std::ops::DerefMut for PostFixture<S>{fn deref_mut(&mut self)->&mut S{assert!(self.original_post_read.demand(PagedOneItemPostMode::Mutable,BODY).unwrap().is_none(),"mutable test post requires explicit funded handback before observed commit");self.original_post_read.access(PagedOneItemPostMode::Mutable).unwrap().mutable().unwrap()}}
impl<S:RetireOwned+Sync+std::fmt::Debug+'static> std::fmt::Debug for PostFixture<S>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Debug::fmt(&**self,f)}}
impl<S:RetireOwned+Sync+PartialEq+'static> PartialEq<S> for PostFixture<S>{fn eq(&self,other:&S)->bool{&**self==other}}
impl<S:RetireOwned+Sync+'static> Drop for PostFixture<S>{
 fn drop(&mut self){
  if std::thread::panicking(){return}
  let Some(value)=self.give_back()else{return};
  let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("test post original typed retirement"));
  for _ in 0..100000{if close.terminal_is_empty(){return}let d=close.next_demand(BODY).unwrap();let step=drive(grant(d),UNITS,|effect|close.step(BODY,effect)).result.unwrap_or_else(|_|panic!("test post original retirement"));assert!(step.progress().fits(grant(d)));}
  panic!("test post original retirement stalled");
 }
}
