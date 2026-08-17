//! 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4 + `important.md`'s io
//! absorption mapping). Bookkeeping for the `jobs` WIT export (`start-job`/`step-job`/
//! `cancel-job`) plus the two well-known cold job kinds this packet absorbs from the peer W1-D
//! io mechanism: `semio.io-run`/`semio.io-sniff` — the plugin's own single-hop local
//! `io_mechanism` registry lookup, previously the guest WIT exports `io-run`/`io-sniff`.
//!
//! Every kind implemented here completes synchronously inside one `step-job` call (each hop is
//! already bounded by a local registry lookup, never an unbounded search), so `job-budget` is
//! accepted but not metered yet — a future kind that genuinely spans multiple `step-job` calls
//! (WFC, FEM solve, SfM, brep tessellation — see design-abi.md §6) will need real fuel/deadline
//! bookkeeping in `JobRecord`; not needed by anything landing in this wave.

use std::cell::RefCell;
use std::collections::HashMap;

/// 🗺️ Absorbed from the peer's guest export `io-run` (single hop, this plugin's own registry —
/// never chains into another plugin; multi-hop routing is the host's `io-run` EFFECT, not this
/// job kind).
pub const JOB_KIND_IO_RUN: &str = "semio.io-run";
/// 🗺️ Absorbed from the peer's guest export `io-sniff`.
pub const JOB_KIND_IO_SNIFF: &str = "semio.io-sniff";

pub enum JobOutcome {
    Done(Vec<u8>),
    Failed(Vec<u8>),
}

struct JobRecord {
    kind: String,
    input: Vec<u8>,
}

thread_local! {
    static JOBS: RefCell<HashMap<u64, JobRecord>> = RefCell::new(HashMap::new());
}

/// 📥️ `jobs::start-job` — records `kind`/`input` under `job`, overwriting any previous record for
/// the same id (the host never reuses a live job id, but a restarted-from-checkpoint actor may
/// legitimately replay a `start-job` for one still in flight from the caller's point of view).
pub fn start_job(job: u64, kind: &str, input: &[u8]) {
    JOBS.with(|jobs| {
        jobs.borrow_mut().insert(job, JobRecord { kind: kind.to_string(), input: input.to_vec() });
    });
}

/// 🛑️ `jobs::cancel-job` — every kind here is already synchronous/instantaneous by the time the
/// host could observe it running, so cancellation just drops the bookkeeping record.
pub fn cancel_job(job: u64) {
    JOBS.with(|jobs| {
        jobs.borrow_mut().remove(&job);
    });
}

/// ▶️ `jobs::step-job` — every absorbed kind resolves to `Done`/`Failed` on its first step (never
/// `Running`); an unknown job id or job kind is `Failed`, never a WIT-level error, since the
/// caller is asking about a specific job's outcome, not making a malformed protocol call.
pub fn step_job(job: u64) -> JobOutcome {
    let record = JOBS.with(|jobs| jobs.borrow_mut().remove(&job));
    let Some(record) = record else {
        return JobOutcome::Failed(fault_bytes("job.unknown", format!("no job registered for id {job}")));
    };
    match record.kind.as_str() {
        JOB_KIND_IO_RUN => run_io_run(&record.input),
        JOB_KIND_IO_SNIFF => run_io_sniff(&record.input),
        other => JobOutcome::Failed(fault_bytes("job.unknown-kind", format!("unknown job kind {other:?}"))),
    }
}

fn fault_bytes(code: &str, message: String) -> Vec<u8> {
    dsl::encode_fault_bytes(&semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new(code), message))
}

#[derive(serde::Deserialize)]
struct IoRunInput {
    source: String,
    target: String,
    payload: semio_framework::io_schema::IoPayload,
}

/// 🌉️ `input` is the JSON-encoded `{source, target, payload}` the WIT guest export `io-run` used
/// to take as three separate params; `Done` carries the JSON-encoded `io_schema::IoPayload`
/// result, matching the old export's ok return exactly.
fn run_io_run(input: &[u8]) -> JobOutcome {
    let Ok(IoRunInput { source, target, payload }) = serde_json::from_slice::<IoRunInput>(input) else {
        return JobOutcome::Failed(fault_bytes("job.io-run.decode", format!("invalid {JOB_KIND_IO_RUN} input")));
    };
    let fail = |message: String| JobOutcome::Failed(fault_bytes("job.io-run", message));
    let source = match semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source) {
        Ok(dialect) => dialect,
        Err(message) => return fail(message),
    };
    let target = match semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target) {
        Ok(dialect) => dialect,
        Err(message) => return fail(message),
    };
    let Some(descriptor) = semio_framework::io::io_mechanism::io_entries().into_iter().find(|entry| entry.from == source && entry.into == target) else {
        return fail(format!("no local io entry for hop {} -> {}", source.to_coordinate(), target.to_coordinate()));
    };
    let fidelity = descriptor.fidelity;
    let route = semio_framework::io_schema::IoRoute { hops: vec![descriptor], fidelity };
    match semio_framework::io::io_mechanism::io_run(&route, payload) {
        Ok(outcome) => match serde_json::to_vec(&outcome.value) {
            Ok(bytes) => JobOutcome::Done(bytes),
            Err(error) => fail(error.to_string()),
        },
        Err(error) => fail(error.message),
    }
}

/// 🔍️ `input` is the same JSON `{source, target, payload}` shape as `run_io_run`; `Done` carries a
/// single-byte `Vec<u8>` of `io_schema::Confidence::rank()` (`0..=3`) — matches the old export's
/// `u8` return, now boxed as job output bytes.
fn run_io_sniff(input: &[u8]) -> JobOutcome {
    let Ok(IoRunInput { source, target, payload }) = serde_json::from_slice::<IoRunInput>(input) else {
        return JobOutcome::Failed(fault_bytes("job.io-sniff.decode", format!("invalid {JOB_KIND_IO_SNIFF} input")));
    };
    let fail = |message: String| JobOutcome::Failed(fault_bytes("job.io-sniff", message));
    let source = match semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source) {
        Ok(dialect) => dialect,
        Err(message) => return fail(message),
    };
    let target = match semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target) {
        Ok(dialect) => dialect,
        Err(message) => return fail(message),
    };
    let carrier = semio_framework::io_schema::ArtifactDialect::from(match &payload {
        semio_framework::io_schema::IoPayload::Binary(_) => semio_framework::io_schema::CARRIER_BINARY,
        semio_framework::io_schema::IoPayload::Text(_) => semio_framework::io_schema::CARRIER_TEXT,
    });
    if source != carrier {
        return JobOutcome::Done(vec![semio_framework::io_schema::Confidence::None.rank()]);
    }
    let confidence = semio_framework::io::io_mechanism::io_identify(&payload).into_iter().find(|(dialect, _)| *dialect == target).map(|(_, confidence)| confidence).unwrap_or(semio_framework::io_schema::Confidence::None);
    JobOutcome::Done(vec![confidence.rank()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_job_on_an_unknown_id_fails_without_panicking() {
        match step_job(999) {
            JobOutcome::Failed(_) => {}
            JobOutcome::Done(_) => panic!("an unregistered job id must fail, not succeed"),
        }
    }

    #[test]
    fn cancel_job_removes_a_pending_record_so_a_later_step_fails() {
        start_job(1, JOB_KIND_IO_RUN, b"{}");
        cancel_job(1);
        match step_job(1) {
            JobOutcome::Failed(_) => {}
            JobOutcome::Done(_) => panic!("a cancelled job must not still be steppable"),
        }
    }

    #[test]
    fn step_job_on_an_unknown_kind_fails_with_a_named_fault() {
        start_job(2, "semio.not-a-real-kind", b"{}");
        match step_job(2) {
            JobOutcome::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                assert_eq!(fault.code.0, "job.unknown-kind");
            }
            JobOutcome::Done(_) => panic!("an unknown job kind must fail"),
        }
    }
}
