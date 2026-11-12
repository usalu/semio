
use super::*;
use semio_framework_test_fixtures::fixture_text;
use std::time::Instant;

pub(super) fn caller_worker_close_policy(phase:WorkerJobClosePhase)->RetainedCloneGrant{
    match phase{
        WorkerJobClosePhase::AuthorityRelease=>RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:0,maximum_release_bytes:65536,maximum_depth:64},
        _=>RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:16384,maximum_depth:64},
    }
}

use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

fn params(operation: OperationId, generation: Generation, cancel: CancelToken) -> BatchJobParams {
    BatchJobParams { operation, generation, cancel, config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "test.retained-job", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 1_000 }, now_us: logical_now_us }
}

struct ShortGrantCloseJob {
    backing: Option<Box<u8>>,
    closing: bool,
}

impl InteractiveJob for ShortGrantCloseJob {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{InteractiveJobStep::from_callback(JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from))}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        self.begin_close();if self.backing.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=RetirementDemand{release_bytes:self.next_close_release_byte_demand()?,depth:1,..Default::default()};close_original_retained_effect_step(cx,demand,|_|{drop(self.backing.take());Ok::<_,ValueError>(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))})
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
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/♻️mounted-close/🔣️.json")).unwrap();
    let profile=fixture["profiles"].as_array().unwrap().iter().find(|profile|profile["wordBytes"].as_u64()==Some(size_of::<usize>()as u64)).expect("canonical physical layout profile for this native address space");
    let expected=profile["turns"].as_array().unwrap();
    let grant = 4_096;
    assert!(grant < JOB_PAYLOAD_PAGE_BYTES, "this law is about a grant shorter than one physical page");
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_030),Generation(23),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut mounted = admit_original_fixture_owner!(issuer,MountedWorkerJobSession,ShortGrantCloseJob { backing: Some(Box::new(51)), closing: false }, params(OperationId(90_030), Generation(23), issuer.admit_alias(source)));
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
        if phase==WorkerJobClosePhase::Payload && demand.release_bytes==JOB_PAYLOAD_PAGE_BYTES{let denied=RetainedCloneGrant{maximum_release_bytes:grant,..caller};let step=issuer.close_owner(denied,&mut mounted).unwrap();assert_eq!(step.progress(),Default::default());assert_eq!(mounted.close_phase(),phase);}
        let release_grant=caller.maximum_release_bytes;
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||issuer.close_owner(caller,&mut mounted).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(heap.requested_bytes,heap.released_bytes));
        assert_eq!(step.progress().copied_bytes,demand.copy_bytes,"a close turn charges exactly the metadata copy extent it quoted");
        let kind=if matches!(step,WorkerJobCloseStep::Complete{..}){"complete"}else{"pending"};
        let actual=serde_json::json!({"phase":format!("{phase:?}"),"next":format!("{:?}",mounted.close_phase()),"kind":kind,"receipt":step.progress()});
        observed.push(actual);
        assert!(step.progress().fits(caller));
        match step {
            WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:released_items,released_bytes,..}} => {
                assert!(released_items <= 1, "a one-item grant releases at most one owner: {released_items}");
                assert!(released_bytes <= release_grant, "a close turn never spends more than its explicit physical release grant: {released_bytes}");
                if matches!(phase,WorkerJobClosePhase::Payload|WorkerJobClosePhase::Job){charged += released_bytes;}
            }
            WorkerJobCloseStep::Complete {progress} => {}
            WorkerJobCloseStep::Blocked => panic!("a mounted close on a positive grant is never blocked"),
        }
    }
    assert!(mounted.terminal_is_empty(), "every named phase completes under its exact queried physical release authority");
    assert_eq!(charged, JOB_PAYLOAD_PAGE_BYTES + size_of::<u8>(), "fault page and domain Box each spend their actual allocation extent");
    assert_eq!(observed,expected.clone(),"original exact funded phase/receipt vector; all per-turn and physical conservation checks retained");
    assert_eq!(turns, expected.len(), "canonical original phase and exact physical owner receipts each receive an admitted turn");
    assert_eq!(ladder.iter().map(|phase|format!("{phase:?}")).collect::<Vec<_>>(),fixture["phases"].as_array().unwrap().iter().map(|phase|phase.as_str().unwrap().to_string()).collect::<Vec<_>>(),"the canonical close cursor walks its named physical phases in order and never revisits one");

    issuer.retire(source);
}

/// ⏳️ A liveness bound, not a performance one: the pool worker that owns the step may be
/// descheduled for a long time on a loaded machine (a hostile panic also unwinds and captures a
/// backtrace there), so the wait is bounded by wall time rather than by a count of yields.
const WORKER_LIVENESS_BOUND: std::time::Duration = std::time::Duration::from_secs(30);

struct HostileJob {
    output: Option<JobPayloadSlot>,
    backing: Option<Box<u8>>,
    steps: Option<Arc<AtomicUsize>>,
    panic: bool,
    closing: bool,
}

impl InteractiveJob for HostileJob {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{
        fn body<'b>(this:&'b mut HostileJob,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'b>>,OriginalTurnFailure>{
            let step=this.steps.as_ref().expect("hostile step counter").fetch_add(1,AtomicOrdering::AcqRel);
            if this.panic{panic!("hostile worker panic");}
            if step==0{return JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from);}
            JobOutcomeBorrow::admit_complete(cx,None,None).map_err(OriginalTurnFailure::from)
        }
        InteractiveJobStep::from_callback(body(self,cx))
    }

    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::Complete=>descriptor.complete(None,self.output.as_ref().and_then(JobPayloadSlot::original)),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"hostile fixture requires original yielded or terminal descriptor"))}}
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        self.begin_close();if let Some(output)=self.output.as_mut(){if !output.is_empty(){return close_original_effect_step(cx,output.retirement_demands()?,|effect|semio_framework_value::RetainedWorkPoll::Complete(output.close_step(effect)))}}
        if self.backing.is_none()&&self.steps.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=RetirementDemand{copy_bytes:self.next_close_copy_byte_demand()?,release_bytes:self.next_close_release_byte_demand()?,depth:1,..Default::default()};close_original_retained_effect_step(cx,demand,|_|{
            let progress=if let Some(backing)=self.backing.take(){drop(backing);RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Box<u8>>(),released_bytes:size_of::<u8>(),..Default::default()}}
            else{let steps=self.steps.take().expect("original hostile counter remains retained");let released_bytes=if let Some(counter)=Arc::into_inner(steps){drop(counter);semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>()}else{0};RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Arc<AtomicUsize>>(),released_bytes,..Default::default()}};
            Ok::<_,ValueError>(RetainedCloneStep::Progress(progress))
        })
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.copy_bytes);}}Ok(if self.backing.is_some(){size_of::<Box<u8>>()}else if self.steps.is_some(){size_of::<Arc<AtomicUsize>>()}else{0})}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.release_bytes);}}Ok(if self.backing.is_some(){size_of::<u8>()}else if self.steps.is_some(){semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>()}else{0})}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.depth);}}Ok(usize::from(self.backing.is_some()||self.steps.is_some()))}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.backing.is_none() && self.steps.is_none() && self.output.as_ref().is_none_or(JobPayloadSlot::is_empty)
    }
}

fn acknowledge_yield(issuer:&mut OriginalJobFixtureCaller,mut acknowledge:impl FnMut(&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>){
    let mut acknowledged=false;
    for _ in 0..16{if issuer.turn(crate::component::TEST_RETAINED_POLICY,&mut acknowledge).unwrap(){acknowledged=true;break}}
    assert!(acknowledged,"a funded caller acknowledges the original outcome within its finite turns");
}

#[test]
fn worker_authority_keeps_one_heap_identity_through_mounted_step_and_checkout() {
    let _slots = super::worker_session_slots_shared();
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_011),Generation(17),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut mounted = admit_original_fixture_owner!(issuer,MountedWorkerJobSession,
        HostileJob { output:None, backing: Some(Box::new(73)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false },
        params(OperationId(90_011), Generation(17), issuer.admit_alias(source)),
    );
    assert_eq!(size_of::<OriginalWorkerJobAuthority<HostileJob>>(),size_of::<Vec<()>>());
    let admitted_identity = unsafe { (&*mounted.session.inner.authority.get()).as_ref().expect("idle session owns its authority").native_identity() };
    assert_eq!(issuer.turn(source,|cx|mounted.pump_one_for_interactive_turn(cx)).unwrap(),WorkerJobPoll::Outcome);
    let checked_out_identity = mounted.checked_out.as_ref().and_then(|outcome| outcome.authority.as_ref()).expect("mounted outcome owns exact authority").native_identity();
    assert_eq!(checked_out_identity, admitted_identity);
    let original_receipt=mounted.take_checked_out_retained_step_receipt().expect("original mounted physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(mounted.checked_out_outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    acknowledge_yield(&mut issuer,|cx|mounted.acknowledge_checked_out_outcome(cx));
    mounted.resume().expect("empty yielded outcome returns the same authority");
    assert_eq!(issuer.turn(source,|cx|mounted.pump_one_for_interactive_turn(cx)).unwrap(),WorkerJobPoll::Terminal,"the original caller runs the step and checks its terminal out in one pump");
    let receipt=mounted.take_checked_out_retained_step_receipt().expect("original mounted terminal turn receipt");assert!(receipt.1.fits(receipt.0));assert!(mounted.take_checked_out_retained_step_receipt().is_none());
    let terminal_identity = mounted.checked_out.as_ref().and_then(|outcome| outcome.authority.as_ref()).expect("mounted terminal owns exact authority").native_identity();
    assert_eq!(terminal_identity, admitted_identity, "every original caller step keeps the same heap authority");
    mounted.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !mounted.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(mounted.close_phase()),&mut mounted).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn worker_session_held_outcome_refuses_another_step_then_take_resume_terminal_drop_and_close_are_exact() {
    let _slots = super::worker_session_slots_shared();
    let operation = OperationId(90_004);
    let generation = Generation(10);
    let steps = Arc::new(AtomicUsize::new(0));
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(operation,generation,WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut session =
        admit_original_fixture_owner!(issuer,WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(91)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(operation, generation, issuer.admit_alias(source)));
    let (first,poll) = issuer.turn(source,|cx|session.try_step_on_original_caller(cx)).unwrap().expect("first opportunity runs on its original caller");
    assert_eq!(poll,WorkerJobPoll::Outcome);
    assert!(issuer.turn(source,|cx|session.try_step_on_original_caller(cx)).unwrap().is_none(),"a held outcome admits no second opportunity");
    let mut first_owner = session.take_outcome(first).expect("first exact outcome");
    let original_receipt=first_owner.take_retained_step_receipt().expect("original caller physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(first_owner.outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    acknowledge_yield(&mut issuer,|cx|first_owner.acknowledge_outcome(cx));
    first_owner.resume().unwrap_or_else(|_| panic!("yield owner resumes exact generation"));
    let (second,poll) = issuer.turn(source,|cx|session.try_step_on_original_caller(cx)).unwrap().expect("second opportunity runs on its original caller");
    assert_eq!(poll,WorkerJobPoll::Terminal);
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
        let _ = issuer.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
    }
    assert_eq!(steps.load(AtomicOrdering::Acquire), 2);

    issuer.retire(source);
}

#[test]
fn denied_original_caller_grant_keeps_exact_job_idle_before_any_step() {
    let _slots = super::worker_session_slots_shared();
    let backing = Box::new(33u8);
    let backing_pointer = backing.as_ref() as *const u8;
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_005),Generation(11),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let steps=Arc::new(AtomicUsize::new(0));
    let mut session = admit_original_fixture_owner!(issuer,WorkerJobSession,HostileJob { output:None, backing: Some(backing), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(OperationId(90_005), Generation(11), issuer.admit_alias(source)));
    for denied in [RetainedCloneGrant{maximum_items:0,..source},RetainedCloneGrant{maximum_depth:0,..source}]{
        assert!(issuer.turn(denied,|cx|session.try_step_on_original_caller(cx)).unwrap().is_none());
        assert_eq!(session.poll(), WorkerJobPoll::Idle);
    }
    assert_eq!(steps.load(AtomicOrdering::Acquire),0,"a denied original caller never enters the job");
    assert_eq!(unsafe{(&*session.inner.authority.get()).as_ref().expect("idle session owns its authority").job().backing.as_deref().expect("idle hostile backing")as*const u8},backing_pointer);
    session.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn worker_terminal_step_raises_one_durable_wake_intent_after_its_terminal() {
    let _slots = super::worker_session_slots_shared();
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_006),Generation(12),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut session = admit_original_fixture_owner!(issuer,WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(1)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_006), Generation(12), issuer.admit_alias(source)));
    session.register_wake(Waker::noop()).expect("quiet wake registration");
    let _ = session.take_wake();
    let (first,poll) = issuer.turn(source,|cx|session.try_step_on_original_caller(cx)).unwrap().expect("first opportunity");
    assert_eq!(poll,WorkerJobPoll::Outcome);
    let mut owner = session.take_outcome(first).expect("first exact outcome");
    acknowledge_yield(&mut issuer,|cx|owner.acknowledge_outcome(cx));
    owner.resume().unwrap_or_else(|_| panic!("yield owner resumes exact generation"));
    let _ = session.take_wake();
    let (_,poll) = issuer.turn(source,|cx|session.try_step_on_original_caller(cx)).unwrap().expect("terminal opportunity");
    assert_eq!(poll,WorkerJobPoll::Terminal);
    assert!(session.take_wake(), "the terminal step raised its durable wake intent");
    assert_eq!(session.poll(), WorkerJobPoll::Terminal, "a raised wake intent is published after the terminal it announces");
    assert!(!session.take_wake(), "redundant quiet poll raises no wake");
    let mut terminal = session.take_terminal().expect("terminal is retained");let original_receipt=terminal.take_retained_step_receipt().expect("original terminal physical receipt");assert!(original_receipt.1.fits(original_receipt.0));
    assert!(matches!(terminal.outcome().unwrap(), Some(JobOutcomeView::Complete{..})));
    terminal.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn worker_quiet_wake_sequence_exhaustion_is_permanent_and_typed() {
    let _slots = super::worker_session_slots_shared();
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_009),Generation(15),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut session = admit_original_fixture_owner!(issuer,WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(2)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_009), Generation(15), issuer.admit_alias(source)));
    session.inner.wake_sequence.store(u64::MAX, Ordering::Release);
    session.inner.wake_pending.store(false, Ordering::Release);
    session.inner.raise_wake();
    assert_eq!(session.register_wake(Waker::noop()), Err(WorkerJobContention::WakeExhausted(Generation(15))));
    session.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn batch_session_advances_exactly_one_external_opportunity() {
    let steps = Arc::new(AtomicUsize::new(0));
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_007),Generation(13),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut batch = admit_original_fixture_owner!(issuer,BatchJobSession,HostileJob { output:None, backing: Some(Box::new(7)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(OperationId(90_007), Generation(13), issuer.admit_alias(source)));
    assert_eq!(issuer.turn(source,|cx|batch.step_on_original_caller(cx)).unwrap(),WorkerJobPoll::Outcome);
    assert_eq!(steps.load(AtomicOrdering::Acquire), 1);
    assert!(batch.checkout_outcome());
    let original_receipt=batch.take_checked_out_retained_step_receipt().expect("original batch physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(batch.checked_out_outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    acknowledge_yield(&mut issuer,|cx|batch.acknowledge_outcome(cx));
    batch.resume().expect("caller explicitly resumes after first opportunity");
    assert_eq!(steps.load(AtomicOrdering::Acquire), 1, "batch adapter never drains itself to terminal");
    batch.begin_close();
    assert_eq!(issuer.close_owner(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()},&mut batch).unwrap(), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !batch.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release() {
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_010),Generation(16),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut batch = admit_original_fixture_owner!(issuer,BatchJobSession,HostileJob { output:None, backing: Some(Box::new(7)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_010), Generation(16), issuer.admit_alias(source)));
    assert_eq!(issuer.turn(source,|cx|batch.step_on_original_caller(cx)).unwrap(),WorkerJobPoll::Outcome);
    assert!(batch.checkout_outcome());
    assert_eq!(issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap(), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:1,released_bytes:0,..RetainedCloneProgress::default()}});
    let receipt=batch.session.take_retained_step_receipt().unwrap().expect("original checked out receipt follows same-owner close handback");assert!(receipt.1.fits(receipt.0));assert!(batch.session.take_retained_step_receipt().unwrap().is_none());
    let header=issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap();
    assert_eq!(header,WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()}});
    assert_eq!(batch.close_phase(),WorkerJobClosePhase::Outcome);
    let presence=issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap();
    assert_eq!(presence,WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()}});
    assert_eq!(batch.close_phase(),WorkerJobClosePhase::BeginClose);
    assert_eq!(issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap(), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !batch.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned() {
    let _slots = super::worker_session_slots_exclusive();let source=crate::component::TEST_RETAINED_POLICY;let mut rejected_caller=OriginalJobFixtureCaller::admit(OperationId(92_000),Generation(500),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    for _ in 0..WORKER_JOB_SESSION_SLOTS * 64 {
        if !worker_job_retirements_are_parked() {
            break;
        }
        let _ = rejected_caller.turn(caller_worker_close_policy(worker_job_retirement_phase(WORKER_JOB_SESSION_SLOTS).expect("parked original owner")),|cx|pump_worker_job_retirements(WORKER_JOB_SESSION_SLOTS,cx)).unwrap();
    }
    assert!(!worker_job_retirements_are_parked(), "earlier tests' dropped sessions retire before this test owns every slot");
    let mut sessions = Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
    for index in 0..WORKER_JOB_SESSION_SLOTS {
        let job = HostileJob { output:None, backing: Some(Box::new(index as u8)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false };
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(91_000+index as u64),Generation(index as u64+1),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);let session=admit_original_fixture_owner!(caller,WorkerJobSession,job, params(OperationId(91_000 + index as u64), Generation(index as u64 + 1), caller.admit_alias(source)));sessions.push((session,caller));
    }
    let rejected_backing = Box::new(211u8);
    let mut rejected_job=Some(HostileJob{output:None,backing:Some(rejected_backing),steps:Some(Arc::new(AtomicUsize::new(0))),panic:false,closing:false});
    let mut rejected_params=Some(params(OperationId(92_000),Generation(500),rejected_caller.admit_alias(source)));
    let mut rejected_admission=WorkerJobSessionAdmission::<HostileJob,WorkerJobSession<HostileJob>>::new();
    let mut rejected_output=None;
    for _ in 0..crate::component::worker_fixture_admission_turns(){
        if rejected_admission.phase()=="slot"{break}
        assert!(!rejected_caller.turn(source,|cx|rejected_admission.advance(&mut rejected_job,&mut rejected_params,&mut rejected_output,cx)).unwrap());
    }
    assert_eq!(rejected_admission.phase(),"slot","every retirement slot is owned, so the extra birth waits for a returned slot");
    for _ in 0..8{assert!(!rejected_caller.turn(source,|cx|rejected_admission.advance(&mut rejected_job,&mut rejected_params,&mut rejected_output,cx)).unwrap());assert_eq!(rejected_admission.phase(),"slot");}
    assert!(rejected_output.is_none());
    let (dropped,mut dropped_caller) = sessions.pop().expect("last fixed session");
    drop(dropped);
    assert!(take_worker_job_retirement_wake());
    for _ in 0..8 {
        let _ = dropped_caller.turn(caller_worker_close_policy(worker_job_retirement_phase(1).expect("parked original owner")),|cx|pump_worker_job_retirements(1,cx)).unwrap();
    }
    let close_started=Instant::now();let mut close_attempts=0;
    while worker_job_retirements_are_parked(){
        close_attempts+=1;assert!(close_attempts<=65536&&close_started.elapsed()<WORKER_LIVENESS_BOUND,"parked original owner retained beyond finite caller control");
        let phase=worker_job_retirement_phase(1).expect("parked original owner phase");
        let step=dropped_caller.turn(caller_worker_close_policy(phase),|cx|pump_worker_job_retirements(1,cx)).unwrap();assert!(step.progress().fits(caller_worker_close_policy(phase)));
    }
    dropped_caller.retire(source);
    let mut admitted=false;
    for _ in 0..WORKER_JOB_SESSION_SLOTS*64{if rejected_caller.turn(source,|cx|rejected_admission.advance(&mut rejected_job,&mut rejected_params,&mut rejected_output,cx)).unwrap(){admitted=true;break}}
    assert!(admitted,"the same original rejected owners enter the returned slot");
    let replacement=rejected_output.take().expect("returned slot admits the waiting birth");
    assert!(rejected_job.is_none()&&rejected_params.is_none()&&rejected_admission.terminal_is_empty());
    drop(rejected_admission);
    sessions.push((replacement,rejected_caller));
    for (mut session,mut caller) in sessions {
        let _ = session.begin_close();
        let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
            let _ = caller.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
        }
        caller.retire(source);
    }
}

#[test]
fn hostile_retained_box_and_arc_require_complete_independent_physical_grants(){
    let source=crate::component::TEST_RETAINED_POLICY;let mut caller=OriginalJobFixtureCaller::admit(OperationId(97842),Generation(1),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut job=HostileJob{output:None,backing:Some(Box::new(9)),steps:Some(Arc::new(AtomicUsize::new(0))),panic:false,closing:false};
    let pointer=job.backing.as_deref().unwrap() as *const u8;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:64,maximum_depth:64};
    let denied=RetainedCloneGrant{maximum_release_bytes:0,..grant};
    assert_eq!(caller.turn(denied,|cx|job.close_step(cx)).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(job.backing.as_deref().unwrap() as *const u8,pointer);
    let box_step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert!(box_step.progress().fits(grant));
    assert_eq!(box_step.progress().released_bytes,1);
    let arc_step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert!(arc_step.progress().fits(grant));
    assert_eq!(arc_step.progress().released_bytes,semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>());
    assert!(matches!(caller.turn(grant,|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Complete{..}));assert!(job.terminal_is_empty());
    let shared=Arc::new(AtomicUsize::new(17));
    let mut job=HostileJob{output:None,backing:None,steps:Some(Arc::clone(&shared)),panic:false,closing:false};
    let step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert_eq!(step.progress().released_bytes,0);assert_eq!(shared.load(AtomicOrdering::Acquire),17);
    assert!(matches!(caller.turn(grant,|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Complete{..}));assert!(job.terminal_is_empty());
    caller.retire(source);
}

#[test]
fn mounted_original_cancel_alias_returns_only_to_its_still_borrowed_matching_root(){
 let law:serde_json::Value=serde_json::from_str(include_str!("cancel-return.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("cancel-return.schema.json")).unwrap();for(k,v)in law.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v)}let _slots=super::worker_session_slots_shared();let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_031),Generation(23),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);let mut mounted=admit_original_fixture_owner!(issuer,MountedWorkerJobSession,ShortGrantCloseJob{backing:Some(Box::new(51)),closing:false},params(OperationId(90_031),Generation(23),issuer.admit_alias(source)));mounted.begin_close();let mut handbacks=0;
 for _ in 0..64{let phase=mounted.close_phase();if phase==WorkerJobClosePhase::Empty{break;}let grant=caller_worker_close_policy(phase);if let Some(step)=issuer.turn(grant,|cx|{if mounted.has_original_cancel_alias_witness(cx.original_cancel_token()).unwrap(){let denied=RetainedCloneGrant{maximum_items:0,..cx.retained_grant()};let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_effect(denied,|cx,effect|mounted.return_original_cancel_alias_step(cx.original_cancel_token(),effect)).unwrap());if let Some((turn,_))=turn{assert_eq!(turn.work,u128::from(law["work"]["denied"].as_u64().unwrap()));assert!(turn.result.unwrap().is_none_or(|step|step.progress()==Default::default()));}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!heap.overflowed);}let turn=cx.with_original_effect(cx.retained_grant(),|cx,effect|mounted.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn);let step=turn.result.as_ref().ok().copied().flatten();let expected=if step.is_some_and(|step|step.progress()!=Default::default()){law["work"]["returned"].as_u64().unwrap()}else{law["work"]["ineligible"].as_u64().unwrap()};assert_eq!(turn.work,u128::from(expected));let result=turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(|error|match error{WorkerJobDemandError::Refused(error)=>error,WorkerJobDemandError::Contention(_)=>ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original mounted witness remains checked out")});receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:turn.work})?;Ok(step)}).unwrap(){let observed=step.progress();assert!(observed.fits(grant));assert_eq!(observed,RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()});assert!(matches!(step,RetainedCloneStep::Complete(_)));handbacks+=1;}else{let step=issuer.turn(grant,|cx|mounted.close_step(cx)).unwrap();assert!(step.progress().fits(grant));assert!(!matches!(step,WorkerJobCloseStep::Blocked));}}
 assert!(mounted.terminal_is_empty());assert_eq!(handbacks,law["handbacks"].as_u64().unwrap());issuer.turn(source,|cx|{let turn=cx.with_original_effect(cx.retained_grant(),|cx,effect|mounted.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn);assert_eq!(turn.work,u128::from(law["work"]["cached"].as_u64().unwrap()));assert!(turn.result.unwrap().is_none());Ok(())}).unwrap();let released=issuer.retire(source);assert!(released>0);
}
