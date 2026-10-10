//! 🧪️ The real registered invocation lends the original context and keeps pending caller bytes intact.
use super::*;
use std::sync::atomic::{AtomicUsize,Ordering};

struct OriginalInvocationProbe{pointer:usize,operation:u64,generation:u64,calls:AtomicUsize}
impl ExtensionResourceOwner for OriginalInvocationProbe{
 fn invoke(&self,_:&str,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<ExtensionInvokeStep,Fault>{
  assert_eq!(request.as_ptr()as usize,self.pointer);assert_eq!(cx.operation().0,self.operation);assert_eq!(cx.generation().0,self.generation);
  let grant=cx.retained_grant();if cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:None})}
  let retained_progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(retained_progress).map_err(|error|semio_framework_diagnostic::FaultFrom::to_fault(&error))?;
  let complete=self.calls.fetch_add(1,Ordering::SeqCst)!=0;Ok(ExtensionInvokeStep{payload:complete.then(Vec::new),retained_progress,refusal:None})
 }
 fn begin_close(&mut self){}
 fn close_step(&mut self,_:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>{Ok(PluginLifecycleStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{true}
 fn retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
}

/// 🤝️ Normal registered handlers receive the same caller identity, source and independently debited receipt.
#[test]
fn original_extension_invoke_lends_caller_context_and_retains_pending_source(){
 use semio_framework_job::{OperationId,Generation,StepBudget,StepContext,root_cancel_token};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📨️invoke/🔣️.json")).unwrap();
 let request=fixture["text"].as_str().unwrap().repeat(fixture["repeat"].as_u64().unwrap()as usize).into_bytes();let pointer=request.as_ptr();assert_eq!(request.len(),fixture["utf8Bytes"].as_u64().unwrap()as usize);
 let operation=fixture["caller"]["operation"].as_u64().unwrap();let generation=fixture["caller"]["generation"].as_u64().unwrap();
 let mut bundle=ExtensionBundle::new("original-invoke","Original Invoke","1").resource_owner(OriginalInvocationProbe{pointer:pointer as usize,operation,generation,calls:AtomicUsize::new(0)}).owned_handler("probe");
 let policy=&fixture["caller"]["grant"];let grant=RetainedCloneGrant{maximum_items:policy["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:policy["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:policy["maximumDepth"].as_u64().unwrap()as usize};
 let cancel=root_cancel_token();let mut sequence=0;
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(operation),Generation(generation),StepBudget::new(1,u64::MAX,denied),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bundle.invoke("probe",&request,&mut cx));let reply=result.unwrap();assert!(reply.payload.is_none());assert_eq!(reply.retained_progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(request.as_ptr(),pointer);}
 for complete in [false,true]{let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(operation),Generation(generation),StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bundle.invoke("probe",&request,&mut cx));let reply=result.unwrap();assert_eq!(reply.payload.is_some(),complete);assert_eq!(reply.retained_progress,cx.retained_progress());assert_eq!(reply.retained_progress.copied_items,1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(request.as_ptr(),pointer);}
 bundle.begin_close();while !bundle.terminal_is_empty(){let demand=bundle.retirement_demands(64).unwrap();bundle.close_step(cold_grant(demand)).unwrap();}
 eprintln!("[DEBUG] original registered invocation sameCaller=true sameRequest=true pendingNone=true receiptOnce=true actualSystem0; cold preparation/disposal outside invocation scope");
}

/// 🛡️ Admission preserves the exact refused payload and typed cause without spending another wallet.
#[test]
fn original_extension_invoke_refusal_preserves_owned_payload_and_actual_receipt(){
 let payload=b"original refused output".to_vec();let pointer=payload.as_ptr();
 let answer=ExtensionInvokeStep{payload:Some(payload),retained_progress:RetainedCloneProgress{copied_items:1,..Default::default()},refusal:None};
 let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||answer.admit(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},Default::default(),Default::default()));
 assert_eq!(answer.payload.as_ref().unwrap().as_ptr(),pointer);let cause=answer.refusal.as_ref().unwrap();assert_eq!(cause.kind,ValueRefusalKind::InvariantViolated);assert_eq!(cause.retained_progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 eprintln!("[DEBUG] original refused reply samePayload=true sameTypedCause=true actualSystem0; original payload cold disposal outside admission scope");
}
