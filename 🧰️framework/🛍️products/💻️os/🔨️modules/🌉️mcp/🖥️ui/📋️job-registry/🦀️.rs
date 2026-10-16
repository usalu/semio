//! 📋️ Job registry of the wire crate: JSON-RPC and MCP wire types, registries, the backend port and the notification lane.

use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;
use crate::notify::{bind_job_to_active_scope, job_progress_changed, release_job_binding};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_foundation::handles::{HandleKind, mint_id};

//#region 🔖️Clock

/// 🕐️ Wall-clock milliseconds — same one-real-clock-read-site convention `🦀️.rs::now_ms`
/// and every other facet's own copy already follow (each facet stays independently testable).
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

//#endregion 🔖️Clock

//#region 🔖️JobRegistry

/// 🚦 One plugin-agnostic job's lifecycle — `Pending` (registered, not yet started), `Running`
/// (progress may update), then exactly one terminal state.
// 🌱️ `rename_all = "SCREAMING_SNAKE_CASE"` has no `#[value(rename_all = …)]` equivalent — spelled
// out per-variant instead, same wire names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToValue, FromValue)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    #[value(rename = "PENDING")]
    Pending,
    #[value(rename = "RUNNING")]
    Running,
    #[value(rename = "AWAITING_APPROVAL")]
    AwaitingApproval,
    #[value(rename = "SUCCEEDED")]
    Succeeded,
    #[value(rename = "FAILED")]
    Failed,
    #[value(rename = "CANCELLED")]
    Cancelled,
}

impl JobStatus {
    fn is_terminal(self) -> bool {
        matches!(self, JobStatus::Succeeded | JobStatus::Failed | JobStatus::Cancelled)
    }
}

struct JobRecord {
    pub kind: String,
    pub status: JobStatus,
    pub progress: Option<f64>,
    pub message: Option<String>,
    pub result: Option<serde_json::Value>,
    pub error: Option<GatewayError>,
    pub cancel_requested: bool,
    on_cancel: Option<Box<dyn FnOnce() + Send>>,
    pub events: Vec<JobEvent>,
    next_ordinal: u64,
}

impl JobRecord {
    pub fn append(&mut self, kind: &str, progress: Option<f64>, message: Option<String>) {
        if self.events.len() == JOB_EVENT_JOURNAL_CAPACITY {
            self.events.remove(0);
        }
        self.next_ordinal += 1;
        self.events.push(JobEvent { ordinal: self.next_ordinal, kind: kind.to_string(), at_ms: now_ms(), progress, message });
    }
}

/// 🧾️ How many journal rows one job retains — the oldest row leaves first, and a reader behind the
/// window sees the gap in the returned rows' own `ordinal`s rather than a silently renumbered stream.
pub const JOB_EVENT_JOURNAL_CAPACITY: usize = 256;

/// 🗓️ One append-only lifecycle fact of one job, ordered by `ordinal`: `accepted`, `progress`,
/// `cancel-requested`, `awaiting-approval`, `approved`, and exactly one terminal `succeeded`/`failed`/
/// `cancelled` — plus whatever domain facts the job's own producer records. The journal is the job's
/// event stream; `JobSnapshot` is only its current fold.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobEvent {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// 📃️ One cursor-paged read of a job's journal: every row after `after`, and the cursor to read on from.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobEventPage {
    pub events: Vec<JobEvent>,
    pub next_cursor: u64,
}

/// 📸️ `job_get`/`semio://job/{id}`'s answer shape — real fields only, never fabricated: `progress`/
/// `message`/`result`/`error` are `None` until the producer that owns this job id reports them.
#[derive(Debug, Clone)]
pub struct JobSnapshot {
    pub job_id: String,
    pub kind: String,
    pub status: JobStatus,
    pub progress: Option<f64>,
    pub message: Option<String>,
    pub result: Option<serde_json::Value>,
    pub error: Option<GatewayError>,
    pub cancel_requested: bool,
}

impl JobSnapshot {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "jobId": self.job_id,
            "kind": self.kind,
            "status": self.status,
            "progress": self.progress,
            "message": self.message,
            "result": self.result,
            "error": self.error.as_ref().map(GatewayError::to_tool_error_payload),
            "cancelRequested": self.cancel_requested,
        })
    }
}

/// 🧵️ The plugin-agnostic job seam `job_get`/`job_cancel` read/act through, and every job producer
/// (W5's inference-as-a-job, a future tessellation/export job, …) mints ids into and reports
/// progress/results into — see [`job_registry`] for the one process-wide instance every tool
/// handler and resource reader in this file shares. Cancellation is cooperative: `request_cancel`
/// flips `cancel_requested` (and, for a `Pending` job with no producer running yet, finishes it
/// immediately — there is nothing to interrupt); a `Running` job's own producer is expected to poll
/// [`JobRegistry::is_cancel_requested`] and call [`JobRegistry::mark_cancelled`] once it has
/// actually stopped. This facet never force-kills work it does not own.
pub struct JobRegistry {
    jobs: Mutex<HashMap<String, JobRecord>>,
}

impl Default for JobRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl JobRegistry {
    pub fn new() -> Self {
        Self { jobs: Mutex::new(HashMap::new()) }
    }

    /// 🆕️ Mints a fresh `job_`-prefixed id (same `mint_id`/`HandleKind::Job` scheme `🎫️handles`
    /// already reserves for jobs) and registers it `Pending`.
    pub fn begin(&self, kind: &str) -> String {
        self.begin_with_id(mint_id(HandleKind::Job, now_ms()), kind)
    }

    /// 🆕️ Registers `job_id` `Pending` under an id the caller already minted elsewhere — the seam a
    /// producer that ALSO wants `crate::handles::HandleTable`'s session-owned/expiring handle (for
    /// its authorization/TTL semantics, which this registry does not provide) uses to track the SAME
    /// id's mutable progress/result here: mint the `HandleTable` entry first with
    /// `HandleKind::Job`/`mint_id`, then call this with that exact id.
    ///
    /// 📈️ Every producer in this crate mints through here, so binding the job to the `tools/call`
    /// progress token currently active on this thread covers all of them by construction — no
    /// producer has to opt in, and none can silently forget to.
    pub fn begin_with_id(&self, job_id: impl Into<String>, kind: &str) -> String {
        let job_id = job_id.into();
        let mut record = JobRecord { kind: kind.to_string(), status: JobStatus::Pending, progress: None, message: None, result: None, error: None, cancel_requested: false, on_cancel: None, events: Vec::new(), next_ordinal: 0 };
        record.append("accepted", None, None);
        self.jobs.lock().expect("job registry lock poisoned").insert(job_id.clone(), record);
        bind_job_to_active_scope(&job_id);
        job_id
    }

    /// 📈️ Real progress from the job's own producer — a no-op (`false`) once the job is terminal.
    pub fn report_progress(&self, job_id: &str, progress: f64, message: Option<String>) -> bool {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let accepted = match jobs.get_mut(job_id) {
            Some(record) if !record.status.is_terminal() && record.status != JobStatus::AwaitingApproval => {
                record.status = JobStatus::Running;
                record.progress = Some(progress.clamp(0.0, 1.0));
                if message.is_some() {
                    record.message = message.clone();
                }
                record.append("progress", record.progress, message.clone());
                true
            }
            _ => false,
        };
        drop(jobs);
        if accepted {
            job_progress_changed(job_id, progress, message.as_deref());
        }
        accepted
    }

    /// 📈️ A terminal transition is the client's LAST progress row (1.0 for a success), then
    /// the binding is released so a long-lived stdio process never accumulates one entry per
    /// job it ever ran.
    pub fn finish(&self, job_id: &str, status: JobStatus, result: Option<serde_json::Value>, error: Option<GatewayError>) -> bool {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let accepted = match jobs.get_mut(job_id) {
            Some(record) if !record.status.is_terminal() => {
                record.status = status;
                record.on_cancel = None;
                if result.is_some() {
                    record.result = result;
                }
                record.error = error;
                if status == JobStatus::Succeeded {
                    record.progress = Some(1.0);
                }
                let kind = match status {
                    JobStatus::Succeeded => "succeeded",
                    JobStatus::Failed => "failed",
                    _ => "cancelled",
                };
                let message = record.error.as_ref().map(|error| error.message.clone());
                record.append(kind, record.progress, message);
                true
            }
            _ => false,
        };
        drop(jobs);
        if accepted {
            if status == JobStatus::Succeeded {
                job_progress_changed(job_id, 1.0, Some("succeeded"));
            }
            release_job_binding(job_id);
        }
        accepted
    }

    pub fn succeed(&self, job_id: &str, result: serde_json::Value) -> bool {
        self.finish(job_id, JobStatus::Succeeded, Some(result), None)
    }

    pub fn fail(&self, job_id: &str, error: GatewayError) -> bool {
        self.finish(job_id, JobStatus::Failed, None, Some(error))
    }

    /// ✅️ Called by a job's own producer once its work loop has actually observed
    /// [`JobRegistry::is_cancel_requested`] and stopped — never called by `job_cancel` itself.
    pub fn mark_cancelled(&self, job_id: &str) -> bool {
        self.finish(job_id, JobStatus::Cancelled, None, None)
    }

    /// 🛑️ The cooperative cancellation flag a job's own producer polls from inside its work loop.
    pub fn is_cancel_requested(&self, job_id: &str) -> bool {
        self.jobs.lock().expect("job registry lock poisoned").get(job_id).map(|record| record.cancel_requested).unwrap_or(false)
    }

    /// 🔗️ Binds the producer's own interrupt to `job_id`: `request_cancel` fires it once, outside the
    /// registry lock, so a `job_cancel` or `notifications/cancelled` reaches work that is blocked in a
    /// guest instead of waiting for it to poll. A cancel that already landed fires it immediately.
    pub fn bind_cancel(&self, job_id: &str, hook: impl FnOnce() + Send + 'static) {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let Some(record) = jobs.get_mut(job_id) else { return };
        if record.status.is_terminal() {
            return;
        }
        if !record.cancel_requested {
            record.on_cancel = Some(Box::new(hook));
            return;
        }
        drop(jobs);
        hook();
    }

    /// 🛑️ `job_cancel`'s real effect: flips the cooperative flag and fires the producer's bound
    /// interrupt; a still-`Pending` job (nothing running yet to interrupt) finishes as `Cancelled`
    /// immediately, a `Running` one waits for its producer to call [`JobRegistry::mark_cancelled`].
    /// `NOT_FOUND`/`PRECONDITION_FAILED` for an unknown or already-terminal id — never a silent no-op.
    pub fn request_cancel(&self, job_id: &str) -> Result<JobSnapshot, GatewayError> {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let record = jobs.get_mut(job_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such job: {job_id}")))?;
        if record.status.is_terminal() {
            return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("job {job_id} already finished as {:?} — nothing to cancel", record.status)));
        }
        record.cancel_requested = true;
        record.append("cancel-requested", None, None);
        if matches!(record.status, JobStatus::Pending | JobStatus::AwaitingApproval) {
            record.status = JobStatus::Cancelled;
            record.append("cancelled", None, None);
        }
        let hook = record.on_cancel.take();
        let snapshot = record_snapshot(job_id, record);
        drop(jobs);
        if let Some(hook) = hook {
            hook();
        }
        Ok(snapshot)
    }

    pub fn snapshot(&self, job_id: &str) -> Option<JobSnapshot> {
        let jobs = self.jobs.lock().expect("job registry lock poisoned");
        jobs.get(job_id).map(|record| record_snapshot(job_id, record))
    }

    /// ⏸️ The producer's work is done and its effect waits on an explicit approval: the job parks,
    /// non-terminal, with the proposal as its result. A cancel now withdraws the proposal and ends
    /// the job; [`Self::resume_approved`] hands it back to its producer to commit.
    pub fn await_approval(&self, job_id: &str, proposal: serde_json::Value) -> bool {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let Some(record) = jobs.get_mut(job_id).filter(|record| !record.status.is_terminal()) else { return false };
        record.status = JobStatus::AwaitingApproval;
        record.progress = Some(1.0);
        record.result = Some(proposal);
        record.append("awaiting-approval", record.progress, None);
        true
    }

    /// ▶️ Consumes a parked approval exactly once: `true` only for a job still `AwaitingApproval`,
    /// which is `Running` again when this returns, so two approvals of one proposal can never both
    /// commit.
    pub fn resume_approved(&self, job_id: &str) -> bool {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let Some(record) = jobs.get_mut(job_id).filter(|record| record.status == JobStatus::AwaitingApproval) else { return false };
        record.status = JobStatus::Running;
        record.append("approved", record.progress, None);
        true
    }

    /// 🗓️ Records one producer-owned domain fact in the job's journal; refused once the job is terminal.
    pub fn record_event(&self, job_id: &str, kind: &str, message: Option<String>) -> bool {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let Some(record) = jobs.get_mut(job_id).filter(|record| !record.status.is_terminal()) else { return false };
        record.append(kind, None, message);
        true
    }

    /// 📃️ Every journal row after `after`, oldest first, at most `limit` of them.
    pub fn events(&self, job_id: &str, after: u64, limit: usize) -> Option<JobEventPage> {
        let jobs = self.jobs.lock().expect("job registry lock poisoned");
        let record = jobs.get(job_id)?;
        let events: Vec<JobEvent> = record.events.iter().filter(|event| event.ordinal > after).take(limit).cloned().collect();
        let next_cursor = events.last().map_or(after, |last| last.ordinal);
        Some(JobEventPage { events, next_cursor })
    }
}

fn record_snapshot(job_id: &str, record: &JobRecord) -> JobSnapshot {
    JobSnapshot { job_id: job_id.to_string(), kind: record.kind.clone(), status: record.status, progress: record.progress, message: record.message.clone(), result: record.result.clone(), error: record.error.clone(), cancel_requested: record.cancel_requested }
}

static JOB_REGISTRY: OnceLock<JobRegistry> = OnceLock::new();

/// 🌍️ The one process-wide [`JobRegistry`] `job_get`/`job_cancel`/`semio://job/{id}` and every job
/// producer share — a process-global singleton (mirroring `🧵️bridge::BridgeHandle`'s own one-per-
/// process shape) rather than a constructor argument, so a future job-producing facet (W5) needs
/// only this function, never a change to `register_ui_tools`'s/`read_ui_resource`'s signatures.
pub fn job_registry() -> &'static JobRegistry {
    JOB_REGISTRY.get_or_init(JobRegistry::new)
}

//#endregion 🔖️JobRegistry
