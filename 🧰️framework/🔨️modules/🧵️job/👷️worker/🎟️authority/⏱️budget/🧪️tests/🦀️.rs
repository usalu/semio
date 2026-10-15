//! ⏱️ Language-neutral deadline vectors and the actual driver entry boundary.

use super::*;
use super::testing::Fixture;

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
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure> {
        self.entered += 1;
        JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}

    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
    fn terminal_is_empty(&self) -> bool { self.closing }
}

//#endregion 🧪️DriverProbe

//#region 🚧️DeadlineAdmission
#[test]
fn microsecond_retained_worker_admits_half_ms_and_rejects_missing_or_overflow_clock() {
    for (clock, grant, fuel, expected_entries, faulted) in [(Some(1_000), 500, 1, 1, false), (Some(1_000), 0, 1, 0, false), (Some(1_000), 500, 0, 0, false), (None, 500, 1, 0, true), (Some(u64::MAX - 100), 500, 1, 0, true)] {
        bind_clock(clock);
        let params = BatchJobParams { operation: allocate_operation_id(), generation: Generation(1), cancel: root_cancel_token(), config: BatchDriveConfig { retained:TEST_RETAINED_POLICY, site: "microsecond-worker", stage: InteractiveStage::InteractiveStep, fuel_per_step: fuel, step_budget_us: grant }, now_us: now };
        let mut authority=Fixture::admit(EntryProbe{entered:0,closing:false},params);
        let terminal=authority.callback(TEST_RETAINED_POLICY);
        let actual_fault=matches!(authority.owner.original.outcome.original(),Some(original) if original.kind()==JobOutcomeKind::Fault);
        let entered=authority.owner.job().entered;
        authority.close();
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
            let params = BatchJobParams { operation: allocate_operation_id(), generation: Generation(1), cancel: root_cancel_token(), config: BatchDriveConfig { retained:TEST_RETAINED_POLICY, site: "microsecond-real-worker", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 }, now_us: default_now_us };
            let mut authority=Fixture::admit(EntryProbe{entered:0,closing:false},params);
            assert!(!authority.callback(TEST_RETAINED_POLICY));
            let entered=authority.owner.job().entered;authority.close();entered
        })
        .find(|entered| *entered != 0);
    assert_eq!(entered, Some(1), "a real 500 µs grant admits exactly one step whenever this thread is not descheduled past it");
}

//#endregion 🧵️RetainedWorker



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
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure> {
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
        JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
    fn terminal_is_empty(&self) -> bool { self.closing }
}

fn drive_slow_probe(slow_steps: usize, attempts: usize) -> (usize, Option<usize>, u32, u32, u64) {
    assert!(install_microsecond_clock(now).is_ok());
    bind_clock(Some(1_000));
    let params = BatchJobParams {
        operation: allocate_operation_id(),
        generation: Generation(1),
        cancel: root_cancel_token(),
        config: BatchDriveConfig { retained:TEST_RETAINED_POLICY, site: "sustained-overrun-probe", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: INTERACTIVE_LANE_WALL_US },
        now_us: now,
    };
    let mut authority=Fixture::admit(SlowProbe{remaining_slow_steps:slow_steps,steps:0,closing:false},params);
    let mut quarantined_at = None;
    for attempt in 0..attempts {
        let terminal=authority.callback(TEST_RETAINED_POLICY);
        if authority.owner.original.fault_pending && quarantined_at.is_none(){quarantined_at=Some(attempt);}
        if terminal {if quarantined_at.is_none(){quarantined_at=Some(attempt);}break;}
        authority.acknowledge();
    }
    assert_eq!(matches!(authority.owner.original.outcome.original(), Some(original) if original.kind()==JobOutcomeKind::Fault), quarantined_at.is_some());
    let ledger = *authority.owner.overrun_ledger();
    let steps=authority.owner.job().steps;authority.close();
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
struct RetainedReceiptProbe{output:Vec<u8>,phase:u8,panic_after_birth:bool,closing:bool}
impl RetainedReceiptProbe{
 fn demand(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:usize::from(self.phase==1)*23,capacity_bytes:usize::from(self.phase==0)*64,release_bytes:usize::from(self.phase==2)*self.output.capacity(),depth:3}}
}
impl InteractiveJob for RetainedReceiptProbe{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  let grant=cx.retained_grant();let demand=self.demand();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None);}
  let progress=match self.phase{0=>{self.output.try_reserve_exact(64).unwrap();RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.output.capacity(),..Default::default()}},1=>{self.output.extend_from_slice(b"original source payload");cx.consume_fuel(2);RetainedCloneProgress{copied_items:1,copied_bytes:23,..Default::default()}},2=>{let bytes=self.output.capacity();drop(std::mem::take(&mut self.output));RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}},_=>return Ok(None)};
  cx.consume_retained(progress).unwrap();self.phase+=1;if self.panic_after_birth{panic!("actual receipt survives producer panic");}Ok(None)
 }

 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if self.output.capacity()==0{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}let demand=RetirementDemand{release_bytes:self.output.capacity(),depth:1,..Default::default()};close_original_retained_step(cx,demand,|_|{drop(std::mem::take(&mut self.output));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))})}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.output.capacity())}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.output.capacity()==0}
}

#[test]
fn original_worker_job_retains_actual_step_receipt_across_producer_panic(){
 bind_clock(Some(1000));let grant=RetainedCloneGrant{maximum_items:3,maximum_copy_bytes:23,maximum_capacity_bytes:64,maximum_release_bytes:65536,maximum_depth:3};let params=BatchJobParams{operation:OperationId(98144),generation:Generation(17),cancel:root_cancel_token(),config:BatchDriveConfig{retained:grant,site:"actual-retained-receipt-panic",stage:InteractiveStage::InteractiveStep,fuel_per_step:7,step_budget_us:500},now_us:now};let mut authority=Fixture::admit(RetainedReceiptProbe{output:Vec::new(),phase:0,panic_after_birth:true,closing:false},params);assert!(authority.callback(grant));assert_eq!(authority.owner.retained_step_progress(),RetainedCloneProgress{copied_items:2,retained_capacity_bytes:64,..Default::default()});assert_eq!(authority.owner.retained_progress(),authority.owner.retained_step_progress());let pointer=authority.owner.job().output.as_ptr();assert_eq!(authority.owner.job().output.capacity(),64);assert_eq!(authority.owner.job().output.as_ptr(),pointer);authority.close();eprintln!("[DEBUG] actual original worker panic preserves born64 receipt and original deep output through paid native closure");
}
