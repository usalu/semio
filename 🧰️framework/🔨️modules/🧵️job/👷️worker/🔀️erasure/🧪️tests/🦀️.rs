//! 🧪️ Actual native header births, refused receipts and every paid partial return preserve one source.
use super::*;
use semio_framework_test_fixtures::fixture_text;
struct OriginalJob{target:usize,closing:bool}
struct OriginalInitializer{stage:u8,target:Option<usize>}
unsafe impl WorkerJobInitializer<OriginalJob> for OriginalInitializer{
 fn initialization_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 unsafe fn initialize_step(&mut self,target:*mut MaybeUninit<OriginalJob>,grant:RetainedCloneGrant)->Result<WorkerJobInitializationStep,ValueError>{
  if self.target.is_some_and(|original|original!=target as usize){return Err(invalid())}if !metadata(grant){return Ok(WorkerJobInitializationStep{progress:Default::default(),complete:false})}self.target=Some(target as usize);let original=target.cast::<OriginalJob>();
  unsafe{match self.stage{0=>std::ptr::addr_of_mut!((*original).target).write(target as usize),1=>std::ptr::addr_of_mut!((*original).closing).write(false),2=>{},_=>return Err(invalid())}}self.stage+=1;Ok(WorkerJobInitializationStep{progress:transferred(),complete:self.stage==3})
 }
 fn initialization_input_is_empty(&self)->bool{self.stage==3}
 fn partial_retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 unsafe fn close_partial_step(&mut self,_:*mut MaybeUninit<OriginalJob>,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}
 fn partial_terminal_is_empty(&self)->bool{true}
}
impl InteractiveJob for OriginalJob{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{assert_eq!(self as*const _ as usize,self.target);JobOutcomeBorrow::admit_complete(cx,None,None).map_err(OriginalTurnFailure::from)}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{assert_eq!(self as*const _ as usize,self.target);descriptor.complete(None,None)}
 fn begin_close(&mut self){assert_eq!(self as*const _ as usize,self.target);self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{assert_eq!(self as*const _ as usize,self.target);Ok(if cx.should_yield(){InteractiveJobCloseStep::Pending{progress:Default::default()}}else{InteractiveJobCloseStep::Complete{progress:Default::default()}})}
 fn terminal_is_empty(&self)->bool{self.closing}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
}
fn clock()->Option<u64>{Some(1000)}
fn observed<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
#[track_caller]
fn physical(progress:RetainedCloneProgress,heap:(usize,usize),grant:RetainedCloneGrant){assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap)}
struct OriginalControl{identity:(OperationId,Generation),grant:RetainedCloneGrant,received:RetainedCloneProgress,reject:bool}
impl WorkerJobAdmissionControl for OriginalControl{
 fn admission_identity(&self)->(OperationId,Generation){self.identity}
 fn admission_grant(&self)->Result<RetainedCloneGrant,ValueError>{Ok(self.grant)}
 fn admission_is_open(&self)->bool{true}
 fn retirement_is_open(&self)->bool{true}
 fn receive_admission(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{self.received=self.received.checked_add(progress)?;if self.reject{return Err(invalid().with_retained_progress(progress))}Ok(())}
}
fn control(identity:(OperationId,Generation),grant:RetainedCloneGrant)->OriginalControl{OriginalControl{identity,grant,received:Default::default(),reject:false}}
fn policy(values:&serde_json::Value)->RetainedCloneGrant{let values=values.as_array().unwrap();RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize}}
struct OriginalCaller{identity:(OperationId,Generation),cancel:CancelToken,authority:JobPayloadAuthority,sequence:u64}
fn close_original_session<S:OriginalPreviewSession>(session:&mut S,caller:&mut OriginalCaller,grant:RetainedCloneGrant)->usize{
 assert_eq!(observed(||session.begin_close()).1,(0,0));let mut released=0;
 for _ in 0..256{if session.terminal_is_empty(){break}let mut recipient=Default::default();let mut cx=StepContext::with_payload_authority(caller.identity.0,caller.identity.1,StepBudget::new(1,u64::MAX,grant),&caller.cancel,clock,&mut caller.sequence,&mut recipient,&caller.authority).unwrap();
  let(progress,heap)=observed(||{if session.has_original_cancel_alias_witness(cx.original_cancel_token()).unwrap_or(false){let step=session.return_original_cancel_alias_step(cx.original_cancel_token(),cx.retained_grant()).unwrap().unwrap();cx.consume_retained(step.progress()).unwrap();cx.consume_fuel(1);step.progress()}else{session.close_step(&mut cx).unwrap().progress()}});assert_eq!(progress,cx.retained_progress());physical(progress,heap,grant);released+=heap.1;
 }assert!(session.terminal_is_empty());released
}
fn retire_original_caller(mut caller:OriginalCaller,grant:RetainedCloneGrant)->usize{
 let mut released=0;for _ in 0..16{if caller.authority.terminal_is_empty(){break}let(step,heap)=observed(||caller.authority.close_step(grant).unwrap());physical(step.progress(),heap,grant);released+=heap.1;}assert!(caller.authority.terminal_is_empty());let mut root=semio_framework_async::CancelTokenRetirement::from_token(caller.cancel);for _ in 0..16{if root.terminal_is_empty(){break}let(step,heap)=observed(||root.close_step(grant).unwrap());physical(step.progress(),heap,grant);released+=heap.1;}assert!(root.terminal_is_empty());assert_eq!(observed(||{drop(caller.authority);drop(root)}).1,(0,0));released
}
fn fixture(grant:RetainedCloneGrant,callback:RetainedCloneGrant,identity:(OperationId,Generation))->(Option<BatchJobSession<OriginalJob>>,usize,OriginalCaller){
 let((cancel,progress),heap)=observed(||CancelToken::admit_root(grant).unwrap().unwrap());physical(progress,heap,grant);let mut born=heap.0;let((authority,progress),heap)=observed(||JobPayloadAuthority::admit(identity.0,identity.1,grant).unwrap().unwrap());physical(progress,heap,grant);born+=heap.0;let mut caller=OriginalCaller{identity,cancel,authority,sequence:0};let mut recipient=Default::default();let mut cx=StepContext::with_payload_authority(identity.0,identity.1,StepBudget::new(1,u64::MAX,grant),&caller.cancel,clock,&mut caller.sequence,&mut recipient,&caller.authority).unwrap();let(alias,heap)=observed(||cx.admit_original_cancel_alias().unwrap().unwrap());physical(cx.retained_progress(),heap,grant);born+=heap.0;drop(cx);let mut initializer=OriginalInitializer{stage:0,target:None};let mut params=Some(BatchJobParams{operation:identity.0,generation:identity.1,cancel:alias,config:BatchDriveConfig{retained:callback,site:"original-native-session-erasure",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:100000},now_us:clock});let mut cx=control(identity,grant);let((mut preparation,progress),heap)=observed(||WorkerJobSessionPreparation::try_begin(&mut initializer,&mut params,&mut cx).unwrap().unwrap());physical(progress,heap,grant);assert_eq!(cx.received,progress);born+=heap.0;let mut prepared=None;
 for _ in 0..64{if prepared.is_some(){break}let mut cx=control(identity,grant);let((ready,progress),heap)=observed(||preparation.advance(&mut initializer,&mut params,&mut cx).unwrap());physical(progress,heap,grant);assert_eq!(cx.received,progress);born+=heap.0;prepared=ready;}
 assert!(prepared.is_some()&&preparation.terminal_is_empty()&&params.is_none()&&initializer.initialization_input_is_empty());assert_eq!(observed(||drop(preparation)).1,(0,0));let mut cx=control(identity,grant);let((session,progress),heap)=observed(||BatchJobSession::admit_prepared(&mut prepared,&mut cx).unwrap().unwrap());physical(progress,heap,grant);assert_eq!(cx.received,progress);born+=heap.0;(Some(session),born,caller)
}
fn close_original(source:&mut Option<BatchJobSession<OriginalJob>>,caller:&mut OriginalCaller,grant:RetainedCloneGrant)->usize{
 let released=close_original_session(source.as_mut().unwrap(),caller,grant);assert_eq!(observed(||drop(source.take())).1,(0,0));released
}
#[test]
fn original_native_session_erasure_keeps_refused_birth_and_every_partial_return(){
 let _slots=worker_session_slots_shared();let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🔀️erasure/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let grant=policy(&law["grant"]);let admission=policy(&law["preparationGrant"]);let identity=(OperationId(919),Generation(17));
 for row in law["cases"].as_array().unwrap(){let(mut source,mut born,mut caller)=fixture(admission,grant,identity);let mut released=0;let source_address=&source as*const _;let mut policy=grant;if !row["items"].as_bool().unwrap(){policy.maximum_items=0}if !row["capacity"].as_bool().unwrap(){policy.maximum_capacity_bytes=OriginalBatchSessionErasure::<OriginalJob>::admission_demand().capacity_bytes-1}if !row["depth"].as_bool().unwrap(){policy.maximum_depth=0}
  if !row["idle"].as_bool().unwrap(){let session=source.as_mut().unwrap();let(poll,heap)=observed(||session.step(grant).unwrap());assert!(matches!(poll,WorkerJobPoll::Outcome|WorkerJobPoll::Terminal));assert!(session.checkout_outcome());let(issued,progress)=session.take_checked_out_retained_step_receipt().unwrap();physical(progress,heap,issued);born+=heap.0;released+=heap.1;}
  let mut cx=control(if row["identity"].as_bool().unwrap(){identity}else{(OperationId(920),identity.1)},policy);cx.reject=!row["rawReceiptAccepted"].as_bool().unwrap();let mut preparation=None;let(result,heap)=observed(||OriginalBatchSessionErasure::try_begin(&mut preparation,&mut source,&mut cx));physical(cx.received,heap,policy);born+=heap.0;released+=heap.1;assert!(source.is_some());assert_eq!(&source as*const _,source_address);assert_eq!(preparation.is_some(),row["phase"].as_str().unwrap()=="storage");if cx.reject{assert!(result.is_err());assert!(preparation.as_ref().unwrap().receipt_refused)}
  if let Some(preparation)=preparation.as_mut(){preparation.begin_close();let mut cx=control(identity,grant);let(step,heap)=observed(||preparation.close_step(&mut source,&mut cx).unwrap());physical(step.progress(),heap,grant);assert_eq!(step.progress(),cx.received);released+=heap.1;assert!(preparation.terminal_is_empty());}assert_eq!(observed(||drop(preparation)).1,(0,0));released+=close_original(&mut source,&mut caller,grant);released+=retire_original_caller(caller,admission);assert_eq!(born,released);
 }
 for cut in 0..=2{let(mut source,mut born,mut caller)=fixture(admission,grant,identity);let mut preparation=None;let mut cx=control(identity,grant);let(result,heap)=observed(||OriginalBatchSessionErasure::try_begin(&mut preparation,&mut source,&mut cx).unwrap());assert!(result.is_some());physical(cx.received,heap,grant);born+=heap.0;let storage=preparation.as_ref().unwrap().storage.as_ref().unwrap().as_ptr()as usize;let mut erased=None;
  for _ in 0..cut{let mut cx=control(identity,grant);let((ready,progress),heap)=observed(||preparation.as_mut().unwrap().advance(&mut source,&mut cx).unwrap());physical(progress,heap,grant);assert_eq!(progress,transferred());assert_eq!(progress,cx.received);erased=ready;}
  let mut released=0;if let Some(session)=erased.as_mut(){assert_eq!(session.storage_identity().unwrap()as usize,storage);assert_eq!(session.original_identity().unwrap(),Some(identity));let(poll,heap)=observed(||session.step(grant).unwrap());assert!(matches!(poll,WorkerJobPoll::Outcome|WorkerJobPoll::Terminal));assert!(session.checkout_outcome());assert!(session.checked_out_outcome().is_err());let(issued,progress)=session.take_checked_out_retained_step_receipt().unwrap();physical(progress,heap,issued);born+=heap.0;released+=heap.1;assert!(session.take_checked_out_retained_step_receipt().is_none());assert!(matches!(session.checked_out_outcome().unwrap(),Some(JobOutcomeView::Complete{..})));let mut acknowledged=false;for _ in 0..16{let(step,heap)=observed(||session.acknowledge_outcome(RetainedCloneGrant{maximum_copy_bytes:0,..grant}));physical(step.progress(),heap,grant);assert_eq!(step.progress().copied_bytes,0);if matches!(step,RetainedCloneStep::Complete(_)){acknowledged=true;break}}assert!(acknowledged);released+=close_original_session(session,&mut caller,grant);assert!(session.terminal_is_empty());assert!(source.is_none());}else{let preparation=preparation.as_mut().unwrap();preparation.begin_close();for _ in 0..3{if preparation.terminal_is_empty(){break}let mut cx=control(identity,grant);let(step,heap)=observed(||preparation.close_step(&mut source,&mut cx).unwrap());physical(step.progress(),heap,grant);assert_eq!(step.progress(),cx.received);released+=heap.1;}assert!(preparation.terminal_is_empty());released+=close_original(&mut source,&mut caller,grant);}
  assert!(preparation.as_ref().unwrap().terminal_is_empty());assert_eq!(observed(||{drop(preparation);drop(erased)}).1,(0,0));released+=retire_original_caller(caller,admission);assert_eq!(born,released);
 }
}
