use super::*;
use super::testing::Fixture;
use std::sync::atomic::{AtomicU64, Ordering};

static FAKE_NOW_US: AtomicU64 = AtomicU64::new(0);
static FAKE_READS: AtomicU64 = AtomicU64::new(0);
static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn fake_now_us() -> Option<u64> {
    FAKE_READS.fetch_add(1, Ordering::SeqCst);
    Some(FAKE_NOW_US.load(Ordering::SeqCst))
}

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🪜️clock-stride-law.json")).expect("clock-stride law fixture")
}

/// 🧮️ The whole clock cost of a worker step outside its job's own strided reads: one entry reading
/// (budget, watchdog start, the step clock's first value) and one exit reading (watchdog finish,
/// outcome trace, the caller's next-step decision) — `stepReadsOutsideTheJob` in the law.
struct StagedStep {
    closing: bool,
}

impl InteractiveJob for StagedStep {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure> {cx.set_stage("stage");JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

#[test]
fn a_worker_step_reads_the_clock_once_on_entry_and_once_on_exit() {
    let _serial = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let law = law();
    FAKE_NOW_US.store(9_000_000, Ordering::SeqCst);
    let params = BatchJobParams { operation: allocate_operation_id(), generation: Generation(1), cancel: root_cancel_token(), config: BatchDriveConfig { retained:TEST_RETAINED_POLICY, site: "clock-stride-law", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 }, now_us: fake_now_us };
    let mut authority=Fixture::admit(StagedStep{closing:false},params);
    for _ in 0..3 {
        FAKE_READS.store(0, Ordering::SeqCst);
        FAKE_NOW_US.fetch_add(10, Ordering::SeqCst);
        assert!(!authority.callback(TEST_RETAINED_POLICY));
        assert!(authority.owner.retained_step_progress().fits(TEST_RETAINED_POLICY));
        assert_eq!(FAKE_READS.load(Ordering::SeqCst), law["stepReadsOutsideTheJob"].as_u64().unwrap(), "a step reads the clock on entry and on exit only");
        assert_eq!(authority.owner.last_step_end_us(), Some(FAKE_NOW_US.load(Ordering::SeqCst)), "the exit reading is kept for the caller");
        authority.acknowledge();
    }
    authority.close();
}
