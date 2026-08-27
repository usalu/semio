//! ⏱️ Language-neutral deadline vectors and the actual driver entry boundary.

use super::*;

//#region 🧪️DriverProbe
thread_local! { static CLOCK: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(Some(1_000)) }; }

fn now() -> Option<u64> { CLOCK.with(std::cell::Cell::get) }

struct EntryProbe { entered: usize, closing: bool }

impl InteractiveJob for EntryProbe {
    fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome {
        self.entered += 1;
        StepOutcome::Yield
    }

    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, _: usize, _: usize) -> InteractiveJobCloseStep { InteractiveJobCloseStep::Complete }
    fn terminal_is_empty(&self) -> bool { self.closing }
}

//#endregion 🧪️DriverProbe

//#region 🚧️DeadlineAdmission
#[test]
fn microsecond_zero_expired_or_empty_fuel_never_enters_job() {
    for (fuel, deadline) in [(1, 1_000), (1, 999), (0, 1_500)] {
        let mut probe = EntryProbe { entered: 0, closing: false };
        let mut sequence = 0;
        let outcome = drive_step(&mut probe, "microsecond-entry", allocate_operation_id(), Generation(1), InteractiveStage::InteractiveStep, StepBudget::new(fuel, deadline), root_cancel_token(), now, &mut sequence, &mut None);
        probe.begin_close();
        assert_eq!(probe.close_step(1, 4_096), InteractiveJobCloseStep::Complete);
        assert!(probe.terminal_is_empty());
        assert!(matches!(outcome, StepOutcome::Yield));
        assert_eq!(probe.entered, 0, "expired or exhausted grant entered job: fuel={fuel}, deadline={deadline}");
    }
}

#[test]
fn microsecond_language_neutral_deadline_boundaries_and_overflow() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
    for law in fixture["cases"].as_array().unwrap() {
        let start = law["start"].as_str().unwrap().parse::<u64>().unwrap();
        let fuel = law["fuel"].as_u64().unwrap();
        let grant = law["grant"].as_u64().unwrap();
        let budget = StepBudget::from_duration(fuel, start, grant);
        assert_eq!(budget.map(|value| value.deadline_us.to_string()), law["deadline"].as_str().map(str::to_owned), "{}", law["id"]);
        for (index, sample) in law["samples"].as_array().unwrap().iter().enumerate() {
            CLOCK.with(|clock| clock.set(Some(sample.as_str().unwrap().parse().unwrap())));
            let mut sequence = 0;
            let (expired, yielded) = match budget {
                Some(budget) => {
                    let cx = StepContext::new(allocate_operation_id(), Generation(1), budget, root_cancel_token(), now, &mut sequence);
                    (cx.deadline_exceeded(), cx.should_yield())
                }
                None => (true, true),
            };
            assert_eq!(expired, law["expired"][index].as_bool().unwrap());
            assert_eq!(yielded, law["yielded"][index].as_bool().unwrap());
        }
        eprintln!("[DEBUG] microsecond fixture={} grant={grant} deadline={:?}", law["id"], budget.map(|budget| budget.deadline_us));
    }
}

//#endregion 🚧️DeadlineAdmission

//#region 🧵️RetainedWorker
fn close_authority(mut authority: WorkerJobAuthority<EntryProbe>) -> usize {
    let job = authority.job.as_mut().unwrap();
    job.begin_close();
    assert_eq!(job.close_step(1, 4_096), InteractiveJobCloseStep::Complete);
    assert!(job.terminal_is_empty());
    let entered = job.entered;
    if let Some(outcome) = authority.outcome.as_mut() {
        while !outcome.terminal_is_empty() { let _ = outcome.close_step(1, 4_096); }
    }
    if let Some(fault) = authority.preadmitted_fault.as_mut() {
        while !fault.terminal_is_empty() { let _ = fault.close_step(1, 4_096); }
    }
    entered
}

#[test]
fn microsecond_retained_worker_admits_half_ms_and_rejects_missing_or_overflow_clock() {
    for (clock, grant, fuel, expected_entries, faulted) in [(Some(1_000), 500, 1, 1, false), (Some(1_000), 0, 1, 0, false), (Some(1_000), 500, 0, 0, false), (None, 500, 1, 0, true), (Some(u64::MAX - 100), 500, 1, 0, true)] {
        CLOCK.with(|value| value.set(clock));
        let params = BatchJobParams { operation: allocate_operation_id(), generation: Generation(1), cancel: root_cancel_token(), config: BatchDriveConfig { site: "microsecond-worker", stage: InteractiveStage::InteractiveStep, fuel_per_step: fuel, step_budget_us: grant }, now_us: now };
        let mut authority = WorkerJobAuthority::try_new(EntryProbe { entered: 0, closing: false }, params).unwrap_or_else(|_| panic!("fixture payload admission"));
        let terminal = drive_worker_job_authority(&mut authority);
        let actual_fault = matches!(authority.outcome, Some(StepOutcome::Fault(_)));
        let entered = close_authority(authority);
        assert_eq!(terminal, faulted);
        assert_eq!(actual_fault, faulted);
        assert_eq!(entered, expected_entries);
        eprintln!("[DEBUG] microsecond worker clock={clock:?} grant={grant} fuel={fuel} entered={entered} fault={actual_fault}");
    }
}

#[test]
fn microsecond_platform_clock_and_real_half_ms_worker_progress() {
    let start = default_now_us().unwrap();
    let mut submillisecond_sample = false;
    for _ in 0..10_000 {
        let current = default_now_us().unwrap();
        assert!(current >= start);
        submillisecond_sample |= current % 1_000 != 0;
        if submillisecond_sample { break; }
    }
    assert!(submillisecond_sample, "platform clock lost microsecond precision");
    let params = BatchJobParams { operation: allocate_operation_id(), generation: Generation(1), cancel: root_cancel_token(), config: BatchDriveConfig { site: "microsecond-real-worker", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 }, now_us: default_now_us };
    let mut authority = WorkerJobAuthority::try_new(EntryProbe { entered: 0, closing: false }, params).unwrap_or_else(|_| panic!("fixture payload admission"));
    let terminal = drive_worker_job_authority(&mut authority);
    let entered = close_authority(authority);
    assert!(!terminal);
    assert_eq!(entered, 1);
    eprintln!("[DEBUG] actual monotonic clock retained 500us worker entered={entered}");
}

//#endregion 🧵️RetainedWorker

//#region 🔒️CallbackQuarantine
struct CompletionProbe { end: Option<u64>, closing: bool }

impl InteractiveJob for CompletionProbe {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, b"kept").unwrap_or_else(|_| panic!("exact fixture page admission"));
        CLOCK.with(|clock| clock.set(self.end));
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
    }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, _: usize, _: usize) -> InteractiveJobCloseStep { InteractiveJobCloseStep::Complete }
    fn terminal_is_empty(&self) -> bool { self.closing }
}

#[test]
fn microsecond_exact_callback_quarantine_retains_original_output_and_session_identity() {
    assert!(install_microsecond_clock(now).is_ok());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../⏱️trace/⏱️clock/🧪️contention.json")).unwrap();
    let operation = allocate_operation_id();
    let generation = Generation(71);
    for law in fixture["verdicts"].as_array().unwrap().iter().filter(|law| !law["start"].is_null()) {
        CLOCK.with(|clock| clock.set(law["start"].as_u64()));
        let params = BatchJobParams { operation, generation, cancel: root_cancel_token(), config: BatchDriveConfig { site: "microsecond-exact-quarantine", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 }, now_us: now };
        let session = WorkerJobSession::try_new(CompletionProbe { end: law["end"].as_u64(), closing: false }, params).unwrap_or_else(|_| panic!("exact worker admission"));
        assert!(matches!(session.try_step_on_caller(), Ok((_, WorkerJobPoll::Terminal))));
        let owner = session.take_terminal().unwrap_or_else(|_| panic!("exact terminal owner"));
        let verdict = owner.callback_verdict().expect("callback verdict is retained by the exact session");
        let faulted = verdict.is_fault();
        assert_eq!(faulted, law["fault"].as_bool().unwrap());
        assert_eq!(verdict.operation(), operation);
        assert_eq!(verdict.generation(), generation);
        if faulted {
            assert!(matches!(owner.outcome(), StepOutcome::Fault(_)));
            let Some(StepOutcome::Complete(candidate)) = owner.authority.as_ref().unwrap().quarantined_outcome.as_ref() else { panic!("quarantined original candidate must remain owned") };
            assert_eq!(candidate.output.single_page(), Some(b"kept".as_slice()));
        } else {
            assert!(matches!(owner.outcome(), StepOutcome::Complete(_)));
            assert!(owner.authority.as_ref().unwrap().quarantined_outcome.is_none());
        }
        let ledger = Arc::clone(&owner.authority.as_ref().unwrap().payload_ledger);
        owner.begin_close();
        let owned_bytes = ledger.bytes.load(Ordering::Acquire);
        let _ = session.close_step(0, 0);
        assert_eq!(ledger.bytes.load(Ordering::Acquire), owned_bytes);
        let mut released_bytes = 0;
        for _ in 0..64 {
            match session.close_step(1, JOB_PAYLOAD_PAGE_BYTES) {
                WorkerJobCloseStep::Pending { released_items, released_bytes: bytes } => { assert!(released_items <= 1 && bytes <= JOB_PAYLOAD_PAGE_BYTES); released_bytes += bytes; }
                WorkerJobCloseStep::Blocked => std::thread::yield_now(),
                WorkerJobCloseStep::Complete => break,
            }
        }
        assert!(session.terminal_is_empty());
        assert!(ledger.terminal_is_empty());
        assert_eq!(released_bytes, 4 + b"job-session.terminal-fault".len());
        eprintln!("[DEBUG] exact callback quarantine {} fault={faulted} original_bytes=4 retired_bytes={released_bytes} same_numeric_identity=true", law["id"]);
    }
}
//#endregion 🔒️CallbackQuarantine

//#region 🌐️PlatformClock
#[test]
fn microsecond_browser_clock_fraction_and_invalid_source_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️clock.json")).unwrap();
    for law in fixture["browser"].as_array().unwrap() {
        let milliseconds = law["milliseconds"].as_str().unwrap().parse::<f64>().unwrap();
        let expected = law["microseconds"].as_str().map(|value| value.parse::<u64>().unwrap());
        assert_eq!(microseconds_from_milliseconds(milliseconds), expected, "{milliseconds}");
    }
    assert_eq!(microseconds_from_milliseconds(f64::NAN), None);
    assert_eq!(microseconds_from_milliseconds(f64::INFINITY), None);
    let before = default_now_us().unwrap();
    let trace_sample = semio_framework_trace::now_us();
    let after = default_now_us().unwrap();
    assert!(before <= trace_sample && trace_sample <= after, "job and watchdog must share one monotonic epoch");
}
//#endregion 🌐️PlatformClock
