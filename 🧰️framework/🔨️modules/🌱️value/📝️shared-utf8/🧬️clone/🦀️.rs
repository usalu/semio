//! 📝️ Clones only the original shared UTF8 lease under its actual retained source binding.
use crate::retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneRef,RetainedCloneStep};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep},SharedUtf8};
use std::mem::{ManuallyDrop,size_of};

struct Utf8State{output:Option<SharedUtf8>,source:Option<RetainedCloneBinding>,close:RetainedCloneClose,closing:bool,spent:bool,admitted:bool}
pub struct SharedUtf8CloneCursor{state:ManuallyDrop<Utf8State>}
impl Default for SharedUtf8CloneCursor{fn default()->Self{Self{state:ManuallyDrop::new(Utf8State{output:None,source:None,close:Default::default(),closing:false,spent:false,admitted:false})}}}
impl RetainedClone for SharedUtf8{type Cursor=SharedUtf8CloneCursor;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl RetainedCloneCursor<SharedUtf8> for SharedUtf8CloneCursor{
    fn output_ready(&self)->bool{!self.state.closing&&!self.state.spent&&self.state.admitted&&self.state.output.is_some()}
 fn advance_demands(&self,source:RetainedCloneRef<'_,SharedUtf8>,_body:usize)->Result<RetirementDemand,ValueError>{if self.state.closing||self.state.spent||self.state.output.is_some()&&!self.state.admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone is closing or spent"));}if let Some(demand)=source.binding_demand(&self.state.source)?{return Ok(demand);}Ok(RetirementDemand{depth:1,..Default::default()})}
 fn advance(&mut self,source:RetainedCloneRef<'_,SharedUtf8>,body: usize, grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.advance_demands(source,body)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;
  if let Some(progress)=source.bind(&mut state.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
  if state.output.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let(output,progress)=source.get().admit_clone(grant)?;state.output=Some(output);let progress=crate::retained_clone::admit_retained_clone_progress(grant,progress,"original shared alias clone")?;state.admitted=true;Ok(RetainedCloneStep::Complete(progress))
 }
 fn take(&mut self)->Option<SharedUtf8>{if !self.output_ready(){return None;}let output=self.state.output.take();if output.is_some(){self.state.spent=true;}output}
 fn begin_close(&mut self)->bool{if self.state.closing{return false;}self.state.closing=true;true}
 fn terminal_is_empty(&self)->bool{self.state.closing&&self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.state.close.next_copy_with_binding(&self.state.source)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.state.close.next_owner_capacity_with_binding::<SharedUtf8>(self.state.output.is_some(),body,&self.state.source)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.state.close.next_release_with_binding(&self.state.source)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.state.close.next_owner_depth_with_binding(self.state.output.is_some(),&self.state.source)}
  fn close_started(&self)->bool{self.state.closing}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.state.closing{return Ok(Default::default());}self.state.close.next_owner_demands::<SharedUtf8>(self.state.output.is_some(),&self.state.source,body)}
fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.state.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared clone requires closing intent"));}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demands(grant.maximum_copy_bytes)?;if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let state=&mut *self.state;let step=if !state.close.is_empty(){state.close.step_granted(grant)?}else if let Some(step)=state.close.begin_granted(&mut state.output,grant)?{step}else{crate::retained_clone::close_retained_binding(&mut state.source,grant)?};
  crate::retained_clone::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original shared clone closure")
 }
}
impl Drop for SharedUtf8CloneCursor{fn drop(&mut self){if self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("original shared clone retains unpublished output or source binding")}}}

struct SharedUtf8CloneRetirement{cursor:ManuallyDrop<SharedUtf8CloneCursor>}
impl RetirementCursor for SharedUtf8CloneRetirement{
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
impl Drop for SharedUtf8CloneRetirement{fn drop(&mut self){if self.cursor.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor)}}else if !std::thread::panicking(){panic!("original shared clone retirement frame remains retained")}}}
impl RetireOwned for SharedUtf8CloneCursor{
 fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(SharedUtf8CloneRetirement{cursor:ManuallyDrop::new(self)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedUtf8CloneRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
