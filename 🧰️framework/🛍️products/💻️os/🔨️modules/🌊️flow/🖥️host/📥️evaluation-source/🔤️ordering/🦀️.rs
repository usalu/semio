//! 📚️ One raw original text comparator serves retained display and authoring contexts.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,retained_clone::retained_clone_grant_admits};
use std::{cmp::Ordering,mem::size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original text ordering changed its source or operation")}
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
/// 🔎️ The actual grant funds a bounded original byte window without manufacturing a context.
pub struct OriginalTextOrdering{source:Option<(usize,usize,usize,usize)>,offset:usize,done:Option<Ordering>,closing:bool,progress:RetainedCloneProgress}
impl OriginalTextOrdering{
 pub fn new()->Self{Self{source:None,offset:0,done:None,closing:false,progress:Default::default()}}
 pub fn next_demands(&self,left:&str,right:&str,copy:usize)->Result<RetirementDemand,ValueError>{let source=(left.as_ptr()as usize,left.len(),right.as_ptr()as usize,right.len());if self.closing||self.source.is_some_and(|s|s!=source){return Err(invalid())}let remaining=left.len().min(right.len()).saturating_sub(self.offset);let count=if self.done.is_some(){0}else{remaining.min(64).min(copy.saturating_sub(size_of::<Self>()))};Ok(RetirementDemand{copy_bytes:size_of::<Self>()+count,..fixed()})}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn advance(&mut self,left:&str,right:&str,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<Ordering>>{
  self.progress=Default::default();let d=match self.next_demands(left,right,g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let remaining=left.len().min(right.len()).saturating_sub(self.offset);let count=d.copy_bytes-size_of::<Self>();if !retained_clone_grant_admits(g,d)||self.done.is_none()&&remaining!=0&&count==0{return RetainedWorkOutcome{result:Ok(None),work:0}}
  self.source=Some((left.as_ptr()as usize,left.len(),right.as_ptr()as usize,right.len()));if self.done.is_none(){if remaining==0{self.done=Some(left.len().cmp(&right.len()))}else{let mut bytes=[0;64];bytes[..count].copy_from_slice(&left.as_bytes()[self.offset..self.offset+count]);let order=bytes[..count].cmp(&right.as_bytes()[self.offset..self.offset+count]);self.offset+=count;if order!=Ordering::Equal{self.done=Some(order)}else if self.offset==left.len().min(right.len()){self.done=Some(left.len().cmp(&right.len()))}}}
  self.progress=RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()};RetainedWorkOutcome{result:Ok(self.done),work:1}
 }
 pub fn reset_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>(),..fixed()}}
 pub fn reset(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{self.progress=Default::default();if self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}let d=self.reset_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.source=None;self.offset=0;self.done=None;self.progress=RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()};RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(self.progress)),work:1}}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self)->RetirementDemand{self.reset_demand()}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{self.progress=Default::default();if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}let d=self.close_demands();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.source=None;self.offset=0;self.done=None;self.progress=RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()};RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(self.progress)),work:1}}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.source.is_none()&&self.done.is_none()&&self.offset==0}
}
impl Drop for OriginalTextOrdering{fn drop(&mut self){assert!(self.terminal_is_empty(),"original raw text ordering requires terminal source binding")}}
/// 🪪️ The original Flow caller binds the same raw comparator to its actual operation and generation.
pub struct FlowTextOrderingWork{raw:OriginalTextOrdering,source:Option<(usize,usize,usize,usize)>,origin:Option<semio_framework_job::OriginalSourceBinding>,closing:bool}
impl FlowTextOrderingWork{
 pub fn new()->Self{Self{raw:OriginalTextOrdering::new(),source:None,origin:None,closing:false}}
 fn verify(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if self.closing||self.origin.is_some_and(|o|!o.matches(cx)){Err(invalid())}else{Ok(())}}
 pub fn next_demands(&self,left:&str,right:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  let source=(left.as_ptr()as usize,left.len(),right.as_ptr()as usize,right.len());if self.closing||self.source.is_some_and(|prior|prior!=source){return Err(invalid())}
  if self.origin.is_none()||self.source.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_job::OriginalSourceBinding>>()+size_of::<Option<(usize,usize,usize,usize)>>(),..fixed()})}
  self.raw.next_demands(left,right,copy)
 }
 pub fn step(&mut self,left:&str,right:&str,cx:&mut StepContext<'_>)->Result<Option<Ordering>,OriginalTurnFailure>{
  self.verify(cx)?;let source=(left.as_ptr()as usize,left.len(),right.as_ptr()as usize,right.len());if self.source.is_some_and(|s|s!=source){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}if self.origin.is_none()||self.source.is_none(){let d=self.next_demands(left,right,cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(None)}self.origin=Some(semio_framework_job::OriginalSourceBinding::borrow(cx));self.source=Some(source);cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})?;return Ok(None)}
  let d=self.next_demands(left,right,cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(None)}let turn=self.raw.advance(left,right,cx.retained_grant());cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));match turn.result{Ok(output)=>{cx.consume_retained(self.raw.step_progress())?;Ok(output)},Err(error)=>Err(OriginalTurnFailure::debit_original(error,cx))}
 }
 pub fn reset(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.verify(cx)?;let own=size_of::<Option<(usize,usize,usize,usize)>>();let mut d=self.raw.reset_demand();d.copy_bytes+=own;if cx.should_yield()||cx.is_cancelled()||cx.fuel_remaining()<2||!cx.retained_work_can_enter(d){return Ok(false)}let g=RetainedCloneGrant{maximum_copy_bytes:cx.retained_grant().maximum_copy_bytes-own,..cx.retained_grant()};let turn=self.raw.reset(g);cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX).saturating_add(1));match turn.result{Ok(step)=>{let mut p=step.progress();p.copied_items=p.copied_items.max(1);p.copied_bytes+=own;if matches!(step,RetainedCloneStep::Complete(_)){self.source=None}cx.consume_retained(p)?;Ok(matches!(step,RetainedCloneStep::Complete(_)))},Err(error)=>{let mut p=error.retained_progress();p.copied_items=p.copied_items.max(1);p.copied_bytes+=own;Err(OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))}}}

 pub fn begin_close(&mut self){self.closing=true;self.raw.begin_close()}
 pub fn close_demands(&self)->RetirementDemand{if !self.raw.terminal_is_empty(){self.raw.close_demands()}else{RetirementDemand{copy_bytes:size_of::<Option<semio_framework_job::OriginalSourceBinding>>()+size_of::<Option<(usize,usize,usize,usize)>>(),..fixed()}}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !self.raw.terminal_is_empty(){return self.raw.close_step(g)}let d=self.close_demands();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.origin=None;self.source=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.origin.is_none()&&self.source.is_none()&&self.raw.terminal_is_empty()}
}
impl Drop for FlowTextOrderingWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original context text ordering requires terminal operation and source")}}

#[doc(hidden)] pub trait OrderingRetire:Send+'static{fn begin(&mut self);fn demand(&self)->RetirementDemand;fn close(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome;fn empty(&self)->bool;}
impl OrderingRetire for OriginalTextOrdering{fn begin(&mut self){self.begin_close()}fn demand(&self)->RetirementDemand{self.close_demands()}fn close(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{self.close_step(g)}fn empty(&self)->bool{self.terminal_is_empty()}}
impl OrderingRetire for FlowTextOrderingWork{fn begin(&mut self){self.begin_close()}fn demand(&self)->RetirementDemand{self.close_demands()}fn close(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{self.close_step(g)}fn empty(&self)->bool{self.terminal_is_empty()}}
#[doc(hidden)] pub struct OrderingRetirement<T:OrderingRetire>{original:T}
impl<T:OrderingRetire> semio_framework_value::retirement::RetirementCursor for OrderingRetirement<T>{
 fn next_demand(&self,_copy:usize)->Result<RetirementDemand,ValueError>{Ok(self.original.demand())}
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(u64::from(!self.original.empty()))}
 fn close_into<'reply>(&mut self,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:neural_engine::retirement::CloseReply<'reply>)->Result<neural_engine::retirement::CloseComplete<'reply>,ValueError>{self.original.begin();let original=&mut self.original;let poll=crate::close_by_grant(effect,|g|original.close(g));neural_engine::retirement::close_reply(poll,effect,reply)}
 fn terminal_is_empty(&self)->bool{self.original.empty()}
 fn terminal_release_bytes(&self)->Option<usize>{self.original.empty().then_some(size_of::<Self>())}
}
macro_rules! owned{($type:ty)=>{impl semio_framework_value::retirement::RetireOwned for $type{type RetirementCursor=OrderingRetirement<Self>;fn retirement_seed(self)->OrderingRetirement<Self>{OrderingRetirement{original:self}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OrderingRetirement<Self>>())}fn controlled_retirement_supported()->bool{true}}}}
owned!(OriginalTextOrdering);
owned!(FlowTextOrderingWork);
