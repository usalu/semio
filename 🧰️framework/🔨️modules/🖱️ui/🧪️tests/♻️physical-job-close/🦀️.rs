use super::{ClipboardIoJob, ClipboardIoOperation};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep as Step, RetainedCloneGrant, RetainedCloneProgress};
use std::alloc::{GlobalAlloc, Layout};
use std::cell::Cell;

thread_local! {
    static OBSERVATION: Cell<Option<(usize, usize)>> = const { Cell::new(None) };
}

struct PhysicalAllocator;

unsafe impl GlobalAlloc for PhysicalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { semio_framework_trace::HeapWitness.alloc(layout) };
        if !pointer.is_null() {
            let _ = OBSERVATION.try_with(|state| if let Some((births, frees)) = state.get() { state.set(Some((births + layout.size(), frees))) });
        }
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let _ = OBSERVATION.try_with(|state| if let Some((births, frees)) = state.get() { state.set(Some((births, frees + layout.size()))) });
        unsafe { semio_framework_trace::HeapWitness.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: PhysicalAllocator = PhysicalAllocator;

pub(crate) fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    OBSERVATION.with(|state| { assert!(state.get().is_none()); state.set(Some((0, 0))); });
    let result = operation();
    let (births, frees) = OBSERVATION.with(|state| state.replace(None).unwrap());
    (result, births, frees)
}

fn grant(copy: usize, release: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: 0, maximum_release_bytes: release, maximum_depth: 1 }
}

#[test]
fn original_clipboard_backing_refusal_cancel_and_terminal_receipts_are_physical() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
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
                    let (result, births, frees) = measured(|| job.close_step(grant(copy, capacity)));
                    assert!(matches!(result, Step::Blocked));
                    assert_eq!((births, frees), (0, 0));
                }
                job.begin_close();
                assert_eq!(job.next_close_copy_byte_demand().unwrap(), 0);
                assert_eq!(job.next_close_capacity_byte_demand(copy).unwrap(), 0);
                assert_eq!(job.next_close_release_byte_demand().unwrap(), capacity);
                let (refused, births, frees) = measured(|| job.close_step(grant(copy, capacity - 1)));
                assert!(matches!(refused, Step::Pending { progress } if progress == RetainedCloneProgress::default()));
                assert_eq!((births, frees), (0, 0));
                match job.operation.as_ref().unwrap() {
                    ClipboardIoOperation::Write(retained) => { assert_eq!(retained.as_ptr(), pointer); assert_eq!(retained, &oracle); },
                    ClipboardIoOperation::Read => panic!("write ownership changed"),
                }
                let (terminal, births, frees) = measured(|| job.close_step(grant(copy, capacity)));
                assert!(matches!(terminal, Step::Complete { progress } if progress.copied_items == 1 && progress.copied_bytes == 0 && progress.retained_capacity_bytes == births && progress.released_bytes == frees));
                assert_eq!((births, frees), (0, capacity));
                assert!(job.terminal_is_empty());
                let (repeat, births, frees) = measured(|| job.close_step(grant(0, 0)));
                assert!(matches!(repeat, Step::Complete { progress } if progress == RetainedCloneProgress::default()));
                assert_eq!((births, frees), (0, 0));
                let (_, births, frees) = measured(|| drop(job));
                assert_eq!((births, frees), (0, 0));
            }
        }
    }
}

#[test]
fn cancelled_clipboard_outcome_preserves_original_input_until_descriptor_ack_and_paid_close() {
    use semio_framework_job::{StepContextOwner, StepBudget, OperationId, Generation, JobOutcomeKind, JobOutcomeView};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["clipboardOutcome"];
    let original: serde_json::Value = serde_json::from_str(include_str!("../../../🧵️job/📬️outcome/🤝️loan/🧫️fixtures/🔣️.json")).unwrap();
    let policy = &original["policy"];
    let supplied = RetainedCloneGrant { maximum_items: policy["items"].as_u64().unwrap() as usize, maximum_copy_bytes: policy["copy"].as_u64().unwrap() as usize, maximum_capacity_bytes: policy["capacity"].as_u64().unwrap() as usize, maximum_release_bytes: policy["release"].as_u64().unwrap() as usize, maximum_depth: policy["depth"].as_u64().unwrap() as usize };
    let (mut owner, _) = StepContextOwner::new(OperationId(1), Generation(1), supplied).unwrap();
    let mut source = String::with_capacity(fixture["capacities"][1].as_u64().unwrap() as usize);
    source.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer = source.as_ptr();
    let capacity = source.capacity();
    let mut job = ClipboardIoJob::write(source);
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut denied_receipt = RetainedCloneProgress::default();
    let mut denied = owner.context(StepBudget::new(100, 10, RetainedCloneGrant { maximum_items: law["deniedItems"].as_u64().unwrap() as usize, ..supplied }), cancel.clone(), || Some(0), &mut sequence, &mut denied_receipt).unwrap();
    let (outcome, born, freed) = measured(|| job.step(&mut denied).unwrap().map(|outcome| outcome.into_descriptor()));
    assert!(outcome.is_none());
    assert_eq!((born, freed), (0, 0));
    assert_eq!(denied.retained_progress(), RetainedCloneProgress::default());
    drop(denied);
    let mut receipt = RetainedCloneProgress::default();
    let mut cx = owner.context(StepBudget::new(100, 10, supplied), cancel, || Some(0), &mut sequence, &mut receipt).unwrap();
    let (outcome, born, freed) = measured(|| job.step(&mut cx).unwrap().map(|outcome| outcome.into_descriptor()));
    let mut descriptor = outcome.expect("original cancelled outcome is admitted");
    assert_eq!((born, freed), (0, 0));
    assert_eq!(descriptor.kind(), JobOutcomeKind::Cancelled);
    assert_eq!(cx.retained_progress(), RetainedCloneProgress { copied_items: 1, ..Default::default() });
    drop(cx);
    let (view, born, freed) = measured(|| job.borrow_outcome(&descriptor).unwrap());
    let JobOutcomeView::Cancelled { admission } = view else { panic!("original cancelled descriptor") };
    assert!(std::ptr::eq(admission, descriptor.admission()));
    assert_eq!((born, freed), (law["borrowHeapBytes"].as_u64().unwrap() as usize, 0));
    let Some(ClipboardIoOperation::Write(text)) = job.operation.as_ref() else { panic!("original write input retained through cancellation") };
    assert_eq!(text.as_ptr(), pointer);
    assert_eq!(text.capacity(), capacity);
    assert_eq!(serde_json::to_value(text).unwrap(), fixture["texts"][1]);
    assert!(!job.terminal_is_empty());
    assert_eq!(descriptor.acknowledge(supplied).progress().copied_items, law["acknowledgementItems"].as_u64().unwrap() as usize);
    drop(descriptor);
    job.begin_close();
    let (closed, born, freed) = measured(|| job.close_step(grant(0, capacity)));
    assert!(matches!(closed, Step::Complete { .. }));
    assert_eq!((born, freed), (0, capacity));
    assert_eq!(closed.progress(), RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..Default::default() });
    assert!(job.terminal_is_empty());
    while !owner.terminal_is_empty() { assert!(owner.close_step(supplied).progress().fits(supplied)); }
    eprintln!("[DEBUG] clipboard cancelled descriptor original input pointer/capacity preserved; denied outcome0heap, borrowed admission0heap, original ACK and exact native release funded separately");
}

#[test]
fn cancelled_clipboard_driver_binds_the_original_producer_witness(){
    use semio_framework_job::{InteractiveJob,StepContextOwner,StepBudget,OperationId,Generation,JobOutcomeView,RetainedCloneGrant,RetainedCloneProgress};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let original:serde_json::Value=serde_json::from_str(include_str!("../../../🧵️job/📬️outcome/🤝️loan/🧫️fixtures/🔣️.json")).unwrap();
    let policy=&original["policy"];
    let grant=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
    let(mut owner,_)=StepContextOwner::new(OperationId(1),Generation(1),grant).unwrap();
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());let capacity=text.capacity();
    let mut job=ClipboardIoJob::write(text);
    let cancel=semio_framework_job::root_cancel_token();cancel.cancel_now();
    let mut sequence=0;let mut receipt=RetainedCloneProgress::default();let mut verdict=None;
    let mut cx=owner.context(StepBudget::new(100,10,grant),cancel,||Some(0),&mut sequence,&mut receipt).unwrap();
    let mut descriptor=semio_framework_job::drive_step(&mut job,&mut cx,"original_clipboard_cancelled_bridge",semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict).unwrap().unwrap().into_descriptor();
    assert_eq!(cx.retained_progress(),RetainedCloneProgress{copied_items:1,..Default::default()});drop(cx);
    let(view,born,freed)=measured(||job.borrow_outcome(&descriptor).unwrap());assert!(matches!(view,JobOutcomeView::Cancelled{..}));assert_eq!((born,freed),(0,0));
    assert_eq!(descriptor.acknowledge(grant).progress().copied_items,1);drop(descriptor);
    job.begin_close();let closed=job.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1});assert_eq!(closed.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert!(job.terminal_is_empty());
    while !owner.terminal_is_empty(){assert!(owner.close_step(grant).progress().fits(grant))}
    eprintln!("[DEBUG] actual clipboard drive_step original cancelled producer witness borrowed until funded ACK and physical input close");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn original_clipboard_session_admission_is_paid_before_birth_and_preserves_denied_owners() {
    use semio_framework_job::{BatchDriveConfig,BatchJobParams,Generation,InteractiveJob,InteractiveStage,OperationId,RetainedCloneGrant,RetainedCloneProgress,StepBudget,StepContextOwner,WorkerJobSession};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let original:serde_json::Value=serde_json::from_str(include_str!("../../../🧵️job/📬️outcome/🤝️loan/🧫️fixtures/🔣️.json")).unwrap();
    let policy=&original["policy"];
    let supplied=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
    let demand=WorkerJobSession::<ClipboardIoJob>::admission_demand();
    assert!(demand.capacity_bytes>0&&demand.capacity_bytes<=supplied.maximum_capacity_bytes);
    assert!(demand.copy_bytes<=supplied.maximum_copy_bytes&&demand.depth<=supplied.maximum_depth);
    let(mut owner,_)=StepContextOwner::new(OperationId(1),Generation(1),supplied).unwrap();
    let cancel=semio_framework_job::root_cancel_token();
    let mut text=String::with_capacity(fixture["capacities"][1].as_u64().unwrap()as usize);text.push_str(fixture["texts"][1].as_str().unwrap());
    let pointer=text.as_ptr();let capacity=text.capacity();let oracle:String=serde_json::from_value(fixture["texts"][1].clone()).unwrap();
    let mut job=Some(ClipboardIoJob::write(text));
    let mut params=Some(BatchJobParams{operation:OperationId(1),generation:Generation(1),cancel:cancel.clone(),config:BatchDriveConfig{retained:RetainedCloneGrant::default(),site:"clipboard_original_owned_admission",stage:InteractiveStage::InteractiveStep,fuel_per_step:100,step_budget_us:10},now_us:||Some(0)});
    let source_address=job.as_ref().unwrap()as*const ClipboardIoJob;let params_address=params.as_ref().unwrap()as*const BatchJobParams;
    let mut sequence=0;
    for (operation,generation) in [(OperationId(2),Generation(1)),(OperationId(1),Generation(2))] {
        let original_params=params.as_mut().unwrap();original_params.operation=operation;original_params.generation=generation;
        let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,supplied),cancel.clone(),||Some(0),&mut sequence,&mut receipt).unwrap();
        let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut job,&mut params,&mut cx));
        assert!(matches!(result,Err(ref error) if error.kind==semio_framework_value::ValueRefusalKind::InvariantViolated));assert_eq!((born,freed),(0,0));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());drop(cx);
        assert_eq!(job.as_ref().unwrap()as*const ClipboardIoJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);
        let Some(ClipboardIoOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("identity refusal retains original source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);
        let original_params=params.as_mut().unwrap();original_params.operation=OperationId(1);original_params.generation=Generation(1);
    }
    let(unrelated,born,freed)=measured(||semio_framework_async::CancelToken::admit_root(supplied));let(unrelated,admitted)=unrelated.unwrap().unwrap();assert!(admitted.fits(supplied));assert_eq!((born,freed),(admitted.retained_capacity_bytes,admitted.released_bytes));
    let original_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,unrelated.clone());let mut original_alias=semio_framework_async::CancelTokenRetirement::from_token(original_alias);
    let(returned,born,freed)=measured(||original_alias.return_alias_step(&cancel,supplied).unwrap());assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));assert!(original_alias.terminal_is_empty());
    let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,supplied),cancel.clone(),||Some(0),&mut sequence,&mut receipt).unwrap();
    let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut job,&mut params,&mut cx));assert!(matches!(result,Err(ref error) if error.kind==semio_framework_value::ValueRefusalKind::InvariantViolated));assert_eq!((born,freed),(0,0));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());drop(cx);
    assert_eq!(job.as_ref().unwrap()as*const ClipboardIoJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);
    let Some(ClipboardIoOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("original cancellation refusal retains source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);
    let unrelated_alias=std::mem::replace(&mut params.as_mut().unwrap().cancel,cancel.clone());let mut unrelated_alias=semio_framework_async::CancelTokenRetirement::from_token(unrelated_alias);
    let(returned,born,freed)=measured(||unrelated_alias.return_alias_step(&unrelated,supplied).unwrap());assert!(returned.progress().fits(supplied));assert_eq!((born,freed),(returned.progress().retained_capacity_bytes,returned.progress().released_bytes));assert!(unrelated_alias.terminal_is_empty());
    let mut unrelated=semio_framework_async::CancelTokenRetirement::from_token(unrelated);while !unrelated.terminal_is_empty(){let(closed,born,freed)=measured(||unrelated.close_step(supplied).unwrap());assert!(closed.progress().fits(supplied));assert_eq!((born,freed),(closed.progress().retained_capacity_bytes,closed.progress().released_bytes));}
    for grant in [RetainedCloneGrant{maximum_items:0,..supplied},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..supplied},RetainedCloneGrant{maximum_depth:0,..supplied}] {
        let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,grant),cancel.clone(),||Some(0),&mut sequence,&mut receipt).unwrap();
        let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut job,&mut params,&mut cx));
        assert!(matches!(result,Ok(None)));assert_eq!((born,freed),(0,0));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());drop(cx);
        assert_eq!(job.as_ref().unwrap()as*const ClipboardIoJob,source_address);assert_eq!(params.as_ref().unwrap()as*const BatchJobParams,params_address);
        let Some(ClipboardIoOperation::Write(text))=job.as_ref().unwrap().operation.as_ref()else{panic!("denial retained original source")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);
    }
    let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,supplied),cancel.clone(),||Some(0),&mut sequence,&mut receipt).unwrap();
    let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut job,&mut params,&mut cx));
    let(mut session,admission)=result.unwrap().expect("original caller independently funds session admission");
    assert!(job.is_none()&&params.is_none());assert_eq!(admission,cx.retained_progress());assert!(admission.fits(supplied));assert_eq!(admission.retained_capacity_bytes,born);assert_eq!(admission.released_bytes,freed);assert_eq!(born,demand.capacity_bytes);assert_eq!(freed,0);drop(cx);
    cancel.cancel_now();let _=session.begin_close();
    for _ in 0..4096 {if session.terminal_is_empty(){break}let(step,born,freed)=measured(||session.close_step(supplied));assert!(step.progress().fits(supplied));assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.progress().released_bytes,freed);}
    assert!(session.terminal_is_empty());let(_,born,freed)=measured(||drop(session));assert_eq!((born,freed),(0,0));
    let mut cancelled_text=String::with_capacity(capacity);cancelled_text.push_str(&oracle);let cancelled_pointer=cancelled_text.as_ptr();
    let mut cancelled_job=Some(ClipboardIoJob::write(cancelled_text));
    let mut cancelled_params=Some(BatchJobParams{operation:OperationId(1),generation:Generation(1),cancel:cancel.clone(),config:BatchDriveConfig{retained:RetainedCloneGrant::default(),site:"clipboard_original_cancelled_admission",stage:InteractiveStage::InteractiveStep,fuel_per_step:100,step_budget_us:10},now_us:||Some(0)});
    let mut cancelled_receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(100,10,supplied),cancel.clone(),||Some(0),&mut sequence,&mut cancelled_receipt).unwrap();
    let(result,born,freed)=measured(||WorkerJobSession::try_admit_owned(&mut cancelled_job,&mut cancelled_params,&mut cx));
    assert!(matches!(result,Ok(None)));assert_eq!((born,freed),(0,0));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());drop(cx);
    let Some(ClipboardIoOperation::Write(text))=cancelled_job.as_ref().unwrap().operation.as_ref()else{panic!("original precancelled input")};assert_eq!(text.as_ptr(),cancelled_pointer);assert_eq!(text.capacity(),capacity);assert_eq!(text,&oracle);assert!(cancelled_params.is_some());
    let mut cancelled_job=cancelled_job.take().unwrap();cancelled_job.begin_close();assert_eq!(cancelled_job.close_step(supplied).progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert!(cancelled_job.terminal_is_empty());
    while !owner.terminal_is_empty(){assert!(owner.close_step(supplied).progress().fits(supplied))}
    eprintln!("[DEBUG] original clipboard session source/params pointer preserved by prebirth denial; admission uses caller context, ignores config policy, returns exact native heap receipt before exposing session");
}
