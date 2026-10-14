//! 🧪️ A normal worker returns its original receipt before lending both complete refusal owners.
use super::*;

struct OriginalPairedFailureJob{failure:ManuallyDrop<Option<OriginalTurnFailure>>,closing:bool}
impl InteractiveJob for OriginalPairedFailureJob{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{let failure=self.failure.take().expect("one original normal failure");cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Err(failure)}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{descriptor.complete(None,None)}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{assert!(self.failure.is_none());Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
 fn terminal_is_empty(&self)->bool{self.closing&&self.failure.is_none()}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
}
impl Drop for OriginalPairedFailureJob{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original failed job requires paid source return");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.failure)}}}}
fn observed<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
fn clock()->Option<u64>{Some(1000)}

#[test]
fn original_normal_worker_failure_loans_both_causes_after_receipt_and_paid_close(){
 let _slots=worker_session_slots_shared();let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let source:RetainedCloneGrant=serde_json::from_value(law["source"].clone()).unwrap();let work:RetainedCloneGrant=serde_json::from_value(law["work"].clone()).unwrap();let close:RetainedCloneGrant=serde_json::from_value(law["close"].clone()).unwrap();
 let(operation,generation)=(OperationId(law["operation"].as_u64().unwrap()),Generation(law["generation"].as_u64().unwrap()));let mut caller=OriginalJobFixtureCaller::admit(operation,generation,source,clock,law["deadline"].as_u64().unwrap(),law["fuel"].as_u64().unwrap());assert_eq!(law["clock"],1000);
 let primary=law["primary"].as_str().unwrap();let secondary=law["receiptRefusal"].as_str().unwrap();let bytes=primary.len()+secondary.len();
 let((failure,pointers),heap)=observed(||caller.turn(source,|cx|{assert!(cx.retained_grant().maximum_copy_bytes>=bytes&&cx.retained_grant().maximum_capacity_bytes>=bytes);let primary=ValueError::new(ValueRefusalKind::UnsupportedOwner,String::from(primary));let secondary=ValueError::new(ValueRefusalKind::InvariantViolated,String::from(secondary));let pointers=(primary.message.as_ptr(),secondary.message.as_ptr());cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,released_bytes:0})?;cx.consume_fuel(1);Ok((OriginalTurnFailure{primary:primary.into(),receipt_refusal:Some(secondary),context_refusal:None},pointers))}).unwrap());assert_eq!(heap,(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));let mut born=heap.0;
 let mut job=Some(OriginalPairedFailureJob{failure:ManuallyDrop::new(Some(failure)),closing:false});let mut params=Some(BatchJobParams{operation,generation,cancel:caller.admit_alias(source),config:BatchDriveConfig{retained:work,site:"original-normal-worker-full-failure",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:500},now_us:clock});
 let((mut session,receipt),heap)=observed(||caller.turn(source,|cx|MountedWorkerJobSession::try_admit_owned(&mut job,&mut params,cx).map_err(OriginalTurnFailure::from)).unwrap().expect("fixed original admission policy"));assert_eq!(receipt,caller.receipt());assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(job.is_none()&&params.is_none());born+=heap.0;
 let(poll,heap)=observed(||session.pump_one_for_interactive_turn(work).unwrap());assert_eq!(poll,WorkerJobPoll::Outcome);assert_eq!(heap,(0,0));assert!(session.checked_out_failure().is_none());assert_eq!(session.acknowledge_checked_out_outcome(work).progress(),Default::default());
 let((issued,receipt),heap)=observed(||session.take_checked_out_retained_step_receipt().unwrap());assert_eq!(issued,work);assert_eq!(receipt,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(session.take_checked_out_retained_step_receipt().is_none());
 let failure=session.checked_out_failure().expect("actual original failure after raw receipt");assert_eq!((failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr()),pointers);assert_eq!(failure.primary.message(),primary);assert_eq!(failure.receipt_refusal.as_ref().unwrap().message.as_ref(),secondary);assert!(failure.context_refusal.is_none());
 session.begin_close();let mut released=0;let mut turns=0;while !session.terminal_is_empty(){let(step,heap)=observed(||caller.close_owner(close,&mut session).unwrap());assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(close));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;turns+=1;assert!(turns<100000);}
 assert_eq!(born,released);assert_eq!(observed(||drop(session)).1,(0,0));caller.retire(source);eprintln!("[DEBUG] actual original normal Worker receipt before both native cause pointers, one paid source and alias, exactBornRelease={born} work17copy temporal500 fuel1 unchanged closeTurns={turns} terminalDrop0");
}
