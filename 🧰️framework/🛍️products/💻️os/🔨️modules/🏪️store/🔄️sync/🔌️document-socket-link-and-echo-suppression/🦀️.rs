//! 🔌️ Document socket link and echo suppression: the document socket connect, the commands frames, the link shortage policy and states, the echo suppression and the document socket door. See [`super`].

use super::*;

//#region 🔖️DocumentSocketConnect
/// 🧬️ The pack schema identity a document socket's hello and bootstrap are checked against: the
/// kind's codec when this process resolves one ([`crate::os_store::codec_registry::document_kind_codec`] — the linked
/// Rust codec, else the codec of the mounted component that owns the kind, asked of that component
/// exactly as the hub's trusted catalog asks it), otherwise the verified execution-target lease's — the
/// hub-selected package a client that mounted no owning component will run. Neither is no identity at
/// all, and the caller refuses to dial rather than guess.
pub async fn document_pack_schema_hash(schema: &str, lease: Option<&semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>) -> Option<[u8; 32]> {
    if let Ok(Some(codec)) = semio_framework_os_store_codec_registry::document_kind_codec(schema).await {
        return codec.pack_schema_hash().await.ok();
    }
    let lease = lease.filter(|lease| lease.artifact.schema == schema)?;
    semio_framework_os_directory_client::decode_lower_hex_32(&lease.artifact.pack_schema_hash)
}

/// 🎫️ What a document actor asks of the hub's open plan for its binding.
pub fn document_socket_expectation(schema: &str, pack_schema_hash: [u8; 32], surface: Option<&str>, lease: Option<&semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>) -> semio_framework_os_directory_client::DocumentSocketExpectationV1 {
    semio_framework_os_directory_client::DocumentSocketExpectationV1 { artifact_schema: schema.to_string(), pack_schema_hash, requested_surface_id: surface.map(str::to_string), lease: lease.cloned() }
}

/// 🧷️ The one binding a document actor holds a hub socket for.
#[derive(Clone, Copy, Debug)]
pub struct DocumentSocketBinding<'a> {
    pub hub_base_url: &'a str,
    pub space_id: &'a str,
    pub document_id: &'a str,
    pub schema: &'a str,
    pub pack_schema_hash: [u8; 32],
    pub surface: Option<&'a str>,
    pub lease: Option<&'a semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
}

/// ⚖️ Whether one admitted authority may carry this binding at `now_ms`: unexpired, from the bound
/// hub origin, for exactly this scope, schema, pack identity and surface, and — when the client verified
/// an execution-target lease — for exactly that lease.
pub fn document_socket_authority_admits(authority: &semio_framework_os_directory_client::DocumentSocketAuthorityV1, binding: &DocumentSocketBinding<'_>, now_ms: u64) -> bool {
    authority.expires_at_unix_ms > now_ms
        && authority.hub_origin.trim_end_matches('/') == binding.hub_base_url.trim_end_matches('/')
        && authority.scope.space_id == binding.space_id
        && authority.scope.document_id == binding.document_id
        && authority.artifact.schema == binding.schema
        && authority.pack_schema_hash == binding.pack_schema_hash
        && binding.surface.is_none_or(|surface| surface == authority.surface.surface_id)
        && binding.lease.is_none_or(|lease| authority.matches_lease_fields(lease))
}

/// 🪢️ A canonical pair's public baseline in the runtime frontier grammar a `SocketHelloV1` advertises.
pub fn canonical_pair_baseline(frontier: &semio_framework_os_directory::ArtifactFrontier) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary { document_id: ArtifactId(frontier.document_id.clone()), head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id.clone(), last_commit_seq: frontier.last_commit_seq, chain_hash: frontier.chain_hash.0 }
}

/// 🛟️ A wire `RebootstrapRequired` control as the directory contract names it (its frontier already projected onto the
/// public document id by the hub).
pub fn rebootstrap_control(control: &semio_framework_os_spr::RebootstrapRequired) -> semio_framework_os_directory::RebootstrapRequired {
    let frontier = &control.baseline_frontier;
    semio_framework_os_directory::RebootstrapRequired {
        scope: semio_framework_os_directory::DocumentScope::new(control.space_id.as_str(), control.document_id.as_str()),
        checkpoint_id: semio_framework_os_directory::ArtifactHash(control.checkpoint_id),
        descriptor_digest_v1: semio_framework_os_directory::ArtifactHash(control.descriptor_hash),
        baseline_frontier: semio_framework_os_directory::ArtifactFrontier { document_id: frontier.document_id.0.clone(), head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id.clone(), last_commit_seq: frontier.last_commit_seq, chain_hash: semio_framework_os_directory::ArtifactHash(frontier.chain_hash) },
    }
}

/// 👋️ The client-first frame every document socket opens with.
pub fn document_socket_hello(schema: &str, pack_schema_hash: [u8; 32], resume_token: Option<String>, frontier: Option<RuntimeFrontierSummary>) -> ClientFrame {
    ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: schema.to_string(), pack_schema_hash, resume_token, frontier }
}

/// 🔐️ The one ordered subprotocol offer a document socket dials with — the grant receipt's
/// protocol, then the session capability — which a browser joins into the single
/// `Sec-WebSocket-Protocol` value the hub splits on `", "`.
pub fn document_socket_protocols(receipt_protocol: &str, capability: &str) -> [String; 2] {
    [receipt_protocol.to_string(), capability.to_string()]
}

/// 📦️ One `Commands` frame ready for a socket: its batch id, the local envelopes it relays (the
/// rollback owner) and its encoded bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandsFrame {
    pub batch_id: u64,
    pub local: Vec<MutationEnvelope>,
    pub bytes: Vec<u8>,
}

/// 📏️ The frames one relay sends, every one within the socket's ceiling, plus the envelopes that
/// cannot fit a frame even alone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandsFramePlan {
    pub frames: Vec<CommandsFrame>,
    pub oversized: Vec<MutationEnvelope>,
}

/// ✂️ Splits one relay into `Commands` frames no larger than `max_frame_bytes`, halving a batch
/// until it fits and keeping envelope order; batch ids run consecutively from `first_batch_id`. `local`
/// and `wire` are the same envelopes before and after socket stamping.
pub async fn commands_frames_within(max_frame_bytes: usize, first_batch_id: u64, local: Vec<MutationEnvelope>, wire: Vec<MutationEnvelope>) -> CommandsFramePlan {
    let mut plan = CommandsFramePlan::default();
    let mut batch_id = first_batch_id;
    let mut pending = std::collections::VecDeque::from([(local, wire)]);
    while let Some((local, wire)) = pending.pop_front() {
        let bytes = encode_client_frame(&ClientFrame::Commands { batch_id, envelopes: wire.clone() }, Lane::Command).await;
        if bytes.len() <= max_frame_bytes {
            plan.frames.push(CommandsFrame { batch_id, local, bytes });
            batch_id = batch_id.wrapping_add(1);
        } else if local.len() > 1 {
            let middle = local.len() / 2;
            let (local_head, local_tail) = local.split_at(middle);
            let (wire_head, wire_tail) = wire.split_at(middle);
            pending.push_front((local_tail.to_vec(), wire_tail.to_vec()));
            pending.push_front((local_head.to_vec(), wire_head.to_vec()));
        } else {
            plan.oversized.extend(local);
        }
    }
    plan
}

/// 🎲️ One document actor's hybrid-logical-clock replica seed: platform entropy, so two replicas
/// of one user (two tabs, two processes) never tie on `(seed, counter)` — H4's per-replica law.
pub fn replica_hlc_seed() -> Result<u64, semio_framework_os_identity::EntropyError> {
    semio_framework_os_identity::entropy_u64()
}
//#endregion 🔖️DocumentSocketConnect

//#region 🔖️DocumentLinkShortage
/// 🔌️ The capped doubling reconnect and the longest shortage one hub document's link rides out
/// (`🧬️schema/document-link-shortage/🔣️.json` `$defs/Policy`). The bound is twice the backoff cap, so at
/// least two capped reconnect attempts fall inside it, and no retry is ever scheduled past it: the last
/// attempt of every shortage runs AT the bound. An attempt that never answers ends the link at the
/// ceiling, the bound plus one capped backoff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentLinkShortagePolicy {
    pub reconnect_min_ms: u64,
    pub reconnect_max_ms: u64,
    pub shortage_bound_ms: u64,
}

impl DocumentLinkShortagePolicy {
    /// 🔁️ When the next attempt of a shortage that began at `since_ms` is due: one backoff after `now_ms`, never past the bound.
    pub fn retry_at(&self, since_ms: u64, now_ms: u64, backoff_ms: u64) -> u64 {
        now_ms.saturating_add(backoff_ms).min(since_ms.saturating_add(self.shortage_bound_ms))
    }

    /// ⏳️ The instant a shortage that began at `since_ms` ends although no attempt answered: the bound plus one capped backoff.
    pub fn ceiling_at(&self, since_ms: u64) -> u64 {
        since_ms.saturating_add(self.shortage_bound_ms).saturating_add(self.reconnect_max_ms)
    }
}

/// 🔌️ The policy every shell drives (`🧫️fixtures/document-link-shortage-v1` `policy`, the React twin
/// `DOCUMENT_LINK_SHORTAGE_POLICY` in `🏪️store/🟦️.ts`).
pub const DOCUMENT_LINK_SHORTAGE_POLICY: DocumentLinkShortagePolicy = DocumentLinkShortagePolicy {
    reconnect_min_ms: semio_framework_os_directory_client::HUB_RECONNECT_MIN_MS,
    reconnect_max_ms: semio_framework_os_directory_client::HUB_RECONNECT_MAX_MS,
    shortage_bound_ms: 2 * semio_framework_os_directory_client::HUB_RECONNECT_MAX_MS,
};

/// 🚫️ The HTTP statuses of a document admission that mean the hub withdrew access (`accessRefusedStatuses`):
/// the link is revoked, never retried.
pub const DOCUMENT_LINK_ACCESS_REFUSED_STATUSES: [u16; 4] = [401, 403, 404, 410];

/// 🚫️ Whether a document admission failed because the hub withdrew access rather than because the link is short.
pub fn document_admission_refuses_access(error: &semio_framework_os_directory_client::DirectoryClientError) -> bool {
    match error {
        semio_framework_os_directory_client::DirectoryClientError::Unauthorized => true,
        semio_framework_os_directory_client::DirectoryClientError::Http { status, .. } => DOCUMENT_LINK_ACCESS_REFUSED_STATUSES.contains(status),
        _ => false,
    }
}

/// 📣️ The message a document actor emits once its link turns terminal: the status code is the fault code a
/// shell localizes ([`DocumentLinkStatus::text`]), the message the English line for logs.
pub fn document_link_terminal_message(document_id: &str, status: DocumentLinkStatus) -> MutationMessage {
    MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new(status.code()), message: status.text(false).unwrap_or_default().to_string(), target: vec![document_id.to_string()], op_index: None }
}

/// 🔌️ A hub document's link, the ONE state machine the native actor, the browser actor and the
/// React worker drive (ticket 26/09/23 audit P2-2): while `Unlinked` the document stays mounted, local
/// edits keep applying and queue, and the link retries at `retry_at_ms`; a shortage that outlasts the
/// policy bound becomes `Expired` (AGENTS.md: short shortages, never long offline periods) and a hub that
/// withdraws access makes it `Revoked`. Both terminal states admit no local edit and never relink — the
/// human reopens. Schema: `🧬️schema/document-link-shortage/🔣️.json`; law: `🧫️fixtures/document-link-shortage-v1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLink {
    Linked,
    Unlinked { since_ms: u64, backoff_ms: u64, retry_at_ms: u64 },
    Expired { since_ms: u64, at_ms: u64 },
    Revoked { at_ms: u64 },
}

/// 🔌️ What happened to a link: an attempt failed or the socket was lost, the link is live again, time
/// passed, or the hub refused access (401/403/404/410).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLinkEvent {
    Failed { now_ms: u64 },
    Restored { now_ms: u64 },
    Tick { now_ms: u64 },
    Refused { now_ms: u64 },
}

/// 🚦️ The status a shell shows for a link (`$defs/Status`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLinkStatus {
    Linked,
    Reconnecting,
    LinkExpired,
    AccessRevoked,
}

impl DocumentLinkStatus {
    /// 🏷️ The schema's status code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::Reconnecting => "reconnecting",
            Self::LinkExpired => "link-expired",
            Self::AccessRevoked => "access-revoked",
        }
    }

    /// 🌐️ The localized line a shell shows for a shortage status (the fixture's `texts`); `None` while linked.
    pub fn text(self, is_de: bool) -> Option<&'static str> {
        match (self, is_de) {
            (Self::Linked, _) => None,
            (Self::Reconnecting, false) => Some("Connection lost. Your edits are kept and sent when the hub is reachable again."),
            (Self::Reconnecting, true) => Some("Verbindung unterbrochen. Ihre Änderungen bleiben erhalten und werden gesendet, sobald der Hub wieder erreichbar ist."),
            (Self::LinkExpired, false) => Some("The connection was lost for too long. Reconnect to keep editing this document."),
            (Self::LinkExpired, true) => Some("Die Verbindung war zu lange unterbrochen. Verbinden Sie sich erneut, um dieses Dokument weiter zu bearbeiten."),
            (Self::AccessRevoked, false) => Some("Your access to this document was removed."),
            (Self::AccessRevoked, true) => Some("Ihr Zugriff auf dieses Dokument wurde entfernt."),
        }
    }
}

impl DocumentLink {
    /// 🌱️ A link opened at `now_ms`: not yet live, first attempt due at once, the shortage bound running.
    pub fn opened(now_ms: u64) -> Self {
        Self::Unlinked { since_ms: now_ms, backoff_ms: 0, retry_at_ms: now_ms }
    }

    /// ⚙️ The one transition function (`$defs/Event` → `$defs/State`).
    #[must_use]
    pub fn apply(self, policy: &DocumentLinkShortagePolicy, event: DocumentLinkEvent) -> Self {
        match (self, event) {
            (Self::Expired { .. } | Self::Revoked { .. }, _) => self,
            (_, DocumentLinkEvent::Refused { now_ms }) => Self::Revoked { at_ms: now_ms },
            (_, DocumentLinkEvent::Restored { .. }) => Self::Linked,
            (Self::Linked, DocumentLinkEvent::Failed { now_ms }) => Self::Unlinked { since_ms: now_ms, backoff_ms: policy.reconnect_min_ms, retry_at_ms: policy.retry_at(now_ms, now_ms, policy.reconnect_min_ms) },
            (Self::Unlinked { since_ms, backoff_ms, .. }, DocumentLinkEvent::Failed { now_ms }) => {
                if now_ms.saturating_sub(since_ms) >= policy.shortage_bound_ms {
                    return Self::Expired { since_ms, at_ms: now_ms };
                }
                let backoff_ms = backoff_ms.saturating_mul(2).clamp(policy.reconnect_min_ms, policy.reconnect_max_ms);
                Self::Unlinked { since_ms, backoff_ms, retry_at_ms: policy.retry_at(since_ms, now_ms, backoff_ms) }
            }
            (Self::Unlinked { since_ms, .. }, DocumentLinkEvent::Tick { now_ms }) if now_ms >= policy.ceiling_at(since_ms) => Self::Expired { since_ms, at_ms: now_ms },
            (_, DocumentLinkEvent::Tick { .. }) => self,
        }
    }

    /// 🚦️ The status a shell shows.
    pub fn status(self) -> DocumentLinkStatus {
        match self {
            Self::Linked => DocumentLinkStatus::Linked,
            Self::Unlinked { .. } => DocumentLinkStatus::Reconnecting,
            Self::Expired { .. } => DocumentLinkStatus::LinkExpired,
            Self::Revoked { .. } => DocumentLinkStatus::AccessRevoked,
        }
    }

    /// ✍️ Whether a local edit applies (and queues while unlinked); a terminal link admits none.
    pub fn admits_local_edits(self) -> bool {
        matches!(self, Self::Linked | Self::Unlinked { .. })
    }

    /// ⏳️ The latest instant an unlinked link can live: its ceiling. A failed attempt at the bound ends it earlier.
    pub fn expires_at_ms(self, policy: &DocumentLinkShortagePolicy) -> Option<u64> {
        match self {
            Self::Unlinked { since_ms, .. } => Some(policy.ceiling_at(since_ms)),
            _ => None,
        }
    }

    /// 🔁️ When the next reconnect attempt is due; `None` while linked or terminal.
    pub fn retry_at_ms(self) -> Option<u64> {
        match self {
            Self::Unlinked { retry_at_ms, .. } => Some(retry_at_ms),
            _ => None,
        }
    }

    /// 🔁️ Whether a reconnect attempt may start now.
    pub fn retry_due(self, now_ms: u64) -> bool {
        self.retry_at_ms().is_some_and(|at| at <= now_ms)
    }

    /// ⏰️ The earliest instant this link needs a turn of its own: its retry or its expiry.
    pub fn next_deadline_ms(self, policy: &DocumentLinkShortagePolicy) -> Option<u64> {
        match (self.retry_at_ms(), self.expires_at_ms(policy)) {
            (Some(retry), Some(expiry)) => Some(retry.min(expiry)),
            (retry, expiry) => retry.or(expiry),
        }
    }
}
//#endregion 🔖️DocumentLinkShortage

//#region 🔁️DocumentEchoSuppression
/// 🔁️ The envelopes of one server `Commands` frame a replica applies, in order: echo suppression is by operation identity,
/// never by frame origin. An envelope is applied unless its id is one this replica authored or already applied, and every admitted id
/// is recorded in `applied`. A frame is never discarded whole — the hub's hello catch-up tail carries anyone's edits, the joiner's own
/// earlier device included (ticket 26/09/23 session 12, run s12i: late joiners saw no history). Schema
/// `🧬️schema/document-echo-suppression`; law `🧫️fixtures/document-echo-suppression-v1`; TS twin `admitRemoteEnvelopes`.
pub fn admit_remote_envelopes(applied: &mut std::collections::HashSet<String>, envelopes: impl IntoIterator<Item = MutationEnvelope>) -> Vec<MutationEnvelope> {
    envelopes.into_iter().filter(|envelope| applied.insert(envelope.mutation_id.0.clone())).collect()
}

/// 🔁️ Records the operations this replica authored, so their echo is never applied a second time.
pub fn note_authored_envelopes(applied: &mut std::collections::HashSet<String>, envelopes: &[MutationEnvelope]) {
    applied.extend(envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()));
}

/// ✅️ Settles this replica's own operations the hub's log already holds: a `Commands` frame (a catch-up tail or a relay)
/// that carries an operation still in `outbox` or a `pending` batch proves the hub committed it although its `Ack` was lost with the
/// socket, so it leaves both (a batch left empty leaves too) and is never resent — a resend is re-stamped and the hub refuses it as a
/// replayed operation (ticket 26/09/23 session 13, run s13b). Answers the settled envelopes, whose backbone retention the caller
/// releases.
pub fn settle_committed_envelopes(outbox: &mut Vec<MutationEnvelope>, pending: &mut std::collections::HashMap<u64, Vec<MutationEnvelope>>, committed: &[MutationEnvelope]) -> Vec<MutationEnvelope> {
    let committed: std::collections::HashSet<&str> = committed.iter().map(|envelope| envelope.mutation_id.0.as_str()).collect();
    let mut settled = Vec::new();
    let mut keep = |envelope: &MutationEnvelope| !committed.contains(envelope.mutation_id.0.as_str()) || {
        settled.push(envelope.clone());
        false
    };
    outbox.retain(&mut keep);
    pending.retain(|_, batch| {
        batch.retain(&mut keep);
        !batch.is_empty()
    });
    settled
}
//#endregion 🔁️DocumentEchoSuppression

//#region 🔖️DocumentSocketDoor
/// 📬️ One observation of a browser document socket's receive side.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentSocketPoll {
    Frame(Vec<u8>),
    Pending,
    Closed(Option<u16>),
    Lost(u32),
}

/// 🔌️ One page-owned document socket as the browser actor drives it: never blocking, polled on
/// the actor's own cadence, and honest about loss — a dropped frame is [`DocumentSocketPoll::Lost`],
/// which the actor answers with a reconnect and a hub catch-up rather than a silent gap.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub trait DocumentSocket {
    fn is_open(&self) -> bool;
    /// 📏️ The largest frame this socket carries; the actor splits a `Commands` batch to fit it.
    fn max_frame_bytes(&self) -> usize;
    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), String>;
    fn poll(&mut self) -> DocumentSocketPoll;
    fn close(&mut self);
}

/// ☎️ Opens document sockets for the browser actor. The host injects it (the wgpu renderer's
/// duplex socket door), so the kernel names no page API and every browser socket has one owner.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub trait DocumentSocketDialer {
    fn dial(&self, url: &str, protocols: &[String]) -> Result<Box<dyn DocumentSocket>, String>;
}
//#endregion 🔖️DocumentSocketDoor
