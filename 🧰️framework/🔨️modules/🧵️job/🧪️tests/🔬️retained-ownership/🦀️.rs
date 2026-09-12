
use super::*;
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

fn params(operation: OperationId, generation: Generation, cancel: CancelToken) -> BatchJobParams {
    BatchJobParams { operation, generation, cancel, config: BatchDriveConfig { site: "test.retained-job", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 1_000 }, now_us: default_now_us }
}

#[test]
fn retained_payload_physical_close_preserves_short_pages_until_the_exact_backing_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../📦️physical-close/🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["pageBytes"], JOB_PAYLOAD_PAGE_BYTES);
    let streams = [JobPayloadStream::CheckpointState, JobPayloadStream::Preview, JobPayloadStream::CommitState, JobPayloadStream::CommitOutput, JobPayloadStream::Fault];
    for stream in streams {
        for row in fixture["cases"].as_array().unwrap() {
            let operation = OperationId(90_010 + stream as u64);
            let generation = Generation(21);
            let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
            let mut sequence = 0;
            let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
            let bytes = vec![42; row["logicalBytes"].as_u64().unwrap() as usize];
            let mut writer = RetainedJobPayloadWriter::new(stream);
            let mut page = writer.admit_page(&mut context).unwrap();
            page.write(&bytes).unwrap();
            page.commit();
            let mut payload = writer.finish().unwrap();
            let pointer = payload.page(0).unwrap().as_ptr();
            let refused = payload.close_step(1, row["insufficientGrant"].as_u64().unwrap() as usize);
            let retained_pointer = payload.page(0).map(|page| page.as_ptr()) == Some(pointer);
            let released = payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
            let remaining_logical_bytes = payload.len();
            let remaining_pages = payload.page_count();
            while !payload.terminal_is_empty() { payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES); }
            let (refused_items, refused_bytes) = match refused { JobPayloadCloseStep::Pending { released_items, released_bytes } => (released_items, released_bytes), JobPayloadCloseStep::Complete => (0, 0) };
            let (released_items, released_bytes) = match released { JobPayloadCloseStep::Pending { released_items, released_bytes } => (released_items, released_bytes), JobPayloadCloseStep::Complete => (0, 0) };
            assert!(ledger.terminal_is_empty());
            let actual = serde_json::json!({
                "refusedItems": refused_items, "refusedBytes": refused_bytes, "retainedPointer": retained_pointer,
                "releasedItems": released_items, "releasedBytes": released_bytes,
                "remainingLogicalBytes": remaining_logical_bytes, "remainingPages": remaining_pages,
            });
            assert_eq!(actual, fixture["expected"], "{stream:?}/{}", row["name"]);
        }
    }
    eprintln!("[DEBUG] Job payload five-stream physical grants preserve exact short-page pointers and release each 16KiB backing exactly");
}

#[test]
fn retained_writer_physical_close_preserves_staged_and_rejected_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../📦️physical-close/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let operation = OperationId(90_020);
        let generation = Generation(21);
        let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
        let mut sequence = 0;
        let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::Preview);
        writer.begin_staged_page(&mut context).unwrap();
        writer.write_staged(&vec![42; row["logicalBytes"].as_u64().unwrap() as usize]).unwrap();
        assert!(matches!(writer.admit_page(&mut context), Err(JobPayloadAdmissionFault::OpportunityExhausted)));
        let staged_pointer = writer.staged.as_ref().unwrap().1.backing_identity();
        let rejected_pointer = writer.rejected.as_ref().unwrap().backing_identity();
        let mut observations = Vec::new();
        for (staged, pointer) in [(true, staged_pointer), (false, rejected_pointer)] {
            let refused = writer.close_step(1, row["insufficientGrant"].as_u64().unwrap() as usize);
            let retained_pointer = if staged {
                writer.staged.as_ref().map(|(_, source, _)| source.backing_identity()) == Some(pointer)
            } else {
                writer.rejected.as_ref().map(JobPayloadPageSource::backing_identity) == Some(pointer)
            };
            let released = writer.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
            observations.push((staged, refused, retained_pointer, released));
        }
        while !writer.terminal_is_empty() { writer.close_step(1, JOB_PAYLOAD_PAGE_BYTES); }
        assert!(ledger.terminal_is_empty());
        for (staged, refused, retained_pointer, released) in observations {
            assert_eq!(refused, JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 }, "staged={staged} {}", row["name"]);
            assert!(retained_pointer, "staged={staged} {}", row["name"]);
            assert_eq!(released, JobPayloadCloseStep::Pending { released_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES }, "staged={staged} {}", row["name"]);
        }
    }
    eprintln!("[DEBUG] Job writer staged and rejected pages retain their backing until an exact physical page grant");
}

fn wait_for(session: &WorkerJobSession<HostileJob>, expected: WorkerJobPoll) {
    for _ in 0..4_096 {
        if session.poll() == expected {
            return;
        }
        std::thread::yield_now();
    }
    panic!("worker session did not reach {expected:?}");
}

#[test]
fn payload_ledger_identity_must_match_the_exact_step_context() {
    let ledger = Arc::new(JobPayloadOperationLedger::new(OperationId(90_000), Generation(6)));
    let operation_mismatch = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut sequence = 0;
        let _ = StepContext::with_payload_ledger(OperationId(90_001), Generation(6), StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
    }));
    assert!(operation_mismatch.is_err());
    let generation_mismatch = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut sequence = 0;
        let _ = StepContext::with_payload_ledger(OperationId(90_000), Generation(7), StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
    }));
    assert!(generation_mismatch.is_err());
}

#[test]
fn retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned() {
    let operation = OperationId(90_001);
    let generation = Generation(7);
    let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
    let process_before = JOB_PAYLOAD_PROCESS_OWNED_BYTES.load(Ordering::Acquire);
    let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
    for index in 0..JOB_PAYLOAD_OPERATION_PAGES {
        let mut preview_sequence = index as u64;
        let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut preview_sequence, Arc::clone(&ledger));
        let source = JobPayloadPageSource::new();
        let mut page = context.admit_payload_page(&mut writer, source).expect("each fixed payload page is admitted before write");
        page.write(&[index as u8]).expect("one byte fits admitted page");
        page.commit();
    }
    let mut sequence = 0;
    let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
    let plus_one = JobPayloadPageSource::new();
    let plus_one_pointer = plus_one.backing_identity();
    let rejected = match context.admit_payload_page(&mut writer, plus_one) {
        Ok(_) => panic!("page maximum plus one must not receive an output grant"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.source().backing_identity(), plus_one_pointer);
    let returned = rejected.into_source();
    assert_eq!(returned.backing_identity(), plus_one_pointer);
    drop(returned);
    let mut payload = writer.finish().expect("full payload has no rejected source retained");
    assert_eq!(payload.page_count(), JOB_PAYLOAD_OPERATION_PAGES);
    assert_eq!(payload.close_step(0, 0), JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 });
    for _ in 0..JOB_PAYLOAD_OPERATION_PAGES {
        let _ = payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    assert!(payload.terminal_is_empty());
    assert!(ledger.terminal_is_empty());
    assert_eq!(JOB_PAYLOAD_PROCESS_OWNED_BYTES.load(Ordering::Acquire), process_before);
}

#[test]
fn retained_state_and_output_have_separate_credits_and_close_one_page_per_grant() {
    let operation = OperationId(90_002);
    let generation = Generation(8);
    let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
    let mut state_writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitState);
    let mut output_writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
    let mut sequence = 0;
    let mut state_context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
    let mut state_page = state_context.admit_payload_page(&mut state_writer, JobPayloadPageSource::new()).expect("state page");
    state_page.write(b"state").expect("state bytes");
    state_page.commit();
    let rejected = state_context.payload_from_bytes(JobPayloadStream::CommitOutput, b"output").expect_err("a second stream cannot bypass the one-page opportunity");
    assert_eq!(rejected.fault, JobPayloadAdmissionFault::OpportunityExhausted);
    drop(rejected.into_source());
    assert_eq!(state_writer.page_count(), 1);
    let mut output_context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
    let mut output_page = output_context.admit_payload_page(&mut output_writer, JobPayloadPageSource::new()).expect("separate output page");
    output_page.write(b"output").expect("output bytes");
    output_page.commit();
    let mut terminal = StepOutcome::Complete(CommitCandidate { state: state_writer.finish().expect("state"), output: output_writer.finish().expect("output") });
    assert_eq!(terminal.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { released_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES });
    assert!(!terminal.terminal_is_empty());
    assert_eq!(terminal.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { released_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES });
    assert!(terminal.terminal_is_empty());
}

#[test]
fn retained_writer_and_reader_advance_exactly_one_page_per_opportunity() {
    let operation = OperationId(90_008);
    let generation = Generation(14);
    let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
    let bytes = vec![19u8; JOB_PAYLOAD_PAGE_BYTES + 1];
    let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
    let mut cursor = 0;
    let mut sequence = 0;
    let mut zero = StepContext::with_payload_ledger(operation, generation, StepBudget::new(0, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
    assert_eq!(writer.write_slice_page(&mut zero, &bytes, &mut cursor), Ok(false));
    assert_eq!(cursor, 0);
    let mut first = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
    assert_eq!(writer.write_slice_page(&mut first, &bytes, &mut cursor), Ok(false));
    assert_eq!(cursor, JOB_PAYLOAD_PAGE_BYTES);
    let mut second = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence, Arc::clone(&ledger));
    assert_eq!(writer.write_slice_page(&mut second, &bytes, &mut cursor), Ok(true));
    let mut payload = writer.finish().expect("two-page retained payload");
    let mut reader = payload.reader();
    assert_eq!(reader.read_page(0, JOB_PAYLOAD_PAGE_BYTES), None);
    assert_eq!(reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES).map(|page| page.len()), Some(JOB_PAYLOAD_PAGE_BYTES));
    assert_eq!(reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES).map(|page| page.len()), Some(1));
    assert!(reader.terminal_is_empty());
    assert_eq!(payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { released_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES });
    assert_eq!(payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { released_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES });
    assert_eq!(payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Complete);
    assert!(payload.terminal_is_empty());
}

struct StructuredChildJob {
    backing: Option<Box<u8>>,
    closing: bool,
}

impl InteractiveJob for StructuredChildJob {
    fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome {
        StepOutcome::Yield
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if self.backing.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.backing = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.backing.is_none()
    }
}

#[test]
fn child_registry_max_plus_one_stale_duplicate_exhaustion_and_parent_completion_are_exact() {
    let scope = JobScope::for_operation(&root_cancel_token(), OperationId(90_003), Generation(9));
    let mut guards: [Option<ChildJobGuard<'_, StructuredChildJob>>; JOB_CHILD_SLOTS] =
        std::array::from_fn(|index| Some(scope.spawn_child(StructuredChildJob { backing: Some(Box::new(index as u8)), closing: false }).unwrap_or_else(|_| panic!("fixed child slot"))));
    let retained_child_pointer = guards[1].as_mut().expect("second structured child").with_child_mut(|child| child.backing.as_deref().expect("retained structured child backing") as *const u8).expect("generation-qualified child checkout");
    assert_eq!(unsafe { *retained_child_pointer }, 1);
    let plus_one_backing = Box::new(211u8);
    let plus_one_pointer = plus_one_backing.as_ref() as *const u8;
    let rejected = match scope.spawn_child(StructuredChildJob { backing: Some(plus_one_backing), closing: false }) {
        Ok(_) => panic!("child maximum plus one must be rejected"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.fault, JobChildAdmissionFault::Capacity);
    assert_eq!(rejected.child().backing.as_deref().expect("rejected structured child backing") as *const u8, plus_one_pointer);
    let mut rejected_child = rejected.into_child();
    assert_eq!(rejected_child.backing.as_deref().expect("returned structured child backing") as *const u8, plus_one_pointer);
    rejected_child.begin_close();
    assert_eq!(rejected_child.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    while !rejected_child.terminal_is_empty() {
        let _ = rejected_child.close_step(1, 0);
    }
    assert_eq!(scope.assert_completable(), Err(JobChildCompletionFault::LiveChildren));
    let token = guards[0].as_ref().expect("first child").token();
    guards[0].take().expect("first child").complete().expect("first exact completion");
    assert_eq!(scope.complete_child(token), Err(JobChildCompletionFault::Duplicate));
    let stale = JobChildToken { generation: token.generation + 1, ..token };
    assert_eq!(scope.complete_child(stale), Err(JobChildCompletionFault::Stale));
    drop(guards);
    assert_eq!(scope.pump_child_close(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(scope.pump_child_close(1, JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }, "child begin-close transfers control without claiming an owner release");
    while !scope.terminal_is_empty() {
        let _ = scope.pump_child_close(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    assert!(scope.assert_completable().is_ok());
    for slot in &scope.slots {
        slot.generation.store(u64::MAX, Ordering::Release);
        slot.state.store(CHILD_VACANT, Ordering::Release);
    }
    let rejected_backing = Box::new(7);
    let rejected_backing_pointer = rejected_backing.as_ref() as *const u8;
    let mut rejected = match scope.spawn_child(StructuredChildJob { backing: Some(rejected_backing), closing: false }) {
        Ok(_) => panic!("exhausted child generation must reject"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.fault, JobChildAdmissionFault::Exhausted);
    assert_eq!(rejected.child().backing.as_deref().expect("exhausted rejected backing") as *const u8, rejected_backing_pointer);
    rejected.begin_close();
    assert_eq!(rejected.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    while !rejected.terminal_is_empty() {
        let _ = rejected.close_step(1, 0);
    }
    scope.begin_close();
    assert!(scope.terminal_is_empty());
}

struct HostileJob {
    backing: Option<Box<u8>>,
    steps: Option<Arc<AtomicUsize>>,
    panic: bool,
    closing: bool,
}

impl InteractiveJob for HostileJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        let step = self.steps.as_ref().expect("hostile step counter").fetch_add(1, AtomicOrdering::AcqRel);
        if self.panic {
            panic!("hostile worker panic");
        }
        if step == 0 {
            return StepOutcome::Yield;
        }
        let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, &[**self.backing.as_ref().expect("hostile backing")]).expect("hostile output page");
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.backing.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.steps.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.backing.is_none() && self.steps.is_none()
    }
}

#[test]
fn worker_authority_keeps_one_heap_identity_through_mounted_submit_and_checkout() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mut mounted = MountedWorkerJobSession::try_new(
        HostileJob { backing: Some(Box::new(73)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false },
        params(OperationId(90_011), Generation(17), root_cancel_token()),
    )
    .unwrap_or_else(|_| panic!("heap authority fixture admission"));
    assert!(size_of::<WorkerJobAuthorityOwner<HostileJob>>() < size_of::<WorkerJobAuthority<HostileJob>>());
    let admitted_identity = unsafe { (&*mounted.session.inner.authority.get()).as_ref().expect("idle session owns its authority").0.as_ptr() };
    assert!(matches!(mounted.pump_one(&pool, Lane::Interactive), Ok(WorkerJobPoll::Submitted)));
    for _ in 0..4_096 {
        if mounted.poll() == WorkerJobPoll::Outcome {
            break;
        }
        std::thread::yield_now();
    }
    assert!(matches!(mounted.pump_one(&pool, Lane::Interactive), Ok(WorkerJobPoll::Outcome)));
    let checked_out_identity = mounted.checked_out.as_ref().and_then(|outcome| outcome.authority.as_ref()).expect("mounted outcome owns exact authority").0.as_ptr();
    assert_eq!(checked_out_identity, admitted_identity);
    assert!(matches!(mounted.take_checked_out_outcome(), Some(StepOutcome::Yield)));
    mounted.resume().expect("empty yielded outcome returns the same authority");
    mounted.begin_close();
    while !mounted.terminal_is_empty() {
        let _ = mounted.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    let _ = pool.shutdown();
    eprintln!("[DEBUG] worker authority retained one heap identity across mounted submit and checkout");
}

#[test]
fn worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let operation = OperationId(90_004);
    let generation = Generation(10);
    let steps = Arc::new(AtomicUsize::new(0));
    let session =
        WorkerJobSession::try_new(HostileJob { backing: Some(Box::new(91)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(operation, generation, root_cancel_token())).unwrap_or_else(|_| panic!("worker session slot"));
    let first = session.try_submit_step(&pool, Lane::Interactive).expect("first opportunity submitted");
    assert!(matches!(session.try_submit_step(&pool, Lane::Interactive), Err(WorkerJobSubmitFault::Contention(WorkerJobContention::Submitted(_)))));
    wait_for(&session, WorkerJobPoll::Outcome);
    let mut first_owner = session.take_outcome(first).expect("first exact outcome");
    assert!(matches!(first_owner.take_outcome(), StepOutcome::Yield));
    first_owner.resume().unwrap_or_else(|_| panic!("yield owner resumes exact generation"));
    let second = session.try_submit_step(&pool, Lane::Interactive).expect("second opportunity submitted");
    wait_for(&session, WorkerJobPoll::Terminal);
    let terminal = session.take_terminal().expect("terminal owner is take-only");
    let terminal_pointer = terminal.job().backing.as_deref().expect("terminal hostile backing") as *const u8;
    drop(terminal);
    let terminal = session.take_terminal().expect("dropped checkout hands exact terminal back");
    assert_eq!(terminal.job().backing.as_deref().expect("returned hostile backing") as *const u8, terminal_pointer);
    assert_eq!(second.generation, generation);
    terminal.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    assert_eq!(steps.load(AtomicOrdering::Acquire), 2);
    pool.shutdown();
}

#[test]
fn worker_pool_rejection_returns_exact_job_before_resume() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    pool.shutdown();
    let backing = Box::new(33u8);
    let backing_pointer = backing.as_ref() as *const u8;
    let session = WorkerJobSession::try_new(HostileJob { backing: Some(backing), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_005), Generation(11), root_cancel_token()))
        .unwrap_or_else(|_| panic!("worker session slot"));
    assert_eq!(session.try_submit_step(&pool, Lane::Interactive), Err(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
    let rejected = session.take_rejected().expect("pool rejection retained exact owner");
    assert_eq!(rejected.job().backing.as_deref().expect("rejected hostile backing") as *const u8, backing_pointer);
    rejected.resume();
    assert_eq!(session.poll(), WorkerJobPoll::Idle);
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
}

#[test]
fn worker_panic_and_quiet_wake_publish_one_durable_terminal_intent() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let session = WorkerJobSession::try_new(HostileJob { backing: Some(Box::new(1)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: true, closing: false }, params(OperationId(90_006), Generation(12), root_cancel_token()))
        .unwrap_or_else(|_| panic!("worker session slot"));
    let preadmitted_fault_pointer = unsafe {
        (&*session.inner.authority.get())
            .as_ref()
            .and_then(|authority| authority.preadmitted_fault.as_ref())
            .and_then(|payload| payload.pages[0].as_ref())
            .map(|page| page.source.backing_identity())
            .expect("panic fault backing is admitted before submission")
    };
    session.register_wake(Waker::noop()).expect("quiet wake registration");
    let _ = session.try_submit_step(&pool, Lane::Interactive).expect("panic opportunity submitted");
    wait_for(&session, WorkerJobPoll::Terminal);
    assert!(session.take_wake());
    assert!(!session.take_wake(), "redundant quiet poll raises no wake");
    let terminal = session.take_terminal().expect("panic becomes retained terminal");
    let StepOutcome::Fault(fault) = terminal.outcome() else { panic!("panic publishes the pre-admitted fault") };
    let returned_fault_pointer = fault.detail.pages[0].as_ref().map(|page| page.source.backing_identity()).expect("terminal fault retains exact backing");
    assert_eq!(returned_fault_pointer, preadmitted_fault_pointer);
    terminal.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    pool.shutdown();
}

#[test]
fn worker_quiet_wake_sequence_exhaustion_is_permanent_and_typed() {
    let session = WorkerJobSession::try_new(HostileJob { backing: Some(Box::new(2)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_009), Generation(15), root_cancel_token()))
        .unwrap_or_else(|_| panic!("worker session slot"));
    session.inner.wake_sequence.store(u64::MAX, Ordering::Release);
    session.inner.wake_pending.store(false, Ordering::Release);
    session.inner.raise_wake();
    assert_eq!(session.register_wake(Waker::noop()), Err(WorkerJobContention::WakeExhausted(Generation(15))));
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
}

#[test]
fn batch_session_advances_exactly_one_external_opportunity() {
    let steps = Arc::new(AtomicUsize::new(0));
    let mut batch = BatchJobSession::try_new(HostileJob { backing: Some(Box::new(7)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(OperationId(90_007), Generation(13), root_cancel_token()))
        .unwrap_or_else(|_| panic!("batch fault page is pre-admitted"));
    assert_eq!(batch.step(), Ok(WorkerJobPoll::Outcome));
    assert_eq!(steps.load(AtomicOrdering::Acquire), 1);
    assert!(matches!(batch.take_outcome(), Some(StepOutcome::Yield)));
    batch.resume().expect("caller explicitly resumes after first opportunity");
    assert_eq!(steps.load(AtomicOrdering::Acquire), 1, "batch adapter never drains itself to terminal");
    batch.begin_close();
    assert_eq!(batch.close_step(0, 0), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    while !batch.terminal_is_empty() {
        let _ = batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
}

#[test]
fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release() {
    let mut batch = BatchJobSession::try_new(HostileJob { backing: Some(Box::new(7)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_010), Generation(16), root_cancel_token()))
        .unwrap_or_else(|_| panic!("batch session authority"));
    assert_eq!(batch.step(), Ok(WorkerJobPoll::Outcome));
    assert!(batch.checkout_outcome());
    assert_eq!(batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    while !batch.terminal_is_empty() {
        let _ = batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
}

#[test]
fn worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned() {
    let mut sessions = Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
    for index in 0..WORKER_JOB_SESSION_SLOTS {
        let job = HostileJob { backing: Some(Box::new(index as u8)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false };
        sessions.push(WorkerJobSession::try_new(job, params(OperationId(91_000 + index as u64), Generation(index as u64 + 1), root_cancel_token())).unwrap_or_else(|_| panic!("each fixed session slot admits once")));
    }
    let rejected_backing = Box::new(211u8);
    let rejected_pointer = rejected_backing.as_ref() as *const u8;
    let mut rejected = match WorkerJobSession::try_new(HostileJob { backing: Some(rejected_backing), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(92_000), Generation(500), root_cancel_token())) {
        Ok(_) => panic!("session maximum plus one must retain exact rejected job"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.job().backing.as_deref().expect("session max plus one backing") as *const u8, rejected_pointer);
    assert_eq!(rejected.params.as_ref().expect("session max plus one parameters").operation, OperationId(92_000));
    assert_eq!(rejected.params.as_ref().expect("session max plus one parameters").generation, Generation(500));
    rejected.begin_close();
    assert_eq!(rejected.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    while !rejected.terminal_is_empty() {
        let _ = rejected.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    let dropped = sessions.pop().expect("last fixed session");
    drop(dropped);
    assert!(take_worker_job_retirement_wake());
    for _ in 0..8 {
        let _ = pump_worker_job_retirements(1, 1, JOB_PAYLOAD_PAGE_BYTES);
    }
    let replacement = WorkerJobSession::try_new(HostileJob { backing: Some(Box::new(17)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(92_001), Generation(501), root_cancel_token()))
        .unwrap_or_else(|_| panic!("retirement pump returns exact fixed session slot"));
    sessions.push(replacement);
    for session in sessions {
        let _ = session.begin_close();
        while !session.terminal_is_empty() {
            let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
    }
}
