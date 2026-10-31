//! 🎟️ Genuine source admission retains each partial owner before its original receipt.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn law()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn clock()->Option<u64>{Some(1000)}
fn stock(row:&serde_json::Value)->RetainedCloneGrant{let a=row["stock"].as_array().unwrap();RetainedCloneGrant{maximum_items:a[0].as_u64().unwrap()as usize,maximum_copy_bytes:a[1].as_u64().unwrap()as usize,maximum_capacity_bytes:a[2].as_u64().unwrap()as usize,maximum_release_bytes:a[3].as_u64().unwrap()as usize,maximum_depth:a[4].as_u64().unwrap()as usize}}
struct Probe{entered:usize,closed:bool}
impl InteractiveJob for Probe{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{self.entered+=1;InteractiveJobStep::from_callback(JobOutcomeBorrow::admit_yield(cx).map_err(Into::into))}
 fn borrow_outcome<'a>(&'a self,d:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{d.yielded()}
 fn begin_close(&mut self){self.closed=true;}
 fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
 fn terminal_is_empty(&self)->bool{self.closed}
}
#[test]
fn authentic_caller_retains_prepared_worker_before_every_posting_boundary(){
 let row=law();let bank=stock(&row);let op=OperationId(row["source"]["operation"].as_u64().unwrap());let generation=Generation(row["source"]["generation"].as_u64().unwrap());
 let mut caller=OriginalJobFixtureCaller::admit(op,generation,bank,clock,row["source"]["deadlineUs"].as_u64().unwrap(),row["fuel"].as_u64().unwrap());
 let mut cancel=None;caller.turn(bank,|cx|{cx.admit_original_cancel_alias(&mut cancel)?;Ok(())}).unwrap();let mut admission=OriginalWorkerJobAdmission::<Probe>::new();let mut job=Some(Probe{entered:0,closed:false});let mut params=Some(BatchJobParams{operation:op,generation,cancel:cancel.take().unwrap(),config:BatchDriveConfig{retained:bank,site:"authentic-worker-admission",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:500},now_us:clock});
 let q=RetainedCloneGrant{maximum_items:1,..bank};let zero=RetainedCloneGrant{maximum_items:0,..q};
 for denied in [zero,RetainedCloneGrant{maximum_depth:0,..q}]{caller.turn(denied,|cx|{let before=cx.retained_progress();let work=cx.original_work_units_spent();let(result,heap)=observe(||admission.advance(&mut job,&mut params,cx));assert!(!result?);assert_eq!(cx.retained_progress(),before);assert_eq!(cx.original_work_units_spent(),work);assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,0);assert!(!heap.overflowed);Ok(())}).unwrap();assert!(admission.terminal_is_empty());assert!(job.is_some()&&params.is_some());}
 let mut born=0;let mut released=0;
 for _ in 0..row["maximumTurns"].as_u64().unwrap(){if admission.ready(){break;}caller.turn(q,|cx|{let view=cx.retained_grant();cx.narrow_original_work_budget(StepBudget::new(1,cx.deadline_us(),view))?;let before=cx.retained_progress();let work=cx.original_work_units_spent();let(result,heap)=observe(||admission.advance(&mut job,&mut params,cx));let after=cx.retained_progress();assert!(!heap.overflowed);assert_eq!(heap.requested_bytes,after.retained_capacity_bytes-before.retained_capacity_bytes);assert_eq!(heap.released_bytes,after.released_bytes-before.released_bytes);assert!(cx.original_work_units_spent()-work<=1);born+=heap.requested_bytes;released+=heap.released_bytes;result?;Ok(())}).unwrap();}
 assert!(admission.ready());assert_eq!(admission.job().unwrap().entered,row["bodyEntriesBeforeReceiver"].as_u64().unwrap()as usize);assert!(job.is_none()&&params.is_none());caller.turn(q,|cx|{let before=cx.retained_progress();let work=cx.original_work_units_spent();let(result,heap)=observe(||admission.advance(&mut job,&mut params,cx));assert!(result?);assert_eq!(cx.retained_progress(),before);assert_eq!(cx.original_work_units_spent(),work);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Ok(())}).unwrap();
 let mut foreign=OriginalJobFixtureCaller::admit(op,generation,bank,clock,row["source"]["deadlineUs"].as_u64().unwrap(),row["fuel"].as_u64().unwrap());
 foreign.turn(q,|cx|{let(result,heap)=observe(||admission.advance(&mut job,&mut params,cx));assert!(result.is_err());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Ok(())}).unwrap();foreign.retire(bank);
 caller.cancel().cancel_now();
 for _ in 0..row["maximumTurns"].as_u64().unwrap(){if admission.terminal_is_empty(){break;}caller.turn(q,|cx|{let before=cx.retained_progress();let(result,heap)=observe(||admission.close_step(cx));let after=cx.retained_progress();assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;result?;Ok(())}).unwrap();}
 assert!(admission.terminal_is_empty());let(_,heap)=observe(||drop(admission));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(born,released);caller.retire(bank);
}
