//! 📬️ Real worker admission preserves original sources and exact physical receipts at every denied currency.
use semio_framework_job::*;
use semio_framework_job_worker::*;
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{ValueError,ValueRefusalKind};
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
fn now()->Option<u64>{Some(1)}
struct Job{closing:bool}
impl InteractiveJob for Job{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original admission fixture has no payload outcome"))}}
 fn begin_close(&mut self){self.closing=true;}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{Ok(if self.closing{InteractiveJobCloseStep::Complete{progress:Default::default()}}else{InteractiveJobCloseStep::Pending{progress:Default::default()}})}
 fn terminal_is_empty(&self)->bool{self.closing}
}
#[test]
fn original_worker_payload_denials_and_actual_paid_lifetime_match_portable_contract(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../📬️payload/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let operation=OperationId(law["operation"].as_u64().unwrap());let generation=Generation(law["generation"].as_u64().unwrap());let mut caller=OriginalJobFixtureCaller::admit(operation,generation,grant,now,law["deadline"].as_u64().unwrap(),law["fuel"].as_u64().unwrap());let mut job=Some(Job{closing:false});let mut params=Some(BatchJobParams{operation,generation,cancel:caller.admit_alias(grant),config:BatchDriveConfig{retained:grant,site:"test.original-worker-payload",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:1000},now_us:now});let job_identity=job.as_ref().unwrap()as*const Job;let params_identity=params.as_ref().unwrap()as*const BatchJobParams;let demand=WorkerJobSession::<Job>::owned_admission_demand().unwrap();
 for axis in law["denials"].as_array().unwrap(){let mut denied=grant;match axis.as_str().unwrap(){"maximum_items"=>denied.maximum_items=0,"maximum_copy_bytes"=>denied.maximum_copy_bytes=demand.copy_bytes.checked_sub(1).unwrap(),"maximum_capacity_bytes"=>denied.maximum_capacity_bytes=demand.capacity_bytes.checked_sub(1).unwrap(),"maximum_depth"=>denied.maximum_depth=demand.depth.checked_sub(1).unwrap(),_=>unreachable!()};let(answer,heap)=observe(||caller.turn(denied,|cx|WorkerJobSession::try_admit_owned(&mut job,&mut params,cx).map_err(OriginalTurnFailure::from)));assert!(answer.unwrap().is_none());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(job.as_ref().unwrap()as*const Job,job_identity);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_identity);}
 let(answer,heap)=observe(||caller.turn(grant,|cx|WorkerJobSession::try_admit_owned(&mut job,&mut params,cx).map_err(OriginalTurnFailure::from)));let(mut session,receipt)=answer.unwrap().unwrap();assert!(job.is_none()&&params.is_none());assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(caller.receipt(),receipt);session.begin_close();for turn in 0..256{if session.terminal_is_empty(){break}let(answer,heap)=observe(||caller.close_owner(grant,&mut session));let progress=answer.unwrap().progress();assert!(progress.fits(grant));assert_eq!(caller.receipt(),progress);assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert!(turn<255);}assert!(session.terminal_is_empty());assert_eq!(observe(||drop(session)).1,(0,0));for turn in 0..64{if caller.terminal_is_empty(){break}let(answer,heap)=observe(||caller.retire_step(grant));let progress=answer.unwrap().progress();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert!(turn<63);}assert!(caller.terminal_is_empty());let totals=caller.cumulative_receipt();assert_eq!(totals.retained_capacity_bytes,totals.released_bytes);assert_eq!(observe(||drop(caller)).1,(0,0));
}
