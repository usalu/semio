//! 🤝️ Installed artifact kinds, session document pairs and the hub relay with its inbound planning.

#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
use std::sync::Mutex;
use semio_framework_os_mcp_port::channel_port::{AppCommand, Fault, HubRelayOutcome};
#[cfg(not(target_arch = "wasm32"))]
use crate::routing_and_shell_routed::PluginArtifactBinding;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_port::channel_port::{AppFrame, HUB_EDIT_UNBOUND_FAULT_CODE, VIEWER_READ_ONLY_FAULT_CODE};

/// 🗂️ One artifact kind an installed plugin really declares — the row `artifact_create{kind}`
/// validates against and `artifact_export` routes through.
#[derive(Clone, Debug)]
pub struct InstalledArtifactKind {
    pub schema: String,
    pub plugin_id: String,
    pub app_id: String,
    pub media_out_ports: Vec<String>,
    pub export_formats: Vec<String>,
}

/// 📦️ One document's authoritative binary pair — the exact `(pack, spr)` `store::os_store::text_format::print_document_pack`
/// writes and the hub's `active-checkpoint/pair` route serves. Held behind an `Arc` on a
/// [`PluginArtifactBinding`] so the ~80 KB of a real document is read from the hub ONCE per open and
/// then only ever cloned into the single archive load that seeds a guest with it.
#[derive(Debug, PartialEq, Eq)]
pub struct SessionDocumentPair {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}

/// 🗂️ One plugin-typed artifact's real ownership: the artifact kind its owning plugin declares
/// (`ArtifactKindSpec.kind_id`/`AppIo.artifact_schema`), that plugin's id, and the app whose media
/// ports can export it.
///
/// 🌎️ For a HUB document the binding additionally carries the surface the hub's own execution-target
/// lease names (`DocumentOpenSurfaceV1.surface_id`) and the document's
/// canonical pair, so the plugin's guest can be seeded with THE HUB'S document rather than with the
/// plugin's genesis — see [`RoutingArtifactChannel::exchange`]. A folder-created artifact has neither:
/// it has no hub surface, and its bytes already live in this workspace's own event log.
/// 🚦️ What this session last heard from a hub document's own actor: its sync status (`version`
/// counts every status it reported) and the last coded fault it raised, e.g. a link that turned
/// terminal. [`HubRelay::await_acknowledged`] is how a commit learns whether the hub HAS the edit.
#[derive(Default, Debug)]
pub struct HubRelay {
    pub state: Mutex<HubRelayState>,
    changed: std::sync::Condvar,
    pub inbound: Mutex<std::collections::VecDeque<HubInbound>>,
}

/// 📥️ What a hub document's actor delivered FOR the guest, in the order it delivered it: the hub's
/// canonical baseline it installed (`ArtifactEvent::DocumentArchiveReplaced`) and every mutation batch
/// it received after it — the bootstrap tail and every other writer's live edits
/// (`ArtifactEvent::DocumentBackbone`). The guest is the only reader; see [`plan_hub_inbound`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubInbound {
    Archive(Vec<u8>),
    Backbone(Vec<u8>),
}

/// 📥️ One take of a bound hub document's deliveries: the relay they came from (batches a guest refuses go
/// back to it), whether the binding's pair was just replaced by a new baseline, and the batches after it.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) struct HubInboundDelivery {
    pub relay: Arc<HubRelay>,
    pub(crate) reseeded: bool,
    pub backbone: Vec<Vec<u8>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl HubInboundDelivery {
    /// ↩️ Hands every batch of this take back to its relay, untouched, for the next command.
    pub(crate) fn restore(self) {
        self.relay.restore_inbound(self.backbone.into_iter().map(HubInbound::Backbone).collect());
    }
}

/// 🧭️ What the guest still needs before its next command: a new baseline to reseed from, and the
/// actor's mutation batches after it, oldest first.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct HubInboundPlan {
    pub reseed: Option<SessionDocumentPair>,
    pub backbone: Vec<Vec<u8>>,
}

/// 🧭️ Folds what the actor delivered against the pair the guest was seeded from. The NEWEST archive
/// supersedes everything delivered before it and reseeds the guest unless it is byte-identical to that
/// seed (the usual first bootstrap: the actor installs the same checkpoint `artifact_open` read); the
/// actor never re-delivers an operation this replica authored or already applied, so the batches after
/// it are exactly what the guest lacks, delivered in order.
///
/// 🐛️ Without this a hub-bound agent acted on the hub's last CHECKPOINT forever: the guest was seeded
/// once from the canonical pair and nothing the actor received ever reached it, so a fresh agent session
/// re-derived the same state — and the same ids (`note`'s `text-<scope>-<block count>`) — as every
/// session before it, and the hub accepted each as a distinct op (WG9 15:4x, G12 ids probe 18:4x).
pub fn plan_hub_inbound(items: Vec<HubInbound>, seeded: &SessionDocumentPair) -> Result<HubInboundPlan, Fault> {
    let newest_archive = items.iter().rposition(|item| matches!(item, HubInbound::Archive(_)));
    let mut plan = HubInboundPlan::default();
    for (index, item) in items.into_iter().enumerate() {
        match item {
            HubInbound::Archive(bytes) if Some(index) == newest_archive => {
                let archive = ::semio_framework_async::poll::resolve_ready(store::decode_document_archive_bytes(&bytes)).map_err(|error| Fault { code: "channel.not-wired".to_string(), message: format!("the hub document actor delivered an undecodable baseline: {error}") })?;
                if !archive.members.is_empty() {
                    return Err(Fault { code: "channel.not-wired".to_string(), message: format!("the hub document actor delivered a baseline with {} owned member(s); a headless session document holds one root pair", archive.members.len()) });
                }
                let unchanged = archive.parent_pack == seeded.pack && archive.parent_spr == seeded.spr;
                plan.reseed = (!unchanged).then_some(SessionDocumentPair { pack: archive.parent_pack, spr: archive.parent_spr });
            }
            HubInbound::Archive(_) => {}
            HubInbound::Backbone(message) if newest_archive.is_none_or(|archive| index > archive) => plan.backbone.push(message),
            HubInbound::Backbone(_) => {}
        }
    }
    Ok(plan)
}

/// 🧵️ Whether a guest may take the actor's inbound batches before `commands`: never while it finishes a
/// transaction (a commit or rollback runs against exactly the state its prepare saw), never on the
/// inference lane (its own guest never holds this document).
pub fn carries_hub_inbound(commands: &[AppCommand]) -> bool {
    !commands.iter().any(|command| matches!(command, AppCommand::TransactionCommit { .. } | AppCommand::TransactionRollback { .. } | AppCommand::Infer(_)))
}

#[derive(Default, Clone, Debug)]
pub(super) struct HubRelayState {
    pub version: u64,
    pub status: Option<store::sync::ArtifactSyncStatus>,
    pub fault: Option<String>,
    pub terminal: Option<String>,
}

/// 🚦️ Why a hub session's edit may not run, decided before any guest executes it — `None` when it may. In order: the
/// bound document's link is terminal (the hub withdrew access or the link expired); the hub holds the principal as a
/// spectator (a `read` delegation) or its authority is not ready; no single hub document of the verb's plugin is bound
/// (nothing the guest authored could reach the hub — a hub session never edits a plugin's own genesis document); the
/// bound document has no live document actor. Every edit that passes is relayed and waits for the hub's acknowledgement.
#[cfg(not(target_arch = "wasm32"))]
pub fn hub_edit_refusal(role: Result<semio_framework_os_kernel::os_directory::DirectorySpaceRole, String>, plugin_id: &str, bound: Option<(&str, &PluginArtifactBinding)>) -> Option<Fault> {
    use semio_framework_os_kernel::os_directory::DirectorySpaceRole;
    if let Some(code) = bound.and_then(|(_, binding)| binding.relay.terminal()) {
        let artifact_id = bound.map_or("", |(artifact_id, _)| artifact_id);
        return Some(Fault { code: code.clone(), message: format!("`{artifact_id}`'s hub link is {code}: the hub takes no further edit from this session") });
    }
    match role {
        Ok(DirectorySpaceRole::Spectator) => {
            return Some(Fault { code: VIEWER_READ_ONLY_FAULT_CODE.to_string(), message: "this agent session is a spectator of its space (its delegation's audience is read): it reads documents and never edits them".to_string() });
        }
        Ok(DirectorySpaceRole::Author) => {}
        Err(detail) => return Some(Fault { code: "plugin.unavailable".to_string(), message: format!("the hub's authority for this session is not ready, so no edit may run: {detail}") }),
    }
    let Some((artifact_id, binding)) = bound else {
        return Some(Fault {
            code: HUB_EDIT_UNBOUND_FAULT_CODE.to_string(),
            message: format!("no single hub document of plugin `{plugin_id}` is open in this session — open exactly one with artifact_open; a hub session never edits a plugin's own genesis document"),
        });
    };
    if binding.backbone.is_none() {
        return Some(Fault {
            code: HUB_EDIT_UNBOUND_FAULT_CODE.to_string(),
            message: format!("`{artifact_id}` has no live link to the hub, so no edit of it could reach the hub: {}", binding.backbone_blocked_by.as_deref().unwrap_or("this document has no open document actor")),
        });
    }
    None
}

/// 📮️ A hub-bound exchange whose guest committed yet published nothing for the hub: the commit is not acknowledged,
/// so the invoke reverts it and never reports it as a write.
#[cfg(not(target_arch = "wasm32"))]
pub fn commits_with_nothing_relayed(frames: Vec<AppFrame>) -> Vec<AppFrame> {
    frames
        .into_iter()
        .map(|frame| match frame {
            AppFrame::TransactionCommitted { txn_id, edit_id, relay: None } => {
                AppFrame::TransactionCommitted { txn_id, edit_id, relay: Some(HubRelayOutcome { acknowledged: false, detail: "the guest committed without publishing an envelope for the hub".to_string(), refused: None }) }
            }
            other => other,
        })
        .collect()
}

/// 🔗️ At most this many hub documents hold a live link — a document socket, one of the hub's bounded WAL writer slots
/// — for one agent session at once ([`hub_links_to_close`]); fixture `🧫️fixtures/🔗️hub-live-links.json`.
pub const HUB_SESSION_LIVE_LINK_LIMIT: usize = 8;

/// 🔗️ Why a session closes one of its live hub links before it links another document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubLinkClose {
    /// ♻️ The document being opened again: its link expired and a fresh one replaces it.
    Relinked,
    /// 🔀️ Another document of the same app: the one being opened becomes the app's session document.
    Superseded,
    /// 📏️ The least recently used link past [`HUB_SESSION_LIVE_LINK_LIMIT`].
    Bounded,
}

/// 🔗️ The live hub links a session closes before `opening`, a document of `route` = (plugin, app), links — `live` =
/// (artifact, plugin, app, used) of every binding with a document actor: the opened document's own (expired) link, every
/// other live link of the same app, then the least recently used until the new link leaves at most `limit` live.
pub fn hub_links_to_close(live: &[(String, String, String, u64)], opening: &str, route: (&str, &str), limit: usize) -> Vec<(String, HubLinkClose)> {
    let mut closes = Vec::new();
    let mut kept = Vec::new();
    for (artifact_id, plugin_id, app_id, used) in live {
        if artifact_id == opening {
            closes.push((artifact_id.clone(), HubLinkClose::Relinked));
        } else if (plugin_id.as_str(), app_id.as_str()) == route {
            closes.push((artifact_id.clone(), HubLinkClose::Superseded));
        } else {
            kept.push((artifact_id, *used));
        }
    }
    kept.sort_by_key(|(_, used)| *used);
    let excess = (kept.len() + 1).saturating_sub(limit);
    closes.extend(kept.into_iter().take(excess).map(|(artifact_id, _)| (artifact_id.clone(), HubLinkClose::Bounded)));
    closes
}

/// 🧭️ A route's session document among the artifacts bound to it — (artifact, hub document?, used): the hub document the
/// agent opened or edited last, else the single folder artifact; two folder artifacts name none.
pub fn route_session_artifact<'a>(bound: impl IntoIterator<Item = (&'a str, bool, u64)>) -> Option<&'a str> {
    let (mut hub, mut folder, mut folders) = (None::<(&'a str, u64)>, None, 0_usize);
    for (artifact_id, hub_document, used) in bound {
        if !hub_document {
            folder = Some(artifact_id);
            folders += 1;
        } else if hub.is_none_or(|(_, latest)| used > latest) {
            hub = Some((artifact_id, used));
        }
    }
    hub.map(|(artifact_id, _)| artifact_id).or(folder.filter(|_| folders == 1))
}

/// 🕰️ The next tick of this process's hub-link clock — monotonic, so "opened or edited last" is a total order.
pub fn hub_link_tick() -> u64 {
    static CLOCK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    CLOCK.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
}

/// 🚫️ Whether a coded message the document actor raised ends its link for good — the hub withdrew access
/// (`access-revoked`) or a shortage outlived its bound (`link-expired`). A terminal link admits no local
/// edit and never relinks ([`store::sync::DocumentLink`]).
pub fn document_link_terminal_code(code: &str) -> bool {
    code == store::sync::DocumentLinkStatus::AccessRevoked.code() || code == store::sync::DocumentLinkStatus::LinkExpired.code()
}

/// ⏱️ How long a commit waits for the hub to acknowledge the envelopes it relayed before it is reverted and answers
/// `hub.relay-unacknowledged` ([`crate::actions::HUB_RELAY_UNACKNOWLEDGED_FAULT_CODE`]) instead of `relay:acknowledged`.
pub const HUB_RELAY_ACK_WAIT_MS: u64 = 10_000;

impl HubRelay {
    pub fn record(&self, status: Option<store::sync::ArtifactSyncStatus>, fault: Option<String>) {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.version += 1;
        if status.is_some() {
            state.status = status;
        }
        if fault.is_some() {
            state.fault = fault;
        }
        self.changed.notify_all();
    }

    /// 🚫️ Records that the actor's link turned terminal (`code`, `message`): no later edit of this session
    /// reaches the hub, and a commit waiting for its acknowledgement learns so at once.
    pub fn record_terminal(&self, code: &str, message: &str) {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.version += 1;
        state.fault = Some(format!("{code}: {message}"));
        state.terminal = Some(code.to_string());
        self.changed.notify_all();
    }

    /// 🚫️ The terminal link code, once the actor reported one.
    pub fn terminal(&self) -> Option<String> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).terminal.clone()
    }

    /// 🔢️ The status version a relay starts from.
    pub fn version(&self) -> u64 {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).version
    }

    /// 📥️ Queues one delivery of the document actor for the guest, in arrival order.
    pub fn deliver(&self, item: HubInbound) {
        self.inbound.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push_back(item);
    }

    /// 📥️ Takes everything the actor delivered since the last take, oldest first.
    pub fn take_inbound(&self) -> Vec<HubInbound> {
        self.inbound.lock().unwrap_or_else(std::sync::PoisonError::into_inner).drain(..).collect()
    }

    /// ↩️ Puts batches a guest could not take back in front of anything delivered meanwhile.
    pub fn restore_inbound(&self, items: Vec<HubInbound>) {
        let mut inbound = self.inbound.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for item in items.into_iter().rev() {
            inbound.push_front(item);
        }
    }

    /// 🟢️ Waits, at most `wait_ms`, until the actor reports a live link — it reports `Live` only once its
    /// bootstrap and catch-up tail are installed, so every batch the hub owed this session at open is
    /// queued by then. Answers whether it got there.
    pub fn await_live(&self, wait_ms: u64) -> bool {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(wait_ms);
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            if state.status.as_ref().is_some_and(|status| matches!(status.remote, store::sync::RemoteState::Live { .. })) {
                return true;
            }
            let now = std::time::Instant::now();
            if now >= deadline {
                return false;
            }
            state = self.changed.wait_timeout(state, deadline - now).unwrap_or_else(std::sync::PoisonError::into_inner).0;
        }
    }

    /// ✅️ Waits, at most `wait_ms`, for a status reported after `since` that says the hub holds every
    /// local mutation (live link, nothing pending, an acknowledged head). Answers the outcome either way,
    /// and at once with the refusal when the link is or turns terminal: the hub will never take the edit.
    pub fn await_acknowledged(&self, since: u64, wait_ms: u64) -> HubRelayOutcome {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(wait_ms);
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            if let Some(code) = state.terminal.clone() {
                return HubRelayOutcome { acknowledged: false, detail: hub_relay_detail(&state), refused: Some(code) };
            }
            if state.version > since && state.status.as_ref().is_some_and(|status| status.pending_mutations == 0 && matches!(status.remote, store::sync::RemoteState::Live { .. }) && status.acknowledged_head.is_some()) {
                return HubRelayOutcome { acknowledged: true, detail: "the hub acknowledged every relayed envelope".to_string(), refused: None };
            }
            let now = std::time::Instant::now();
            if now >= deadline {
                return HubRelayOutcome { acknowledged: false, detail: hub_relay_detail(&state), refused: None };
            }
            state = self.changed.wait_timeout(state, deadline - now).unwrap_or_else(std::sync::PoisonError::into_inner).0;
        }
    }

    /// 🧾️ The sync fields `artifact_open` reports for a bound hub document.
    pub fn report(&self) -> serde_json::Value {
        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let status = state.status.unwrap_or_default();
        serde_json::json!({ "remote": remote_state_word(&status.remote), "pendingMutations": status.pending_mutations, "acknowledged": status.acknowledged_head.is_some(), "lastFault": state.fault })
    }
}

fn remote_state_word(remote: &store::sync::RemoteState) -> String {
    match remote {
        store::sync::RemoteState::Detached => "detached".to_string(),
        store::sync::RemoteState::Connecting => "connecting".to_string(),
        store::sync::RemoteState::Live { .. } => "live".to_string(),
        store::sync::RemoteState::Backoff { retry_in_ms } => format!("backoff (retry in {retry_in_ms} ms)"),
    }
}

fn hub_relay_detail(state: &HubRelayState) -> String {
    let status = state.status.clone().unwrap_or_default();
    let fault = state.fault.as_deref().map(|fault| format!("; last fault: {fault}")).unwrap_or_default();
    format!("the hub has not acknowledged it yet: link {}, {} local mutation(s) pending{fault}", remote_state_word(&status.remote), status.pending_mutations)
}
