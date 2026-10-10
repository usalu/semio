use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static FAKE_NOW_US: AtomicU64 = AtomicU64::new(0);
static FAKE_READS: AtomicU64 = AtomicU64::new(0);
static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn fake_now_us() -> Option<u64> {
    FAKE_READS.fetch_add(1, Ordering::SeqCst);
    Some(FAKE_NOW_US.load(Ordering::SeqCst))
}

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪜️clock-stride-law.json")).expect("clock-stride law fixture")
}

/// 🏃️ Runs units of `unit_cost_us` in one step that began at `start_us` until the step's clock
/// reports the deadline; answers (real reads, overshoot past the deadline in µs).
fn run_window(stride: &mut ClockStride, start_us: u64, window_us: u64, unit_cost_us: u64) -> (u64, u64) {
    FAKE_NOW_US.store(start_us, Ordering::SeqCst);
    FAKE_READS.store(0, Ordering::SeqCst);
    let deadline_us = start_us + window_us;
    stride.begin(Some(start_us));
    loop {
        FAKE_NOW_US.fetch_add(unit_cost_us, Ordering::SeqCst);
        if stride.read(fake_now_us).is_none_or(|now_us| now_us >= deadline_us) {
            return (FAKE_READS.load(Ordering::SeqCst), FAKE_NOW_US.load(Ordering::SeqCst) - deadline_us);
        }
    }
}

#[test]
fn a_clock_stride_reads_about_once_per_interval_and_bounds_its_overshoot() {
    let _serial = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let law = law();
    assert_eq!(ClockStride::TARGET_READ_INTERVAL_US, law["targetReadIntervalUs"].as_u64().unwrap());
    assert_eq!(u64::from(ClockStride::MAXIMUM_STRIDE), law["maximumStride"].as_u64().unwrap());
    for case in law["cases"].as_array().unwrap() {
        let mut stride = ClockStride::new();
        let (reads, overshoot) = run_window(&mut stride, 1_000_000, case["windowUs"].as_u64().unwrap(), case["unitCostUs"].as_u64().unwrap());
        assert!(reads <= case["maxReads"].as_u64().unwrap(), "{}: {reads} reads", case["name"]);
        assert!(overshoot <= case["maxOvershootUs"].as_u64().unwrap(), "{}: {overshoot} µs past the deadline", case["name"]);
    }
}

#[test]
fn a_new_step_starts_at_its_drivers_reading_and_keeps_the_calibrated_stride() {
    let _serial = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let law = law();
    let mut stride = ClockStride::new();
    let mut start_us = 5_000_000;
    let _ = run_window(&mut stride, start_us, 2_000, 2);
    for _ in 0..law["newStep"]["windows"].as_u64().unwrap() {
        start_us += 1_000_000;
        let before = stride.stride();
        FAKE_NOW_US.store(start_us + 7, Ordering::SeqCst);
        FAKE_READS.store(0, Ordering::SeqCst);
        stride.begin(Some(start_us));
        assert_eq!(stride.latest_us(), Some(start_us), "the step's first reading is its driver's entry reading");
        assert_eq!(stride.stride(), before, "a new step keeps the calibrated stride");
        if before > 1 {
            assert_eq!(stride.read(fake_now_us), Some(start_us));
            assert_eq!(FAKE_READS.load(Ordering::SeqCst), law["newStep"]["readsForTheFirstCall"].as_u64().unwrap(), "the first in-step call costs no read of its own");
        }
        let _ = run_window(&mut stride, start_us, 2_000, 2);
    }
}

/// 🧮️ The whole clock cost of a worker step outside its job's own strided reads: one entry reading
/// (budget, watchdog start, the step clock's first value) and one exit reading (watchdog finish,
/// outcome trace, the caller's next-step decision) — `stepReadsOutsideTheJob` in the law.
struct StagedStep {
    closing: bool,
}

impl InteractiveJob for StagedStep {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {cx.set_stage("stage");JobOutcomeBorrow::admit_yield(cx)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _grant:RetainedCloneGrant) -> InteractiveJobCloseStep {
        InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()}
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

#[test]
fn a_worker_step_reads_the_clock_once_on_entry_and_once_on_exit() {
    let _serial = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let law = law();
    FAKE_NOW_US.store(9_000_000, Ordering::SeqCst);
    let params = BatchJobParams { operation: allocate_operation_id(), generation: Generation(1), cancel: root_cancel_token(), config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "clock-stride-law", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 }, now_us: fake_now_us };
    let mut authority = admit_original_fixture_owner!(WorkerJobAuthorityOwner,StagedStep { closing: false }, params).unwrap_or_else(|_| panic!("fixture admission"));
    for _ in 0..3 {
        FAKE_READS.store(0, Ordering::SeqCst);
        FAKE_NOW_US.fetch_add(10, Ordering::SeqCst);
        authority.issued_retained=crate::component::TEST_RETAINED_POLICY;
        assert!(!drive_worker_job_authority(&mut authority));
        assert!(authority.retained_receipt_pending);assert!(authority.retained_step_progress.fits(authority.issued_retained));authority.retained_receipt_pending=false;
        assert_eq!(FAKE_READS.load(Ordering::SeqCst), law["stepReadsOutsideTheJob"].as_u64().unwrap(), "a step reads the clock on entry and on exit only");
        assert_eq!(authority.last_step_end_us, Some(FAKE_NOW_US.load(Ordering::SeqCst)), "the exit reading is kept for the caller");
        while !authority.outcome.is_empty() { let receipt=authority.outcome.close_step(crate::component::TEST_RETAINED_POLICY).progress(); assert!(receipt.fits(crate::component::TEST_RETAINED_POLICY)); }
    }
    let job = authority.job.original_mut().expect("fixture job");
    job.begin_close();
    assert!(job.terminal_is_empty());
    while !authority.preadmitted_fault.is_empty() {
        authority.preadmitted_fault.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap();
    }
    authority.job.remove_terminal(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64}).unwrap();
}

/// 🧮️ The logical clock advances exactly one microsecond per read and belongs to its thread: a law driven by it measures
/// clock reads, never the wall time a descheduled thread loses.
#[test]
fn the_logical_clock_advances_one_microsecond_per_read_per_thread() {
    let first = logical_now_us().expect("logical clock reads");
    assert_eq!(logical_now_us(), Some(first + 1));
    assert_eq!(logical_now_us(), Some(first + 2));
    let other = std::thread::spawn(|| (logical_now_us(), logical_now_us())).join().expect("logical clock thread");
    assert_eq!(other, (Some(1), Some(2)));
    assert_eq!(logical_now_us(), Some(first + 3));
}
