//! ⚙️ engine crates' native contract tests (a dev-dependency only).
//! 🧪️ Retained numerical payload projection, borrowed-outcome observation and close accounting shared by the fem

use semio_framework_job::Generation;

use semio_framework_job::InteractiveJob;

use semio_framework_job::InteractiveJobCloseStep;

use semio_framework_job::OriginalTurnFailure;

use semio_framework_job::JobOutcomeBorrow;

use semio_framework_job_payload::JobPayloadStream;

use semio_framework_job::Operation;

use semio_framework_job::OperationId;

use semio_framework_job::RetainedCloneGrant;

use semio_framework_job::RetainedCloneProgress;

use semio_framework_job_payload::RetainedJobPayload;

use semio_s_fem_payload_turns::{NumericalPayloadSlot, numerical_begin_page, numerical_commit_page, numerical_finish_writer, numerical_open_writer, numerical_post, numerical_writer_close, numerical_writer_demand};
use semio_framework_value::ValueError;
use semio_framework_value_testing_admission::{EffectBank, Wallet, bank_run};
use semio_s_fem_payload_turns::NumericalStep;

use semio_framework_job::RevisionId;

use semio_framework_job::StepBudget;

use semio_framework_job::StepContext;

use semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
use semio_framework_value::RetirementDemand;

/// 🎟️ The wallet one observed test step may spend.
pub const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🧭️ What one step lent to its caller, copied out page by page before the borrow is released.
#[derive(Debug, PartialEq, Eq)]
pub enum Seen {
    Yield,
    Preview(Vec<Vec<u8>>),
    Checkpoint(Vec<Vec<u8>>),
    Fault(Vec<Vec<u8>>),
    Cancelled,
    Complete(Vec<Vec<u8>>),
}

/// 📖️ Copies every admitted page of a borrowed payload.
pub fn payload_pages(payload: &RetainedJobPayload) -> Vec<Vec<u8>> {
    (0..payload.page_count()).map(|page| payload.page(page).expect("retained payload page").to_vec()).collect()
}

/// 📖️ Concatenates every admitted page of a borrowed payload before comparing native fixtures.
pub fn payload_bytes(payload: &RetainedJobPayload) -> Vec<u8> {
    payload_pages(payload).concat()
}

/// 📖️ Concatenates copied pages.
pub fn pages_bytes(pages: &[Vec<u8>]) -> Vec<u8> {
    pages.concat()
}

/// 🔭️ Copies the lent outcome out of one admitted step so the job can be driven again.
pub fn seen_of<E: std::fmt::Debug>(result: Result<Option<JobOutcomeBorrow<'_>>, E>) -> Seen {
    match result.expect("numerical step admission") {
        None | Some(JobOutcomeBorrow::Yield { .. }) => Seen::Yield,
        Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => Seen::Preview(payload_pages(payload)),
        Some(JobOutcomeBorrow::CheckpointReady { state, .. }) => Seen::Checkpoint(payload_pages(state)),
        Some(JobOutcomeBorrow::Fault { detail, .. }) => Seen::Fault(payload_pages(detail)),
        Some(JobOutcomeBorrow::Cancelled { .. }) => Seen::Cancelled,
        Some(JobOutcomeBorrow::Complete { output, .. }) => Seen::Complete(output.map(payload_pages).unwrap_or_default()),
    }
}

/// 🎟️ The exact one-item grant that covers a quoted demand.
pub fn close_grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 📏️ The job's current quoted close demand.
pub fn job_close_demand(job: &impl InteractiveJob) -> RetirementDemand {
    let copy_bytes = job.next_close_copy_byte_demand().expect("a locally owned job quotes its copy demand");
    RetirementDemand { copy_bytes, capacity_bytes: job.next_close_capacity_byte_demand(copy_bytes).expect("capacity quote"), release_bytes: job.next_close_release_byte_demand().expect("release quote"), depth: job.next_close_depth_demand().expect("depth quote") }
}

/// 🧹️ Closes a locally owned job through its own quoted grants until it is terminal-empty.
pub fn close_job(job: &mut impl InteractiveJob) {
    job.begin_close();
    for _ in 0..10_000_000 {
        if job.terminal_is_empty() {
            return;
        }
        let step = close_turn(job, close_grant(job_close_demand(job)));
        assert!(matches!(step, Ok(InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. })), "a locally owned job close has no external owner and no refusal");
    }
    panic!("the job close ladder never reached terminal-empty");
}

/// 🧹️ Closes an owned payload through its own quoted grants.
pub fn close_payload(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let demand = payload.retirement_demands().expect("a locally owned payload quotes its close demand");
        payload_close(&mut payload, demand);
    }
}

thread_local! { static BANK: std::cell::RefCell<EffectBank> = std::cell::RefCell::new(EffectBank::new(Wallet::new(RetainedCloneGrant { maximum_items: 1 << 30, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 1 << 20 }, 1 << 40))); }

/// 🎟️ Runs one body inside the genuine original effect of the shared effect bank, funded with exactly `grant`.
pub fn granted<R>(grant: RetainedCloneGrant, run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> R) -> R {
    BANK.with(|bank| {
        let mut output = None;
        bank_run(&mut bank.borrow_mut(), grant, 1 << 20, |effect| {
            output = Some(run(&mut *effect));
            semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result: Ok::<(), ValueError>(()), work: effect.performed_work() })
        });
        output.expect("the shared effect bank ran its original action")
    })
}

/// 🎟️ Closes one payload frontier inside the shared bank's original effect that grants exactly its quote.
fn payload_close(payload: &mut RetainedJobPayload, demand: RetirementDemand) -> semio_framework_job::RetainedCloneStep {
    granted(close_grant(demand), |effect| payload.close_step(effect).result).expect("a locally owned payload close has no external owner")
}

/// 🪪️ Runs one body inside a fresh single-fuel step context under the shared test wallet.
fn with_context<T>(operation: Operation, run: impl FnOnce(&mut StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let mut receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    run(&mut context)
}

/// 🔁️ Re-enters one shared funded builder turn under fresh contexts until it reports completion.
fn until_done(operation: Operation, mut turn: impl FnMut(&mut StepContext<'_>) -> Result<bool, ValueError>) {
    for _ in 0..10_000 {
        if with_context(operation, |context| turn(context)).expect("a locally owned builder turn is admitted by the test wallet") {
            return;
        }
    }
    panic!("a locally owned builder turn never completed");
}

/// 🧱️ Rebuilds an owned payload from copied pages so a restore cursor can consume it.
pub fn payload_from_pages(operation: Operation, stream: JobPayloadStream, pages: &[Vec<u8>]) -> RetainedJobPayload {
    let mut slot: NumericalPayloadSlot = None;
    let mut output = None;
    until_done(operation, |cx| numerical_open_writer(&mut slot, stream, cx));
    for page in pages {
        until_done(operation, |cx| numerical_begin_page(slot.as_deref_mut().expect("boxed builder"), cx));
        until_done(operation, |cx| { let turn = slot.as_deref_mut().expect("boxed builder").write_staged_source_funded(cx, page.as_slice()); numerical_post(cx, turn) });
        until_done(operation, |cx| numerical_commit_page(slot.as_deref_mut().expect("boxed builder"), cx));
    }
    until_done(operation, |cx| numerical_finish_writer(&mut slot, &mut output, cx));
    output.expect("rebuilt payload builder hands its sealed payload over")
}

/// 🧹️ Closes a locally owned writer slot through its own quoted grants, then releases its box.
pub fn close_writer(writer: &mut NumericalPayloadSlot) {
    let operation = test_operation(0);
    while writer.is_some() {
        let grant = match numerical_writer_demand(writer) { Some(demand) => close_grant(demand.expect("a locally owned builder quotes its close demand")), None => break };
        if granted(grant, |effect| numerical_writer_close(writer, effect)).is_none() { break; }
    }
}

/// 🧹️ Closes an owned payload and returns the page backing bytes it released, excluding the operation ledger.
pub fn close_payload_page_bytes(mut payload: RetainedJobPayload) -> usize {
    let mut released = 0;
    while !payload.terminal_is_empty() {
        let demand = payload.retirement_demands().expect("a locally owned payload quotes its close demand");
        let pages = payload.page_count();
        let step = payload_close(&mut payload, demand);
        if payload.page_count() < pages {
            released += step.progress().released_bytes;
        }
    }
    released
}

/// 🪪️ The fixed operation identity every engine contract test steps under.
pub fn test_operation(id: u64) -> Operation {
    Operation::new(OperationId(id), RevisionId(7), Generation(3), 11)
}

/// 🎟️ A test grant narrowed to `items` copied items and `bytes` released bytes.
pub fn grant_of(items: usize, bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: items, maximum_release_bytes: bytes, ..TEST_GRANT }
}

/// 🔢️ Flattens one close turn into (complete, copied items, released bytes).
pub fn tuple_of(step: InteractiveJobCloseStep) -> (bool, usize, usize) {
    let progress = step.progress();
    (matches!(step, InteractiveJobCloseStep::Complete { .. }), progress.copied_items, progress.released_bytes)
}

/// 🧱️ Rebuilds a checkpoint-state payload from copied pages.
pub fn checkpoint_payload(operation: Operation, pages: &[Vec<u8>]) -> RetainedJobPayload {
    payload_from_pages(operation, JobPayloadStream::CheckpointState, pages)
}

/// 🧮️ Drives a close ladder to completion and reports (retired bytes, every turn exact, every turn bounded, complete).
pub fn accounted_close<C>(owner: &mut C, mut step: impl FnMut(&mut C) -> InteractiveJobCloseStep, physical: impl Fn(&C) -> usize) -> (usize, bool, bool, bool) {
    let (mut retired, mut exact, mut bounded, mut complete) = (0, true, true, false);
    for _ in 0..10_000 {
        let before = physical(owner);
        let turn = step(owner);
        let delta = before - physical(owner);
        let progress = turn.progress();
        exact &= progress.released_bytes == delta || delta == 0;
        bounded &= progress.copied_items <= 1 && progress.released_bytes <= JOB_PAYLOAD_PAGE_BYTES;
        retired += delta;
        match turn {
            InteractiveJobCloseStep::Complete { .. } => { complete = true; break; }
            InteractiveJobCloseStep::Blocked => { bounded = false; break; }
            InteractiveJobCloseStep::Pending { .. } => {}
        }
    }
    (retired, exact, bounded, complete)
}

/// 🎟️ One original-context close turn of a locally owned job whose caller wallet is exactly `grant`.
pub fn close_turn<J: InteractiveJob>(job: &mut J, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
    let (mut sequence, mut receipt) = (0, RetainedCloneProgress::default());
    let mut context = StepContext::new(OperationId(0), Generation(0), StepBudget::new(u64::MAX, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    job.close_step(&mut context)
}

/// 🧫️ A committed language-agnostic fixture read once at test time, `$path` relative to the calling package's manifest
/// directory: the fixtures stay the source of truth the oracles share, without becoming compile inputs of the test unit.
#[macro_export]
macro_rules! fixture {
    ($path:literal) => {
        ::std::sync::LazyLock::new(|| ::std::fs::read_to_string(::std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join($path)).expect(concat!("committed fixture ", $path)))
    };
}

/// 🎟️ A finite native numerical source owns its original caller and decoder through every child lifetime.
pub struct OriginalNumericalFixture{caller:semio_framework_job::OriginalJobFixtureCaller,decoder:semio_framework_job::OriginalJobDecodeOwner}
impl OriginalNumericalFixture{
    pub const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:100_000_000,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:128};
    pub fn new(operation:Operation)->Self{Self{caller:semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,Self::STOCK,||Some(0),u64::MAX,100_000_000),decoder:semio_framework_job::OriginalJobDecodeOwner::empty()}}
    pub fn receipt(&self)->RetainedCloneProgress{self.caller.receipt()}
    pub fn remaining_grant(&self)->RetainedCloneGrant{self.caller.remaining_grant()}
    pub fn cancel(&self){self.caller.cancel().cancel_now()}
    pub fn turn<T>(&mut self,grant:RetainedCloneGrant,fuel:u64,deadline:u64,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{
        let remaining=self.caller.remaining_grant();let grant=RetainedCloneGrant{maximum_items:grant.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:grant.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:grant.maximum_depth.min(remaining.maximum_depth)};
        let mut result=None;self.caller.turn_with_decoder(grant,&mut self.decoder,|cx|cx.with_original_work_loan(fuel,deadline,|cx|{result=Some(run(cx)?);Ok(())}))?;Ok(result.expect("actual numerical callback retained its result"))
    }
    pub fn close_turn(&mut self,job:&mut impl InteractiveJob,grant:RetainedCloneGrant)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.turn(grant,1,u64::MAX,|cx|cx.with_original_retirement(|cx|job.close_step(cx)))}
    pub fn close(&mut self,job:&mut impl InteractiveJob){job.begin_close();for _ in 0..10_000_000{if job.terminal_is_empty(){return}let grant=close_grant(job_close_demand(job));let step=self.close_turn(job,grant).expect("same original numerical close");assert!(!matches!(step,InteractiveJobCloseStep::Blocked));}panic!("same original numerical close stalled")}
    pub fn step_seen(&mut self,job:&mut impl NumericalStep,grant:RetainedCloneGrant,fuel:u64,deadline:u64)->Seen{self.turn(grant,fuel,deadline,|cx|Ok(seen_of(job.step_plain(cx)))).expect("same original numerical observation")}
    /// 🔁️ Re-enters one shared funded builder turn under fresh original turns until it reports completion.
    pub fn until_done(&mut self,mut run:impl FnMut(&mut StepContext<'_>)->Result<bool,ValueError>){
        for _ in 0..10_000{
            if self.turn(TEST_GRANT,1,u64::MAX,|cx|run(cx).map_err(OriginalTurnFailure::from)).expect("same original builder turn"){return}
        }
        panic!("same original builder turn never completed");
    }
    pub fn open_writer(&mut self,stream:JobPayloadStream)->NumericalPayloadSlot{
        let mut slot=None;
        self.until_done(|cx|numerical_open_writer(&mut slot,stream,cx));
        slot
    }
    pub fn finish_writer(&mut self,mut slot:NumericalPayloadSlot)->RetainedJobPayload{
        let mut output=None;
        self.until_done(|cx|numerical_finish_writer(&mut slot,&mut output,cx));
        output.expect("original builder output handoff")
    }
    pub fn payload_from_pages(&mut self,stream:JobPayloadStream,pages:&[Vec<u8>])->RetainedJobPayload{
        let mut writer=self.open_writer(stream);
        for page in pages{
            self.until_done(|cx|numerical_begin_page(writer.as_deref_mut().expect("original boxed builder"),cx));
            self.until_done(|cx|{let turn=writer.as_deref_mut().expect("original boxed builder").write_staged_source_funded(cx,page.as_slice());numerical_post(cx,turn)});
            self.until_done(|cx|numerical_commit_page(writer.as_deref_mut().expect("original boxed builder"),cx));
        }
        self.finish_writer(writer)
    }
    pub fn close_payload(&mut self,payload:&mut RetainedJobPayload){while !payload.terminal_is_empty(){let demand=payload.retirement_demands().expect("original payload close quote");self.turn(close_grant(demand),1,u64::MAX,|cx|semio_framework_job::close_original_effect_step(cx,demand,|effect|semio_framework_value::RetainedWorkPoll::Complete(payload.close_step(effect)))).expect("same original payload close");}}
    pub fn close_writer(&mut self,writer:&mut NumericalPayloadSlot){while let Some(demand)=numerical_writer_demand(writer){let demand=demand.expect("original writer close quote");self.turn(close_grant(demand),1,u64::MAX,|cx|semio_framework_job::close_original_effect_step(cx,demand,|effect|semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:match numerical_writer_close(writer,effect){Some(Ok(step))=>Ok(semio_framework_job::RetainedCloneStep::Progress(step.progress())),Some(Err(error))=>Err(error),None=>Ok(semio_framework_job::RetainedCloneStep::Complete(Default::default()))},work:effect.performed_work()}))).expect("same original writer close");}}
    pub fn finish(mut self){while !self.decoder.terminal_is_empty(){let grant=self.caller.remaining_grant();self.caller.turn(grant,|cx|self.decoder.close_on_source(cx)).expect("original numerical decoder return");}self.caller.retire(Self::STOCK);}
}

/// 🔬️ Measures actual caller-thread allocations and releases without constructing an observer backing.
pub fn observe_original_numerical_allocations<T>(run:impl FnOnce()->T)->(T,(usize,usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);assert!(!heap.overflowed);(value,(heap.requested_bytes,heap.released_bytes,heap.requested_bytes.max(heap.largest_release_bytes)))}
