//! 📥️ Inference payload bytes stay borrowed under the schema-owned Job input view.
use semio_framework_pack_json::{JsonWriteNode,JsonWriteSource};
use semio_framework_value::{Number,ValueError,ValueRefusalKind};
/// 💡️ Presents the actual retained inference payload as the original job source without allocating an envelope.
pub struct GuestRelayInferenceSource<'a>{pub job:u64,pub kind:&'static str,pub payload:&'a[u8]}
impl JsonWriteSource for GuestRelayInferenceSource<'_>{
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{match path{
  []=>Ok(JsonWriteNode::Object(4)),[0]=>Ok(JsonWriteNode::Number(Number::UInt(self.job))),[1]=>Ok(JsonWriteNode::String(self.kind)),[2]=>Ok(JsonWriteNode::Array(self.payload.len())),[2,index]=>self.payload.get(*index).map(|value|JsonWriteNode::Number(Number::UInt(u64::from(*value)))).ok_or_else(refused),[3]=>Ok(JsonWriteNode::Null),_=>Err(refused())
 }}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if !path.is_empty(){return Err(refused())}match index{0=>Ok("job"),1=>Ok("kind"),2=>Ok("input"),3=>Ok("checkpoint"),_=>Err(refused())}}
}
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"inference job Source path is absent")}

use super::{GuestRelayJobInput,GuestRelayUnitFlow,SourceJsonEncoder};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome,retirement::controlled::ControlledRetirement};
use semio_plugin_host_guest_runtime::{GuestInstance,GuestRuntimes};
use std::mem::size_of;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Payload,PayloadClose,Job,Ready,Closing}
/// 🪢️ The actual first-party payload and its tagged Job donor remain in this same Source until paid closure.
pub struct GuestRelayInferenceInput{encoder:SourceJsonEncoder,payload:Option<Vec<u8>>,payload_retirement:Option<ControlledRetirement<Vec<u8>>>,unit:GuestRelayUnitFlow,phase:Phase,job:u64,kind:&'static str,raw_binding:Option<semio_framework_job::OriginalSourceBinding>,raw_pointer:usize,raw_length:usize,raw_cursor:usize}
impl GuestRelayInferenceInput{
 pub fn new(job:u64,kind:&'static str)->Result<Self,ValueError>{if job==0||kind.is_empty(){return Err(refused())}Ok(Self{encoder:SourceJsonEncoder::new(),payload:None,payload_retirement:None,unit:GuestRelayUnitFlow::empty(),phase:Phase::Payload,job,kind,raw_binding:None,raw_pointer:0,raw_length:0,raw_cursor:0})}
 /// 🔤️ Both JSON layers use the actual Source encoder, retaining their separate physical byte owners.
 pub fn encode_step<S:JsonWriteSource+?Sized>(&mut self,input:&S,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.phase==Phase::Closing||self.raw_binding.is_some(){return Err(refused().into())}if self.phase==Phase::Ready{return Ok(true)}let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<Phase>()||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Phase>(),..Default::default()})?;cx.consume_fuel(1);
  match self.phase{Phase::Payload=>{if self.encoder.advance(input,&mut self.payload,cx)?{self.phase=Phase::PayloadClose}},Phase::PayloadClose=>{if self.encoder.close_step(cx)?{self.phase=Phase::Job}},Phase::Job=>{let source=GuestRelayInferenceSource{job:self.job,kind:self.kind,payload:self.payload.as_deref().ok_or_else(refused)?};if self.unit.encode_step(&GuestRelayJobInput(&source),cx)?{self.phase=Phase::Ready}},_=>{}}Ok(self.phase==Phase::Ready)
 }
 /// 📄️ Copies one bounded borrowed binary span into this Source before encoding its tagged Job envelope.
 pub fn encode_bytes_step(&mut self,bytes:&[u8],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.phase==Phase::Closing{return Err(refused().into())}if self.phase==Phase::Ready{return Ok(true)}
  if self.phase==Phase::Payload{
   if self.raw_binding.is_none(){let grant=cx.retained_grant();let meta=size_of::<Option<semio_framework_job::OriginalSourceBinding>>()+3*size_of::<usize>();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<meta||grant.maximum_capacity_bytes<bytes.len()||grant.maximum_depth==0{return Ok(false)}let mut payload=Vec::new();payload.try_reserve_exact(bytes.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"job input allocation failed"))?;let capacity=payload.capacity();self.payload=Some(payload);self.raw_binding=Some(semio_framework_job::OriginalSourceBinding::borrow(cx));self.raw_pointer=bytes.as_ptr()as usize;self.raw_length=bytes.len();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:meta,retained_capacity_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
   if !self.raw_binding.unwrap().matches(cx)||self.raw_pointer!=bytes.as_ptr()as usize||self.raw_length!=bytes.len(){return Err(refused().into())}let grant=cx.retained_grant();let count=(bytes.len()-self.raw_cursor).min(grant.maximum_copy_bytes.saturating_sub(size_of::<usize>())).min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));if self.raw_cursor<bytes.len(){if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||count==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count+size_of::<usize>(),..Default::default()})?;cx.consume_fuel(count as u64+1);self.payload.as_mut().ok_or_else(refused)?.extend_from_slice(&bytes[self.raw_cursor..self.raw_cursor+count]);self.raw_cursor+=count;return Ok(false)}
   let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<Phase>()||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Phase>(),..Default::default()})?;cx.consume_fuel(1);self.phase=Phase::Job;return Ok(false)
  }
  if self.phase!=Phase::Job{return Err(refused().into())}let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<Phase>()||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Phase>(),..Default::default()})?;cx.consume_fuel(1);let source=GuestRelayInferenceSource{job:self.job,kind:self.kind,payload:self.payload.as_deref().ok_or_else(refused)?};if self.unit.encode_step(&GuestRelayJobInput(&source),cx)?{self.phase=Phase::Ready;return Ok(true)}Ok(false)
 }
 pub fn unit(&self)->&GuestRelayUnitFlow{&self.unit}
 pub fn unit_mut(&mut self)->&mut GuestRelayUnitFlow{&mut self.unit}
 /// 🧹️ Returns a live donor through its actual instance before retiring the independently encoded payload.
 pub fn close_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.unit.terminal_is_empty(){if !self.unit.close_step(runtime,instance,cx)?{return Ok(false)}}self.close_unopened(cx)}
 pub fn close_unopened(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.phase!=Phase::Closing{let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<Phase>()||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Phase>(),..Default::default()})?;cx.consume_fuel(1);self.phase=Phase::Closing;}if !self.unit.terminal_is_empty(){self.unit.close_unopened(cx)?;return Ok(false)}if !self.encoder.terminal_is_empty(){self.encoder.close_step(cx)?;return Ok(false)}
  if let Some(owner)=self.payload_retirement.as_mut(){if !owner.terminal_is_empty(){let result=owner.step(cx.retained_grant());let work=owner.last_turn_work();semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work})?;return Ok(false)}self.payload_retirement.take();return Ok(false)}
  if self.payload.is_some(){let bytes=size_of::<Option<Vec<u8>>>()+size_of::<Option<ControlledRetirement<Vec<u8>>>>();let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.payload_retirement=Some(ControlledRetirement::retain(self.payload.take().unwrap()));return Ok(false)}if let Some(binding)=self.raw_binding{if !binding.matches(cx){return Err(refused().into())}let bytes=size_of::<Option<semio_framework_job::OriginalSourceBinding>>()+3*size_of::<usize>();let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.raw_binding=None;self.raw_pointer=0;self.raw_length=0;self.raw_cursor=0;return Ok(false)}Ok(true)
 }
 pub fn terminal_is_empty(&self)->bool{self.encoder.terminal_is_empty()&&self.payload.is_none()&&self.payload_retirement.is_none()&&self.unit.terminal_is_empty()&&self.raw_binding.is_none()}
}
impl Drop for GuestRelayInferenceInput{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"inference input requires its actual Source-funded retirement")}}

#[cfg(test)]
mod tests{
 use super::*;use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};use semio_framework_value::{DslValue,RetainedCloneGrant};
 fn clock()->Option<u64>{Some(1000)}
 #[test]
 fn source_encodes_payload_and_tagged_job_then_retires_both_backings(){
  let law:serde_json::Value=serde_json::from_str(include_str!("law.json")).unwrap();let payload=DslValue::String(law["value"].as_str().unwrap().into());let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(794),Generation(1),grant,clock,1000000,1000000);let mut input=GuestRelayInferenceInput::new(7,"semio.infer").unwrap();let mut epoch=0;let mut ready=false;
  for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;ready=caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;input.encode_step(&payload,cx)}).unwrap();if ready{break}}assert!(ready);let wire=input.unit.encoded().unwrap();let actual:serde_json::Value=serde_json::from_slice(wire).unwrap();let expected=serde_json::json!({"Job":{"job":7,"kind":"semio.infer","input":serde_json::to_vec(&law["value"]).unwrap(),"checkpoint":null}});assert_eq!(actual,expected);
  caller.cancel().cancel_now();for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|input.close_unopened(cx))}).unwrap(){break}}assert!(input.terminal_is_empty());caller.retire(grant);
 }
 #[test]
 fn binary_job_input_retains_exact_bytes_and_matches_serde(){
  let law:serde_json::Value=serde_json::from_str(include_str!("law.json")).unwrap();let bytes=[0,255,13,10];let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(795),Generation(1),grant,clock,1000000,1000000);let mut input=GuestRelayInferenceInput::new(7,"semio.mutation-plan").unwrap();let mut epoch=0;let mut ready=false;
  for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;ready=caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;input.encode_bytes_step(&bytes,cx)}).unwrap();if ready{break}}assert!(ready);let actual:serde_json::Value=serde_json::from_slice(input.unit.encoded().unwrap()).unwrap();assert_eq!(actual,serde_json::json!({"Job":{"job":7,"kind":"semio.mutation-plan","input":bytes,"checkpoint":null}}));
  caller.cancel().cancel_now();for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|input.close_unopened(cx))}).unwrap(){break}}assert!(input.terminal_is_empty());caller.retire(grant);
 }

}
