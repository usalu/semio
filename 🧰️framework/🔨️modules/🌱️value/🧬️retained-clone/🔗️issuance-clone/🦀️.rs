//! 🔗️ Clones genuine original shared issuances through separately funded alias and projection turns.
use super::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneRef,RetainedCloneStep};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep},shared_issuance::SealedShared};
use std::mem::{ManuallyDrop,size_of};

struct State<T:Send+Sync+'static>{output:Option<SealedShared<T>>,source:Option<RetainedCloneBinding>,close:RetainedCloneClose,closing:bool,spent:bool,admitted:bool}
pub struct SharedCloneCursor<T:Send+Sync+'static>{state:ManuallyDrop<State<T>>}
impl<T:Send+Sync+'static> Default for SharedCloneCursor<T>{fn default()->Self{Self{state:ManuallyDrop::new(State{output:None,source:None,close:Default::default(),closing:false,spent:false,admitted:false})}}}
impl<T:Send+Sync+'static> RetainedClone for SealedShared<T>{type Cursor=SharedCloneCursor<T>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl<T:Send+Sync+'static> RetainedCloneCursor<SealedShared<T>> for SharedCloneCursor<T>{
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_demand(self.state.closing))}
fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.state.closing))}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.state.closing,effect)}
fn output_ready(&self)->bool{!self.state.closing&&!self.state.spent&&self.state.admitted&&self.state.output.is_some()}
 fn advance_demands(&self,source:RetainedCloneRef<'_,SealedShared<T>>,_body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.state.closing||self.state.spent||self.state.output.is_some()&&!self.state.admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone is closing or spent"));}if let Some(demand)=source.binding_demand(&self.state.source)?{return Ok(demand);}Ok(RetirementDemand{depth:1,..Default::default()})}
 fn advance(&mut self,source:RetainedCloneRef<'_,SealedShared<T>>,body: usize,maximum_work_units:u64, effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
  let demand=self.advance_demands(source,body,maximum_work_units)?;if !super::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;
  if let Some(progress)={let turn=source.bind(&mut state.source,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
  if state.output.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if source.get().terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"original shared alias was already transferred")).into())}let turn=source.get().try_duplicate(grant);work+=turn.work;let(output,progress)=turn.result?;state.output=Some(output);let progress=super::admit_retained_clone_progress(grant,progress,"original shared alias clone")?;state.admitted=true;Ok(RetainedCloneStep::Complete(progress))
 })();crate::RetainedWorkOutcome{result,work}}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<SealedShared<T>,bool>()}
    fn take<'reply>(&mut self,output:&mut Option<SealedShared<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.state.output,output,self.state.spent,true,effect,reply,SealedShared<T>,bool)}
 
 fn terminal_is_empty(&self)->bool{self.state.closing&&self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.state.close.next_copy_with_binding(&self.state.source)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.state.close.next_owner_capacity_with_binding::<SealedShared<T>>(self.state.output.is_some(),body,&self.state.source)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.state.close.next_release_with_binding(&self.state.source)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.state.close.next_owner_depth_with_binding(self.state.output.is_some(),&self.state.source)}
  fn close_started(&self)->bool{self.state.closing}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.state.closing{return Ok(Default::default());}self.state.close.next_owner_demands::<SealedShared<T>>(self.state.output.is_some(),&self.state.source,body)}
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
  if !self.state.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone requires closing intent").into());}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demands(grant.maximum_copy_bytes)?;if !super::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;let step=if !state.close.is_empty(){{let turn=state.close.step_granted(effect);work+=turn.work;turn.result}?}else if let Some(step)={let turn=state.close.begin_granted(&mut state.output,effect);work+=turn.work;turn.result}?{step}else{{let turn=super::close_retained_binding(&mut state.source,effect);work+=turn.work;turn.result}?};
  super::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original shared clone closure").map_err(Into::into)
 })();crate::RetainedWorkOutcome{result,work}}
}
impl<T:Send+Sync+'static> Drop for SharedCloneCursor<T>{fn drop(&mut self){if self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("original shared clone retains unpublished output or source binding")}}}

#[doc(hidden)]
pub struct SharedCloneRetirement<T:Send+Sync+'static>{cursor:ManuallyDrop<SharedCloneCursor<T>>}
impl<T:Send+Sync+'static> RetirementCursor for SharedCloneRetirement<T>{
  fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.cursor.close_started(){self.cursor.close_demands(body)}else{self.cursor.begin_close_demand()}}
fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.cursor.close_started(){self.cursor.close_work_demand(body)}else{self.cursor.begin_close_work_demand()}}
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();let work=match self.next_computational_work(grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return RetirementStep::Failure(error.into(),0)};if effect.maximum_work_units()<work{return RetirementStep::BudgetExhausted}let turn=if self.cursor.close_started(){self.cursor.close_step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}else{let turn=self.cursor.begin_close_paid(grant);crate::RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress).map_err(Into::into),work:turn.work}};match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
 fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_copy_byte_demand()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.cursor.next_close_capacity_byte_demand(body).ok()}
 fn next_close_byte_demand(&self)->Option<usize>{self.cursor.next_close_release_byte_demand().ok()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_depth_demand()}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+Sync+'static> Drop for SharedCloneRetirement<T>{fn drop(&mut self){if self.cursor.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor)}}else if !std::thread::panicking(){panic!("original shared clone retirement frame remains retained")}}}
impl<T:Send+Sync+'static> RetireOwned for SharedCloneCursor<T>{
 type RetirementCursor=SharedCloneRetirement<T>;
 fn retirement_seed(self)->Self::RetirementCursor{SharedCloneRetirement{cursor:ManuallyDrop::new(self)}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedCloneRetirement<T>>())}
 fn controlled_retirement_supported()->bool{true}
}
