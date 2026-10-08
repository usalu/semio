//! ↩️ Optional text inverses clone only original paged identifiers and prior values.
use super::{ChangeNodeIcon,EditTargetRegionLabel,Puzzle2dMutation,text_preparation::{Puzzle2dTextIntent,Puzzle2dTextPlan,Puzzle2dTextPreparationCursor,Puzzle2dTextPreparationStep}};
use crate::Puzzle2dSnapshot;
use semio_framework_value::{list::PagedList,paged::PagedUtf8,retirement::controlled::ControlledRetirement,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
use std::{mem::{ManuallyDrop,size_of},ops::{Deref,DerefMut}};
type Text=PagedUtf8<{usize::MAX}>;
type Inverse=PagedList<Puzzle2dMutation,{usize::MAX}>;

pub trait Puzzle2dTextInverse:Puzzle2dTextIntent+Send {
 fn retained_id(snapshot:&Puzzle2dSnapshot,index:usize)->&Text;
 fn inverse_payload(id:Text,previous:Option<Text>)->Puzzle2dMutation;
}
impl Puzzle2dTextInverse for ChangeNodeIcon {
 fn retained_id(snapshot:&Puzzle2dSnapshot,index:usize)->&Text{&snapshot.nodes.get(index).expect("immutable icon inverse ordinal").id}
 fn inverse_payload(id:Text,previous:Option<Text>)->Puzzle2dMutation{Puzzle2dMutation::ChangeNodeIcon(Self{id,new_icon_kind:previous})}
}
impl Puzzle2dTextInverse for EditTargetRegionLabel {
 fn retained_id(snapshot:&Puzzle2dSnapshot,index:usize)->&Text{&snapshot.target_regions.get(index).expect("immutable label inverse ordinal").id}
 fn inverse_payload(id:Text,previous:Option<Text>)->Puzzle2dMutation{Puzzle2dMutation::EditTargetRegionLabel(Self{id,new_label:previous})}
}

pub struct Puzzle2dTextInverseCursor<T:Puzzle2dTextInverse>{state:ManuallyDrop<Puzzle2dTextInverseState<T>>}
#[doc(hidden)]
pub struct Puzzle2dTextInverseState<T:Puzzle2dTextInverse>{source:Option<RetainedCloneBinding>,mutation:Option<RetainedCloneBinding>,preparation:Puzzle2dTextPreparationCursor<T>,plan:Option<Puzzle2dTextPlan>,identifier:<Text as RetainedClone>::Cursor,previous:<Option<Text> as RetainedClone>::Cursor,pending:Option<Puzzle2dMutation>,inverse:Inverse,pending_close:Option<ControlledRetirement<Puzzle2dMutation>>,inverse_close:Option<ControlledRetirement<Inverse>>,phase:u8,closing:bool}
impl<T:Puzzle2dTextInverse>Deref for Puzzle2dTextInverseCursor<T>{type Target=Puzzle2dTextInverseState<T>;fn deref(&self)->&Self::Target{&self.state}}
impl<T:Puzzle2dTextInverse>DerefMut for Puzzle2dTextInverseCursor<T>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.state}}
impl<T:Puzzle2dTextInverse>Default for Puzzle2dTextInverseCursor<T>{fn default()->Self{Self{state:ManuallyDrop::new(Puzzle2dTextInverseState{source:None,mutation:None,preparation:Default::default(),plan:None,identifier:Text::retained_clone_cursor(),previous:Option::<Text>::retained_clone_cursor(),pending:None,inverse:Default::default(),pending_close:None,inverse_close:None,phase:0,closing:false})}}}
impl<T:Puzzle2dTextInverse>Puzzle2dTextInverseCursor<T>{
 pub fn advance(&mut self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,mutation:RetainedCloneRef<'_,T>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.closing||self.phase==9{return Err(refusal("optional text inverse is closing or spent"))}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  source.bind(&mut self.source)?;mutation.bind(&mut self.mutation)?;
  match self.phase{
   0=>{let step=self.preparation.advance(source,mutation,grant)?;let progress=match step{Puzzle2dTextPreparationStep::Pending(progress)=>progress,Puzzle2dTextPreparationStep::Complete{plan,progress}=>{self.plan=self.preparation.take();self.preparation.begin_close();self.phase=if plan.index.is_some(){1}else{7};progress}};Ok(RetainedCloneStep::Progress(progress))}
   1=>{let index=self.plan.unwrap().index.unwrap();let step=self.identifier.advance(source.project(11,|snapshot|T::retained_id(snapshot,index)),grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2}Ok(RetainedCloneStep::Progress(step.progress()))}
   2=>{let index=self.plan.unwrap().index.unwrap();let step=self.previous.advance(source.project(12,|snapshot|T::previous(snapshot,index)),grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=3}Ok(RetainedCloneStep::Progress(step.progress()))}
   3=>{if grant.maximum_copy_bytes<size_of::<Puzzle2dMutation>(){return Ok(RetainedCloneStep::Progress(Default::default()))}let id=self.identifier.take().ok_or_else(||refusal("optional inverse lost original ID"))?;let previous=self.previous.take().ok_or_else(||refusal("optional inverse lost original value"))?;self.pending=Some(T::inverse_payload(id,previous));self.identifier.begin_close();self.previous.begin_close();self.phase=4;Ok(payload(size_of::<Puzzle2dMutation>()))}
   4=>{if !self.identifier.terminal_is_empty(){return self.identifier.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}if !self.previous.terminal_is_empty(){return self.previous.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.phase=5;Ok(payload(0))}
   5=>{if !self.inverse.has_reserved_slot(){let demand=self.inverse.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.ok_or_else(||refusal("optional inverse lost page demand"))?;if demand>grant.maximum_capacity_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}let progress=self.inverse.reserve_one(grant.maximum_capacity_bytes).map_err(|error|ValueError::from(error.refusal()))?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(progress.progressed),copied_bytes:0,retained_capacity_bytes:progress.allocated_bytes,released_bytes:0}))}self.phase=6;Ok(payload(0))}
   6=>{if grant.maximum_copy_bytes<size_of::<Puzzle2dMutation>(){return Ok(RetainedCloneStep::Progress(Default::default()))}let operation=self.pending.take().ok_or_else(||refusal("optional inverse lost payload"))?;if let Err(owner)=self.inverse.push_reserved(operation){self.pending=Some(owner);return Err(refusal("optional inverse lost admitted page"))}self.phase=7;Ok(payload(size_of::<Puzzle2dMutation>()))}
   7=>{if !self.preparation.terminal_is_empty(){return self.preparation.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}if grant.maximum_copy_bytes<size_of::<Inverse>(){return Ok(RetainedCloneStep::Progress(Default::default()))}self.phase=8;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Inverse>(),retained_capacity_bytes:0,released_bytes:0}))}
   8=>Ok(RetainedCloneStep::Complete(Default::default())),_=>Err(refusal("optional inverse phase is invalid"))
  }
 }
 pub fn take(&mut self)->Option<Inverse>{if self.closing||self.phase!=8{return None}self.phase=9;Some(std::mem::take(&mut self.inverse))}
 pub fn begin_close(&mut self){self.closing=true;self.preparation.begin_close();self.identifier.begin_close();self.previous.begin_close();}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{
  if !self.preparation.terminal_is_empty(){return self.preparation.next_close_copy_byte_demand()}
  if !self.identifier.terminal_is_empty(){return self.identifier.next_close_copy_byte_demand()}
  if !self.previous.terminal_is_empty(){return self.previous.next_close_copy_byte_demand()}
  if let Some(owner)=self.pending_close.as_ref(){return owner.next_copy_byte_demand()}
  if self.pending.is_some(){return Ok(size_of::<Puzzle2dMutation>())}
  if let Some(owner)=self.inverse_close.as_ref(){return owner.next_copy_byte_demand()}
  if !self.inverse.terminal_is_empty(){return Ok(size_of::<Inverse>())}
  if self.plan.is_some(){return Ok(size_of::<Option<Puzzle2dTextPlan>>())}
  RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.mutation})
 }
 pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{
  if !self.preparation.terminal_is_empty(){return self.preparation.next_close_capacity_byte_demand(body)}
  if !self.identifier.terminal_is_empty(){return self.identifier.next_close_capacity_byte_demand(body)}
  if !self.previous.terminal_is_empty(){return self.previous.next_close_capacity_byte_demand(body)}
  if let Some(owner)=self.pending_close.as_ref(){return owner.next_capacity_byte_demand(body)}
  if self.pending.is_some(){return Ok(0)}
  if let Some(owner)=self.inverse_close.as_ref(){return owner.next_capacity_byte_demand(body)}
  if !self.inverse.terminal_is_empty(){return Ok(0)}
  if self.plan.is_some(){return Ok(0)}
  RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.mutation},body)
 }
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{
  if !self.preparation.terminal_is_empty(){return self.preparation.next_close_release_byte_demand()}
  if !self.identifier.terminal_is_empty(){return self.identifier.next_close_release_byte_demand()}
  if !self.previous.terminal_is_empty(){return self.previous.next_close_release_byte_demand()}
  if let Some(owner)=self.pending_close.as_ref(){return owner.next_release_byte_demand()}
  if self.pending.is_some(){return Ok(0)}
  if let Some(owner)=self.inverse_close.as_ref(){return owner.next_release_byte_demand()}
  if !self.inverse.terminal_is_empty(){return Ok(0)}
  if self.plan.is_some(){return Ok(0)}
  RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.mutation})
 }
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{
  if !self.preparation.terminal_is_empty(){return self.preparation.next_close_depth_demand()}
  if !self.identifier.terminal_is_empty(){return self.identifier.next_close_depth_demand()}
  if !self.previous.terminal_is_empty(){return self.previous.next_close_depth_demand()}
  if let Some(owner)=self.pending_close.as_ref(){return owner.next_depth_demand()}
  if self.pending.is_some(){return Ok(1)}
  if let Some(owner)=self.inverse_close.as_ref(){return owner.next_depth_demand()}
  if !self.inverse.terminal_is_empty(){return Ok(1)}
  if self.plan.is_some(){return Ok(1)}
  RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.mutation})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(refusal("optional inverse close was not begun"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.preparation.terminal_is_empty(){return self.preparation.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if !self.identifier.terminal_is_empty(){return self.identifier.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if !self.previous.terminal_is_empty(){return self.previous.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(owner)=self.pending_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.pending_close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if self.pending.is_some(){if grant.maximum_copy_bytes<size_of::<Puzzle2dMutation>(){return Ok(RetainedCloneStep::Progress(Default::default()))}match ControlledRetirement::new(self.pending.take().unwrap()){Ok(owner)=>self.pending_close=Some(owner),Err((error,owner))=>{self.pending=Some(owner);return Err(error)}}return Ok(payload(size_of::<Puzzle2dMutation>()))}
  if let Some(owner)=self.inverse_close.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.inverse_close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if !self.inverse.terminal_is_empty(){if grant.maximum_copy_bytes<size_of::<Inverse>(){return Ok(RetainedCloneStep::Progress(Default::default()))}match ControlledRetirement::new(std::mem::take(&mut self.inverse)){Ok(owner)=>self.inverse_close=Some(owner),Err((error,owner))=>{self.inverse=owner;return Err(error)}}return Ok(payload(size_of::<Inverse>()))}
  if self.plan.is_some(){let bytes=size_of::<Option<Puzzle2dTextPlan>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}self.plan=None;return Ok(payload(bytes))}
  let step=RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation},grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.preparation.terminal_is_empty()&&self.identifier.terminal_is_empty()&&self.previous.terminal_is_empty()&&self.pending.is_none()&&self.inverse.terminal_is_empty()&&self.pending_close.is_none()&&self.inverse_close.is_none()&&self.plan.is_none()&&self.source.is_none()&&self.mutation.is_none()}
}
impl<T:Puzzle2dTextInverse>Drop for Puzzle2dTextInverseCursor<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"optional text inverse abandoned before controlled closure");if empty{unsafe{ManuallyDrop::drop(&mut self.state)}}}}
fn refusal(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,message)}
fn payload(bytes:usize)->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:0,released_bytes:0})}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
