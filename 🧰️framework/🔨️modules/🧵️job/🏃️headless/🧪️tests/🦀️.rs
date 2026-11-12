//! 🧪️ Portable headless failure laws preserve the original source, causes and successful output through blocked teardown.
use super::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

struct FixtureJob {
    operation: Operation,
    id: u64,
    source: Option<usize>,
    close_open: &'static AtomicBool,
    closes: &'static AtomicUsize,
    fail: bool,
}

impl FixtureJob {
    fn bind_source(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        assert_eq!(cx.operation(), self.operation.operation);
        assert_eq!(cx.generation(), self.operation.generation);
        let source = cx.original_source_parent()?.source_identity()?;
        if let Some(original) = self.source { assert_eq!(source, original); }
        else {
            self.source = Some(source); cx.consume_fuel(1);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        }
        Ok(())
    }
}

impl InteractiveJob for FixtureJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>, _claim: &mut semio_framework_job::OriginalJobReturnClaim) -> semio_framework_job::InteractiveJobStep<'a> {
        let result: Result<_, OriginalTurnFailure> = (|| {
            self.bind_source(cx)?;
            if self.fail { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "portable-headless-original-cause").into()); }
            Ok(semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None)?)
        })();
        semio_framework_job::InteractiveJobStep::from_callback(result)
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => descriptor.yielded(),
        }
    }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.source.is_none() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        assert_eq!(cx.operation(), self.operation.operation);
        assert_eq!(cx.generation(), self.operation.generation);
        assert_eq!(Some(cx.original_source_parent()?.source_identity()?), self.source);
        if !self.close_open.load(Ordering::SeqCst) { return Ok(InteractiveJobCloseStep::Blocked); }
        semio_framework_job::close_original_granted_step(cx, |grant| {
            if grant.maximum_items == 0 || grant.maximum_depth == 0 { return RetainedWorkOutcome::<_, ValueError> { result: Ok(InteractiveJobCloseStep::Pending { progress: Default::default() }), work: 0 }; }
            self.source = None;
            self.closes.fetch_add(1, Ordering::SeqCst);
            RetainedWorkOutcome { result: Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }), work: 1 }
        })
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.source.is_some())) }
    fn terminal_is_empty(&self) -> bool { self.source.is_none() }
}

fn operation() -> Operation { Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), Generation(9), 17) }
fn site() -> HeadlessSite {
    HeadlessSite {
        site: "job.headless.lifecycle", label: "headless-law", stage: InteractiveStage::InteractiveStep,
        fuel_per_step: 64, step_budget_us: 100_000,
        retained_stock: RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 16 << 20, maximum_capacity_bytes: 16 << 20, maximum_release_bytes: 16 << 20, maximum_depth: 128 },
        lifetime_fuel: 1_000_000, lifetime_budget_us: 1_000_000,
    }
}
fn expected(id: &str) -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️lifecycle/🔣️.json")).unwrap();
    fixture["cases"].as_array().unwrap().iter().find(|case| case["id"] == id).unwrap()["expected"].clone()
}
fn settle<J: InteractiveJob + Send + 'static, R>(failure: &mut HeadlessFailure<J, R>) -> (RetainedCloneProgress, u64) {
    let mut total = RetainedCloneProgress::default();
    let mut work = 0;
    for _ in 0..4096 {
        let (progress, used) = match failure.close_step() {
            HeadlessCloseStep::Progress { progress, work } | HeadlessCloseStep::Complete { progress, work } => (progress, work),
            other => panic!("original finite teardown stalled: {other:?}"),
        };
        assert!(progress.fits(HEADLESS_GRANT));
        if progress != RetainedCloneProgress::default() { assert!(used > 0); }
        total = total.checked_add(progress).unwrap(); work += used;
        if failure.terminal_is_empty() { return (total, work); }
    }
    panic!("original teardown exceeded the portable bound");
}

#[test]
fn rejected_failures_are_already_closed_and_convert_without_ownership() {
    let failure = HeadlessFailure::<FixtureJob>::rejected("portable-rejection".into());
    assert_eq!(serde_json::json!({"closed":failure.terminal_is_empty(),"jobReturned":false,"outputKept":false,"causeKept":failure.original_failure().is_some()}), expected("rejected"));
    assert_eq!(failure.into_message().unwrap(), "portable-rejection");
    let failure = HeadlessFailure::<FixtureJob>::rejected("portable-rejection".into());
    assert_eq!(failure.into_result().unwrap(), Err("portable-rejection".into()));
}

#[test]
fn refused_finite_source_preserves_the_exact_unadmitted_job() {
    static OPEN: AtomicBool = AtomicBool::new(false);
    static CLOSES: AtomicUsize = AtomicUsize::new(0);
    let operation = operation();
    let job = FixtureJob { operation, id: 73, source: None, close_open: &OPEN, closes: &CLOSES, fail: false };
    let site = HeadlessSite { retained_stock: RetainedCloneGrant { maximum_items: 0, ..site().retained_stock }, ..site() };
    let failure = run_headless(job, operation, || Some(0), site, |_, _| Ok(())).unwrap_err();
    assert!(!failure.terminal_is_empty());
    let (job, message) = failure.into_unadmitted_job().unwrap();
    assert_eq!(serde_json::json!({"closed":false,"jobReturned":job.id==73,"outputKept":false,"causeKept":false}), expected("admission"));
    assert!(message.contains("source"));
    assert!(job.source.is_none()); assert!(job.terminal_is_empty());
    assert_eq!(CLOSES.load(Ordering::SeqCst), 0);
}

#[derive(Debug)]
struct KeptOutput { value: u64, drops: &'static AtomicUsize }
impl Drop for KeptOutput { fn drop(&mut self) { self.drops.fetch_add(1, Ordering::SeqCst); } }

#[test]
fn blocked_close_preserves_success_until_the_same_original_source_resumes() {
    static OPEN: AtomicBool = AtomicBool::new(false);
    static CLOSES: AtomicUsize = AtomicUsize::new(0);
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    let operation = operation();
    let job = FixtureJob { operation, id: 73, source: None, close_open: &OPEN, closes: &CLOSES, fail: false };
    let failure = run_headless(job, operation, || Some(0), site(), |job, _| Ok(KeptOutput { value: job.id, drops: &DROPS })).unwrap_err();
    assert_eq!(serde_json::json!({"closed":failure.terminal_is_empty(),"jobReturned":false,"outputKept":failure.output.is_some(),"causeKept":failure.original_failure().is_some()}), expected("blocked-output"));
    assert_eq!(DROPS.load(Ordering::SeqCst), 0);
    let mut failure = failure.into_result().unwrap_err();
    assert!(matches!(failure.close_step(), HeadlessCloseStep::Blocked));
    let caller = failure.state.as_ref().unwrap().caller.as_ref().unwrap();
    let before = caller.receipt(); let fuel = caller.fuel_remaining();
    OPEN.store(true, Ordering::SeqCst);
    let (closed, work) = settle(&mut failure);
    assert!(work > 0 && work <= fuel);
    assert_eq!(before.retained_capacity_bytes + closed.retained_capacity_bytes, before.released_bytes + closed.released_bytes);
    let output = failure.into_result().unwrap().unwrap();
    assert_eq!(output.value, 73); assert_eq!(DROPS.load(Ordering::SeqCst), 0);
    assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
    drop(output); assert_eq!(DROPS.load(Ordering::SeqCst), 1);
}

#[test]
fn original_callback_causes_remain_borrowable_until_explicit_retirement() {
    static OPEN: AtomicBool = AtomicBool::new(false);
    static CLOSES: AtomicUsize = AtomicUsize::new(0);
    let operation = operation();
    let job = FixtureJob { operation, id: 89, source: None, close_open: &OPEN, closes: &CLOSES, fail: true };
    let mut failure = run_headless(job, operation, || Some(0), site(), |_, _| Ok(())).unwrap_err();
    assert_eq!(serde_json::json!({"closed":failure.terminal_is_empty(),"jobReturned":false,"outputKept":failure.output.is_some(),"causeKept":failure.original_failure().is_some()}), expected("original-cause"));
    assert!(format!("{:?}", failure.original_failure().unwrap()).contains("portable-headless-original-cause"));
    assert!(failure.original_continuation().is_some());
    OPEN.store(true, Ordering::SeqCst);
    let (_, work) = settle(&mut failure); assert!(work > 0);
    assert!(failure.original_failure().is_none());
    assert!(failure.into_result().unwrap().is_err());
    assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
}

#[path = "🧭️supervision/🦀️.rs"]
mod supervision;
