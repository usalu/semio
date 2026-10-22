//! 🌱️ Original typed cache admission retains the incoming and displaced native Dictionary owners.
use super::*;
use protocol::causal::transition::HistoryFoldIndexInsert;
use semio_framework_job::{StepContext,OriginalTurnFailure,InteractiveJobCloseStep};
use semio_framework_value::{RetirementDemand,ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn number_demand(_: &u64,_:&u64,_:usize,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<u64>(),depth:1,..Default::default()})}
fn number_step(left:&u64,right:&u64,_:usize,g:RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>{if g.maximum_copy_bytes<2*size_of::<u64>(){return Ok((None,0,Default::default()))}Ok((Some(left.cmp(right)),0,RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<u64>(),..Default::default()}))}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Fields{insertion:Option<HistoryFoldIndexInsert<u64,(u64,Dictionary)>>,displaced:Option<ControlledRetirement<(u64,(u64,Dictionary))>>}
/// 🧳️ This exact original admission belongs to one business turn, one cache and one epoch.
pub struct NeuralCacheInsert{origin:(u64,u64),cache:usize,epoch:u64,fields:ManuallyDrop<Option<Fields>>,close:Option<ControlledRetirement<Fields>>,phase:u8,closing:bool,committed:bool}
impl NeuralCacheInsert{
 pub fn new(cache:&NeuralCache,hash:u64,output:Dictionary,cx:&mut StepContext<'_>)->Result<Option<Self>,(OriginalTurnFailure,Dictionary)>{
  let g=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||g.maximum_items==0||g.maximum_depth==0{return Err((invalid("original neural cache intake awaits admission").into(),output))}
  cx.consume_fuel(1);let original=Self{origin:(cx.operation().0,cx.generation().0),cache:cache as*const NeuralCache as usize,epoch:cache.current_epoch(),fields:ManuallyDrop::new(Some(Fields{insertion:Some(HistoryFoldIndexInsert::new(hash,(cache.current_epoch(),output))),displaced:None})),close:None,phase:0,closing:false,committed:false};
  if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}){let mut original=original;let((_,(_,output)),_)=original.fields.as_mut().unwrap().insertion.as_mut().unwrap().return_original(g).unwrap();original.fields.as_mut().unwrap().insertion=None;original.fields.take();return Err((error.into(),output))}Ok(Some(original))
 }
 pub fn next_demands(&self,cache:&NeuralCache,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(body)}let Some(fields)=self.fields.as_ref()else{return Ok(Default::default())};if let Some(close)=fields.displaced.as_ref(){return close.selected_demand(body)}let Ok(state)=cache.state.try_lock()else{return Ok(RetirementDemand{depth:1,..Default::default()})};fields.insertion.as_ref().unwrap().next_demands(&state.entries,body,number_demand)
 }
 pub fn step(&mut self,cache:&NeuralCache,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(self.committed)}if self.closing{return Err(invalid("original neural cache admission is closing").into())}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let d=self.next_demands(cache,cx.retained_grant().maximum_copy_bytes)?;let g=cx.retained_grant();if g.maximum_items==0||g.maximum_copy_bytes<d.copy_bytes||g.maximum_capacity_bytes<d.capacity_bytes||g.maximum_release_bytes<d.release_bytes||g.maximum_depth<d.depth{return Ok(false)}
  if self.origin!=(cx.operation().0,cx.generation().0)||self.cache!=cache as*const NeuralCache as usize||self.epoch!=cache.current_epoch(){cx.consume_fuel(1);return Err(invalid("original neural cache initiating source changed").into())}
  if let Some(close)=self.fields.as_mut().unwrap().displaced.as_mut(){semio_framework_job::close_original_retained_step(cx,d,|g|close.step(g))?;if close.terminal_is_empty(){self.fields.as_mut().unwrap().displaced=None;self.fields.take();}return Ok(self.terminal_is_empty())}
  let Ok(mut state)=cache.state.try_lock()else{return Ok(false)};cx.consume_fuel(1);let mut physical=Default::default();let result=(||->Result<(),ValueError>{let insertion=self.fields.as_mut().unwrap().insertion.as_mut().unwrap();if self.phase==0{let(ready,p)=insertion.step(&mut state.entries,RetainedCloneGrant{maximum_items:1,..g},number_step)?;physical=p;if ready{self.phase=1;}}else{let(slot,original,p)=insertion.commit_or_replace_with(&mut state.entries,RetainedCloneGrant{maximum_items:1,..g},|value|value)?;physical=p;if slot.is_some(){self.fields.as_mut().unwrap().insertion=None;self.committed=true;if let Some(original)=original{self.fields.as_mut().unwrap().displaced=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!()));}else{self.fields.take();}}}Ok(())})();drop(state);
  match result{Ok(())=>cx.consume_retained(physical)?,Err(error)=>return Err(OriginalTurnFailure::debit_original(if error.retained_progress()==Default::default(){error.with_retained_progress(physical)}else{error},cx))}Ok(self.terminal_is_empty())
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close.as_ref().map_or(Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()}),|close|close.selected_demand(body))}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.begin_close();if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}if self.origin!=(cx.operation().0,cx.generation().0){if cx.should_yield(){return Ok(InteractiveJobCloseStep::Blocked)}cx.consume_fuel(1);return Err(invalid("original neural cache close initiating source changed").into())}
  let d=self.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=semio_framework_job::close_original_retained_step(cx,d,|g|if let Some(close)=self.close.as_mut(){close.step(g)}else{self.close=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!()));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))})?;if self.close.as_ref().is_some_and(|close|close.terminal_is_empty()){self.close=None;}Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}})
 }
 pub fn terminal_is_empty(&self)->bool{self.fields.is_none()&&self.close.is_none()}
}
impl Drop for NeuralCacheInsert{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original neural cache requires paid full close");}}
