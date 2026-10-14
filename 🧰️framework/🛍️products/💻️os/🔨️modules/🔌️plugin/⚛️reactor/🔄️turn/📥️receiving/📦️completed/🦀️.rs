//! 📦️ The admitted original Completed event and correlation remain resident through bounded decoding and paid close.
use super::*;
use semio_framework_value::{DslValue,RetainedCloneGrant};
use semio_framework_job::{OriginalNativeDecodeSource,OriginalNativeDecodePolicy,OriginalNativeDecodeIssued};
use semio_framework_pack_json::{JsonGrammarCursor,JsonSourceCursor,JsonReadLimits,JsonMemberPolicy};
use semio_framework_pack::record::intrinsic::{RetainedIntrinsicDocument,RetainedIntrinsicInput};
#[path="⚠️fault/🦀️.rs"]
mod fault;
#[path="⚠️failure/🦀️.rs"]
mod failure;
use failure::DecodeFailure;
const MAXIMUM:usize=semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES;
type Grammar=JsonGrammarCursor<DslValue>;
pub(super) enum CompletedResult{Response(ResponseFields),Request(Result<Vec<u8>,semio_framework::Fault>)}
#[derive(semio_framework_value::RetireOwned)]
struct Inputs{event:Option<Event>,continuation:Option<requests::ExtensionContinuation>,correlation:Option<DslValue>,output:Option<DslValue>,fault_value:Option<DslValue>,fault:Option<semio_framework::Fault>,fields:Vec<(String,DslValue)>,wire:Vec<u8>,partial:Vec<u8>}
pub(super) struct CompletedInput{inputs:ManuallyDrop<Option<Inputs>>,closing:ManuallyDrop<Option<ControlledRetirement<Inputs>>>,native:ManuallyDrop<OriginalNativeDecodeSource>,grammar:ManuallyDrop<Option<Grammar>>,grammar_close:ManuallyDrop<Option<ControlledRetirement<Grammar>>>,pack:ManuallyDrop<Option<RetainedIntrinsicDocument<'static>>>,binding:ManuallyDrop<Option<fault::FaultCursor>>,issue:ManuallyDrop<Option<DecodeFailure>>,issue_close:ManuallyDrop<Option<ControlledRetirement<DecodeFailure>>>,phase:u8,field:usize,string_offset:usize,string_cost:usize,wire_offset:usize,fields_reserved:bool,closed:bool}
impl CompletedInput{
 fn with_inputs(event:Event,continuation:Option<requests::ExtensionContinuation>,partial:Vec<u8>,phase:u8)->Self{Self{inputs:ManuallyDrop::new(Some(Inputs{event:Some(event),continuation,correlation:None,output:None,fault_value:None,fault:None,fields:Vec::new(),wire:Vec::new(),partial})),closing:ManuallyDrop::new(None),native:ManuallyDrop::new(OriginalNativeDecodeSource::empty(OriginalNativeDecodePolicy{maximum_owned_bytes:MAXIMUM})),grammar:ManuallyDrop::new(None),grammar_close:ManuallyDrop::new(None),pack:ManuallyDrop::new(None),binding:ManuallyDrop::new(None),issue:ManuallyDrop::new(None),issue_close:ManuallyDrop::new(None),phase,field:0,string_offset:0,string_cost:0,wire_offset:0,fields_reserved:false,closed:false}}
 pub(super) fn new(event:Event,continuation:requests::ExtensionContinuation)->Self{Self::with_inputs(event,Some(continuation),Vec::new(),0)}
 pub(super) fn new_request(event:Event,partial:Vec<u8>)->Self{let phase=if matches!(event,Event::Completed{result:semio_framework::kernel::RequestOutcome::Err(_),..}){3}else{8};Self::with_inputs(event,None,partial,phase)}
 fn partial(inputs:&Inputs)->&[u8]{match inputs.continuation.as_ref(){Some(continuation)=>continuation.partial.as_ref().unwrap(),None=>&inputs.partial}}
 fn wire_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let inputs=self.inputs.as_mut().unwrap();let partial=Self::partial(inputs);let tail=Self::outcome_bytes(inputs);let total=partial.len().checked_add(tail.len()).filter(|size|*size<=MAXIMUM).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original answer exceeds the declared input ceiling"))?;if partial.is_empty(){return Ok(true)}if inputs.wire.capacity()==0{Self::reserve(&mut inputs.wire,total,cx)?;return Ok(false)}if self.wire_offset==total{return Ok(true)}let grant=cx.retained_grant();let length=(total-self.wire_offset).min(64);if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<length{return Ok(false)}let mut page=[0;64];for(index,byte)in page[..length].iter_mut().enumerate(){let offset=self.wire_offset+index;*byte=if offset<partial.len(){partial[offset]}else{tail[offset-partial.len()]}}inputs.wire.extend_from_slice(&page[..length]);self.wire_offset+=length;cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:length,..Default::default()})?;Ok(false)}
 fn take_wire(inputs:&mut Inputs)->Vec<u8>{if Self::partial(inputs).is_empty(){match inputs.event.as_mut().unwrap(){Event::Completed{result:semio_framework::kernel::RequestOutcome::Ok(bytes),..}=>std::mem::take(bytes),_=>unreachable!()}}else{std::mem::take(&mut inputs.wire)}}
 fn grammar_step(&mut self,correlation:bool,cx:&mut StepContext<'_>)->Result<Option<DslValue>,OriginalTurnFailure>{
  let inputs=self.inputs.as_ref().unwrap();let source=if correlation{inputs.continuation.as_ref().unwrap().request_json.as_bytes()}else{match inputs.event.as_ref().unwrap(){Event::Completed{result:semio_framework::kernel::RequestOutcome::Err(bytes),..}=>bytes.as_slice(),_=>unreachable!()}};
  if self.grammar.is_none(){if !metadata(cx)?{return Ok(None)}let(_,cursor)=JsonSourceCursor::<_,DslValue>::new(source,JsonMemberPolicy::Reject,JsonReadLimits{maximum_bytes:MAXIMUM as u64,maximum_allocation_bytes:MAXIMUM,maximum_depth:128,maximum_items:MAXIMUM as u64})?.into_grammar();*self.grammar=Some(cursor);return Ok(None)}
  let grammar=self.grammar.as_mut().unwrap();let native=&mut*self.native;
  let Some(outcome)=native.receive_step(cx,|control,grant|{let result=grammar.step_source(&source,1,control,grant,false);let progress=grammar.normal_step_progress();OriginalNativeDecodeIssued{value:Ok(result),grant,progress,work:u64::from(progress.copied_items!=0)}})?else{return Ok(None)};
  if let Some(failure)=outcome.receiving_refusal{return Err(failure)}
  match outcome.value?{Ok(value)=>Ok(value),Err(error)=>{*self.issue=Some(DecodeFailure::Json(error,outcome.progress));self.phase=250;Ok(None)}}
 }
 fn close_grammar(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(owner)=self.grammar_close.as_mut(){if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(false)}if !metadata(cx)?{return Ok(false)}self.grammar_close.take();return Ok(false)}
  if self.grammar.is_some(){if !metadata(cx)?{return Ok(false)}let source=self.grammar.take().unwrap();match ControlledRetirement::new(source){Ok(owner)=>*self.grammar_close=Some(owner),Err((error,source))=>{*self.grammar=Some(source);return Err(error.into())}}return Ok(false)}Ok(true)
 }
 fn outcome_bytes(inputs:&Inputs)->&[u8]{match inputs.event.as_ref().unwrap(){Event::Completed{result:semio_framework::kernel::RequestOutcome::Ok(bytes),..}=>bytes,_=>unreachable!()}}
 fn reserve<T>(target:&mut Vec<T>,count:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();let capacity=count.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original response extent overflow"))?;if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<capacity{return Ok(false)}cx.consume_fuel(1);target.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original response backing reserve refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:target.capacity()*std::mem::size_of::<T>(),..Default::default()})?;Ok(true)}
 pub(super) fn step(&mut self,cx:&mut StepContext<'_>)->Result<Option<CompletedResult>,OriginalTurnFailure>{
  if cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  match self.phase{
   0=>{if let Some(value)=self.grammar_step(true,cx)?{self.inputs.as_mut().unwrap().correlation=Some(value);self.phase=1}Ok(None)},
   1=>{if !self.close_grammar(cx)?{return Ok(None)}if !metadata(cx)?{return Ok(None)}let inputs=self.inputs.as_mut().unwrap();if inputs.continuation.as_ref().unwrap().partial.is_err(){let Err(fault)=std::mem::replace(&mut inputs.continuation.as_mut().unwrap().partial,Ok(Vec::new()))else{unreachable!()};inputs.fault=Some(fault);self.phase=5;}else{self.phase=if matches!(inputs.event,Some(Event::Completed{result:semio_framework::kernel::RequestOutcome::Err(_),..})){3}else{2}}Ok(None)},
   2=>self.pack_step(cx),
   3=>{if let Some(value)=self.grammar_step(false,cx)?{self.inputs.as_mut().unwrap().fault_value=Some(value);self.phase=4}Ok(None)},
   4=>{if self.binding.is_none(){if !metadata(cx)?{return Ok(None)}*self.binding=Some(fault::FaultCursor::empty());return Ok(None)}let inputs=self.inputs.as_mut().unwrap();if let Some(fault)=self.binding.as_mut().unwrap().step(inputs.fault_value.as_mut().unwrap(),cx)?{inputs.fault=Some(fault);self.phase=if inputs.continuation.is_some(){5}else{9}}Ok(None)},
   5=>self.fields_step(cx),
   6=>{if !metadata(cx)?{return Ok(None)}let inputs=self.inputs.as_mut().unwrap();let action=std::mem::take(&mut inputs.continuation.as_mut().unwrap().response_action);let args=DslValue::Object(std::mem::take(&mut inputs.fields));let fault=inputs.fault.take();self.phase=7;Ok(Some(CompletedResult::Response((Some(action),Some(args),fault))))},
   7=>Ok(None),
   8=>{if !self.wire_step(cx)?||!metadata(cx)?{return Ok(None)}let bytes=Self::take_wire(self.inputs.as_mut().unwrap());self.phase=7;Ok(Some(CompletedResult::Request(Ok(bytes))))},
   9=>{if !metadata(cx)?{return Ok(None)}let fault=self.inputs.as_mut().unwrap().fault.take().unwrap();self.phase=7;Ok(Some(CompletedResult::Request(Err(fault))))},
   250=>{if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.issue,cx)?{return Err(failure)}Ok(None)},
   _=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original response decoder phase changed").into())
  }
 }
 fn pack_step(&mut self,cx:&mut StepContext<'_>)->Result<Option<CompletedResult>,OriginalTurnFailure>{
  if self.pack.is_none(){
   if !self.wire_step(cx)?||!metadata(cx)?{return Ok(None)}let inputs=self.inputs.as_mut().unwrap();let bytes=Self::take_wire(inputs);
   let limits=semio_framework_pack::PackLimits{max_file_len:MAXIMUM as u64,max_segment_len:MAXIMUM as u64,max_symbols:MAXIMUM as u32,max_depth:128,max_items:MAXIMUM as u64,max_total_alloc:MAXIMUM as u64};
   match RetainedIntrinsicDocument::new(RetainedIntrinsicInput::OwnedBytes(bytes),limits,MAXIMUM){Ok(owner)=>*self.pack=Some(owner),Err((error,RetainedIntrinsicInput::OwnedBytes(bytes)))=>{inputs.wire=bytes;*self.issue=Some(DecodeFailure::Pack(error,Default::default()));self.phase=250},Err(_)=>unreachable!()}return Ok(None)
  }
  let owner=self.pack.as_mut().unwrap();let demand=match owner.original_step_demands(){Ok(demand)=>demand,Err(error)=>{*self.issue=Some(DecodeFailure::Pack(error,Default::default()));self.phase=250;return Ok(None)}};let grant=cx.retained_grant();if !permits(cx,grant,demand){return Ok(None)}let call=owner.advance_original_step(RetainedCloneGrant{maximum_items:1,..grant},false);let progress=call.progress;cx.consume_fuel(u64::from(progress.copied_items!=0));cx.consume_retained(progress)?;match call.result{Ok(step)if step.complete=>{if !metadata(cx)?{return Ok(None)}self.inputs.as_mut().unwrap().output=owner.take_output();self.phase=5},Ok(_)=>{},Err(error)=>{*self.issue=Some(DecodeFailure::Pack(error,progress));self.phase=250}}Ok(None)
 }
 fn fields_step(&mut self,cx:&mut StepContext<'_>)->Result<Option<CompletedResult>,OriginalTurnFailure>{
  let inputs=self.inputs.as_mut().unwrap();let count=match inputs.correlation.as_ref(){Some(DslValue::Object(fields))=>fields.len(),_=>0};
  if !self.fields_reserved{if !Self::reserve(&mut inputs.fields,count+2,cx)?{return Ok(None)}self.fields_reserved=true;return Ok(None)}
  if self.field<count{let Some(DslValue::Object(fields))=inputs.correlation.as_mut()else{unreachable!()};let(name,value)=&mut fields[self.field];let ignored=matches!(name.as_str(),"ok"|"output"|"fault"|"faultCode"|"faultMessage");if let DslValue::String(text)=value{if !ignored&&self.string_offset<text.len(){if !metadata(cx)?{return Ok(None)}let next=text[self.string_offset..].chars().next().unwrap();self.string_offset+=next.len_utf8();self.string_cost=self.string_cost.saturating_add(semio_framework::public_invocation_char_cost(next));return Ok(None)}}if !metadata(cx)?{return Ok(None)}if !ignored&&self.string_cost<=semio_framework::PUBLIC_INVOCATION_STRING_BYTES{inputs.fields.push((std::mem::take(name),std::mem::replace(value,DslValue::Null)))}self.field+=1;self.string_offset=0;self.string_cost=0;return Ok(None)}
  let name=if self.field==count{"ok"}else{"output"};let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<name.len()||grant.maximum_capacity_bytes<name.len(){return Ok(None)}let mut key=String::new();cx.consume_fuel(1);key.try_reserve_exact(name.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original response key reserve refused"))?;key.push_str(name);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:name.len(),retained_capacity_bytes:key.capacity(),..Default::default()})?;let value=if self.field==count{DslValue::Bool(inputs.fault.is_none())}else{inputs.output.take().unwrap_or(DslValue::Null)};inputs.fields.push((key,value));self.field+=1;if self.field==count+2{self.phase=6}Ok(None)
 }
 pub(super) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(owner)=self.pack.as_mut(){if !owner.terminal_is_empty(){let grant=cx.retained_grant();let demand=owner.original_close_demands(grant.maximum_copy_bytes).map_err(|error|ValueError::literal(error.kind(),"original intrinsic close quote refused"))?;if !permits(cx,grant,demand){return Ok(false)}let call=owner.close_original_step(RetainedCloneGrant{maximum_items:1,..grant});cx.consume_fuel(u64::from(call.progress.copied_items!=0));cx.consume_retained(call.progress)?;if let Err(error)=call.result{*self.issue=Some(DecodeFailure::Pack(error,call.progress));return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original intrinsic close retained its refusal").into())}return Ok(false)}if !metadata(cx)?{return Ok(false)}self.pack.take();return Ok(false)}
  if !self.close_grammar(cx)?{return Ok(false)}
  if let Some(binding)=self.binding.as_mut(){if !binding.close_step(cx)?{return Ok(false)}if !metadata(cx)?{return Ok(false)}self.binding.take();return Ok(false)}
  if let Some(owner)=self.closing.as_mut(){if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(false)}if !metadata(cx)?{return Ok(false)}self.closing.take();return Ok(false)}
  if self.inputs.is_some(){if !metadata(cx)?{return Ok(false)}let inputs=self.inputs.take().unwrap();match ControlledRetirement::new(inputs){Ok(owner)=>*self.closing=Some(owner),Err((error,inputs))=>{*self.inputs=Some(inputs);return Err(error.into())}}return Ok(false)}
  if let Some(owner)=self.issue_close.as_mut(){if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(false)}if !metadata(cx)?{return Ok(false)}self.issue_close.take();return Ok(false)}
  if self.issue.is_some(){if !metadata(cx)?{return Ok(false)}let issue=self.issue.take().unwrap();match ControlledRetirement::new(issue){Ok(owner)=>*self.issue_close=Some(owner),Err((error,issue))=>{*self.issue=Some(issue);return Err(error.into())}}return Ok(false)}
  if !self.native.terminal_is_empty(){self.native.close_step(cx)?;return Ok(false)}
  if !metadata(cx)?{return Ok(false)}self.closed=true;Ok(true)
 }
}
fn permits(cx:&StepContext<'_>,grant:RetainedCloneGrant,demand:semio_framework_value::RetirementDemand)->bool{!cx.should_yield()&&grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
impl Drop for CompletedInput{fn drop(&mut self){assert!(std::thread::panicking()||self.closed,"original Completed source requires complete input and decoder close")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
