//! 🐚️ The LIVE-SHELL artifact route — the `ArtifactChannel` that executes the mutation protocol [Design notes](README.md#source-notes-1).

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;
use semio_os_mcp_bridge::session::{BridgeHandle, ShellConnectionId};
use semio_os_mcp_bridge::shell_command_dispatch::{BridgeSlot, active_shell_connection};
use semio_os_mcp_bridge_codec::bridge::{BridgeInstanceRef, GatewayToShell};
use semio_os_mcp_catalog::catalog::{CapabilityOwner, Catalog};
use semio_os_mcp_foundation::base64_codec::{BASE64_ALPHABET, encode_base64};
use semio_os_mcp_foundation::reports::RevisionStamp;
use semio_os_mcp_port::channel_port::{ActivationScope, ActionTarget, AppCommand, AppFrame, ArtifactChannel, Fault, MutationOrigin, PreparedOps};
use semio_os_mcp_port::policy::AgentPrincipal;

//#region 🔖️Base64

/// 🔡️ Total inverse of [`encode_base64`] — `None` for any string that is not well-formed base64,
/// never a partial decode and never a panic (these bytes arrive from another process).
pub fn decode_base64(text: &str) -> Option<Vec<u8>> {
    let body = text.as_bytes();
    if body.len() % 4 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(body.len() / 4 * 3);
    for chunk in body.chunks(4) {
        let mut quad = [0u8; 4];
        let mut padding = 0usize;
        for (index, byte) in chunk.iter().enumerate() {
            if *byte == b'=' {
                if index < 2 {
                    return None;
                }
                padding += 1;
                quad[index] = 0;
                continue;
            }
            if padding > 0 {
                return None;
            }
            quad[index] = BASE64_ALPHABET.iter().position(|candidate| candidate == byte)? as u8;
        }
        let triple = ((quad[0] as u32) << 18) | ((quad[1] as u32) << 12) | ((quad[2] as u32) << 6) | quad[3] as u32;
        out.push((triple >> 16) as u8);
        if padding < 2 {
            out.push((triple >> 8) as u8);
        }
        if padding < 1 {
            out.push(triple as u8);
        }
    }
    Some(out)
}

//#endregion 🔖️Base64

//#region 🔖️PayloadCodec

/// 🏷️ The version every payload of this codec carries. The frame envelope (`🧵️bridge`) has its own
/// `BRIDGE_VERSION`; this is the *payload* contract inside `AppCommand.command` and each
/// `AppFrames.frames[i]`, which the two banks (`🐚️channel/🦀️.rs` and `🔗️AgentBridge/🟦️.tsx`) must
/// agree on independently of it. A shell that reads a version it does not know answers a typed
/// fault naming both versions rather than guessing.
pub const SHELL_CHANNEL_PAYLOAD_VERSION: u32 = 1;

/// 📤️ Encodes one [`AppCommand`] as this codec's JSON payload. Field names are camelCase and the [Design notes](README.md#source-notes-94).
pub fn encode_app_command(command: &AppCommand) -> serde_json::Value {
    let body = match command {
        AppCommand::ReadHistory => serde_json::json!({ "kind": "readHistory" }),
        AppCommand::ReadArtifact => serde_json::json!({ "kind": "readArtifact" }),
        AppCommand::PureCommand { capability_id, input, view_state } => serde_json::json!({ "kind": "pureCommand", "capabilityId": capability_id, "input": input, "context": view_state }),
        AppCommand::TransactionPrepare { txn_id, ops, origin } => {
            let MutationOrigin::Agent { principal, invocation_id } = origin;
            serde_json::json!({
                "kind": "transactionPrepare",
                "txnId": txn_id,
                "ops": encode_prepared_ops(ops),
                "origin": { "kind": "agent", "principal": principal, "invocationId": invocation_id },
            })
        }
        AppCommand::TransactionCommit { txn_id } => serde_json::json!({ "kind": "transactionCommit", "txnId": txn_id }),
        AppCommand::TransactionRollback { txn_id } => serde_json::json!({ "kind": "transactionRollback", "txnId": txn_id }),
        AppCommand::TransactionUndo { group_id } => serde_json::json!({ "kind": "transactionUndo", "groupId": group_id }),
        AppCommand::TransactionRedo { group_id } => serde_json::json!({ "kind": "transactionRedo", "groupId": group_id }),
        AppCommand::ExportMedia { port, document, document_spr } => {
            serde_json::json!({ "kind": "exportMedia", "port": port, "document": encode_base64(document), "documentSpr": encode_base64(document_spr) })
        }
        AppCommand::Infer(_) => serde_json::json!({ "kind": "infer" }),
    };
    let mut payload = body;
    if let Some(object) = payload.as_object_mut() {
        object.insert("version".to_string(), serde_json::json!(SHELL_CHANNEL_PAYLOAD_VERSION));
    }
    payload
}

fn encode_prepared_ops(ops: &PreparedOps) -> serde_json::Value {
    serde_json::json!({
        "document": ops.document.iter().map(|lane| encode_base64(lane)).collect::<Vec<_>>(),
        "config": ops.config.iter().map(|lane| encode_base64(lane)).collect::<Vec<_>>(),
        "draft": ops.draft.iter().map(|lane| encode_base64(lane)).collect::<Vec<_>>(),
    })
}

fn decode_lane(value: Option<&serde_json::Value>) -> Result<Vec<Vec<u8>>, String> {
    let Some(array) = value else { return Ok(Vec::new()) };
    let array = array.as_array().ok_or_else(|| "prepared-ops lane is not an array".to_string())?;
    array.iter().map(|entry| entry.as_str().and_then(decode_base64).ok_or_else(|| "prepared-ops lane entry is not base64".to_string())).collect()
}

fn field_str(value: &serde_json::Value, key: &str) -> Result<String, String> {
    value.get(key).and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(|| format!("shell AppFrame payload is missing the string field `{key}`"))
}

fn field_bytes(value: &serde_json::Value, key: &str) -> Result<Vec<u8>, String> {
    value.get(key).and_then(serde_json::Value::as_str).and_then(decode_base64).ok_or_else(|| format!("shell AppFrame payload field `{key}` is not base64"))
}

/// 📥️ Decodes one of the shell's reply payloads into the port's own [`AppFrame`]. Total: every
/// malformed shape answers `Err(String)` naming what was wrong, never a panic and never a
/// silently-substituted default — a shell that answers rubbish must not be able to look like a
/// successful mutation.
pub fn decode_app_frame(value: &serde_json::Value) -> Result<AppFrame, String> {
    let version = value.get("version").and_then(serde_json::Value::as_u64).unwrap_or(0);
    if version != u64::from(SHELL_CHANNEL_PAYLOAD_VERSION) {
        return Err(format!("shell AppFrame payload declares codec version {version}, this gateway speaks {SHELL_CHANNEL_PAYLOAD_VERSION}"));
    }
    let kind = value.get("kind").and_then(serde_json::Value::as_str).ok_or_else(|| "shell AppFrame payload has no `kind`".to_string())?;
    match kind {
        "historySnapshot" => Ok(AppFrame::HistorySnapshot(RevisionStamp { artifact_id: field_str(value, "artifactId")?, head_edit_id: field_str(value, "headEditId")?, cursor: field_str(value, "cursor")? })),
        "emit" => {
            let ops = value.get("ops").ok_or_else(|| "emit payload has no `ops`".to_string())?;
            let warnings = value.get("warnings").and_then(serde_json::Value::as_array).map(|entries| entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect()).unwrap_or_default();
            Ok(AppFrame::Emit { ops: PreparedOps { document: decode_lane(ops.get("document"))?, config: decode_lane(ops.get("config"))?, draft: decode_lane(ops.get("draft"))?, children: Vec::new() }, warnings, output: value.get("output").cloned() })
        }
        "transactionPrepared" => Ok(AppFrame::TransactionPrepared { txn_id: field_str(value, "txnId")? }),
        "transactionCommitted" => Ok(AppFrame::TransactionCommitted { txn_id: field_str(value, "txnId")?, edit_id: field_str(value, "editId")?, relay: None, output: value.get("output").cloned() }),
        "transactionRolledBack" => Ok(AppFrame::TransactionRolledBack { txn_id: field_str(value, "txnId")? }),
        "transactionUndone" => Ok(AppFrame::TransactionUndone { group_id: field_str(value, "groupId")? }),
        "transactionRedone" => Ok(AppFrame::TransactionRedone { group_id: field_str(value, "groupId")? }),
        "artifact" => Ok(AppFrame::Artifact { pack: field_bytes(value, "pack")?, spr: field_bytes(value, "spr")? }),
        "exported" => Ok(AppFrame::Exported { port: field_str(value, "port")?, descriptor: field_bytes(value, "descriptor")?, data: field_bytes(value, "data")? }),
        "error" => Ok(AppFrame::Error(Fault { code: field_str(value, "code")?, message: field_str(value, "message")? })),
        other => Err(format!("shell AppFrame payload declares unknown kind `{other}`")),
    }
}

//#endregion 🔖️PayloadCodec

//#region 🔖️Selection

/// 🔀️ Which document owner a session's artifact verbs execute against — the value
/// `ContextSummary.channel` reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelKind {
    /// 🐚️ The user's running shell owns the document; the agent's edits happen on screen.
    Shell,
    /// 🏠️ This process's own `--folder`/`--hub` workspace owns the document.
    Headless,
}

impl ChannelKind {
    pub fn label(self) -> &'static str {
        match self {
            ChannelKind::Shell => "shell",
            ChannelKind::Headless => "headless",
        }
    }
}

/// 🔒️ One session's channel decision, taken once and then sticky. Sticky is the point: a client
/// that resolved `channel: "shell"` and then saw the shell disconnect must get a typed, retryable
/// `PLUGIN_UNAVAILABLE` naming the lost shell — never a silent re-route onto a DIFFERENT document
/// owner in the middle of a transaction.
pub struct SessionChannelBinding {
    pub bridge: Option<BridgeSlot>,
    pub(crate) decided: Mutex<Option<ChannelKind>>,
}

impl SessionChannelBinding {
    pub fn new(bridge: Option<BridgeSlot>) -> Self {
        Self { bridge, decided: Mutex::new(None) }
    }

    /// 🤖️ The binding for `principal`'s artifact verbs. A delegated agent is its own hub participant: its edits run on
    /// its own hub-bound guests under its agent principal, never through the human's attached shell — that route would
    /// author them as the human, and would fail whenever the human has not opened the document. The attached shell still
    /// carries approvals, conversation and UI verbs (measured on the live user path, ticket 26/09/23, G12 session 14c).
    pub fn for_principal(principal: &AgentPrincipal, bridge: Option<BridgeSlot>) -> Self {
        Self::new(if principal.delegated_by.is_some() { None } else { bridge })
    }

    /// 🔎️ The shell connection this session routes through, when one is live AND declared the
    /// relay. A shell that never claimed `relayAppCommands` is a shell that cannot answer an
    /// `AppCommand`, so it is not a candidate.
    pub fn relay_connection(&self) -> Option<(Arc<BridgeHandle>, ShellConnectionId)> {
        let handle = self.bridge.as_ref()?.get()?;
        let id = active_shell_connection(handle)?;
        handle.shell_flags(id).relay_app_commands.then(|| (Arc::clone(handle), id))
    }

    /// 🧭️ Decides this session's channel, once. Called by `context_resolve` (so the answer it
    /// reports IS the answer every later tool call uses) and lazily by the channel itself for a
    /// client that never resolves a context.
    pub fn resolve(&self) -> ChannelKind {
        let mut decided = self.decided.lock().expect("session channel binding lock poisoned");
        match *decided {
            Some(kind) => kind,
            None => {
                let kind = if self.relay_connection().is_some() { ChannelKind::Shell } else { ChannelKind::Headless };
                *decided = Some(kind);
                kind
            }
        }
    }

    /// 👀️ The decision taken so far, without taking one.
    pub fn decided(&self) -> Option<ChannelKind> {
        *self.decided.lock().expect("session channel binding lock poisoned")
    }
}

//#endregion 🔖️Selection

//#region 🔖️ShellArtifactChannel

/// ⏱️ How long one `AppCommand` may wait for the shell's `AppFrames` reply. Generous compared with
/// `ui_focus`'s 4 s because this lane carries real mutations through a real plugin guest turn — but
/// bounded, because a shell that stops answering must not hang a tool call forever.
pub const SHELL_APP_COMMAND_TIMEOUT_MS: u64 = 60_000;

/// ⏳️ How long a command waits for the attached shell to publish an open instance of its plugin before it is refused
/// by name. A shell that just landed in a space is still opening the space's document when an agent's first edit
/// arrives — measured on the live user path (de, 7800 p24): the agent's `addBlock` 5 s after the landing found only
/// the space home, the note's instance followed moments later (ticket 26/09/23, G12 session 14c).
pub const SHELL_INSTANCE_WAIT_MS: u64 = 10_000;

const SHELL_APP_COMMAND_POLL_INTERVAL_MS: u64 = 10;

static SHELL_APP_COMMAND_SEQ: AtomicU64 = AtomicU64::new(1);

fn next_app_command_seq() -> u64 {
    SHELL_APP_COMMAND_SEQ.fetch_add(1, Ordering::Relaxed)
}

fn fault(code: &str, message: impl Into<String>) -> Fault {
    Fault { code: code.to_string(), message: message.into() }
}

/// 🐚️ The `ArtifactChannel` whose other end is the human's shell. [Design notes](README.md#source-notes-284).
pub struct ShellArtifactChannel {
    pub binding: Arc<SessionChannelBinding>,
    pub catalog: Arc<Catalog>,
    pub bound: Mutex<HashMap<u32, String>>,
    /// 🎯️ The plugin this channel's artifact belongs to, when the caller already knows it — the [Design notes](README.md#source-notes-295).
    plugin: Option<String>,
    targets: Mutex<HashMap<u32, ActionTarget>>,
    timeout: Duration,
    instance_wait: Duration,
}

/// 🎯️ One look at the shell's published instances: bound, not published yet (worth waiting for), or refused.
enum InstanceResolution {
    Bound(String),
    Pending(Fault),
    Refused(Fault),
}

impl ShellArtifactChannel {
    pub fn new(binding: Arc<SessionChannelBinding>, catalog: Arc<Catalog>) -> Self {
        Self { binding, catalog, bound: Mutex::new(HashMap::new()), plugin: None, targets: Mutex::new(HashMap::new()), timeout: Duration::from_millis(SHELL_APP_COMMAND_TIMEOUT_MS), instance_wait: Duration::from_millis(SHELL_INSTANCE_WAIT_MS) }
    }

    /// 🎯️ Pins the plugin whose artifact this channel drives, so a capability-less command resolves
    /// the same instance a capability-carrying one would.
    #[must_use]
    pub fn for_plugin(mut self, plugin_id: impl Into<String>) -> Self {
        self.plugin = Some(plugin_id.into());
        self
    }

    /// 🎯️ The same pin, per exchange — for `ShellRoutedArtifactChannel`, which serves EVERY plugin
    /// of a session and therefore cannot name one at construction. It resolves the owner the way the
    /// headless lane already does (the command's capability, else the instance slot `prepare_action`
    /// minted) and hands it over here, so both lanes answer "which instance is this?" identically.
    pub fn pin_plugin(&mut self, plugin_id: Option<String>) {
        self.plugin = plugin_id;
    }

    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    #[must_use]
    pub fn with_instance_wait(mut self, instance_wait: Duration) -> Self {
        self.instance_wait = instance_wait;
        self
    }

    /// 🔌️ The plugin id that owns `capability_id`, per the compiled catalog — the same resolution
    /// `RoutingArtifactChannel` performs, so the two channels can never disagree about ownership.
    fn owning_plugin(&self, capability_id: &str) -> Option<String> {
        match &self.catalog.get(capability_id)?.owner {
            CapabilityOwner::Plugin { plugin_id, .. } => Some(plugin_id.clone()),
            _ => None,
        }
    }

    fn capability_of(command: &AppCommand) -> Option<&str> {
        match command {
            AppCommand::PureCommand { capability_id, .. } => Some(capability_id.as_str()),
            _ => None,
        }
    }

    /// 🎯️ Resolves the shell-side instance id this exchange addresses, and remembers it — waiting up to
    /// [`SHELL_INSTANCE_WAIT_MS`] for the shell to publish an instance of the command's plugin, then refusing by name.
    ///
    /// 🎯️ ONE open program of the artifact's own plugin is the artifact: bind it. Several is
    /// genuinely ambiguous, and an ambiguous handle is refused BY NAME rather than resolved
    /// silently — picking one would edit a document the agent never named.
    fn resolve_instance_id(&self, handle: &BridgeHandle, connection: ShellConnectionId, instance: u32, command: &AppCommand) -> Result<String, Fault> {
        let deadline = Instant::now() + self.instance_wait;
        loop {
            match self.resolve_published_instance(handle, connection, instance, command) {
                InstanceResolution::Bound(instance_id) => return Ok(instance_id),
                InstanceResolution::Refused(fault) => return Err(fault),
                InstanceResolution::Pending(fault) if Instant::now() >= deadline => return Err(fault),
                InstanceResolution::Pending(_) => std::thread::sleep(Duration::from_millis(SHELL_APP_COMMAND_POLL_INTERVAL_MS)),
            }
        }
    }

    fn resolve_published_instance(&self, handle: &BridgeHandle, connection: ShellConnectionId, instance: u32, command: &AppCommand) -> InstanceResolution {
        let waited = self.instance_wait.as_millis();
        let entries = handle.last_instances(connection).unwrap_or_default();
        if entries.is_empty() {
            return InstanceResolution::Pending(fault("plugin.unavailable", format!("the attached shell has published no open plugin instances within {waited}ms — open an artifact in the shell before driving it from an agent")));
        }
        if let Some(existing) = self.bound.lock().expect("shell channel instance binding lock poisoned").get(&instance).cloned() {
            let target = self.targets.lock().expect("shell target lock poisoned").get(&instance).cloned();
            let entry = entries.iter().find(|entry| entry.instance_id == existing).filter(|entry| {
                Self::capability_of(command).is_none_or(|capability_id| self.catalog.get(capability_id).is_some_and(|capability| match &capability.owner {
                    CapabilityOwner::Plugin { plugin_id, app_id, .. } => &entry.plugin_id == plugin_id && app_id.as_ref().is_none_or(|app| &entry.app_id == app),
                    _ => false,
                }))
            });
            if let Some(entry) = entry.filter(|entry| target.as_ref().is_none_or(|target|
                target.plugin_id.as_ref().is_none_or(|id| &entry.plugin_id == id)
                && target.app_id.as_ref().is_none_or(|id| &entry.app_id == id)
                && target.artifact_ref.as_ref().is_none_or(|id| &entry.artifact_ref == id)
                && target.window_id.as_ref().is_none_or(|id| entry.window_ids.contains(id)))) {
                return InstanceResolution::Bound(entry.instance_id.clone());
            }
            return InstanceResolution::Refused(fault("plugin.unavailable", "the bound target disappeared or changed ownership; prepare a new action"));
        }
        if let Some(capability_id) = Self::capability_of(command) {
            if let Some(plugin_id) = self.owning_plugin(capability_id) {
                let app_id = self.catalog.get(capability_id).and_then(|capability| match &capability.owner { CapabilityOwner::Plugin { app_id, .. } => app_id.as_deref(), _ => None });
                let owned: Vec<_> = entries.iter().filter(|entry| entry.plugin_id == plugin_id && app_id.is_none_or(|id| entry.app_id == id)).collect();
                if owned.len() == 1 {
                    self.bound.lock().expect("shell channel instance binding lock poisoned").insert(instance, owned[0].instance_id.clone());
                    return InstanceResolution::Bound(owned[0].instance_id.clone());
                }
                if owned.len() > 1 {
                    return InstanceResolution::Refused(fault("plugin.unavailable", "capability has multiple live instances; supply an explicit target"));
                }
                return InstanceResolution::Pending(fault(
                    "plugin.unavailable",
                    format!("capability `{capability_id}` is owned by plugin `{plugin_id}`, which has no open instance in the attached shell within {waited}ms — open one there first (the shell reports {} open instance(s))", entries.len()),
                ));
            }
        }
        if let Some(existing) = self.bound.lock().expect("shell channel instance binding lock poisoned").get(&instance) {
            if entries.iter().any(|entry| &entry.instance_id == existing) {
                return InstanceResolution::Bound(existing.clone());
            }
        }
        if let Some(plugin_id) = self.plugin.as_deref() {
            let owned: Vec<&BridgeInstanceRef> = entries.iter().filter(|entry| entry.plugin_id == plugin_id).collect();
            if owned.len() == 1 {
                let only = owned[0].instance_id.clone();
                self.bound.lock().expect("shell channel instance binding lock poisoned").insert(instance, only.clone());
                return InstanceResolution::Bound(only);
            }
            if owned.len() > 1 {
                let candidates = owned.iter().map(|entry| entry.artifact_ref.clone()).collect::<Vec<_>>().join(", ");
                return InstanceResolution::Refused(fault(
                    "plugin.unavailable",
                    format!("this artifact handle names plugin `{plugin_id}`, which has {} open instances in the attached shell — name one by preparing an action against it (candidates: {candidates})", owned.len()),
                ));
            }
            return InstanceResolution::Pending(fault(
                "plugin.unavailable",
                format!("this artifact handle names plugin `{plugin_id}`, which has no open instance in the attached shell within {waited}ms — open one there first (the shell reports {} open instance(s))", entries.len()),
            ));
        }
        if entries.len() == 1 {
            let only = entries[0].instance_id.clone();
            self.bound.lock().expect("shell channel instance binding lock poisoned").insert(instance, only.clone());
            return InstanceResolution::Bound(only);
        }
        InstanceResolution::Refused(fault(
            "plugin.unavailable",
            format!("this command carries no capability to resolve an instance from and the attached shell has {} open instances — prepare an action against the artifact first so the route binds", entries.len()),
        ))
    }

    /// 📤️ One correlated request/response round trip over the bridge.
    ///
    /// 🛑️ A timed-out command is cancelled in the shell, not left running: the same
    /// cooperative flag the human's own Cancel button flips, so a shell that is still
    /// grinding on an abandoned command stops instead of committing it after the agent
    /// gave up on it.
    fn round_trip(&self, handle: &BridgeHandle, connection: ShellConnectionId, instance_id: &str, command: &AppCommand) -> Result<Vec<AppFrame>, Fault> {
        let seq = next_app_command_seq();
        let payload = serde_json::to_vec(&encode_app_command(command)).map_err(|error| fault("channel.not-wired", format!("encoding an AppCommand for the shell: {error}")))?;
        if !handle.send_to(connection, GatewayToShell::AppCommand { seq, instance_id: instance_id.to_string(), command: payload }) {
            return Err(fault("plugin.unavailable", "the attached shell's connection closed while dispatching this command"));
        }
        let deadline = Instant::now() + self.timeout;
        loop {
            if let Some((in_reply_to, replied_instance, frames)) = handle.last_app_frames(connection) {
                if in_reply_to == seq {
                    if replied_instance != instance_id {
                        return Err(fault("channel.not-wired", format!("the shell answered command {seq} for instance `{replied_instance}`, not the `{instance_id}` it was addressed to")));
                    }
                    return frames
                        .iter()
                        .map(|bytes| {
                            let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|error| fault("channel.not-wired", format!("the shell's reply frame is not JSON: {error}")))?;
                            decode_app_frame(&value).map_err(|message| fault("channel.not-wired", message))
                        })
                        .collect();
                }
            }
            if Instant::now() >= deadline {
                handle.send_to(connection, GatewayToShell::AppCommand { seq, instance_id: instance_id.to_string(), command: serde_json::to_vec(&serde_json::json!({ "version": SHELL_CHANNEL_PAYLOAD_VERSION, "kind": "cancel", "cancelSeq": seq })).unwrap_or_default() });
                return Err(fault("budget.exceeded", format!("the attached shell did not answer this command within {}ms", self.timeout.as_millis())));
            }
            std::thread::sleep(Duration::from_millis(SHELL_APP_COMMAND_POLL_INTERVAL_MS));
        }
    }
}

impl ArtifactChannel for ShellArtifactChannel {
    fn select_target(&mut self, instance: u32, capability_id: &str, target: &ActionTarget) -> Result<ActionTarget, Fault> {
        let capability = self.catalog.get(capability_id).ok_or_else(|| fault("capability.not-found", "unknown target capability"))?;
        let CapabilityOwner::Plugin { plugin_id, app_id, .. } = &capability.owner else {
            return Err(fault("plugin.unavailable", "target capability has no plugin owner"));
        };
        if target.plugin_id.as_ref().is_some_and(|id| id != plugin_id) || target.app_id.as_ref().is_some_and(|id| app_id.as_ref().is_some_and(|owner| owner != id)) {
            return Err(fault("capability-denied", "target plugin/app does not own this capability"));
        }
        let (handle, connection) = self.binding.relay_connection().ok_or_else(|| fault("plugin.unavailable", "target shell is disconnected"))?;
        let entries = handle.last_instances(connection).unwrap_or_default();
        let candidates: Vec<_> = entries.iter().filter(|entry|
            entry.plugin_id == *plugin_id
            && app_id.as_ref().is_none_or(|id| &entry.app_id == id)
            && target.instance_id.as_ref().is_none_or(|id| &entry.instance_id == id)
            && target.plugin_id.as_ref().is_none_or(|id| &entry.plugin_id == id)
            && target.app_id.as_ref().is_none_or(|id| &entry.app_id == id)
            && target.artifact_ref.as_ref().is_none_or(|id| &entry.artifact_ref == id)
            && target.window_id.as_ref().is_none_or(|id| entry.window_ids.contains(id))
        ).collect();
        if candidates.len() != 1 {
            return Err(fault("plugin.unavailable", format!("target resolves to {} live instances; exactly one is required", candidates.len())));
        }
        let selected = candidates[0].instance_id.clone();
        let mut bound = self.bound.lock().expect("shell channel instance binding lock poisoned");
        if bound.get(&instance).is_some_and(|existing| existing != &selected) {
            return Err(fault("capability-denied", "a prepared target cannot be rebound to another live instance"));
        }
        bound.insert(instance, selected.clone());
        let resolved = ActionTarget { instance_id: Some(selected), plugin_id: Some(candidates[0].plugin_id.clone()), app_id: Some(candidates[0].app_id.clone()), artifact_ref: Some(candidates[0].artifact_ref.clone()), window_id: target.window_id.clone(), mode_id: target.mode_id.clone() };
        self.targets.lock().expect("shell target lock poisoned").insert(instance, resolved.clone());
        Ok(resolved)
    }

    async fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault> {
        if commands.len() != 1 {
            return Err(fault("channel.not-wired", format!("the shell route sends exactly one command per exchange, received {}", commands.len())));
        }
        let command = commands.into_iter().next().expect("length checked above");
        if matches!(command, AppCommand::Infer(_)) {
            return Err(fault("plugin.unavailable", "inference never travels the shell route — the gateway owns its own inference guest; resolve a headless session for inference work"));
        }
        let Some((handle, connection)) = self.binding.relay_connection() else {
            return Err(fault(
                "plugin.unavailable",
                "this session resolved the `shell` channel and the shell is no longer attached — reconnect the shell and resolve a new context rather than silently editing a different document",
            ));
        };
        let instance_id = self.resolve_instance_id(&handle, connection, instance, &command)?;
        self.round_trip(&handle, connection, &instance_id, &command)
    }

    /// 🐚️ The attached shell owns this session's guest and its document; the gateway has nothing of
    /// its own to bring up, so activation only honours the caller's cancel.
    fn activate(&mut self, _instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        scope.checkpoint()
    }
}

//#endregion 🔖️ShellArtifactChannel

//#region 🧪️Tests
//#endregion 🧪️Tests
