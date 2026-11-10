//! 📨️ Canonical invocation JSON copied from its genuine retained origin under explicit grants.
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::ManuallyDrop;

pub trait FlowInvocationRequestSource:Sync{
 fn invocation_source_identity(&self)->usize;
 fn extension_id(&self)->&str;
 fn neuron_id(&self)->&str;
 fn operator_id(&self)->&str;
 fn input_json(&self)->&str;
 fn dependency_json(&self)->&str;
 fn operator_version(&self)->&str;
 fn window_id(&self)->&str;
 fn window_kind_id(&self)->&str;
 fn node_hash(&self)->u64;
}
#[derive(Clone,Copy,Default)]
struct RequestCursor{part:u8,offset:usize}
impl RequestCursor{
 fn chunk(&mut self,origin:&dyn FlowInvocationRequestSource,resume:bool)->Option<([u8;32],usize)>{
  let mut out=[0;32];
  loop{
   let literal=match self.part{0=>Some("{\"neuronId\":"),2=>Some(",\"operatorId\":"),4=>Some(",\"inputJson\":"),6=>Some(",\"dependencyJson\":"),8=>Some(",\"operatorVersion\":"),10=>Some(",\"nodeHash\":"),12=>Some(",\"resume\":"),14=>Some(",\"windowId\":"),16=>Some(",\"windowKindId\":"),18=>Some(",\"extensionId\":"),20=>Some("}"),_=>None};
   if let Some(literal)=literal{self.part+=1;self.offset=0;out[..literal.len()].copy_from_slice(literal.as_bytes());return Some((out,literal.len()))}
   let text=match self.part{1=>Some(origin.neuron_id()),3=>Some(origin.operator_id()),5=>Some(if resume{""}else{origin.input_json()}),7=>Some(if resume{""}else{origin.dependency_json()}),9=>Some(origin.operator_version()),15=>Some(origin.window_id()),17=>Some(origin.window_kind_id()),19=>Some(origin.extension_id()),_=>None};
   if let Some(text)=text{
    let offset=self.offset;self.offset+=1;
    if offset==0{out[0]=b'"';return Some((out,1))}
    if offset==text.len()+1{self.part+=1;self.offset=0;out[0]=b'"';return Some((out,1))}
    let byte=text.as_bytes()[offset-1];
    let mut escape=[0;6];let length=semio_framework_pack_json::canonical_escape(byte,&mut escape);out[..length].copy_from_slice(&escape[..length]);return Some((out,length))
   }
   match self.part{
    11=>{let mut value=origin.node_hash();let mut digits=[0;20];let mut length=0;loop{digits[length]=b'0'+(value%10)as u8;length+=1;value/=10;if value==0{break}}for index in 0..length{out[index]=digits[length-index-1]}self.part+=1;return Some((out,length))},
    13=>{let text=if resume{b"true".as_slice()}else{b"false".as_slice()};out[..text.len()].copy_from_slice(text);self.part+=1;return Some((out,text.len()))},
    21=>return None,
    _=>unreachable!("invocation request cursor"),
   }
  }
 }
}

#[derive(semio_framework_value::RetireOwned)]
struct RequestFields{buffer:Vec<u8>,output:Option<String>}
pub struct FlowInvocationRequestWork{fields:ManuallyDrop<Option<RequestFields>>,retirement:Option<ControlledRetirement<RequestFields>>,source:Option<usize>,operation:Option<(u64,u64)>,cursor:RequestCursor,length:usize,resume:bool,phase:u8,closing:bool,progress:RetainedCloneProgress,work:u64}
impl FlowInvocationRequestWork{
 pub fn new(resume:bool)->Self{Self{fields:ManuallyDrop::new(Some(RequestFields{buffer:Vec::new(),output:None})),retirement:None,source:None,operation:None,cursor:Default::default(),length:0,resume,phase:0,closing:false,progress:Default::default(),work:0}}
 fn authenticate(&self,origin:&dyn FlowInvocationRequestSource)->Result<(),ValueError>{if self.source.is_some_and(|source|source!=origin.invocation_source_identity()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation request source changed"))}Ok(())}
 pub fn next_demands(&self,origin:&dyn FlowInvocationRequestSource,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.authenticate(origin)?;
  let copy_bytes=if self.phase==2{let mut cursor=self.cursor;cursor.chunk(origin,self.resume).map_or(0,|(_,length)|length)}else{0};
  Ok(RetirementDemand{copy_bytes,capacity_bytes:if self.phase==1{self.length}else{0},depth:1,..Default::default()})
 }
 pub fn step(&mut self,origin:&dyn FlowInvocationRequestSource,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<String>,semio_framework_job::OriginalTurnFailure>{
  let operation=(cx.operation().0,cx.generation().0);if self.operation.is_some_and(|original|original!=operation){return Err(semio_framework_job::OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation request operation changed"),cx))}
  let mut total=RetainedCloneProgress::default();let mut total_work=0;
  for _ in 0..512{
   if cx.is_cancelled()||cx.should_yield(){self.progress=total;self.work=total_work;return Ok(None)}
   let result=self.advance(origin,cx.retained_grant());let progress=self.progress;let work=self.work.max(u64::from(result.is_err()));total_work+=work;
   if progress!=Default::default(){self.operation=Some(operation)}
   if progress!=Default::default()||result.is_err(){cx.consume_fuel(work)}
   let sum=match total.checked_add(progress){Ok(sum)=>sum,Err(error)=>{if let Ok(Some(output))=result{self.restore_output(output)}self.progress=total;self.work=total_work;return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx))}};
   match result{
    Ok(output)=>{if let Err(error)=cx.consume_retained(progress){if let Some(output)=output{self.restore_output(output)}self.progress=sum;self.work=total_work;return Err(error.into())}total=sum;if let Some(output)=output{self.progress=total;self.work=total_work;return Ok(Some(output))}if progress==Default::default(){self.progress=total;self.work=total_work;return Ok(None)}},
    Err(error)=>{self.progress=sum;self.work=total_work;return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx))},
   }
  }self.progress=total;self.work=total_work;Ok(None)
 }

 pub fn advance(&mut self,origin:&dyn FlowInvocationRequestSource,grant:RetainedCloneGrant)->Result<Option<String>,ValueError>{
  self.progress=Default::default();self.work=0;if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original invocation request is closing"))}
  let demand=self.next_demands(origin,grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
  self.source=Some(origin.invocation_source_identity());let fields=self.fields.as_mut().unwrap();
  self.work=1;self.progress.copied_items=1;
  match self.phase{
   0=>{if let Some((_,length))=self.cursor.chunk(origin,self.resume){self.length=self.length.checked_add(length).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation request length overflows"))?}else{self.cursor=Default::default();self.phase=1}},
   1=>{fields.buffer.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original invocation request allocation refused"))?;self.progress.retained_capacity_bytes=fields.buffer.capacity();self.phase=2},
   2=>{if let Some((chunk,length))=self.cursor.chunk(origin,self.resume){fields.buffer.extend_from_slice(&chunk[..length]);self.progress.copied_bytes=length}else{fields.output=Some(unsafe{String::from_utf8_unchecked(std::mem::take(&mut fields.buffer))});self.phase=3}},
   3=>{self.phase=4;return Ok(fields.output.take())},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation request was transferred")),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:String){assert_eq!(self.phase,4);self.fields.as_mut().unwrap().output=Some(output);self.phase=3}
 #[cfg(feature="testkit")]
 #[doc(hidden)]
 pub fn test_diagnostic(&self)->(u8,usize){(self.cursor.part,self.cursor.offset)}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.retirement.as_ref(){return owner.selected_demand(copy)}
  Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  self.progress=Default::default();self.work=0;if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation request close was not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(owner)=self.retirement.as_mut(){let poll=owner.step(grant.maximum_copy_bytes,effect);self.progress=owner.step_progress();let step=match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>{self.work=u64::try_from(turn.work).unwrap_or(u64::MAX);turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?},semio_framework_value::RetainedWorkPoll::Pending{work}=>{self.work=u64::try_from(work).unwrap_or(u64::MAX);return Ok(RetainedCloneStep::Progress(Default::default()))}};if owner.terminal_is_empty(){self.retirement=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
  self.retirement=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!("invocation request fields support controlled retirement")));self.work=1;self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowInvocationRequestWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original invocation request requires explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}



/// 📡️ The native nested answer borrows both genuine origins and its actual output through one canonical wire projection.
pub struct FlowInvocationExternalResumeSource<'a>{pub outer:&'a dyn FlowInvocationRequestSource,pub child:&'a dyn FlowInvocationRequestSource,pub output:&'a semio_framework_value::DslValue}
impl semio_framework_pack_json::JsonWriteSource for FlowInvocationExternalResumeSource<'_>{
 fn node_at_path<'a>(&'a self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'a>,ValueError>{
  use semio_framework_pack_json::{JsonWriteSource,JsonWriteNode as N};if path.first()==Some(&10)&&path.get(1)==Some(&4){return self.output.node_at_path(&path[2..])}
  Ok(match path{[]=>N::Object(11),[0]=>N::String(self.outer.neuron_id()),[1]=>N::String(self.outer.operator_id()),[2]|[3]=>N::String(""),[4]=>N::String(self.outer.operator_version()),[5]=>N::Number(semio_framework_value::Number::UInt(self.outer.node_hash())),[6]=>N::Bool(true),[7]=>N::String(self.outer.window_id()),[8]=>N::String(self.outer.window_kind_id()),[9]=>N::String(self.outer.extension_id()),[10]=>N::Object(5),[10,0]=>N::String(self.child.neuron_id()),[10,1]=>N::String(self.child.extension_id()),[10,2]=>N::String(self.child.operator_id()),[10,3]=>N::Number(semio_framework_value::Number::UInt(self.child.node_hash())),_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original nested evaluation request path is absent"))})
 }
 fn object_key_at_path<'a>(&'a self,path:&[usize],index:usize)->Result<&'a str,ValueError>{
  use semio_framework_pack_json::JsonWriteSource;let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original nested evaluation request key is absent");if path.first()==Some(&10)&&path.get(1)==Some(&4){return self.output.object_key_at_path(&path[2..],index)}if path.is_empty(){return ["neuronId","operatorId","inputJson","dependencyJson","operatorVersion","nodeHash","resume","windowId","windowKindId","extensionId","externalResult"].get(index).copied().ok_or_else(invalid)}if path==[10]{return ["neuronId","extensionId","operatorId","nodeHash","output"].get(index).copied().ok_or_else(invalid)}Err(invalid())
 }
}
