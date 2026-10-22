//! 🌉️ Wire bridge, sync session and presence heartbeat: the rollback envelope, the history transition refusal, the presence bytes, the session stamp, the sync session and the presence heartbeat producer. See [`super`].

use super::*;

//#region 🔖️WireBridge
// 🎯️ W6 kernel unification: `to_wire_envelope`/`from_wire_envelope` are DELETED — this actor's
// local envelope shape (`LocalMutations`/`RemoteMutations`/`ChannelBackbone`) and the wire shape
// (`crate::os_spr::wire::ClientFrame::Commands`/`ServerFrame::Commands`) are now the SAME type,
// `crate::os_spr::MutationEnvelope`, throughout: `crate::os_store::backbone::BackboneMessage::Mutations` already carries
// it natively (W6's `store` repoint), so there is no local/wire boundary left to bridge across.
// The old local-only fields (`payload_hash`, `schema_version` separate from `diff.schema`) are
// dropped, not replaced — a repo-wide grep confirmed neither was ever read outside this
// now-deleted bridge (`payload_hash` was write-only; `schema_version`/`deps` were only read back
// by these same two functions and two test assertions, both updated alongside this change).

/// ↩️ Synthesizes a local "undo" envelope from a speculative envelope's own precomputed
/// `inverse`, so a semio_hub `Ack::Applied::{Rejected,Transformed}` outcome can roll back (or replace)
/// the local speculative head without a second round trip. This actor stays JSON-payload-typed end
/// to end (never touches `vcs`/`protocol_command`'s typed `Mutation`/`MutationDiff` trait
/// machinery — see the crate doc), so "the inverse machinery" it uses is simply replaying the
/// envelope's own already-computed inverse payload as a synthetic remote operation, the same path
/// {@link ArtifactActor::deliver_remote_operations} already uses for any other remote edit. 🎯️ W6:
/// re-emits `envelope.inverse` as the rollback's forward diff; the rollback's OWN inverse is the
/// original forward diff (inverse-of-inverse) — `crate::os_spr::causal::InverseMutation` carries no
/// `target_mutation`/`base_version`/`dependencies`/`undo_policy` (a deliberately simpler shape
/// than the old kernel-local one), so those are gone, not defaulted.
pub async fn rollback_envelope(envelope: &MutationEnvelope) -> Option<MutationEnvelope> {
    if semio_framework_os_spr::is_history_transition(envelope) {
        return None;
    }
    let undo_id = MutationId(format!("{}~undo", envelope.mutation_id.0));
    Some(MutationEnvelope {
        mutation_id: undo_id,
        document_id: envelope.document_id.clone(),
        actor: envelope.actor.clone(),
        dependencies: vec![envelope.mutation_id.clone()],
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: envelope.inverse.schema.clone(), payload: envelope.inverse.payload.clone() },
        inverse: semio_framework_os_spr::InverseMutation { schema: envelope.diff.schema.clone(), payload: envelope.diff.payload.clone() },
        timestamp: envelope.timestamp,
        transaction: None, verb: None, line: None,
    })
}

/// 🛟️ The code of the typed refusal an actor raises when the hub refused a batch holding history transitions.
pub const HISTORY_TRANSITION_REFUSED_CODE: &str = "history.transition-refused";

/// 🪧️ The typed refusal of `refused`, a batch the hub did not take as sent (`reason`), when it holds history transitions
/// (a refused `Supersede`, a foreign undo): no inverse rolls a transition back ([`rollback_envelope`]), so the actor
/// retracts them from its store (`BackboneMessage::Retract`, which drops every local transition depending on them too)
/// and from its persisted log, and the author converges with the hub, which never took them. The message targets every
/// refused transition, so the runtime can say which history step the hub did not take (ticket 26/09/30
/// NON-DESTRUCTIVE-HISTORY-EDITING).
pub fn transition_refusal(refused: &[MutationEnvelope], reason: &str) -> Option<MutationMessage> {
    let transitions: Vec<String> = refused.iter().filter(|envelope| semio_framework_os_spr::is_history_transition(envelope)).map(|envelope| envelope.mutation_id.0.clone()).collect();
    (!transitions.is_empty()).then(|| MutationMessage {
        level: semio_framework_diagnostic::Severity::Error,
        code: semio_framework_diagnostic::FaultCode::new(HISTORY_TRANSITION_REFUSED_CODE),
        message: format!("the hub refused {} history step(s) ({reason}); they are withdrawn", transitions.len()),
        target: transitions,
        op_index: None,
    })
}

/// 📡️ `PresencePeer` -> the binary blob `crate::os_spr::wire::ClientFrame::Presence` carries
/// opaquely (`crate::os_spr::encode_presence_peer` — `protocol_wire` has no dependency on
/// this crate's `PresencePeer` type, so the frame only ever moves the pre-encoded bytes).
pub async fn presence_to_bytes(peer: &PresencePeer) -> Vec<u8> {
    semio_framework_os_spr::encode_presence_peer(peer).await
}

/// 📡️ The inverse of {@link presence_to_bytes}, for `ServerFrame::Presence`'s peer roster.
pub async fn presence_from_bytes(bytes: &[u8]) -> Option<PresencePeer> {
    semio_framework_os_spr::decode_presence_peer(bytes).await.ok()
}

// 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
// `assemble_presence_interaction` MOVED to `crate::os_spr::wire`'s `🔖️PresenceInteraction` region —
// guests never enable this crate's `sync` feature, and `VcsArtifactApp` must be able to call it
// without pulling in the whole actor layer. Re-imported at the top of this module (`use
// crate::os_spr::assemble_presence_interaction`-equivalent via the `crate::os_spr::{...}` import
// list) rather than re-exported here — every call site below already goes through
// `crate::os_spr::assemble_presence_interaction` directly.

/// 🎨️ Stamps a peer's hub-assigned session color and canonical surface onto an outbound
/// `PresencePeer` right before {@link presence_to_bytes} — the ONE place either field is ever
/// filled; shells never set `peer.color`/`peer.surface` themselves (contract-freeze §C7.4). Pure so
/// both the native and wasm actors (and their tests) can share it.
pub async fn stamp_session(peer: &mut PresencePeer, session_color: Option<u8>, surface: Option<&str>) {
    peer.color = session_color;
    peer.surface = surface.map(str::to_string);
}

/// ⏰️ Millisecond wall-clock reads for {@link next_timestamp} and the native folder endpoint — the only
/// readers, both native: the browser actor never stamps a clock, it carries the author's.
#[cfg(not(target_arch = "wasm32"))]
pub async fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

/// 🧮️ A stable, deterministic `u64` seed for an actor id string, for
/// `crate::os_spr::HybridLogicalTimestamp::actor` (which is `u64`-shaped; this actor's own id is a
/// free-form `String`).
pub async fn actor_seed(actor: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    actor.hash(&mut hasher);
    hasher.finish()
}

/// ⏰️ Advances `counter` and stamps a fresh {@link crate::os_spr::HybridLogicalTimestamp} for an
/// external folder envelope that carries no clock. An authored envelope's `(hlc, id)` is canonical: it
/// crosses the wire exactly as the author folded it, so every replica orders it — and resolves the last
/// supersession of an operation — by the same key (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING).
#[cfg(not(target_arch = "wasm32"))]
pub async fn next_timestamp(seed: u64, counter: &mut u64) -> semio_framework_os_spr::HybridLogicalTimestamp {
    *counter = counter.wrapping_add(1);
    semio_framework_os_spr::HybridLogicalTimestamp { actor: seed, physical_ms: now_ms().await, logical: *counter }
}
//#endregion 🔖️WireBridge

//#region 🔖️SyncSession
/// 🔁️ Pairs a document's vcs store with the causal DAG that reconciles remote envelopes into
/// it. Extended into the actor world via {@link SyncSession::attach}: it holds the actor command
/// channel and event stream, drains status on {@link SyncSession::tick}, and delegates store IO.
pub struct SyncSession<P, Mutation>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + pack::snapshot::ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub store: ArtifactStore<P, Mutation>,
    pub cmd_tx: Option<ArtifactMailboxSender>,
    pub events: Option<broadcast::Receiver<ArtifactEvent>>,
    pub status: ArtifactSyncStatus,
}

impl<P, Mutation> SyncSession<P, Mutation>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + pack::snapshot::ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    pub async fn new(store: ArtifactStore<P, Mutation>) -> Self {
        Self { store, cmd_tx: None, events: None, status: ArtifactSyncStatus::default() }
    }

    /// 🔌️ Attaches this session's store to a document actor: the actor's `ChannelBackbone` end
    /// is wired into the store, and the command/event channels are retained for wake + status.
    pub async fn attach<Runner>(&mut self, channels: ArtifactChannelsOf<Runner>, events: broadcast::Receiver<ArtifactEvent>) -> Result<(), SyncError> {
        self.store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.map_err(|error| SyncError::Vcs(error.to_string()))?;
        self.cmd_tx = Some(channels.cmd_tx);
        self.events = Some(events);
        Ok(())
    }

    /// ✂️ Detaches from the actor (asking it to flush + stop) and unbinds the store's backbone.
    pub async fn detach(&mut self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::Detach);
        }
        let _ = self.store.detach_backbone();
        self.cmd_tx = None;
        self.events = None;
    }

    /// 🔔️ Nudges the actor to drain the store's outbound queue without waiting for its poll tick.
    pub async fn wake(&self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        }
    }

    /// 👻️ Publishes an ephemeral preview blob on the semio_hub's preview lane. See
    /// {@link ArtifactActorMsg::PublishPreview}.
    pub async fn publish_preview(&self, key: String, seq: u64, payload: Vec<u8>) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(ArtifactActorMsg::PublishPreview { key, seq, payload });
        }
    }

    /// 📥️ Advances one buffered sync event, then gives the store one inbound pump opportunity.
    pub async fn tick(&mut self) -> Result<bool, SyncError> {
        if let Some(events) = &mut self.events {
            if let Ok(ArtifactEvent::Status(status)) = events.try_recv() {
                self.status = status;
            }
        }
        self.store.tick().await.map_err(|error| SyncError::Vcs(error.to_string()))
    }

    /// 🚦️ The latest sync status seen on the event stream (updated by {@link SyncSession::tick}).
    pub async fn status(&self) -> ArtifactSyncStatus {
        self.status.clone()
    }

    /// 🕸️ Feeds a remote envelope through the store's causal DAG, materializing it (and any
    /// now-unblocked dependents) into the edit timeline. Kept for direct/test injection.
    pub async fn receive(&mut self, envelope: MutationEnvelope, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), SyncError> {
        self.store.dispatch(semio_framework_os_store_space_schema::ArtifactCommand::IngestRemote { envelope }, identity).await.map(|_| ()).map_err(|error| SyncError::Vcs(error.to_string()))
    }

    pub async fn reconcile_branch(&mut self, _alternative_name: &str, _message: Option<String>, _authors: Vec<semio_framework_os_vcs::Author>) -> Result<String, SyncError> {
        Err(SyncError::Vcs("branch reconciliation requires the retained envelope candidate job; synchronous whole-envelope duplication is forbidden".into()))
    }
}
//#endregion 🔖️SyncSession

//#region 🔖️Host
/// 💓️ Maximum generic host heartbeat frequency. Presence is lossy, last-writer-wins state:
/// callers may offer cursor/viewport/app-presence updates as often as input arrives, while the host
/// publishes only the newest complete peer snapshot at ten hertz.
pub const PRESENCE_HEARTBEAT_INTERVAL_MS: u64 = 100;

/// 🧮️ The summary a peer may publish: `completed` never above `total`, so the wire decoder never refuses it.
pub fn presence_tool_run_clamped(tool_run: PresenceToolRun) -> PresenceToolRun {
    let completed = tool_run.total.map_or(tool_run.completed, |total| tool_run.completed.min(total));
    PresenceToolRun { completed, ..tool_run }
}

/// 💓️ Per-document last-writer-wins presence producer. The producer owns cadence rather than
/// every renderer/app inventing a timer: offers inside the minimum interval replace `pending`, the
/// first offer publishes immediately, and a later offer publishes the newest complete snapshot.
#[derive(Clone, Debug)]
pub struct PresenceHeartbeatProducer {
    interval_ms: u64,
    last_sent_at_ms: Option<u64>,
    pub pending: Option<PresencePeer>,
    observed_tool_run: Option<Option<PresenceToolRun>>,
}

impl Default for PresenceHeartbeatProducer {
    fn default() -> Self {
        Self::new(PRESENCE_HEARTBEAT_INTERVAL_MS)
    }
}

impl PresenceHeartbeatProducer {
    /// 🚫️async: E1 pure struct-literal builder consumed by `impl Default` (sync-only external
    /// trait) — see R9. No I/O, no suspension point.
    pub fn new(interval_ms: u64) -> Self {
        Self { interval_ms: interval_ms.max(1), last_sent_at_ms: None, pending: None, observed_tool_run: None }
    }

    /// ⏯️ Records the document instance's latest tool run summary (a guest's `AppFrame::Ephemeral.tool_run`,
    /// `📋️tool-run-contract.md` §3.4). Once observed, the guest's summary is authoritative: every later offer
    /// carries it (or its absence) instead of whatever the offering renderer assembled.
    pub fn observe_tool_run(&mut self, tool_run: Option<PresenceToolRun>) {
        let tool_run = tool_run.map(presence_tool_run_clamped);
        if let Some(pending) = self.pending.as_mut() {
            pending.tool_run = tool_run.clone();
        }
        self.observed_tool_run = Some(tool_run);
    }

    /// 📡️ Offers the newest whole peer snapshot and returns it only when this document's
    /// cadence permits a publish. A backward-moving clock conservatively waits for the next interval.
    /// The tool run summary is the observed one when there is one, and `completed ≤ total` either way.
    pub fn offer(&mut self, now_ms: u64, mut peer: PresencePeer) -> Option<PresencePeer> {
        peer.tool_run = match &self.observed_tool_run {
            Some(observed) => observed.clone(),
            None => peer.tool_run.map(presence_tool_run_clamped),
        };
        self.pending = Some(peer);
        let due = self.last_sent_at_ms.is_none_or(|last| now_ms.saturating_sub(last) >= self.interval_ms);
        if !due {
            return None;
        }
        self.last_sent_at_ms = Some(now_ms);
        self.pending.take()
    }

    pub fn pending(&self) -> Option<&PresencePeer> {
        self.pending.as_ref()
    }
}
//#endregion 🔖️Host
