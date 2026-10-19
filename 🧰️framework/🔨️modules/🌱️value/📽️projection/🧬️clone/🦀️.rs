//! 🔗️ Clones genuine original immutable roots through separately funded binding and projection turns.
use crate::retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneRef,RetainedCloneStep};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,list::persistent::PersistentList,snapshot_projection::ImmutableRootProjection,retirement::{RetireOwned,RetirementCursor,RetirementStep},shared_issuance::SealedShared};
use std::mem::{ManuallyDrop,size_of};
#[cfg(test)]
#[path="../../🧬️retained-clone/🔗️shared/📋️list/🧪️tests/🦀️.rs"]
mod list_tests;

struct State<S>{output:Option<S>,source:Option<RetainedCloneBinding>,close:RetainedCloneClose,closing:bool,spent:bool,projected:bool,admitted:bool}
/// 🌳️ One original immutable root cursor retains candidate custody through refusal, transfer and cancellation.
pub struct ImmutableRootCloneCursor<S:RetainedClone+ImmutableRootProjection>{state:ManuallyDrop<State<S>>}
impl<S:RetainedClone+ImmutableRootProjection> Default for ImmutableRootCloneCursor<S>{fn default()->Self{Self{state:ManuallyDrop::new(State{output:None,source:None,close:Default::default(),closing:false,spent:false,projected:false,admitted:false})}}}
impl<T:Send+Sync+'static> RetainedClone for PersistentList<T>{type Cursor=ImmutableRootCloneCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl<S:RetainedClone+ImmutableRootProjection> RetainedCloneCursor<S> for ImmutableRootCloneCursor<S>{
 fn output_ready(&self)->bool{!self.state.closing&&!self.state.spent&&self.state.admitted&&self.state.output.is_some()}
 fn close_started(&self)->bool{self.state.closing}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.state.closing{return Ok(Default::default());}self.state.close.next_owner_demands::<S>(self.state.output.is_some(),&self.state.source,body)}
 fn advance_demands(&self,source:RetainedCloneRef<'_,S>,_body:usize)->Result<RetirementDemand,ValueError>{
  if self.state.closing||self.state.spent||self.state.projected&&!self.state.admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original immutable root clone is closing, spent or refused"));}
  if !source.check_binding(&self.state.source)?{return Ok(RetirementDemand{copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.output_ready(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  S::root_projection_demand(source)
 }
 fn advance(&mut self,source:RetainedCloneRef<'_,S>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.advance_demands(source,body)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;
  if let Some(progress)=source.bind(&mut state.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
  if state.admitted{return Ok(RetainedCloneStep::Complete(Default::default()));}
  let Some((output,progress))=S::project_root(source,grant)?else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"funded original immutable root omitted its candidate"));};
  state.output=Some(output);state.projected=true;
  let progress=crate::retained_clone::admit_retained_clone_progress(grant,progress,"original immutable root clone")?;
  if progress.copied_items==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original immutable root omitted its work receipt"));}
  state.admitted=true;Ok(RetainedCloneStep::Complete(progress))
 }
 fn take(&mut self)->Option<S>{if !self.output_ready(){return None;}let output=self.state.output.take();self.state.spent=true;output}
 fn begin_close(&mut self)->bool{if self.state.closing{return false;}self.state.closing=true;true}
 fn terminal_is_empty(&self)->bool{self.state.closing&&self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.state.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original immutable root clone requires closing intent"));}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demands(grant.maximum_copy_bytes)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;
  let step=if !state.close.is_empty(){state.close.step_granted(grant)?}else if let Some(step)=state.close.begin_granted(&mut state.output,grant)?{step}else{crate::retained_clone::close_retained_binding(&mut state.source,grant)?};
  crate::retained_clone::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original immutable root clone closure")
 }
}
impl<S:RetainedClone+ImmutableRootProjection> Drop for ImmutableRootCloneCursor<S>{fn drop(&mut self){if self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("original immutable root clone retains unpublished output or source binding")}}}

struct ImmutableRootCloneRetirement<S:RetainedClone+ImmutableRootProjection>{cursor:ManuallyDrop<ImmutableRootCloneCursor<S>>}
impl<S:RetainedClone+ImmutableRootProjection> RetirementCursor for ImmutableRootCloneRetirement<S>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.close_demands(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.cursor.close_step(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_copy_byte_demand()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.cursor.next_close_capacity_byte_demand(body).ok()}
 fn next_close_byte_demand(&self)->Option<usize>{self.cursor.next_close_release_byte_demand().ok()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_depth_demand()}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<S:RetainedClone+ImmutableRootProjection> Drop for ImmutableRootCloneRetirement<S>{fn drop(&mut self){if self.cursor.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor)}}else if !std::thread::panicking(){panic!("original immutable root clone retirement frame remains retained")}}}
impl<S:RetainedClone+ImmutableRootProjection> RetireOwned for ImmutableRootCloneCursor<S>{
 fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(ImmutableRootCloneRetirement{cursor:ManuallyDrop::new(self)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ImmutableRootCloneRetirement<S>>())}
 fn controlled_retirement_supported()->bool{true}
}
