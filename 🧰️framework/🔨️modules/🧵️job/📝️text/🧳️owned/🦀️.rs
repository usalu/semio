//! 📑️ One original borrowed text becomes an owned field under its initiating job grants.
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retained_clone::allocation::backing::allocate_exact_string;
use std::mem::{ManuallyDrop,size_of};

fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"authoring text changed its original source or receiving phase")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn idle<T,E>(value:T)->RetainedWorkOutcome<T,E>{RetainedWorkOutcome{result:Ok(value),work:0}}
fn funded<'a>(cx:&mut StepContext<'a>,work:u64,run:impl FnOnce(&mut StepContext<'a>)->Result<bool,OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{if cx.fuel_remaining()<work||cx.original_total_fuel_remaining()<work||cx.should_yield()||cx.is_cancelled(){return Ok(false)}let deadline=cx.deadline_us().min(cx.original_absolute_deadline_us());cx.with_original_work_loan(work,deadline,run)}
fn original(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}

pub struct FundedText{source:Option<(usize,usize)>,owner:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,offset:usize,phase:u8,closing:bool}
impl FundedText{
 pub fn new()->Self{Self{source:None,owner:ManuallyDrop::new(None),retirement:Default::default(),offset:0,phase:0,closing:false}}
 fn validate(&self,source:&str)->Result<(),ValueError>{if self.closing||self.source.is_some_and(|binding|binding!=(source.as_ptr()as usize,source.len())){return Err(invalid())}Ok(())}
 fn chunk(&self,source:&str,body:usize)->(usize,u64){let start=self.offset;let header=size_of::<usize>();let mut end=start.saturating_add(body.saturating_sub(header).min(4096)).min(source.len());let mut checks=1;while end>start&&!source.is_char_boundary(end){end-=1;checks+=1}if end==start&&start<source.len(){end=start+source.as_bytes()[start].leading_ones().max(1)as usize;checks+=1}(end-start,checks)}
 pub fn next_demands(&self,source:&str,body:usize)->Result<RetirementDemand,ValueError>{self.validate(source)?;Ok(match self.phase{0=>RetirementDemand{capacity_bytes:source.len(),..fixed(size_of::<Option<(usize,usize)>>()+size_of::<Option<String>>())},1=>fixed(self.chunk(source,body).0+size_of::<usize>()),2=>fixed(2*size_of::<Option<String>>()+size_of::<u8>()),_=>return Err(invalid())})}
 /// ⛽️ Quotes UTF-8 copying and its bounded boundary selection using only original source metadata.
 pub fn next_work_demand(&self,source:&str,body:usize)->Result<u64,ValueError>{self.validate(source)?;match self.phase{0=>Ok(1),1=>{let remaining=source.len().checked_sub(self.offset).ok_or_else(invalid)?;let bytes=body.saturating_sub(size_of::<usize>()).min(4096).min(remaining).max(remaining.min(4));u64::try_from(bytes).ok().and_then(|bytes|bytes.checked_add(6)).ok_or_else(invalid)},2=>Ok(0),_=>Err(invalid())}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==2}
 /// 🧶️ One actual supplied grant funds the same bounded original UTF-8 copy event.
 pub fn advance_original(&mut self,source:&str,g:RetainedCloneGrant,maximum_work_units:u64)->RetainedWorkOutcome{
  let quote=match self.next_work_demand(source,g.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if self.phase==2{return idle(RetainedCloneStep::Complete(Default::default()))}if maximum_work_units<quote{return idle(RetainedCloneStep::Progress(Default::default()))}let(d,work)=if self.phase==1{let(count,checks)=self.chunk(source,g.maximum_copy_bytes);(fixed(count+size_of::<usize>()),count as u64+checks+1)}else{match self.next_demands(source,g.maximum_copy_bytes){Ok(d)=>(d,1),Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}}};if !retained_clone_grant_admits(g,d){return idle(RetainedCloneStep::Progress(Default::default()))}
  let result=(||{let mut p=progress(d);match self.phase{
   0=>{let text=allocate_exact_string(source.len())?;p.retained_capacity_bytes=text.capacity();*self.owner=Some(text);self.source=Some((source.as_ptr()as usize,source.len()));self.phase=if source.is_empty(){2}else{1};},
   1=>{let count=d.copy_bytes-size_of::<usize>();let end=self.offset+count;self.owner.as_mut().ok_or_else(invalid)?.push_str(&source[self.offset..end]);self.offset=end;if self.offset==source.len(){self.phase=2}},
   _=>return Err(invalid()),
  }Ok(if self.phase==2{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})();RetainedWorkOutcome{result,work}
 }
 /// 📏️ Quotes the original output header before its actual ownership transfer.
 pub fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed(2*size_of::<Option<String>>()+size_of::<u8>()))}
 /// 🎟️ The original string handoff moves one fixed ownership header.
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demands()?;Ok(1)}
 /// 📤️ The complete raw outcome preserves the exact original string until an admitted handoff.
 pub fn take_original(&mut self,g:RetainedCloneGrant,maximum_work_units:u64)->RetainedWorkOutcome<Option<(String,RetainedCloneProgress)>>{
  let d=match self.take_demands(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if maximum_work_units<1||!retained_clone_grant_admits(g,d){return idle(None)}
  let result=self.owner.take().ok_or_else(invalid).map(|text|{self.phase=3;Some((text,progress(d)))});RetainedWorkOutcome{result,work:1}
 }
 pub fn advance(&mut self,source:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let work=self.next_work_demand(source,cx.retained_grant().maximum_copy_bytes)?;if self.ready(){return Ok(true)}
  funded(cx,work,|cx|{let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();let fuel=cx.fuel_remaining();let mut accepted=None;admit_original_construction(cx,&mut accepted,original(d),|_|{let turn=self.advance_original(source,grant,fuel);RetainedWorkOutcome{result:turn.result.map(|step|((),step.progress())),work:turn.work}})?;Ok(self.ready())})
 }
 pub fn take_into(&mut self,output:&mut Option<String>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(invalid().into())}let work=self.take_work_demand()?;
  funded(cx,work,|cx|{let d=self.take_demands()?;let grant=cx.retained_grant();let fuel=cx.fuel_remaining();admit_original_construction(cx,output,original(d),|_|{let turn=self.take_original(grant,fuel);RetainedWorkOutcome{result:turn.result.and_then(|output|output.ok_or_else(invalid)),work:turn.work}})})
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(body)}
  if self.owner.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}
  Ok(if self.source.is_some(){fixed(size_of::<Option<(usize,usize)>>())}else{Default::default()})
 }
 /// ♻️ Quotes the genuine selected retirement child before its close turn.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.retirement.is_empty()||self.owner.is_some(){return self.retirement.next_owner_work::<String>(self.owner.is_some(),&None,body)}Ok(u64::from(self.source.is_some()))}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid().into()),work:0}}
  if !self.retirement.is_empty(){return self.retirement.step_granted(g)}
  if self.owner.is_some(){let turn=self.retirement.begin_granted(&mut self.owner,g);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}
  if self.source.is_some(){let d=fixed(size_of::<Option<(usize,usize)>>());if !retained_clone_grant_admits(g,d){return idle(RetainedCloneStep::Progress(Default::default()))}self.source=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(progress(d))),work:1}}
  idle(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.owner.is_none()&&self.retirement.is_empty()}
}
impl Drop for FundedText{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original authoring text requires funded output or closure")}}
