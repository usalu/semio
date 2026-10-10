//! ⏱️ Language-neutral deadline vectors and the actual driver entry boundary.

use super::*;

//#region 🧪️DriverProbe
thread_local! { static CLOCK: std::cell::Cell<Option<Option<u64>>> = const { std::cell::Cell::new(None) }; }

/// 🕰️ The fixture clock is installed process-wide by [`install_microsecond_clock`], so an UNBOUND
/// thread must still read a real monotonic source — otherwise one test's fixture reading silently
/// becomes every other test's platform clock, and the tests that measure real precision or real
/// elapsed time have no subject left to measure.
fn platform_now_us() -> Option<u64> {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    u64::try_from(START.get_or_init(std::time::Instant::now).elapsed().as_micros()).ok()
}

fn now() -> Option<u64> { CLOCK.with(std::cell::Cell::get).unwrap_or_else(platform_now_us) }

/// 🔗️ Binds this thread's fixture reading, including a deliberately absent one.
fn bind_clock(reading: Option<u64>) { CLOCK.with(|clock| clock.set(Some(reading))); }

/// ✂️ Returns this thread to the real platform clock.
fn unbind_clock() { CLOCK.with(|clock| clock.set(None)); }

struct EntryProbe { entered: usize, closing: bool }

impl InteractiveJob for EntryProbe {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {
        self.entered += 1;
        JobOutcomeBorrow::admit_yield(cx)
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}

    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self,_:RetainedCloneGrant)->InteractiveJobCloseStep { InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()} }
    fn terminal_is_empty(&self) -> bool { self.closing }
}

//#endregion 🧪️DriverProbe

//#region 🚧️DeadlineAdmission
#[test]
fn microsecond_zero_expired_or_empty_fuel_never_enters_job() {
    bind_clock(Some(1_000));
    for (fuel, deadline) in [(1, 1_000), (1, 999), (0, 1_500)] {
        let mut probe = EntryProbe { entered: 0, closing: false };
        let mut sequence = 0;
        let mut receipt=RetainedCloneProgress::default(); let mut context=StepContext::new(allocate_operation_id(),Generation(1),StepBudget::new(fuel,deadline,crate::component::TEST_RETAINED_POLICY),root_cancel_token(),now,&mut sequence,&mut receipt); let outcome=drive_step(&mut probe,&mut context,"microsecond-entry",InteractiveStage::InteractiveStep,&mut None); assert!(outcome.unwrap().is_none());
        probe.begin_close();
        assert_eq!(probe.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:4_096,maximum_depth:64,..RetainedCloneGrant::default()}), InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()});
        assert!(probe.terminal_is_empty());
        assert_eq!(receipt,RetainedCloneProgress::default());
        assert_eq!(probe.entered, 0, "expired or exhausted grant entered job: fuel={fuel}, deadline={deadline}");
    }
}

#[test]
fn microsecond_language_neutral_deadline_boundaries_and_overflow() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for law in fixture["cases"].as_array().unwrap() {
        let start = law["start"].as_str().unwrap().parse::<u64>().unwrap();
        let fuel = law["fuel"].as_u64().unwrap();
        let grant = law["grant"].as_u64().unwrap();
        let budget = StepBudget::from_duration(fuel, start, grant,crate::component::TEST_RETAINED_POLICY);
        assert_eq!(budget.map(|value| value.deadline_us.to_string()), law["deadline"].as_str().map(str::to_owned), "{}", law["id"]);
        for (index, sample) in law["samples"].as_array().unwrap().iter().enumerate() {
            bind_clock(Some(sample.as_str().unwrap().parse().unwrap()));
            let mut sequence = 0;
            let (expired, yielded) = match budget {
                Some(budget) => {
                    let mut actual_retained_progress=RetainedCloneProgress::default();let cx = StepContext::new(allocate_operation_id(), Generation(1), budget, root_cancel_token(), now, &mut sequence,&mut actual_retained_progress);
                    (cx.deadline_exceeded(), cx.should_yield())
                }
                None => (true, true),
            };
            assert_eq!(expired, law["expired"][index].as_bool().unwrap());
            assert_eq!(yielded, law["yielded"][index].as_bool().unwrap());
        }
    }
}

//#endregion 🚧️DeadlineAdmission

//#region 🧵️RetainedWorker
fn close_authority(mut authority: WorkerJobAuthorityOwner<EntryProbe>) -> usize {
    let job = authority.job.original_mut().unwrap();
    job.begin_close();
    assert_eq!(job.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:4_096,maximum_depth:64,..RetainedCloneGrant::default()}), InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()});
    assert!(job.terminal_is_empty());
    let entered = job.entered;
    while !authority.outcome.is_empty() { let _ = authority.outcome.close_step( RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }); }
    while !authority.preadmitted_fault.is_empty() { let _ = authority.preadmitted_fault.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(); }
    authority.job.remove_terminal(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64}).unwrap();
    entered
}

#[test]
fn microsecond_retained_worker_admits_half_ms_and_rejects_missing_or_overflow_clock() {
    for (clock, grant, fuel, expected_entries, faulted) in [(Some(1_000), 500, 1, 1, false), (Some(1_000), 0, 1, 0, false), (Some(1_000), 500, 0, 0, false), (None, 500, 1, 0, true), (Some(u64::MAX - 100), 500, 1, 0, true)] {
        bind_clock(clock);
        let params = BatchJobParams { operation: allocate_operation_id(), generation: Generation(1), cancel: root_cancel_token(), config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "microsecond-worker", stage: InteractiveStage::InteractiveStep, fuel_per_step: fuel, step_budget_us: grant }, now_us: now };
        let mut authority = admit_original_fixture_owner!(WorkerJobAuthorityOwner,EntryProbe { entered: 0, closing: false }, params).unwrap_or_else(|_| panic!("fixture payload admission"));
        authority.issued_retained=crate::component::TEST_RETAINED_POLICY;
        let terminal = drive_worker_job_authority(&mut authority);
        assert!(authority.retained_step_progress.fits(authority.issued_retained));assert!(authority.retained_receipt_pending);authority.retained_receipt_pending=false;
        let actual_fault = matches!(authority.outcome.original(), Some(original) if original.kind()==JobOutcomeKind::Fault);
        let entered = close_authority(authority);
        assert_eq!(terminal, faulted);
        assert_eq!(actual_fault, faulted);
        assert_eq!(entered, expected_entries);
    }
}

/// 🎲️ A real 500 µs grant is spent by wall time, and a loaded host may deschedule this thread for
/// longer than that between reading the grant's start and the first step — an honest overrun that
/// admits no step. Fresh attempts separate that from a clock or budget defect, which fails them all.
const REAL_CLOCK_ATTEMPTS: usize = 64;

#[test]
fn microsecond_platform_clock_and_real_half_ms_worker_progress() {
    unbind_clock();
    let start = default_now_us().unwrap();
    let mut submillisecond_sample = false;
    for _ in 0..10_000 {
        let current = default_now_us().unwrap();
        assert!(current >= start);
        submillisecond_sample |= current % 1_000 != 0;
        if submillisecond_sample { break; }
    }
    assert!(submillisecond_sample, "platform clock lost microsecond precision");
    let entered = (0..REAL_CLOCK_ATTEMPTS)
        .map(|_| {
            let params = BatchJobParams { operation: allocate_operation_id(), generation: Generation(1), cancel: root_cancel_token(), config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "microsecond-real-worker", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 }, now_us: default_now_us };
            let mut authority = admit_original_fixture_owner!(WorkerJobAuthorityOwner,EntryProbe { entered: 0, closing: false }, params).unwrap_or_else(|_| panic!("fixture payload admission"));
            authority.issued_retained=crate::component::TEST_RETAINED_POLICY;
            assert!(!drive_worker_job_authority(&mut authority), "a spent real grant yields, it never terminates");
            assert!(authority.retained_step_progress.fits(authority.issued_retained));assert!(authority.retained_receipt_pending);authority.retained_receipt_pending=false;
            close_authority(authority)
        })
        .find(|entered| *entered != 0);
    assert_eq!(entered, Some(1), "a real 500 µs grant admits exactly one step whenever this thread is not descheduled past it");
}

//#endregion 🧵️RetainedWorker

//#region 🔒️CallbackQuarantine
struct CompletionProbe { end: Option<u64>, closing: bool, output: Option<JobPayloadSlot> }

impl InteractiveJob for CompletionProbe {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {
        if self.output.is_none(){let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, b"kept").unwrap_or_else(|_| panic!("exact fixture page admission"));self.output=Some(JobPayloadSlot::from_payload(output));}
        bind_clock(self.end);
        JobOutcomeBorrow::admit_complete(cx,None,self.output.as_ref().and_then(JobPayloadSlot::original))
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{descriptor.complete(None,self.output.as_ref().and_then(JobPayloadSlot::original))}
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep { if let Some(output)=self.output.as_mut(){if !output.is_empty(){return match output.close_step(grant){Ok(step)=>InteractiveJobCloseStep::Pending{progress:step.progress()},Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};} } InteractiveJobCloseStep::Complete{progress:Default::default()} }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.output.as_ref().map(JobPayloadSlot::retirement_demands).transpose()?.unwrap_or_default().copy_bytes)}
    fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.output.as_ref().map(JobPayloadSlot::retirement_demands).transpose()?.unwrap_or_default().release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.output.as_ref().map(JobPayloadSlot::retirement_demands).transpose()?.unwrap_or_default().depth)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.output.as_ref().is_none_or(JobPayloadSlot::is_empty) }
}

#[test]
fn microsecond_exact_callback_quarantine_retains_original_output_and_session_identity() {
    let _slots = super::worker_session_slots_shared();
    assert!(install_microsecond_clock(now).is_ok());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../⏱️trace/⏱️clock/🧫️fixtures/🧪️contention/🔣️.json")).unwrap();
    let operation = allocate_operation_id();
    let generation = Generation(71);
    for law in fixture["verdicts"].as_array().unwrap().iter().filter(|law| !law["start"].is_null()) {
        bind_clock(law["start"].as_u64());
        let params = BatchJobParams { operation, generation, cancel: root_cancel_token(), config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "microsecond-exact-quarantine", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 }, now_us: now };
        let mut session = admit_original_fixture_owner!(WorkerJobSession,CompletionProbe { end: law["end"].as_u64(), closing: false, output:None }, params).unwrap_or_else(|_| panic!("exact worker admission"));
        let(ticket,poll)=session.try_step_on_caller(crate::component::TEST_RETAINED_POLICY).expect("one actual caller opportunity");
        let mut first=match poll{WorkerJobPoll::Terminal=>session.take_terminal().unwrap(),WorkerJobPoll::Outcome=>session.take_outcome(ticket).unwrap(),_=>panic!("original opportunity must retain its exact owner")};
        let original_receipt=first.take_retained_step_receipt().expect("original callback receipt recipient");assert!(original_receipt.1.fits(original_receipt.0));let verdict=first.callback_verdict().expect("first original callback verdict retained");let faulted=verdict.is_fault();assert_eq!(faulted,law["fault"].as_bool().unwrap());assert_eq!(verdict.operation(),operation);assert_eq!(verdict.generation(),generation);
        let original_output=first.job().output.as_ref().and_then(JobPayloadSlot::original).expect("original output birth stays in producer").page(0).unwrap().as_ptr();
        let owner=if poll==WorkerJobPoll::Terminal{first}else{match first.resume(){Ok(())=>{},Err(original)=>{let _original=original;panic!("original semantic absence failed to resume same owner")}};let mut terminal=None;for _ in 0..64{let(ticket,poll)=session.try_step_on_caller(crate::component::TEST_RETAINED_POLICY).unwrap();if poll==WorkerJobPoll::Terminal{let mut original=session.take_terminal().unwrap();let original_receipt=original.take_retained_step_receipt().expect("original terminal receipt");assert!(original_receipt.1.fits(original_receipt.0));terminal=Some(original);break;}let mut original=session.take_outcome(ticket).unwrap();let original_receipt=original.take_retained_step_receipt().expect("original continuation receipt");assert!(original_receipt.1.fits(original_receipt.0));for _ in 0..2{original.acknowledge_outcome(crate::component::TEST_RETAINED_POLICY);}match original.resume(){Ok(())=>{},Err(original)=>{let _original=original;panic!("funded original continuation failed to resume same owner")}};}terminal.expect("funded original continuation reaches terminal within64 opportunities")};
        assert_eq!(owner.job().output.as_ref().and_then(JobPayloadSlot::original).unwrap().page(0).unwrap().as_ptr(),original_output);
        let quarantined = law["sessionTerminal"].as_bool().unwrap();
        assert!(!quarantined || faulted, "a quarantine implies a breached sample");
        if quarantined {
            assert!(matches!(owner.outcome().unwrap(), Some(JobOutcomeView::Fault{..})));
            let output=owner.job().output.as_ref().and_then(JobPayloadSlot::original).expect("quarantined original output remains owned before semantic publication");
            assert_eq!(output.single_page(), Some(b"kept".as_slice()));
        } else {
            assert!(matches!(owner.outcome().unwrap(), Some(JobOutcomeView::Complete{..})));
            assert!(owner.authority.as_ref().unwrap().quarantined_outcome.is_empty());
        }
        let ledger = Arc::clone(owner.authority.as_ref().unwrap().payload_ledger.as_ref().expect("original admitted worker ledger"));
        owner.begin_close();
        let owned_bytes = ledger.bytes.load(Ordering::Acquire);
        let _ = session.close_step(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()});
        assert_eq!(ledger.bytes.load(Ordering::Acquire), owned_bytes);
        let mut released_bytes = 0;
        for _ in 0..64 {
            let phase=session.close_phase();let caller=super::retained_ownership_tests::caller_worker_close_policy(phase);
            match session.close_step(caller) {
                WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:released_items,released_bytes:bytes,..}} => { assert!(released_items <= 1 && bytes <= caller.maximum_release_bytes); if matches!(phase,WorkerJobClosePhase::QuarantinedOutcome|WorkerJobClosePhase::Outcome|WorkerJobClosePhase::PreadmittedFault|WorkerJobClosePhase::Job){assert!(bytes<=JOB_PAYLOAD_PAGE_BYTES);released_bytes += bytes;} }
                WorkerJobCloseStep::Blocked => std::thread::yield_now(),
            WorkerJobCloseStep::Refused{kind,..}=>panic!("original close grant refused: {kind:?}"),
                WorkerJobCloseStep::Complete {progress} => break,
            }
        }
        assert!(session.terminal_is_empty());
        assert!(ledger.terminal_is_empty());
        assert_eq!(owned_bytes, 2 * JOB_PAYLOAD_PAGE_BYTES);
        assert_eq!(released_bytes, owned_bytes);
    }
}
//#endregion 🔒️CallbackQuarantine

//#region 📒️SustainedOverrun
/// 🐢️ Sleeps for real on its slow steps and reports the wall time it actually burned to the same
/// installed monotonic clock the watchdog reads, so "the thread was descheduled once" and "this step
/// never fits the ceiling" differ only in how many steps are slow — exactly as they do in production.
struct SlowProbe {
    remaining_slow_steps: usize,
    steps: usize,
    closing: bool,
}

impl InteractiveJob for SlowProbe {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {
        self.steps += 1;
        let slow = self.remaining_slow_steps > 0;
        if slow {
            self.remaining_slow_steps -= 1;
        }
        let started = std::time::Instant::now();
        if slow {
            std::thread::sleep(std::time::Duration::from_micros(semio_framework_trace::INTERACTIVE_STEP_CEILING_US + 2_000));
        }
        let elapsed_us = u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX);
        bind_clock(now().map(|value| value.saturating_add(elapsed_us)));
        JobOutcomeBorrow::admit_yield(cx)
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self,_:RetainedCloneGrant)->InteractiveJobCloseStep { InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()} }
    fn terminal_is_empty(&self) -> bool { self.closing }
}

fn close_slow_authority(mut authority: WorkerJobAuthorityOwner<SlowProbe>) -> usize {
    let job = authority.job.original_mut().unwrap();
    job.begin_close();
    assert_eq!(job.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:4_096,maximum_depth:64,..RetainedCloneGrant::default()}), InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()});
    let steps = job.steps;
    while !authority.outcome.is_empty() { let _ = authority.outcome.close_step( RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }); }
    while !authority.quarantined_outcome.is_empty() { let _ = authority.quarantined_outcome.close_step( RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }); }
    while !authority.preadmitted_fault.is_empty() { let _ = authority.preadmitted_fault.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }).unwrap(); }
    authority.job.remove_terminal(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64}).unwrap();
    steps
}

fn drive_slow_probe(slow_steps: usize, attempts: usize) -> (usize, Option<usize>, u32, u32, u64) {
    assert!(install_microsecond_clock(now).is_ok());
    bind_clock(Some(1_000));
    let params = BatchJobParams {
        operation: allocate_operation_id(),
        generation: Generation(1),
        cancel: root_cancel_token(),
        config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "sustained-overrun-probe", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: INTERACTIVE_LANE_WALL_US },
        now_us: now,
    };
    let mut authority = admit_original_fixture_owner!(WorkerJobAuthorityOwner,SlowProbe { remaining_slow_steps: slow_steps, steps: 0, closing: false }, params).unwrap_or_else(|_| panic!("exact worker admission"));
    let mut quarantined_at = None;
    for attempt in 0..attempts {
        authority.issued_retained=crate::component::TEST_RETAINED_POLICY;
        let terminal=drive_worker_job_authority(&mut authority);
        assert!(authority.retained_step_progress.fits(authority.issued_retained));assert!(authority.retained_receipt_pending);authority.retained_receipt_pending=false;
        if authority.fault_pending && quarantined_at.is_none(){quarantined_at=Some(attempt);}
        if terminal {if quarantined_at.is_none(){quarantined_at=Some(attempt);}break;}
        while !authority.outcome.is_empty() { authority.outcome.close_step( RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 64 }); }
    }
    assert_eq!(matches!(authority.outcome.original(), Some(original) if original.kind()==JobOutcomeKind::Fault), quarantined_at.is_some());
    let ledger = authority.overruns;
    let steps = close_slow_authority(authority);
    (steps, quarantined_at, ledger.consecutive_overruns(), ledger.total_overruns(), ledger.worst_elapsed_us())
}

#[test]
fn descheduled_step_over_the_ceiling_is_recorded_and_the_job_keeps_running() {
    let attempts = usize::try_from(semio_framework_trace::SUSTAINED_OVERRUN_QUARANTINE_STEPS).unwrap() + 2;
    let (steps, quarantined_at, consecutive, total, worst_us) = drive_slow_probe(1, attempts);
    assert_eq!(quarantined_at, None, "one over-ceiling wall reading is the machine, not the job");
    assert_eq!(steps, attempts, "every later step must still run");
    assert_eq!(consecutive, 0, "a step that fit the ceiling clears the run");
    assert_eq!(total, 1);
    assert!(worst_us >= semio_framework_trace::INTERACTIVE_STEP_CEILING_US, "the overrun really happened and was recorded");
}

#[test]
fn every_step_over_the_ceiling_is_quarantined_within_the_sustained_threshold() {
    let threshold = usize::try_from(semio_framework_trace::SUSTAINED_OVERRUN_QUARANTINE_STEPS).unwrap();
    let (steps, quarantined_at, consecutive, total, _) = drive_slow_probe(threshold + 4, threshold + 4);
    assert_eq!(quarantined_at, Some(threshold - 1), "a runaway step is stopped exactly at the threshold");
    assert_eq!(steps, threshold, "no step may run after the quarantine");
    assert_eq!(consecutive, u32::try_from(threshold).unwrap());
    assert_eq!(total, u32::try_from(threshold).unwrap());
}
//#endregion 📒️SustainedOverrun

//#region 🌐️PlatformClock
#[test]
fn microsecond_browser_clock_fraction_and_invalid_source_vectors() {
    unbind_clock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🕰️clock.json")).unwrap();
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


struct RetainedReceiptProbe{output:Vec<u8>,phase:u8,panic_after_birth:bool,closing:bool}
impl RetainedReceiptProbe{
 fn demand(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:usize::from(self.phase==1)*23,capacity_bytes:usize::from(self.phase==0)*64,release_bytes:usize::from(self.phase==2)*self.output.capacity(),depth:3}}
}
impl InteractiveJob for RetainedReceiptProbe{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{
  let grant=cx.retained_grant();let demand=self.demand();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None);}
  let progress=match self.phase{0=>{self.output.try_reserve_exact(64).unwrap();RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.output.capacity(),..Default::default()}},1=>{self.output.extend_from_slice(b"original source payload");cx.consume_fuel(2);RetainedCloneProgress{copied_items:1,copied_bytes:23,..Default::default()}},2=>{let bytes=self.output.capacity();drop(std::mem::take(&mut self.output));RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}},_=>return Ok(None)};
  cx.consume_retained(progress).unwrap();self.phase+=1;if self.panic_after_birth{panic!("actual receipt survives producer panic");}Ok(None)
 }

 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep{if self.output.capacity()==0{return InteractiveJobCloseStep::Complete{progress:Default::default()};}if grant.maximum_items==0||grant.maximum_release_bytes<self.output.capacity()||grant.maximum_depth==0{return InteractiveJobCloseStep::Pending{progress:Default::default()};}let bytes=self.output.capacity();drop(std::mem::take(&mut self.output));InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}}}
 fn terminal_is_empty(&self)->bool{self.closing&&self.output.capacity()==0}
}

#[test]
fn original_job_step_context_conserves_actual_independent_retained_wallet(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["retained"];let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let mut owner=StepContextOwner::new(OperationId(98143),Generation(17),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:StepContextOwner::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap().0;let cancel=root_cancel_token();let mut probe=RetainedReceiptProbe{output:Vec::new(),phase:0,panic_after_birth:false,closing:false};let mut progress=RetainedCloneProgress::default();let mut sequence=0;
 for expected in law["receipts"].as_array().unwrap(){
  let demand=probe.demand();let original=(probe.phase,probe.output.as_ptr(),probe.output.len(),probe.output.capacity());
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:2,..grant}),if demand.copy_bytes>0{Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant})}else{None},if demand.capacity_bytes>0{Some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})}else{None},if demand.release_bytes>0{Some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})}else{None}].into_iter().flatten(){let mut refused=RetainedCloneProgress::default();let mut refused_sequence=0;let mut cx=owner.context(StepBudget::new(7,1500,denied),cancel.clone(),now,&mut refused_sequence,&mut refused).unwrap();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||probe.step(&mut cx));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),Default::default());assert_eq!((probe.phase,probe.output.as_ptr(),probe.output.len(),probe.output.capacity()),original);}
  let before=progress;let mut cx=owner.context(StepBudget::new(7,1500,grant),cancel.clone(),now,&mut sequence,&mut progress).unwrap();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||probe.step(&mut cx));let actual=cx.retained_progress();let receipt=RetainedCloneProgress{copied_items:actual.copied_items-before.copied_items,copied_bytes:actual.copied_bytes-before.copied_bytes,retained_capacity_bytes:actual.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:actual.released_bytes-before.released_bytes};assert_eq!(receipt,serde_json::from_value(expected.clone()).unwrap());assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));assert_eq!(cx.operation(),OperationId(98143));assert_eq!(cx.generation(),Generation(17));assert_eq!(cx.deadline_us(),1500);assert_eq!(cx.fuel_remaining(),if probe.phase==2{5}else{7});if probe.phase==2{assert_eq!(std::str::from_utf8(&probe.output).unwrap(),law["source"].as_str().unwrap());}
 }
 let mut cx=owner.context(StepBudget::new(7,1500,grant),cancel.clone(),now,&mut sequence,&mut progress).unwrap();assert_eq!(cx.retained_grant(),serde_json::from_value(law["remaining"].clone()).unwrap());drop(cx);probe.begin_close();assert!(probe.terminal_is_empty());while !owner.terminal_is_empty(){owner.close_step(crate::component::TEST_RETAINED_POLICY);}eprintln!("[DEBUG] actual original StepContext same operation/source, separate born64/copy23/free64, zero/short allocator0, actual valid child receipts remain in original recipient, fuel7/deadline1500 independent");
}

#[test]
fn original_worker_job_retains_actual_step_receipt_across_producer_panic(){
 bind_clock(Some(1000));let grant=RetainedCloneGrant{maximum_items:3,maximum_copy_bytes:23,maximum_capacity_bytes:64,maximum_release_bytes:65536,maximum_depth:3};let params=BatchJobParams{operation:OperationId(98144),generation:Generation(17),cancel:root_cancel_token(),config:BatchDriveConfig{retained:grant,site:"actual-retained-receipt-panic",stage:InteractiveStage::InteractiveStep,fuel_per_step:7,step_budget_us:500},now_us:now};let mut authority=admit_original_fixture_owner!(WorkerJobAuthorityOwner,RetainedReceiptProbe{output:Vec::new(),phase:0,panic_after_birth:true,closing:false},params).unwrap_or_else(|_|panic!("original worker owner admission"));authority.issued_retained=grant;assert!(drive_worker_job_authority(&mut authority));assert!(authority.retained_receipt_pending);assert!(authority.retained_step_progress.fits(grant));authority.retained_receipt_pending=false;assert_eq!(authority.retained_step_progress,RetainedCloneProgress{copied_items:2,retained_capacity_bytes:64,..Default::default()});assert_eq!(authority.retained_progress,authority.retained_step_progress);let pointer=authority.job.original().unwrap().output.as_ptr();assert_eq!(authority.job.original().unwrap().output.capacity(),64);authority.job.original_mut().unwrap().begin_close();assert_eq!(authority.job.original().unwrap().output.as_ptr(),pointer);authority.job.original_mut().unwrap().close_step(crate::component::TEST_RETAINED_POLICY);authority.job.remove_terminal(crate::component::TEST_RETAINED_POLICY).unwrap();while !authority.outcome.is_empty(){authority.outcome.close_step(crate::component::TEST_RETAINED_POLICY);}while !authority.preadmitted_fault.is_empty(){authority.preadmitted_fault.close_step(crate::component::TEST_RETAINED_POLICY).unwrap();}while authority.original_worker_failure_demands(crate::component::TEST_RETAINED_POLICY.maximum_copy_bytes).unwrap().is_some(){authority.close_original_worker_failure(crate::component::TEST_RETAINED_POLICY).unwrap();}while authority.params.is_some()||authority.cancel_retirement.is_some(){authority.close_original_worker_params(crate::component::TEST_RETAINED_POLICY);}while !authority.0.is_empty(){authority.close_terminal_worker_authority(crate::component::TEST_RETAINED_POLICY);}eprintln!("[DEBUG] actual original worker panic retains born64 receipt and same original deep allocation until funded close");
}

#[test]
fn original_job_failed_receipt_ingress_keeps_actual_external_recipient(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../⏱️context/📦️owner/⚠️failure/🧫️fixtures/🔣️.json")).unwrap();
 let mut owner=StepContextOwner::new(OperationId(98145),Generation(17),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:StepContextOwner::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap().0;
 let mut sequence=0;let mut recipient=RetainedCloneProgress::default();let mut probe=RetainedReceiptProbe{output:Vec::new(),phase:0,panic_after_birth:false,closing:false};
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:64,maximum_depth:3};let mut cx=owner.context(StepBudget::new(7,1500,grant),root_cancel_token(),now,&mut sequence,&mut recipient).unwrap();
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{probe.output.try_reserve_exact(law["producerBytes"].as_u64().unwrap()as usize).unwrap();let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:probe.output.capacity(),..Default::default()};cx.consume_retained(progress)});
 let error=result.unwrap_err();let actual=cx.retained_progress();assert_eq!(actual.retained_capacity_bytes,heap.requested_bytes);assert_eq!(heap.released_bytes,0);assert_eq!(error.retained_progress(),actual);assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(cx.retained_grant().maximum_capacity_bytes,0);assert_eq!(cx.retained_grant().maximum_items,0);assert_eq!(cx.fuel_remaining(),7);assert_eq!(cx.deadline_us(),1500);assert_eq!(cx.operation(),OperationId(98145));let pointer=probe.output.as_ptr();
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.retained_grant());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(cx);assert_eq!(recipient,actual);assert_eq!(probe.output.as_ptr(),pointer);probe.begin_close();
 let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||probe.close_step(grant));assert_eq!(step.progress().released_bytes,actual.retained_capacity_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,actual.retained_capacity_bytes));assert!(probe.terminal_is_empty());while !owner.terminal_is_empty(){owner.close_step(crate::component::TEST_RETAINED_POLICY);}
 eprintln!("[DEBUG] actual failed Job child born64 survives denied incoming capacity0 in same external recipient and error; remaining0/fuel7/deadline1500, original pointer until granted free64");
}
