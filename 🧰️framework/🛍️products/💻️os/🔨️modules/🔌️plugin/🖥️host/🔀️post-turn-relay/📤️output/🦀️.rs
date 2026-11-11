//! 📤️ A result remains in its actual Source until a receiving owner returns the generation-qualified capsule.
use semio_framework_job::{OriginalNativeLease,OriginalNativeRetirement,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,size_of};
use semio_plugin_host_wake::GuestRelayAdmissionFailure;
pub struct GuestRelayOutputHome{bytes:ManuallyDrop<Vec<u8>>,expected:Option<usize>,sealed:bool}
impl GuestRelayOutputHome{
 pub fn empty()->Self{Self{bytes:ManuallyDrop::new(Vec::new()),expected:None,sealed:false}}
 /// 📏️ Admits one exact result backing before any page changes its actual source-owned length.
 pub fn prepare(&mut self,length:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if length>semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES{return Err(refused("guest result exceeds the declared operation payload extent").into())}if let Some(expected)=self.expected{if expected!=length{return Err(refused("guest result changed its admitted output extent").into())}return Ok(true)}
  let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<length||grant.maximum_copy_bytes<size_of::<Option<usize>>(){return Ok(false)}
  self.bytes.try_reserve_exact(length).map_err(|_|refused("guest result backing allocation failed"))?;self.expected=Some(length);
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<usize>>(),retained_capacity_bytes:self.bytes.capacity(),..Default::default()})?;cx.consume_fuel(1);Ok(true)
 }
 /// 📄️ Copies only the next paid span of an actual frozen guest page and retains the cursor on refusal.
 pub fn append_page(&mut self,offset:usize,page:&[u8],cursor:&mut usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let expected=self.expected.ok_or_else(||refused("guest output has no admitted backing"))?;if self.sealed||*cursor>page.len()||offset.checked_add(*cursor)!=Some(self.bytes.len())||offset.checked_add(page.len()).is_none_or(|end|end>expected){return Err(refused("guest page changes its original offset, extent or sealed receiver").into())}if *cursor==page.len(){return Ok(true)}
  let grant=cx.retained_grant();let metadata=2*size_of::<usize>();let count=(page.len()-*cursor).min(grant.maximum_copy_bytes.saturating_sub(metadata)).min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||count==0{return Ok(false)}
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count+metadata,..Default::default()})?;cx.consume_fuel(u64::try_from(count).unwrap_or(u64::MAX)+1);self.bytes.extend_from_slice(&page[*cursor..*cursor+count]);*cursor+=count;Ok(*cursor==page.len())
 }
 /// 🔒️ Publishes only the fully copied result after the caller has received its real guest protocol acknowledgement.
 pub fn seal(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.sealed{return Ok(true)}if self.expected!=Some(self.bytes.len()){return Err(refused("guest output cannot seal before its exact admitted extent is received").into())}let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<bool>()||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()})?;cx.consume_fuel(1);self.sealed=true;Ok(true)
 }
 pub fn is_sealed(&self)->bool{self.sealed}
 pub fn bytes(&self)->Result<&[u8],ValueError>{if !self.sealed{return Err(refused("guest result is not yet sealed by its actual producer"))}Ok(&self.bytes)}
}
impl OriginalNativeRetirement for GuestRelayOutputHome{
 fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if !self.bytes.is_empty(){let metadata=size_of::<usize>();if grant.maximum_copy_bytes<=metadata{return Ok(false)}let count=self.bytes.len().min(grant.maximum_copy_bytes-metadata);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count+metadata,..Default::default()})?;cx.consume_fuel(1);let keep=self.bytes.len()-count;self.bytes.truncate(keep);return Ok(false)}
  if self.bytes.capacity()!=0{let capacity=self.bytes.capacity();if grant.maximum_copy_bytes<size_of::<Vec<u8>>()||grant.maximum_release_bytes<capacity{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<u8>>(),released_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);drop(std::mem::take(&mut*self.bytes));return Ok(false)}
  if self.expected.is_some()||self.sealed{let bytes=size_of::<Option<usize>>()+size_of::<bool>();if grant.maximum_copy_bytes<bytes{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.expected=None;self.sealed=false;}Ok(true)
 }
 fn terminal_is_empty(&self)->bool{self.bytes.is_empty()&&self.bytes.capacity()==0&&self.expected.is_none()&&!self.sealed}
}
impl Drop for GuestRelayOutputHome{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"guest result backing requires its actual Source-funded return");if self.bytes.is_empty()&&self.bytes.capacity()==0{unsafe{ManuallyDrop::drop(&mut self.bytes)}}}}
pub struct GuestRelayOutput{pub(super) native:Option<OriginalNativeLease<GuestRelayOutputHome>>}
impl GuestRelayOutput{
 /// 🪢️ Transfers an existing paid result lease only after its actual producer sealed the complete bytes.
 pub fn take_sealed_step(native:&mut Option<OriginalNativeLease<GuestRelayOutputHome>>,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
  let lease=native.as_mut().ok_or_else(||refused("guest result capsule has no original Source lease"))?;let mut sealed=false;if !lease.with_native(cx,|home,_|{sealed=home.is_sealed();Ok(())})?||!sealed{return Ok(None)}let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<Self>()||grant.maximum_depth==0{return Ok(None)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>(),..Default::default()})?;cx.consume_fuel(1);Ok(Some(Self{native:native.take()}))
 }
 /// 👁️ A receiving callback borrows frozen bytes on their exact original Source and cannot return an owned semantic value.
 pub fn with_bytes(&mut self,cx:&mut StepContext<'_>,receive:impl FnOnce(&[u8],&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
  self.native.as_mut().ok_or_else(||refused("guest output capsule was already returned"))?.with_native(cx,|home,cx|receive(home.bytes()?,cx))
 }
 pub fn return_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{OriginalNativeLease::return_slot_step(&mut self.native,cx)}
 pub fn terminal_is_empty(&self)->bool{self.native.is_none()}
}
/// 🧯️ Full original causes stay in the admitting Source instead of escaping as formatted strings.
pub struct GuestRelayFaultHome{failure:Option<semio_framework_value::retirement::controlled::ControlledRetirement<GuestRelayAdmissionFailure>>}
impl GuestRelayFaultHome{
 pub fn empty()->Self{Self{failure:None}}
 pub fn has_failure(&self)->bool{self.failure.as_ref().is_some_and(|failure|failure.original().is_some())}
 pub fn with_failure(&self,cx:&mut StepContext<'_>,receive:impl FnOnce(&GuestRelayAdmissionFailure,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<(),OriginalTurnFailure>{receive(self.failure.as_ref().and_then(|failure|failure.original()).ok_or_else(||refused("original fault has not been captured"))?,cx)}

 /// 📥️ Captures the complete original callback failure.
 pub fn capture_turn_failure_step(&mut self,failure:&mut Option<OriginalTurnFailure>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.failure.is_some()||failure.is_none(){return Err(refused("one original fault required").into())}if !capsule_metadata(cx,size_of::<Self>(),true)?{return Ok(false)}self.failure=Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain(GuestRelayAdmissionFailure::from(failure.take().unwrap())));Ok(true)}
 pub fn capture_step(&mut self,failure:&mut Option<GuestRelayAdmissionFailure>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.failure.is_some()||failure.is_none(){return Err(refused("one original fault required").into())}if !capsule_metadata(cx,size_of::<Self>(),true)?{return Ok(false)}self.failure=Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain(failure.take().unwrap()));Ok(true)}
}
impl OriginalNativeRetirement for GuestRelayFaultHome{
 fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let Some(failure)=self.failure.as_mut()else{return Ok(true)};if !failure.terminal_is_empty(){let grant=cx.retained_grant();let body=grant.maximum_copy_bytes;let Some((poll,_))=cx.with_original_effect(grant,|_,effect|failure.step(body,effect))? else{return Ok(false)};let turn=match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,semio_framework_value::RetainedWorkPoll::Pending{work}=>semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),work}};semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(false)}if !capsule_metadata(cx,size_of::<Self>(),true)?{return Ok(false)}self.failure.take();Ok(true)}
 fn terminal_is_empty(&self)->bool{self.failure.is_none()}
}
impl Drop for GuestRelayFaultHome{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original fault requires its paid Source retirement")}}
pub enum GuestRelayFault{Native{native:Option<OriginalNativeLease<GuestRelayFaultHome>>},Guest(GuestRelayOutput),ActorFailure(crate::actor_failure::ActorFailureCapsule)}
impl GuestRelayFault{
 pub fn take_captured_step(native:&mut Option<OriginalNativeLease<GuestRelayFaultHome>>,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{let mut ready=false;if !native.as_mut().ok_or_else(||refused("fault capsule requires its original lease"))?.with_native(cx,|home,_|{ready=home.failure.as_ref().is_some_and(|failure|failure.original().is_some());Ok(())})?||!ready||!capsule_metadata(cx,size_of::<Self>(),true)?{return Ok(None)}Ok(Some(Self::Native{native:native.take()}))}
 /// 📩️ The actual failed job hands over its complete sealed producer bytes without a host-side copy or interpretation.
 pub fn take_guest_failure_step(native:&mut Option<OriginalNativeLease<GuestRelayOutputHome>>,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{if !capsule_metadata(cx,size_of::<Self>(),false)?{return Ok(None)}Ok(GuestRelayOutput::take_sealed_step(native,cx)?.map(Self::Guest))}
 pub fn with_fault(&mut self,cx:&mut StepContext<'_>,receive:impl FnOnce(&GuestRelayAdmissionFailure,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{let Self::Native{native}=self else{return Err(refused("guest fault bytes require their original typed receiving path").into())};native.as_mut().ok_or_else(||refused("fault capsule was already returned"))?.with_native(cx,|home,cx|receive(home.failure.as_ref().and_then(|failure|failure.original()).ok_or_else(||refused("fault recipient lost its original cause"))?,cx))}
 pub fn with_guest_fault_bytes(&mut self,cx:&mut StepContext<'_>,receive:impl FnOnce(&[u8],&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{match self{Self::Guest(output)=>output.with_bytes(cx,receive),Self::ActorFailure(fault)=>fault.with_bytes(cx,receive),Self::Native{..}=>Err(refused("native fault retains its complete original causes").into())}}
 pub fn return_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self{Self::Guest(output)=>output.return_step(cx),Self::ActorFailure(fault)=>fault.return_step(cx),Self::Native{native}=>OriginalNativeLease::return_slot_step(native,cx)}}
 pub fn terminal_is_empty(&self)->bool{match self{Self::Guest(output)=>output.terminal_is_empty(),Self::ActorFailure(fault)=>fault.terminal_is_empty(),Self::Native{native}=>native.is_none()}}
}
/// 📬️ The caller receives exactly one existing Source lease and retains its producer until that lease returns.
pub enum GuestRelayResult{Success(GuestRelayOutput),Failure(GuestRelayFault)}
impl GuestRelayResult{
 pub fn return_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self{Self::Success(output)=>output.return_step(cx),Self::Failure(fault)=>fault.return_step(cx)}}
 pub fn terminal_is_empty(&self)->bool{match self{Self::Success(output)=>output.terminal_is_empty(),Self::Failure(fault)=>fault.terminal_is_empty()}}
}
fn capsule_metadata(cx:&mut StepContext<'_>,bytes:usize,retirement:bool)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if (if retirement{cx.retirement_should_yield()}else{cx.should_yield()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,message)}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn actual_native_output_retains_backing_until_source_capsule_acknowledgement_and_paid_release(){
  use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
  use semio_framework_value::RetainedCloneGrant;
  fn clock()->Option<u64>{Some(1000)}
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let bytes:Vec<u8>=serde_json::from_value(law["bytes"].clone()).unwrap();let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1048576,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),grant,clock,1000000,1000000);let mut input=Some(GuestRelayOutputHome::empty());let mut native=None;
  for _ in 0..64{caller.turn(grant,|cx|cx.admit_original_native(&mut input,&mut native)).unwrap();if native.is_some(){break}}assert!(input.is_none());
  let ready=caller.turn(RetainedCloneGrant{maximum_capacity_bytes:0,..grant},|cx|{let mut ready=false;native.as_mut().unwrap().with_native(cx,|home,cx|{ready=home.prepare(bytes.len(),cx)?;Ok(())})?;Ok(ready)}).unwrap();assert!(!ready);assert_eq!(caller.receipt().retained_capacity_bytes,0);
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|native.as_mut().unwrap().with_native(cx,|home,cx|{assert!(home.prepare(bytes.len(),cx)?);Ok(())})).unwrap());assert_eq!(heap.requested_bytes,bytes.len());assert_eq!(caller.receipt().retained_capacity_bytes,heap.requested_bytes);
  for row in law["pages"].as_array().unwrap(){let page:Vec<u8>=serde_json::from_value(row["bytes"].clone()).unwrap();let offset=row["offset"].as_u64().unwrap()as usize;let mut cursor=0;let copied=caller.turn(RetainedCloneGrant{maximum_copy_bytes:0,..grant},|cx|{let mut copied=false;native.as_mut().unwrap().with_native(cx,|home,cx|{copied=home.append_page(offset,&page,&mut cursor,cx)?;Ok(())})?;Ok(copied)}).unwrap();assert!(!copied);assert_eq!(cursor,0);for _ in 0..page.len()+1{let copied=caller.turn(RetainedCloneGrant{maximum_copy_bytes:2*size_of::<usize>()+1,..grant},|cx|{let mut copied=false;native.as_mut().unwrap().with_native(cx,|home,cx|{copied=home.append_page(offset,&page,&mut cursor,cx)?;Ok(())})?;Ok(copied)}).unwrap();if copied{break}}assert_eq!(cursor,page.len());}
  caller.turn(grant,|cx|native.as_mut().unwrap().with_native(cx,|home,cx|{assert!(home.seal(cx)?);Ok(())})).unwrap();let mut output=caller.turn(grant,|cx|GuestRelayOutput::take_sealed_step(&mut native,cx)).unwrap().unwrap();assert!(native.is_none());caller.turn(grant,|cx|output.with_bytes(cx,|received,_|{assert_eq!(received,bytes);Ok(())})).unwrap();assert!(!output.terminal_is_empty());caller.cancel().cancel_now();
  for _ in 0..64{if caller.turn(grant,|cx|cx.with_original_retirement(|cx|output.return_step(cx))).unwrap(){break}}assert!(output.terminal_is_empty());let mut released=0;
  for _ in 0..128{let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|cx.close_original_native_step()).unwrap());assert_eq!(caller.receipt().released_bytes,heap.released_bytes);released+=heap.released_bytes;if done{break}}assert!(released>=bytes.len());caller.retire(grant);
 }
 #[test]
 fn fault_capsule_preserves_all_original_causes_until_paid_return(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture["fault"]["expectedCauses"],3);
  use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};use semio_framework_value::RetainedCloneGrant;fn clock()->Option<u64>{Some(1000)}let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1048576,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),grant,clock,1000000,1000000);let mut input=Some(GuestRelayFaultHome::empty());let mut native=None;for _ in 0..64{caller.turn(grant,|cx|cx.admit_original_native(&mut input,&mut native)).unwrap();if native.is_some(){break}}
  let mut primary=OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::InvalidValue,"body"));primary.receipt_refusal=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"receipt"));let mut original=Some(GuestRelayAdmissionFailure{closure:None,primary:Some(primary),settlement:Some(ValueError::literal(ValueRefusalKind::WorkLimit,"settlement").into())});caller.turn(grant,|cx|native.as_mut().unwrap().with_native(cx,|home,cx|{assert!(home.capture_step(&mut original,cx)?);Ok(())})).unwrap();assert!(original.is_none());let mut fault=caller.turn(grant,|cx|GuestRelayFault::take_captured_step(&mut native,cx)).unwrap().unwrap();assert!(native.is_none());caller.turn(grant,|cx|fault.with_fault(cx,|failure,_|{assert_eq!(failure.primary.as_ref().unwrap().primary_value().unwrap().kind,ValueRefusalKind::InvalidValue);assert_eq!(failure.primary.as_ref().unwrap().receipt_refusal.as_ref().unwrap().kind,ValueRefusalKind::InvariantViolated);assert_eq!(failure.settlement.as_ref().unwrap().primary_value().unwrap().kind,ValueRefusalKind::WorkLimit);Ok(())})).unwrap();
  caller.cancel().cancel_now();for _ in 0..64{if caller.turn(grant,|cx|cx.with_original_retirement(|cx|fault.return_step(cx))).unwrap(){break}}assert!(fault.terminal_is_empty());let mut closed=false;for _ in 0..256{let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|cx.close_original_native_step()).unwrap());assert_eq!(caller.receipt().released_bytes,heap.released_bytes);if done{closed=true;break}}assert!(closed);caller.retire(grant);
 }

}
