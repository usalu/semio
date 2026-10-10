//! ⚖️ Exact original diagnostics match the canonical independent JSON view and actual caller receipts.
use super::*;
use semio_framework_value::ToValue;
use semio_framework_job::{OperationId,Generation,StepBudget,root_cancel_token};
fn fault_bytes(fault:&Fault)->usize{
 let scope=&fault.scope;let mut bytes=std::mem::size_of::<FaultScope>()+fault.code.0.capacity()+fault.message.capacity()+fault.causes.capacity()*std::mem::size_of::<FaultCause>();
 for text in [&scope.plugin_id,&scope.app_id,&scope.instance_id,&scope.module,&scope.body_key]{bytes+=text.as_ref().map_or(0,String::capacity)}
 for cause in &fault.causes{bytes+=cause.message.capacity()+cause.code.as_ref().map_or(0,|code|code.0.capacity())}
 if let Some(params)=&fault.params{bytes+=std::mem::size_of::<FaultParams>()+params.0.capacity()*std::mem::size_of::<(String,String)>();for(key,value)in &params.0{bytes+=key.capacity()+value.capacity()}}bytes
}
/// 🧾️ Moving the original inline header preserves the same physical Fault scope identity throughout output.
#[test]
fn original_extension_invoke_fault_reply_preserves_source_system_receipts_and_canonical_wire(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/📨️invoke/🔣️.json")).unwrap();
 for copy in corpus["failureCopyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
  let mut fault=Fault::new(FaultOrigin::Extension,"extension.original","源🌊 diagnostic");fault.scope.plugin_id=Some("original-plugin".into());fault.scope.body_key=Some("body-71".into());fault.span=Some(TextSpan{line:3,column:7,length:11});fault.causes=vec![FaultCause{message:"original nested cause".into(),code:Some("extension.child".into())}];fault.params=Some(Box::new(FaultParams(vec![("count".into(),"7".into())])));
  let expected:serde_json::Value=fault.to_value().into();let original=fault_bytes(&fault);let address=fault.scope.as_ref()as *const FaultScope as usize;let cause=ExtensionInvocationCause::Fault(fault);
  let(mut owner,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||RetainedExtensionFaultReply::new(cause));assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));assert_eq!(owner.cause.as_ref().unwrap().projection_identity(),address);
  let cancel=root_cancel_token();let mut sequence=0;let mut born=0;let mut freed=0;let mut output=None;
  for turn in 0..100000{
   let demand=owner.demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};
   let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(71),Generation(3),StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.advance(&mut cx));let result=result.unwrap();let progress=cx.retained_progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;
   if let Some(source)=owner.cause.as_ref(){assert_eq!(source.projection_identity(),address)}
   if result.is_some(){output=result;break}assert!(turn<99999,"original fault output stalled copy={copy} phase={} demand={demand:?}",owner.phase);
  }
  let output=output.unwrap();assert!(owner.terminal_is_empty());assert_eq!(original+born,freed+output.capacity());let decoded=semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(&output).unwrap();let produced:serde_json::Value=decoded.into();assert_eq!(produced,expected);
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  eprintln!("[DEBUG] original fault output copy={copy} sameSource=true canonicalWire=true born={born} freed={freed} outputCapacity={} Drop0=true",output.capacity());
 }
}

/// 🍂️ Every original failure prefix survives cancellation until its whole actual backing closes.
#[test]
fn original_extension_invoke_fault_reply_cancel_closes_original_prefix_and_output(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/📨️invoke/🔣️.json")).unwrap();
 for copy in corpus["failureCopyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){for stop in corpus["cancelCuts"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
  let fault=Fault::new(FaultOrigin::Extension,"extension.original-cancel","original cancellation 源🌊");let original=fault_bytes(&fault);let mut owner=RetainedExtensionFaultReply::new(ExtensionInvocationCause::Fault(fault));let cancel=root_cancel_token();let mut sequence=0;let mut born=0;let mut freed=0;
  for turn in 0..100000{if turn==stop{owner.begin_close();cancel.cancel_now()}let demand=owner.demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(71),Generation(3),StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.advance(&mut cx));assert!(result.unwrap().is_none());let progress=cx.retained_progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;if owner.terminal_is_empty(){break}assert!(turn<99999,"original cancellation failed copy={copy} stop={stop} phase={} demand={demand:?}",owner.phase)}
  assert!(owner.terminal_is_empty());assert_eq!(original+born,freed);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original fault cancel copy={copy} stop={stop} actualSystemConservation=true Drop0=true");
 }}
}

/// 📬️ Every inference cause retains its original owned fields through canonical fault output.
#[test]
fn original_inference_gateway_failure_all_owned_causes(){
 for copy in [1,3,64]{for case in 0..4{
  let value=match case{0=>crate::ArtifactInferenceGatewayFailure::Native(ValueError::new(ValueRefusalKind::InvalidValue,"same native 源")),1=>crate::ArtifactInferenceGatewayFailure::Json(semio_framework_pack_json::JsonError::DuplicateMember{name:"same member 源".into(),offset:11}),2=>crate::ArtifactInferenceGatewayFailure::Execution(crate::ArtifactInferenceExecutionError::new("inference.original","same execution 源")),_=>crate::ArtifactInferenceGatewayFailure::Literal("same literal source")};
  let(code,message,progress)=value.fault_wire_fields();let expected=serde_json::json!({"origin":"plugin","code":code,"severity":"error","message":message,"scope":{},"retainedProgress":{"copiedItems":progress.copied_items,"copiedBytes":progress.copied_bytes,"retainedCapacityBytes":progress.retained_capacity_bytes,"releasedBytes":progress.released_bytes},"retryable":false});let address=value.source_identity();
  let(mut owner,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||RetainedExtensionFaultReply::new(ExtensionInvocationCause::Inference(value)));assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));let cancel=root_cancel_token();let mut sequence=0;let mut output=None;
  for turn in 0..100000{let demand=owner.demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(71),Generation(3),StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.advance(&mut cx));let progress=cx.retained_progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if let Some(source)=owner.cause.as_ref(){assert_eq!(source.projection_identity(),address);if let ExtensionInvocationCause::Inference(crate::ArtifactInferenceGatewayFailure::Json(semio_framework_pack_json::JsonError::DuplicateMember{name,offset}))=source{assert_eq!((name.as_str(),*offset),("same member 源",11));}}if let Some(bytes)=result.unwrap(){output=Some(bytes);break}assert!(turn<99999,"gateway cause case={case} copy={copy} phase={} demand={demand:?}",owner.phase);}
  assert!(owner.terminal_is_empty());let decoded:serde_json::Value=semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(&output.unwrap()).unwrap().into();assert_eq!(decoded,expected);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original gateway failure case={case} copy={copy} retains source and owned JSON detail; actual System receipt and canonical Serde view; terminalDrop0");
 }}
}
