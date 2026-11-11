//! 🔤️ A resumable JSON frontier retains its actual storage and replaces only paid Source work ports.
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_pack_json::{JsonBorrowedWriteCursor,JsonWriteSource};
use semio_framework_value::{NativeEncodeControl,RetainedCloneProgress,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::size_of;
pub struct SourceJsonEncoder{cursor:Option<ControlledRetirement<JsonBorrowedWriteCursor>>,binding:Option<OriginalSourceBinding>,source:Option<usize>,epoch:u64,closing:bool}
impl SourceJsonEncoder{
 pub const fn new()->Self{Self{cursor:None,binding:None,source:None,epoch:0,closing:false}}
 pub const fn minimum_birth_metadata_copy_bytes()->usize{size_of::<Option<OriginalSourceBinding>>()+size_of::<Option<usize>>()+size_of::<u64>()+size_of::<Option<ControlledRetirement<JsonBorrowedWriteCursor>>>()}
 pub const fn minimum_cursor_metadata_copy_bytes()->usize{size_of::<u64>()+size_of::<bool>()}
 pub const fn minimum_output_metadata_copy_bytes()->usize{size_of::<u64>()+size_of::<Option<String>>()+size_of::<Option<Vec<u8>>>()+size_of::<u8>()}
 /// ▶️ Advances one real structural phase and retains the full output before receiving any failed effect receipt.
 pub fn advance<S:JsonWriteSource+?Sized>(&mut self,view:&S,output:&mut Option<Vec<u8>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;let source=parent.source_identity()?;if self.binding.is_some()!=self.source.is_some(){return Err(refused("Source JSON parent identity is incomplete").into())}if self.source.is_some_and(|original|original!=source){return Err(refused("Source JSON changed its native parent Source").into())}if self.closing{return Err(refused("closing JSON Source cannot restart semantic encoding").into())}
  let epoch=cx.original_source_epoch().ok_or_else(||refused("Source JSON requires its actual issued epoch"))?;if self.binding.is_some_and(|binding|!binding.matches(cx))||epoch<self.epoch{return Err(refused("Source JSON changed its actual parent or renewed an earlier epoch").into())}if output.is_some(){if !self.output_is_ready(output){return Err(refused("Source JSON output is not a completed paid frontier").into())}return Ok(true)}
  if self.binding.is_none(){let bytes=Self::minimum_birth_metadata_copy_bytes();if !admit(cx,bytes)?{require_remaining(cx,bytes,1)?;return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);let mut cursor=JsonBorrowedWriteCursor::new();cursor.set_output_ceiling(cx.original_total_retained_grant().maximum_capacity_bytes);self.cursor=Some(ControlledRetirement::retain(cursor));self.binding=Some(OriginalSourceBinding::borrow(cx));self.source=Some(source);self.epoch=epoch;return Ok(false)}
  let epoch_bytes=if epoch!=self.epoch{size_of::<u64>()}else{0};
  if self.cursor.as_ref().and_then(ControlledRetirement::original).is_some_and(JsonBorrowedWriteCursor::completed_output_ready){
   let bytes=epoch_bytes+size_of::<Option<String>>()+size_of::<Option<Vec<u8>>>()+size_of::<u8>();if !admit(cx,bytes)?{require_remaining(cx,bytes,1)?;return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
   let text=self.cursor.as_mut().and_then(ControlledRetirement::original_mut).and_then(JsonBorrowedWriteCursor::take_completed_output).ok_or_else(||refused("Source JSON lost its validated completed output"))?;self.epoch=epoch;*output=Some(text.into_bytes());return Ok(true)
  }
  let binding=self.binding.ok_or_else(||refused("Source JSON lost its parent binding"))?;let mut grant=cx.retained_grant();let reserved_bytes=epoch_bytes+size_of::<bool>();if cx.should_yield()||cx.fuel_remaining()<2||grant.maximum_items==0||grant.maximum_copy_bytes<reserved_bytes||grant.maximum_depth==0{require_remaining(cx,reserved_bytes,2)?;return Ok(false)}grant.maximum_copy_bytes-=reserved_bytes;let total=cx.original_total_retained_grant();
  let Some((outcome,_))=cx.with_original_effect(grant,|cx,effect|->Result<_,ValueError>{
   let limits=effect.grant();let live:&StepContext<'_>=&*cx;let mut observe=|_|binding.matches(live)&&!live.should_yield();
   let mut control=NativeEncodeControl::new_retained(&mut observe);
   let cursor=self.cursor.as_mut().and_then(ControlledRetirement::original_mut).ok_or_else(||refused("Source JSON lost its original frontier"))?;
   let stepped=cursor.step_with_demand(view,1,&mut control,limits);let progress=cursor.normal_step_progress();drop(control);
   let work=cursor.last_turn_work().max(1);let stepped=if effect.performed(work)?{stepped}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Source JSON work reached its effect ceiling"))};
   Ok((stepped,progress))
  })?else{return Ok(false)};
  let (stepped,mut progress)=outcome?;
  let result=match stepped{Ok((None,Some(demand)))if total.maximum_items==0||total.maximum_copy_bytes.saturating_sub(reserved_bytes)<demand.copy_bytes||total.maximum_capacity_bytes<demand.capacity_bytes||total.maximum_release_bytes<demand.release_bytes||total.maximum_depth<demand.depth=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Source JSON remaining budget cannot fund its next cursor phase")),Ok((output,_))=>Ok(output),Err(error)=>Err(error)};
  let metadata_bytes=epoch_bytes;let metadata_overflow=progress.copied_bytes.checked_add(metadata_bytes).is_none();if metadata_bytes!=0{progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.saturating_add(metadata_bytes);self.epoch=epoch;}
  let result=match result{Ok(Some(text))=>{*output=Some(text.into_bytes());Ok(true)},Ok(None)=>Ok(false),Err(error)=>Err(error)};
  let result=if metadata_overflow{result.and_then(|_|Err(refused("Source JSON metadata receipt overflow")))}else{result};
  if result.is_err(){self.closing=true;progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.saturating_add(size_of::<bool>());}
  let receipt=cx.consume_retained(progress);if progress.copied_items!=0{cx.consume_fuel(1)}
  let rejected_close=receipt.is_err()&&!self.closing;if rejected_close{self.closing=true;if progress.copied_items==0{cx.consume_fuel(1);}}
  match(result,receipt){(Ok(ready),Ok(()))=>Ok(ready),(result,receipt)=>{let primary=result.err().unwrap_or_else(||refused("Source JSON actual receiving receipt was rejected"));let mut failure=OriginalTurnFailure::from(primary);failure.effects=progress;if rejected_close{failure.effects.copied_items=failure.effects.copied_items.max(1);failure.effects.copied_bytes=failure.effects.copied_bytes.saturating_add(size_of::<bool>());}failure.receipt_refusal=receipt.err();Err(failure)}}
 }
 /// ♻️ Returns the real frames, partial string and opaque admission metadata on the same original Source.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let parent=cx.original_source_parent()?;parent.validate_borrowed_context(cx)?;if self.binding.is_some()!=self.source.is_some(){return Err(refused("Source JSON parent identity is incomplete").into())}if self.source.is_some_and(|source|!parent.source_identity().is_ok_and(|actual|actual==source))||self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(refused("Source JSON retirement requires its exact original parent").into())}if self.terminal_is_empty(){return Ok(true)}
  if !self.closing{let bytes=size_of::<bool>();if cx.retirement_should_yield()||!admit(cx,bytes)?{require_remaining(cx,bytes,1)?;return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.closing=true;return Ok(false)}
  if let Some(cursor)=self.cursor.as_mut(){
   if !cursor.terminal_is_empty(){
    if cursor.original().is_some_and(|cursor|cursor.retained_capacity_bytes().is_ok_and(|bytes|bytes==0)){let bytes=size_of::<Option<JsonBorrowedWriteCursor>>();if cx.retirement_should_yield()||!admit(cx,bytes)?{require_remaining(cx,bytes,1)?;return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);drop(cursor.take_original());return Ok(false)}
    let grant=cx.retained_grant();let Some((poll,performed))=cx.with_original_effect(grant,|_,effect|cursor.step(grant.maximum_copy_bytes,effect))?else{return Ok(false)};
    let Ok(turn)=RetirementPollExit::settled(poll)else{return Err(refused("Source JSON cleanup left its producer reply pending").into())};
    let overrun=performed>turn.work;let settled=semio_framework_job::receive_original_retained_work(cx,turn).map(|_|());
    if overrun{let refusal=refused("Source JSON cleanup performed more work than it settled");return Err(match settled{Ok(())=>refusal.into(),Err(mut failure)=>{failure.work_refusal=Some(refusal);failure}})}
    settled?;return Ok(false)
   }
   let bytes=size_of::<Option<ControlledRetirement<JsonBorrowedWriteCursor>>>();if cx.retirement_should_yield()||!admit(cx,bytes)?{require_remaining(cx,bytes,1)?;return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.cursor.take();return Ok(false)
  }
  let bytes=size_of::<Option<OriginalSourceBinding>>()+size_of::<Option<usize>>()+size_of::<u64>()+size_of::<bool>();if cx.retirement_should_yield()||!admit(cx,bytes)?{require_remaining(cx,bytes,1)?;return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.binding=None;self.source=None;self.epoch=0;self.closing=false;Ok(true)
 }
 pub fn output_is_ready(&self,output:&Option<Vec<u8>>)->bool{!self.closing&&self.binding.is_some()&&self.source.is_some()&&output.is_some()&&self.cursor.as_ref().and_then(ControlledRetirement::original).is_some_and(|cursor|cursor.phase==3)}
 pub fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.binding.is_none()&&self.source.is_none()&&self.epoch==0&&!self.closing}
}
impl Drop for SourceJsonEncoder{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Source JSON frontier requires paid original retirement")}}
fn admit(cx:&StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();Ok(!(cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0))}
fn require_remaining(cx:&StepContext<'_>,bytes:usize,fuel:u64)->Result<(),OriginalTurnFailure>{let total=cx.original_total_retained_grant();if total.maximum_items==0||total.maximum_copy_bytes<bytes||total.maximum_depth==0||cx.original_total_fuel_remaining()<fuel{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Source JSON remaining budget cannot cover its minimum metadata phase").into())}Ok(())}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_framework_value::{DslValue,Number,RetainedCloneGrant};
 fn clock()->Option<u64>{Some(1000)}
 #[test]
 fn source_json_progresses_after_birth_with_one_item_per_epoch(){
  let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1048576,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:64};let one=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:128,..grant};let mut caller=OriginalJobFixtureCaller::admit(OperationId(72),Generation(3),grant,clock,1000000,1000000);let view=DslValue::String("one".into());let mut encoder=SourceJsonEncoder::new();let mut output=None;let mut epoch=0;for _ in 0..10000{epoch+=1;caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}).unwrap();if encoder.binding.is_some(){break}}assert!(encoder.binding.is_some());if epoch>1{let old_epoch=epoch-1;assert!(caller.turn(grant,|cx|{cx.bind_original_source_epoch(old_epoch)?;encoder.advance(&view,&mut output,cx)}).is_err())}
  for _ in 0..10000{epoch+=1;let done=caller.turn(one,|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}).unwrap();if done{break}}
  assert_eq!(output.as_deref(),Some(b"\"one\"".as_slice()));assert!(encoder.output_is_ready(&output));caller.cancel().cancel_now();for _ in 0..10000{epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|encoder.close_step(cx))}).unwrap(){break}}assert!(encoder.terminal_is_empty());epoch+=1;caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;let capacity=output.as_ref().unwrap().capacity();cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);drop(output.take());Ok(())}).unwrap();caller.retire(grant);
 }
 #[test]
 fn actual_source_json_resumes_paid_storage_without_capacity_renewal_and_rejects_foreign_binding(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let values=law["value"]["bytes"].as_array().unwrap().iter().map(|value|DslValue::Number(Number::UInt(value.as_u64().unwrap()))).collect();let view=DslValue::Object(vec![("bytes".into(),DslValue::Array(values)),("text".into(),DslValue::String(law["value"]["text"].as_str().unwrap().into()))]);let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1048576,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),grant,clock,1000000,1000000);let mut encoder=SourceJsonEncoder::new();let mut output=None;let mut epoch=0;
  for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}).unwrap());assert_eq!(caller.receipt().retained_capacity_bytes,heap.requested_bytes);if encoder.cursor.as_ref().and_then(ControlledRetirement::original).is_some_and(|cursor|cursor.phase==1){break}}
  let mut foreign=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),grant,clock,1000000,1000000);assert!(foreign.turn(grant,|cx|{cx.bind_original_source_epoch(epoch+1)?;encoder.advance(&view,&mut output,cx)}).is_err());foreign.retire(grant);
  for _ in 0..law["maximumSlices"].as_u64().unwrap(){if encoder.cursor.as_ref().and_then(ControlledRetirement::original).and_then(JsonBorrowedWriteCursor::measured_bytes).is_some(){break}epoch+=1;let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(RetainedCloneGrant{maximum_capacity_bytes:0,..grant},|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}).unwrap());assert_eq!((caller.receipt().retained_capacity_bytes,heap.requested_bytes),(0,0));}
  assert!(encoder.cursor.as_ref().unwrap().original().unwrap().measured_bytes().is_some());epoch+=1;let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}).unwrap());assert_eq!(caller.receipt().retained_capacity_bytes,heap.requested_bytes);assert!(heap.requested_bytes>0);
  for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(RetainedCloneGrant{maximum_capacity_bytes:0,..grant},|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}).unwrap());assert_eq!((caller.receipt().retained_capacity_bytes,heap.requested_bytes),(0,0));if done{break}}
  assert_eq!(output.as_deref().unwrap(),serde_json::to_vec(&law["value"]).unwrap());caller.cancel().cancel_now();
  for _ in 0..law["maximumSlices"].as_u64().unwrap(){epoch+=1;let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|encoder.close_step(cx))}).unwrap());assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),(heap.requested_bytes,heap.released_bytes));if done{break}}assert!(encoder.terminal_is_empty());epoch+=1;let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;let bytes=output.as_ref().unwrap().capacity();cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);drop(output.take());Ok(())}).unwrap());assert_eq!(caller.receipt().released_bytes,heap.released_bytes);caller.retire(grant);
 }

 #[test]
 fn a_grant_smaller_than_the_next_allocation_is_refused_without_a_heap_birth(){
  let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1048576,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:64};let tiny=RetainedCloneGrant{maximum_capacity_bytes:1024,..grant};let view=DslValue::String("one".into());
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(74),Generation(3),grant,clock,1000000,1000000);let mut encoder=SourceJsonEncoder::new();let mut output=None;let mut epoch=1;
  caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}).unwrap();assert!(encoder.binding.is_some());
  for _ in 0..8{epoch+=1;let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(tiny,|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}).unwrap());assert!(!done);assert_eq!(heap.requested_bytes,0);assert_eq!(caller.receipt().retained_capacity_bytes,0);}
  assert!(output.is_none());caller.cancel().cancel_now();for _ in 0..10000{epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|encoder.close_step(cx))}).unwrap(){break}}assert!(encoder.terminal_is_empty());caller.retire(grant);
  let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1024,&mut accept);let mut cursor=JsonBorrowedWriteCursor::new();
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step_with_demand(&view,1,&mut control,grant));
  assert!(result.is_err());assert_eq!(heap.requested_bytes,0);drop(control);
 }
 #[test]
 fn the_caller_ceiling_bounds_the_whole_output_and_exceeding_it_is_a_typed_refusal(){
  let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1048576,maximum_capacity_bytes:16384,maximum_release_bytes:4194304,maximum_depth:64};let view=DslValue::String("a".repeat(40000));
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(75),Generation(3),grant,clock,1000000,1000000);let mut encoder=SourceJsonEncoder::new();let mut output=None;let mut epoch=0;let mut refusal=None;
  for _ in 0..10000{epoch+=1;match caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&view,&mut output,cx)}){Ok(done)=>assert!(!done),Err(_)=>{refusal=Some(epoch);break}}}
  assert!(refusal.is_some());assert!(output.is_none());caller.cancel().cancel_now();for _ in 0..10000{epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|encoder.close_step(cx))}).unwrap(){break}}assert!(encoder.terminal_is_empty());caller.retire(grant);
 }
}

