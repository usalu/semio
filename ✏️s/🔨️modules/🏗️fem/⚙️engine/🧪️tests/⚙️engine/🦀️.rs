//! 🧪️ Retained numerical payload projection and borrowed-outcome observation for native contract tests.

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RetainedJobPayloadWriter, StepBudget, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};

/// 🎟️ The wallet one observed test step may spend.
pub(crate) const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🧭️ What one step lent to its caller, copied out page by page before the borrow is released.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Seen {
    Yield,
    Preview(Vec<Vec<u8>>),
    Checkpoint(Vec<Vec<u8>>),
    Fault(Vec<Vec<u8>>),
    Cancelled,
    Complete(Vec<Vec<u8>>),
}

/// 📖️ Copies every admitted page of a borrowed payload.
pub(crate) fn payload_pages(payload: &RetainedJobPayload) -> Vec<Vec<u8>> {
    (0..payload.page_count()).map(|page| payload.page(page).expect("retained payload page").to_vec()).collect()
}

/// 📖️ Concatenates every admitted page of a borrowed payload before comparing native fixtures.
pub(crate) fn payload_bytes(payload: &RetainedJobPayload) -> Vec<u8> {
    payload_pages(payload).concat()
}

/// 📖️ Concatenates copied pages.
pub(crate) fn pages_bytes(pages: &[Vec<u8>]) -> Vec<u8> {
    pages.concat()
}

/// 🔭️ Copies the lent outcome out of one admitted step so the job can be driven again.
pub(crate) fn seen_of(result: Result<Option<JobOutcomeBorrow<'_>>, ValueError>) -> Seen {
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
pub(crate) fn close_grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 📏️ The job's current quoted close demand.
pub(crate) fn job_close_demand(job: &impl InteractiveJob) -> RetirementDemand {
    let copy_bytes = job.next_close_copy_byte_demand().expect("a locally owned job quotes its copy demand");
    RetirementDemand { copy_bytes, capacity_bytes: job.next_close_capacity_byte_demand(copy_bytes).expect("capacity quote"), release_bytes: job.next_close_release_byte_demand().expect("release quote"), depth: job.next_close_depth_demand().expect("depth quote") }
}

/// 🧹️ Closes a locally owned job through its own quoted grants until it is terminal-empty.
pub(crate) fn close_job(job: &mut impl InteractiveJob) {
    job.begin_close();
    for _ in 0..10_000_000 {
        if job.terminal_is_empty() {
            return;
        }
        let step = job.close_step(close_grant(job_close_demand(job)));
        assert!(!matches!(step, InteractiveJobCloseStep::Refused { .. } | InteractiveJobCloseStep::Blocked), "a locally owned job close has no external owner and no refusal");
    }
    panic!("the job close ladder never reached terminal-empty");
}

/// 🧹️ Closes an owned payload through its own quoted grants.
pub(crate) fn close_payload(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let demand = payload.retirement_demands().expect("a locally owned payload quotes its close demand");
        payload.close_step(close_grant(demand)).expect("a locally owned payload close has no external owner");
    }
}

/// 🧱️ Rebuilds an owned payload from copied pages so a restore cursor can consume it.
pub(crate) fn payload_from_pages(operation: Operation, stream: JobPayloadStream, pages: &[Vec<u8>]) -> RetainedJobPayload {
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
pub(crate) fn close_writer(writer: &mut RetainedJobPayloadWriter) {
    writer.begin_close();
    while !writer.terminal_is_empty() {
        let demand = writer.retirement_demands().expect("a locally owned writer quotes its close demand");
        writer.close_step(close_grant(demand)).expect("a locally owned writer close has no external owner");
    }
}

/// 🧹️ Closes an owned payload and returns the page backing bytes it released, excluding the operation ledger.
pub(crate) fn close_payload_page_bytes(mut payload: RetainedJobPayload) -> usize {
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
pub(crate) fn close_writer_released(writer: &mut RetainedJobPayloadWriter) -> usize {
    writer.begin_close();
    let mut released = 0;
    while !writer.terminal_is_empty() {
        let demand = writer.retirement_demands().expect("a locally owned writer quotes its close demand");
        released += writer.close_step(close_grant(demand)).expect("a locally owned writer close has no external owner").progress().released_bytes;
    }
    released
}
