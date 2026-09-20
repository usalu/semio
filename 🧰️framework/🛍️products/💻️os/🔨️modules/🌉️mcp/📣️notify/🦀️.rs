//! 📣️ Server→client notifications — the one lane every unprompted MCP message leaves this process
//! through (`notifications/resources/updated`, `notifications/resources/list_changed`,
//! `notifications/progress`), plus the bookkeeping that decides WHICH connection gets each one.
//!
//! Three real seams, no polling anywhere:
//! - [`NotificationSink`]/[`NotificationSlot`] — one per connection, filled by whichever transport
//!   owns the socket (`StdioLines` writes a line; `HttpEventPublisher` pushes onto the SSE event
//!   log). A server with no sink bound publishes nothing and costs nothing: every call site below is
//!   a no-op rather than a branch each caller re-invents.
//! - [`ResourceSubscriptions`] — one per connection, the live answer to `resources/subscribe`. The
//!   process-wide [`ResourceUpdateBroker`] holds WEAK references to every live one, so a closed
//!   connection's set disappears with its server rather than accumulating.
//! - [`ProgressScope`] — the `_meta.progressToken` of the `tools/call` currently running on THIS
//!   thread. A job minted inside that scope is bound to the token, so every later
//!   [`JobRegistry`](crate::ui::JobRegistry) progress report of that job id is pushed as a real
//!   `notifications/progress`, and an incoming `notifications/cancelled` naming that request id is
//!   mapped onto the very `job_registry().request_cancel` path `job_cancel` uses.

use crate::protocol::{JsonRpcNotification, NOTIFICATION_RESOURCES_LIST_CHANGED, NOTIFICATION_RESOURCES_UPDATED};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex, OnceLock, Weak};

//#region 🔖️NotificationSink
/// 📤️ One connection's server→client notification lane. The transport that owns the descriptors
/// implements it; nothing else in this crate ever writes to a client socket.
pub trait NotificationSink: Send + Sync {
    fn publish(&self, notification: JsonRpcNotification);
}

/// 🕳️ The late-filled, cloneable handle a transport publishes its sink into — same one-shot shape as
/// `BridgeSlot`/`ElicitationSlot`, so "no sink bound" stays an ordinary tier and never an error.
pub type NotificationSlot = Arc<OnceLock<Arc<dyn NotificationSink>>>;

/// 🆕️ A fresh, unfilled slot.
pub fn notification_slot() -> NotificationSlot {
    Arc::new(OnceLock::new())
}

/// 📤️ Publishes through `slot` when one is bound and filled — `false` means nobody was listening,
/// which is the ordinary headless/test tier and never a failure.
pub fn publish(slot: Option<&NotificationSlot>, notification: JsonRpcNotification) -> bool {
    match slot.and_then(|slot| slot.get()) {
        Some(sink) => {
            sink.publish(notification);
            true
        }
        None => false,
    }
}

/// 🧪️ A sink that keeps every notification in memory — the one a test asserts against, and the exact
/// same type the in-memory transport tests use, so nothing is proven against a bespoke double.
#[derive(Default)]
pub struct RecordingSink {
    published: Mutex<Vec<JsonRpcNotification>>,
}

impl RecordingSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn taken(&self) -> Vec<JsonRpcNotification> {
        std::mem::take(&mut *self.published.lock().expect("recording sink lock poisoned"))
    }

    pub fn seen(&self) -> Vec<JsonRpcNotification> {
        self.published.lock().expect("recording sink lock poisoned").clone()
    }
}

impl NotificationSink for RecordingSink {
    fn publish(&self, notification: JsonRpcNotification) {
        self.published.lock().expect("recording sink lock poisoned").push(notification);
    }
}
//#endregion 🔖️NotificationSink

//#region 🔖️ResourceSubscriptions
/// 🔔️ What changed about one resource — `Updated` is a content change at a URI a client may be
/// subscribed to, `ListChanged` is a change to WHICH resources exist at all (the roster itself).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceChange {
    Updated { uri: String },
    ListChanged,
}

/// 🔔️ One connection's live `resources/subscribe` set and the sink its notifications go to. The
/// set is authoritative: a `notifications/resources/updated` is published for a URI if and only if
/// THIS connection subscribed to it and has not unsubscribed, which is exactly what the
/// `"resources": {"subscribe": true}` capability promises.
pub struct ResourceSubscriptions {
    uris: Mutex<BTreeSet<String>>,
    sink: Mutex<Option<NotificationSlot>>,
}

impl Default for ResourceSubscriptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceSubscriptions {
    pub fn new() -> Self {
        Self { uris: Mutex::new(BTreeSet::new()), sink: Mutex::new(None) }
    }

    /// 🆕️ A fresh set already registered with the process-wide broker, so it starts receiving
    /// fan-out the moment it exists. The broker keeps only a `Weak`, so dropping the returned `Arc`
    /// (a closed connection) removes it.
    pub fn registered() -> Arc<Self> {
        let subscriptions = Arc::new(Self::new());
        resource_update_broker().register(&subscriptions);
        subscriptions
    }

    pub fn bind_sink(&self, slot: NotificationSlot) {
        *self.sink.lock().expect("subscription sink lock poisoned") = Some(slot);
    }

    /// ➕️ Records a subscription — `true` when it was not already held (the spec allows a repeat
    /// subscribe, and a repeat must not produce two notifications per change).
    pub fn subscribe(&self, uri: &str) -> bool {
        self.uris.lock().expect("subscription set lock poisoned").insert(uri.to_string())
    }

    /// ➖️ Drops a subscription — `false` when this connection did not hold it.
    pub fn unsubscribe(&self, uri: &str) -> bool {
        self.uris.lock().expect("subscription set lock poisoned").remove(uri)
    }

    pub fn is_subscribed(&self, uri: &str) -> bool {
        self.uris.lock().expect("subscription set lock poisoned").contains(uri)
    }

    pub fn subscribed(&self) -> Vec<String> {
        self.uris.lock().expect("subscription set lock poisoned").iter().cloned().collect()
    }

    fn slot(&self) -> Option<NotificationSlot> {
        self.sink.lock().expect("subscription sink lock poisoned").clone()
    }

    /// 📤️ Delivers one change to THIS connection if it is interested — `true` when a notification
    /// was actually written. `ListChanged` always goes out (the roster is not something a client
    /// subscribes to per-URI; `listChanged: true` is a blanket capability).
    pub fn deliver(&self, change: &ResourceChange) -> bool {
        let slot = self.slot();
        match change {
            ResourceChange::Updated { uri } => {
                if !self.is_subscribed(uri) {
                    return false;
                }
                publish(slot.as_ref(), JsonRpcNotification::new(NOTIFICATION_RESOURCES_UPDATED, Some(serde_json::json!({ "uri": uri }))))
            }
            ResourceChange::ListChanged => publish(slot.as_ref(), JsonRpcNotification::new(NOTIFICATION_RESOURCES_LIST_CHANGED, None)),
        }
    }
}

/// 🌍️ The one process-wide fan-out point every artifact change passes through. Weak references
/// only: a connection that closed is pruned on the next fan-out rather than kept alive by the
/// broker that merely wants to notify it.
#[derive(Default)]
pub struct ResourceUpdateBroker {
    subscribers: Mutex<Vec<Weak<ResourceSubscriptions>>>,
}

impl ResourceUpdateBroker {
    pub fn register(&self, subscriptions: &Arc<ResourceSubscriptions>) {
        self.subscribers.lock().expect("resource broker lock poisoned").push(Arc::downgrade(subscriptions));
    }

    /// 📢️ Fans one change out to every live connection, returning how many notifications were
    /// actually written (not how many connections exist) — the number a test asserts on.
    pub fn broadcast(&self, change: &ResourceChange) -> usize {
        let mut subscribers = self.subscribers.lock().expect("resource broker lock poisoned");
        subscribers.retain(|weak| weak.strong_count() > 0);
        let live: Vec<Arc<ResourceSubscriptions>> = subscribers.iter().filter_map(Weak::upgrade).collect();
        drop(subscribers);
        live.iter().filter(|subscriptions| subscriptions.deliver(change)).count()
    }

    pub fn live_connections(&self) -> usize {
        let mut subscribers = self.subscribers.lock().expect("resource broker lock poisoned");
        subscribers.retain(|weak| weak.strong_count() > 0);
        subscribers.len()
    }
}

static RESOURCE_UPDATE_BROKER: OnceLock<ResourceUpdateBroker> = OnceLock::new();

pub fn resource_update_broker() -> &'static ResourceUpdateBroker {
    RESOURCE_UPDATE_BROKER.get_or_init(ResourceUpdateBroker::default)
}

/// 🗿️ One artifact's content changed (a committed mutation, an undo, a redo, a rollback, an edit
/// that arrived through the workspace) — every URI that projects that artifact is updated, because a
/// client watching `semio://artifact/{id}/history` is watching the same change as one watching
/// `semio://artifact/{id}`.
pub fn artifact_changed(artifact_id: &str) -> usize {
    let broker = resource_update_broker();
    artifact_resource_uris(artifact_id).iter().map(|uri| broker.broadcast(&ResourceChange::Updated { uri: uri.clone() })).sum()
}

/// 🗿️ The resource roster itself changed (an artifact was created or removed) — every connection
/// gets `notifications/resources/list_changed`, and the workspace projections are `updated` too.
pub fn artifact_roster_changed() -> usize {
    resource_update_broker().broadcast(&ResourceChange::ListChanged)
        + resource_update_broker().broadcast(&ResourceChange::Updated { uri: "semio://workspace".to_string() })
        + resource_update_broker().broadcast(&ResourceChange::Updated { uri: "semio://workspace/artifacts".to_string() })
}

/// 🔗️ Every resource URI this crate projects for one artifact id, in the order `resources/list` and
/// `📇️registry`'s templates declare them.
pub fn artifact_resource_uris(artifact_id: &str) -> Vec<String> {
    vec![
        format!("semio://artifact/{artifact_id}"),
        format!("semio://artifact/{artifact_id}/history"),
        format!("semio://artifact/{artifact_id}/validation"),
        format!("semio://artifact/{artifact_id}/inference"),
    ]
}
//#endregion 🔖️ResourceSubscriptions

//#region 🔖️ToolResultChanges
/// 🔁️ The DECLARED table of which tool result changes which resource — never inferred from a
/// result's shape. Each row names the tool and where in its own `structuredContent` the affected
/// artifact id lives, so a new mutation tool must add a row here rather than silently going quiet.
/// `roster` marks the two tools that change WHICH artifacts exist at all.
struct ToolResourceEffect {
    tool: &'static str,
    artifact_id_at: &'static [&'static str],
    roster: bool,
}

const TOOL_RESOURCE_EFFECTS: &[ToolResourceEffect] = &[
    ToolResourceEffect { tool: "action_invoke", artifact_id_at: &["revisionAfter/artifactId", "revisionBefore/artifactId"], roster: false },
    ToolResourceEffect { tool: "transaction_commit", artifact_id_at: &["revisionAfter/artifactId", "revisionBefore/artifactId"], roster: false },
    ToolResourceEffect { tool: "transaction_rollback", artifact_id_at: &["revisionAfter/artifactId", "revisionBefore/artifactId"], roster: false },
    ToolResourceEffect { tool: "history_undo", artifact_id_at: &["revisionAfter/artifactId", "artifactId"], roster: false },
    ToolResourceEffect { tool: "history_redo", artifact_id_at: &["revisionAfter/artifactId", "artifactId"], roster: false },
    ToolResourceEffect { tool: "artifact_create", artifact_id_at: &["artifactId"], roster: true },
    ToolResourceEffect { tool: "artifact_open", artifact_id_at: &["artifactId"], roster: true },
];

fn pointer_lookup<'a>(value: &'a serde_json::Value, path: &str) -> Option<&'a str> {
    let mut cursor = value;
    for segment in path.split('/') {
        cursor = cursor.get(segment)?;
    }
    cursor.as_str()
}

/// 🔁️ What one completed `tools/call` changed, per the declared table above. An `isError` result and
/// a tool with no row change nothing — the empty vec is the common case and costs one table scan.
pub fn changes_from_tool_result(tool: &str, is_error: bool, structured: Option<&serde_json::Value>) -> Vec<ResourceChange> {
    if is_error {
        return Vec::new();
    }
    let Some(effect) = TOOL_RESOURCE_EFFECTS.iter().find(|effect| effect.tool == tool) else {
        return Vec::new();
    };
    let Some(structured) = structured else {
        return Vec::new();
    };
    let Some(artifact_id) = effect.artifact_id_at.iter().find_map(|path| pointer_lookup(structured, path)) else {
        return Vec::new();
    };
    let mut changes: Vec<ResourceChange> = artifact_resource_uris(artifact_id).into_iter().map(|uri| ResourceChange::Updated { uri }).collect();
    if effect.roster {
        changes.push(ResourceChange::ListChanged);
        changes.push(ResourceChange::Updated { uri: "semio://workspace".to_string() });
        changes.push(ResourceChange::Updated { uri: "semio://workspace/artifacts".to_string() });
    }
    changes
}

/// 📢️ Applies [`changes_from_tool_result`] to the process-wide broker — the single call
/// `McpServer::handle_tools_call` makes after every tool call, returning how many notifications
/// actually reached a subscribed client.
pub fn broadcast_tool_result_changes(tool: &str, is_error: bool, structured: Option<&serde_json::Value>) -> usize {
    changes_from_tool_result(tool, is_error, structured).iter().map(|change| resource_update_broker().broadcast(change)).sum()
}
//#endregion 🔖️ToolResultChanges

//#region 🔖️Progress
pub const NOTIFICATION_PROGRESS: &str = "notifications/progress";

/// 📈️ What the `tools/call` running on this thread asked to be told about: the client's own opaque
/// `_meta.progressToken` and the connection to push it down. Both are needed — a token with no sink
/// has nowhere to go, and a sink with no token has nothing the client could correlate.
#[derive(Clone)]
pub struct ProgressBinding {
    pub token: serde_json::Value,
    pub request_id: Option<serde_json::Value>,
    pub slot: NotificationSlot,
}

thread_local! {
    static ACTIVE_PROGRESS: RefCell<Option<ProgressBinding>> = const { RefCell::new(None) };
}

/// 🧷️ RAII guard for the thread-local binding — restoring the previous value on drop rather than
/// clearing it, so a nested call can never silence its caller's progress.
pub struct ProgressScope {
    previous: Option<ProgressBinding>,
}

impl Drop for ProgressScope {
    fn drop(&mut self) {
        let previous = self.previous.take();
        ACTIVE_PROGRESS.with(|active| *active.borrow_mut() = previous);
    }
}

/// 📈️ Enters the progress scope for one `tools/call`. `binding` is `None` for the ordinary call that
/// carried no `_meta.progressToken`, and the guard still restores correctly.
pub fn enter_progress_scope(binding: Option<ProgressBinding>) -> ProgressScope {
    let previous = ACTIVE_PROGRESS.with(|active| std::mem::replace(&mut *active.borrow_mut(), binding));
    ProgressScope { previous }
}

pub fn active_progress_binding() -> Option<ProgressBinding> {
    ACTIVE_PROGRESS.with(|active| active.borrow().clone())
}

/// 🔗️ Job id → the progress binding of the `tools/call` that minted it, plus request id → the job
/// ids minted under it. Both are process-wide because the job registry itself is, and because an
/// incoming `notifications/cancelled` arrives on the transport thread, not the tool's thread.
#[derive(Default)]
struct ProgressBindings {
    by_job: BTreeMap<String, ProgressBinding>,
    jobs_by_request: BTreeMap<String, Vec<String>>,
}

static PROGRESS_BINDINGS: OnceLock<Mutex<ProgressBindings>> = OnceLock::new();

fn progress_bindings() -> &'static Mutex<ProgressBindings> {
    PROGRESS_BINDINGS.get_or_init(|| Mutex::new(ProgressBindings::default()))
}

fn request_key(request_id: &serde_json::Value) -> String {
    request_id.to_string()
}

/// 🔗️ Binds a freshly-minted job id to whatever progress scope this thread is in — called by
/// `JobRegistry::begin_with_id`, so EVERY job producer in the crate is covered by construction
/// rather than each one remembering to opt in. Outside a scope this is a no-op.
pub fn bind_job_to_active_scope(job_id: &str) {
    let Some(binding) = active_progress_binding() else { return };
    let mut bindings = progress_bindings().lock().expect("progress binding lock poisoned");
    if let Some(request_id) = binding.request_id.as_ref() {
        bindings.jobs_by_request.entry(request_key(request_id)).or_default().push(job_id.to_string());
    }
    bindings.by_job.insert(job_id.to_string(), binding);
}

/// 📈️ Publishes one `notifications/progress` for `job_id` if it was minted under a progress token —
/// called by `JobRegistry::report_progress` and by its terminal transitions, so the client sees the
/// same progression `job_get` polling would have shown, pushed instead of polled.
pub fn job_progress_changed(job_id: &str, progress: f64, message: Option<&str>) -> bool {
    let binding = {
        let bindings = progress_bindings().lock().expect("progress binding lock poisoned");
        bindings.by_job.get(job_id).cloned()
    };
    let Some(binding) = binding else { return false };
    let mut params = serde_json::json!({ "progressToken": binding.token, "progress": progress.clamp(0.0, 1.0), "total": 1.0 });
    if let Some(message) = message {
        if let Some(object) = params.as_object_mut() {
            object.insert("message".to_string(), serde_json::json!(message));
        }
    }
    publish(Some(&binding.slot), JsonRpcNotification::new(NOTIFICATION_PROGRESS, Some(params)))
}

/// 🧹️ Forgets a terminal job's binding — a long-lived stdio process must not accumulate one entry
/// per job it ever ran.
pub fn release_job_binding(job_id: &str) {
    let mut bindings = progress_bindings().lock().expect("progress binding lock poisoned");
    bindings.by_job.remove(job_id);
    for jobs in bindings.jobs_by_request.values_mut() {
        jobs.retain(|candidate| candidate != job_id);
    }
    bindings.jobs_by_request.retain(|_, jobs| !jobs.is_empty());
}

/// 🛑️ Every job id minted under `request_id` — what `notifications/cancelled` maps onto.
pub fn jobs_for_request(request_id: &serde_json::Value) -> Vec<String> {
    let bindings = progress_bindings().lock().expect("progress binding lock poisoned");
    bindings.jobs_by_request.get(&request_key(request_id)).cloned().unwrap_or_default()
}
//#endregion 🔖️Progress

//#region 🔖️Pagination
/// 📄️ The default page every `*/list` method serves when the client names no `cursor`. Large enough
/// that today's 27 tools and small resource roster are one page (no client sees a behaviour change),
/// small enough that a spec-strict client's paging code is genuinely exercised by a grown catalog.
pub const DEFAULT_PAGE_SIZE: usize = 100;

/// 📄️ An opaque cursor. It is opaque to the CLIENT (the spec's requirement) but not magic: it is the
/// zero-padded next offset behind a fixed prefix, so a malformed or foreign cursor is a loud
/// `INVALID_PARAMS` rather than a silently-wrong page.
const CURSOR_PREFIX: &str = "semio.page.";

pub fn encode_cursor(next_offset: usize) -> String {
    format!("{CURSOR_PREFIX}{next_offset:08}")
}

pub fn decode_cursor(cursor: &str) -> Option<usize> {
    cursor.strip_prefix(CURSOR_PREFIX)?.parse::<usize>().ok()
}

/// 📄️ One page of an already-stably-ordered list, plus the cursor that continues it. An offset past
/// the end is an empty final page, never an error — a client that keeps a cursor across a shrinking
/// list must be able to finish its walk.
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

pub fn paginate<T>(items: Vec<T>, offset: usize, page_size: usize) -> Page<T> {
    let page_size = page_size.max(1);
    let total = items.len();
    if offset >= total {
        return Page { items: Vec::new(), next_cursor: None };
    }
    let end = offset.saturating_add(page_size).min(total);
    let items: Vec<T> = items.into_iter().skip(offset).take(end - offset).collect();
    let next_cursor = if end < total { Some(encode_cursor(end)) } else { None };
    Page { items, next_cursor }
}
//#endregion 🔖️Pagination

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
