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

fn fixture_now_us()->Option<u64>{Some(0)}

struct ClipboardCaller{caller:semio_framework_job::OriginalJobFixtureCaller,cancel:semio_framework_job::CancelToken,stock:RetainedCloneGrant}
impl ClipboardCaller{
 fn new(operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation)->Self{let stock=close_source::STOCK;let caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation,generation,stock,fixture_now_us,u64::MAX,1_000_000);let cancel=caller.cancel().clone();Self{caller,cancel,stock}}
 fn turn<R>(&mut self,grant:RetainedCloneGrant,advance:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->R{self.caller.bare_turn(grant,advance).expect("original clipboard fixture turn")}
 fn alias(&mut self)->semio_framework_job::CancelToken{self.caller.admit_alias(self.stock)}
 fn finish(self){let Self{caller,cancel,stock}=self;drop(cancel);caller.retire(stock);}
 fn admit_origin(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin,grant:RetainedCloneGrant)->(bool,RetainedCloneProgress,(usize,usize)){
  let mut admitted=(false,RetainedCloneProgress::default(),(0,0));
  self.caller.refreshed_turn(grant,|cx|{let before=cx.retained_progress();let(result,born,freed)=measured(||origin.admit(cx));admitted=(result?,spent(before,cx.retained_progress()),(born,freed));Ok(())}).expect("original clipboard return origin turn");
  admitted
 }
 fn drive(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin,job:&mut ClipboardIoJob,site:&'static str)->(bool,RetainedCloneProgress,(usize,usize)){
  let mut driven=(false,RetainedCloneProgress::default(),(0,0));let stock=self.stock;
  self.caller.refreshed_turn(stock,|cx|cx.with_original_work_loan(1,cx.original_absolute_deadline_us(),|cx|{let before=cx.retained_progress();let receiver=origin.receiver()?;let mut verdict=None;let(returned,born,freed)=measured(||semio_framework_job::drive_step(job,cx,receiver,site,semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict));driven=(matches!(returned?,semio_framework_job::OriginalJobReturn::Preparing),spent(before,cx.retained_progress()),(born,freed));Ok(())})).expect("original clipboard return turn");
  driven
 }
 fn close_origin(&mut self,origin:&mut semio_framework_job::OriginalJobReturnOrigin){for _ in 0..64{if origin.terminal_is_empty(){break}let stock=self.stock;self.caller.refreshed_turn(stock,|cx|{origin.close_step(cx)?;Ok(())}).expect("original clipboard return close turn");}assert!(origin.terminal_is_empty());}
}

fn spent(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}}

fn policy_grant(original:&serde_json::Value)->RetainedCloneGrant{let policy=&original["policy"];RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize}}

/// 🚫️ An already cancelled caller never admits a return origin, so the write never enters its producer and keeps its input until the paid close.
#[test]
fn cancelled_clipboard_input_never_enters_its_producer_and_is_held_until_paid_close() {
    use semio_framework_job::{OperationId, Generation, OriginalJobReturnOrigin};
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["clipboardOutcome"];
    let original: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../🧵️job/📬️outcome/🤝️loan/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let supplied = policy_grant(&original);
    let mut owner = ClipboardCaller::new(OperationId(1), Generation(1));
    let mut source = String::with_capacity(fixture["capacities"][1].as_u64().unwrap() as usize);
    source.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer = source.as_ptr();
    let capacity = source.capacity();
    let mut job = ClipboardIoJob::write(source);
    owner.caller.admit_return_table();
    owner.cancel.cancel_now();
    let mut origin = OriginalJobReturnOrigin::empty();
    let refused = law["refusedHeapBytes"].as_u64().unwrap() as usize;
    for grant in [RetainedCloneGrant { maximum_items: law["deniedItems"].as_u64().unwrap() as usize, ..supplied }, supplied] {
        let (admitted, receipt, heap) = owner.admit_origin(&mut origin, grant);
        assert!(!admitted);
        assert_eq!(receipt, RetainedCloneProgress::default());
        assert_eq!(heap, (refused, refused));
        assert!(origin.terminal_is_empty());
    }
    let Some(ClipboardIoOperation::Write(text)) = job.operation.as_ref() else { panic!("original write input retained through cancellation") };
    assert_eq!(text.as_ptr() == pointer, law["sameInputPointer"].as_bool().unwrap());
    assert_eq!(text.capacity(), capacity);
    assert_eq!(serde_json::to_value(text).unwrap(), fixture["texts"][1]);
    assert_eq!(!job.terminal_is_empty(), law["inputHeldUntilClose"].as_bool().unwrap());
    job.begin_close();
    owner.turn(grant(0, capacity), |cx| {
        let before = cx.retained_progress();
        let (closed, born, freed) = measured(|| job.close_step(cx).unwrap());
        assert_eq!(closed.progress(), spent(before, cx.retained_progress()));
        assert!(matches!(closed, Step::Complete { .. }));
        assert_eq!((born, freed), (0, capacity));
        assert_eq!(closed.progress(), RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..Default::default() });
        Ok(())
    });
    assert!(job.terminal_is_empty());
    drop(origin);
    owner.finish();
}

/// 🛑️ A return origin admitted before cancellation stays unentered: the driver prepares nothing and the write keeps its input until both paid closes.
#[test]
fn cancelled_clipboard_driver_never_enters_an_admitted_origin(){
    use semio_framework_job::{OperationId,Generation,OriginalJobReturnOrigin};
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let law=&fixture["clipboardOutcome"];let refused=law["refusedHeapBytes"].as_u64().unwrap()as usize;
    let mut owner=ClipboardCaller::new(OperationId(1),Generation(1));
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());let pointer=text.as_ptr();let capacity=text.capacity();
    let mut job=ClipboardIoJob::write(text);
    owner.caller.admit_return_table();
    let mut origin=OriginalJobReturnOrigin::empty();let stock=owner.stock;let mut admitted=false;
    for _ in 0..4096{admitted=owner.admit_origin(&mut origin,stock).0;if admitted{break}}
    assert!(admitted,"the original return origin admits inside its finite turns");
    owner.cancel.cancel_now();
    let(admitted,receipt,heap)=owner.admit_origin(&mut origin,stock);assert!(!admitted);assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!(heap,(refused,refused));
    let(preparing,receipt,heap)=owner.drive(&mut origin,&mut job,"original_clipboard_cancelled_bridge");assert!(preparing,"a cancelled driver never enters its producer");assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!(heap,(refused,refused));
    assert!(origin.outcome().is_none()&&origin.continuation().is_none());
    let Some(ClipboardIoOperation::Write(text))=job.operation.as_ref()else{panic!("original write input retained through cancellation")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);
    job.begin_close();
    owner.turn(grant(0,capacity),|cx|{let before=cx.retained_progress();let closed=job.close_step(cx).unwrap();assert_eq!(closed.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert_eq!(closed.progress(),spent(before,cx.retained_progress()));Ok(())});
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
    let mut owner=ClipboardCaller::new(OperationId(1),Generation(1));
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer=text.as_ptr();let capacity=text.capacity();let oracle:String=serde_json::from_value(fixture["texts"][1].clone()).unwrap();
    let mut job=Some(ClipboardIoJob::write(text));
    let params_for=|cancel:semio_framework_job::CancelToken,operation:OperationId,generation:Generation,site:&'static str|BatchJobParams{operation,generation,cancel,config:BatchDriveConfig{retained:RetainedCloneGrant::default(),site,stage:InteractiveStage::InteractiveStep,fuel_per_step:100,step_budget_us:10},now_us:fixture_now_us};
    let mut params=Some(params_for(owner.alias(),OperationId(1),Generation(1),"clipboard_original_owned_admission"));
    let source_address=job.as_ref().unwrap()as*const ClipboardIoJob;let params_address=params.as_ref().unwrap()as*const BatchJobParams;
    let mut admission=Admission::new();let mut output:Option<BatchJobSession<ClipboardIoJob>>=None;
    let retained=|job:&Option<ClipboardIoJob>|{let Some(ClipboardIoOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("refusal retains original source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);};
    let advance=|owner:&mut ClipboardCaller,admission:&mut Admission,job:&mut Option<ClipboardIoJob>,params:&mut Option<BatchJobParams>,output:&mut Option<BatchJobSession<ClipboardIoJob>>,grant:RetainedCloneGrant|{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.turn(grant,|cx|Ok(admission.advance(job,params,output,cx))));(result,heap.requested_bytes,heap.released_bytes,owner.caller.receipt())};
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
    let restored=owner.alias();let unrelated_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,restored);let mut unrelated_alias=semio_framework_async::CancelTokenRetirement::from_token(unrelated_alias);
    owner.turn(supplied,|cx|{let(returned,born,freed)=measured(||cx.with_original_effect(supplied,|_,effect|unrelated_alias.return_alias_step(&unrelated,effect).result.unwrap()).unwrap().expect("original effect funded").0);assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));Ok(())});assert!(unrelated_alias.terminal_is_empty());
    let mut unrelated=semio_framework_async::CancelTokenRetirement::from_token(unrelated);
    while !unrelated.terminal_is_empty(){owner.turn(supplied,|cx|{let(closed,born,freed)=measured(||cx.with_original_effect(supplied,|_,effect|unrelated.close_step(effect).result.unwrap()).unwrap().expect("original effect funded").0);assert!(closed.progress().fits(supplied));assert_eq!((born,freed),(closed.progress().retained_capacity_bytes,closed.progress().released_bytes));Ok(())});}
    for denied in [RetainedCloneGrant{maximum_items:0,..supplied},RetainedCloneGrant{maximum_depth:0,..supplied}] {
        let(result,born,freed,receipt)=advance(&mut owner,&mut admission,&mut job,&mut params,&mut output,denied);
        assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());
        assert_eq!(job.as_ref().unwrap()as*const ClipboardIoJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);retained(&job);
    }
    owner.caller.admit_return_table();let mut turns=0;while output.is_none(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.caller.refreshed_turn(supplied,|cx|admission.advance(&mut job,&mut params,&mut output,cx)));result.expect("original clipboard session admission turn");let receipt=owner.caller.receipt();assert!(owner.caller.business_receipt().fits(supplied));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));turns+=1;assert!(turns<4096);}
    assert!(admission.terminal_is_empty()&&job.is_none()&&params.is_none());let mut session=output.take().unwrap();
    let cancelled_alias=owner.alias();owner.cancel.cancel_now();let _=session.begin_close();
    let closing=RetainedCloneGrant{maximum_items:fixture["sessionClose"]["items"].as_u64().unwrap()as usize,maximum_copy_bytes:fixture["sessionClose"]["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:fixture["sessionClose"]["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:fixture["sessionClose"]["release"].as_u64().unwrap()as usize,maximum_depth:fixture["sessionClose"]["depth"].as_u64().unwrap()as usize};
    for _ in 0..4096 {if session.terminal_is_empty(){break}owner.caller.refreshed_turn(closing,|cx|{let(step,born,freed)=measured(||{if session.has_original_cancel_alias_witness(cx.original_cancel_token()).unwrap(){let step=cx.with_original_effect(cx.retained_grant(),|cx,effect|session.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn).result.unwrap().unwrap();cx.consume_retained(step.progress()).unwrap();cx.consume_fuel(1);Ok::<_,semio_framework_job::OriginalTurnFailure>(semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{progress:step.progress()})}else{session.close_step(cx)}});let step=step?;assert!(step.progress().fits(closing));assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.progress().released_bytes,freed);Ok(())}).expect("original clipboard session close turn");}
    assert!(session.terminal_is_empty(),"original clipboard session closes inside its finite turns: phase={:?} demand={:?}",session.close_phase(),session.retirement_demands(closing.maximum_copy_bytes));let(_,born,freed)=measured(||drop(session));assert_eq!((born,freed),(0,0));
    let mut cancelled_text=String::with_capacity(capacity);cancelled_text.push_str(&oracle);let cancelled_pointer=cancelled_text.as_ptr();
    let mut cancelled_job=Some(ClipboardIoJob::write(cancelled_text));
    let mut cancelled_params=Some(params_for(cancelled_alias,OperationId(1),Generation(1),"clipboard_original_cancelled_admission"));
    let mut cancelled_admission=Admission::new();let mut cancelled_output=None;
    let(result,born,freed,receipt)=advance(&mut owner,&mut cancelled_admission,&mut cancelled_job,&mut cancelled_params,&mut cancelled_output,supplied);assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));
    let(result,born,freed,receipt)=advance(&mut owner,&mut cancelled_admission,&mut cancelled_job,&mut cancelled_params,&mut cancelled_output,supplied);assert!(matches!(result,Ok(false)));assert_eq!((born,freed),(0,0));assert_eq!(receipt,RetainedCloneProgress::default());assert!(cancelled_output.is_none());
    let Some(ClipboardIoOperation::Write(text))=cancelled_job.as_ref().unwrap().operation.as_ref()else{panic!("original precancelled input")};assert_eq!(text.as_ptr(),cancelled_pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);assert!(cancelled_params.is_some());
    for _ in 0..4096{if cancelled_admission.terminal_is_empty(){break}owner.turn(supplied,|cx|{cancelled_admission.close_step(cx)?;Ok(())});}assert!(cancelled_admission.terminal_is_empty());
    let mut cancelled_job=cancelled_job.take().unwrap();cancelled_job.begin_close();owner.turn(RetainedCloneGrant{maximum_items:1,..supplied},|cx|{let before=cx.retained_progress();let step=cancelled_job.close_step(cx).unwrap();assert_eq!(step.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert_eq!(step.progress(),spent(before,cx.retained_progress()));Ok(())});assert!(cancelled_job.terminal_is_empty());
    let params=cancelled_params.take().unwrap();let mut alias=semio_framework_async::CancelTokenRetirement::from_token(params.cancel);while !alias.terminal_is_empty(){owner.turn(supplied,|cx|{assert!(cx.with_original_effect(supplied,|_,effect|alias.return_alias_step(&witness,effect).result.unwrap()).unwrap().expect("original effect funded").0.progress().fits(supplied));Ok(())});}
    drop(witness);
    owner.finish();
}

#[path="../../../🧵️job/🧪️tests/♻️close/🦀️.rs"]
pub(crate) mod close_source;
