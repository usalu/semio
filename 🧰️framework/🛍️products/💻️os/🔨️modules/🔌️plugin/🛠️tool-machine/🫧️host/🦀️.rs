//! 🫧️ Native host facts retain their actual tag or resident key until the original scrub writer and every displaced owner close.
pub use semio_framework_plugin_tool_run::tool_jobs_tool_machine::{ToolTag,ScrubTag,ToolMachineRuntime};
use semio_framework_tool_machine::ToolStep;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_tool_machine::{OriginalScrubRequest,OriginalScrubWrite};
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalCallerSourceLoan,OriginalTurnFailure};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,SharedUtf8};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::size_of;
#[derive(semio_framework_value::RetireOwned)]
struct HostOwners<M>{tag:Option<ToolTag>,key:Option<SharedUtf8>,text:Vec<u8>,tool:Option<String>,request:Option<OriginalScrubRequest<M>>,result:Option<ToolStep<M>>}
pub struct OriginalScrubHost<M:RetireOwned+Send+Sync+'static>{binding:Option<OriginalSourceBinding>,decoder:(u64,u64),caller:Option<OriginalCallerSourceLoan>,writer:Option<Box<OriginalScrubWrite<M>>>,owners:ControlledRetirement<HostOwners<M>>,published:bool,closing:bool,forced_reason:Option<semio_framework_tool_machine::ToolAbortReason>}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original scrub host requires its actual tag, resident address and receiving caller")}
fn moved(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn funded(cx:&StepContext<'_>,d:RetirementDemand)->bool{!cx.should_yield()&&cx.retained_grant().maximum_items!=0&&cx.retained_work_can_enter(d)}
fn paid(cx:&mut StepContext<'_>,d:RetirementDemand)->Result<(),OriginalTurnFailure>{cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(())}
impl<M:RetireOwned+Send+Sync+'static> OriginalScrubHost<M>{
 fn birth(tag:Option<ToolTag>,key:Option<SharedUtf8>,binding:OriginalSourceBinding,decoder:(u64,u64))->Self{Self{binding:Some(binding),decoder,caller:None,writer:None,owners:ControlledRetirement::retain(HostOwners{tag,key,text:Vec::new(),tool:None,request:None,result:None}),published:false,closing:false,forced_reason:None}}
 pub fn abort_admission_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>()+2*size_of::<Option<ToolTag>>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 pub fn retire_admission_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>()+size_of::<SharedUtf8>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 pub fn admit_abort(input:&mut Option<ToolTag>,output:&mut Option<Box<Self>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{Self::admit_abort_reason(input,None,output,cx)}
 fn admit_abort_reason(input:&mut Option<ToolTag>,reason:Option<semio_framework_tool_machine::ToolAbortReason>,output:&mut Option<Box<Self>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let _=cx.original_source_parent()?;if !<HostOwners<M> as RetireOwned>::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original scrub host payload requires controlled retirement").into())}if output.is_some(){return Err(fault().into())}
  if !matches!(input,Some(ToolTag::Scrub(_)))||reason.is_none()&&!matches!(input,Some(ToolTag::Scrub(ScrubTag{phase:semio_framework_tool_machine::ScrubPhase::Abort{..},..}))){return Err(fault().into())}
  let d=Self::abort_admission_demands();
  let binding=OriginalSourceBinding::borrow(cx);
  semio_framework_job::admit_original_construction(cx,output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|control|{let decoder=(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(fault)?);let mut owner=Self::birth(input.take(),None,binding,decoder);owner.forced_reason=reason;Ok((Box::new(owner),receipt(d)))})

 }
 pub fn admit_retire(window:&SharedUtf8,output:&mut Option<Box<Self>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let _=cx.original_source_parent()?;if !<HostOwners<M> as RetireOwned>::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original scrub host payload requires controlled retirement").into())}if output.is_some(){return Err(fault().into())}
  let d=Self::retire_admission_demands();
  let binding=OriginalSourceBinding::borrow(cx);
  semio_framework_job::admit_original_construction(cx,output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|control|{let decoder=(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(fault)?);Ok((Box::new(Self::birth(None,Some(window.clone()),binding,decoder)),receipt(d)))})

 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.is_some_and(|binding|binding.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original scrub host requires its original source").into())}if cx.native_receiving_identity()!=Some(self.decoder){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original scrub host requires its receiving decoder").into())}Ok(())}
 pub fn demands(&self,ledger:&semio_framework_tool_machine::ScrubLedger<M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.published{return self.close_demands(body)}
  if self.caller.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  let owners=self.owners.original().ok_or_else(fault)?;
  if owners.request.is_none()&&self.writer.is_none(){if owners.tag.is_some(){return Ok(moved(2*size_of::<ToolTag>()+size_of::<OriginalScrubRequest<M>>()))}let key=owners.key.as_ref().ok_or_else(fault)?;if owners.text.capacity()<key.len(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Vec<u8>>(),capacity_bytes:key.len(),depth:1,..Default::default()})}if owners.text.len()<key.len(){return Ok(moved((key.len()-owners.text.len()).min(64)))}return Ok(moved(size_of::<OriginalScrubRequest<M>>()+size_of::<Vec<u8>>()))}
  if let Some(writer)=self.writer.as_ref(){if owners.result.is_some(){return Ok(moved(size_of::<bool>()))}writer.demands(ledger,body)}else{OriginalScrubWrite::admission_demands(ledger,owners.request.as_ref().unwrap())}
 }
 pub fn advance(&mut self,ledger:&mut semio_framework_tool_machine::ScrubLedger<M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing||cx.is_cancelled(){return self.close_step(ledger,cx)}
  if self.published{return self.close_step(ledger,cx)}
  if self.caller.is_none(){self.caller=cx.original_source_parent()?.admit_loan(cx)?;return Ok(false)}
  self.caller.as_ref().unwrap().validate_context(cx)?;
  let owners=self.owners.original_mut().ok_or_else(fault)?;
  if owners.request.is_none()&&self.writer.is_none(){
   if owners.tag.is_some(){let d=moved(2*size_of::<ToolTag>()+size_of::<OriginalScrubRequest<M>>());if !funded(cx,d){return Ok(false)}let Some(ToolTag::Scrub(ScrubTag{window,tool,phase}))=owners.tag.take()else{unreachable!("original host admitted its actual scrub tag")};let(gesture,reason)=match phase{semio_framework_tool_machine::ScrubPhase::Abort{gesture,reason}=>(gesture,self.forced_reason.unwrap_or(reason)),semio_framework_tool_machine::ScrubPhase::Tick{gesture}|semio_framework_tool_machine::ScrubPhase::Commit{gesture}=>(gesture,self.forced_reason.expect("frozen host owns its explicit abort reason"))};owners.tool=Some(tool);owners.request=Some(OriginalScrubRequest::Abort{window,gesture:Some(gesture),reason});paid(cx,d)?;return Ok(false)}
   let key=owners.key.as_ref().ok_or_else(fault)?;
   if owners.text.capacity()<key.len(){let d=RetirementDemand{copy_bytes:2*size_of::<Vec<u8>>(),capacity_bytes:key.len(),depth:1,..Default::default()};if !funded(cx,d){return Ok(false)}owners.text.try_reserve_exact(key.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original scrub host window allocation refused"))?;let mut d=d;d.capacity_bytes=owners.text.capacity();paid(cx,d)?;return Ok(false)}
   if owners.text.len()<key.len(){let count=(key.len()-owners.text.len()).min(64);let d=moved(count);if !funded(cx,d){return Ok(false)}let start=owners.text.len();owners.text.extend_from_slice(&key.as_str().as_bytes()[start..start+count]);paid(cx,d)?;return Ok(false)}
   let d=moved(size_of::<OriginalScrubRequest<M>>()+size_of::<Vec<u8>>());if !funded(cx,d){return Ok(false)}owners.request=Some(OriginalScrubRequest::Retire{window:String::from_utf8(std::mem::take(&mut owners.text)).expect("actual resident window is UTF-8")});paid(cx,d)?;return Ok(false)
  }
  if owners.result.is_some(){let d=moved(size_of::<bool>());if !funded(cx,d){return Ok(false)}self.published=true;paid(cx,d)?;return Ok(false)}
  if self.writer.is_none(){OriginalScrubWrite::admit(ledger,&mut owners.request,&mut self.writer,cx)?;return Ok(false)}
  self.writer.as_mut().unwrap().advance(ledger,&mut owners.result,cx)?;Ok(false)
 }
 pub fn published(&self)->bool{self.published}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(writer)=self.writer.as_ref(){return Ok(if writer.terminal_is_empty(){writer.shell_close_demands()}else{writer.close_demands(body)?})}
  if !self.owners.terminal_is_empty(){return self.owners.next_demand(body)}
  if let Some(caller)=self.caller.as_ref(){return Ok(caller.retirement_demands())}
  Ok(moved(size_of::<Option<OriginalSourceBinding>>()+size_of::<bool>()))
 }
 pub fn close_step(&mut self,ledger:&mut semio_framework_tool_machine::ScrubLedger<M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}self.validate(cx)?;
  if let Some(writer)=self.writer.as_mut(){if writer.terminal_is_empty(){let d=writer.shell_close_demands();semio_framework_job::close_original_retained_step(cx,d,|_|{self.writer.take();Ok(semio_framework_value::RetainedCloneStep::Progress(receipt(d)))})?;}else{cx.with_original_retirement(|cx|writer.close_step(ledger,cx))?;}return Ok(false)}
  if !self.owners.terminal_is_empty(){let d=self.owners.next_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|grant|self.owners.step(grant))?;return Ok(false)}
  if let Some(caller)=self.caller.as_mut(){caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){self.caller.take();}return Ok(false)}
  let d=moved(size_of::<Option<OriginalSourceBinding>>()+size_of::<bool>());semio_framework_job::close_original_retained_step(cx,d,|_|{self.binding=None;self.closing=true;Ok(semio_framework_value::RetainedCloneStep::Complete(receipt(d)))})?;Ok(self.terminal_is_empty())
 }
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.writer.is_none()&&self.caller.is_none()&&self.owners.terminal_is_empty()}
 pub fn shell_close_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<M:RetireOwned+Send+Sync+'static> Drop for OriginalScrubHost<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original scrub host custody requires funded closure")}}

/// 🚦️ A real dispatch compares one roster span per turn and retains every host writer before publishing its decision.
pub struct OriginalScrubDispatch<M:RetireOwned+Send+Sync+'static>{binding:Option<OriginalSourceBinding>,ledger:usize,decoder:(u64,u64),view:Option<(usize,usize)>,revision:u64,index:usize,roster:usize,offset:usize,phase:u8,frozen:bool,host:Option<Box<OriginalScrubHost<M>>>}
impl<M:RetireOwned+Send+Sync+'static> OriginalScrubDispatch<M>{
 pub fn admit(ledger:&semio_framework_tool_machine::ScrubLedger<M>,view:Option<&semio_framework::ViewModel>,frozen:bool,output:&mut Option<Box<Self>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let _=cx.original_source_parent()?;if output.is_some(){return Err(fault().into())}
  let d=RetirementDemand{copy_bytes:2*size_of::<Self>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()};let binding=OriginalSourceBinding::borrow(cx);let address=ledger as*const _ as usize;let revision=ledger.source_revision();let view=view.map(|view|(view.window_instances.as_ptr()as usize,view.window_instances.len()));
  semio_framework_job::admit_original_construction(cx,output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|control|{let decoder=(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(fault)?);Ok((Box::new(Self{binding:Some(binding),ledger:address,decoder,view,revision,index:0,roster:0,offset:0,phase:0,frozen,host:None}),receipt(d)))})

 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.is_some_and(|binding|binding.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original scrub dispatch requires its original source").into())}if cx.native_receiving_identity()!=Some(self.decoder){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original scrub dispatch requires its receiving decoder").into())}Ok(())}
 pub fn demands(&self,ledger:&semio_framework_tool_machine::ScrubLedger<M>,body:usize)->Result<RetirementDemand,ValueError>{if let Some(host)=self.host.as_ref(){return if host.terminal_is_empty(){let mut d=OriginalScrubHost::<M>::shell_close_demands();d.copy_bytes+=size_of::<u64>()+size_of::<u8>()+2*size_of::<usize>();Ok(d)}else{host.demands(ledger,body)}}if self.phase==2{return Ok(OriginalScrubHost::<M>::abort_admission_demands())}if self.phase==0&&self.view.is_some_and(|(_,length)|length!=0&&self.roster==length)&&ledger.retained_window_at(self.index).is_some(){return Ok(OriginalScrubHost::<M>::retire_admission_demands())}Ok(moved(64+4*size_of::<usize>()+size_of::<u64>()+size_of::<u8>()+size_of::<Option<bool>>()))}
 pub fn advance(&mut self,ledger:&mut semio_framework_tool_machine::ScrubLedger<M>,view:Option<&semio_framework::ViewModel>,tag:&mut Option<ToolTag>,decision:&mut Option<bool>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.ledger!=ledger as*const _ as usize{return Err(fault().into())}if self.view!=view.map(|view|(view.window_instances.as_ptr()as usize,view.window_instances.len())){return Err(fault().into())}
  if cx.is_cancelled(){self.close_step(ledger,cx)?;return Ok(false)}
  if let Some(host)=self.host.as_mut(){
   if host.terminal_is_empty(){let d=OriginalScrubHost::<M>::shell_close_demands();let extra=moved(size_of::<u64>()+size_of::<u8>()+2*size_of::<usize>());let d=RetirementDemand{copy_bytes:d.copy_bytes+extra.copy_bytes,..d};semio_framework_job::close_original_retained_step(cx,d,|_|{self.host.take();self.revision=ledger.source_revision();self.roster=0;self.offset=0;if self.phase==2{self.phase=3}Ok(semio_framework_value::RetainedCloneStep::Progress(receipt(d)))})?;return Ok(false)}
   host.advance(ledger,cx)?;return Ok(false)
  }
  let d=moved(64+4*size_of::<usize>()+size_of::<u64>()+size_of::<u8>()+size_of::<Option<bool>>());if !funded(cx,d){return Ok(false)}
  if self.phase==0{
   if ledger.source_revision()!=self.revision{self.revision=ledger.source_revision();self.index=0;self.roster=0;self.offset=0;paid(cx,d)?;return Ok(false)}
   let windows=view.map(|view|view.window_instances.as_slice()).filter(|windows|!windows.is_empty());
   let Some((window,_))=ledger.retained_window_at(self.index).filter(|_|windows.is_some())else{self.phase=1;paid(cx,d)?;return Ok(false)};
   if window.as_str().is_empty(){self.index+=1;self.roster=0;self.offset=0;paid(cx,d)?;return Ok(false)}
   let windows=windows.unwrap();
   if self.roster==windows.len(){OriginalScrubHost::admit_retire(window,&mut self.host,cx)?;return Ok(false)}
   let candidate=windows[self.roster].id.as_bytes();let source=window.as_str().as_bytes();
   if candidate.len()!=source.len(){self.roster+=1;self.offset=0;paid(cx,d)?;return Ok(false)}
   let count=(source.len()-self.offset).min(64);if source[self.offset..self.offset+count]!=candidate[self.offset..self.offset+count]{self.roster+=1;self.offset=0;paid(cx,d)?;return Ok(false)}
   self.offset+=count;if self.offset==source.len(){self.index+=1;self.roster=0;self.offset=0}paid(cx,d)?;return Ok(false)
  }
  if self.phase==1&&(self.frozen||matches!(tag,Some(ToolTag::Scrub(ScrubTag{phase:semio_framework_tool_machine::ScrubPhase::Abort{..},..})))){self.phase=2;paid(cx,d)?;return Ok(false)}
  if self.phase==2{OriginalScrubHost::admit_abort_reason(tag,self.frozen.then_some(semio_framework_tool_machine::ToolAbortReason::Frozen),&mut self.host,cx)?;return Ok(false)}
  if decision.is_some(){return Err(fault().into())}
  *decision=Some(self.phase==3);self.binding=None;paid(cx,d)?;Ok(true)
 }
 pub fn frozen(&self)->bool{self.frozen}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(host)=self.host.as_ref(){if !host.terminal_is_empty(){return host.close_demands(body)}return Ok(OriginalScrubHost::<M>::shell_close_demands())}Ok(moved(size_of::<Option<OriginalSourceBinding>>()))}
 pub fn close_step(&mut self,ledger:&mut semio_framework_tool_machine::ScrubLedger<M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}self.validate(cx)?;if self.ledger!=ledger as*const _ as usize{return Err(fault().into())}
  if let Some(host)=self.host.as_mut(){if !host.terminal_is_empty(){cx.with_original_retirement(|cx|host.close_step(ledger,cx))?;return Ok(false)}let d=OriginalScrubHost::<M>::shell_close_demands();semio_framework_job::close_original_retained_step(cx,d,|_|{self.host.take();Ok(semio_framework_value::RetainedCloneStep::Progress(receipt(d)))})?;return Ok(false)}
  let d=moved(size_of::<Option<OriginalSourceBinding>>());semio_framework_job::close_original_retained_step(cx,d,|_|{self.binding=None;Ok(semio_framework_value::RetainedCloneStep::Complete(receipt(d)))})?;Ok(self.terminal_is_empty())
 }
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.host.is_none()}
 pub fn shell_close_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<M:RetireOwned+Send+Sync+'static> Drop for OriginalScrubDispatch<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original scrub dispatch requires actual source-funded closure")}}

/// 🏷️ The actual original construction transfers a tag and its pending receiver together after bounded registry backing is admitted.
pub fn admit_original_tagged<P,M:RetireOwned+Send+'static,C,CM,R>(machines:&mut semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime<P,M,C,CM>,operation:u64,tag:&mut Option<ToolTag>,output:&mut Option<R>,mut demand:RetirementDemand,cx:&mut StepContext<'_>,build:impl FnOnce()->R)->Result<bool,OriginalTurnFailure>{
 let _=cx.original_source_parent()?;if output.is_some()||demand.copy_bytes<2*size_of::<R>(){return Err(fault().into())}
 if machines.operations.iter().any(|(id,_)|*id==operation){return Err(fault().into())}
 if tag.is_some(){
  let limit=semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
  if machines.operations.len()>=limit{return Ok(false)}
  if machines.operations.capacity()<limit{
   let old=machines.operations.capacity()*size_of::<(u64,ToolTag)>();let capacity=limit*size_of::<(u64,ToolTag)>();let d=RetirementDemand{copy_bytes:old+2*size_of::<Vec<(u64,ToolTag)>>(),capacity_bytes:capacity,release_bytes:old,depth:1};let mut done=None;
   semio_framework_job::admit_original_construction(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|_|{machines.operations.try_reserve_exact(limit-machines.operations.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original tool tag registry allocation refused"))?;Ok(((),RetainedCloneProgress{retained_capacity_bytes:machines.operations.capacity()*size_of::<(u64,ToolTag)>(),..receipt(d)}))})?;return Ok(false)
  }
  demand.copy_bytes=demand.copy_bytes.checked_add(2*size_of::<Option<ToolTag>>()+size_of::<(u64,ToolTag)>()).ok_or_else(fault)?;
 }
 semio_framework_job::admit_original_construction(cx,output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|_|{let receiver=build();if let Some(tag)=tag.take(){machines.operations.push((operation,tag));}Ok((receiver,receipt(demand)))})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🏷️tag/🦀️.rs"]
mod tag_retirement;
pub use tag_retirement::OriginalToolTagRetirement;
