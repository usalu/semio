//! 🎞️ Protocol app-engine channel: the `AppCommand`/`AppFrame` binary frame taxonomy every app,
//! once turned into a headless engine driven by bidirectional streaming of typed binary commands,
//! exchanges with its client (a UI or a headless runner) — every UI interaction becomes a
//! forwarded `AppCommand`, every engine reaction a returned `AppFrame`. Ticket:
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️01/HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS/`.
//!
//! 🎯️ Mirrors `protocol_wire`'s W5 hand-rolled binary layout exactly: `tag: u8` (assigned
//! sequentially in match-arm declaration order below, NOT the enum's own discriminant) followed by
//! its fields in declaration order, no per-field tags, no body-length prefix — one frame per
//! channel message. `crate::os_spr::wire::🔖️WireCodec` supplies the primitive codec
//! (`write_varint_u64`/`write_str`/`write_bytes`/`write_bool` and their `read_*` twins); this crate
//! adds only the option/vec combinators and the two enums' tag dispatch below. Unlike
//! `crate::os_spr::wire::ClientFrame`/`ServerFrame`, `AppCommand`/`AppFrame` carry no `Lane` byte —
//! the app-engine channel is a single logical stream, not split into causally-ordered vs.
//! best-effort lanes.

//#region 🔖️Version
/// @emoji 🔢️ The channel wire format's own version, pinned against the shared cross-language
/// fixture `channel-version.json` so a half-done bump fails a test instead of drifting silently.
/// Channel v12 (`📓️design-abi.md` §2 "`exchange` collapse") retires the `AppCommand::Hello` /
/// `AppFrame::Welcome` handshake entirely — lifecycle now arrives through the reactor ABI's
/// `Event::InstanceOpen`/`InstanceClose`, so this constant is no longer carried on the wire by any
/// frame; it exists purely as the drift guard the tests below assert against.
pub const CHANNEL_VERSION: u32 = 12;
//#endregion 🔖️Version

//#region 🔖️ChildPackEntry
/// @emoji 🧸️ One owned child's whole persisted envelope, as it travels between host and guest.
/// Composed children are their OWN envelopes with their own `ArtifactVcs` history, so a composing
/// document's `LoadDocument`/`Document` pair is not sufficient to save or restore it — its children
/// would exist only until the process ended. `AppCommand::LoadChildren`/`AppFrame::Children` carry
/// exactly these, keyed the way the parent's `ArtifactChild` handles name them.
#[derive(Clone, Debug, PartialEq)]
pub struct ChildPackEntry {
    pub slot: String,
    pub child_id: String,
    /// 🎯️ `ArtifactDialect` as its `<kind>@<standard>/<subset>` wire string — the guest needs it to
    /// pick the right `ChildStoreFactory`, and it is not recoverable from the pack bytes alone.
    pub dialect: String,
    /// 📦️ The child's full envelope pack (`encode_document_pack_bytes` framing: pack + spr).
    pub envelope_pack: Vec<u8>,
}
//#endregion 🔖️ChildPackEntry

//#region 🔖️AppCommand
/// @emoji 📨️ One frame a client (UI or headless runner) sends to the app engine.
#[derive(Clone, Debug, PartialEq)]
pub enum AppCommand {
    ConfigCommand {
        seq: u64,
        command: Vec<u8>,
    },
    Command {
        seq: u64,
        command: Vec<u8>,
        /// 🗣️ Packed `ViewModel` (see `crate::os_store::pack_rt`) the client wants this command evaluated against.
        view_state: Vec<u8>,
    },
    CommandText {
        seq: u64,
        line: String,
    },
    ContextMenu {
        seq: u64,
        request: Vec<u8>,
    },
    ArtifactCommand {
        seq: u64,
        command: Vec<u8>,
    },
    ApplyEnvelopes {
        seq: u64,
        envelopes: Vec<crate::os_spr::causal::MutationEnvelope>,
    },
    LoadDocument {
        seq: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
    },
    ReadDocument {
        seq: u64,
    },
    LoadConfig {
        seq: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
    },
    ReadConfig {
        seq: u64,
    },
    MediaIn {
        seq: u64,
        port: String,
        descriptor: Vec<u8>,
        data: Vec<u8>,
    },
    MediaOut {
        seq: u64,
        port: String,
        request: Vec<u8>,
    },
    MediaFingerprint {
        seq: u64,
        port: String,
    },
    /// 🧾 Host-authoritative command: document/config/draft packs travel with the command; guest
    /// returns `AppFrame::Emit` ops only (host applies). CHANNEL_VERSION 5 wire addition.
    PureCommand {
        seq: u64,
        command: Vec<u8>,
        document: Vec<u8>,
        document_spr: Vec<u8>,
        config: Vec<u8>,
        config_spr: Vec<u8>,
        draft: Vec<u8>,
        draft_spr: Vec<u8>,
    },
    /// 🧸️ Restores a composing document's owned children into the engine, each as its own live
    /// store. Sent after `LoadDocument` (the parent must exist before its children can be adopted).
    /// CHANNEL_VERSION 6 wire addition.
    LoadChildren {
        seq: u64,
        entries: Vec<ChildPackEntry>,
    },
    /// 🧸️ Asks the engine for every owned child's current envelope, for persistence — the child-side
    /// counterpart of `ReadDocument`. CHANNEL_VERSION 6 wire addition.
    ReadChildren {
        seq: u64,
    },
    /// 🧾️ Reads a complete history projection after initial connection or cursor resynchronization.
    ReadHistory {
        seq: u64,
    },
    /// 🤝️ Phase-1 prepare for one transaction member — flat fields carry EITHER the owner-mutation
    /// form (`mutation_id`+`payload` set, `prepared_ops` empty) OR the pre-planned form
    /// (`prepared_ops`+`label`+`origin` set, `mutation_id` empty); see contract-freeze.md §2 of
    /// `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/`.
    /// CHANNEL_VERSION 9 wire addition.
    TransactionPrepare {
        seq: u64,
        txn_id: String,
        mutation_id: String,
        payload: Vec<u8>,
        prepared_ops: Vec<Vec<u8>>,
        label: String,
        origin: Vec<u8>,
    },
    /// ✅️ Phase-2 commit for one transaction member. CHANNEL_VERSION 9 wire addition.
    TransactionCommit {
        seq: u64,
        txn_id: String,
    },
    /// ↩️ Aborts a not-yet-committed transaction member. CHANNEL_VERSION 9 wire addition.
    TransactionRollback {
        seq: u64,
        txn_id: String,
    },
    /// ⏪️ Fans a group undo out to one already-committed transaction member. CHANNEL_VERSION 9 wire addition.
    TransactionUndo {
        seq: u64,
        group_id: String,
    },
    /// ⏩️ Fans a group redo out to one already-committed transaction member. CHANNEL_VERSION 9 wire addition.
    TransactionRedo {
        seq: u64,
        group_id: String,
    },
    /// 📂️ Opens an artifact in its resolved (or explicitly named) viewer/editor surface — see
    /// contract-freeze.md §3 of `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`.
    /// Empty `plugin_id`/`app_id` means "resolve via `OpeningResolver`". CHANNEL_VERSION 10 wire addition.
    OpenArtifact {
        seq: u64,
        artifact_ref: String,
        role: u8,
        plugin_id: String,
        app_id: String,
    },
    /// 🎚️ Pins a viewer/editor default for one `(artifact_kind, standard, subset, role)` coordinate,
    /// persisted event-sourced in the OS `🎚️config` opening-preferences facet. CHANNEL_VERSION 10 wire addition.
    SetDefaultApp {
        seq: u64,
        artifact_kind: String,
        standard: String,
        subset: String,
        role: u8,
        plugin_id: String,
        app_id: String,
    },
    /// 🎚️ Clears a previously pinned default, falling back to the `OpeningResolver`'s owner/router
    /// order. CHANNEL_VERSION 10 wire addition.
    ClearDefaultApp {
        seq: u64,
        artifact_kind: String,
        standard: String,
        subset: String,
        role: u8,
    },
    /// ⚖️ Pins this connection's local/authority `MergePolicy` (`0`=`LaissezFaire`, `1`=`Normal`,
    /// `2`=`Vigilant`) — never carried on a `MutationEnvelope`/`BackboneMessage`, see
    /// contract-freeze.md §C3/C8 of `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
    /// CHANNEL_VERSION 11 wire addition.
    SetMergePolicy {
        seq: u64,
        policy: u8,
    },
    /// ⚔️ Resolves one open `Conflict` (`0`=`Accept`, `1`=`Discard`) — see contract-freeze.md §C5/C6.
    /// CHANNEL_VERSION 11 wire addition.
    ResolveConflict {
        seq: u64,
        conflict_id: String,
        resolution: u8,
    },
    /// ⚔️ Reads every open `Conflict` for the current artifact. CHANNEL_VERSION 11 wire addition.
    ReadConflicts {
        seq: u64,
    },
    /// 👥️ Pushes the document-wide presence roster into this app instance — the ONLY plugin ingress
    /// for peers (contract-freeze §C7.6 of ticket
    /// `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`).
    /// `own_color` is this actor's hub-assigned palette index (`None` for a folder-only session with
    /// no hub); `peers` are `encode_presence_peer` blobs, the whole roster with the wrapper's own
    /// actor already dropped. Reply is a plain `AppFrame::Done`. CHANNEL_VERSION 12 wire addition.
    Presence {
        seq: u64,
        own_color: Option<u8>,
        peers: Vec<Vec<u8>>,
    },
}
//#endregion 🔖️AppCommand

//#region 🔖️AppFrame
/// @emoji 📬️ One frame the app engine sends to its client.
#[derive(Clone, Debug, PartialEq)]
pub enum AppFrame {
    Done { in_reply_to: u64 },
    /// 🧾 `messages` (CHANNEL_VERSION 11 trailing addition) is one packed `DispatchReport` for this
    /// dispatch — see contract-freeze.md §C8.
    Invocation { in_reply_to: u64, output: Vec<u8>, diagnostics: Vec<u8>, ui_scope: Vec<u8>, history_patch: Vec<u8>, messages: Vec<u8> },
    DocumentChanged { envelopes: Vec<crate::os_spr::causal::MutationEnvelope>, origin: String },
    Document { in_reply_to: u64, pack: Vec<u8>, spr: Vec<u8>, ops: String },
    Config { in_reply_to: u64, pack: Vec<u8>, spr: Vec<u8>, ops: String },
    ConfigChanged { envelopes: Vec<crate::os_spr::causal::MutationEnvelope>, origin: String },
    ContextMenu { in_reply_to: u64, items: Vec<u8> },
    Media { in_reply_to: u64, port: String, descriptor: Vec<u8>, data: Vec<u8> },
    MediaFingerprint { in_reply_to: u64, port: String, fingerprint: Vec<u8> },
    /// 🧾 `report` (CHANNEL_VERSION 11 trailing addition) is one packed `DispatchReport` of the
    /// rejected dispatch, accompanying a `Fault.code == "mutation.rejected"` — see
    /// contract-freeze.md §C8/C9.
    Error { in_reply_to: Option<u64>, fault: Vec<u8>, report: Vec<u8> },
    /// 📤️ Guest Emit bytes for host-applied store authority (document/config/draft op packs).
    Emit {
        in_reply_to: u64,
        document_ops: Vec<u8>,
        config_ops: Vec<u8>,
        draft_ops: Vec<u8>,
        output: Vec<u8>,
        diagnostics: Vec<u8>,
    },
    /// 📝️ Draft-lane pack snapshot (volatile; never enters a Change/Checkpoint).
    Draft {
        in_reply_to: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
        ops: String,
    },
    /// 🧸️ Every owned child's current envelope — the reply to `ReadChildren`, and also emitted
    /// unsolicited (`in_reply_to` of the originating command) after a composite gesture creates new
    /// children, so the host learns about a genesis child without having to poll for it.
    /// CHANNEL_VERSION 6 wire addition.
    Children {
        in_reply_to: u64,
        entries: Vec<ChildPackEntry>,
    },
    /// 👥️ Typed guest ephemeral-lane snapshot. Presence is the app-defined `ArtifactPack` payload;
    /// generations let hosts skip unchanged renderer work while transient remains local-only.
    /// `interaction` (trailing field, CHANNEL_VERSION 12 wire addition, contract-freeze §C7.6) is the
    /// output of `encode_presence_interaction` over the app's own declared broadcast domains — empty
    /// bytes when the app declares no interaction domains or nothing is selected/hovered right now.
    Ephemeral {
        presence: Vec<u8>,
        presence_generation: u64,
        transient_generation: u64,
        interaction: Vec<u8>,
    },
    /// 🧾️ Full history patch for initial host projection and gap recovery.
    HistorySnapshot { in_reply_to: u64, history_patch: Vec<u8> },
    /// 📣️ A guest's dispatch touched a foreign artifact — the host mints `txn_id`, resolves each
    /// opaque `ForeignStep` in `foreign` (one `store::pack_rt::encode_wire_value`-encoded serde
    /// form per element; not decoded at this layer), and drives the transaction protocol (contract
    /// freeze §5). CHANNEL_VERSION 9 wire addition.
    TransactionProposal {
        in_reply_to: u64,
        proposal_id: String,
        local_ops: Vec<Vec<u8>>,
        description: String,
        coalesce_key: String,
        foreign: Vec<Vec<u8>>,
    },
    /// 🤝️ Phase-1 reply — empty `rejection` means the member is prepared. CHANNEL_VERSION 9 wire addition.
    TransactionPrepared {
        txn_id: String,
        foreign: Vec<Vec<u8>>,
        rejection: Vec<u8>,
    },
    /// ✅️ Phase-2 commit succeeded for a member. CHANNEL_VERSION 9 wire addition.
    TransactionCommitted {
        txn_id: String,
        edit_id: String,
    },
    /// ↩️ A member rolled back its not-yet-committed transaction. CHANNEL_VERSION 9 wire addition.
    TransactionRolledBack {
        txn_id: String,
    },
    /// ⚔️ Pushed unsolicited (next to `DocumentChanged`) after every ingest: one packed `MergeReport`
    /// describing how the batch was resolved. CHANNEL_VERSION 11 wire addition.
    MergeReport {
        in_reply_to: Option<u64>,
        report: Vec<u8>,
    },
    /// ⚔️ Pushed unsolicited (next to `DocumentChanged`) after every ingest, and the reply to
    /// `AppCommand::ReadConflicts`: one packed `Vec<Conflict>`. CHANNEL_VERSION 11 wire addition.
    Conflicts {
        in_reply_to: Option<u64>,
        conflicts: Vec<u8>,
    },
    /// 🎨️ Revisioned UI patch batch for one surface — replaces `UiSection`'s cache-probe push. The
    /// reactor ABI's guest returns `turn-result.ui-patches` (`semio_framework::kernel::UiPatch`,
    /// `📓️design-abi.md` §2 "`exchange` collapse"); the host re-frames each one as this channel
    /// frame to reach a UI client. `surface`/`kind`/`revision`/`base_revision` mirror
    /// `kernel::UiPatch` field-for-field; `ops` is `kernel::PatchOp` — reused from
    /// `semio_framework::kernel`, never redefined here — pack-encoded via
    /// `store::pack_rt::encode_wire_value` (`Vec<PatchOp>`), same "nested payload stays opaque
    /// bytes" convention every other structured field in this file already uses.
    /// `base_revision` lets the client detect a stale diff and fall back to a full body instead of
    /// reconciling a diff it can't trust. `in_reply_to` is `None` for an unsolicited push (the
    /// common case — surfaces render lazily off `surface-visible`/timers, not off a command reply).
    /// CHANNEL_VERSION 12 wire addition.
    UiPatch {
        in_reply_to: Option<u64>,
        surface: String,
        kind: String,
        revision: u64,
        base_revision: u64,
        ops: Vec<u8>,
    },
    /// 🏁️ Marks the end of one surface's initial full-body snapshot burst, so a client that just
    /// subscribed (`surface-visible`) knows when it has seen a complete tree and can start applying
    /// incremental `UiPatch` frames instead of buffering. CHANNEL_VERSION 12 wire addition.
    UiSnapshotEnd {
        revision: u64,
    },
}
//#endregion 🔖️AppFrame

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `tag: u8 | fields...` — see the module-level docstring.
// `crate::os_spr::wire::🔖️WireCodec` supplies the primitives; this crate adds only the option/vec
// combinators the frame shapes need plus the tag-dispatch match arms below.

fn malformed(what: &'static str, offset: u64, detail: &str) -> crate::os_spr::ProtocolError {
    crate::os_spr::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

//#region 🔖️Combinators
fn write_opt_u64(out: &mut Vec<u8>, value: &Option<u64>) {
    crate::os_spr::write_bool(out, value.is_some());
    if let Some(v) = value {
        crate::os_spr::write_varint_u64(out, *v);
    }
}

fn read_opt_u64(bytes: &[u8], pos: &mut usize) -> Result<Option<u64>, crate::os_spr::ProtocolError> {
    if crate::os_spr::read_bool(bytes, pos)? {
        Ok(Some(crate::os_spr::read_varint_u64(bytes, pos)?))
    } else {
        Ok(None)
    }
}

/// 🎞️ `presence u8 | byte` — an `Option<u8>` (`AppCommand::Presence.own_color`), the same
/// presence-byte convention as {@link write_opt_u64} above.
fn write_opt_u8(out: &mut Vec<u8>, value: &Option<u8>) {
    crate::os_spr::write_bool(out, value.is_some());
    if let Some(v) = value {
        out.push(*v);
    }
}

fn read_opt_u8(bytes: &[u8], pos: &mut usize) -> Result<Option<u8>, crate::os_spr::ProtocolError> {
    if crate::os_spr::read_bool(bytes, pos)? {
        let byte = *bytes.get(*pos).ok_or_else(|| malformed("channel app-command opt-u8", *pos as u64, "truncated"))?;
        *pos += 1;
        Ok(Some(byte))
    } else {
        Ok(None)
    }
}

fn write_vec_bytes(out: &mut Vec<u8>, values: &[Vec<u8>]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::os_spr::write_bytes(out, value);
    }
}

fn read_vec_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<Vec<u8>>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| crate::os_spr::read_bytes(bytes, pos)).collect()
}

fn write_vec_envelope(out: &mut Vec<u8>, values: &[crate::os_spr::causal::MutationEnvelope]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::os_spr::causal::encode_envelope(value, out);
    }
}

fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<crate::os_spr::causal::MutationEnvelope>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| crate::os_spr::causal::decode_envelope(bytes, pos)).collect()
}
//#endregion 🔖️Combinators

/// @emoji 📤️ Encodes one `AppCommand`: `tag u8 | fields`.
pub fn encode_app_command(command: &AppCommand) -> Vec<u8> {
    let mut out = Vec::new();
    match command {
        AppCommand::ConfigCommand { seq, command } => {
            out.push(0);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, command);
        }
        AppCommand::Command { seq, command, view_state } => {
            out.push(1);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, command);
            crate::os_spr::write_bytes(&mut out, view_state);
        }
        AppCommand::CommandText { seq, line } => {
            out.push(2);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, line);
        }
        AppCommand::ContextMenu { seq, request } => {
            out.push(3);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, request);
        }
        AppCommand::ArtifactCommand { seq, command } => {
            out.push(4);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, command);
        }
        AppCommand::ApplyEnvelopes { seq, envelopes } => {
            out.push(5);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            write_vec_envelope(&mut out, envelopes);
        }
        AppCommand::LoadDocument { seq, pack, spr } => {
            out.push(6);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
        }
        AppCommand::ReadDocument { seq } => {
            out.push(7);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::LoadConfig { seq, pack, spr } => {
            out.push(8);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
        }
        AppCommand::ReadConfig { seq } => {
            out.push(9);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::MediaIn { seq, port, descriptor, data } => {
            out.push(10);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, descriptor);
            crate::os_spr::write_bytes(&mut out, data);
        }
        AppCommand::MediaOut { seq, port, request } => {
            out.push(11);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, request);
        }
        AppCommand::MediaFingerprint { seq, port } => {
            out.push(12);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, port);
        }
        AppCommand::PureCommand { seq, command, document, document_spr, config, config_spr, draft, draft_spr } => {
            out.push(13);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, command);
            crate::os_spr::write_bytes(&mut out, document);
            crate::os_spr::write_bytes(&mut out, document_spr);
            crate::os_spr::write_bytes(&mut out, config);
            crate::os_spr::write_bytes(&mut out, config_spr);
            crate::os_spr::write_bytes(&mut out, draft);
            crate::os_spr::write_bytes(&mut out, draft_spr);
        }
        AppCommand::LoadChildren { seq, entries } => {
            out.push(14);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            write_vec_child_pack(&mut out, entries);
        }
        AppCommand::ReadChildren { seq } => {
            out.push(15);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::ReadHistory { seq } => {
            out.push(16);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, label, origin } => {
            out.push(17);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, txn_id);
            crate::os_spr::write_str(&mut out, mutation_id);
            crate::os_spr::write_bytes(&mut out, payload);
            write_vec_bytes(&mut out, prepared_ops);
            crate::os_spr::write_str(&mut out, label);
            crate::os_spr::write_bytes(&mut out, origin);
        }
        AppCommand::TransactionCommit { seq, txn_id } => {
            out.push(18);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, txn_id);
        }
        AppCommand::TransactionRollback { seq, txn_id } => {
            out.push(19);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, txn_id);
        }
        AppCommand::TransactionUndo { seq, group_id } => {
            out.push(20);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, group_id);
        }
        AppCommand::TransactionRedo { seq, group_id } => {
            out.push(21);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, group_id);
        }
        AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => {
            out.push(22);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, artifact_ref);
            out.push(*role);
            crate::os_spr::write_str(&mut out, plugin_id);
            crate::os_spr::write_str(&mut out, app_id);
        }
        AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => {
            out.push(23);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, artifact_kind);
            crate::os_spr::write_str(&mut out, standard);
            crate::os_spr::write_str(&mut out, subset);
            out.push(*role);
            crate::os_spr::write_str(&mut out, plugin_id);
            crate::os_spr::write_str(&mut out, app_id);
        }
        AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => {
            out.push(24);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, artifact_kind);
            crate::os_spr::write_str(&mut out, standard);
            crate::os_spr::write_str(&mut out, subset);
            out.push(*role);
        }
        AppCommand::SetMergePolicy { seq, policy } => {
            out.push(25);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            out.push(*policy);
        }
        AppCommand::ResolveConflict { seq, conflict_id, resolution } => {
            out.push(26);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, conflict_id);
            out.push(*resolution);
        }
        AppCommand::ReadConflicts { seq } => {
            out.push(27);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::Presence { seq, own_color, peers } => {
            out.push(28);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            write_opt_u8(&mut out, own_color);
            write_vec_bytes(&mut out, peers);
        }
    }
    out
}

/// @emoji 🧸️ `count varint | (slot, child_id, dialect, envelope_pack)*` — the shared list codec for
/// both `AppCommand::LoadChildren` and `AppFrame::Children`.
fn write_vec_child_pack(out: &mut Vec<u8>, entries: &[ChildPackEntry]) {
    crate::os_spr::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        crate::os_spr::write_str(out, &entry.slot);
        crate::os_spr::write_str(out, &entry.child_id);
        crate::os_spr::write_str(out, &entry.dialect);
        crate::os_spr::write_bytes(out, &entry.envelope_pack);
    }
}

/// @emoji 🧸️ Inverse of [`write_vec_child_pack`].
fn read_vec_child_pack(bytes: &[u8], pos: &mut usize) -> Result<Vec<ChildPackEntry>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(ChildPackEntry {
            slot: crate::os_spr::read_str(bytes, pos)?,
            child_id: crate::os_spr::read_str(bytes, pos)?,
            dialect: crate::os_spr::read_str(bytes, pos)?,
            envelope_pack: crate::os_spr::read_bytes(bytes, pos)?,
        });
    }
    Ok(entries)
}

/// @emoji 📥️ Decodes one `AppCommand`, the inverse of [`encode_app_command`].
pub fn decode_app_command(bytes: &[u8]) -> Result<AppCommand, crate::os_spr::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-command tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let command = match tag {
        0 => AppCommand::ConfigCommand { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)? },
        1 => AppCommand::Command { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)?, view_state: crate::os_spr::read_bytes(bytes, &mut pos)? },
        2 => AppCommand::CommandText { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, line: crate::os_spr::read_str(bytes, &mut pos)? },
        3 => AppCommand::ContextMenu { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, request: crate::os_spr::read_bytes(bytes, &mut pos)? },
        4 => AppCommand::ArtifactCommand { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)? },
        5 => AppCommand::ApplyEnvelopes { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos)? },
        6 => AppCommand::LoadDocument { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)? },
        7 => AppCommand::ReadDocument { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        8 => AppCommand::LoadConfig { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)? },
        9 => AppCommand::ReadConfig { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        10 => AppCommand::MediaIn { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, descriptor: crate::os_spr::read_bytes(bytes, &mut pos)?, data: crate::os_spr::read_bytes(bytes, &mut pos)? },
        11 => AppCommand::MediaOut { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, request: crate::os_spr::read_bytes(bytes, &mut pos)? },
        12 => AppCommand::MediaFingerprint { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)? },
        13 => AppCommand::PureCommand {
            seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            command: crate::os_spr::read_bytes(bytes, &mut pos)?,
            document: crate::os_spr::read_bytes(bytes, &mut pos)?,
            document_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        14 => AppCommand::LoadChildren { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos)? },
        15 => AppCommand::ReadChildren { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        16 => AppCommand::ReadHistory { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        17 => AppCommand::TransactionPrepare {
            seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            txn_id: crate::os_spr::read_str(bytes, &mut pos)?,
            mutation_id: crate::os_spr::read_str(bytes, &mut pos)?,
            payload: crate::os_spr::read_bytes(bytes, &mut pos)?,
            prepared_ops: read_vec_bytes(bytes, &mut pos)?,
            label: crate::os_spr::read_str(bytes, &mut pos)?,
            origin: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        18 => AppCommand::TransactionCommit { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
        19 => AppCommand::TransactionRollback { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
        20 => AppCommand::TransactionUndo { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, group_id: crate::os_spr::read_str(bytes, &mut pos)? },
        21 => AppCommand::TransactionRedo { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, group_id: crate::os_spr::read_str(bytes, &mut pos)? },
        22 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let artifact_ref = crate::os_spr::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command OpenArtifact.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = crate::os_spr::read_str(bytes, &mut pos)?;
            let app_id = crate::os_spr::read_str(bytes, &mut pos)?;
            AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id }
        }
        23 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = crate::os_spr::read_str(bytes, &mut pos)?;
            let standard = crate::os_spr::read_str(bytes, &mut pos)?;
            let subset = crate::os_spr::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command SetDefaultApp.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = crate::os_spr::read_str(bytes, &mut pos)?;
            let app_id = crate::os_spr::read_str(bytes, &mut pos)?;
            AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id }
        }
        24 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = crate::os_spr::read_str(bytes, &mut pos)?;
            let standard = crate::os_spr::read_str(bytes, &mut pos)?;
            let subset = crate::os_spr::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ClearDefaultApp.role", pos as u64, "truncated"))?;
            AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role }
        }
        25 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let policy = *bytes.get(pos).ok_or_else(|| malformed("channel app-command SetMergePolicy.policy", pos as u64, "truncated"))?;
            AppCommand::SetMergePolicy { seq, policy }
        }
        26 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let conflict_id = crate::os_spr::read_str(bytes, &mut pos)?;
            let resolution = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ResolveConflict.resolution", pos as u64, "truncated"))?;
            AppCommand::ResolveConflict { seq, conflict_id, resolution }
        }
        27 => AppCommand::ReadConflicts { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        28 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let own_color = read_opt_u8(bytes, &mut pos)?;
            let peers = read_vec_bytes(bytes, &mut pos)?;
            AppCommand::Presence { seq, own_color, peers }
        }
        other => return Err(malformed("channel app-command tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(command)
}

/// @emoji 📤️ Encodes one `AppFrame`: `tag u8 | fields`.
pub fn encode_app_frame(frame: &AppFrame) -> Vec<u8> {
    let mut out = Vec::new();
    match frame {
        AppFrame::Done { in_reply_to } => {
            out.push(0);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
        }
        AppFrame::Invocation { in_reply_to, output, diagnostics, ui_scope, history_patch, messages } => {
            out.push(1);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, output);
            crate::os_spr::write_bytes(&mut out, diagnostics);
            crate::os_spr::write_bytes(&mut out, ui_scope);
            crate::os_spr::write_bytes(&mut out, history_patch);
            crate::os_spr::write_bytes(&mut out, messages);
        }
        AppFrame::DocumentChanged { envelopes, origin } => {
            out.push(2);
            write_vec_envelope(&mut out, envelopes);
            crate::os_spr::write_str(&mut out, origin);
        }
        AppFrame::Document { in_reply_to, pack, spr, ops } => {
            out.push(3);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::Config { in_reply_to, pack, spr, ops } => {
            out.push(4);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::ConfigChanged { envelopes, origin } => {
            out.push(5);
            write_vec_envelope(&mut out, envelopes);
            crate::os_spr::write_str(&mut out, origin);
        }
        AppFrame::ContextMenu { in_reply_to, items } => {
            out.push(6);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, items);
        }
        AppFrame::Media { in_reply_to, port, descriptor, data } => {
            out.push(7);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, descriptor);
            crate::os_spr::write_bytes(&mut out, data);
        }
        AppFrame::MediaFingerprint { in_reply_to, port, fingerprint } => {
            out.push(8);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, fingerprint);
        }
        AppFrame::Error { in_reply_to, fault, report } => {
            out.push(9);
            write_opt_u64(&mut out, in_reply_to);
            crate::os_spr::write_bytes(&mut out, fault);
            crate::os_spr::write_bytes(&mut out, report);
        }
        AppFrame::Emit { in_reply_to, document_ops, config_ops, draft_ops, output, diagnostics } => {
            out.push(10);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, document_ops);
            crate::os_spr::write_bytes(&mut out, config_ops);
            crate::os_spr::write_bytes(&mut out, draft_ops);
            crate::os_spr::write_bytes(&mut out, output);
            crate::os_spr::write_bytes(&mut out, diagnostics);
        }
        AppFrame::Draft { in_reply_to, pack, spr, ops } => {
            out.push(11);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::Children { in_reply_to, entries } => {
            out.push(12);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            write_vec_child_pack(&mut out, entries);
        }
        AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction } => {
            out.push(13);
            crate::os_spr::write_bytes(&mut out, presence);
            crate::os_spr::write_varint_u64(&mut out, *presence_generation);
            crate::os_spr::write_varint_u64(&mut out, *transient_generation);
            crate::os_spr::write_bytes(&mut out, interaction);
        }
        AppFrame::HistorySnapshot { in_reply_to, history_patch } => {
            out.push(14);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, history_patch);
        }
        AppFrame::TransactionProposal { in_reply_to, proposal_id, local_ops, description, coalesce_key, foreign } => {
            out.push(15);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, proposal_id);
            write_vec_bytes(&mut out, local_ops);
            crate::os_spr::write_str(&mut out, description);
            crate::os_spr::write_str(&mut out, coalesce_key);
            write_vec_bytes(&mut out, foreign);
        }
        AppFrame::TransactionPrepared { txn_id, foreign, rejection } => {
            out.push(16);
            crate::os_spr::write_str(&mut out, txn_id);
            write_vec_bytes(&mut out, foreign);
            crate::os_spr::write_bytes(&mut out, rejection);
        }
        AppFrame::TransactionCommitted { txn_id, edit_id } => {
            out.push(17);
            crate::os_spr::write_str(&mut out, txn_id);
            crate::os_spr::write_str(&mut out, edit_id);
        }
        AppFrame::TransactionRolledBack { txn_id } => {
            out.push(18);
            crate::os_spr::write_str(&mut out, txn_id);
        }
        AppFrame::MergeReport { in_reply_to, report } => {
            out.push(19);
            write_opt_u64(&mut out, in_reply_to);
            crate::os_spr::write_bytes(&mut out, report);
        }
        AppFrame::Conflicts { in_reply_to, conflicts } => {
            out.push(20);
            write_opt_u64(&mut out, in_reply_to);
            crate::os_spr::write_bytes(&mut out, conflicts);
        }
        AppFrame::UiPatch { in_reply_to, surface, kind, revision, base_revision, ops } => {
            out.push(21);
            write_opt_u64(&mut out, in_reply_to);
            crate::os_spr::write_str(&mut out, surface);
            crate::os_spr::write_str(&mut out, kind);
            crate::os_spr::write_varint_u64(&mut out, *revision);
            crate::os_spr::write_varint_u64(&mut out, *base_revision);
            crate::os_spr::write_bytes(&mut out, ops);
        }
        AppFrame::UiSnapshotEnd { revision } => {
            out.push(22);
            crate::os_spr::write_varint_u64(&mut out, *revision);
        }
    }
    out
}

/// @emoji 📥️ Decodes one `AppFrame`, the inverse of [`encode_app_frame`].
pub fn decode_app_frame(bytes: &[u8]) -> Result<AppFrame, crate::os_spr::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-frame tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let frame = match tag {
        0 => AppFrame::Done { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        1 => AppFrame::Invocation {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            output: crate::os_spr::read_bytes(bytes, &mut pos)?,
            diagnostics: crate::os_spr::read_bytes(bytes, &mut pos)?,
            ui_scope: crate::os_spr::read_bytes(bytes, &mut pos)?,
            history_patch: crate::os_spr::read_bytes(bytes, &mut pos)?,
            messages: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        2 => AppFrame::DocumentChanged { envelopes: read_vec_envelope(bytes, &mut pos)?, origin: crate::os_spr::read_str(bytes, &mut pos)? },
        3 => AppFrame::Document { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)?, ops: crate::os_spr::read_str(bytes, &mut pos)? },
        4 => AppFrame::Config { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)?, ops: crate::os_spr::read_str(bytes, &mut pos)? },
        5 => AppFrame::ConfigChanged { envelopes: read_vec_envelope(bytes, &mut pos)?, origin: crate::os_spr::read_str(bytes, &mut pos)? },
        6 => AppFrame::ContextMenu { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, items: crate::os_spr::read_bytes(bytes, &mut pos)? },
        7 => {
            AppFrame::Media { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, descriptor: crate::os_spr::read_bytes(bytes, &mut pos)?, data: crate::os_spr::read_bytes(bytes, &mut pos)? }
        }
        8 => AppFrame::MediaFingerprint { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, fingerprint: crate::os_spr::read_bytes(bytes, &mut pos)? },
        9 => AppFrame::Error { in_reply_to: read_opt_u64(bytes, &mut pos)?, fault: crate::os_spr::read_bytes(bytes, &mut pos)?, report: crate::os_spr::read_bytes(bytes, &mut pos)? },
        10 => AppFrame::Emit {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            document_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            output: crate::os_spr::read_bytes(bytes, &mut pos)?,
            diagnostics: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        11 => AppFrame::Draft {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            pack: crate::os_spr::read_bytes(bytes, &mut pos)?,
            spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            ops: crate::os_spr::read_str(bytes, &mut pos)?,
        },
        12 => AppFrame::Children { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos)? },
        13 => AppFrame::Ephemeral {
            presence: crate::os_spr::read_bytes(bytes, &mut pos)?,
            presence_generation: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            transient_generation: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            interaction: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        14 => AppFrame::HistorySnapshot { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, history_patch: crate::os_spr::read_bytes(bytes, &mut pos)? },
        15 => AppFrame::TransactionProposal {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            proposal_id: crate::os_spr::read_str(bytes, &mut pos)?,
            local_ops: read_vec_bytes(bytes, &mut pos)?,
            description: crate::os_spr::read_str(bytes, &mut pos)?,
            coalesce_key: crate::os_spr::read_str(bytes, &mut pos)?,
            foreign: read_vec_bytes(bytes, &mut pos)?,
        },
        16 => AppFrame::TransactionPrepared {
            txn_id: crate::os_spr::read_str(bytes, &mut pos)?,
            foreign: read_vec_bytes(bytes, &mut pos)?,
            rejection: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        17 => AppFrame::TransactionCommitted { txn_id: crate::os_spr::read_str(bytes, &mut pos)?, edit_id: crate::os_spr::read_str(bytes, &mut pos)? },
        18 => AppFrame::TransactionRolledBack { txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
        19 => AppFrame::MergeReport { in_reply_to: read_opt_u64(bytes, &mut pos)?, report: crate::os_spr::read_bytes(bytes, &mut pos)? },
        20 => AppFrame::Conflicts { in_reply_to: read_opt_u64(bytes, &mut pos)?, conflicts: crate::os_spr::read_bytes(bytes, &mut pos)? },
        21 => {
            let in_reply_to = read_opt_u64(bytes, &mut pos)?;
            let surface = crate::os_spr::read_str(bytes, &mut pos)?;
            let kind = crate::os_spr::read_str(bytes, &mut pos)?;
            let revision = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let base_revision = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let ops = crate::os_spr::read_bytes(bytes, &mut pos)?;
            AppFrame::UiPatch { in_reply_to, surface, kind, revision, base_revision, ops }
        }
        22 => AppFrame::UiSnapshotEnd { revision: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        other => return Err(malformed("channel app-frame tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(frame)
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    fn sample_envelope(id: &str) -> crate::os_spr::causal::MutationEnvelope {
        crate::os_spr::causal::MutationEnvelope {
            mutation_id: crate::os_spr::ids::MutationId(id.to_string()),
            document_id: crate::os_spr::ids::ArtifactId("document-1".to_string()),
            actor: crate::os_spr::ids::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: crate::os_spr::causal::ArtifactDiff { schema: crate::os_spr::ids::SchemaId("diff.v1".to_string()), payload: format!("value:{id}").into_bytes() },
            inverse: crate::os_spr::causal::InverseMutation { schema: crate::os_spr::ids::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 0),
        }
    }

    /// @emoji #️⃣ Tiny hand-rolled `&[u8] -> String` hex encoder for this crate's own fixture-corpus
    /// tests — mirrors `db_engine`'s `write!("{byte:02x}")` idiom (no `hex` crate dependency exists
    /// anywhere in `framework/product/os`, so this crate does not introduce one either).
    fn hex_encode(bytes: &[u8]) -> String {
        use std::fmt::Write;
        let mut out = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            let _ = write!(out, "{byte:02x}");
        }
        out
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️AppCommand
    fn assert_command_round_trips(command: &AppCommand) {
        let bytes = encode_app_command(command);
        let decoded = decode_app_command(&bytes).expect("decode must succeed");
        assert_eq!(&decoded, command);
    }

    #[test]
    fn app_command_config_command_round_trips() {
        assert_command_round_trips(&AppCommand::ConfigCommand { seq: 1, command: vec![9, 9] });
    }

    #[test]
    fn app_command_command_round_trips() {
        assert_command_round_trips(&AppCommand::Command { seq: 2, command: vec![1, 2], view_state: vec![] });
    }

    #[test]
    fn app_command_command_text_round_trips() {
        assert_command_round_trips(&AppCommand::CommandText { seq: 3, line: "set foo = 1".to_string() });
    }

    #[test]
    fn app_command_context_menu_round_trips() {
        assert_command_round_trips(&AppCommand::ContextMenu { seq: 5, request: vec![7] });
    }

    #[test]
    fn app_command_document_command_round_trips() {
        assert_command_round_trips(&AppCommand::ArtifactCommand { seq: 6, command: vec![8, 8] });
    }

    #[test]
    fn app_command_apply_envelopes_round_trips() {
        assert_command_round_trips(&AppCommand::ApplyEnvelopes { seq: 7, envelopes: vec![sample_envelope("op-1"), sample_envelope("op-2")] });
    }

    #[test]
    fn app_command_load_document_round_trips() {
        assert_command_round_trips(&AppCommand::LoadDocument { seq: 8, pack: vec![1], spr: vec![2] });
    }

    #[test]
    fn app_command_read_artifact_round_trips() {
        assert_command_round_trips(&AppCommand::ReadDocument { seq: 9 });
    }

    #[test]
    fn app_command_load_config_round_trips() {
        assert_command_round_trips(&AppCommand::LoadConfig { seq: 10, pack: vec![1], spr: vec![2] });
    }

    #[test]
    fn app_command_read_config_round_trips() {
        assert_command_round_trips(&AppCommand::ReadConfig { seq: 11 });
    }

    #[test]
    fn app_command_media_in_round_trips() {
        assert_command_round_trips(&AppCommand::MediaIn { seq: 14, port: "camera".to_string(), descriptor: vec![1], data: vec![2, 3] });
    }

    #[test]
    fn app_command_media_out_round_trips() {
        assert_command_round_trips(&AppCommand::MediaOut { seq: 15, port: "speaker".to_string(), request: vec![4] });
    }

    #[test]
    fn app_command_media_fingerprint_round_trips() {
        assert_command_round_trips(&AppCommand::MediaFingerprint { seq: 16, port: "camera".to_string() });
    }

    #[test]
    fn app_command_pure_command_round_trips() {
        assert_command_round_trips(&AppCommand::PureCommand {
            seq: 18,
            command: vec![1],
            document: vec![2],
            document_spr: vec![3],
            config: vec![4],
            config_spr: vec![5],
            draft: vec![6],
            draft_spr: vec![7],
        });
    }

    //#region 🔖️Transaction
    #[test]
    fn app_command_transaction_prepare_round_trips_owner_and_preplanned_forms() {
        assert_command_round_trips(&AppCommand::TransactionPrepare {
            seq: 1,
            txn_id: "t".to_string(),
            mutation_id: "m".to_string(),
            payload: vec![9],
            prepared_ops: Vec::new(),
            label: String::new(),
            origin: Vec::new(),
        });
        assert_command_round_trips(&AppCommand::TransactionPrepare {
            seq: 2,
            txn_id: "t".to_string(),
            mutation_id: String::new(),
            payload: Vec::new(),
            prepared_ops: vec![vec![1], vec![2, 2]],
            label: "l".to_string(),
            origin: vec![9],
        });
    }

    #[test]
    fn app_command_transaction_commit_round_trips() {
        assert_command_round_trips(&AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() });
    }

    #[test]
    fn app_command_transaction_rollback_round_trips() {
        assert_command_round_trips(&AppCommand::TransactionRollback { seq: 4, txn_id: "t".to_string() });
    }

    #[test]
    fn app_command_transaction_undo_round_trips() {
        assert_command_round_trips(&AppCommand::TransactionUndo { seq: 5, group_id: "g".to_string() });
    }

    #[test]
    fn app_command_transaction_redo_round_trips() {
        assert_command_round_trips(&AppCommand::TransactionRedo { seq: 6, group_id: "g".to_string() });
    }
    //#endregion 🔖️Transaction

    //#region 🔖️Opening
    #[test]
    fn app_command_open_artifact_round_trips_resolved_and_explicit_forms() {
        assert_command_round_trips(&AppCommand::OpenArtifact { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer".to_string(), role: 0, plugin_id: String::new(), app_id: String::new() });
        assert_command_round_trips(&AppCommand::OpenArtifact { seq: 2, artifact_ref: "s.cad.cad@1/*#editor".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() });
    }

    #[test]
    fn app_command_set_default_app_round_trips() {
        assert_command_round_trips(&AppCommand::SetDefaultApp { seq: 3, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() });
    }

    #[test]
    fn app_command_clear_default_app_round_trips() {
        assert_command_round_trips(&AppCommand::ClearDefaultApp { seq: 4, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 0 });
    }
    //#endregion 🔖️Opening

    //#region 🔖️Merge
    #[test]
    fn app_command_set_merge_policy_round_trips() {
        assert_command_round_trips(&AppCommand::SetMergePolicy { seq: 5, policy: 0 });
        assert_command_round_trips(&AppCommand::SetMergePolicy { seq: 6, policy: 2 });
    }

    #[test]
    fn app_command_resolve_conflict_round_trips() {
        assert_command_round_trips(&AppCommand::ResolveConflict { seq: 7, conflict_id: "c-1".to_string(), resolution: 0 });
        assert_command_round_trips(&AppCommand::ResolveConflict { seq: 8, conflict_id: "c-1".to_string(), resolution: 1 });
    }

    #[test]
    fn app_command_read_conflicts_round_trips() {
        assert_command_round_trips(&AppCommand::ReadConflicts { seq: 9 });
    }
    //#endregion 🔖️Merge
    //#endregion 🔖️AppCommand

    //#region 🔖️AppFrame
    fn assert_frame_round_trips(frame: &AppFrame) {
        let bytes = encode_app_frame(frame);
        let decoded = decode_app_frame(&bytes).expect("decode must succeed");
        assert_eq!(&decoded, frame);
    }

    #[test]
    fn app_frame_done_round_trips() {
        assert_frame_round_trips(&AppFrame::Done { in_reply_to: 1 });
    }

    #[test]
    fn app_frame_invocation_round_trips() {
        assert_frame_round_trips(&AppFrame::Invocation { in_reply_to: 2, output: vec![1], diagnostics: vec![2], ui_scope: vec![3], history_patch: vec![4], messages: vec![5] });
        assert_frame_round_trips(&AppFrame::Invocation { in_reply_to: 2, output: vec![1], diagnostics: vec![2], ui_scope: vec![3], history_patch: vec![4], messages: Vec::new() });
    }

    #[test]
    fn app_frame_document_changed_round_trips() {
        assert_frame_round_trips(&AppFrame::DocumentChanged { envelopes: vec![sample_envelope("op-1")], origin: "peer-1".to_string() });
    }

    #[test]
    fn app_frame_document_round_trips() {
        assert_frame_round_trips(&AppFrame::Document { in_reply_to: 5, pack: vec![1], spr: vec![2], ops: "set foo = 1".to_string() });
    }

    #[test]
    fn app_frame_config_round_trips() {
        assert_frame_round_trips(&AppFrame::Config { in_reply_to: 5, pack: vec![1], spr: vec![2], ops: "set cam = 1".to_string() });
    }

    #[test]
    fn app_frame_config_changed_round_trips() {
        assert_frame_round_trips(&AppFrame::ConfigChanged { envelopes: vec![sample_envelope("cfg-1")], origin: "peer-1".to_string() });
    }

    #[test]
    fn app_frame_context_menu_round_trips() {
        assert_frame_round_trips(&AppFrame::ContextMenu { in_reply_to: 6, items: vec![1, 2, 3] });
    }

    #[test]
    fn app_frame_media_round_trips() {
        assert_frame_round_trips(&AppFrame::Media { in_reply_to: 7, port: "camera".to_string(), descriptor: vec![1], data: vec![2] });
    }

    #[test]
    fn app_frame_media_fingerprint_round_trips() {
        assert_frame_round_trips(&AppFrame::MediaFingerprint { in_reply_to: 8, port: "camera".to_string(), fingerprint: vec![1, 2] });
    }

    #[test]
    fn app_frame_error_round_trips() {
        assert_frame_round_trips(&AppFrame::Error { in_reply_to: Some(9), fault: b"rejected:bad command".to_vec(), report: vec![1, 2] });
        assert_frame_round_trips(&AppFrame::Error { in_reply_to: None, fault: b"rejected:bad command".to_vec(), report: Vec::new() });
    }

    #[test]
    fn app_frame_emit_round_trips() {
        assert_frame_round_trips(&AppFrame::Emit {
            in_reply_to: 14,
            document_ops: vec![1],
            config_ops: vec![2],
            draft_ops: vec![3],
            output: vec![4],
            diagnostics: vec![5],
        });
    }

    #[test]
    fn app_frame_draft_round_trips() {
        assert_frame_round_trips(&AppFrame::Draft { in_reply_to: 15, pack: vec![1], spr: vec![2], ops: "d".to_string() });
        assert_frame_round_trips(&AppFrame::Children { in_reply_to: 16, entries: sample_child_entries() });
        assert_frame_round_trips(&AppFrame::Children { in_reply_to: 17, entries: Vec::new() });
        assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![9, 9] });
        assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: Vec::new() });
    }

    //#region 🔖️Children
    /// 🧸️ Two children in different slots, one of them with an empty pack (a genesis child whose
    /// envelope has not been printed yet), so the list codec is exercised at both extremes.
    fn sample_child_entries() -> Vec<ChildPackEntry> {
        vec![
            ChildPackEntry { slot: "mesh".to_string(), child_id: "child-1".to_string(), dialect: "s.stdio.mesh@1/*".to_string(), envelope_pack: vec![7, 8, 9] },
            ChildPackEntry { slot: "brep".to_string(), child_id: "child-2".to_string(), dialect: "s.stdio.brep@1/*".to_string(), envelope_pack: Vec::new() },
        ]
    }

    #[test]
    fn child_pack_commands_round_trip() {
        assert_command_round_trips(&AppCommand::LoadChildren { seq: 19, entries: sample_child_entries() });
        assert_command_round_trips(&AppCommand::LoadChildren { seq: 20, entries: Vec::new() });
        assert_command_round_trips(&AppCommand::ReadChildren { seq: 21 });
        assert_command_round_trips(&AppCommand::ReadHistory { seq: 22 });
    }
    //#endregion 🔖️Children

    //#region 🔖️Transaction
    #[test]
    fn app_frame_transaction_proposal_round_trips() {
        assert_frame_round_trips(&AppFrame::TransactionProposal {
            in_reply_to: 1,
            proposal_id: "p".to_string(),
            local_ops: vec![vec![1]],
            description: "d".to_string(),
            coalesce_key: "k".to_string(),
            foreign: Vec::new(),
        });
    }

    #[test]
    fn app_frame_transaction_prepared_round_trips_with_and_without_rejection() {
        assert_frame_round_trips(&AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: vec![vec![1]], rejection: Vec::new() });
        assert_frame_round_trips(&AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: Vec::new(), rejection: b"rejected".to_vec() });
    }

    #[test]
    fn app_frame_transaction_committed_round_trips() {
        assert_frame_round_trips(&AppFrame::TransactionCommitted { txn_id: "t".to_string(), edit_id: "e".to_string() });
    }

    #[test]
    fn app_frame_transaction_rolled_back_round_trips() {
        assert_frame_round_trips(&AppFrame::TransactionRolledBack { txn_id: "t".to_string() });
    }
    //#endregion 🔖️Transaction

    //#region 🔖️Merge
    #[test]
    fn app_frame_merge_report_round_trips() {
        assert_frame_round_trips(&AppFrame::MergeReport { in_reply_to: Some(1), report: vec![1, 2, 3] });
        assert_frame_round_trips(&AppFrame::MergeReport { in_reply_to: None, report: Vec::new() });
    }

    #[test]
    fn app_frame_conflicts_round_trips() {
        assert_frame_round_trips(&AppFrame::Conflicts { in_reply_to: Some(2), conflicts: vec![4, 5] });
        assert_frame_round_trips(&AppFrame::Conflicts { in_reply_to: None, conflicts: Vec::new() });
    }
    //#endregion 🔖️Merge

    //#region 🔖️UiPatch
    #[test]
    fn app_frame_ui_patch_round_trips_with_and_without_in_reply_to() {
        assert_frame_round_trips(&AppFrame::UiPatch {
            in_reply_to: Some(3),
            surface: "1:body".to_string(),
            kind: "window".to_string(),
            revision: 5,
            base_revision: 4,
            ops: vec![1, 2, 3],
        });
        assert_frame_round_trips(&AppFrame::UiPatch {
            in_reply_to: None,
            surface: "1:body".to_string(),
            kind: "window".to_string(),
            revision: 1,
            base_revision: 0,
            ops: Vec::new(),
        });
    }

    #[test]
    fn app_frame_ui_snapshot_end_round_trips() {
        assert_frame_round_trips(&AppFrame::UiSnapshotEnd { revision: 7 });
    }
    //#endregion 🔖️UiPatch
    //#endregion 🔖️AppFrame

    //#region 🔖️Codec
    #[test]
    fn encoding_is_deterministic() {
        let command = AppCommand::ContextMenu { seq: 1, request: vec![1, 2, 3] };
        assert_eq!(encode_app_command(&command), encode_app_command(&command));

        let frame = AppFrame::Error { in_reply_to: Some(1), fault: b"e:m".to_vec(), report: vec![9] };
        assert_eq!(encode_app_frame(&frame), encode_app_frame(&frame));
    }

    #[test]
    fn decode_app_command_rejects_empty_bytes() {
        let err = decode_app_command(&[]).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-command tag", .. }));
    }

    #[test]
    fn decode_app_frame_rejects_empty_bytes() {
        let err = decode_app_frame(&[]).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
    }

    #[test]
    fn decode_app_command_rejects_unknown_tag() {
        let err = decode_app_command(&[0xFF]).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-command tag", .. }));
    }

    #[test]
    fn decode_app_frame_rejects_unknown_tag() {
        let err = decode_app_frame(&[0xFF]).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
    }

    #[test]
    fn decode_app_command_rejects_truncated_field() {
        let bytes = encode_app_command(&AppCommand::CommandText { seq: 1, line: "hello".to_string() });
        let truncated = &bytes[..bytes.len() - 2];
        assert!(decode_app_command(truncated).is_err());
    }

    #[test]
    fn decode_app_frame_rejects_truncated_field() {
        let bytes = encode_app_frame(&AppFrame::Error { in_reply_to: Some(1), fault: b"e:message".to_vec(), report: Vec::new() });
        let truncated = &bytes[..bytes.len() - 2];
        assert!(decode_app_frame(truncated).is_err());
    }

    #[test]
    fn decode_app_command_never_panics_on_arbitrary_short_buffers() {
        for len in 0..8 {
            let buf = vec![0u8; len];
            let _ = decode_app_command(&buf);
        }
    }

    #[test]
    fn decode_app_frame_never_panics_on_arbitrary_short_buffers() {
        for len in 0..8 {
            let buf = vec![0u8; len];
            let _ = decode_app_frame(&buf);
        }
    }
    //#endregion 🔖️Codec

    //#region 🔖️Corpus
    // Cross-language drift fixture: a sibling TypeScript work package duplicates these exact hex
    // strings in a vitest suite, so `AppCommand`/`AppFrame` and the TS-side codec they hand-port
    // stay byte-exact. Every entry is `(variant label, value)`; `channel_command_fixture_hex`/
    // `channel_frame_fixture_hex` below are this codec's own committed golden hex per label —
    // sourced from `encode_app_command`/`encode_app_frame`'s actual output, not hand-computed.

    /// @emoji 🧾️ Named `AppCommand` fixture corpus, one entry per variant.
    fn channel_command_fixture_corpus() -> Vec<(&'static str, AppCommand)> {
        vec![
            ("ConfigCommand", AppCommand::ConfigCommand { seq: 1, command: vec![9] }),
            ("Command", AppCommand::Command { seq: 1, command: vec![1], view_state: vec![] }),
            ("CommandText", AppCommand::CommandText { seq: 1, line: "go".to_string() }),
            ("ContextMenu", AppCommand::ContextMenu { seq: 1, request: vec![1] }),
            ("ArtifactCommand", AppCommand::ArtifactCommand { seq: 1, command: vec![1] }),
            ("ApplyEnvelopes", AppCommand::ApplyEnvelopes { seq: 1, envelopes: Vec::new() }),
            ("LoadDocument", AppCommand::LoadDocument { seq: 1, pack: vec![1], spr: vec![2] }),
            ("ReadDocument", AppCommand::ReadDocument { seq: 1 }),
            ("LoadConfig", AppCommand::LoadConfig { seq: 1, pack: vec![1], spr: vec![2] }),
            ("ReadConfig", AppCommand::ReadConfig { seq: 1 }),
            ("MediaIn", AppCommand::MediaIn { seq: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
            ("MediaOut", AppCommand::MediaOut { seq: 1, port: "p".to_string(), request: vec![1] }),
            ("MediaFingerprint", AppCommand::MediaFingerprint { seq: 1, port: "p".to_string() }),
            ("PureCommand", AppCommand::PureCommand {
                seq: 1,
                command: vec![1],
                document: vec![2],
                document_spr: vec![3],
                config: vec![4],
                config_spr: vec![5],
                draft: vec![6],
                draft_spr: vec![7],
            }),
            ("LoadChildren", AppCommand::LoadChildren { seq: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1] }] }),
            ("ReadChildren", AppCommand::ReadChildren { seq: 1 }),
            ("ReadHistory", AppCommand::ReadHistory { seq: 1 }),
            ("TransactionPrepareOwner", AppCommand::TransactionPrepare {
                seq: 1,
                txn_id: "t".to_string(),
                mutation_id: "m".to_string(),
                payload: vec![9],
                prepared_ops: Vec::new(),
                label: String::new(),
                origin: Vec::new(),
            }),
            ("TransactionPreparePrePlanned", AppCommand::TransactionPrepare {
                seq: 2,
                txn_id: "t".to_string(),
                mutation_id: String::new(),
                payload: Vec::new(),
                prepared_ops: vec![vec![1], vec![2, 2]],
                label: "l".to_string(),
                origin: vec![9],
            }),
            ("TransactionCommit", AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() }),
            ("TransactionRollback", AppCommand::TransactionRollback { seq: 4, txn_id: "t".to_string() }),
            ("TransactionUndo", AppCommand::TransactionUndo { seq: 5, group_id: "g".to_string() }),
            ("TransactionRedo", AppCommand::TransactionRedo { seq: 6, group_id: "g".to_string() }),
            ("OpenArtifactResolve", AppCommand::OpenArtifact { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer".to_string(), role: 0, plugin_id: String::new(), app_id: String::new() }),
            ("OpenArtifactExplicit", AppCommand::OpenArtifact { seq: 2, artifact_ref: "s.cad.cad@1/*#editor".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() }),
            ("SetDefaultApp", AppCommand::SetDefaultApp { seq: 3, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() }),
            ("ClearDefaultApp", AppCommand::ClearDefaultApp { seq: 4, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 0 }),
            ("SetMergePolicy", AppCommand::SetMergePolicy { seq: 5, policy: 1 }),
            ("ResolveConflict", AppCommand::ResolveConflict { seq: 6, conflict_id: "conflict-1".to_string(), resolution: 0 }),
            ("ReadConflicts", AppCommand::ReadConflicts { seq: 7 }),
            ("Presence", AppCommand::Presence { seq: 8, own_color: Some(3), peers: vec![vec![1, 2], vec![9]] }),
        ]
    }

    /// @emoji 🧾️ Named `AppFrame` fixture corpus, one entry per variant.
    fn channel_frame_fixture_corpus() -> Vec<(&'static str, AppFrame)> {
        vec![
            ("Done", AppFrame::Done { in_reply_to: 1 }),
            ("Invocation", AppFrame::Invocation { in_reply_to: 1, output: vec![1], diagnostics: vec![], ui_scope: vec![], history_patch: vec![], messages: vec![9] }),
            ("DocumentChanged", AppFrame::DocumentChanged { envelopes: vec![], origin: "o".to_string() }),
            ("Document", AppFrame::Document { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "o".to_string() }),
            ("Config", AppFrame::Config { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "c".to_string() }),
            ("ConfigChanged", AppFrame::ConfigChanged { envelopes: vec![], origin: "o".to_string() }),
            ("ContextMenu", AppFrame::ContextMenu { in_reply_to: 1, items: vec![1] }),
            ("Media", AppFrame::Media { in_reply_to: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
            ("MediaFingerprint", AppFrame::MediaFingerprint { in_reply_to: 1, port: "p".to_string(), fingerprint: vec![1] }),
            ("Error", AppFrame::Error { in_reply_to: None, fault: vec![99], report: vec![7] }),
            ("Emit", AppFrame::Emit {
                in_reply_to: 1,
                document_ops: vec![1],
                config_ops: vec![],
                draft_ops: vec![],
                output: vec![2],
                diagnostics: vec![],
            }),
            ("Draft", AppFrame::Draft { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "d".to_string() }),
            ("Children", AppFrame::Children { in_reply_to: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1] }] }),
            ("Ephemeral", AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![7] }),
            ("HistorySnapshot", AppFrame::HistorySnapshot { in_reply_to: 1, history_patch: vec![1] }),
            ("TransactionProposal", AppFrame::TransactionProposal {
                in_reply_to: 1,
                proposal_id: "p".to_string(),
                local_ops: vec![vec![1]],
                description: "d".to_string(),
                coalesce_key: "k".to_string(),
                foreign: Vec::new(),
            }),
            ("TransactionPrepared", AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: vec![vec![1]], rejection: Vec::new() }),
            ("TransactionCommitted", AppFrame::TransactionCommitted { txn_id: "t".to_string(), edit_id: "e".to_string() }),
            ("TransactionRolledBack", AppFrame::TransactionRolledBack { txn_id: "t".to_string() }),
            ("MergeReport", AppFrame::MergeReport { in_reply_to: Some(1), report: vec![1] }),
            ("Conflicts", AppFrame::Conflicts { in_reply_to: None, conflicts: vec![2] }),
            ("UiPatch", AppFrame::UiPatch { in_reply_to: Some(1), surface: "1:body".to_string(), kind: "window".to_string(), revision: 2, base_revision: 1, ops: vec![3] }),
            ("UiSnapshotEnd", AppFrame::UiSnapshotEnd { revision: 4 }),
        ]
    }

    /// @emoji 🔒️ Golden hex per `AppCommand` fixture-corpus label — sourced by actually running
    /// `encode_app_command` over `channel_command_fixture_corpus()` (never hand-computed), then
    /// committed here as the drift guard: any future codec change that shifts these bytes fails
    /// this test, forcing a deliberate update of both this table and the TS-side twin (WP-0B).
    fn channel_command_fixture_hex(label: &str) -> &'static str {
        match label {
            "ConfigCommand" => "00010109",
            "Command" => "0101010100",
            "CommandText" => "020102676f",
            "ContextMenu" => "03010101",
            "ArtifactCommand" => "04010101",
            "ApplyEnvelopes" => "050100",
            "LoadDocument" => "060101010102",
            "ReadDocument" => "0701",
            "LoadConfig" => "080101010102",
            "ReadConfig" => "0901",
            "MediaIn" => "0a01017001010102",
            "MediaOut" => "0b0101700101",
            "MediaFingerprint" => "0c010170",
            "PureCommand" => "0d010101010201030104010501060107",
            "LoadChildren" => "0e01010173016301640101",
            "ReadChildren" => "0f01",
            "ReadHistory" => "1001",
            "TransactionPrepareOwner" => "11010174016d0109000000",
            "TransactionPreparePrePlanned" => "110201740000020101020202016c0109",
            "TransactionCommit" => "12030174",
            "TransactionRollback" => "13040174",
            "TransactionUndo" => "14050167",
            "TransactionRedo" => "15060167",
            "OpenArtifactResolve" => "160114732e6361642e63616440312f2a23766965776572000000",
            "OpenArtifactExplicit" => "160214732e6361642e63616440312f2a23656469746f72010363616414732e6361642e63616440312f2a23656469746f72",
            "SetDefaultApp" => "170309732e6361642e6361640131012a010363616414732e6361642e63616440312f2a23656469746f72",
            "ClearDefaultApp" => "180409732e6361642e6361640131012a00",
            "SetMergePolicy" => "190501",
            "ResolveConflict" => "1a060a636f6e666c6963742d3100",
            "ReadConflicts" => "1b07",
            "Presence" => "1c080103020201020109",
            other => panic!("channel_command_fixture_hex: no golden hex registered for label {other:?}"),
        }
    }

    /// @emoji 🔒️ Golden hex per `AppFrame` fixture-corpus label — see
    /// `channel_command_fixture_hex`'s docstring for provenance/drift-guard rationale.
    fn channel_frame_fixture_hex(label: &str) -> &'static str {
        match label {
            "Done" => "0001",
            "Invocation" => "010101010000000109",
            "DocumentChanged" => "0200016f",
            "Document" => "030101010102016f",
            "Config" => "0401010101020163",
            "ConfigChanged" => "0500016f",
            "ContextMenu" => "06010101",
            "Media" => "0701017001010102",
            "MediaFingerprint" => "080101700101",
            "Error" => "090001630107",
            "Emit" => "0a0101010000010200",
            "Draft" => "0b01010101020164",
            "Children" => "0c01010173016301640101",
            "Ephemeral" => "0d02010203040107",
            "HistorySnapshot" => "0e010101",
            "TransactionProposal" => "0f0101700101010164016b00",
            "TransactionPrepared" => "10017401010100",
            "TransactionCommitted" => "1101740165",
            "TransactionRolledBack" => "120174",
            "MergeReport" => "1301010101",
            "Conflicts" => "14000102",
            "UiPatch" => "15010106313a626f64790677696e646f7702010103",
            "UiSnapshotEnd" => "1604",
            other => panic!("channel_frame_fixture_hex: no golden hex registered for label {other:?}"),
        }
    }

    #[test]
    fn app_command_fixture_corpus_matches_golden_hex_and_round_trips() {
        for (label, value) in channel_command_fixture_corpus() {
            let actual = hex_encode(&encode_app_command(&value));
            assert_eq!(actual, channel_command_fixture_hex(label), "{label}'s encoding drifted from its committed golden hex");
            let decoded = decode_app_command(&encode_app_command(&value)).unwrap();
            assert_eq!(decoded, value, "{label} must round-trip");
        }
    }

    #[test]
    fn app_frame_fixture_corpus_matches_golden_hex_and_round_trips() {
        for (label, value) in channel_frame_fixture_corpus() {
            let actual = hex_encode(&encode_app_frame(&value));
            assert_eq!(actual, channel_frame_fixture_hex(label), "{label}'s encoding drifted from its committed golden hex");
            let decoded = decode_app_frame(&encode_app_frame(&value)).unwrap();
            assert_eq!(decoded, value, "{label} must round-trip");
        }
    }

    /// @emoji 📡️ The wire version is owned by `🧫️fixtures/📡️channel/channel-version.json`, not by
    /// either language's constant, so a bump that updates only one host fails here instead of at
    /// runtime — the drift this guard was added for was a live `APP_CHANNEL_VERSION = 8` in
    /// TypeScript against `CHANNEL_VERSION = 10` in Rust. The TS twin asserts the same file.
    #[test]
    fn channel_version_matches_the_shared_cross_language_pin() {
        let json = include_str!("../../../🧫️fixtures/📡️channel/channel-version.json");
        let pin: serde_json::Value = serde_json::from_str(json).expect("channel-version.json must parse");
        let pinned = pin.get("channelVersion").and_then(serde_json::Value::as_u64).expect("channel-version.json must carry channelVersion");
        assert_eq!(u64::from(CHANNEL_VERSION), pinned, "CHANNEL_VERSION and the shared cross-language pin disagree — bump both, plus APP_CHANNEL_VERSION in 🟦️component.ts");
    }

    /// @emoji 🔗️ Cross-language drift guard for the M2 transaction variants (tags 17-21/15-18): the
    /// two JSON files under `🧫️fixtures/📡️channel/` are the single source of truth this codec's TS
    /// twin (`🟦️component.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too —
    /// a change to either side's encode/decode that shifts these bytes fails on exactly one side.
    #[test]
    fn channel_transaction_fixtures_match_shared_cross_language_json_vectors() {
        let command_json = include_str!("../../../🧫️fixtures/📡️channel/app-command-transaction.json");
        let frame_json = include_str!("../../../🧫️fixtures/📡️channel/app-frame-transaction.json");
        let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("app-command-transaction.json must parse");
        let frame_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(frame_json).expect("app-frame-transaction.json must parse");
        assert_eq!(command_vectors.len(), 6, "app-command-transaction.json vector count changed");
        assert_eq!(frame_vectors.len(), 4, "app-frame-transaction.json vector count changed");

        for (label, value) in channel_command_fixture_corpus() {
            if let Some(expected) = command_vectors.get(label) {
                let actual = hex_encode(&encode_app_command(&value));
                assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
            }
        }
        for (label, value) in channel_frame_fixture_corpus() {
            if let Some(expected) = frame_vectors.get(label) {
                let actual = hex_encode(&encode_app_frame(&value));
                assert_eq!(&actual, expected, "AppFrame::{label} drifted from the shared cross-language fixture");
            }
        }
    }

    /// @emoji 🔗️ Cross-language drift guard for the C3 opening variants (tags 22-24): the JSON file
    /// under `🧫️fixtures/📡️channel/` is the single source of truth this codec's TS twin
    /// (`🟦️component.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too — no
    /// `AppFrame` variants were added for opening, so only the command-side vector file exists.
    #[test]
    fn channel_opening_fixtures_match_shared_cross_language_json_vectors() {
        let command_json = include_str!("../../../🧫️fixtures/📡️channel/app-command-opening.json");
        let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("app-command-opening.json must parse");
        assert_eq!(command_vectors.len(), 4, "app-command-opening.json vector count changed");

        for (label, value) in channel_command_fixture_corpus() {
            if let Some(expected) = command_vectors.get(label) {
                let actual = hex_encode(&encode_app_command(&value));
                assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
            }
        }
    }

    /// @emoji 🔗️ Cross-language drift guard for the C8 merge-policy/conflict variants (tags 25-27,
    /// 19-20) plus the extended `Invocation`/`Error` frames: the two JSON files under
    /// `🧫️fixtures/📡️channel/` are the single source of truth this codec's TS twin
    /// (`🟦️component.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too — see
    /// contract-freeze.md §C8 of
    /// `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
    #[test]
    fn channel_merge_fixtures_match_shared_cross_language_json_vectors() {
        let command_json = include_str!("../../../🧫️fixtures/📡️channel/app-command-merge.json");
        let frame_json = include_str!("../../../🧫️fixtures/📡️channel/app-frame-merge.json");
        let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("app-command-merge.json must parse");
        let frame_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(frame_json).expect("app-frame-merge.json must parse");
        assert_eq!(command_vectors.len(), 3, "app-command-merge.json vector count changed");
        assert_eq!(frame_vectors.len(), 4, "app-frame-merge.json vector count changed");

        for (label, value) in channel_command_fixture_corpus() {
            if let Some(expected) = command_vectors.get(label) {
                let actual = hex_encode(&encode_app_command(&value));
                assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
            }
        }
        for (label, value) in channel_frame_fixture_corpus() {
            if let Some(expected) = frame_vectors.get(label) {
                let actual = hex_encode(&encode_app_frame(&value));
                assert_eq!(&actual, expected, "AppFrame::{label} drifted from the shared cross-language fixture");
            }
        }
    }
    //#endregion 🔖️Corpus
}
//#endregion 🧪️Tests
