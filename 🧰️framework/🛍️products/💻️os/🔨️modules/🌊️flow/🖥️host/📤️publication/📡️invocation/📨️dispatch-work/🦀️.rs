//! 📨️ A complete evaluation invocation wire owns every string before crossing the plugin interface.
use crate::invocation_request::{FlowInvocationRequestSource,FlowInvocationRequestWork};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::ManuallyDrop;
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowInvocationDispatch{pub extension_id:String,pub capability:String,pub request_json:String,pub response_action:String}
#[derive(semio_framework_value::RetireOwned)]
struct DispatchFields{request:Option<String>,extension:Option<String>,capability:Option<String>,response:Option<String>,buffer:Vec<u8>,output:Option<FlowInvocationDispatch>}
pub struct FlowInvocationDispatchWork{fields:ManuallyDrop<Option<DispatchFields>>,request:Option<FlowInvocationRequestWork>,retirement:Option<ControlledRetirement<DispatchFields>>,source:Option<usize>,operation:Option<semio_framework_job::OriginalSourceBinding>,phase:u8,field:u8,offset:usize,closing:bool,progress:RetainedCloneProgress,work:u64}
impl FlowInvocationDispatchWork{
 pub fn new(resume:bool)->Self{Self{fields:ManuallyDrop::new(Some(DispatchFields{request:None,extension:None,capability:None,response:None,buffer:Vec::new(),output:None})),request:Some(FlowInvocationRequestWork::new(resume)),retirement:None,source:None,operation:None,phase:0,field:0,offset:0,closing:false,progress:Default::default(),work:0}}
 /// 📥️ Keeps an already funded native request while the same producer constructs its remaining invocation fields.
 pub fn from_request(request:String)->Self{Self{fields:ManuallyDrop::new(Some(DispatchFields{request:Some(request),extension:None,capability:None,response:None,buffer:Vec::new(),output:None})),request:None,retirement:None,source:None,operation:None,phase:2,field:0,offset:0,closing:false,progress:Default::default(),work:0}}
 fn text<'a>(&self,origin:&'a dyn FlowInvocationRequestSource)->&'a str{match self.field{0=>origin.extension_id(),1=>"evaluate",2=>"flowEvalResolve",_=>unreachable!("original invocation dispatch field")}}
 fn authenticate(&self,origin:&dyn FlowInvocationRequestSource)->Result<(),ValueError>{if self.source.is_some_and(|source|source!=origin.invocation_source_identity()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation dispatch source changed"))}Ok(())}
 pub fn next_demands(&self,origin:&dyn FlowInvocationRequestSource,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.authenticate(origin)?;
  if self.phase==0{let mut demand=self.request.as_ref().unwrap().next_demands(origin,copy)?;demand.depth+=1;return Ok(demand)}
  if self.phase==1{let mut demand=self.request.as_ref().unwrap().next_close_demands(copy)?;demand.depth+=1;return Ok(demand)}
  Ok(RetirementDemand{copy_bytes:if self.phase==3{usize::from(self.offset<self.text(origin).len())}else{0},capacity_bytes:if self.phase==2{self.text(origin).len()}else{0},depth:1,..Default::default()})
 }
 pub fn step(&mut self,origin:&dyn FlowInvocationRequestSource,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<FlowInvocationDispatch>,semio_framework_job::OriginalTurnFailure>{
  let operation=semio_framework_job::OriginalSourceBinding::borrow(cx);if self.operation.is_some_and(|original|!original.matches(cx)){return Err(semio_framework_job::OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation dispatch operation changed"),cx))}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  let mut total=RetainedCloneProgress::default();let mut total_work=0;
  for _ in 0..512{
   if cx.is_cancelled()||cx.should_yield(){self.progress=total;self.work=total_work;return Ok(None)}
   let result=self.advance(origin,cx.retained_grant());let progress=self.progress;let work=self.work;total_work+=work;
   if progress!=Default::default(){self.operation=Some(operation)}
   cx.consume_fuel(work);
   let sum=match total.checked_add(progress){Ok(sum)=>sum,Err(error)=>{if let Ok(Some(output))=result{self.restore_output(output)}self.progress=total;self.work=total_work;return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx))}};
   match result{
    Ok(output)=>{if let Err(error)=cx.consume_retained(progress){if let Some(output)=output{self.restore_output(output)}self.progress=sum;self.work=total_work;return Err(error.into())}total=sum;if let Some(output)=output{self.progress=total;self.work=total_work;return Ok(Some(output))}if progress==Default::default(){self.progress=total;self.work=total_work;return Ok(None)}},
    Err(error)=>{self.progress=sum;self.work=total_work;return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx))},
   }
  }self.progress=total;self.work=total_work;Ok(None)
 }

 pub fn advance(&mut self,origin:&dyn FlowInvocationRequestSource,grant:RetainedCloneGrant)->Result<Option<FlowInvocationDispatch>,ValueError>{
  self.progress=Default::default();self.work=0;if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original invocation dispatch is closing"))}let demand=self.next_demands(origin,grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
  self.source=Some(origin.invocation_source_identity());self.work=1;self.progress.copied_items=1;
  match self.phase{
   0=>{let child=self.request.as_mut().unwrap();let result=child.advance(origin,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});self.progress=child.step_progress();self.work=child.last_turn_work();if let Some(request)=result?{self.fields.as_mut().unwrap().request=Some(request);child.begin_close();self.phase=2}},
   2=>{let length=self.text(origin).len();let fields=self.fields.as_mut().unwrap();fields.buffer.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original invocation dispatch text allocation refused"))?;self.progress.retained_capacity_bytes=fields.buffer.capacity();self.offset=0;self.phase=3},
   3=>{let length=self.text(origin).len();if self.offset==length{let fields=self.fields.as_mut().unwrap();let text=unsafe{String::from_utf8_unchecked(std::mem::take(&mut fields.buffer))};match self.field{0=>fields.extension=Some(text),1=>fields.capability=Some(text),2=>fields.response=Some(text),_=>unreachable!("original invocation dispatch field")};if self.field==2{self.phase=4}else{self.field+=1;self.phase=2}}
    else{let count=(length-self.offset).min(grant.maximum_copy_bytes).min(64);let text=match self.field{0=>origin.extension_id(),1=>"evaluate",2=>"flowEvalResolve",_=>unreachable!("original invocation dispatch field")};self.fields.as_mut().unwrap().buffer.extend_from_slice(&text.as_bytes()[self.offset..self.offset+count]);self.offset+=count;self.progress.copied_bytes=count}},
   4=>{let fields=self.fields.as_mut().unwrap();fields.output=Some(FlowInvocationDispatch{extension_id:fields.extension.take().unwrap(),capability:fields.capability.take().unwrap(),request_json:fields.request.take().unwrap(),response_action:fields.response.take().unwrap()});self.phase=5},
   5=>{self.phase=6;return Ok(self.fields.as_mut().unwrap().output.take())},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation dispatch was transferred")),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:FlowInvocationDispatch){assert_eq!(self.phase,6);self.fields.as_mut().unwrap().output=Some(output);self.phase=5}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn begin_close(&mut self){self.closing=true;if let Some(child)=self.request.as_mut(){child.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(child)=self.request.as_ref(){let mut demand=child.next_close_demands(copy)?;demand.depth+=1;return Ok(demand)}if let Some(owner)=self.retirement.as_ref(){return owner.selected_demand(copy)}Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  self.progress=Default::default();self.work=0;if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation dispatch close was not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(child)=self.request.as_mut(){if grant.maximum_depth<2{return Ok(RetainedCloneStep::Progress(Default::default()))}let result=child.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant},effect);self.progress=child.step_progress();self.work=child.last_turn_work();let step=result?;if child.terminal_is_empty(){self.request=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(owner)=self.retirement.as_mut(){let poll=owner.step(grant.maximum_copy_bytes,effect);self.progress=owner.step_progress();let step=match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>{self.work=u64::try_from(turn.work).unwrap_or(u64::MAX);turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?},semio_framework_value::RetainedWorkPoll::Pending{work}=>{self.work=u64::try_from(work).unwrap_or(u64::MAX);return Ok(RetainedCloneStep::Progress(Default::default()))}};if owner.terminal_is_empty(){self.retirement=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
  self.retirement=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!("invocation dispatch fields support controlled retirement")));self.work=1;self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.request.is_none()&&self.fields.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowInvocationDispatchWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original invocation dispatch requires explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
