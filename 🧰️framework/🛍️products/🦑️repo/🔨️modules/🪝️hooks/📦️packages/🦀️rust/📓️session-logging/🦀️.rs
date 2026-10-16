//! 📓️ Session logging: the session event log, its stores and the hook recorder.
//!
//! Holds the `📓️SessionLogging` region of the crate root; the root re-exports its items.

use super::*;

//#region 📓️SessionLogging
/// 📍️ One recorded hook invocation in a session log.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionEventEntry {
    #[serde(rename = "event")]
    pub event: Value,
    #[serde(rename = "native", default, skip_serializing_if = "Option::is_none")]
    pub native: Option<SessionNativeEnvelope>,
    #[serde(rename = "response", default, skip_serializing_if = "Option::is_none")]
    pub response: Option<SessionResponse>,
}

/// 🧾️ The native payload, kept only at `full` detail.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionNativeEnvelope {
    #[serde(rename = "event")]
    pub event: Value,
}

/// 📤️ What the hook answered, kept at `standard` and `full` detail.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionResponse {
    #[serde(rename = "blocked", default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "message", default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(rename = "reason", default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

/// 🔸️ The `session.json` record of one agent session.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionMeta {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "uri", default, skip_serializing_if = "String::is_empty")]
    pub uri: String,
    #[serde(rename = "client", default, skip_serializing_if = "String::is_empty")]
    pub client: String,
    #[serde(rename = "second", default, skip_serializing_if = "String::is_empty")]
    pub second: String,
    #[serde(rename = "checkpoint", default, skip_serializing_if = "String::is_empty")]
    pub checkpoint: String,
    #[serde(rename = "contributor", default, skip_serializing_if = "String::is_empty")]
    pub contributor: String,
    #[serde(rename = "transcript", default, skip_serializing_if = "String::is_empty")]
    pub transcript: String,
    #[serde(rename = "events", default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<SessionEventEntry>,
    #[serde(rename = "plan", default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TicketAgentPlan>,
}

/// 💾️ Where a session log lives. The filesystem implementation is one of several: a caller under
/// test hands in a memory store and never touches a disk.
pub trait SessionStore {
    /// 📥️ The recorded session, when one exists.
    fn read(&self, session_id: &str) -> Option<SessionMeta>;
    /// 📤️ Replaces the recorded session.
    fn write(&mut self, session_id: &str, meta: &SessionMeta) -> HookOutcome<()>;
}

/// 🧠️ An in-memory session store — the deterministic default for tests and dry runs.
#[derive(Debug, Default, Clone)]
pub struct MemorySessionStore {
    pub(super) sessions: BTreeMap<String, SessionMeta>,
}

impl MemorySessionStore {
    /// 🆕️ An empty store.
    pub fn new() -> MemorySessionStore {
        MemorySessionStore::default()
    }

    /// 📋️ Every recorded session, by id.
    pub fn sessions(&self) -> &BTreeMap<String, SessionMeta> {
        &self.sessions
    }
}

impl SessionStore for MemorySessionStore {
    fn read(&self, session_id: &str) -> Option<SessionMeta> {
        self.sessions.get(session_id).cloned()
    }

    fn write(&mut self, session_id: &str, meta: &SessionMeta) -> HookOutcome<()> {
        self.sessions.insert(session_id.to_string(), meta.clone());
        Ok(())
    }
}

/// 💽️ A session store rooted at one directory, writing `<root>/<session id>/session.json`.
#[derive(Debug, Clone)]
pub struct DirectorySessionStore {
    root: PathBuf,
}

impl DirectorySessionStore {
    /// 🆕️ A store under `root`.
    pub fn new(root: impl Into<PathBuf>) -> DirectorySessionStore {
        DirectorySessionStore { root: root.into() }
    }

    /// 📄️ The file one session is recorded in.
    pub fn path(&self, session_id: &str) -> PathBuf {
        self.root.join(session_id).join(SESSION_FILE_NAME)
    }
}

impl SessionStore for DirectorySessionStore {
    fn read(&self, session_id: &str) -> Option<SessionMeta> {
        let raw = std::fs::read_to_string(self.path(session_id)).ok()?;
        serde_json::from_str(&raw).ok()
    }

    fn write(&mut self, session_id: &str, meta: &SessionMeta) -> HookOutcome<()> {
        let path = self.path(session_id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| HookError::new(format!("cannot create {}: {error}", parent.display())))?;
        }
        let document = serde_json::to_string_pretty(meta).map_err(|error| HookError::new(error.to_string()))?;
        std::fs::write(&path, document).map_err(|error| HookError::new(format!("cannot write {}: {error}", path.display())))
    }
}

/// 📄️ The file name a session log is recorded in.
pub const SESSION_FILE_NAME: &str = "session.json";

/// 📁️ The directory a session log lives in: `.🧬semio/🦑️repo/⚡️cache/🤖️generated/YY/MM/DD/<session>`.
pub fn session_log_dir(repo_root: &Path, year: i32, month: u32, day: u32, session_id: &str) -> PathBuf {
    repo_meta_dir_for_root(repo_root).join("⚡️cache").join("🤖️generated").join(format!("🎆️{:02}", year % 100)).join(format!("🌙️{month:02}")).join(format!("☀️{day:02}")).join(session_id)
}

/// 🪪️ The session id a log is filed under: the payload's, `kiro-<pid>` for the Kiro CLI, otherwise
/// the literal `unknown` — a hook is never dropped just because its client forgot to identify itself.
pub fn resolve_log_session_id(context: &HookContext, kiro_parent_pid: Option<u32>) -> String {
    let session = extract_session_id(context.input.as_ref());
    if !session.is_empty() {
        return session;
    }
    if context.client == "kiro-cli" {
        if let Some(pid) = kiro_parent_pid {
            return format!("kiro-{pid}");
        }
    }
    "unknown".to_string()
}

/// 📓️ Appends one hook invocation to a session log, creating the record on first sight.
///
/// Nothing is written for a `version.*` event or when `[logging] session` is off. `detail` decides
/// how much of each entry survives: `minimal` keeps only the neutral event, `standard` adds the
/// response, `full` adds the native payload as well. `[logging] plan` folds a finished plan update
/// into the session's plan.
pub fn record_session_hook(store: &mut dyn SessionStore, context: &HookContext, result: &HookResult, session_id: &str, logging: &LoggingConfig, environment: &dyn HookEnvironment) -> HookOutcome<Option<SessionMeta>> {
    let Some(event) = context.hook_event() else { return Ok(None) };
    if hook_event_kind(event) == HookKind::Version || !logging.session {
        return Ok(None);
    }
    let input = context.input.as_ref();
    let mut meta = store.read(session_id).unwrap_or_default();
    let second = first_non_empty(&[context.second.clone(), find_nested_string_value(input, &["second"])]);
    let checkpoint = extract_checkpoint_id(input, environment);
    let transcript = extract_transcript(input);
    if meta.id.is_empty() {
        meta.id = session_id.to_string();
        meta.uri = format!("repo://session/{session_id}");
        meta.client = context.client.clone();
        meta.second = second.clone();
        meta.checkpoint = checkpoint.clone();
        meta.contributor = "unknown".to_string();
        meta.transcript = transcript.clone();
    } else {
        if meta.client.is_empty() {
            meta.client = context.client.clone();
        }
        if meta.second.is_empty() {
            meta.second = second.clone();
        }
        if meta.checkpoint.is_empty() {
            meta.checkpoint = checkpoint.clone();
        }
        if meta.transcript.is_empty() {
            meta.transcript = transcript.clone();
        }
    }
    let mut neutral = Map::new();
    neutral.insert("kind".to_string(), Value::String(event.as_str().to_string()));
    neutral.insert("client".to_string(), Value::String(context.client.clone()));
    neutral.insert("session".to_string(), Value::String(session_id.to_string()));
    neutral.insert("second".to_string(), Value::String(second.clone()));
    neutral.insert("checkpoint".to_string(), Value::String(checkpoint));
    neutral.insert("contributor".to_string(), Value::String("unknown".to_string()));
    if !transcript.is_empty() {
        neutral.insert("transcript".to_string(), Value::String(transcript));
    }
    let mut entry = SessionEventEntry { event: Value::Object(neutral), native: None, response: None };
    if logging.include_native() {
        if let Some(input) = input {
            entry.native = Some(SessionNativeEnvelope { event: input.clone() });
        }
    }
    if logging.include_response() {
        entry.response = Some(if result.allowed {
            SessionResponse::default()
        } else {
            SessionResponse { blocked: Some(true), message: result.message.clone(), reason: result.message.clone() }
        });
    }
    meta.events.push(entry);
    if logging.plan && event == HookEvent::AgentToolPlanUpdatingEnded {
        let steps = extract_plan_steps(input, &context.tool_args);
        if !steps.is_empty() {
            let existing = meta.plan.as_ref().map(|plan| plan.steps.clone()).unwrap_or_default();
            meta.plan = Some(TicketAgentPlan { steps: merge_plan_steps(&existing, &steps, &second) });
        }
    }
    store.write(session_id, &meta)?;
    Ok(Some(meta))
}
//#endregion 📓️SessionLogging
