//! 🖥️ UI information, UI actions, and jobs — ticket 26/08/29/AI-MCP-END-TO-END packet
//! W6-ui-and-jobs. `ui_focus`/`ui_reveal` forward a `ShellCommand` (JSON-encoded, matching
//! `💻️os/🔨️modules/🖥️shell`'s own wire vocabulary) to the attached `/bridge` shell as a
//! `GatewayToShell::ShellCommand` frame and block on its `ShellCommandResult` (bounded by
//! [`SHELL_COMMAND_TIMEOUT_MS`], never forever). `semio://window[/…]`/`semio://ui/active-context`/
//! `semio://ui/selection` project the SAME `ShellState` mirror `🧵️bridge::BridgeHandle::last_shell_state`
//! already holds — never a second, invented source of truth. `job_get`/`job_cancel` read/act through
//! [`job_registry`], a plugin-agnostic, process-wide job tracker any producer (W5's inference, a
//! future tessellation job, …) mints ids into via [`JobRegistry::begin`] and reports progress/result
//! into via [`JobRegistry::report_progress`]/[`JobRegistry::succeed`]/[`JobRegistry::fail`] — this
//! facet never reaches into the plugin-host reactor's own `job: u64` ids directly (those are
//! per-guest-instance and require a live `GuestInstance` this crate's tool layer never holds); a
//! producer that spawns one is the bridge between the two id spaces. Every tool this file registers
//! stays PRESENT in `tools/list` and every resource stays LISTED regardless of tier — only a `call`/
//! `read` result varies (bare: no bridge at all; headless: a bridge exists but no shell has dialed
//! it yet, a normal, retryable state; attached: a live shell). Never a panic, never a protocol-level
//! failure, never fabricated UI state — see `read_ui_resource`'s doc for the one deliberate honesty
//! trade-off (`semio://ui/selection` omits per-artifact object selection, which `ShellState` does
//! not model).

use crate::bridge::{BridgeHandle, GatewayToShell, ShellConnectionId, ShellToGateway};

/// 🔌️ The late-bound `/bridge` handle. `McpServer` is built BEFORE the HTTP transport creates its
/// `BridgeHandle` (the handle is minted from the transport's own worker pool, so hoisting its
/// construction would mean a second pool), and `stdio` never serves `/bridge` at all — so the tool
/// registry captures this slot at build time and the transport fills it in once, on `start`. An
/// unfilled slot is exactly the "no bridge at all" tier, not an error.
pub type BridgeSlot = Arc<std::sync::OnceLock<Arc<BridgeHandle>>>;

/// 🔎️ Resolves the slot to the live handle, if one was ever published into it.
fn resolve_bridge(slot: Option<&BridgeSlot>) -> Option<&Arc<BridgeHandle>> {
    slot.and_then(|slot| slot.get())
}
use crate::catalog::{CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, ToolExposure};
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::handles::{mint_id, HandleKind};
use crate::protocol::{CallToolResult, ContentBlock, InMemoryToolRegistry, Resource, ResourceContent, ResourceTemplate, Tool};
use crate::workspace::HeadlessWorkspace;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

//#region 🔖️Clock
/// 🕐️ Wall-clock milliseconds — same one-real-clock-read-site convention `🦀️component.rs::now_ms`
/// and every other facet's own copy already follow (each facet stays independently testable).
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}
//#endregion 🔖️Clock

//#region 🔖️ShellCommandDispatch
/// ⏱️ How long `ui_focus`/`ui_reveal` block waiting for the attached shell's `ShellCommandResult`
/// before giving up — a shell that never replies must never hang a tool call forever.
const SHELL_COMMAND_TIMEOUT_MS: u64 = 4_000;
const SHELL_COMMAND_POLL_INTERVAL_MS: u64 = 20;

static SHELL_COMMAND_SEQ: AtomicU64 = AtomicU64::new(1);

fn next_shell_command_seq() -> u64 {
    SHELL_COMMAND_SEQ.fetch_add(1, Ordering::Relaxed)
}

/// 🕳️ No `BridgeHandle` at all — this gateway is not serving `/bridge` (e.g. `stdio` transport, or
/// `http` before a shell has ever been expected). The bare tier for every UI tool/resource.
fn bridge_not_running_error() -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, "no `/bridge` is running on this gateway — start it with the `http` transport so a shell can dial `/bridge`").with_details(serde_json::json!({ "bindWith": ["http"] })).retryable()
}

/// 🕳️ A `BridgeHandle` exists but no shell connection is live — the normal headless state, not a
/// bug: an agent should retry once a shell has dialed `/bridge`.
fn no_shell_attached_error() -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, "no shell is attached to `/bridge` yet — this is expected until a shell connects; retry once one does").retryable()
}

/// 🧭️ The connection this facet routes `ShellCommand`/`semio://window…` reads through — the
/// highest (most recently registered) `ShellConnectionId`, i.e. the most-recently-connected shell.
/// `🧵️bridge` carries no notion of "the" primary shell yet; a later packet that needs explicit
/// multi-shell targeting adds a `shellId` argument to these tools without touching this fallback.
fn active_shell_connection(bridge: &BridgeHandle) -> Option<ShellConnectionId> {
    bridge.connections().into_iter().max()
}

/// 📤️ Encodes `command_json` the same way `AgentBridge`'s renderer-side hook does (opaque JSON
/// bytes inside `GatewayToShell::ShellCommand.command` — `🧵️bridge`'s own design note: the codec
/// never interprets these bytes), sends it to the active shell connection, and blocks (bounded by
/// `timeout`, polling `BridgeHandle::last_command_result`) for a `ShellCommandResult` whose
/// `in_reply_to` matches the `seq` this call minted. A stale/unrelated result already sitting on the
/// connection is never mistaken for this call's reply (`in_reply_to` must match exactly).
fn dispatch_shell_command_with_timeout(bridge: &BridgeHandle, command_json: serde_json::Value, timeout: Duration) -> Result<(), GatewayError> {
    let connection_id = active_shell_connection(bridge).ok_or_else(no_shell_attached_error)?;
    let seq = next_shell_command_seq();
    let payload = serde_json::to_vec(&command_json).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("failed to encode ShellCommand payload: {error}")))?;
    if !bridge.send_to(connection_id, GatewayToShell::ShellCommand { seq, command: payload }) {
        return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "the attached shell's connection closed while dispatching this command").retryable());
    }
    let deadline = Instant::now() + timeout;
    loop {
        if let Some((in_reply_to, ok, fault)) = bridge.last_command_result(connection_id) {
            if in_reply_to == seq {
                return if ok { Ok(()) } else { Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, fault.unwrap_or_else(|| "the shell rejected this command".to_string()))) };
            }
        }
        if Instant::now() >= deadline {
            return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("the attached shell did not reply to this command within {}ms", timeout.as_millis())).retryable());
        }
        std::thread::sleep(Duration::from_millis(SHELL_COMMAND_POLL_INTERVAL_MS));
    }
}

fn dispatch_shell_command(bridge: &BridgeHandle, command_json: serde_json::Value) -> Result<(), GatewayError> {
    dispatch_shell_command_with_timeout(bridge, command_json, Duration::from_millis(SHELL_COMMAND_TIMEOUT_MS))
}
//#endregion 🔖️ShellCommandDispatch

//#region 🔖️JobRegistry
/// 🚦 One plugin-agnostic job's lifecycle — `Pending` (registered, not yet started), `Running`
/// (progress may update), then exactly one terminal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl JobStatus {
    fn is_terminal(self) -> bool {
        matches!(self, JobStatus::Succeeded | JobStatus::Failed | JobStatus::Cancelled)
    }
}

struct JobRecord {
    kind: String,
    status: JobStatus,
    progress: Option<f64>,
    message: Option<String>,
    result: Option<serde_json::Value>,
    error: Option<GatewayError>,
    cancel_requested: bool,
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
    fn to_json(&self) -> serde_json::Value {
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
    pub fn begin_with_id(&self, job_id: impl Into<String>, kind: &str) -> String {
        let job_id = job_id.into();
        let record = JobRecord { kind: kind.to_string(), status: JobStatus::Pending, progress: None, message: None, result: None, error: None, cancel_requested: false };
        self.jobs.lock().expect("job registry lock poisoned").insert(job_id.clone(), record);
        job_id
    }

    /// 📈️ Real progress from the job's own producer — a no-op (`false`) once the job is terminal.
    pub fn report_progress(&self, job_id: &str, progress: f64, message: Option<String>) -> bool {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        match jobs.get_mut(job_id) {
            Some(record) if !record.status.is_terminal() => {
                record.status = JobStatus::Running;
                record.progress = Some(progress.clamp(0.0, 1.0));
                if message.is_some() {
                    record.message = message;
                }
                true
            }
            _ => false,
        }
    }

    fn finish(&self, job_id: &str, status: JobStatus, result: Option<serde_json::Value>, error: Option<GatewayError>) -> bool {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        match jobs.get_mut(job_id) {
            Some(record) if !record.status.is_terminal() => {
                record.status = status;
                record.result = result;
                record.error = error;
                if status == JobStatus::Succeeded {
                    record.progress = Some(1.0);
                }
                true
            }
            _ => false,
        }
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

    /// 🛑️ `job_cancel`'s real effect: flips the cooperative flag; a still-`Pending` job (nothing
    /// running yet to interrupt) finishes as `Cancelled` immediately, a `Running` one waits for its
    /// producer to call [`JobRegistry::mark_cancelled`]. `NOT_FOUND`/`PRECONDITION_FAILED` for an
    /// unknown or already-terminal id — never a silent no-op.
    pub fn request_cancel(&self, job_id: &str) -> Result<JobSnapshot, GatewayError> {
        let mut jobs = self.jobs.lock().expect("job registry lock poisoned");
        let record = jobs.get_mut(job_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such job: {job_id}")))?;
        if record.status.is_terminal() {
            return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("job {job_id} already finished as {:?} — nothing to cancel", record.status)));
        }
        record.cancel_requested = true;
        if record.status == JobStatus::Pending {
            record.status = JobStatus::Cancelled;
        }
        Ok(record_snapshot(job_id, record))
    }

    pub fn snapshot(&self, job_id: &str) -> Option<JobSnapshot> {
        let jobs = self.jobs.lock().expect("job registry lock poisoned");
        jobs.get(job_id).map(|record| record_snapshot(job_id, record))
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

//#region 🔖️Schemas
fn ui_focus_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/ui.focus/input",
        "type": "object",
        "properties": { "windowId": { "type": "string" } },
        "additionalProperties": false,
    })
}

fn ui_focus_output_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/ui.focus/output",
        "type": "object",
        "properties": { "ok": { "type": "boolean" }, "windowId": {} },
    })
}

fn ui_reveal_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/ui.reveal/input",
        "type": "object",
        "properties": { "anchor": { "type": "string", "enum": ["left", "right", "top", "bottom"] }, "path": { "type": "array", "items": { "type": "string" } } },
        "required": ["anchor", "path"],
        "additionalProperties": false,
    })
}

fn ui_reveal_output_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/ui.reveal/output",
        "type": "object",
        "properties": { "ok": { "type": "boolean" }, "anchor": { "type": "string" }, "path": { "type": "array", "items": { "type": "string" } } },
    })
}

fn job_get_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/job.get/input",
        "type": "object",
        "properties": { "jobId": { "type": "string" } },
        "required": ["jobId"],
        "additionalProperties": false,
    })
}

fn job_snapshot_output_schema(id: &str) -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": format!("semio://capability/{id}/output"),
        "type": "object",
        "properties": {
            "jobId": { "type": "string" },
            "kind": { "type": "string" },
            "status": { "type": "string" },
            "progress": {},
            "message": {},
            "result": {},
            "error": {},
            "cancelRequested": { "type": "boolean" },
        },
    })
}

fn job_cancel_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/job.cancel/input",
        "type": "object",
        "properties": { "jobId": { "type": "string" } },
        "required": ["jobId"],
        "additionalProperties": false,
    })
}
//#endregion 🔖️Schemas

//#region 🔖️Capabilities
fn ui_focus_capability() -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef("ui.focus".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Ui,
        title: "Focus Window".to_string(),
        description: "Focuses a window on the attached shell (omit windowId to clear focus).".to_string(),
        artifact_kind: None,
        use_when: vec!["focus a window".to_string(), "bring a window to the front".to_string()],
        input_schema: ui_focus_input_schema(),
        output_schema: ui_focus_output_schema(),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "ui_focus".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("focus".to_string()), category: Some("ui".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn ui_reveal_capability() -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef("ui.reveal".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Ui,
        title: "Reveal In Panel".to_string(),
        description: "Makes a panel visible and navigates it to the given path on the attached shell.".to_string(),
        artifact_kind: None,
        use_when: vec!["reveal an item in a panel".to_string(), "show this in the explorer".to_string()],
        input_schema: ui_reveal_input_schema(),
        output_schema: ui_reveal_output_schema(),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "ui_reveal".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("reveal".to_string()), category: Some("ui".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn job_get_capability() -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef("job.get".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Job,
        title: "Get Job".to_string(),
        description: "Reads the status/progress/result of one plugin-agnostic job by id.".to_string(),
        artifact_kind: None,
        use_when: vec!["check on a running job".to_string(), "is this job done yet".to_string()],
        input_schema: job_get_input_schema(),
        output_schema: job_snapshot_output_schema("job.get"),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "job_get".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("job".to_string()), category: Some("job".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn job_cancel_capability() -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef("job.cancel".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Job,
        title: "Cancel Job".to_string(),
        description: "Requests cooperative cancellation of one running or pending job by id.".to_string(),
        artifact_kind: None,
        use_when: vec!["cancel a running job".to_string(), "stop this job".to_string()],
        input_schema: job_cancel_input_schema(),
        output_schema: job_snapshot_output_schema("job.cancel"),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "job_cancel".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("stop".to_string()), category: Some("job".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

/// 🖥️ The UI + job capabilities, folded into `CatalogSource.gateway` alongside `🦀️component.rs`'s
/// own `core_tool_capabilities()` — same pattern, disjoint ids (`ui.*`/`job.*` vs `capabilities.*`/
/// `context.*`), so both compile into the SAME catalog with zero collision risk.
pub fn ui_capabilities() -> Vec<CapabilityDefinition> {
    vec![ui_focus_capability(), ui_reveal_capability(), job_get_capability(), job_cancel_capability()]
}
//#endregion 🔖️Capabilities

//#region 🔖️ToolHandlers
fn input_invalid(message: impl Into<String>) -> CallToolResult {
    CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, message.into()))
}

fn ui_focus_handler(bridge: Option<&Arc<BridgeHandle>>, arguments: serde_json::Value) -> CallToolResult {
    let window_id = match arguments.get("windowId") {
        None => None,
        Some(serde_json::Value::String(window_id)) => Some(window_id.clone()),
        Some(_) => return input_invalid("windowId must be a string when present"),
    };
    let Some(bridge) = bridge else {
        return CallToolResult::tool_error(&bridge_not_running_error());
    };
    let command = serde_json::json!({ "type": "focusWindow", "windowId": window_id });
    match dispatch_shell_command(bridge, command) {
        Ok(()) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("focused {window_id:?}") }], Some(serde_json::json!({ "ok": true, "windowId": window_id }))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn ui_reveal_handler(bridge: Option<&Arc<BridgeHandle>>, arguments: serde_json::Value) -> CallToolResult {
    let anchor = match arguments.get("anchor").and_then(serde_json::Value::as_str) {
        Some(anchor) if ["left", "right", "top", "bottom"].contains(&anchor) => anchor.to_string(),
        _ => return input_invalid("anchor must be one of left|right|top|bottom"),
    };
    let path: Vec<String> = match arguments.get("path").and_then(serde_json::Value::as_array) {
        Some(items) => match items.iter().map(|item| item.as_str().map(str::to_string)).collect::<Option<Vec<_>>>() {
            Some(path) => path,
            None => return input_invalid("path must be an array of strings"),
        },
        None => return input_invalid("path is required"),
    };
    let Some(bridge) = bridge else {
        return CallToolResult::tool_error(&bridge_not_running_error());
    };
    if let Err(error) = dispatch_shell_command(bridge, serde_json::json!({ "type": "setPanelVisible", "anchor": anchor, "visible": true })) {
        return CallToolResult::tool_error(&error);
    }
    match dispatch_shell_command(bridge, serde_json::json!({ "type": "setPanelPath", "anchor": anchor, "path": path })) {
        Ok(()) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("revealed {path:?} in {anchor}") }], Some(serde_json::json!({ "ok": true, "anchor": anchor, "path": path }))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn job_get_handler(arguments: serde_json::Value) -> CallToolResult {
    let job_id = match arguments.get("jobId").and_then(serde_json::Value::as_str) {
        Some(job_id) => job_id.to_string(),
        None => return input_invalid("jobId is required"),
    };
    match job_registry().snapshot(&job_id) {
        Some(snapshot) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("job {job_id} is {:?}", snapshot.status) }], Some(snapshot.to_json())),
        None => CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::NotFound, format!("no such job: {job_id}"))),
    }
}

fn job_cancel_handler(arguments: serde_json::Value) -> CallToolResult {
    let job_id = match arguments.get("jobId").and_then(serde_json::Value::as_str) {
        Some(job_id) => job_id.to_string(),
        None => return input_invalid("jobId is required"),
    };
    match job_registry().request_cancel(&job_id) {
        Ok(snapshot) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("job {job_id} cancellation requested ({:?})", snapshot.status) }], Some(snapshot.to_json())),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 🖥️ Registers `ui_focus`, `ui_reveal`, `job_get`, `job_cancel` — always registered, always in
/// `tools/list`, regardless of tier (§ module doc). `_workspace` is accepted (not yet read) for
/// seam symmetry with `read_ui_resource`/a later packet that scopes jobs to one workspace; today's
/// job seam is the process-wide [`job_registry`], deliberately workspace-independent.
pub fn register_ui_tools(registry: &mut InMemoryToolRegistry, bridge: Option<BridgeSlot>, _workspace: Option<Arc<HeadlessWorkspace>>) {
    let mut ui_focus = Tool::new("ui_focus", ui_focus_input_schema());
    ui_focus.title = Some("Focus Window".to_string());
    ui_focus.description = Some("Focuses a window on the attached shell (omit windowId to clear focus).".to_string());
    ui_focus.output_schema = Some(ui_focus_output_schema());
    let focus_bridge = bridge.clone();
    registry.register(ui_focus, move |arguments| ui_focus_handler(resolve_bridge(focus_bridge.as_ref()), arguments)).expect("ui_focus is a valid tool name");

    let mut ui_reveal = Tool::new("ui_reveal", ui_reveal_input_schema());
    ui_reveal.title = Some("Reveal In Panel".to_string());
    ui_reveal.description = Some("Makes a panel visible and navigates it to the given path on the attached shell.".to_string());
    ui_reveal.output_schema = Some(ui_reveal_output_schema());
    let reveal_bridge = bridge.clone();
    registry.register(ui_reveal, move |arguments| ui_reveal_handler(resolve_bridge(reveal_bridge.as_ref()), arguments)).expect("ui_reveal is a valid tool name");

    let mut job_get = Tool::new("job_get", job_get_input_schema());
    job_get.title = Some("Get Job".to_string());
    job_get.description = Some("Reads the status/progress/result of one plugin-agnostic job by id.".to_string());
    job_get.output_schema = Some(job_snapshot_output_schema("job.get"));
    registry.register(job_get, move |arguments| job_get_handler(arguments)).expect("job_get is a valid tool name");

    let mut job_cancel = Tool::new("job_cancel", job_cancel_input_schema());
    job_cancel.title = Some("Cancel Job".to_string());
    job_cancel.description = Some("Requests cooperative cancellation of one running or pending job by id.".to_string());
    job_cancel.output_schema = Some(job_snapshot_output_schema("job.cancel"));
    registry.register(job_cancel, move |arguments| job_cancel_handler(arguments)).expect("job_cancel is a valid tool name");
}
//#endregion 🔖️ToolHandlers

//#region 🔖️ShellStateProjection
/// 🧭️ The attached shell's last full `ShellState` snapshot, decoded as raw JSON — `🧵️bridge`'s own
/// design note says `state`/`command` bytes are opaque, AgentBridge encodes/decodes them as JSON on
/// the renderer side, so this facet mirrors that instead of taking a new compile-time dependency on
/// `semio-framework-os-shell`'s Rust types (see this file's W6 report for the tradeoff). A
/// `ShellStatePatch`-only connection (no full snapshot received yet) is a typed, retryable
/// `PLUGIN_UNAVAILABLE`, never a guess at merging a patch onto nothing.
fn full_shell_state(bridge: Option<&Arc<BridgeHandle>>) -> Result<serde_json::Value, GatewayError> {
    let bridge = bridge.ok_or_else(bridge_not_running_error)?;
    let connection_id = active_shell_connection(bridge).ok_or_else(no_shell_attached_error)?;
    match bridge.last_shell_state(connection_id) {
        Some(ShellToGateway::ShellState { state, .. }) => serde_json::from_slice(&state).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("failed to decode the attached shell's ShellState JSON: {error}"))),
        Some(ShellToGateway::ShellStatePatch { .. }) => Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "the attached shell has only sent incremental ShellStatePatch frames so far — no full ShellState snapshot to project yet; this resolves once the shell publishes its next full snapshot").retryable()),
        _ => Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "the attached shell has not published a ShellState snapshot yet").retryable()),
    }
}

fn panel_summary(state: &serde_json::Value) -> serde_json::Value {
    let mut panels = serde_json::Map::new();
    for anchor in ["left", "right", "top", "bottom"] {
        panels.insert(
            anchor.to_string(),
            serde_json::json!({
                "visible": state.pointer(&format!("/panelsVisible/{anchor}")).cloned().unwrap_or(serde_json::Value::Null),
                "size": state.pointer(&format!("/panelsSize/{anchor}")).cloned().unwrap_or(serde_json::Value::Null),
                "path": state.pointer(&format!("/panelsPath/{anchor}")).cloned().unwrap_or(serde_json::Value::Null),
            }),
        );
    }
    serde_json::Value::Object(panels)
}

fn window_entries(state: &serde_json::Value) -> Vec<serde_json::Value> {
    let extra_windows = state.get("extraWindows").and_then(serde_json::Value::as_array).cloned().unwrap_or_default();
    let titles = state.get("windowTitlesById").cloned().unwrap_or_else(|| serde_json::json!({}));
    let icons = state.get("windowIconsById").cloned().unwrap_or_else(|| serde_json::json!({}));
    let active_window_id = state.get("activeWindowId").and_then(serde_json::Value::as_str);
    extra_windows
        .iter()
        .map(|window| {
            let window_id = window.get("windowId").and_then(serde_json::Value::as_str).unwrap_or_default();
            serde_json::json!({
                "windowId": window_id,
                "kind": window.get("kind").cloned().unwrap_or(serde_json::Value::Null),
                "title": titles.get(window_id).cloned().unwrap_or(serde_json::Value::Null),
                "icon": icons.get(window_id).cloned().unwrap_or(serde_json::Value::Null),
                "focused": active_window_id == Some(window_id),
            })
        })
        .collect()
}

fn read_window_resource(bridge: Option<&Arc<BridgeHandle>>, window_id: Option<&str>) -> Result<Vec<ResourceContent>, GatewayError> {
    let state = full_shell_state(bridge)?;
    let windows = window_entries(&state);
    match window_id {
        Some(window_id) => match windows.into_iter().find(|window| window.get("windowId").and_then(serde_json::Value::as_str) == Some(window_id)) {
            Some(window) => Ok(vec![ResourceContent { uri: format!("semio://window/{window_id}"), mime_type: Some("application/json".to_string()), text: Some(window.to_string()), blob: None }]),
            None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no such window: {window_id}"))),
        },
        None => {
            let payload = serde_json::json!({ "activeWindowId": state.get("activeWindowId").cloned().unwrap_or(serde_json::Value::Null), "windows": windows, "panels": panel_summary(&state) });
            Ok(vec![ResourceContent { uri: "semio://window".to_string(), mime_type: Some("application/json".to_string()), text: Some(payload.to_string()), blob: None }])
        }
    }
}

fn read_active_context_resource(bridge: Option<&Arc<BridgeHandle>>) -> Result<Vec<ResourceContent>, GatewayError> {
    let state = full_shell_state(bridge)?;
    let payload = serde_json::json!({
        "revision": state.get("revision").cloned().unwrap_or(serde_json::Value::Null),
        "activeWindowId": state.get("activeWindowId").cloned().unwrap_or(serde_json::Value::Null),
        "activeToolId": state.get("activeToolId").cloned().unwrap_or(serde_json::Value::Null),
        "activeUtilityByWindow": state.get("activeUtilityByWindow").cloned().unwrap_or_else(|| serde_json::json!({})),
        "activeExampleId": state.get("activeExampleId").cloned().unwrap_or(serde_json::Value::Null),
        "openWithFocusRole": state.get("openWithFocusRole").cloned().unwrap_or(serde_json::Value::Null),
        "activeTutorialId": state.get("activeTutorialId").cloned().unwrap_or(serde_json::Value::Null),
        "uiLocale": state.get("uiLocale").cloned().unwrap_or(serde_json::Value::Null),
        "uiThemeId": state.get("uiThemeId").cloned().unwrap_or(serde_json::Value::Null),
    });
    Ok(vec![ResourceContent { uri: "semio://ui/active-context".to_string(), mime_type: Some("application/json".to_string()), text: Some(payload.to_string()), blob: None }])
}

/// 🎯️ `ShellState` models exactly ONE selection-shaped field — `selectedConflictId` (merge conflict
/// preview). Per-artifact object selection (e.g. a CAD scene's selected entities) is app-instance
/// state carried over `AppFrames`/`Instances`, not shell state, and this facet does not reach into
/// it (design requirement: never a second, invented source of truth) — `activeWindowId`/
/// `activeToolId` are included as the closest real "what's focused" signals `ShellState` has.
fn read_selection_resource(bridge: Option<&Arc<BridgeHandle>>) -> Result<Vec<ResourceContent>, GatewayError> {
    let state = full_shell_state(bridge)?;
    let payload = serde_json::json!({
        "revision": state.get("revision").cloned().unwrap_or(serde_json::Value::Null),
        "selectedConflictId": state.get("selectedConflictId").cloned().unwrap_or(serde_json::Value::Null),
        "activeWindowId": state.get("activeWindowId").cloned().unwrap_or(serde_json::Value::Null),
        "activeToolId": state.get("activeToolId").cloned().unwrap_or(serde_json::Value::Null),
    });
    Ok(vec![ResourceContent { uri: "semio://ui/selection".to_string(), mime_type: Some("application/json".to_string()), text: Some(payload.to_string()), blob: None }])
}

fn read_job_resource(job_id: &str) -> Result<Vec<ResourceContent>, GatewayError> {
    match job_registry().snapshot(job_id) {
        Some(snapshot) => Ok(vec![ResourceContent { uri: format!("semio://job/{job_id}"), mime_type: Some("application/json".to_string()), text: Some(snapshot.to_json().to_string()), blob: None }]),
        None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no such job: {job_id}"))),
    }
}

/// 🖥️ The UI/job resource entries to advertise in `resources/list` — presence never depends on
/// tier (§ module doc); `bridge` is accepted for signature symmetry with `read_ui_resource`, not
/// read (the returned list is identical bare/headless/attached, matching
/// `WorkspaceResourceRegistry::list`'s own convention for `semio://workspace`).
pub fn ui_resources(_bridge: Option<&BridgeSlot>) -> Vec<Resource> {
    vec![
        Resource {
            uri: "semio://window".to_string(),
            name: "ui-windows".to_string(),
            title: Some("Windows & panels".to_string()),
            description: Some("Window/panel inventory projected from the attached shell's ShellState (PLUGIN_UNAVAILABLE with no shell attached)".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        },
        Resource {
            uri: "semio://ui/active-context".to_string(),
            name: "ui-active-context".to_string(),
            title: Some("Active UI context".to_string()),
            description: Some("Active window/tool/utility/example on the attached shell".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        },
        Resource {
            uri: "semio://ui/selection".to_string(),
            name: "ui-selection".to_string(),
            title: Some("UI selection".to_string()),
            description: Some("Shell-level selection only (selected conflict, active window/tool) — per-artifact object selection lives in app-instance state and is not projected here".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        },
    ]
}

/// 🖥️ The UI/job resource TEMPLATES to advertise in `resources/templates/list` — split from
/// [`ui_resources`] because `semio://window/{windowId}`/`semio://job/{jobId}` are parametrized
/// (`ResourceTemplate`, not `Resource`); not in the W6 contract's original shape, added because
/// `semio://job/{id}` has nowhere else to be listed (see this file's W6 report).
pub fn ui_resource_templates() -> Vec<ResourceTemplate> {
    vec![
        ResourceTemplate { uri_template: "semio://window/{windowId}".to_string(), name: "ui-window".to_string(), title: Some("One window".to_string()), description: Some("One extra window's title/icon/focus state".to_string()), mime_type: Some("application/json".to_string()) },
        ResourceTemplate { uri_template: "semio://job/{jobId}".to_string(), name: "job".to_string(), title: Some("One job".to_string()), description: Some("Status/progress/result for one plugin-agnostic job id".to_string()), mime_type: Some("application/json".to_string()) },
    ]
}

/// 🖥️ Answers `semio://window[/…]`, `semio://ui/active-context`, `semio://ui/selection`,
/// `semio://job/{id}`; returns `None` when `uri` is not one of ours so the composing registry can
/// fall through to its own `NOT_FOUND`. `_workspace` is accepted for signature symmetry (today's UI
/// projections read only the bridge's `ShellState` mirror, per design requirement #1).
pub fn read_ui_resource(uri: &str, bridge: Option<&BridgeSlot>, _workspace: Option<&Arc<HeadlessWorkspace>>) -> Option<Result<Vec<ResourceContent>, GatewayError>> {
    let bridge = resolve_bridge(bridge);
    if let Some(job_id) = uri.strip_prefix("semio://job/") {
        return Some(read_job_resource(job_id));
    }
    if let Some(window_id) = uri.strip_prefix("semio://window/") {
        return Some(read_window_resource(bridge, Some(window_id)));
    }
    match uri {
        "semio://window" => Some(read_window_resource(bridge, None)),
        "semio://ui/active-context" => Some(read_active_context_resource(bridge)),
        "semio://ui/selection" => Some(read_selection_resource(bridge)),
        _ => None,
    }
}
//#endregion 🔖️ShellStateProjection

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::protocol::ToolRegistry;

    fn full_registry() -> InMemoryToolRegistry {
        let mut registry = InMemoryToolRegistry::new();
        register_ui_tools(&mut registry, None, None);
        registry
    }

    /// 🔌️ A `BridgeSlot` already filled with a live handle — the "bridge exists, no shell attached"
    /// tier, which every `Option<BridgeSlot>` entry point now takes instead of a bare handle.
    fn filled_slot() -> BridgeSlot {
        let slot: BridgeSlot = Arc::new(std::sync::OnceLock::new());
        assert!(slot.set(Arc::new(BridgeHandle::new())).is_ok(), "a fresh slot is empty");
        slot
    }

    #[test]
    fn all_four_tools_register_under_valid_mcp_names_with_object_top_level_schemas() {
        let registry = full_registry();
        let tools = registry.list();
        let names = ["ui_focus", "ui_reveal", "job_get", "job_cancel"];
        for name in names {
            let tool = tools.iter().find(|tool| tool.name == name).unwrap_or_else(|| panic!("missing tool {name}"));
            assert!(crate::protocol::is_valid_tool_name(&tool.name));
            assert_eq!(tool.input_schema["type"], "object", "{name} input_schema");
            let output_schema = tool.output_schema.as_ref().unwrap_or_else(|| panic!("{name} has no output_schema"));
            assert_eq!(output_schema["type"], "object", "{name} output_schema");
        }
        assert_eq!(tools.len(), names.len());
    }

    #[test]
    fn ui_capabilities_expose_the_same_four_tool_names_with_the_right_kinds() {
        let capabilities = ui_capabilities();
        assert_eq!(capabilities.len(), 4);
        let expectations = [("ui.focus", "ui_focus", CapabilityKind::Ui), ("ui.reveal", "ui_reveal", CapabilityKind::Ui), ("job.get", "job_get", CapabilityKind::Job), ("job.cancel", "job_cancel", CapabilityKind::Job)];
        for (id, tool_name, kind) in expectations {
            let capability = capabilities.iter().find(|capability| capability.id.as_str() == id).unwrap_or_else(|| panic!("missing capability {id}"));
            assert_eq!(capability.kind, kind);
            assert_eq!(capability.exposure, ToolExposure::Direct { tool_name: tool_name.to_string() });
            assert_eq!(capability.input_schema["type"], "object");
            assert_eq!(capability.output_schema["type"], "object");
        }
    }

    #[test]
    fn ui_focus_with_no_bridge_is_a_retryable_plugin_unavailable_not_a_panic_or_protocol_failure() {
        let registry = full_registry();
        let result = registry.call("ui_focus", serde_json::json!({ "windowId": "w1" })).expect("known tool name resolves");
        assert!(result.is_error);
        let structured = result.structured_content.expect("structured content");
        assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE");
        assert_eq!(structured["retryable"], true);
    }

    #[test]
    fn ui_reveal_with_no_bridge_is_a_retryable_plugin_unavailable() {
        let registry = full_registry();
        let result = registry.call("ui_reveal", serde_json::json!({ "anchor": "left", "path": ["a"] })).expect("known tool name resolves");
        assert!(result.is_error);
        let structured = result.structured_content.expect("structured content");
        assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE");
        assert_eq!(structured["retryable"], true);
    }

    #[test]
    fn ui_focus_with_a_bridge_but_no_shell_attached_is_a_normal_retryable_state() {
        let mut registry = InMemoryToolRegistry::new();
        register_ui_tools(&mut registry, Some(filled_slot()), None);
        let result = registry.call("ui_focus", serde_json::json!({ "windowId": "w1" })).expect("known tool name resolves");
        assert!(result.is_error);
        let structured = result.structured_content.expect("structured content");
        assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE");
        assert_eq!(structured["retryable"], true);
        assert!(structured["message"].as_str().unwrap().contains("no shell is attached"));
    }

    #[test]
    fn ui_reveal_rejects_a_bad_anchor_as_input_invalid() {
        let registry = full_registry();
        let result = registry.call("ui_reveal", serde_json::json!({ "anchor": "middle", "path": [] })).expect("known tool name resolves");
        assert!(result.is_error);
        assert_eq!(result.structured_content.unwrap()["code"], "INPUT_INVALID");
    }

    #[test]
    fn ui_resources_and_templates_never_depend_on_bridge_presence() {
        assert_eq!(ui_resources(None).len(), 3);
        assert_eq!(ui_resources(Some(&filled_slot())).len(), 3);
        let templates = ui_resource_templates();
        assert!(templates.iter().any(|template| template.uri_template == "semio://window/{windowId}"));
        assert!(templates.iter().any(|template| template.uri_template == "semio://job/{jobId}"));
    }

    #[test]
    fn read_ui_resource_returns_none_for_a_non_ui_uri() {
        assert!(read_ui_resource("semio://not-a-resource", None, None).is_none());
        assert!(read_ui_resource("semio://capability", None, None).is_none());
    }

    #[test]
    fn read_ui_resource_for_window_degrades_to_a_typed_error_without_a_shell() {
        let outcome = read_ui_resource("semio://window", None, None).expect("ours");
        let error = outcome.expect_err("no bridge means no window data");
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
    }

    #[test]
    fn read_ui_resource_for_active_context_and_selection_also_degrade_cleanly() {
        assert!(read_ui_resource("semio://ui/active-context", None, None).expect("ours").is_err());
        assert!(read_ui_resource("semio://ui/selection", None, None).expect("ours").is_err());
    }

    #[test]
    fn dispatch_shell_command_times_out_when_the_shell_never_replies() {
        let bridge = BridgeHandle::new();
        let (_connection_id, _outbox) = bridge.register();
        let result = dispatch_shell_command_with_timeout(&bridge, serde_json::json!({ "type": "focusWindow", "windowId": null }), Duration::from_millis(60));
        let error = result.expect_err("no reply was ever recorded");
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
    }

    #[test]
    fn dispatch_shell_command_succeeds_once_the_matching_reply_arrives() {
        let bridge = Arc::new(BridgeHandle::new());
        let (connection_id, _outbox) = bridge.register();
        let expected_seq = SHELL_COMMAND_SEQ.load(Ordering::SeqCst);
        let reply_bridge = bridge.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(10));
            reply_bridge.record(connection_id, ShellToGateway::ShellCommandResult { in_reply_to: expected_seq, ok: true, fault: None });
        });
        let result = dispatch_shell_command_with_timeout(&bridge, serde_json::json!({ "type": "focusWindow", "windowId": null }), Duration::from_millis(2_000));
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn dispatch_shell_command_surfaces_a_shell_fault_as_side_effect_rejected() {
        let bridge = Arc::new(BridgeHandle::new());
        let (connection_id, _outbox) = bridge.register();
        let expected_seq = SHELL_COMMAND_SEQ.load(Ordering::SeqCst);
        let reply_bridge = bridge.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(10));
            reply_bridge.record(connection_id, ShellToGateway::ShellCommandResult { in_reply_to: expected_seq, ok: false, fault: Some("unknown window".to_string()) });
        });
        let result = dispatch_shell_command_with_timeout(&bridge, serde_json::json!({ "type": "focusWindow", "windowId": "nope" }), Duration::from_millis(2_000));
        let error = result.expect_err("the shell rejected the command");
        assert_eq!(error.code, GatewayErrorCode::SideEffectRejected);
    }

    #[test]
    fn job_registry_round_trips_pending_running_succeeded() {
        let registry = JobRegistry::new();
        let job_id = registry.begin("inference.demo");
        let pending = registry.snapshot(&job_id).expect("just began");
        assert_eq!(pending.status, JobStatus::Pending);
        assert!(registry.report_progress(&job_id, 0.5, Some("halfway".to_string())));
        let running = registry.snapshot(&job_id).expect("still tracked");
        assert_eq!(running.status, JobStatus::Running);
        assert_eq!(running.progress, Some(0.5));
        assert!(registry.succeed(&job_id, serde_json::json!({ "answer": 42 })));
        let done = registry.snapshot(&job_id).expect("still tracked");
        assert_eq!(done.status, JobStatus::Succeeded);
        assert_eq!(done.progress, Some(1.0));
        assert!(!registry.report_progress(&job_id, 0.1, None), "a terminal job never regresses");
    }

    #[test]
    fn begin_with_id_lets_a_producer_reuse_an_id_it_minted_elsewhere() {
        let registry = JobRegistry::new();
        let returned = registry.begin_with_id("job_from_elsewhere", "inference.demo");
        assert_eq!(returned, "job_from_elsewhere");
        assert_eq!(registry.snapshot("job_from_elsewhere").unwrap().status, JobStatus::Pending);
    }

    #[test]
    fn cancelling_a_pending_job_finishes_it_immediately() {
        let registry = JobRegistry::new();
        let job_id = registry.begin("inference.demo");
        let snapshot = registry.request_cancel(&job_id).expect("pending job is cancellable");
        assert_eq!(snapshot.status, JobStatus::Cancelled);
    }

    #[test]
    fn cancelling_a_running_job_only_sets_the_cooperative_flag_until_the_producer_acknowledges() {
        let registry = JobRegistry::new();
        let job_id = registry.begin("inference.demo");
        registry.report_progress(&job_id, 0.1, None);
        let snapshot = registry.request_cancel(&job_id).expect("running job is cancellable");
        assert_eq!(snapshot.status, JobStatus::Running);
        assert!(registry.is_cancel_requested(&job_id));
        assert!(registry.mark_cancelled(&job_id));
        assert_eq!(registry.snapshot(&job_id).unwrap().status, JobStatus::Cancelled);
    }

    #[test]
    fn cancelling_an_unknown_or_already_terminal_job_is_a_typed_error_not_a_silent_no_op() {
        let registry = JobRegistry::new();
        assert_eq!(registry.request_cancel("job_missing").unwrap_err().code, GatewayErrorCode::NotFound);
        let job_id = registry.begin("inference.demo");
        registry.succeed(&job_id, serde_json::Value::Null);
        assert_eq!(registry.request_cancel(&job_id).unwrap_err().code, GatewayErrorCode::PreconditionFailed);
    }

    #[test]
    fn job_get_tool_reports_not_found_for_an_unknown_id() {
        let registry = full_registry();
        let result = registry.call("job_get", serde_json::json!({ "jobId": "job_missing_entirely" })).expect("known tool name resolves");
        assert!(result.is_error);
        assert_eq!(result.structured_content.unwrap()["code"], "NOT_FOUND");
    }

    #[test]
    fn job_get_and_job_cancel_tools_round_trip_through_the_shared_registry() {
        let registry = full_registry();
        let job_id = job_registry().begin("ui.quick.test");
        let got = registry.call("job_get", serde_json::json!({ "jobId": job_id })).expect("known tool name resolves");
        assert!(!got.is_error, "{got:?}");
        assert_eq!(got.structured_content.as_ref().unwrap()["status"], "PENDING");
        let cancelled = registry.call("job_cancel", serde_json::json!({ "jobId": job_id })).expect("known tool name resolves");
        assert!(!cancelled.is_error, "{cancelled:?}");
        assert_eq!(cancelled.structured_content.unwrap()["status"], "CANCELLED");
    }

    #[test]
    fn read_ui_resource_for_a_known_job_id_reflects_the_shared_registry() {
        let job_id = job_registry().begin("ui.quick.test.resource");
        let outcome = read_ui_resource(&format!("semio://job/{job_id}"), None, None).expect("ours").expect("job exists");
        assert_eq!(outcome[0].uri, format!("semio://job/{job_id}"));
        assert!(outcome[0].text.as_ref().unwrap().contains("PENDING"));
    }
}
//#endregion 🧪️Tests
