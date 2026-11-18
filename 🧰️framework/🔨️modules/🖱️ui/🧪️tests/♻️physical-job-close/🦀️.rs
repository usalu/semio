use super::{ClipboardIoJob, ClipboardIoOperation};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep as Step, RetainedCloneGrant, RetainedCloneProgress};
use super::physical_allocator::measured;

fn close_turn(caller: &mut semio_framework_job::OriginalJobFixtureCaller, grant: RetainedCloneGrant, job: &mut ClipboardIoJob) -> (Step, usize, usize) {
    let mut witnessed = (0, 0);
    let step = caller.turn(grant, |cx| { let (result, births, frees) = measured(|| job.close_step(cx)); witnessed = (births, frees); result }).expect("original fixture close turn");
    (step, witnessed.0, witnessed.1)
}

fn grant(copy: usize, release: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: 0, maximum_release_bytes: release, maximum_depth: 1 }
}

#[test]
fn original_clipboard_backing_refusal_cancel_and_terminal_receipts_are_physical() {
    let mut close_caller=close_source::caller(1,1);
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    for (index, content) in fixture["texts"].as_array().unwrap().iter().enumerate() {
        let text = content.as_str().unwrap();
        let oracle: String = serde_json::from_value(content.clone()).unwrap();
        assert_eq!(oracle.as_bytes(), text.as_bytes());
        for copy in fixture["copyGrants"].as_array().unwrap() {
            for cancel in fixture["cancelTurns"].as_array().unwrap() {
                let mut original = String::with_capacity(fixture["capacities"][index].as_u64().unwrap() as usize);
                original.push_str(text);
                let pointer = original.as_ptr();
                let capacity = original.capacity();
                let (mut job, births, frees) = measured(|| ClipboardIoJob::write(original));
                assert_eq!((births, frees), (0, 0));
                let copy = copy.as_u64().unwrap() as usize;
                for _ in 0..cancel.as_u64().unwrap() {
                    let (result, births, frees) = close_turn(&mut close_caller, grant(copy, capacity), &mut job);
                    assert!(matches!(result, Step::Blocked));
                    assert_eq!((births, frees), (0, 0));
                }
                job.begin_close();
                assert_eq!(job.next_close_copy_byte_demand().unwrap(), 0);
                assert_eq!(job.next_close_capacity_byte_demand(copy).unwrap(), 0);
                assert_eq!(job.next_close_release_byte_demand().unwrap(), capacity);
                let (refused, births, frees) = close_turn(&mut close_caller, grant(copy, capacity - 1), &mut job);
                assert!(matches!(refused, Step::Pending { progress } if progress == RetainedCloneProgress::default()));
                assert_eq!((births, frees), (0, 0));
                match job.operation.as_ref().unwrap() {
                    ClipboardIoOperation::Write(retained) => { assert_eq!(retained.as_ptr(), pointer); assert_eq!(retained, &oracle); },
                    ClipboardIoOperation::Read => panic!("write ownership changed"),
                }
                let (terminal, births, frees) = close_turn(&mut close_caller, grant(copy, capacity), &mut job);
                assert!(matches!(terminal, Step::Complete { progress } if progress.copied_items == 1 && progress.copied_bytes == 0 && progress.retained_capacity_bytes == births && progress.released_bytes == frees));
                assert_eq!((births, frees), (0, capacity));
                assert!(job.terminal_is_empty());
                let (repeat, births, frees) = close_turn(&mut close_caller, grant(0, 0), &mut job);
                assert!(matches!(repeat, Step::Complete { progress } if progress == RetainedCloneProgress::default()));
                assert_eq!((births, frees), (0, 0));
                let (_, births, frees) = measured(|| drop(job));
                assert_eq!((births, frees), (0, 0));
            }
        }
    }

    close_caller.retire(close_source::STOCK);
}

struct ClipboardCaller{caller:semio_framework_job::OriginalJobFixtureCaller,cancel:semio_framework_job::CancelToken,stock:RetainedCloneGrant}
impl ClipboardCaller{
 fn new(operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation,stock:RetainedCloneGrant)->Self{let caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation,generation,stock,||Some(0),u64::MAX,20000);let cancel=caller.cancel().clone();Self{caller,cancel,stock}}
 fn turn<R>(&mut self,grant:RetainedCloneGrant,advance:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->R{self.caller.bare_turn(grant,advance).expect("original clipboard fixture turn")}
 fn alias(&mut self)->semio_framework_job::CancelToken{self.caller.admit_alias(self.stock)}
 fn finish(self){self.caller.retire(self.stock);}
 fn outcome(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin,job:&mut ClipboardIoJob,business:RetainedCloneGrant,site:&'static str,heap:&mut (usize,usize))->bool{
  let mut received=false;
  self.caller.refreshed_turn(business,|cx|cx.with_original_work_loan(1,cx.original_absolute_deadline_us(),|cx|{
   if origin.admit(cx)?{
    let receiver=origin.receiver()?;let mut verdict=None;
    let(returned,born,freed)=measured(||semio_framework_job::drive_step(job,cx,receiver,site,semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict));heap.0+=born;heap.1+=freed;
    if let semio_framework_job::OriginalJobReturn::Outcome(outcome)=returned?{let mut input=Some(outcome.into_descriptor());received=origin.receive_outcome(cx,&mut input)?;}
   }
   Ok(())
  })).expect("original clipboard return turn");
  received
 }
 fn acknowledge(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin,business:RetainedCloneGrant)->RetainedCloneProgress{let mut receipt=Default::default();self.caller.refreshed_turn(business,|cx|{assert!(origin.acknowledge_outcome(cx)?);receipt=cx.retained_progress();Ok(())}).expect("original clipboard acknowledgement turn");receipt}
 fn close_origin(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin){for _ in 0..64{if origin.terminal_is_empty(){break}let stock=self.stock;self.caller.refreshed_turn(stock,|cx|{origin.close_step(cx)?;Ok(())}).expect("original clipboard return close turn");}assert!(origin.terminal_is_empty());}
}

fn policy_grant(original:&serde_json::Value)->RetainedCloneGrant{let policy=&original["policy"];RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize}}

#[test]
fn cancelled_clipboard_outcome_preserves_original_input_until_descriptor_ack_and_paid_close() {
    use semio_framework_job::{OperationId, Generation, JobOutcomeKind, JobOutcomeView, OriginalJobReturnOrigin};
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["clipboardOutcome"];
    let original: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../🧵️job/📬️outcome/🤝️loan/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let supplied = policy_grant(&original);
    let mut owner = ClipboardCaller::new(OperationId(1), Generation(1), supplied);
    let mut source = String::with_capacity(fixture["capacities"][1].as_u64().unwrap() as usize);
    source.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer = source.as_ptr();
    let capacity = source.capacity();
    let mut job = ClipboardIoJob::write(source);
    owner.cancel.cancel_now();
    owner.caller.admit_return_table();
    let mut origin = OriginalJobReturnOrigin::empty();
    let mut heap = (0, 0);
    let denied = RetainedCloneGrant { maximum_items: law["deniedItems"].as_u64().unwrap() as usize, ..supplied };
    assert!(!owner.outcome(&mut origin, &mut job, denied, "original_clipboard_cancelled_outcome", &mut heap));
    assert_eq!(heap, (0, 0));
    let mut received = false;
    for _ in 0..64 { received = owner.outcome(&mut origin, &mut job, supplied, "original_clipboard_cancelled_outcome", &mut heap); if received { break; } }
    assert!(received, "original cancelled outcome is admitted");
    assert_eq!(heap, (0, 0));
    {
        let descriptor = origin.outcome().expect("original cancelled outcome is held by its receiver");
        assert_eq!(descriptor.kind(), JobOutcomeKind::Cancelled);
        let (view, born, freed) = measured(|| job.borrow_outcome(descriptor).unwrap());
        let JobOutcomeView::Cancelled { admission } = view else { panic!("original cancelled descriptor") };
        assert!(std::ptr::eq(admission, descriptor.admission()));
        assert_eq!((born, freed), (law["borrowHeapBytes"].as_u64().unwrap() as usize, 0));
    }
    let Some(ClipboardIoOperation::Write(text)) = job.operation.as_ref() else { panic!("original write input retained through cancellation") };
    assert_eq!(text.as_ptr(), pointer);
    assert_eq!(text.capacity(), capacity);
    assert_eq!(serde_json::to_value(text).unwrap(), fixture["texts"][1]);
    assert!(!job.terminal_is_empty());
    assert_eq!(owner.acknowledge(&mut origin, supplied).copied_items, law["acknowledgementItems"].as_u64().unwrap() as usize);
    assert!(origin.outcome().is_none());
    job.begin_close();
    owner.turn(grant(0, capacity), |cx| {
        let (closed, born, freed) = measured(|| job.close_step(cx).unwrap());
        assert_eq!(closed.progress(), cx.retained_progress());
        assert!(matches!(closed, Step::Complete { .. }));
        assert_eq!((born, freed), (0, capacity));
        assert_eq!(closed.progress(), RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..Default::default() });
        Ok(())
    });
    assert!(job.terminal_is_empty());
    owner.close_origin(&mut origin);
    drop(origin);
    owner.finish();
}

#[test]
fn cancelled_clipboard_driver_binds_the_original_producer_witness(){
    use semio_framework_job::{OperationId,Generation,JobOutcomeView,OriginalJobReturnOrigin};
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let original:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../../🧵️job/📬️outcome/🤝️loan/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let supplied=policy_grant(&original);
    let mut owner=ClipboardCaller::new(OperationId(1),Generation(1),supplied);
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());let capacity=text.capacity();
    let mut job=ClipboardIoJob::write(text);
    owner.cancel.cancel_now();owner.caller.admit_return_table();
    let mut origin=OriginalJobReturnOrigin::empty();let mut heap=(0,0);let mut received=false;
    for _ in 0..64{received=owner.outcome(&mut origin,&mut job,supplied,"original_clipboard_cancelled_bridge",&mut heap);if received{break}}
    assert!(received,"original cancelled bridge outcome is admitted");
    {let(view,born,freed)=measured(||job.borrow_outcome(origin.outcome().expect("original cancelled bridge outcome is held")).unwrap());assert!(matches!(view,JobOutcomeView::Cancelled{..}));assert_eq!((born,freed),(0,0));}
    assert_eq!(owner.acknowledge(&mut origin,supplied).copied_items,1);
    job.begin_close();
    owner.turn(grant(0,capacity),|cx|{let closed=job.close_step(cx).unwrap();assert_eq!(closed.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert_eq!(closed.progress(),cx.retained_progress());Ok(())});
    assert!(job.terminal_is_empty());owner.close_origin(&mut origin);drop(origin);
    owner.finish();
}

#[test]
fn original_clipboard_session_admission_is_paid_before_birth_and_preserves_denied_owners() {
    use semio_framework_job::{Generation,InteractiveStage,OperationId};
    use semio_framework_job_worker::{BatchJobSession,WorkerJobSessionAdmission};use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
    type Admission=WorkerJobSessionAdmission<ClipboardIoJob,BatchJobSession<ClipboardIoJob>>;
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let original:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../../🧵️job/📬️outcome/🤝️loan/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let supplied=policy_grant(&original);
    let mut owner=ClipboardCaller::new(OperationId(1),Generation(1),supplied);
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer=text.as_ptr();let capacity=text.capacity();let oracle:String=serde_json::from_value(fixture["texts"][1].clone()).unwrap();
    let mut job=Some(ClipboardIoJob::write(text));
    let params_for=|cancel:semio_framework_job::CancelToken,operation:OperationId,generation:Generation,site:&'static str|BatchJobParams{operation,generation,cancel,config:BatchDriveConfig{retained:RetainedCloneGrant::default(),site,stage:InteractiveStage::InteractiveStep,fuel_per_step:100,step_budget_us:10},now_us:||Some(0)};
    let mut params=Some(params_for(owner.alias(),OperationId(1),Generation(1),"clipboard_original_owned_admission"));
    let source_address=job.as_ref().unwrap()as*const ClipboardIoJob;let params_address=params.as_ref().unwrap()as*const BatchJobParams;
    let mut admission=Admission::new();let mut output:Option<BatchJobSession<ClipboardIoJob>>=None;
    let retained=|job:&Option<ClipboardIoJob>|{let Some(ClipboardIoOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("refusal retains original source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);};
    let advance=|owner:&mut ClipboardCaller,admission:&mut Admission,job:&mut Option<ClipboardIoJob>,params:&mut Option<BatchJobParams>,output:&mut Option<BatchJobSession<ClipboardIoJob>>,grant:RetainedCloneGrant|owner.turn(grant,|cx|{let(result,born,freed)=measured(||admission.advance(job,params,output,cx));Ok((result,born,freed,cx.retained_progress()))});
    let(first,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,supplied);assert!(!first.expect("source binding turn"));assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));
    for (operation,generation) in [(OperationId(2),Generation(1)),(OperationId(1),Generation(2))] {
        let original_params=params.as_mut().unwrap();original_params.operation=operation;original_params.generation=generation;
        let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,supplied);
        assert!(result.is_err());assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());
        assert_eq!(job.as_ref().unwrap()as*const ClipboardIoJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);retained(&job);
        let original_params=params.as_mut().unwrap();original_params.operation=OperationId(1);original_params.generation=Generation(1);
    }
    let stock=owner.stock;let unrelated=owner.turn(stock,|_|{let(root,born,freed)=measured(||semio_framework_async::CancelToken::admit_root(supplied));let(root,admitted)=root.result.unwrap().unwrap();assert!(admitted.fits(supplied));assert_eq!((born,freed),(admitted.retained_capacity_bytes,admitted.released_bytes));Ok(root)});
    let original_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,unrelated.clone());let mut original_alias=semio_framework_async::CancelTokenRetirement::from_token(original_alias);let witness=owner.cancel.clone();
    owner.turn(supplied,|cx|{let(returned,born,freed)=measured(||cx.with_original_effect(supplied,|_,effect|original_alias.return_alias_step(&witness,effect).result.unwrap()).unwrap().expect("original effect funded").0);assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));Ok(())});assert!(original_alias.terminal_is_empty());
    let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,supplied);assert!(result.is_err());assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());
    assert_eq!(job.as_ref().unwrap()as*const ClipboardIoJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);retained(&job);
    let unrelated_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,witness.clone());let mut unrelated_alias=semio_framework_async::CancelTokenRetirement::from_token(unrelated_alias);
    owner.turn(supplied,|cx|{let(returned,born,freed)=measured(||cx.with_original_effect(supplied,|_,effect|unrelated_alias.return_alias_step(&unrelated,effect).result.unwrap()).unwrap().expect("original effect funded").0);assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));Ok(())});assert!(unrelated_alias.terminal_is_empty());
    let mut unrelated=semio_framework_async::CancelTokenRetirement::from_token(unrelated);
    while !unrelated.terminal_is_empty(){owner.turn(supplied,|cx|{let(closed,born,freed)=measured(||cx.with_original_effect(supplied,|_,effect|unrelated.close_step(effect).result.unwrap()).unwrap().expect("original effect funded").0);assert!(closed.progress().fits(supplied));assert_eq!((born,freed),(closed.progress().retained_capacity_bytes,closed.progress().released_bytes));Ok(())});}
    for denied in [RetainedCloneGrant{maximum_items:0,..supplied},RetainedCloneGrant{maximum_depth:0,..supplied}] {
        let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,denied);
        assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());
        assert_eq!(job.as_ref().unwrap()as*const ClipboardIoJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);retained(&job);
    }
    let mut turns=0;while output.is_none(){let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,supplied);result.expect("original clipboard session admission turn");assert!(receipt.fits(supplied));assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));turns+=1;assert!(turns<4096);}
    assert!(admission.terminal_is_empty()&&job.is_none()&&params.is_none());let mut session=output.take().unwrap();
    let cancelled_alias=owner.alias();owner.cancel.cancel_now();let _=session.begin_close();
    for _ in 0..4096 {if session.terminal_is_empty(){break}owner.turn(supplied,|cx|{let(step,born,freed)=measured(||{if session.has_original_cancel_alias_witness(cx.original_cancel_token()).unwrap(){let step=cx.with_original_effect(cx.retained_grant(),|cx,effect|session.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn).result.unwrap().unwrap();cx.consume_retained(step.progress()).unwrap();cx.consume_fuel(1);Ok::<_,semio_framework_job::OriginalTurnFailure>(semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{progress:step.progress()})}else{session.close_step(cx)}});let step=step?;assert!(step.progress().fits(supplied));assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.progress().released_bytes,freed);Ok(())});}
    assert!(session.terminal_is_empty());let(_,born,freed)=measured(||drop(session));assert_eq!((born,freed),(0,0));
    let mut cancelled_text=String::with_capacity(capacity);cancelled_text.push_str(&oracle);let cancelled_pointer=cancelled_text.as_ptr();
    let mut cancelled_job=Some(ClipboardIoJob::write(cancelled_text));
    let mut cancelled_params=Some(params_for(cancelled_alias,OperationId(1),Generation(1),"clipboard_original_cancelled_admission"));
    let mut cancelled_admission=Admission::new();let mut cancelled_output=None;
    let(result,born,freed,receipt)=advance(&mut owner,&mut cancelled_admission,&mut cancelled_job,&mut cancelled_params,&mut cancelled_output,supplied);assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));
    let(result,born,freed,receipt)=advance(&mut owner,&mut cancelled_admission,&mut cancelled_job,&mut cancelled_params,&mut cancelled_output,supplied);assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());assert!(cancelled_output.is_none());
    let Some(ClipboardIoOperation::Write(text))=cancelled_job.as_ref().unwrap().operation.as_ref()else{panic!("original precancelled input")};assert_eq!(text.as_ptr(),cancelled_pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);assert!(cancelled_params.is_some());
    for _ in 0..4096{if cancelled_admission.terminal_is_empty(){break}owner.turn(supplied,|cx|{cancelled_admission.close_step(cx)?;Ok(())});}assert!(cancelled_admission.terminal_is_empty());
    let mut cancelled_job=cancelled_job.take().unwrap();cancelled_job.begin_close();owner.turn(RetainedCloneGrant{maximum_items:1,..supplied},|cx|{let step=cancelled_job.close_step(cx).unwrap();assert_eq!(step.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert_eq!(step.progress(),cx.retained_progress());Ok(())});assert!(cancelled_job.terminal_is_empty());
    let params=cancelled_params.take().unwrap();let mut alias=semio_framework_async::CancelTokenRetirement::from_token(params.cancel);let witness=owner.cancel.clone();while !alias.terminal_is_empty(){owner.turn(supplied,|cx|{assert!(cx.with_original_effect(supplied,|_,effect|alias.return_alias_step(&witness,effect).result.unwrap()).unwrap().expect("original effect funded").0.progress().fits(supplied));Ok(())});}
    owner.finish();
}

#[path="../../../🧵️job/🧪️testing/♻️close/🦀️.rs"]
pub(crate) mod close_source;
