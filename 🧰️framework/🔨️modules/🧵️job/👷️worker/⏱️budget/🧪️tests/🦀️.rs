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

//#region 🔒️CallbackQuarantine
struct CompletionProbe { end: Option<u64>, closing: bool, output: Option<JobPayloadSlot> }

impl InteractiveJob for CompletionProbe {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure> {
        if self.output.is_none(){let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, b"kept").unwrap_or_else(|_| panic!("exact fixture page admission"));self.output=Some(JobPayloadSlot::from_payload(output));}
        bind_clock(self.end);
        JobOutcomeBorrow::admit_complete(cx,None,self.output.as_ref().and_then(JobPayloadSlot::original)).map_err(OriginalTurnFailure::from)
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{descriptor.complete(None,self.output.as_ref().and_then(JobPayloadSlot::original))}
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if let Some(output)=self.output.as_mut(){if !output.is_empty(){return close_original_retained_step(cx,output.retirement_demands()?,|grant|output.close_step(grant))}}Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../⏱️trace/⏱️clock/🧪️tests/🧫️fixtures/🧪️contention/🔣️.json")).unwrap();
    let operation = allocate_operation_id();
    let generation = Generation(71);
    for law in fixture["verdicts"].as_array().unwrap().iter().filter(|law| !law["start"].is_null()) {
        bind_clock(law["start"].as_u64());
        let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(operation,generation,WORKER_TEST_STOCK,now,u64::MAX,20000);let params = BatchJobParams { operation, generation, cancel: issuer.admit_alias(source), config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "microsecond-exact-quarantine", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 }, now_us: now };
        let mut session = admit_original_fixture_owner!(issuer,WorkerJobSession,CompletionProbe { end: law["end"].as_u64(), closing: false, output:None }, params).unwrap_or_else(|_| panic!("exact worker admission"));
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
            assert!(owner.authority.as_ref().unwrap().close_phase()!=OriginalWorkerAuthorityPhase::QuarantinedOutcome);
        }
        let owned_bytes=owner.job().output.as_ref().and_then(JobPayloadSlot::original).unwrap().allocated_capacity_bytes()+owner.authority.as_ref().unwrap().fault_payload_capacity_bytes();
        bind_clock(Some(law["start"].as_u64().unwrap().max(law["end"].as_u64().unwrap_or(0))+1000));
        owner.begin_close();
        
        let _ = issuer.close_owner(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()},&mut session).unwrap();
        assert!(!session.terminal_is_empty());
        let mut released_bytes = 0;
        for _ in 0..64 {
            let phase=session.close_phase();let caller=super::retained_ownership_tests::caller_worker_close_policy(phase);
            match issuer.close_owner(caller,&mut session).unwrap() {
                WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:released_items,released_bytes:bytes,..}} => { assert!(released_items <= 1 && bytes <= caller.maximum_release_bytes); if matches!(phase,WorkerJobClosePhase::QuarantinedOutcome|WorkerJobClosePhase::Outcome|WorkerJobClosePhase::PreadmittedFault|WorkerJobClosePhase::Job){assert!(bytes<=JOB_PAYLOAD_PAGE_BYTES);released_bytes += bytes;} }
                WorkerJobCloseStep::Blocked => std::thread::yield_now(),
                WorkerJobCloseStep::Complete {progress} => break,
            }
        }
        assert!(session.terminal_is_empty());
        assert!(session.terminal_is_empty());
        assert_eq!(owned_bytes, 2 * JOB_PAYLOAD_PAGE_BYTES);
        assert_eq!(released_bytes, owned_bytes);issuer.retire(source);
    }
}
//#endregion 🔒️CallbackQuarantine
