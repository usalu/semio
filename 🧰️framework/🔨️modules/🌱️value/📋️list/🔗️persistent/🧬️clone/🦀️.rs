//! 🔗️ Clones genuine original immutable roots through separately funded binding and projection turns.
use crate::retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep};
use crate::{RetirementDemand, ValueError, ValueRefusalKind, snapshot_projection::ImmutableRootProjection, retirement::{RetireOwned, RetirementCursor, RetirementStep}};
use crate::list::persistent::PersistentList;
use std::mem::{ManuallyDrop,size_of};
#[cfg(test)]
#[path="../../../🧬️retained-clone/🔗️sealed-shared-list/📋️list/🧪️tests/🦀️.rs"]
mod list_tests;

impl<T:RetireOwned+Send+Sync+'static> ImmutableRootProjection for PersistentList<T>{
 fn root_projection_demand(_original:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{Ok(Self::projection_demand())}
 fn root_projection_work_demand(original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>{if original.get().root_transferred(){return Err(ValueError::invariant("original persistent root is transferred"))}original.get().root_projection_work()}
 fn project_root(original:RetainedCloneRef<'_,Self>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{Self::project(original,effect)}
}

struct State<S>{output:Option<S>,source:Option<RetainedCloneBinding>,close:RetainedCloneClose,closing:bool,spent:bool,projected:bool,admitted:bool}
/// 🌳️ One original immutable root cursor retains candidate custody through refusal, transfer and cancellation.
pub struct ImmutableRootCloneCursor<S:RetainedClone+ImmutableRootProjection>{state:ManuallyDrop<State<S>>}
impl<S:RetainedClone+ImmutableRootProjection> Default for ImmutableRootCloneCursor<S>{fn default()->Self{Self{state:ManuallyDrop::new(State{output:None,source:None,close:Default::default(),closing:false,spent:false,projected:false,admitted:false})}}}
impl<T:RetireOwned+Send+Sync+'static> RetainedClone for PersistentList<T>{type Cursor=ImmutableRootCloneCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl<S:RetainedClone+ImmutableRootProjection> RetainedCloneCursor<S> for ImmutableRootCloneCursor<S>{
 fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_demand(self.state.closing))}
fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.state.closing))}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.state.closing,effect)}
fn output_ready(&self)->bool{!self.state.closing&&!self.state.spent&&self.state.admitted&&self.state.output.is_some()}
 fn close_started(&self)->bool{self.state.closing}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.state.closing{return Ok(Default::default());}self.state.close.next_owner_demands::<S>(self.state.output.is_some(),&self.state.source,body)}
 fn advance_demands(&self,source:RetainedCloneRef<'_,S>,_body:usize,_maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.state.closing||self.state.spent||self.state.projected&&!self.state.admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original immutable root clone is closing, spent or refused"));}
  if !source.check_binding(&self.state.source)?{return Ok(RetirementDemand{copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.output_ready(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  S::root_projection_demand(source)
 }
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,S>,_body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
  if self.state.closing||self.state.spent||self.state.projected&&!self.state.admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original immutable root clone is closing, spent or refused"));}
  let work=if !source.check_binding(&self.state.source)?{source.binding_work_demand(&self.state.source)?}else if self.state.admitted{1}else{S::root_projection_work_demand(source)?};
  Ok(if work<=maximum_work_units{work}else{0})
 }
 fn advance(&mut self,source:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,semio_framework_value_retirement::OriginalRetirementFailure>{
  let demand=self.advance_demands(source,body,maximum_work_units)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;
  if let Some(progress)={let turn=source.bind(&mut state.source,effect);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
  if state.admitted{return Ok(RetainedCloneStep::Complete(Default::default()));}
  let Some((output,progress))={let turn=S::project_root(source,effect);work+=turn.work;turn.result}?else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"funded original immutable root omitted its candidate").into());};
  state.output=Some(output);state.projected=true;
  let progress=crate::retained_clone::admit_retained_clone_progress(grant,progress,"original immutable root clone")?;
  if progress.copied_items==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original immutable root omitted its work receipt").into());}
  state.admitted=true;Ok(RetainedCloneStep::Complete(progress))
 })();crate::RetainedWorkOutcome{result,work}}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<S,bool>()}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
 fn take<'reply>(&mut self,output:&mut Option<S>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.state.output,output,self.state.spent,true,effect,reply,S,bool)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.state.closing{return Ok(0)}self.state.close.next_owner_work::<S>(self.state.output.is_some(),&self.state.source,body)}
 
 fn terminal_is_empty(&self)->bool{self.state.closing&&self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::RetirementPollExit>{
  if !self.state.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original immutable root clone requires closing intent").into());}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demands(grant.maximum_copy_bytes)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;
  let step=if !state.close.is_empty(){let turn=crate::RetirementPollExit::settled(state.close.step_granted(effect))?;work+=turn.work;turn.result?}else if let Some(step)={let turn=state.close.begin_granted(&mut state.output,effect);work+=turn.work;turn.result}?{step}else{let turn=crate::RetirementPollExit::settled(crate::retained_clone::close_retained_binding(&mut state.source,effect))?;work+=turn.work;turn.result?};
  crate::retained_clone::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original immutable root clone closure").map_err(Into::into)
 })();crate::RetirementPollExit::finish(result,work)}
}
impl<S:RetainedClone+ImmutableRootProjection> Drop for ImmutableRootCloneCursor<S>{fn drop(&mut self){if self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("original immutable root clone retains unpublished output or source binding")}}}

#[doc(hidden)]
pub struct ImmutableRootCloneRetirement<S:RetainedClone+ImmutableRootProjection>{cursor:ManuallyDrop<ImmutableRootCloneCursor<S>>}
impl<S:RetainedClone+ImmutableRootProjection> RetirementCursor for ImmutableRootCloneRetirement<S>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.cursor.close_demands(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.cursor.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{crate::retirement::erased_cursor_turn(self.cursor.close_step(effect))}
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
 type RetirementCursor=ImmutableRootCloneRetirement<S>;
 fn retirement_seed(mut self)->Self::RetirementCursor{self.state.closing=true;ImmutableRootCloneRetirement{cursor:ManuallyDrop::new(self)}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ImmutableRootCloneRetirement<S>>())}
 fn controlled_retirement_supported()->bool{true}
}
