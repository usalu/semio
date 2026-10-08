//! 📸️ Optional text candidates retain paged snapshots and retire replaced field owners before publication.
use super::{ChangeNodeIcon,EditTargetRegionLabel,Puzzle2dTextIntent,Puzzle2dTextDisposition,Puzzle2dTextPlan,Puzzle2dTextPreparationCursor,Puzzle2dTextPreparationStep,Puzzle2dSnapshot};
use semio_framework_value::{paged::PagedUtf8,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement,retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedFieldCursor}};
use std::{mem::{ManuallyDrop,size_of},ops::{Deref,DerefMut}};
type OptionalText=Option<PagedUtf8<{usize::MAX}>>;
pub trait Puzzle2dTextCandidateIntent:Puzzle2dTextIntent+Send{fn replace(snapshot:&mut Puzzle2dSnapshot,index:usize,next:OptionalText)->OptionalText;}
impl Puzzle2dTextCandidateIntent for ChangeNodeIcon{fn replace(snapshot:&mut Puzzle2dSnapshot,index:usize,next:OptionalText)->OptionalText{std::mem::replace(&mut snapshot.nodes.get_mut(index).expect("original icon candidate ordinal").icon_kind,next)}}
impl Puzzle2dTextCandidateIntent for EditTargetRegionLabel{fn replace(snapshot:&mut Puzzle2dSnapshot,index:usize,next:OptionalText)->OptionalText{std::mem::replace(&mut snapshot.target_regions.get_mut(index).expect("original label candidate ordinal").label,next)}}
pub struct Puzzle2dTextCandidate{pub plan:Puzzle2dTextPlan,pub snapshot:Option<Puzzle2dSnapshot>}
pub struct Puzzle2dTextCandidateCursor<T:Puzzle2dTextCandidateIntent>{state:ManuallyDrop<Puzzle2dTextCandidateState<T>>}
#[doc(hidden)]
pub struct Puzzle2dTextCandidateState<T:Puzzle2dTextCandidateIntent>{source:Option<RetainedCloneBinding>,mutation:Option<RetainedCloneBinding>,preparation:Puzzle2dTextPreparationCursor<T>,snapshot_clone:RetainedFieldCursor<Puzzle2dSnapshot>,next:<OptionalText as RetainedClone>::Cursor,plan:Option<Puzzle2dTextPlan>,candidate:Option<Puzzle2dSnapshot>,displaced:Option<OptionalText>,displaced_close:Option<ControlledRetirement<OptionalText>>,candidate_close:Option<ControlledRetirement<Puzzle2dSnapshot>>,phase:u8,closing:bool}
impl<T:Puzzle2dTextCandidateIntent>Deref for Puzzle2dTextCandidateCursor<T>{type Target=Puzzle2dTextCandidateState<T>;fn deref(&self)->&Self::Target{&self.state}}
impl<T:Puzzle2dTextCandidateIntent>DerefMut for Puzzle2dTextCandidateCursor<T>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.state}}
impl<T:Puzzle2dTextCandidateIntent>Default for Puzzle2dTextCandidateCursor<T>{fn default()->Self{Self{state:ManuallyDrop::new(Puzzle2dTextCandidateState{source:None,mutation:None,preparation:Default::default(),snapshot_clone:Default::default(),next:OptionalText::retained_clone_cursor(),plan:None,candidate:None,displaced:None,displaced_close:None,candidate_close:None,phase:0,closing:false})}}}
impl<T:Puzzle2dTextCandidateIntent>Puzzle2dTextCandidateCursor<T>{
 pub fn advance(&mut self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,mutation:RetainedCloneRef<'_,T>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.closing||self.phase==12{return Err(refusal("optional text candidate is closing or spent"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}source.bind(&mut self.source)?;mutation.bind(&mut self.mutation)?;
  match self.phase{
   0=>{let step=self.preparation.advance(source,mutation,grant)?;let progress=match step{Puzzle2dTextPreparationStep::Pending(progress)=>progress,Puzzle2dTextPreparationStep::Complete{plan,progress}=>{self.plan=self.preparation.take();if self.plan!=Some(plan){return Err(refusal("optional candidate lost its original plan"))}self.preparation.begin_close();self.phase=1;progress}};Ok(RetainedCloneStep::Progress(progress))}
   1=>{let step=self.preparation.close_step(grant)?;if self.preparation.terminal_is_empty(){self.phase=if self.plan.is_some_and(|plan|plan.disposition==Puzzle2dTextDisposition::Changed){2}else{10}}Ok(RetainedCloneStep::Progress(step.progress()))}
   2=>{let step=self.snapshot_clone.advance(source,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=3}Ok(RetainedCloneStep::Progress(step.progress()))}
   3=>{if grant.maximum_copy_bytes<size_of::<Puzzle2dSnapshot>(){return Ok(RetainedCloneStep::Progress(Default::default()))}self.candidate=Some(self.snapshot_clone.take().ok_or_else(||refusal("optional candidate lost original snapshot clone"))?);self.snapshot_clone.begin_close();self.phase=4;Ok(payload(size_of::<Puzzle2dSnapshot>()))}
   4=>{let step=self.snapshot_clone.close_step(grant)?;if self.snapshot_clone.terminal_is_empty(){self.phase=5}Ok(RetainedCloneStep::Progress(step.progress()))}
   5=>{let step=self.next.advance(mutation.project(15,T::next),grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=6}Ok(RetainedCloneStep::Progress(step.progress()))}
   6=>{if grant.maximum_copy_bytes<2*size_of::<OptionalText>(){return Ok(RetainedCloneStep::Progress(Default::default()))}let index=self.plan.and_then(|plan|plan.index).ok_or_else(||refusal("changed text candidate has no native ordinal"))?;let next=self.next.take().ok_or_else(||refusal("optional candidate lost incoming native text"))?;self.displaced=Some(T::replace(self.candidate.as_mut().ok_or_else(||refusal("optional candidate lost native snapshot"))?,index,next));self.next.begin_close();self.phase=7;Ok(payload(2*size_of::<OptionalText>()))}
   7=>{let step=self.next.close_step(grant)?;if self.next.terminal_is_empty(){self.phase=8}Ok(RetainedCloneStep::Progress(step.progress()))}
   8=>{if let Some(owner)=self.displaced_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.displaced_close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}if self.displaced.is_some(){return self.handoff_displaced(grant)}self.phase=10;Ok(payload(0))}
   10=>{if grant.maximum_copy_bytes<size_of::<Puzzle2dTextCandidate>(){return Ok(RetainedCloneStep::Progress(Default::default()))}self.phase=11;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Puzzle2dTextCandidate>(),retained_capacity_bytes:0,released_bytes:0}))}
   11=>Ok(RetainedCloneStep::Complete(Default::default())),_=>Err(refusal("optional text candidate phase is invalid"))
  }
 }
 pub fn take(&mut self)->Option<Puzzle2dTextCandidate>{if self.closing||self.phase!=11{return None}self.phase=12;Some(Puzzle2dTextCandidate{plan:self.plan.take()?,snapshot:self.candidate.take()})}
 pub fn begin_close(&mut self){self.closing=true;self.preparation.begin_close();self.snapshot_clone.begin_close();self.next.begin_close();}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{
  if !self.preparation.terminal_is_empty(){return self.preparation.next_close_copy_byte_demand()}
  if !self.snapshot_clone.terminal_is_empty(){return self.snapshot_clone.next_close_copy_byte_demand()}
  if !self.next.terminal_is_empty(){return self.next.next_close_copy_byte_demand()}
  if let Some(owner)=self.displaced_close.as_ref(){return owner.next_copy_byte_demand()}
  if self.displaced.is_some(){return Ok(size_of::<OptionalText>())}
  if let Some(owner)=self.candidate_close.as_ref(){return owner.next_copy_byte_demand()}
  if self.candidate.is_some(){return Ok(size_of::<Puzzle2dSnapshot>())}
  if self.plan.is_some(){return Ok(size_of::<Option<Puzzle2dTextPlan>>())}
  RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.mutation})
 }
 pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{
  if !self.preparation.terminal_is_empty(){return self.preparation.next_close_capacity_byte_demand(body)}
  if !self.snapshot_clone.terminal_is_empty(){return self.snapshot_clone.next_close_capacity_byte_demand(body)}
  if !self.next.terminal_is_empty(){return self.next.next_close_capacity_byte_demand(body)}
  if let Some(owner)=self.displaced_close.as_ref(){return owner.next_capacity_byte_demand(body)}
  if self.displaced.is_some(){return Ok(0)}
  if let Some(owner)=self.candidate_close.as_ref(){return owner.next_capacity_byte_demand(body)}
  if self.candidate.is_some(){return Ok(0)}
  if self.plan.is_some(){return Ok(0)}
  RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.mutation},body)
 }
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{
  if !self.preparation.terminal_is_empty(){return self.preparation.next_close_release_byte_demand()}
  if !self.snapshot_clone.terminal_is_empty(){return self.snapshot_clone.next_close_release_byte_demand()}
  if !self.next.terminal_is_empty(){return self.next.next_close_release_byte_demand()}
  if let Some(owner)=self.displaced_close.as_ref(){return owner.next_release_byte_demand()}
  if self.displaced.is_some(){return Ok(0)}
  if let Some(owner)=self.candidate_close.as_ref(){return owner.next_release_byte_demand()}
  if self.candidate.is_some(){return Ok(0)}
  if self.plan.is_some(){return Ok(0)}
  RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.mutation})
 }
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{
  if !self.preparation.terminal_is_empty(){return self.preparation.next_close_depth_demand()}
  if !self.snapshot_clone.terminal_is_empty(){return self.snapshot_clone.next_close_depth_demand()}
  if !self.next.terminal_is_empty(){return self.next.next_close_depth_demand()}
  if let Some(owner)=self.displaced_close.as_ref(){return owner.next_depth_demand()}
  if self.displaced.is_some(){return Ok(1)}
  if let Some(owner)=self.candidate_close.as_ref(){return owner.next_depth_demand()}
  if self.candidate.is_some(){return Ok(1)}
  if self.plan.is_some(){return Ok(1)}
  RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.mutation})
 }
 fn handoff_displaced(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_copy_bytes<size_of::<OptionalText>(){return Ok(RetainedCloneStep::Progress(Default::default()))}match ControlledRetirement::new(self.displaced.take().unwrap()){Ok(owner)=>self.displaced_close=Some(owner),Err((error,owner))=>{self.displaced=Some(owner);return Err(error)}}Ok(payload(size_of::<OptionalText>()))}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(refusal("optional text candidate close was not begun"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.preparation.terminal_is_empty(){return self.preparation.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}if !self.snapshot_clone.terminal_is_empty(){return self.snapshot_clone.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}if !self.next.terminal_is_empty(){return self.next.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(owner)=self.displaced_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.displaced_close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}if self.displaced.is_some(){return self.handoff_displaced(grant)}
  if let Some(owner)=self.candidate_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.candidate_close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if self.candidate.is_some(){if grant.maximum_copy_bytes<size_of::<Puzzle2dSnapshot>(){return Ok(RetainedCloneStep::Progress(Default::default()))}match ControlledRetirement::new(self.candidate.take().unwrap()){Ok(owner)=>self.candidate_close=Some(owner),Err((error,owner))=>{self.candidate=Some(owner);return Err(error)}}return Ok(payload(size_of::<Puzzle2dSnapshot>()))}
  if self.plan.is_some(){let bytes=size_of::<Option<Puzzle2dTextPlan>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}self.plan=None;return Ok(payload(bytes))}let step=RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation},grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.preparation.terminal_is_empty()&&self.snapshot_clone.terminal_is_empty()&&self.next.terminal_is_empty()&&self.plan.is_none()&&self.candidate.is_none()&&self.displaced.is_none()&&self.displaced_close.is_none()&&self.candidate_close.is_none()&&self.source.is_none()&&self.mutation.is_none()}
}
impl<T:Puzzle2dTextCandidateIntent>Drop for Puzzle2dTextCandidateCursor<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"optional text candidate abandoned before controlled closure");if empty{unsafe{ManuallyDrop::drop(&mut self.state)}}}}
fn refusal(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,message)}
fn payload(bytes:usize)->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:0,released_bytes:0})}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
