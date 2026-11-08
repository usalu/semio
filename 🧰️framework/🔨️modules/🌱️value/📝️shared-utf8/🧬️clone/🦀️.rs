//! 📝️ Clones only the original shared UTF8 lease under its actual retained source binding.
use crate::retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneCursor,RetainedCloneRef,RetainedCloneStep};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,SharedUtf8};
use std::mem::ManuallyDrop;

struct Utf8State{output:Option<SharedUtf8>,source:Option<RetainedCloneBinding>,close:RetainedCloneClose,closing:bool,spent:bool,admitted:bool}
pub struct SharedUtf8CloneCursor{state:ManuallyDrop<Utf8State>}
impl Default for SharedUtf8CloneCursor{fn default()->Self{Self{state:ManuallyDrop::new(Utf8State{output:None,source:None,close:Default::default(),closing:false,spent:false,admitted:false})}}}
impl RetainedClone for SharedUtf8{type Cursor=SharedUtf8CloneCursor;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl RetainedCloneCursor<SharedUtf8> for SharedUtf8CloneCursor{
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_demand(self.state.closing))}
fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.state.closing))}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.state.closing,effect)}
fn output_ready(&self)->bool{!self.state.closing&&!self.state.spent&&self.state.admitted&&self.state.output.is_some()}
 fn advance_demands(&self,source:RetainedCloneRef<'_,SharedUtf8>,_body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.state.closing||self.state.spent||self.state.output.is_some()&&!self.state.admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone is closing or spent"));}if maximum_work_units==0{return Ok(Default::default())}if let Some(demand)=source.binding_demand(&self.state.source)?{return Ok(demand);}Ok(RetirementDemand{depth:1,..Default::default()})}
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,SharedUtf8>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.advance_demands(source,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if source.check_binding(&self.state.source)?{Ok(u64::from(self.state.output.is_none()))}else{source.binding_work_demand(&self.state.source)}}
 fn advance(&mut self,source:RetainedCloneRef<'_,SharedUtf8>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(||->Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
  if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let demand=self.advance_demands(source,body,maximum_work_units)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;
  if let Some(progress)={let turn=source.bind(&mut state.source,effect);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
  if state.output.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let(output,progress)={let turn=source.get().admit_clone(effect);work+=turn.work;turn.result}?;state.output=Some(output);let progress=crate::retained_clone::admit_retained_clone_progress(grant,progress,"original shared alias clone")?;state.admitted=true;Ok(RetainedCloneStep::Complete(progress))
 })();crate::RetainedWorkOutcome{result,work}}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<SharedUtf8,bool>()}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.state.closing{return Ok(0);}self.state.close.next_owner_work::<SharedUtf8>(self.state.output.is_some(),&self.state.source,body)}
 fn take<'reply>(&mut self,output:&mut Option<SharedUtf8>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.state.output,output,self.state.spent,true,effect,reply,SharedUtf8,bool)}
 fn terminal_is_empty(&self)->bool{self.state.closing&&self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.state.close.next_copy_with_binding(&self.state.source)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.state.close.next_owner_capacity_with_binding::<SharedUtf8>(self.state.output.is_some(),body,&self.state.source)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.state.close.next_release_with_binding(&self.state.source)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.state.close.next_owner_depth_with_binding(self.state.output.is_some(),&self.state.source)}
  fn close_started(&self)->bool{self.state.closing}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.state.closing{return Ok(Default::default());}self.state.close.next_owner_demands::<SharedUtf8>(self.state.output.is_some(),&self.state.source,body)}
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();let mut work=0;let result=(||->Result<RetainedCloneStep,crate::RetirementPollExit>{
  if !self.state.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone requires closing intent").into());}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demands(grant.maximum_copy_bytes)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let step=if !self.state.close.is_empty(){let turn=crate::RetirementPollExit::settled(self.state.close.step_granted(effect))?;work+=turn.work;turn.result?}else{let state=&mut *self.state;let began={let turn=state.close.begin_granted(&mut state.output,effect);work+=turn.work;turn.result?};match began{Some(step)=>step,None=>{let turn=crate::RetirementPollExit::settled(crate::retained_clone::close_retained_binding(&mut state.source,effect))?;work+=turn.work;turn.result?}}};
  crate::retained_clone::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original shared clone closure").map_err(Into::into)
 })();crate::RetirementPollExit::finish(result,work)}
}
impl Drop for SharedUtf8CloneCursor{fn drop(&mut self){if self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("original shared clone retains unpublished output or source binding")}}}
