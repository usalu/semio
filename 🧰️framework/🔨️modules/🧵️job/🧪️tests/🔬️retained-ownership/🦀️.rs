
use super::*;

pub(super) fn caller_worker_close_policy(phase:WorkerJobClosePhase)->RetainedCloneGrant{
    match phase{
        WorkerJobClosePhase::AuthorityRelease=>RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:0,maximum_release_bytes:65536,maximum_depth:64},
        _=>RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:16384,maximum_depth:64},
    }
}

use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

fn params(operation: OperationId, generation: Generation, cancel: CancelToken) -> BatchJobParams {
    BatchJobParams { operation, generation, cancel, config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "test.retained-job", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 1_000 }, now_us: default_now_us }
}

#[test]
fn retained_payload_physical_close_preserves_short_pages_until_the_exact_backing_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️physical-close/🔣️.json")).unwrap();
    assert_eq!(fixture["pageBytes"], JOB_PAYLOAD_PAGE_BYTES);
    let streams = [JobPayloadStream::CheckpointState, JobPayloadStream::Preview, JobPayloadStream::CommitState, JobPayloadStream::CommitOutput, JobPayloadStream::Fault];
    for stream in streams {
        for row in fixture["cases"].as_array().unwrap() {
            let operation = OperationId(90_010 + stream as u64);
            let generation = Generation(21);
            let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
            let mut sequence = 0;
            let mut actual_retained_progress=RetainedCloneProgress::default();let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
            let bytes = vec![42; row["logicalBytes"].as_u64().unwrap() as usize];
            let mut writer = RetainedJobPayloadWriter::new(stream);
            let mut page = writer.admit_page(&mut context).unwrap();
            page.write(&bytes).unwrap();
            page.commit();
            let mut payload = writer.finish().unwrap();
            let pointer = payload.page(0).unwrap().as_ptr();
            let insufficient_grant = row["insufficientGrant"].as_u64().unwrap() as usize;
            assert_eq!(payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..Default::default() }), "a zero-byte grant buys nothing");
            assert_eq!(payload.next_close_byte_demand(), JOB_PAYLOAD_PAGE_BYTES);
            assert_eq!(payload.close_step(RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..Default::default() }));
            for _ in 0..4 {
                assert_eq!(payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: insufficient_grant, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..Default::default() }));
                assert_eq!(payload.page(0).unwrap().as_ptr(), pointer);
                assert_eq!(payload.page(0).unwrap(), bytes);
                assert_eq!(ledger.process_share_bytes(), JOB_PAYLOAD_PAGE_BYTES);
            }
            let refused = payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: insufficient_grant, maximum_depth: 64 }).unwrap();
            let retained_pointer = payload.page(0).map(|page| page.as_ptr()) == Some(pointer);
            let released = payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: payload.next_close_byte_demand(), maximum_depth: 64 }).unwrap();
            assert_eq!(payload.next_close_byte_demand(), semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>());
            let remaining_logical_bytes = payload.len();
            let remaining_pages = payload.page_count();
            while !payload.terminal_is_empty() { payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(); }
            let (refused_items, refused_bytes) = match refused { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, .. }) => (released_items, released_bytes), RetainedCloneStep::Complete(_) => (0, 0) };
            let (released_items, released_bytes) = match released { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, .. }) => (released_items, released_bytes), RetainedCloneStep::Complete(_) => (0, 0) };
            assert!(ledger.terminal_is_empty());
            let actual = serde_json::json!({
                "refusedItems": refused_items, "refusedBytes": refused_bytes, "retainedPointer": retained_pointer,
                "releasedItems": released_items, "releasedBytes": released_bytes,
                "remainingLogicalBytes": remaining_logical_bytes, "remainingPages": remaining_pages,
                "chargedTotal": refused_bytes + released_bytes,
            });
            assert_eq!(actual, fixture["expected"], "{stream:?}/{}", row["name"]);
        }
    }
}

#[test]
fn retained_writer_physical_close_preserves_staged_and_rejected_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️physical-close/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let operation = OperationId(90_020);
        let generation = Generation(21);
        let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
        let mut sequence = 0;
        let mut actual_retained_progress=RetainedCloneProgress::default();let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::Preview);
        writer.begin_staged_page(&mut context).unwrap();
        writer.write_staged(&vec![42; row["logicalBytes"].as_u64().unwrap() as usize]).unwrap();
        assert!(matches!(writer.admit_page(&mut context), Err(JobPayloadAdmissionFault::OpportunityExhausted)));
        let staged_pointer = writer.staged.as_ref().unwrap().1.backing_identity();
        let rejected_pointer = writer.rejected.as_ref().unwrap().backing_identity();
        let mut observations = Vec::new();
        for (staged, pointer) in [(true, staged_pointer), (false, rejected_pointer)] {
            assert_eq!(writer.next_close_byte_demand(), JOB_PAYLOAD_PAGE_BYTES);
            for _ in 0..4 {
                assert_eq!(writer.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: row["insufficientGrant"].as_u64().unwrap() as usize, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..Default::default() }));
            }
            let refused = writer.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: row["insufficientGrant"].as_u64().unwrap() as usize, maximum_depth: 64 }).unwrap();
            let retained_pointer = if staged {
                writer.staged.as_ref().map(|(_, source, _)| source.backing_identity()) == Some(pointer)
            } else {
                writer.rejected.as_ref().map(JobPayloadPageSource::backing_identity) == Some(pointer)
            };
            let released = writer.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: writer.next_close_byte_demand(), maximum_depth: 64 }).unwrap();
            observations.push((staged, refused, retained_pointer, released));
        }
        while !writer.terminal_is_empty() { writer.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(); }
        assert!(ledger.terminal_is_empty());
        for (staged, refused, retained_pointer, released) in observations {
            assert_eq!(refused, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..Default::default() }), "staged={staged} {}", row["name"]);
            assert!(retained_pointer, "staged={staged} {}", row["name"]);
            assert_eq!(released, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }), "staged={staged} {}", row["name"]);
        }
    }
}

struct ShortGrantCloseJob {
    backing: Option<Box<u8>>,
    closing: bool,
}

impl InteractiveJob for ShortGrantCloseJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {JobOutcomeBorrow::admit_yield(cx)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        if maximum_items==0||maximum_bytes<self.next_close_release_byte_demand().unwrap()||grant.maximum_depth==0 {
            return InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}};
        }
        if self.backing.take().is_some() {
            return InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:1,released_bytes:size_of::<u8>(),..RetainedCloneProgress::default()}};
        }
        InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()}
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.backing.is_some(){size_of::<u8>()}else{0})}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.backing.is_some()))}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.backing.is_none()
    }
}

/// 🔭️ Logical work remains one item while each named phase declares its physical release extent.
#[test]
fn mounted_close_on_a_short_byte_grant_walks_every_named_phase_to_terminal() {
    let _slots = super::worker_session_slots_shared();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/♻️mounted-close/🔣️.json")).unwrap();
    let profile=fixture["profiles"].as_array().unwrap().iter().find(|profile|profile["wordBytes"].as_u64()==Some(size_of::<usize>()as u64)).expect("canonical physical layout profile for this native address space");
    let expected=profile["turns"].as_array().unwrap();
    let grant = 4_096;
    assert!(grant < JOB_PAYLOAD_PAGE_BYTES, "this law is about a grant shorter than one physical page");
    let mut mounted = admit_original_fixture_owner!(MountedWorkerJobSession,ShortGrantCloseJob { backing: Some(Box::new(51)), closing: false }, params(OperationId(90_030), Generation(23), root_cancel_token()))
        .unwrap_or_else(|_| panic!("short-grant close fixture admission"));
    assert_eq!(mounted.close_phase(), WorkerJobClosePhase::Open);
    mounted.begin_close();
    let mut ladder = Vec::new();
    let mut charged = 0;
    let mut turns = 0;
    let mut observed=Vec::new();
    for _ in 0..64 {
        let phase = mounted.close_phase();
        if ladder.last() != Some(&phase) {
            ladder.push(phase);
        }
        if mounted.terminal_is_empty() {
            break;
        }
        turns += 1;
        let caller=caller_worker_close_policy(phase);
        assert_eq!(caller.maximum_items,fixture["perTurnItems"].as_u64().unwrap()as usize);
        let demand=mounted.retirement_demands(caller.maximum_copy_bytes).expect("exclusive mounted fixture close query");
        assert!(demand.copy_bytes<=caller.maximum_copy_bytes && demand.capacity_bytes<=caller.maximum_capacity_bytes && demand.release_bytes<=caller.maximum_release_bytes && demand.depth<=caller.maximum_depth);
        if phase==WorkerJobClosePhase::PreadmittedFault && demand.release_bytes==JOB_PAYLOAD_PAGE_BYTES{let denied=RetainedCloneGrant{maximum_release_bytes:grant,..caller};let step=mounted.close_step(denied);assert_eq!(step.progress(),Default::default());assert_eq!(mounted.close_phase(),phase);}
        let release_grant=caller.maximum_release_bytes;
        let step=mounted.close_step(caller);
        let kind=if matches!(step,WorkerJobCloseStep::Complete{..}){"complete"}else{"pending"};
        let actual=serde_json::json!({"phase":format!("{phase:?}"),"next":format!("{:?}",mounted.close_phase()),"kind":kind,"receipt":step.progress()});
        eprintln!("[DEBUG] original mounted actual turn{turns} {actual}");
        observed.push(actual);
        assert!(step.progress().fits(caller));
        match step {
            WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:released_items,released_bytes,..}} => {
                assert!(released_items <= 1, "a one-item grant releases at most one owner: {released_items}");
                assert!(released_bytes <= release_grant, "a close turn never spends more than its explicit physical release grant: {released_bytes}");
                if matches!(phase,WorkerJobClosePhase::PreadmittedFault|WorkerJobClosePhase::Job){charged += released_bytes;}
            }
            WorkerJobCloseStep::Complete {progress} => {}
            WorkerJobCloseStep::Blocked => panic!("a mounted close on a positive grant is never blocked"),
            WorkerJobCloseStep::Refused{kind,..}=>panic!("original close grant refused: {kind:?}"),
        }
    }
    assert!(mounted.terminal_is_empty(), "every named phase completes under its exact queried physical release authority");
    assert_eq!(charged, JOB_PAYLOAD_PAGE_BYTES + size_of::<u8>(), "fault page and domain Box each spend their actual allocation extent");
    assert_eq!(observed,expected.clone(),"original exact funded phase/receipt vector; all per-turn and physical conservation checks retained");
    assert_eq!(turns, expected.len(), "canonical original phase and exact physical owner receipts each receive an admitted turn");
    assert_eq!(ladder.iter().map(|phase|format!("{phase:?}")).collect::<Vec<_>>(),fixture["phases"].as_array().unwrap().iter().map(|phase|phase.as_str().unwrap().to_string()).collect::<Vec<_>>(),"the canonical close cursor walks its named physical phases in order and never revisits one");
}

/// ⏳️ A liveness bound, not a performance one: the pool worker that owns the step may be
/// descheduled for a long time on a loaded machine (a hostile panic also unwinds and captures a
/// backtrace there), so the wait is bounded by wall time rather than by a count of yields.
const WORKER_LIVENESS_BOUND: std::time::Duration = std::time::Duration::from_secs(30);

fn wait_for(session: &WorkerJobSession<HostileJob>, expected: WorkerJobPoll) {
    let deadline = std::time::Instant::now() + WORKER_LIVENESS_BOUND;
    while std::time::Instant::now() < deadline {
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
        let mut actual_retained_progress=RetainedCloneProgress::default();let _ = StepContext::with_payload_ledger(OperationId(90_001), Generation(6), StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    }));
    assert!(operation_mismatch.is_err());
    let generation_mismatch = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut sequence = 0;
        let mut actual_retained_progress=RetainedCloneProgress::default();let _ = StepContext::with_payload_ledger(OperationId(90_000), Generation(7), StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    }));
    assert!(generation_mismatch.is_err());
}

#[test]
fn retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned() {
    let operation = OperationId(90_001);
    let generation = Generation(7);
    let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
    assert_eq!(ledger.process_share_bytes(), 0, "a fresh ledger holds none of the process budget");
    let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
    for index in 0..JOB_PAYLOAD_OPERATION_PAGES {
        let mut preview_sequence = index as u64;
        let mut actual_retained_progress=RetainedCloneProgress::default();let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut preview_sequence, Arc::clone(&ledger),&mut actual_retained_progress);
        let source = JobPayloadPageSource::new();
        let mut page = context.admit_payload_page(&mut writer, source).expect("each fixed payload page is admitted before write");
        page.write(&[index as u8]).expect("one byte fits admitted page");
        page.commit();
    }
    let mut sequence = 0;
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
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
    assert_eq!(payload.close_step(RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..Default::default() }));
    for _ in 0..JOB_PAYLOAD_OPERATION_PAGES {
        let _ = payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap();
    }
    let ledger_step = payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap();
    assert_eq!(ledger_step.progress().released_bytes, 0);
    assert!(payload.terminal_is_empty());
    assert!(ledger.terminal_is_empty());
    assert_eq!(ledger.process_share_bytes(), 0, "the exact close ladder returns every page this operation took from the process budget");
}

#[test]
fn retained_state_and_output_have_separate_credits_and_close_one_page_per_grant() {
    let operation = OperationId(90_002);
    let generation = Generation(8);
    let ledger = Arc::new(JobPayloadOperationLedger::new(operation, generation));
    let mut state_writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitState);
    let mut output_writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
    let mut sequence = 0;
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut state_context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    let mut state_page = state_context.admit_payload_page(&mut state_writer, JobPayloadPageSource::new()).expect("state page");
    state_page.write(b"state").expect("state bytes");
    state_page.commit();
    let rejected = state_context.payload_from_bytes(JobPayloadStream::CommitOutput, b"output").expect_err("a second stream cannot bypass the one-page opportunity");
    assert_eq!(rejected.fault, JobPayloadAdmissionFault::OpportunityExhausted);
    drop(rejected.into_source());
    assert_eq!(state_writer.page_count(), 1);
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut output_context = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    let mut output_page = output_context.admit_payload_page(&mut output_writer, JobPayloadPageSource::new()).expect("separate output page");
    output_page.write(b"output").expect("output bytes");
    output_page.commit();
    let mut terminal = StepOutcome::Complete(CommitCandidate { state: state_writer.finish().expect("state"), output: output_writer.finish().expect("output") });
    assert_eq!(terminal.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }));
    assert!(!terminal.terminal_is_empty());
    let ledger_step=terminal.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap();
    assert_eq!(ledger_step.progress().released_bytes, 0);
    assert!(!terminal.terminal_is_empty());
    assert_eq!(terminal.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }));
    let ledger_step=terminal.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap();
    assert_eq!(ledger_step.progress().released_bytes, 0);
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
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut zero = StepContext::with_payload_ledger(operation, generation, StepBudget::new(0, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    assert_eq!(writer.write_slice_page(&mut zero, &bytes, &mut cursor), Ok(false));
    assert_eq!(cursor, 0);
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut first = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    assert_eq!(writer.write_slice_page(&mut first, &bytes, &mut cursor), Ok(false));
    assert_eq!(cursor, JOB_PAYLOAD_PAGE_BYTES);
    let mut actual_retained_progress=RetainedCloneProgress::default();let mut second = StepContext::with_payload_ledger(operation, generation, StepBudget::new(1, u64::MAX,crate::component::TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, ClockStride::new(), &mut sequence, Arc::clone(&ledger),&mut actual_retained_progress);
    assert_eq!(writer.write_slice_page(&mut second, &bytes, &mut cursor), Ok(true));
    let mut payload = writer.finish().expect("two-page retained payload");
    let mut reader = payload.reader();
    assert_eq!(reader.read_page(0, JOB_PAYLOAD_PAGE_BYTES), None);
    assert_eq!(reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES).map(|page| page.len()), Some(JOB_PAYLOAD_PAGE_BYTES));
    assert_eq!(reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES).map(|page| page.len()), Some(1));
    assert!(reader.terminal_is_empty());
    assert_eq!(payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }));
    assert_eq!(payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JOB_PAYLOAD_PAGE_BYTES, ..Default::default() }));
    let ledger_step=payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap();
    assert_eq!(ledger_step.progress().released_bytes, 0);
    assert_eq!(payload.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(), RetainedCloneStep::Complete(Default::default()));
    assert!(payload.terminal_is_empty());
}

struct StructuredChildJob {
    backing: Option<Box<u8>>,
    closing: bool,
}

impl InteractiveJob for StructuredChildJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {JobOutcomeBorrow::admit_yield(cx)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;
        self.begin_close();
        if self.backing.is_some() {
            if maximum_items == 0 || grant.maximum_release_bytes < 1 || grant.maximum_depth == 0 {
                return InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}};
            }
            self.backing = None;
            return InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:1,released_bytes:1,..RetainedCloneProgress::default()}};
        }
        InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()}
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.backing.is_some()))}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}

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
    assert_eq!(rejected_child.close_step(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()}), InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !rejected_child.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = rejected_child.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:1,maximum_depth:64,..RetainedCloneGrant::default()});
    }
    assert_eq!(scope.assert_completable(), Err(JobChildCompletionFault::LiveChildren));
    let token = guards[0].as_ref().expect("first child").token();
    guards[0].take().expect("first child").complete().expect("first exact completion");
    assert_eq!(scope.complete_child(token), Err(JobChildCompletionFault::Duplicate));
    let stale = JobChildToken { generation: token.generation + 1, ..token };
    assert_eq!(scope.complete_child(stale), Err(JobChildCompletionFault::Stale));
    drop(guards);
    assert_eq!(scope.pump_child_close(RetainedCloneGrant{maximum_items:0,maximum_depth:64,..Default::default()}), InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    assert_eq!(scope.pump_child_close(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64}), InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}}, "child begin-close transfers control without claiming an owner release");
    let close_started=Instant::now();let mut close_attempts=0;
    while !scope.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = scope.pump_child_close(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64});
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
    assert_eq!(rejected.close_step(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()}), InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !rejected.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = rejected.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:1,maximum_depth:64,..RetainedCloneGrant::default()});
    }
    scope.begin_close();
    assert!(scope.terminal_is_empty());
}

struct HostileJob {
    output: Option<JobPayloadSlot>,
    backing: Option<Box<u8>>,
    steps: Option<Arc<AtomicUsize>>,
    panic: bool,
    closing: bool,
}

impl InteractiveJob for HostileJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {
        let step = self.steps.as_ref().expect("hostile step counter").fetch_add(1, AtomicOrdering::AcqRel);
        if self.panic {
            panic!("hostile worker panic");
        }
        if step == 0 {
            return JobOutcomeBorrow::admit_yield(cx);
        }
        let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, &[**self.backing.as_ref().expect("hostile backing")]).expect("hostile output page");
        self.output=Some(JobPayloadSlot::from_payload(output));
        JobOutcomeBorrow::admit_complete(cx,None,self.output.as_ref().and_then(JobPayloadSlot::original))
    }

    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::Complete=>descriptor.complete(None,self.output.as_ref().and_then(JobPayloadSlot::original)),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"hostile fixture requires original yielded or terminal descriptor"))}}
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep {
        self.begin_close();
        if let Some(output)=self.output.as_mut(){if !output.is_empty(){return match output.close_step(grant){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};}}
        let copy=self.next_close_copy_byte_demand().expect("declared hostile copy");
        let release=self.next_close_release_byte_demand().expect("declared hostile release");
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release||grant.maximum_depth<1 {
            return InteractiveJobCloseStep::Pending{progress:Default::default()};
        }
        if let Some(backing)=self.backing.take(){
            drop(backing);
            return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Box<u8>>(),released_bytes:size_of::<u8>(),..Default::default()}};
        }
        if let Some(steps)=self.steps.take(){
            let released_bytes=if let Some(counter)=Arc::into_inner(steps){drop(counter);semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>()}else{0};
            return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Arc<AtomicUsize>>(),released_bytes,..Default::default()}};
        }
        InteractiveJobCloseStep::Complete{progress:Default::default()}
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.copy_bytes);}}Ok(if self.backing.is_some(){size_of::<Box<u8>>()}else if self.steps.is_some(){size_of::<Arc<AtomicUsize>>()}else{0})}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.release_bytes);}}Ok(if self.backing.is_some(){size_of::<u8>()}else if self.steps.is_some(){semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>()}else{0})}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.depth);}}Ok(usize::from(self.backing.is_some()||self.steps.is_some()))}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.backing.is_none() && self.steps.is_none() && self.output.as_ref().is_none_or(JobPayloadSlot::is_empty)
    }
}

/// 🔁️ [`admitted`] for a mounted session, whose own pump takes a rejected opportunity back and
/// resumes it (`WorkerJobPoll::Rejected`) on the pump after a contended submit.
fn mounted_admitted<J: InteractiveJob + Send + 'static>(mounted: &mut MountedWorkerJobSession<J>, pool: &WorkerPool, lane: Lane) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault> {
    loop {
        match mounted.pump_one(pool, lane,crate::component::TEST_RETAINED_POLICY) {
            Err(MountedWorkerJobPumpFault::Submit(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Contended))) | Ok(WorkerJobPoll::Rejected) => std::thread::yield_now(),
            answer => return answer,
        }
    }
}

#[test]
fn worker_authority_keeps_one_heap_identity_through_mounted_submit_and_checkout() {
    let _slots = super::worker_session_slots_shared();
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mut mounted = admit_original_fixture_owner!(MountedWorkerJobSession,
        HostileJob { output:None, backing: Some(Box::new(73)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false },
        params(OperationId(90_011), Generation(17), root_cancel_token()),
    )
    .unwrap_or_else(|_| panic!("heap authority fixture admission"));
    assert!(size_of::<WorkerJobAuthorityOwner<HostileJob>>() < size_of::<WorkerJobAuthority<HostileJob>>());
    let admitted_identity = unsafe { (&*mounted.session.inner.authority.get()).as_ref().expect("idle session owns its authority").0.as_ptr() };
    assert!(matches!(mounted_admitted(&mut mounted, &pool, Lane::Background), Ok(WorkerJobPoll::Submitted)), "a non-interactive lane submits to the pool");
    let deadline = std::time::Instant::now() + WORKER_LIVENESS_BOUND;
    while mounted.poll() != WorkerJobPoll::Outcome && std::time::Instant::now() < deadline {
        std::thread::yield_now();
    }
    assert!(matches!(mounted.pump_one(&pool, Lane::Background,crate::component::TEST_RETAINED_POLICY), Ok(WorkerJobPoll::Outcome)));
    let checked_out_identity = mounted.checked_out.as_ref().and_then(|outcome| outcome.authority.as_ref()).expect("mounted outcome owns exact authority").0.as_ptr();
    assert_eq!(checked_out_identity, admitted_identity);
    let original_receipt=mounted.take_checked_out_retained_step_receipt().expect("original mounted physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(mounted.checked_out_outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    for _ in 0..2 { assert!(mounted.acknowledge_checked_out_outcome(crate::component::TEST_RETAINED_POLICY).progress().fits(crate::component::TEST_RETAINED_POLICY)); }
    mounted.resume().expect("empty yielded outcome returns the same authority");
    assert!(matches!(mounted.pump_one(&pool, Lane::Interactive,crate::component::TEST_RETAINED_POLICY), Ok(WorkerJobPoll::Terminal)), "the interactive lane runs the step on the caller and checks its terminal out in one pump");
    let receipt=mounted.take_checked_out_retained_step_receipt().expect("original mounted terminal turn receipt");assert!(receipt.1.fits(receipt.0));assert!(mounted.take_checked_out_retained_step_receipt().is_none());
    let terminal_identity = mounted.checked_out.as_ref().and_then(|outcome| outcome.authority.as_ref()).expect("mounted terminal owns exact authority").0.as_ptr();
    assert_eq!(terminal_identity, admitted_identity, "the caller-run interactive step keeps the same heap authority as the pooled one");
    mounted.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !mounted.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = mounted.close_step(caller_worker_close_policy(mounted.close_phase()));
    }
    let _ = pool.shutdown();
}

/// 🔁️ `try_submit` never waits for queue ownership: a lane queue the pool's own worker holds for
/// a moment answers `Contended`, the session retains the exact rejected opportunity, and its owner
/// takes it back and resumes before retrying. Tests that need an admitted opportunity follow exactly
/// that protocol for exactly that answer, and consume the wake their own resume raised, so a test's
/// wake assertions see only the transitions it is asserting.
fn admitted<J: InteractiveJob + Send + 'static>(session: &WorkerJobSession<J>, pool: &WorkerPool, lane: Lane) -> Result<WorkerJobTicket, WorkerJobSubmitFault> {
    loop {
        match session.try_submit_step(pool, lane,crate::component::TEST_RETAINED_POLICY) {
            Err(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Contended)) => {
                session.take_rejected().unwrap_or_else(|_| panic!("a contended opportunity is retained for its owner")).resume();
                let _ = session.take_wake();
                std::thread::yield_now();
            }
            answer => return answer,
        }
    }
}

#[test]
fn worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact() {
    let _slots = super::worker_session_slots_shared();
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let operation = OperationId(90_004);
    let generation = Generation(10);
    let steps = Arc::new(AtomicUsize::new(0));
    let mut session =
        admit_original_fixture_owner!(WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(91)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(operation, generation, root_cancel_token())).unwrap_or_else(|_| panic!("worker session slot"));
    let first = admitted(&session, &pool, Lane::Interactive).expect("first opportunity submitted");
    assert!(matches!(session.try_submit_step(&pool, Lane::Interactive,crate::component::TEST_RETAINED_POLICY), Err(WorkerJobSubmitFault::Contention(WorkerJobContention::Submitted(_)))));
    wait_for(&session, WorkerJobPoll::Outcome);
    let mut first_owner = session.take_outcome(first).expect("first exact outcome");
    let original_receipt=first_owner.take_retained_step_receipt().expect("original pool physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(first_owner.outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    for _ in 0..2 { assert!(first_owner.acknowledge_outcome(crate::component::TEST_RETAINED_POLICY).progress().fits(crate::component::TEST_RETAINED_POLICY)); }
    first_owner.resume().unwrap_or_else(|_| panic!("yield owner resumes exact generation"));
    let second = admitted(&session, &pool, Lane::Interactive).expect("second opportunity submitted");
    wait_for(&session, WorkerJobPoll::Terminal);
    let mut terminal = session.take_terminal().expect("terminal owner is take-only");
    let receipt=terminal.take_retained_step_receipt().expect("original terminal turn receipt");assert!(receipt.1.fits(receipt.0));assert!(terminal.take_retained_step_receipt().is_none());
    let terminal_pointer = terminal.job().backing.as_deref().expect("terminal hostile backing") as *const u8;
    drop(terminal);
    let terminal = session.take_terminal().expect("dropped checkout hands exact terminal back");
    assert_eq!(terminal.job().backing.as_deref().expect("returned hostile backing") as *const u8, terminal_pointer);
    assert_eq!(second.generation, generation);
    terminal.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = session.close_step(caller_worker_close_policy(session.close_phase()));
    }
    assert_eq!(steps.load(AtomicOrdering::Acquire), 2);
    pool.shutdown();
}

#[test]
fn worker_pool_rejection_returns_exact_job_before_resume() {
    let _slots = super::worker_session_slots_shared();
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    pool.shutdown();
    let backing = Box::new(33u8);
    let backing_pointer = backing.as_ref() as *const u8;
    let mut session = admit_original_fixture_owner!(WorkerJobSession,HostileJob { output:None, backing: Some(backing), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_005), Generation(11), root_cancel_token()))
        .unwrap_or_else(|_| panic!("worker session slot"));
    assert_eq!(session.try_submit_step(&pool, Lane::Interactive,crate::component::TEST_RETAINED_POLICY), Err(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
    let rejected = session.take_rejected().expect("pool rejection retained exact owner");
    assert_eq!(rejected.job().backing.as_deref().expect("rejected hostile backing") as *const u8, backing_pointer);
    rejected.resume();
    assert_eq!(session.poll(), WorkerJobPoll::Idle);
    session.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = session.close_step(caller_worker_close_policy(session.close_phase()));
    }
}

#[test]
fn worker_panic_and_quiet_wake_publish_one_durable_terminal_intent() {
    let _slots = super::worker_session_slots_shared();
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mut session = admit_original_fixture_owner!(WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(1)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: true, closing: false }, params(OperationId(90_006), Generation(12), root_cancel_token()))
        .unwrap_or_else(|_| panic!("worker session slot"));
    let preadmitted_fault_pointer = unsafe {
        (&*session.inner.authority.get())
            .as_ref()
            .and_then(|authority| authority.preadmitted_fault.original())
            .and_then(|payload| payload.pages[0].as_ref())
            .map(|page| page.source.backing_identity())
            .expect("panic fault backing is admitted before submission")
    };
    session.register_wake(Waker::noop()).expect("quiet wake registration");
    let _ = admitted(&session, &pool, Lane::Interactive).expect("panic opportunity submitted");
    let deadline = std::time::Instant::now() + WORKER_LIVENESS_BOUND;
    while !session.take_wake() {
        assert!(std::time::Instant::now() < deadline, "the worker panic never raised its durable wake intent");
        std::thread::yield_now();
    }
    assert_eq!(session.poll(), WorkerJobPoll::Terminal, "a raised wake intent is published after the terminal it announces");
    assert!(!session.take_wake(), "redundant quiet poll raises no wake");
    let mut terminal = session.take_terminal().expect("panic becomes retained terminal");let original_receipt=terminal.take_retained_step_receipt().expect("original panic physical receipt");assert!(original_receipt.1.fits(original_receipt.0));
    let Some(JobOutcomeView::Fault{detail:fault,..}) = terminal.outcome().unwrap() else { panic!("panic publishes the pre-admitted fault") };
    let returned_fault_pointer = fault.pages[0].as_ref().map(|page| page.source.backing_identity()).expect("terminal fault retains exact backing");
    assert_eq!(returned_fault_pointer, preadmitted_fault_pointer);
    terminal.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = session.close_step(caller_worker_close_policy(session.close_phase()));
    }
    pool.shutdown();
}

#[test]
fn worker_quiet_wake_sequence_exhaustion_is_permanent_and_typed() {
    let _slots = super::worker_session_slots_shared();
    let mut session = admit_original_fixture_owner!(WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(2)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_009), Generation(15), root_cancel_token()))
        .unwrap_or_else(|_| panic!("worker session slot"));
    session.inner.wake_sequence.store(u64::MAX, Ordering::Release);
    session.inner.wake_pending.store(false, Ordering::Release);
    session.inner.raise_wake();
    assert_eq!(session.register_wake(Waker::noop()), Err(WorkerJobContention::WakeExhausted(Generation(15))));
    session.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = session.close_step(caller_worker_close_policy(session.close_phase()));
    }
}

#[test]
fn batch_session_advances_exactly_one_external_opportunity() {
    let steps = Arc::new(AtomicUsize::new(0));
    let mut batch = admit_original_fixture_owner!(BatchJobSession,HostileJob { output:None, backing: Some(Box::new(7)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(OperationId(90_007), Generation(13), root_cancel_token()))
        .unwrap_or_else(|_| panic!("batch fault page is pre-admitted"));
    assert_eq!(batch.step(crate::component::TEST_RETAINED_POLICY), Ok(WorkerJobPoll::Outcome));
    assert_eq!(steps.load(AtomicOrdering::Acquire), 1);
    assert!(batch.checkout_outcome());
    let original_receipt=batch.take_checked_out_retained_step_receipt().expect("original batch physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(batch.checked_out_outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    for _ in 0..2 { assert!(batch.acknowledge_outcome(crate::component::TEST_RETAINED_POLICY).progress().fits(crate::component::TEST_RETAINED_POLICY)); }
    batch.resume().expect("caller explicitly resumes after first opportunity");
    assert_eq!(steps.load(AtomicOrdering::Acquire), 1, "batch adapter never drains itself to terminal");
    batch.begin_close();
    assert_eq!(batch.close_step(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()}), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !batch.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = batch.close_step(caller_worker_close_policy(batch.close_phase()));
    }
}

#[test]
fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release() {
    let mut batch = admit_original_fixture_owner!(BatchJobSession,HostileJob { output:None, backing: Some(Box::new(7)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_010), Generation(16), root_cancel_token()))
        .unwrap_or_else(|_| panic!("batch session authority"));
    assert_eq!(batch.step(crate::component::TEST_RETAINED_POLICY), Ok(WorkerJobPoll::Outcome));
    assert!(batch.checkout_outcome());
    assert_eq!(batch.close_step(caller_worker_close_policy(batch.close_phase())), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let receipt=batch.session.take_retained_step_receipt().unwrap().expect("original checked out receipt follows same-owner close handback");assert!(receipt.1.fits(receipt.0));assert!(batch.session.take_retained_step_receipt().unwrap().is_none());
    let header=batch.close_step(caller_worker_close_policy(batch.close_phase()));
    assert_eq!(header,WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}});
    assert_eq!(batch.close_phase(),WorkerJobClosePhase::Outcome);
    let presence=batch.close_step(caller_worker_close_policy(batch.close_phase()));
    assert_eq!(presence,WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}});
    assert_eq!(batch.close_phase(),WorkerJobClosePhase::BeginClose);
    assert_eq!(batch.close_step(caller_worker_close_policy(batch.close_phase())), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !batch.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = batch.close_step(caller_worker_close_policy(batch.close_phase()));
    }
}

#[test]
fn worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned() {
    let _slots = super::worker_session_slots_exclusive();
    for _ in 0..WORKER_JOB_SESSION_SLOTS * 64 {
        if !worker_job_retirements_are_parked() {
            break;
        }
        let _ = pump_worker_job_retirements(WORKER_JOB_SESSION_SLOTS,caller_worker_close_policy(worker_job_retirement_phase(WORKER_JOB_SESSION_SLOTS).expect("parked original owner")));
    }
    assert!(!worker_job_retirements_are_parked(), "earlier tests' dropped sessions retire before this test owns every slot");
    let mut sessions = Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
    for index in 0..WORKER_JOB_SESSION_SLOTS {
        let job = HostileJob { output:None, backing: Some(Box::new(index as u8)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false };
        sessions.push(admit_original_fixture_owner!(WorkerJobSession,job, params(OperationId(91_000 + index as u64), Generation(index as u64 + 1), root_cancel_token())).unwrap_or_else(|_| panic!("each fixed session slot admits once")));
    }
    let rejected_backing = Box::new(211u8);
    let rejected_pointer = rejected_backing.as_ref() as *const u8;
    let mut rejected_job=Some(HostileJob{output:None,backing:Some(rejected_backing),steps:Some(Arc::new(AtomicUsize::new(0))),panic:false,closing:false});
    let mut rejected_params=Some(params(OperationId(92_000),Generation(500),root_cancel_token()));
    let original_grant=rejected_params.as_ref().unwrap().config.retained;
    let mut recipient=RetainedCloneProgress::default();
    let mut control=WorkerJobAdmissionContext::new(OperationId(92_000),Generation(500),StepBudget::new(1,u64::MAX,original_grant),rejected_params.as_ref().unwrap().now_us,&mut recipient).unwrap();
    assert!(WorkerJobSession::try_admit_owned(&mut rejected_job,&mut rejected_params,&mut control).unwrap().is_none());
    drop(control);
    assert_eq!(recipient,RetainedCloneProgress::default());
    assert_eq!(rejected_job.as_ref().unwrap().backing.as_deref().unwrap()as*const u8,rejected_pointer);
    assert_eq!(rejected_params.as_ref().unwrap().operation,OperationId(92_000));
    assert_eq!(rejected_params.as_ref().unwrap().generation,Generation(500));
    let dropped = sessions.pop().expect("last fixed session");
    drop(dropped);
    assert!(take_worker_job_retirement_wake());
    for _ in 0..8 {
        let _ = pump_worker_job_retirements(1,caller_worker_close_policy(worker_job_retirement_phase(1).expect("parked original owner")));
    }
    let close_started=Instant::now();let mut close_attempts=0;
    while worker_job_retirements_are_parked(){
        close_attempts+=1;assert!(close_attempts<=65536&&close_started.elapsed()<WORKER_LIVENESS_BOUND,"parked original owner retained beyond finite caller control");
        let phase=worker_job_retirement_phase(1).expect("parked original owner phase");
        let step=pump_worker_job_retirements(1,caller_worker_close_policy(phase));
        assert!(!matches!(step,WorkerJobCloseStep::Refused{..}),"parked original owner refused caller policy: {step:?}");
    }
    let mut control=WorkerJobAdmissionContext::new(OperationId(92_000),Generation(500),StepBudget::new(1,u64::MAX,original_grant),rejected_params.as_ref().unwrap().now_us,&mut recipient).unwrap();
    let(replacement,progress)=WorkerJobSession::try_admit_owned(&mut rejected_job,&mut rejected_params,&mut control).unwrap().expect("same original rejected owners enter returned slot");
    drop(control);
    assert_eq!(recipient,progress);
    assert!(rejected_job.is_none()&&rejected_params.is_none());
    sessions.push(replacement);
    for mut session in sessions {
        let _ = session.begin_close();
        let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
            let _ = session.close_step(caller_worker_close_policy(session.close_phase()));
        }
    }
}

#[test]
fn hostile_retained_box_and_arc_require_complete_independent_physical_grants(){
    let mut job=HostileJob{output:None,backing:Some(Box::new(9)),steps:Some(Arc::new(AtomicUsize::new(0))),panic:false,closing:false};
    let pointer=job.backing.as_deref().unwrap() as *const u8;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:64,maximum_depth:64};
    let denied=RetainedCloneGrant{maximum_release_bytes:0,..grant};
    assert_eq!(job.close_step(denied).progress(),RetainedCloneProgress::default());
    assert_eq!(job.backing.as_deref().unwrap() as *const u8,pointer);
    let box_step=job.close_step(grant);assert!(box_step.progress().fits(grant));
    assert_eq!(box_step.progress().released_bytes,1);
    let arc_step=job.close_step(grant);assert!(arc_step.progress().fits(grant));
    assert_eq!(arc_step.progress().released_bytes,semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>());
    assert!(matches!(job.close_step(grant),InteractiveJobCloseStep::Complete{..}));assert!(job.terminal_is_empty());
    let shared=Arc::new(AtomicUsize::new(17));
    let mut job=HostileJob{output:None,backing:None,steps:Some(Arc::clone(&shared)),panic:false,closing:false};
    let step=job.close_step(grant);assert_eq!(step.progress().released_bytes,0);assert_eq!(shared.load(AtomicOrdering::Acquire),17);
    assert!(matches!(job.close_step(grant),InteractiveJobCloseStep::Complete{..}));assert!(job.terminal_is_empty());
}

#[test]
fn mounted_original_cancel_alias_returns_only_to_its_still_borrowed_matching_root(){
 let _slots=super::worker_session_slots_shared();let root=root_cancel_token();let mut mounted=admit_original_fixture_owner!(MountedWorkerJobSession,ShortGrantCloseJob{backing:Some(Box::new(51)),closing:false},params(OperationId(90_031),Generation(23),root.clone())).unwrap_or_else(|_|panic!("original witnessed alias fixture admission"));mounted.begin_close();let mut handbacks=0;
 for _ in 0..64{let phase=mounted.close_phase();if phase==WorkerJobClosePhase::Empty{break;}let grant=caller_worker_close_policy(phase);if let Some(step)=mounted.return_original_cancel_alias_step(&root,grant).unwrap(){let observed=step.progress();assert!(observed.fits(grant));assert_eq!(observed,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<CancelToken>(),..Default::default()});assert!(matches!(step,RetainedCloneStep::Complete(_)));handbacks+=1;}else{let step=mounted.close_step(grant);assert!(step.progress().fits(grant));assert!(!matches!(step,WorkerJobCloseStep::Blocked|WorkerJobCloseStep::Refused{..}));}}
 assert!(mounted.terminal_is_empty());assert_eq!(handbacks,1);let mut original=semio_framework_async::CancelTokenRetirement::from_token(root);let grant=crate::component::TEST_RETAINED_POLICY;let mut released=0;for _ in 0..4{if original.terminal_is_empty(){break;}released+=original.close_step(grant).unwrap().progress().released_bytes;}assert!(original.terminal_is_empty());assert!(released>0);eprintln!("[DEBUG] actual mounted original cancellation alias returned once to borrowed caller root; mounted fullowner terminal before separate root Arc release");
}
