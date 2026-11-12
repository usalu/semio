//! 🧪️ Actual native header births, refused receipts and every paid partial return preserve one source.
use super::*;
use semio_framework_test_fixtures::fixture_text;
use std::cell::Cell;
struct OriginalJob{target:Cell<usize>,closing:bool}
impl OriginalJob{fn bound(&self){let address=self as*const Self as usize;if self.target.get()==0{self.target.set(address)}assert_eq!(address,self.target.get(),"the original job never moves after admission")}}
impl InteractiveJob for OriginalJob{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{self.bound();InteractiveJobStep::from_callback(JobOutcomeBorrow::admit_complete(cx,None,None).map_err(OriginalTurnFailure::from))}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{self.bound();descriptor.complete(None,None)}
 fn begin_close(&mut self){self.bound();self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.bound();Ok(if cx.should_yield(){InteractiveJobCloseStep::Pending{progress:Default::default()}}else{InteractiveJobCloseStep::Complete{progress:Default::default()}})}
 fn terminal_is_empty(&self)->bool{self.closing}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
}
fn clock()->Option<u64>{Some(1000)}
fn policy(values:&serde_json::Value)->RetainedCloneGrant{let values=values.as_array().unwrap();RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize}}
fn transferred()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn refusal(error:ValueError)->OriginalTurnFailure{OriginalTurnFailure::from(error)}
fn close_session<S:OriginalPreviewSession+?Sized>(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant,session:&mut S){
 assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||session.begin_close()).1.requested_bytes,0);
 for _ in 0..256{if session.terminal_is_empty(){break}
  let progress=caller.turn(grant,|cx|{
   if session.has_original_cancel_alias_witness(cx.original_cancel_token()).unwrap_or(false){
    let turn=cx.with_original_effect(cx.retained_grant(),|cx,effect|session.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn);
    let result=turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(|error|match error{WorkerJobDemandError::Refused(error)=>error,WorkerJobDemandError::Contention(_)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original erased session alias remains checked out")});
    Ok(receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:turn.work})?.progress())
   }else{Ok(session.close_step(cx)?.progress())}
  }).unwrap();
  assert!(progress.fits(grant));
 }
 assert!(session.terminal_is_empty());
}
fn fixture(callback:RetainedCloneGrant,identity:(OperationId,Generation))->(Option<BatchJobSession<OriginalJob>>,OriginalJobFixtureCaller){
 let mut caller=OriginalJobFixtureCaller::admit(identity.0,identity.1,WORKER_TEST_STOCK,clock,u64::MAX,20000);caller.audit_heap();
 let params=BatchJobParams{operation:identity.0,generation:identity.1,cancel:caller.admit_alias(callback),config:BatchDriveConfig{retained:callback,site:"original-native-session-erasure",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:100000},now_us:clock};
 let session=admit_original_fixture_owner!(caller,BatchJobSession,OriginalJob{target:Cell::new(0),closing:false},params);
 (Some(session),caller)
}
fn close_source(source:&mut Option<BatchJobSession<OriginalJob>>,caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant){
 close_session(caller,grant,source.as_mut().unwrap());assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(source.take())).1.released_bytes,0);
}
fn try_begin(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant,preparation:&mut Option<OriginalBatchSessionErasure<OriginalJob>>,source:&mut Option<BatchJobSession<OriginalJob>>)->Result<Option<RetainedCloneProgress>,OriginalTurnFailure>{
 caller.turn(grant,|cx|OriginalBatchSessionErasure::try_begin(preparation,source,cx).map_err(refusal))
}
#[test]
fn original_native_session_erasure_keeps_refused_birth_and_every_partial_return(){
 let _slots=worker_session_slots_shared();let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🔀️erasure/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let grant=policy(&law["grant"]);let closing=policy(&law["closeGrant"]);let identity=(OperationId(919),Generation(17));
 for row in law["cases"].as_array().unwrap(){
  if !row["rawReceiptAccepted"].as_bool().unwrap(){continue}
  let(mut source,mut caller)=fixture(grant,identity);let source_address=&source as*const _;let mut policy=grant;if !row["items"].as_bool().unwrap(){policy.maximum_items=0}if !row["capacity"].as_bool().unwrap(){policy.maximum_capacity_bytes=OriginalBatchSessionErasure::<OriginalJob>::admission_demand().capacity_bytes-1}if !row["depth"].as_bool().unwrap(){policy.maximum_depth=0}
  if !row["idle"].as_bool().unwrap(){let session=source.as_mut().unwrap();let poll=caller.turn(grant,|cx|session.step_on_original_caller(cx)).unwrap();assert!(matches!(poll,WorkerJobPoll::Outcome|WorkerJobPoll::Terminal));assert!(session.checkout_outcome());let(issued,progress)=session.take_checked_out_retained_step_receipt().unwrap();assert!(progress.fits(issued));}
  let mut preparation=None;let result=if row["identity"].as_bool().unwrap(){try_begin(&mut caller,policy,&mut preparation,&mut source)}else{let mut foreign=OriginalJobFixtureCaller::admit(OperationId(920),identity.1,WORKER_TEST_STOCK,clock,u64::MAX,20000);foreign.audit_heap();let result=try_begin(&mut foreign,policy,&mut preparation,&mut source);foreign.retire(grant);result};
  assert!(source.is_some());assert_eq!(&source as*const _,source_address);assert_eq!(preparation.is_some(),row["phase"].as_str().unwrap()=="storage");
  if row["identity"].as_bool().unwrap()&&row["idle"].as_bool().unwrap(){assert!(result.is_ok())}else{assert!(result.is_err()||result.is_ok_and(|progress|progress.is_none()))}
  if let Some(preparation)=preparation.as_mut(){preparation.begin_close();let step=caller.turn(grant,|cx|preparation.close_step(&mut source,cx).map_err(refusal)).unwrap();assert!(step.progress().fits(grant));assert!(preparation.terminal_is_empty());}assert_eq!(observed_release(||drop(preparation)),0);close_source(&mut source,&mut caller,closing);caller.retire(grant);
 }
 for cut in 0..=2{
  let(mut source,mut caller)=fixture(grant,identity);let mut preparation=None;let result=try_begin(&mut caller,grant,&mut preparation,&mut source).unwrap();assert!(result.is_some());let storage=preparation.as_ref().unwrap().storage.as_ref().unwrap().as_ptr()as usize;let mut erased=None;
  for _ in 0..cut{let(ready,progress)=caller.turn(grant,|cx|preparation.as_mut().unwrap().advance(&mut source,cx).map_err(refusal)).unwrap();assert_eq!(progress,transferred());erased=ready;}
  if let Some(session)=erased.as_mut(){
   assert_eq!(session.storage_identity().unwrap()as usize,storage);assert_eq!(session.original_identity().unwrap(),Some(identity));let poll=caller.turn(grant,|cx|session.step_on_original_caller(cx)).unwrap();assert!(matches!(poll,WorkerJobPoll::Outcome|WorkerJobPoll::Terminal));assert!(session.checkout_outcome());assert!(session.checked_out_outcome().is_err());let(issued,progress)=session.take_checked_out_retained_step_receipt().unwrap();assert!(progress.fits(issued));assert!(session.take_checked_out_retained_step_receipt().is_none());assert!(matches!(session.checked_out_outcome().unwrap(),Some(JobOutcomeView::Complete{..})));
   let zero_copy=RetainedCloneGrant{maximum_copy_bytes:0,..grant};let mut acknowledged=false;for _ in 0..16{if caller.turn(zero_copy,|cx|session.acknowledge_outcome(cx)).unwrap(){acknowledged=true;break}assert_eq!(caller.receipt().copied_bytes,0)}assert!(acknowledged);
   close_session(&mut caller,closing,session);assert!(session.terminal_is_empty());assert!(source.is_none());
  }else{let preparation=preparation.as_mut().unwrap();preparation.begin_close();for _ in 0..3{if preparation.terminal_is_empty(){break}let step=caller.turn(grant,|cx|preparation.close_step(&mut source,cx).map_err(refusal)).unwrap();assert!(step.progress().fits(grant));}assert!(preparation.terminal_is_empty());close_source(&mut source,&mut caller,closing);}
  assert!(preparation.as_ref().unwrap().terminal_is_empty());assert_eq!(observed_release(||{drop(preparation);drop(erased)}),0);caller.retire(grant);
 }
}
fn observed_release<T>(body:impl FnOnce()->T)->usize{let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);heap.requested_bytes+heap.released_bytes}
