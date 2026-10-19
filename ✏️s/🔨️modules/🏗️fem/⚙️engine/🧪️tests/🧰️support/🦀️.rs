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

use semio_framework_job_payload::RetainedJobPayloadWriter;

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
        payload.close_step(close_grant(demand)).expect("a locally owned payload close has no external owner");
    }
}

/// 🧱️ Rebuilds an owned payload from copied pages so a restore cursor can consume it.
pub fn payload_from_pages(operation: Operation, stream: JobPayloadStream, pages: &[Vec<u8>]) -> RetainedJobPayload {
    let mut writer = RetainedJobPayloadWriter::new(stream);
    let mut sequence = 0;
    for page in pages {
        let mut receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
        writer.begin_staged_page(&mut context).expect("rebuilt payload page admission");
        writer.write_staged(page).expect("rebuilt payload page bytes");
        writer.commit_staged_page().expect("rebuilt payload page commit");
    }
    writer.finish().unwrap_or_else(|_| panic!("rebuilt payload writer finishes committed pages"))
}

/// 🧹️ Closes a locally owned writer through its own quoted grants.
pub fn close_writer(writer: &mut RetainedJobPayloadWriter) {
    writer.begin_close();
    while !writer.terminal_is_empty() {
        let demand = writer.retirement_demands().expect("a locally owned writer quotes its close demand");
        writer.close_step(close_grant(demand)).expect("a locally owned writer close has no external owner");
    }
}

/// 🧹️ Closes an owned payload and returns the page backing bytes it released, excluding the operation ledger.
pub fn close_payload_page_bytes(mut payload: RetainedJobPayload) -> usize {
    let mut released = 0;
    while !payload.terminal_is_empty() {
        let demand = payload.retirement_demands().expect("a locally owned payload quotes its close demand");
        let pages = payload.page_count();
        let step = payload.close_step(close_grant(demand)).expect("a locally owned payload close has no external owner");
        if payload.page_count() < pages {
            released += step.progress().released_bytes;
        }
    }
    released
}

/// 🧹️ Closes a locally owned writer and returns every byte its close turns released.
pub fn close_writer_released(writer: &mut RetainedJobPayloadWriter) -> usize {
    writer.begin_close();
    let mut released = 0;
    while !writer.terminal_is_empty() {
        let demand = writer.retirement_demands().expect("a locally owned writer quotes its close demand");
        released += writer.close_step(close_grant(demand)).expect("a locally owned writer close has no external owner").progress().released_bytes;
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
    pub fn step_seen(&mut self,job:&mut impl InteractiveJob,grant:RetainedCloneGrant,fuel:u64,deadline:u64)->Seen{self.turn(grant,fuel,deadline,|cx|Ok(seen_of(job.step(cx)))).expect("same original numerical observation")}
    pub fn payload_from_pages(&mut self,stream:JobPayloadStream,pages:&[Vec<u8>])->RetainedJobPayload{
        let mut writer=None;
        self.turn(TEST_GRANT,1,u64::MAX,|cx|{writer=RetainedJobPayloadWriter::new_funded(stream,cx)?;Ok(())}).expect("original writer header birth");
        assert!(writer.is_some());
        for page in pages{
            assert!(self.turn(TEST_GRANT,1,u64::MAX,|cx|writer.as_mut().unwrap().begin_staged_page_funded(cx).map_err(Into::into)).expect("original staged page birth"));
            assert!(self.turn(TEST_GRANT,1,u64::MAX,|cx|writer.as_mut().unwrap().write_staged_source_funded(cx,page.as_slice(),0).map_err(Into::into)).expect("original staged source copy"));
            assert!(self.turn(TEST_GRANT,1,u64::MAX,|cx|writer.as_mut().unwrap().commit_staged_page_funded(cx,0).map_err(Into::into)).expect("original staged page commit"));
        }
        let mut output=None;
        assert!(self.turn(TEST_GRANT,1,u64::MAX,|cx|RetainedJobPayloadWriter::finish_funded(&mut writer,&mut output,cx).map_err(Into::into)).expect("original writer output handoff"));
        output.unwrap()
    }
    pub fn close_payload(&mut self,payload:&mut RetainedJobPayload){while !payload.terminal_is_empty(){let demand=payload.retirement_demands().expect("original payload close quote");self.turn(close_grant(demand),1,u64::MAX,|cx|semio_framework_job::close_original_retained_step(cx,demand,|grant|payload.close_step(grant))).expect("same original payload close");}}
    pub fn close_writer(&mut self,writer:&mut RetainedJobPayloadWriter){writer.begin_close();while !writer.terminal_is_empty(){let demand=writer.retirement_demands().expect("original writer close quote");self.turn(close_grant(demand),1,u64::MAX,|cx|semio_framework_job::close_original_retained_step(cx,demand,|grant|writer.close_step(grant))).expect("same original writer close");}}
    pub fn finish(mut self){while !self.decoder.terminal_is_empty(){let grant=self.caller.remaining_grant();self.caller.turn(grant,|cx|semio_framework_job::close_original_retained_step(cx,self.decoder.retirement_demands(grant.maximum_copy_bytes)?,|grant|self.decoder.close_step(grant))).expect("original numerical decoder return");}self.caller.retire(Self::STOCK);}
}

/// 🔬️ Measures actual caller-thread allocations and releases without constructing an observer backing.
pub fn observe_original_numerical_allocations<T>(run:impl FnOnce()->T)->(T,(usize,usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);assert!(!heap.overflowed);(value,(heap.requested_bytes,heap.released_bytes,heap.requested_bytes.max(heap.largest_release_bytes)))}
