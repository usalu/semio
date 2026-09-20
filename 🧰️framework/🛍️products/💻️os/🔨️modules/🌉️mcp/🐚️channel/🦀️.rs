//! 🐚️ The LIVE-SHELL artifact route — the `ArtifactChannel` that executes the mutation protocol
//! **in the user's running shell** instead of in this process's own headless interpreter.
//!
//! `📓️r2-reactor-retained-command-owner.md` §12 measured the gap this module closes: only
//! `ui_reveal`/`ui_focus` were ever shell-bound, so `artifact_open/create`, `action_prepare/invoke`,
//! `artifact_snapshot`, `history_undo/redo`, `transaction_*` and `artifact_export` all resolved
//! through `🏠️workspace::PluginArtifactChannel` → the headless `OwnedRuntime` interpreter, against
//! the gateway's OWN `--folder` workspace. That is a different document owner from the shell the
//! human is looking at: the agent's edit never appeared on screen, never entered the human's undo
//! history, and never replicated to a collaborator.
//!
//! The route is one more implementation of the SAME narrow port (`crate::actions::ArtifactChannel`,
//! one `exchange(instance, commands) -> Vec<AppFrame>`), so every tool above it — the whole
//! `🗿️artifact`/`🔀️dispatch` stack, its handles, its typed results, its approval gate — is unchanged.
//! What changes is who executes: [`ShellArtifactChannel`] encodes each `AppCommand` with this
//! module's own versioned JSON codec, sends it as a `GatewayToShell::AppCommand` frame carrying a
//! correlation `seq`, and blocks (bounded, never forever) on the `ShellToGateway::AppFrames` whose
//! `in_reply_to` equals that `seq`. The shell answers by dispatching through the very action lane
//! its own UI uses, so the human watches the edit happen and can undo it.
//!
//! **Selection is per session and is reported.** [`SessionChannelBinding`] decides ONCE — at
//! `context_resolve` time, or lazily on the first exchange if a client never resolves — between
//! [`ChannelKind::Shell`] (a live shell connection that declared `BridgeFlags::relay_app_commands`)
//! and [`ChannelKind::Headless`] (everything else), and `ContextSummary.channel` reports the answer
//! so a client can never be wrong about which document owner it is talking to. The headless channel
//! remains the path whenever no shell is attached — it is a fallback, not a deprecated lane.

use crate::actions::{AppCommand, AppFrame, ArtifactChannel, Fault, MutationOrigin, PreparedOps};
use crate::bridge::{BridgeHandle, BridgeInstanceRef, GatewayToShell, ShellConnectionId};
use crate::schema::RevisionStamp;
use crate::ui::BridgeSlot;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

//#region 🔖️Base64
/// 🔤️ The byte fields this codec carries (`ops` lanes, an artifact's pack/spr, exported media) are
/// opaque binary. JSON has no byte type, so they travel as standard base64 — 1.33× rather than the
/// ~4× a number array costs against a bridge outbox whose per-frame credit is fixed.
const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(BASE64_ALPHABET[(b0 >> 2) as usize] as char);
        out.push(BASE64_ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 { BASE64_ALPHABET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { BASE64_ALPHABET[(b2 & 0x3f) as usize] as char } else { '=' });
    }
    out
}

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

/// 📤️ Encodes one [`AppCommand`] as this codec's JSON payload. Field names are camelCase and the
/// `kind` discriminator is the variant's own name — the shared fixtures
/// (`🐚️channel/🧫️fixtures/🗿️app-commands.json`) are the SSOT both banks are tested against.
pub fn encode_app_command(command: &AppCommand) -> serde_json::Value {
    let body = match command {
        AppCommand::ReadHistory => serde_json::json!({ "kind": "readHistory" }),
        AppCommand::ReadArtifact => serde_json::json!({ "kind": "readArtifact" }),
        AppCommand::PureCommand { capability_id, input } => serde_json::json!({ "kind": "pureCommand", "capabilityId": capability_id, "input": input }),
        AppCommand::TransactionPrepare { txn_id, ops, label, origin } => {
            let MutationOrigin::Agent { principal, invocation_id } = origin;
            serde_json::json!({
                "kind": "transactionPrepare",
                "txnId": txn_id,
                "ops": encode_prepared_ops(ops),
                "label": label,
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
        // 💡️ An inference never travels the shell route: `ensure_inference_route` owns its own guest
        // and the shell has no seam to drive one. `ShellArtifactChannel::exchange` refuses it before
        // encoding, so this arm exists only to keep the match total.
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
            Ok(AppFrame::Emit { ops: PreparedOps { document: decode_lane(ops.get("document"))?, config: decode_lane(ops.get("config"))?, draft: decode_lane(ops.get("draft"))? }, warnings })
        }
        "transactionPrepared" => Ok(AppFrame::TransactionPrepared { txn_id: field_str(value, "txnId")? }),
        "transactionCommitted" => Ok(AppFrame::TransactionCommitted { txn_id: field_str(value, "txnId")?, edit_id: field_str(value, "editId")? }),
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
    bridge: Option<BridgeSlot>,
    decided: Mutex<Option<ChannelKind>>,
}

impl SessionChannelBinding {
    pub fn new(bridge: Option<BridgeSlot>) -> Self {
        Self { bridge, decided: Mutex::new(None) }
    }

    /// 🔎️ The shell connection this session routes through, when one is live AND declared the
    /// relay. A shell that never claimed `relayAppCommands` is a shell that cannot answer an
    /// `AppCommand`, so it is not a candidate.
    pub fn relay_connection(&self) -> Option<(Arc<BridgeHandle>, ShellConnectionId)> {
        let handle = self.bridge.as_ref()?.get()?;
        let id = crate::ui::active_shell_connection(handle)?;
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
const SHELL_APP_COMMAND_POLL_INTERVAL_MS: u64 = 10;

static SHELL_APP_COMMAND_SEQ: AtomicU64 = AtomicU64::new(1);

fn next_app_command_seq() -> u64 {
    SHELL_APP_COMMAND_SEQ.fetch_add(1, Ordering::Relaxed)
}

fn fault(code: &str, message: impl Into<String>) -> Fault {
    Fault { code: code.to_string(), message: message.into() }
}

/// 🐚️ The `ArtifactChannel` whose other end is the human's shell.
///
/// `instance` is the adapter's own opaque handle; the shell addresses its plugin instances by
/// string id (`BridgeInstanceRef.instance_id`). The mapping is resolved from the shell's own
/// `Instances` frame — by the owning plugin of the command's capability when the command carries
/// one, and otherwise by a sticky binding this channel remembers per `instance`, so every leg of a
/// prepare/commit/undo saga lands on the SAME live instance that ran the first leg.
pub struct ShellArtifactChannel {
    binding: Arc<SessionChannelBinding>,
    catalog: Arc<crate::catalog::Catalog>,
    bound: Mutex<HashMap<u32, String>>,
    /// 🎯️ The plugin this channel's artifact belongs to, when the caller already knows it — the
    /// artifact's own kind names its owner, and `artifact_create`/`artifact_open`/`artifact_snapshot`
    /// all resolve that owner before they open a channel. Without it a capability-less command
    /// (`ReadArtifact` carries no capability at all) had nothing to resolve an instance from and
    /// refused outright as soon as the shell had more than one program open — measured inside `s`
    /// with a spawned editor: `the attached shell has 3 open instances` (ticket 26/09/18 S6 §5.4).
    plugin: Option<String>,
    timeout: Duration,
}

impl ShellArtifactChannel {
    pub fn new(binding: Arc<SessionChannelBinding>, catalog: Arc<crate::catalog::Catalog>) -> Self {
        Self { binding, catalog, bound: Mutex::new(HashMap::new()), plugin: None, timeout: Duration::from_millis(SHELL_APP_COMMAND_TIMEOUT_MS) }
    }

    /// 🎯️ Pins the plugin whose artifact this channel drives, so a capability-less command resolves
    /// the same instance a capability-carrying one would.
    #[must_use]
    pub fn for_plugin(mut self, plugin_id: impl Into<String>) -> Self {
        self.plugin = Some(plugin_id.into());
        self
    }

    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 🔌️ The plugin id that owns `capability_id`, per the compiled catalog — the same resolution
    /// `RoutingArtifactChannel` performs, so the two channels can never disagree about ownership.
    fn owning_plugin(&self, capability_id: &str) -> Option<String> {
        match &self.catalog.get(capability_id)?.owner {
            crate::catalog::CapabilityOwner::Plugin { plugin_id, .. } => Some(plugin_id.clone()),
            _ => None,
        }
    }

    fn capability_of(command: &AppCommand) -> Option<&str> {
        match command {
            AppCommand::PureCommand { capability_id, .. } => Some(capability_id.as_str()),
            _ => None,
        }
    }

    /// 🎯️ Resolves the shell-side instance id this exchange addresses, and remembers it.
    fn resolve_instance_id(&self, handle: &BridgeHandle, connection: ShellConnectionId, instance: u32, command: &AppCommand) -> Result<String, Fault> {
        let entries = handle.last_instances(connection).unwrap_or_default();
        if entries.is_empty() {
            return Err(fault("plugin.unavailable", "the attached shell has published no open plugin instances yet — open an artifact in the shell (or wait for its first `Instances` frame) before driving it from an agent"));
        }
        if let Some(capability_id) = Self::capability_of(command) {
            if let Some(plugin_id) = self.owning_plugin(capability_id) {
                if let Some(entry) = entries.iter().find(|entry| entry.plugin_id == plugin_id) {
                    self.bound.lock().expect("shell channel instance binding lock poisoned").insert(instance, entry.instance_id.clone());
                    return Ok(entry.instance_id.clone());
                }
                return Err(fault(
                    "plugin.unavailable",
                    format!("capability `{capability_id}` is owned by plugin `{plugin_id}`, which has no open instance in the attached shell — open one there first (the shell reports {} open instance(s))", entries.len()),
                ));
            }
        }
        if let Some(existing) = self.bound.lock().expect("shell channel instance binding lock poisoned").get(&instance) {
            if entries.iter().any(|entry| &entry.instance_id == existing) {
                return Ok(existing.clone());
            }
        }
        if let Some(plugin_id) = self.plugin.as_deref() {
            let owned: Vec<&BridgeInstanceRef> = entries.iter().filter(|entry| entry.plugin_id == plugin_id).collect();
            // 🎯️ ONE open program of the artifact's own plugin is the artifact: bind it. Several is
            // genuinely ambiguous, and an ambiguous handle is refused BY NAME rather than resolved
            // silently — picking one would edit a document the agent never named.
            if owned.len() == 1 {
                let only = owned[0].instance_id.clone();
                self.bound.lock().expect("shell channel instance binding lock poisoned").insert(instance, only.clone());
                return Ok(only);
            }
            if owned.len() > 1 {
                let candidates = owned.iter().map(|entry| entry.artifact_ref.clone()).collect::<Vec<_>>().join(", ");
                return Err(fault(
                    "plugin.unavailable",
                    format!("this artifact handle names plugin `{plugin_id}`, which has {} open instances in the attached shell — name one by preparing an action against it (candidates: {candidates})", owned.len()),
                ));
            }
            return Err(fault(
                "plugin.unavailable",
                format!("this artifact handle names plugin `{plugin_id}`, which has no open instance in the attached shell — open one there first (the shell reports {} open instance(s))", entries.len()),
            ));
        }
        if entries.len() == 1 {
            let only = entries[0].instance_id.clone();
            self.bound.lock().expect("shell channel instance binding lock poisoned").insert(instance, only.clone());
            return Ok(only);
        }
        Err(fault(
            "plugin.unavailable",
            format!("this command carries no capability to resolve an instance from and the attached shell has {} open instances — prepare an action against the artifact first so the route binds", entries.len()),
        ))
    }

    /// 📤️ One correlated request/response round trip over the bridge.
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
                // 🛑️ A timed-out command is cancelled in the shell, not left running: the same
                // cooperative flag the human's own Cancel button flips, so a shell that is still
                // grinding on an abandoned command stops instead of committing it after the agent
                // gave up on it.
                handle.send_to(connection, GatewayToShell::AppCommand { seq, instance_id: instance_id.to_string(), command: serde_json::to_vec(&serde_json::json!({ "version": SHELL_CHANNEL_PAYLOAD_VERSION, "kind": "cancel", "cancelSeq": seq })).unwrap_or_default() });
                return Err(fault("budget.exceeded", format!("the attached shell did not answer this command within {}ms", self.timeout.as_millis())));
            }
            std::thread::sleep(Duration::from_millis(SHELL_APP_COMMAND_POLL_INTERVAL_MS));
        }
    }
}

impl ArtifactChannel for ShellArtifactChannel {
    fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault> {
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
}
//#endregion 🔖️ShellArtifactChannel

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
