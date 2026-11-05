//! 🧭️ Supervision observes genuine outcomes and retains its original result through paid teardown.
use super::*;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job_payload::{JobPayloadStream, RetainedPayloadBuilder};

static DATA: [u8; 32785] = [0x5a; 32785];
static WRONG: [u8; 32785] = { let mut bytes = [0x5a; 32785]; bytes[32784] = 0x59; bytes };

#[derive(Clone, Copy)]
enum Terminal { Complete, Cancelled, Fault }

struct SupervisedJob {
    base: FixtureJob,
    preparation: usize,
    yields: usize,
    yield_pending: bool,
    business_steps: usize,
    observed_steps: &'static AtomicUsize,
    terminal: Terminal,
    terminal_recorded: bool,
    payload: RetainedPayloadBuilder,
    cursor: usize,
    payload_bytes: usize,
}

impl SupervisedJob {
    fn new(operation: Operation, case: &serde_json::Value, open: &'static AtomicBool, closes: &'static AtomicUsize, steps: &'static AtomicUsize) -> Self {
        let terminal = match case["terminal"].as_str().unwrap() { "complete" => Terminal::Complete, "cancelled" => Terminal::Cancelled, "fault" => Terminal::Fault, _ => unreachable!() };
        Self {
            base: FixtureJob { operation, id: 101, source: None, close_open: open, closes, fail: false },
            preparation: case["preparationTurns"].as_u64().unwrap() as usize,
            yields: case["yields"].as_u64().unwrap() as usize, yield_pending: false,
            business_steps: 0, observed_steps: steps, terminal, terminal_recorded: false,
            payload: RetainedPayloadBuilder::new(if matches!(terminal, Terminal::Fault) { JobPayloadStream::Fault } else { JobPayloadStream::CommitOutput }),
            cursor: 0, payload_bytes: case["payloadBytes"].as_u64().unwrap() as usize,
        }
    }
    fn span(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        self.business_steps += 1;
        self.observed_steps.store(self.business_steps, Ordering::SeqCst);
        cx.consume_fuel(1);
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(())
    }
    fn advance<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        self.base.bind_source(cx)?;
        if self.preparation != 0 {
            self.preparation -= 1; cx.consume_fuel(1);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            return Ok(None);
        }
        if self.yields != 0 {
            if !self.yield_pending { self.span(cx)?; self.yield_pending = true; }
            let outcome = JobOutcomeBorrow::admit_yield(cx)?;
            if outcome.is_some() { self.yields -= 1; self.yield_pending = false; }
            return Ok(outcome);
        }
        if !matches!(self.terminal, Terminal::Cancelled) {
            if !self.payload.is_initialized() {
                let turn = self.payload.advance_initialization(cx);
                semio_framework_job::receive_original_retained_work(cx, turn)?; return Ok(None);
            }
            if self.cursor < self.payload_bytes {
                let turn = self.payload.append_original(cx, &DATA[..self.payload_bytes], &mut self.cursor);
                semio_framework_job::receive_original_retained_work(cx, turn)?; return Ok(None);
            }
            if self.payload.published().is_none() {
                let turn = self.payload.seal(cx);
                semio_framework_job::receive_original_retained_work(cx, turn)?; return Ok(None);
            }
        }
        if !self.terminal_recorded { self.span(cx)?; self.terminal_recorded = true; }
        Ok(match self.terminal {
            Terminal::Complete => JobOutcomeBorrow::admit_complete(cx, None, self.payload.published())?,
            Terminal::Cancelled => JobOutcomeBorrow::admit_cancelled(cx)?,
            Terminal::Fault => JobOutcomeBorrow::admit_fault(cx, self.payload.published().unwrap())?,
        })
    }
    fn demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.payload.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.base.source.is_some()), ..Default::default() }); }
        let mut demand = self.payload.retirement_demands()?; demand.depth = demand.depth.saturating_add(1); Ok(demand)
    }
}

impl InteractiveJob for SupervisedJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>, _claim: &mut semio_framework_job::OriginalJobReturnClaim) -> semio_framework_job::InteractiveJobStep<'a> {
        semio_framework_job::InteractiveJobStep::from_callback(self.advance(cx))
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, self.payload.published()),
            semio_framework_job::JobOutcomeKind::Fault => descriptor.fault(self.payload.published().unwrap()),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => descriptor.yielded(),
        }
    }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if !self.terminal_is_empty() && !self.base.close_open.load(Ordering::SeqCst) { return Ok(InteractiveJobCloseStep::Blocked); }
        if let Some(source) = self.base.source { assert_eq!(cx.original_source_parent()?.source_identity()?, source); }
        if !self.payload.terminal_is_empty() {
            let grant = RetainedCloneGrant { maximum_depth: cx.retained_grant().maximum_depth.saturating_sub(1), ..cx.retained_grant() };
            let step = semio_framework_job::receive_original_retained_work(cx, self.payload.close_step_granted(grant))?;
            return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
        }
        self.base.close_step(cx)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demand()?.depth) }
    fn terminal_is_empty(&self) -> bool { self.payload.terminal_is_empty() && self.base.terminal_is_empty() }
}

#[derive(Debug)]
struct SupervisedOutput { tag: &'static str, drops: &'static AtomicUsize }
impl Drop for SupervisedOutput { fn drop(&mut self) { self.drops.fetch_add(1, Ordering::SeqCst); } }

fn verify_payload(job: &SupervisedJob, bytes: &[u8]) {
    assert_eq!(bytes, &DATA[..job.payload_bytes]);
    let payload = job.payload.published().unwrap();
    assert!(payload_equals(payload, &DATA[..job.payload_bytes]));
    assert!(!payload_equals(payload, &DATA[..job.payload_bytes - 1]));
    assert!(!payload_equals(payload, &WRONG[..job.payload_bytes]));
}

fn cases() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️supervision/🔣️.json")).unwrap() }

#[test]
fn supervised_ordering_and_owned_results_follow_the_portable_cases() {
    static OPEN: AtomicBool = AtomicBool::new(true);
    static CLOSES: AtomicUsize = AtomicUsize::new(0);
    static STEPS: AtomicUsize = AtomicUsize::new(0);
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    for case in cases()["cases"].as_array().unwrap() {
        OPEN.store(!case["blockClose"].as_bool().unwrap(), Ordering::SeqCst);
        CLOSES.store(0, Ordering::SeqCst); STEPS.store(0, Ordering::SeqCst); DROPS.store(0, Ordering::SeqCst);
        let operation = operation();
        let job = SupervisedJob::new(operation, case, &OPEN, &CLOSES, &STEPS);
        let mut supervisor_calls = 0; let mut finish_calls = 0;
        let initial = case["initialStop"].as_str();
        let threshold = case["stopAtStep"].as_u64();
        let result = run_headless_supervised(job, operation, || Some(0), site(), |job| {
            supervisor_calls += 1;
            let tag = if initial == Some("cancelled") { Some("supervisor:cancelled") }
                else if initial == Some("budget") || threshold.is_some_and(|limit| job.business_steps as u64 >= limit) { Some("supervisor:budget") }
                else { None };
            tag.map(|tag| SupervisedOutput { tag, drops: &DROPS })
        }, |job, outcome| {
            finish_calls += 1;
            let tag = match outcome {
                HeadlessOutcome::Complete(bytes) => { verify_payload(job, bytes); "finish:complete" }
                HeadlessOutcome::Fault(bytes) => { verify_payload(job, bytes); "finish:fault" }
                HeadlessOutcome::Cancelled => "finish:cancelled",
            };
            Ok(SupervisedOutput { tag, drops: &DROPS })
        });
        let (output, kept) = match result {
            Ok(output) => (output, false),
            Err(failure) => {
                assert!(case["blockClose"].as_bool().unwrap(), "{}: {failure:?}", case["id"]);
                assert_eq!(DROPS.load(Ordering::SeqCst), 0);
                let mut failure = failure.into_result().unwrap_err();
                let caller = failure.state.as_ref().unwrap().caller.as_ref().unwrap();
                let before = caller.receipt(); let fuel = caller.fuel_remaining();
                OPEN.store(true, Ordering::SeqCst);
                let (closed, work) = settle(&mut failure);
                assert!(work > 0 && work <= fuel);
                assert_eq!(before.retained_capacity_bytes + closed.retained_capacity_bytes, before.released_bytes + closed.released_bytes);
                (failure.into_result().unwrap().unwrap(), true)
            }
        };
        let observed = serde_json::json!({"businessSteps":STEPS.load(Ordering::SeqCst),"supervisorCalls":supervisor_calls,"finishCalls":finish_calls,"result":output.tag,"keptAcrossBlocked":kept});
        assert_eq!(observed, case["expected"], "{}", case["id"]);
        assert_eq!(CLOSES.load(Ordering::SeqCst), usize::from(STEPS.load(Ordering::SeqCst) != 0));
        assert_eq!(DROPS.load(Ordering::SeqCst), 0); drop(output); assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    }
}

fn checkout_complete(state: &mut HeadlessState<SupervisedJob>) {
    for _ in 0..4096 {
        let poll = state.turn(|cx, _, _, _, session| session.as_mut().unwrap().step_on_original_caller(cx)).unwrap();
        if !matches!(poll, WorkerJobPoll::Outcome | WorkerJobPoll::Terminal) { continue; }
        let complete = {
            let session = state.session.as_mut().unwrap();
            assert!(session.checkout_outcome());
            let (grant, progress) = session.take_checked_out_retained_step_receipt().unwrap();
            assert!(progress.fits(grant));
            assert!(session.checked_out_continuation().is_none());
            match session.checked_out_outcome().unwrap() {
                Some(JobOutcomeView::Complete { output: Some(output), .. }) => { assert!(payload_equals(output, &DATA)); true }
                None | Some(JobOutcomeView::Yield { .. }) => false,
                _ => panic!("expected a complete multi-page fixture"),
            }
        };
        if complete { return; }
        let mut acknowledged = false;
        for _ in 0..4096 {
            if state.turn(|cx, _, _, _, session| session.as_mut().unwrap().acknowledge_outcome(cx)).unwrap() { acknowledged = true; break; }
        }
        assert!(acknowledged); state.session.as_mut().unwrap().resume().unwrap();
    }
    panic!("fixture did not expose a complete original outcome");
}

#[test]
fn capture_is_paid_and_acknowledgement_refusal_preserves_both_original_owners() {
    static OPEN: AtomicBool = AtomicBool::new(true);
    static CLOSES: AtomicUsize = AtomicUsize::new(0);
    static STEPS: AtomicUsize = AtomicUsize::new(0);
    let fixture = cases();
    let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["id"] == "blocked-terminal-capture").unwrap();
    let operation = operation();
    let job = SupervisedJob::new(operation, case, &OPEN, &CLOSES, &STEPS);
    let mut state = HeadlessState::new(job, site());
    let mut supervisor = |_: &mut SupervisedJob| None::<()>;
    let mut early = None;
    state.prepare(operation, || Some(0), &mut supervisor, &mut early).unwrap();
    assert!(early.is_none()); checkout_complete(&mut state);
    let before = state.caller.as_ref().unwrap().receipt();
    let fuel = state.caller.as_ref().unwrap().fuel_remaining();
    let mut captured = false;
    for _ in 0..4096 { if state.capture_step().unwrap() { captured = true; break; } }
    assert!(captured);
    let original = state.capture.bytes.as_ref().unwrap();
    assert_eq!(original.as_slice(), &DATA[..]);
    let backing = original.as_ptr() as usize;
    let capacity = original.capacity();
    let after = state.caller.as_ref().unwrap().receipt();
    assert!(after.copied_bytes - before.copied_bytes >= DATA.len());
    assert!(after.retained_capacity_bytes - before.retained_capacity_bytes >= capacity);
    assert!(state.caller.as_ref().unwrap().fuel_remaining() < fuel);
    let denied_before = state.caller.as_ref().unwrap().receipt();
    let acknowledged = state.turn(|cx, _, _, _, session| {
        let denied = RetainedCloneGrant { maximum_items: 0, ..cx.retained_grant() };
        cx.with_retained_loan(denied, |same| session.as_mut().unwrap().acknowledge_outcome(same))
    }).unwrap();
    assert!(!acknowledged);
    assert_eq!(state.caller.as_ref().unwrap().receipt(), denied_before);
    assert_eq!(state.capture.bytes.as_ref().unwrap().as_ptr() as usize, backing);
    assert_eq!(state.capture.bytes.as_ref().unwrap().as_slice(), &DATA[..]);
    match state.session.as_ref().unwrap().checked_out_outcome().unwrap() {
        Some(JobOutcomeView::Complete { output: Some(output), .. }) => assert!(payload_equals(output, &DATA)),
        _ => panic!("denied acknowledgement lost its original payload"),
    }
    let mut acknowledged = false;
    for _ in 0..4096 {
        if state.turn(|cx, _, _, _, session| session.as_mut().unwrap().acknowledge_outcome(cx)).unwrap() { acknowledged = true; break; }
    }
    assert!(acknowledged);
    let before_close = state.caller.as_ref().unwrap().receipt();
    let mut failure = HeadlessFailure::<_, ()> { message: "manual-ack-law".into(), state: Some(state), output: None };
    let (closed, work) = settle(&mut failure); assert!(work > 0);
    assert_eq!(before_close.retained_capacity_bytes + closed.retained_capacity_bytes, before_close.released_bytes + closed.released_bytes);
    assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
    assert!(failure.terminal_is_empty());
}
