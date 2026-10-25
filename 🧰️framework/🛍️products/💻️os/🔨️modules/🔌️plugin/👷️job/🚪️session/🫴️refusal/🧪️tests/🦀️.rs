use super::*;
use semio_framework_job::{Generation,OperationId,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,StepBudget,root_cancel_token};
struct Probe{bytes:Vec<u8>,closing:bool}
impl InteractiveJob for Probe{
 fn step<'a>(&'a mut self,_:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{Ok(None)}
 fn borrow_outcome<'a>(&'a self,_:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"probe has no published outcome"))}
 fn begin_close(&mut self){self.closing=true}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.bytes.capacity())}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_release_bytes<self.bytes.capacity()||grant.maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}let progress=RetainedCloneProgress{copied_items:1,released_bytes:self.bytes.capacity(),..Default::default()};drop(std::mem::take(&mut self.bytes));cx.consume_fuel(1);cx.consume_retained(progress)?;Ok(InteractiveJobCloseStep::Complete{progress})}
 fn terminal_is_empty(&self)->bool{self.bytes.capacity()==0}
}
#[test]
fn original_lifecycle_refused_source_original_heap_and_context_are_retained_until_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let turn:RetainedCloneGrant=serde_json::from_value(law["turn"].clone()).unwrap();let whole:RetainedCloneGrant=serde_json::from_value(law["whole"].clone()).unwrap();
 for case in law["cases"].as_array().unwrap(){
  let((job,error),initial)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let primary=if case["kind"]=="fault"{semio_framework_job::OriginalTurnPrimary::Fault(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework,semio_framework::FaultCode::new("original.worker.refusal"),law["message"].as_str().unwrap()))}else{semio_framework_job::OriginalTurnPrimary::Value(ValueError::new(ValueRefusalKind::InvalidValue,law["message"].as_str().unwrap()))};
   (Probe{bytes:vec![7;law["ownedJobBytes"].as_u64().unwrap()as usize],closing:false},{let original=OriginalTurnFailure::from(primary);OriginalTurnFailure{receipt_refusal:case["receipt"].as_str().map(|text|ValueError::new(ValueRefusalKind::OwnershipLimit,text)),context_refusal:case["context"].as_str().map(|text|ValueError::new(ValueRefusalKind::DepthLimit,text)),..original}})
  });
  let source=job.bytes.as_ptr();let cause=error.primary.message().as_ptr();let receipt_pointer=error.receipt_refusal.as_ref().map(|error|error.message.as_ptr());let context_pointer=error.context_refusal.as_ref().map(|error|error.message.as_ptr());
  let(cancel,cancel_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(root_cancel_token);
  let params=BatchJobParams{operation:OperationId(17),generation:Generation(9),cancel,config:semio_framework_job_worker_authority::BatchDriveConfig{retained:turn,site:"original-refused-source",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:1000000},now_us:||Some(1)};
  let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||refused(Some(job),Some(params),Some(error)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut receipt=Default::default();let mut sequence=0;let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(law["fuel"].as_u64().unwrap(),law["deadlineMicros"].as_u64().unwrap(),whole),root_cancel_token(),||Some(1),&mut sequence,&mut receipt);let original=&mut cx as*mut StepContext<'_>;let cancel=cx.original_cancel_token()as*const _;let mut born=initial.requested_bytes+cancel_heap.requested_bytes;let mut freed=initial.released_bytes;
  for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){
   if owner.terminal_is_empty(){break}
   let before=cx.retained_progress();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_items:0,..turn},|same|owner.close_step(same)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);
   if let Some(job)=owner.job().filter(|job|job.bytes.capacity()>0){assert_eq!(job.bytes.as_ptr(),source)}
   if let Some(error)=owner.error(){assert_eq!(error.primary.message().as_ptr(),cause);assert_eq!(error.primary.message(),law["message"].as_str().unwrap());assert_eq!(error.receipt_refusal.as_ref().map(|error|error.message.as_ptr()),receipt_pointer);assert_eq!(error.context_refusal.as_ref().map(|error|error.message.as_ptr()),context_pointer);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(turn,|same|{assert_eq!(same as*mut _,original);assert_eq!(same.original_cancel_token()as*const _,cancel);owner.close_step(same)}).unwrap());let progress=step.progress();assert!(progress.fits(turn));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(cx.retained_progress().copied_items-before.copied_items,progress.copied_items);assert_eq!(cx.retained_progress().released_bytes-before.released_bytes,progress.released_bytes);born+=heap.requested_bytes;freed+=heap.released_bytes;
  }
  assert!(owner.terminal_is_empty());let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(born,freed);
 }
}

#[test]
fn original_reserved_pending_worker_same_source_denials_admission_cancel_and_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🎟️pending/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let source=RetainedCloneGrant{maximum_items:1000000,maximum_copy_bytes:4194304,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:128};
 for body in law["bodyCredits"].as_array().unwrap(){for stop in 0..4{
  let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(OperationId(17),Generation(9),source,||Some(1),600000000,1000000);
  let(job,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Probe{bytes:vec![7;128],closing:false});let pointer=job.bytes.as_ptr();let mut born=heap.requested_bytes;let mut freed=heap.released_bytes;
  let(mut pending,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||RefusedWorkerSource::pending(job,(OperationId(17),Generation(9))));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:source,site:"original-reserved-pending",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:1000000,step_budget_us:600000000};
  let denied=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:body.as_u64().unwrap()as usize,..source};
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|pending.admit_pending(config,cx)));assert!(result.unwrap().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(pending.job().unwrap().bytes.as_ptr(),pointer);
  let mut session=None;
  if stop>0{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(source,|cx|pending.admit_pending(config,cx)));assert!(result.unwrap().is_none());assert_eq!(caller.receipt(),RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(pending.params.as_ref().unwrap().cancel.is_same_node(caller.cancel()));}
  if stop>1{let short=RetainedCloneGrant{maximum_copy_bytes:body.as_u64().unwrap()as usize,..source};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(short,|cx|pending.admit_pending(config,cx)));assert!(result.unwrap().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(pending.job().unwrap().bytes.as_ptr(),pointer);}
  if stop>2{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(source,|cx|pending.admit_pending(config,cx)));session=result.unwrap();assert!(session.is_some());let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}
  caller.cancel().cancel_now();
  for _ in 0..100000{if pending.terminal_is_empty(){break}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(source,|cx|pending.close_step(cx)).unwrap());assert_eq!(caller.receipt(),step.progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}
  assert!(pending.terminal_is_empty());let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(pending));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  if let Some(mut session)=session{session.begin_close();for _ in 0..100000{if session.terminal_is_empty(){break}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(source,|cx|session.close_step(cx)).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(session.terminal_is_empty());let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(session));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  assert_eq!(born,freed);caller.retire(source);
 }}
}

#[test]
fn original_reserved_pending_worker_erased_box_is_physically_retired_on_same_funded_context(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let turn:RetainedCloneGrant=serde_json::from_value(law["turn"].clone()).unwrap();let whole:RetainedCloneGrant=serde_json::from_value(law["whole"].clone()).unwrap();
 let mut receipt=RetainedCloneProgress::default();let mut sequence=0;let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(10000,600000000,whole),root_cancel_token(),||Some(1),&mut sequence,&mut receipt);let original_context=&mut cx as*mut StepContext<'_>;
 let(original,initial)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Box::new(Probe{bytes:vec![7;128],closing:false}) as Box<dyn InteractiveJob+Send>);
 let pointer=original.as_ref()as*const dyn InteractiveJob;let shell=std::mem::size_of_val(original.as_ref());assert!(shell>0);
 let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework::ErasedToolJob::from_owned(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 assert_eq!(owner.retained_job().unwrap()as*const(dyn InteractiveJob+Send)as*const(),pointer as*const());owner.begin_close();let mut born=initial.requested_bytes;let mut freed=initial.released_bytes;
 for _ in 0..10{
  if owner.terminal_is_empty(){break}
  let before=cx.retained_progress();
  for denied in [RetainedCloneGrant{maximum_items:0,..turn},RetainedCloneGrant{maximum_release_bytes:0,..turn},RetainedCloneGrant{maximum_depth:0,..turn}]{
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|{assert_eq!(same as*mut _,original_context);owner.close_step(same)}));
   match result{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>{assert_eq!(error.primary_value().unwrap().kind,ValueRefusalKind::DepthLimit);assert_eq!(error.primary.retained_progress(),Default::default());}}
   assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert!(!owner.terminal_is_empty());assert_eq!(owner.retained_job().unwrap()as*const(dyn InteractiveJob+Send)as*const(),pointer as*const());
  }
  if owner.next_close_copy_byte_demand().unwrap()>0{
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_copy_bytes:0,..turn},|same|owner.close_step(same)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(owner.retained_job().unwrap()as*const(dyn InteractiveJob+Send)as*const(),pointer as*const());
  }
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_capacity_bytes:0,..turn},|same|{assert_eq!(same as*mut _,original_context);owner.close_step(same)}).unwrap());let after=cx.retained_progress();assert_eq!((after.copied_items-before.copied_items,after.copied_bytes-before.copied_bytes,after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes),(step.progress().copied_items,step.progress().copied_bytes,step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;
 }
 assert!(owner.terminal_is_empty());assert!(owner.retained_job().is_none());let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_items:0,..turn},|same|owner.close_step(same)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(born,freed);
}
