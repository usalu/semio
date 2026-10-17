//! ⏱️ Language-neutral deadline vectors and the actual driver entry boundary.

use semio_framework_job::*;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand};
#[global_allocator]
static ORIGINAL_BUDGET_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};

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
fn microsecond_zero_expired_or_empty_fuel_never_enters_job() {
    bind_clock(Some(1_000));
    for (fuel, deadline) in [(1, 1_000), (1, 999), (0, 1_500)] {
        let mut probe = EntryProbe { entered: 0, closing: false };
        let source=TEST_RETAINED_POLICY;let mut caller=OriginalJobFixtureCaller::admit(allocate_operation_id(),Generation(1),source,now,u64::MAX,100000);
        assert!(caller.turn(source,|context|context.with_original_work_loan(fuel,deadline,|context|drive_step(&mut probe,context,"microsecond-entry",InteractiveStage::InteractiveStep,&mut None).map(|outcome|outcome.is_none()))).unwrap());
        probe.begin_close();
        assert_eq!(caller.turn(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:4_096,maximum_depth:64,..RetainedCloneGrant::default()},|cx|probe.close_step(cx)).unwrap(), InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()});
        assert!(probe.terminal_is_empty());
        assert_eq!(caller.receipt(),RetainedCloneProgress::default());caller.retire(source);
        assert_eq!(probe.entered, 0, "expired or exhausted grant entered job: fuel={fuel}, deadline={deadline}");
    }
}

#[test]
fn microsecond_language_neutral_deadline_boundaries_and_overflow() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for law in fixture["cases"].as_array().unwrap() {
        let start = law["start"].as_str().unwrap().parse::<u64>().unwrap();
        let fuel = law["fuel"].as_u64().unwrap();
        let grant = law["grant"].as_u64().unwrap();
        let budget = StepBudget::from_duration(fuel, start, grant,TEST_RETAINED_POLICY);
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
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  let grant=cx.retained_grant();let demand=self.demand();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None);}
  let progress=match self.phase{0=>{self.output.try_reserve_exact(64).unwrap();RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.output.capacity(),..Default::default()}},1=>{self.output.extend_from_slice(b"original source payload");cx.consume_fuel(2);RetainedCloneProgress{copied_items:1,copied_bytes:23,..Default::default()}},2=>{let bytes=self.output.capacity();drop(std::mem::take(&mut self.output));RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}},_=>return Ok(None)};
  cx.consume_retained(progress).unwrap();self.phase+=1;if self.panic_after_birth{panic!("actual receipt survives producer panic");}Ok(None)
 }

 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if self.output.capacity()==0{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}let demand=RetirementDemand{release_bytes:self.output.capacity(),depth:1,..Default::default()};close_original_retained_step(cx,demand,|_|{drop(std::mem::take(&mut self.output));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))})}
 fn terminal_is_empty(&self)->bool{self.closing&&self.output.capacity()==0}
}

#[test]
fn original_job_step_context_conserves_actual_independent_retained_wallet(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["retained"];let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let mut owner=StepContextOwner::new(OperationId(98143),Generation(17),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:StepContextOwner::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap().0;let cancel=root_cancel_token();let mut probe=RetainedReceiptProbe{output:Vec::new(),phase:0,panic_after_birth:false,closing:false};let mut progress=RetainedCloneProgress::default();let mut sequence=0;
 for expected in law["receipts"].as_array().unwrap(){
  let demand=probe.demand();let original=(probe.phase,probe.output.as_ptr(),probe.output.len(),probe.output.capacity());
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:2,..grant}),if demand.copy_bytes>0{Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant})}else{None},if demand.capacity_bytes>0{Some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})}else{None},if demand.release_bytes>0{Some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})}else{None}].into_iter().flatten(){let mut refused=RetainedCloneProgress::default();let mut refused_sequence=0;let mut cx=owner.context(StepBudget::new(7,1500,denied),cancel.clone(),now,&mut refused_sequence,&mut refused).unwrap();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||probe.step(&mut cx));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),Default::default());assert_eq!((probe.phase,probe.output.as_ptr(),probe.output.len(),probe.output.capacity()),original);}
  let before=progress;let mut cx=owner.context(StepBudget::new(7,1500,grant),cancel.clone(),now,&mut sequence,&mut progress).unwrap();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||probe.step(&mut cx));let actual=cx.retained_progress();let receipt=RetainedCloneProgress{copied_items:actual.copied_items-before.copied_items,copied_bytes:actual.copied_bytes-before.copied_bytes,retained_capacity_bytes:actual.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:actual.released_bytes-before.released_bytes};assert_eq!(receipt,serde_json::from_value(expected.clone()).unwrap());assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));assert_eq!(cx.operation(),OperationId(98143));assert_eq!(cx.generation(),Generation(17));assert_eq!(cx.deadline_us(),1500);assert_eq!(cx.fuel_remaining(),if probe.phase==2{5}else{7});if probe.phase==2{assert_eq!(std::str::from_utf8(&probe.output).unwrap(),law["source"].as_str().unwrap());}
 }
 let mut cx=owner.context(StepBudget::new(7,1500,grant),cancel.clone(),now,&mut sequence,&mut progress).unwrap();assert_eq!(cx.retained_grant(),serde_json::from_value(law["remaining"].clone()).unwrap());drop(cx);probe.begin_close();assert!(probe.terminal_is_empty());while !owner.terminal_is_empty(){owner.close_step(TEST_RETAINED_POLICY);}
}

#[test]
fn original_job_failed_receipt_ingress_keeps_actual_external_recipient(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../⏱️context/📦️owner/⚠️failure/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let mut owner=StepContextOwner::new(OperationId(98145),Generation(17),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:StepContextOwner::birth_bytes(),maximum_depth:1,..Default::default()}).unwrap().0;
 let cancel=root_cancel_token();let mut sequence=0;let mut recipient=RetainedCloneProgress::default();let mut probe=RetainedReceiptProbe{output:Vec::new(),phase:0,panic_after_birth:false,closing:false};
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:64,maximum_depth:3};let mut cx=owner.context(StepBudget::new(7,1500,grant),cancel.clone(),now,&mut sequence,&mut recipient).unwrap();
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{probe.output.try_reserve_exact(law["producerBytes"].as_u64().unwrap()as usize).unwrap();let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:probe.output.capacity(),..Default::default()};cx.consume_retained(progress)});
 let error=result.unwrap_err();let actual=cx.retained_progress();assert_eq!(actual.retained_capacity_bytes,heap.requested_bytes);assert_eq!(heap.released_bytes,0);assert_eq!(error.retained_progress(),actual);assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(cx.retained_grant().maximum_capacity_bytes,0);assert_eq!(cx.retained_grant().maximum_items,0);assert_eq!(cx.fuel_remaining(),7);assert_eq!(cx.deadline_us(),1500);assert_eq!(cx.operation(),OperationId(98145));let pointer=probe.output.as_ptr();
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.retained_grant());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(cx);assert_eq!(recipient,actual);assert_eq!(probe.output.as_ptr(),pointer);probe.begin_close();
 let mut close_receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(7,1500,grant),cancel,now,&mut sequence,&mut close_receipt).unwrap();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||probe.close_step(&mut cx).unwrap());drop(cx);assert_eq!(close_receipt,step.progress());assert_eq!(step.progress().released_bytes,actual.retained_capacity_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,actual.retained_capacity_bytes));assert!(probe.terminal_is_empty());while !owner.terminal_is_empty(){owner.close_step(TEST_RETAINED_POLICY);}
}
