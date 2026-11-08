//! 📑️ One original borrowed text becomes an owned field under its initiating job grants.
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
use semio_framework_job::{receive_original_retained_work,OriginalTurnFailure,StepContext};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::retained_clone::allocation::backing::allocate_exact_string;
use std::mem::{ManuallyDrop,size_of};

fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"authoring text changed its original source or receiving phase")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn funded<'a>(cx:&mut StepContext<'a>,work:u64,run:impl FnOnce(&mut StepContext<'a>)->Result<bool,OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{if cx.fuel_remaining()<work||cx.original_total_fuel_remaining()<work||cx.should_yield()||cx.is_cancelled(){return Ok(false)}let deadline=cx.deadline_us().min(cx.original_absolute_deadline_us());cx.with_original_work_loan(work,deadline,run)}
fn performed(effect:&mut RetainedAllocationEffect<'_>,work:u64)->Result<(),ValueError>{if effect.performed(work)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"authoring text work exceeds its effect quote"))}}
fn settled<T>(effect:&RetainedAllocationEffect<'_>,result:Result<T,ValueError>)->RetainedWorkOutcome<T>{RetainedWorkOutcome{result,work:effect.performed_work()}}

pub struct FundedText{source:Option<(usize,usize)>,owner:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,offset:usize,phase:u8,closing:bool}
impl FundedText{
 pub fn new()->Self{Self{source:None,owner:ManuallyDrop::new(None),retirement:Default::default(),offset:0,phase:0,closing:false}}
 fn validate(&self,source:&str)->Result<(),ValueError>{if self.closing||self.source.is_some_and(|binding|binding!=(source.as_ptr()as usize,source.len())){return Err(invalid())}Ok(())}
 fn chunk(&self,source:&str,body:usize)->(usize,u64){let start=self.offset;let header=size_of::<usize>();let mut end=start.saturating_add(body.saturating_sub(header).min(4096)).min(source.len());let mut checks=1;while end>start&&!source.is_char_boundary(end){end-=1;checks+=1}if end==start&&start<source.len(){end=start+source.as_bytes()[start].leading_ones().max(1)as usize;checks+=1}(end-start,checks)}
 pub fn next_demands(&self,source:&str,body:usize)->Result<RetirementDemand,ValueError>{self.validate(source)?;Ok(match self.phase{0=>RetirementDemand{capacity_bytes:source.len(),..fixed(size_of::<Option<(usize,usize)>>()+size_of::<Option<String>>())},1=>fixed(self.chunk(source,body).0+size_of::<usize>()),2=>fixed(2*size_of::<Option<String>>()+size_of::<u8>()),_=>return Err(invalid())})}
 /// ⛽️ Quotes UTF-8 copying and its bounded boundary selection using only original source metadata.
 pub fn next_work_demand(&self,source:&str,body:usize)->Result<u64,ValueError>{self.validate(source)?;match self.phase{0=>Ok(1),1=>{let remaining=source.len().checked_sub(self.offset).ok_or_else(invalid)?;let bytes=body.saturating_sub(size_of::<usize>()).min(4096).min(remaining).max(remaining.min(4));u64::try_from(bytes).ok().and_then(|bytes|bytes.checked_add(6)).ok_or_else(invalid)},2=>Ok(0),_=>Err(invalid())}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==2}
 /// 🧶️ One actual supplied effect funds the same bounded original UTF-8 copy event and records its work once.
 pub fn advance_original(&mut self,source:&str,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let g=effect.grant();let maximum_work_units=effect.maximum_work_units();
  let quote=match self.next_work_demand(source,g.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return settled(effect,Err(error))};if self.phase==2{return settled(effect,Ok(RetainedCloneStep::Complete(Default::default())))}if maximum_work_units<quote{return settled(effect,Ok(RetainedCloneStep::Progress(Default::default())))}let(d,work)=if self.phase==1{let(count,checks)=self.chunk(source,g.maximum_copy_bytes);(fixed(count+size_of::<usize>()),count as u64+checks+1)}else{match self.next_demands(source,g.maximum_copy_bytes){Ok(d)=>(d,1),Err(error)=>return settled(effect,Err(error))}};if !retained_clone_grant_admits(g,d){return settled(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  if let Err(error)=performed(effect,work){return settled(effect,Err(error))}
  let result=(||{let mut p=progress(d);match self.phase{
   0=>{let text=allocate_exact_string(source.len())?;p.retained_capacity_bytes=text.capacity();*self.owner=Some(text);self.source=Some((source.as_ptr()as usize,source.len()));self.phase=if source.is_empty(){2}else{1};},
   1=>{let count=d.copy_bytes-size_of::<usize>();let end=self.offset+count;self.owner.as_mut().ok_or_else(invalid)?.push_str(&source[self.offset..end]);self.offset=end;if self.offset==source.len(){self.phase=2}},
   _=>return Err(invalid()),
  }Ok(if self.phase==2{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})();settled(effect,result)
 }
 /// 📏️ Quotes the original output header before its actual ownership transfer.
 pub fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed(2*size_of::<Option<String>>()+size_of::<u8>()))}
 /// 🎟️ The original string handoff moves one fixed ownership header.
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demands()?;Ok(1)}
 /// 📤️ The complete raw outcome preserves the exact original string until an admitted handoff.
 pub fn take_original(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<(String,RetainedCloneProgress)>>{
  let d=match self.take_demands(){Ok(d)=>d,Err(error)=>return settled(effect,Err(error))};if effect.maximum_work_units()<1||!retained_clone_grant_admits(effect.grant(),d){return settled(effect,Ok(None))}
  if let Err(error)=performed(effect,1){return settled(effect,Err(error))}
  let result=self.owner.take().ok_or_else(invalid).map(|text|{self.phase=3;Some((text,progress(d)))});settled(effect,result)
 }
 pub fn advance(&mut self,source:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let work=self.next_work_demand(source,cx.retained_grant().maximum_copy_bytes)?;if self.ready(){return Ok(true)}
  funded(cx,work,|cx|{let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|self.advance_original(source,effect))?else{return Ok(self.ready())};
   let overrun=turn.work!=performed;receive_original_retained_work(cx,turn)?;if overrun{return Err(invalid().into())}Ok(self.ready())})
 }
 pub fn take_into(&mut self,output:&mut Option<String>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(invalid().into())}let work=self.take_work_demand()?;
  funded(cx,work,|cx|{let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|self.take_original(effect))?else{return Ok(false)};
   let overrun=turn.work!=performed;let mut moved=false;
   let settled=RetainedWorkOutcome{result:turn.result.map(|pair|match pair{Some((text,p))=>{*output=Some(text);moved=true;RetainedCloneStep::Complete(p)},None=>RetainedCloneStep::Progress(Default::default())}),work:turn.work};
   receive_original_retained_work(cx,settled)?;if overrun{return Err(invalid().into())}Ok(moved)})
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(body)}
  if self.owner.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}
  Ok(if self.source.is_some(){fixed(size_of::<Option<(usize,usize)>>())}else{Default::default()})
 }
 /// ♻️ Quotes the genuine selected retirement child before its close turn.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.retirement.is_empty()||self.owner.is_some(){return self.retirement.next_owner_work::<String>(self.owner.is_some(),&None,body)}Ok(u64::from(self.source.is_some()))}
 /// ♻️ One close step runs inside the caller's effect and records its work once.
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let done=|effect:&RetainedAllocationEffect<'_>,result|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()});
  if !self.closing{return done(effect,Err(invalid().into()))}
  if !self.retirement.is_empty(){return self.retirement.step_granted(effect)}
  if self.owner.is_some(){let turn=self.retirement.begin_granted(&mut self.owner,effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  if self.source.is_some(){let d=fixed(size_of::<Option<(usize,usize)>>());if !retained_clone_grant_admits(effect.grant(),d){return done(effect,Ok(RetainedCloneStep::Progress(Default::default())))}if let Err(error)=performed(effect,1){return done(effect,Err(error.into()))}self.source=None;return done(effect,Ok(RetainedCloneStep::Complete(progress(d))))}
  done(effect,Ok(RetainedCloneStep::Complete(Default::default())))
 }
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.owner.is_none()&&self.retirement.is_empty()}
}
impl Drop for FundedText{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original authoring text requires funded output or closure")}}
