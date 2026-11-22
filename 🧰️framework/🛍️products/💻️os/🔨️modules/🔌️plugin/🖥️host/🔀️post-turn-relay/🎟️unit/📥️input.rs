//! 📥️ Inference payload bytes stay borrowed under the schema-owned Job input view.
use semio_framework_pack_json::JsonWriteSource;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"inference job Source path is absent")}

use super::{GuestRelayUnitFlow,SourceJsonEncoder};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome,retirement::controlled::ControlledRetirement};
use semio_plugin_host_guest_runtime::{GuestInstance,GuestRuntimes};
use std::mem::size_of;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Payload,PayloadClose,Job,Ready,Closing}
/// 🪢️ The actual first-party payload and its tagged Job donor remain in this same Source until paid closure.
pub struct GuestRelayInferenceInput{encoder:SourceJsonEncoder,payload:Option<Vec<u8>>,payload_retirement:Option<ControlledRetirement<Vec<u8>>>,unit:GuestRelayUnitFlow,phase:Phase,job:u64,kind:&'static str,raw_binding:Option<semio_framework_job::OriginalSourceBinding>,raw_pointer:usize,raw_length:usize}
impl GuestRelayInferenceInput{
 pub fn new(job:u64,kind:&'static str)->Result<Self,ValueError>{if job==0||kind.is_empty(){return Err(refused())}Ok(Self{encoder:SourceJsonEncoder::new(),payload:None,payload_retirement:None,unit:GuestRelayUnitFlow::empty(),phase:Phase::Payload,job,kind,raw_binding:None,raw_pointer:0,raw_length:0})}
 /// 🔤️ The application payload uses its native JSON schema before compact Job framing.
 pub fn encode_step<S:JsonWriteSource+?Sized>(&mut self,input:&S,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.phase==Phase::Closing||self.raw_binding.is_some(){return Err(refused().into())}if self.phase==Phase::Ready{return Ok(true)}let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<Phase>()||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Phase>(),..Default::default()})?;cx.consume_fuel(1);
  match self.phase{Phase::Payload=>{if self.encoder.advance(input,&mut self.payload,cx)?{self.phase=Phase::PayloadClose}},Phase::PayloadClose=>{if self.encoder.close_step(cx)?{self.phase=Phase::Job}},Phase::Job=>{if self.unit.encode_job_frame_step(self.job,self.kind,self.payload.as_deref().ok_or_else(refused)?,cx)?{self.phase=Phase::Ready}},_=>{}}Ok(self.phase==Phase::Ready)
 }
 /// 📄️ The exact borrowed application payload fills its compact Job donor in paid pages.
 pub fn encode_bytes_step(&mut self,bytes:&[u8],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if self.phase==Phase::Closing{return Err(refused().into())}
  if let Some(binding)=self.raw_binding{
   if !binding.matches(cx)||self.raw_pointer!=bytes.as_ptr() as usize||self.raw_length!=bytes.len(){return Err(refused().into())}
  }else{
   if self.phase!=Phase::Payload||!self.encoder.terminal_is_empty()||self.payload.is_some(){return Err(refused().into())}
   let grant=cx.retained_grant();let meta=size_of::<Option<semio_framework_job::OriginalSourceBinding>>()+2*size_of::<usize>();
   if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<meta||grant.maximum_depth==0{return Ok(false)}
   cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:meta,..Default::default()})?;
   cx.consume_fuel(1);
   self.raw_binding=Some(semio_framework_job::OriginalSourceBinding::borrow(cx));
   self.raw_pointer=bytes.as_ptr() as usize;
   self.raw_length=bytes.len();
   return Ok(false)
  }
  if self.phase==Phase::Ready{return Ok(true)}
  let grant=cx.retained_grant();let meta=size_of::<Phase>();
  if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<meta||grant.maximum_depth==0{return Ok(false)}
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:meta,..Default::default()})?;
  cx.consume_fuel(1);
  if !self.unit.encode_job_frame_step(self.job,self.kind,bytes,cx)?{return Ok(false)}
  self.phase=Phase::Ready;
  Ok(true)
 }
 pub fn unit(&self)->&GuestRelayUnitFlow{&self.unit}
 pub fn unit_mut(&mut self)->&mut GuestRelayUnitFlow{&mut self.unit}
 /// 🧹️ Returns a live donor through its actual instance before retiring the independently encoded payload.
 pub fn close_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.unit.terminal_is_empty(){if !self.unit.close_step(runtime,instance,cx)?{return Ok(false)}}self.close_unopened(cx)}
 pub fn close_unopened(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.phase!=Phase::Closing{let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<Phase>()||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Phase>(),..Default::default()})?;cx.consume_fuel(1);self.phase=Phase::Closing;}if !self.unit.terminal_is_empty(){self.unit.close_unopened(cx)?;return Ok(false)}if !self.encoder.terminal_is_empty(){self.encoder.close_step(cx)?;return Ok(false)}
  if let Some(owner)=self.payload_retirement.as_mut(){if !owner.terminal_is_empty(){let grant=cx.retained_grant();let body=grant.maximum_copy_bytes;let Some((poll,_))=cx.with_original_effect(grant,|_,effect|owner.step(body,effect))? else{return Ok(false)};let turn=match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,semio_framework_value::RetainedWorkPoll::Pending{work}=>semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),work}};semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(false)}self.payload_retirement.take();return Ok(false)}
  if self.payload.is_some(){let bytes=size_of::<Option<Vec<u8>>>()+size_of::<Option<ControlledRetirement<Vec<u8>>>>();let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.payload_retirement=Some(ControlledRetirement::retain(self.payload.take().unwrap()));return Ok(false)}if let Some(binding)=self.raw_binding{if !binding.matches(cx){return Err(refused().into())}let bytes=size_of::<Option<semio_framework_job::OriginalSourceBinding>>()+2*size_of::<usize>();let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.raw_binding=None;self.raw_pointer=0;self.raw_length=0;return Ok(false)}Ok(true)
 }
 pub fn terminal_is_empty(&self)->bool{self.encoder.terminal_is_empty()&&self.payload.is_none()&&self.payload_retirement.is_none()&&self.unit.terminal_is_empty()&&self.raw_binding.is_none()}
}
impl Drop for GuestRelayInferenceInput{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"inference input requires its actual Source-funded retirement")}}

#[cfg(test)]
mod tests{
 use super::*;use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};use semio_framework_value::{DslValue,RetainedCloneGrant};
 fn clock()->Option<u64>{Some(1000)}
 fn reference_frame(job:u64,kind:&str,bytes:&[u8])->Vec<u8>{
  let mut expected=b"SJS1\x01\x00\x00\x00".to_vec();
  for field in [job,kind.len() as u64,bytes.len() as u64,0]{expected.extend(field.to_le_bytes());}
  expected.extend(kind.as_bytes());
  expected.extend(bytes);
  expected
 }
 #[test]
 fn source_encodes_payload_and_tagged_job_then_retires_both_backings(){
  let law:serde_json::Value=serde_json::from_str(include_str!("law.json")).unwrap();let payload=DslValue::String(law["value"].as_str().unwrap().into());let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(794),Generation(1),grant,clock,1000000,1000000);let mut input=GuestRelayInferenceInput::new(7,"semio.infer").unwrap();let mut epoch=0;let mut ready=false;
  for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;ready=caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;input.encode_step(&payload,cx)}).unwrap();if ready{break}}assert!(ready);let wire=input.unit.encoded().unwrap();assert_eq!(wire,reference_frame(7,"semio.infer",&serde_json::to_vec(&law["value"]).unwrap()));
  caller.cancel().cancel_now();for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|input.close_unopened(cx))}).unwrap(){break}}assert!(input.terminal_is_empty());caller.retire(grant);
 }
 #[test]
 fn binary_job_input_retains_exact_bytes_and_matches_serde(){
  let law:serde_json::Value=serde_json::from_str(include_str!("law.json")).unwrap();let bytes=[0,255,13,10];let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(795),Generation(1),grant,clock,1000000,1000000);let mut input=GuestRelayInferenceInput::new(7,"semio.mutation-plan").unwrap();let mut epoch=0;let mut ready=false;
  for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;ready=caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;input.encode_bytes_step(&bytes,cx)}).unwrap();if ready{break}}assert!(ready);assert_eq!(input.unit.encoded().unwrap(),reference_frame(7,"semio.mutation-plan",&bytes));
  caller.cancel().cancel_now();for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|input.close_unopened(cx))}).unwrap(){break}}assert!(input.terminal_is_empty());caller.retire(grant);
 }

}
