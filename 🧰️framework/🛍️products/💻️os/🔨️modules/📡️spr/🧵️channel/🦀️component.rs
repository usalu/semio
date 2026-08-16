//! 🎞️ Protocol app-engine channel: the `AppCommand`/`AppFrame` binary frame taxonomy every app,
//! once turned into a headless engine driven by bidirectional streaming of typed binary commands,
//! exchanges with its client (a UI or a headless runner) — every UI interaction becomes a
//! forwarded `AppCommand`, every engine reaction a returned `AppFrame`. Ticket:
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️01/HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS/`.
//!
//! 🎯️ Mirrors `protocol_wire`'s W5 hand-rolled binary layout exactly: `tag: u8` (the enum
//! variant's declaration order) followed by its fields in declaration order, no per-field tags, no
//! body-length prefix — one frame per channel message. `crate::os_spr::wire::🔖️WireCodec` supplies the
//! primitive codec (`write_varint_u64`/`write_str`/`write_bytes`/`write_bool` and their `read_*`
//! twins); this crate adds only the option/vec/`SectionProbe` combinators and the two enums' tag
//! dispatch below. Unlike `crate::os_spr::wire::ClientFrame`/`ServerFrame`, `AppCommand`/`AppFrame` carry
//! no `Lane` byte — the app-engine channel is a single logical stream, not split into
//! causally-ordered vs. best-effort lanes.

//#region 🔖️Version
/// @emoji 🔢️ The channel wire format's own version, advertised by `AppCommand::Hello` and echoed
/// back by `AppFrame::Welcome` so either side can detect a mismatched build before exchanging any
/// other frame.
pub const CHANNEL_VERSION: u32 = 10;
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

//#region 🔖️SectionProbe
/// @emoji 🔍️ One UI-section cache probe: `AppCommand::RefreshUi` sends a batch of these so the
/// engine can skip re-sending a `AppFrame::UiSection` body the client's `hash` already matches.
#[derive(Clone, Debug, PartialEq)]
pub struct SectionProbe {
    pub kind: u8,
    pub key: String,
    pub hash: Option<u64>,
}
//#endregion 🔖️SectionProbe

//#region 🔖️AppCommand
/// @emoji 📨️ One frame a client (UI or headless runner) sends to the app engine.
#[derive(Clone, Debug, PartialEq)]
pub enum AppCommand {
    Hello {
        channel_version: u32,
        app_id: String,
        actor: String,
        config: Vec<u8>,
    },
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
    RefreshUi {
        seq: u64,
        sections: Vec<SectionProbe>,
        /// 🗣️ Packed `ViewModel` for this refresh — locale/terminology/active-utility must arrive before any
        /// Command, otherwise first-paint `app_labels` resolve against `ViewModel::default()`.
        view_state: Vec<u8>,
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
    AttachBackbone {
        seq: u64,
        uri: String,
    },
    DetachBackbone {
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
    Bye,
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
}
//#endregion 🔖️AppCommand

//#region 🔖️AppFrame
/// @emoji 📬️ One frame the app engine sends to its client.
#[derive(Clone, Debug, PartialEq)]
pub enum AppFrame {
    Welcome { channel_version: u32, instance: u32, manifest: Vec<u8> },
    Done { in_reply_to: u64 },
    Invocation { in_reply_to: u64, output: Vec<u8>, diagnostics: Vec<u8>, ui_scope: Vec<u8>, history_patch: Vec<u8> },
    UiSection { in_reply_to: Option<u64>, kind: u8, key: String, hash: u64, body: Option<Vec<u8>> },
    Effects { in_reply_to: Option<u64>, effects: Vec<Vec<u8>> },
    Events { in_reply_to: Option<u64>, events: Vec<Vec<u8>> },
    DocumentChanged { envelopes: Vec<crate::os_spr::causal::MutationEnvelope>, origin: String },
    Document { in_reply_to: u64, pack: Vec<u8>, spr: Vec<u8>, ops: String },
    Config { in_reply_to: u64, pack: Vec<u8>, spr: Vec<u8>, ops: String },
    ConfigChanged { envelopes: Vec<crate::os_spr::causal::MutationEnvelope>, origin: String },
    ContextMenu { in_reply_to: u64, items: Vec<u8> },
    Media { in_reply_to: u64, port: String, descriptor: Vec<u8>, data: Vec<u8> },
    MediaFingerprint { in_reply_to: u64, port: String, fingerprint: Vec<u8> },
    Error { in_reply_to: Option<u64>, fault: Vec<u8> },
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
    Ephemeral {
        presence: Vec<u8>,
        presence_generation: u64,
        transient_generation: u64,
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
}
//#endregion 🔖️AppFrame

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `tag: u8 | fields...` — see the module-level docstring.
// `crate::os_spr::wire::🔖️WireCodec` supplies the primitives; this region adds the option/vec/
// `SectionProbe` combinators the frame shapes need plus the tag-dispatch match arms.

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

fn write_opt_bytes(out: &mut Vec<u8>, value: &Option<Vec<u8>>) {
    crate::os_spr::write_bool(out, value.is_some());
    if let Some(b) = value {
        crate::os_spr::write_bytes(out, b);
    }
}

fn read_opt_bytes(bytes: &[u8], pos: &mut usize) -> Result<Option<Vec<u8>>, crate::os_spr::ProtocolError> {
    if crate::os_spr::read_bool(bytes, pos)? {
        Ok(Some(crate::os_spr::read_bytes(bytes, pos)?))
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

fn encode_section_probe(probe: &SectionProbe, out: &mut Vec<u8>) {
    out.push(probe.kind);
    crate::os_spr::write_str(out, &probe.key);
    write_opt_u64(out, &probe.hash);
}

fn decode_section_probe(bytes: &[u8], pos: &mut usize) -> Result<SectionProbe, crate::os_spr::ProtocolError> {
    let kind = *bytes.get(*pos).ok_or_else(|| malformed("channel section-probe kind", *pos as u64, "truncated"))?;
    *pos += 1;
    let key = crate::os_spr::read_str(bytes, pos)?;
    let hash = read_opt_u64(bytes, pos)?;
    Ok(SectionProbe { kind, key, hash })
}

fn write_vec_section_probe(out: &mut Vec<u8>, values: &[SectionProbe]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_section_probe(value, out);
    }
}

fn read_vec_section_probe(bytes: &[u8], pos: &mut usize) -> Result<Vec<SectionProbe>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| decode_section_probe(bytes, pos)).collect()
}
//#endregion 🔖️Combinators

/// @emoji 📤️ Encodes one `AppCommand`: `tag u8 | fields`.
pub fn encode_app_command(command: &AppCommand) -> Vec<u8> {
    let mut out = Vec::new();
    match command {
        AppCommand::Hello { channel_version, app_id, actor, config } => {
            out.push(0);
            crate::os_spr::write_varint_u64(&mut out, *channel_version as u64);
            crate::os_spr::write_str(&mut out, app_id);
            crate::os_spr::write_str(&mut out, actor);
            crate::os_spr::write_bytes(&mut out, config);
        }
        AppCommand::ConfigCommand { seq, command } => {
            out.push(1);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, command);
        }
        AppCommand::Command { seq, command, view_state } => {
            out.push(2);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, command);
            crate::os_spr::write_bytes(&mut out, view_state);
        }
        AppCommand::CommandText { seq, line } => {
            out.push(3);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, line);
        }
        AppCommand::RefreshUi { seq, sections, view_state } => {
            out.push(4);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            write_vec_section_probe(&mut out, sections);
            crate::os_spr::write_bytes(&mut out, view_state);
        }
        AppCommand::ContextMenu { seq, request } => {
            out.push(5);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, request);
        }
        AppCommand::ArtifactCommand { seq, command } => {
            out.push(6);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, command);
        }
        AppCommand::ApplyEnvelopes { seq, envelopes } => {
            out.push(7);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            write_vec_envelope(&mut out, envelopes);
        }
        AppCommand::LoadDocument { seq, pack, spr } => {
            out.push(8);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
        }
        AppCommand::ReadDocument { seq } => {
            out.push(9);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::LoadConfig { seq, pack, spr } => {
            out.push(10);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
        }
        AppCommand::ReadConfig { seq } => {
            out.push(11);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::AttachBackbone { seq, uri } => {
            out.push(12);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, uri);
        }
        AppCommand::DetachBackbone { seq } => {
            out.push(13);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::MediaIn { seq, port, descriptor, data } => {
            out.push(14);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, descriptor);
            crate::os_spr::write_bytes(&mut out, data);
        }
        AppCommand::MediaOut { seq, port, request } => {
            out.push(15);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, request);
        }
        AppCommand::MediaFingerprint { seq, port } => {
            out.push(16);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, port);
        }
        AppCommand::Bye => out.push(17),
        AppCommand::PureCommand { seq, command, document, document_spr, config, config_spr, draft, draft_spr } => {
            out.push(18);
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
            out.push(19);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            write_vec_child_pack(&mut out, entries);
        }
        AppCommand::ReadChildren { seq } => {
            out.push(20);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::ReadHistory { seq } => {
            out.push(21);
            crate::os_spr::write_varint_u64(&mut out, *seq);
        }
        AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, label, origin } => {
            out.push(22);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, txn_id);
            crate::os_spr::write_str(&mut out, mutation_id);
            crate::os_spr::write_bytes(&mut out, payload);
            write_vec_bytes(&mut out, prepared_ops);
            crate::os_spr::write_str(&mut out, label);
            crate::os_spr::write_bytes(&mut out, origin);
        }
        AppCommand::TransactionCommit { seq, txn_id } => {
            out.push(23);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, txn_id);
        }
        AppCommand::TransactionRollback { seq, txn_id } => {
            out.push(24);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, txn_id);
        }
        AppCommand::TransactionUndo { seq, group_id } => {
            out.push(25);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, group_id);
        }
        AppCommand::TransactionRedo { seq, group_id } => {
            out.push(26);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, group_id);
        }
        AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => {
            out.push(27);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, artifact_ref);
            out.push(*role);
            crate::os_spr::write_str(&mut out, plugin_id);
            crate::os_spr::write_str(&mut out, app_id);
        }
        AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => {
            out.push(28);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, artifact_kind);
            crate::os_spr::write_str(&mut out, standard);
            crate::os_spr::write_str(&mut out, subset);
            out.push(*role);
            crate::os_spr::write_str(&mut out, plugin_id);
            crate::os_spr::write_str(&mut out, app_id);
        }
        AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => {
            out.push(29);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_str(&mut out, artifact_kind);
            crate::os_spr::write_str(&mut out, standard);
            crate::os_spr::write_str(&mut out, subset);
            out.push(*role);
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
        0 => AppCommand::Hello {
            channel_version: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32,
            app_id: crate::os_spr::read_str(bytes, &mut pos)?,
            actor: crate::os_spr::read_str(bytes, &mut pos)?,
            config: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        1 => AppCommand::ConfigCommand { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)? },
        2 => AppCommand::Command { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)?, view_state: crate::os_spr::read_bytes(bytes, &mut pos)? },
        3 => AppCommand::CommandText { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, line: crate::os_spr::read_str(bytes, &mut pos)? },
        4 => AppCommand::RefreshUi { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, sections: read_vec_section_probe(bytes, &mut pos)?, view_state: crate::os_spr::read_bytes(bytes, &mut pos)? },
        5 => AppCommand::ContextMenu { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, request: crate::os_spr::read_bytes(bytes, &mut pos)? },
        6 => AppCommand::ArtifactCommand { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)? },
        7 => AppCommand::ApplyEnvelopes { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos)? },
        8 => AppCommand::LoadDocument { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)? },
        9 => AppCommand::ReadDocument { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        10 => AppCommand::LoadConfig { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)? },
        11 => AppCommand::ReadConfig { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        12 => AppCommand::AttachBackbone { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, uri: crate::os_spr::read_str(bytes, &mut pos)? },
        13 => AppCommand::DetachBackbone { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        14 => AppCommand::MediaIn { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, descriptor: crate::os_spr::read_bytes(bytes, &mut pos)?, data: crate::os_spr::read_bytes(bytes, &mut pos)? },
        15 => AppCommand::MediaOut { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, request: crate::os_spr::read_bytes(bytes, &mut pos)? },
        16 => AppCommand::MediaFingerprint { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)? },
        17 => AppCommand::Bye,
        18 => AppCommand::PureCommand {
            seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            command: crate::os_spr::read_bytes(bytes, &mut pos)?,
            document: crate::os_spr::read_bytes(bytes, &mut pos)?,
            document_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        19 => AppCommand::LoadChildren { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos)? },
        20 => AppCommand::ReadChildren { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        21 => AppCommand::ReadHistory { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        22 => AppCommand::TransactionPrepare {
            seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            txn_id: crate::os_spr::read_str(bytes, &mut pos)?,
            mutation_id: crate::os_spr::read_str(bytes, &mut pos)?,
            payload: crate::os_spr::read_bytes(bytes, &mut pos)?,
            prepared_ops: read_vec_bytes(bytes, &mut pos)?,
            label: crate::os_spr::read_str(bytes, &mut pos)?,
            origin: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        23 => AppCommand::TransactionCommit { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
        24 => AppCommand::TransactionRollback { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
        25 => AppCommand::TransactionUndo { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, group_id: crate::os_spr::read_str(bytes, &mut pos)? },
        26 => AppCommand::TransactionRedo { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, group_id: crate::os_spr::read_str(bytes, &mut pos)? },
        27 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let artifact_ref = crate::os_spr::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command OpenArtifact.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = crate::os_spr::read_str(bytes, &mut pos)?;
            let app_id = crate::os_spr::read_str(bytes, &mut pos)?;
            AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id }
        }
        28 => {
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
        29 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = crate::os_spr::read_str(bytes, &mut pos)?;
            let standard = crate::os_spr::read_str(bytes, &mut pos)?;
            let subset = crate::os_spr::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ClearDefaultApp.role", pos as u64, "truncated"))?;
            AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role }
        }
        other => return Err(malformed("channel app-command tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(command)
}

/// @emoji 📤️ Encodes one `AppFrame`: `tag u8 | fields`.
pub fn encode_app_frame(frame: &AppFrame) -> Vec<u8> {
    let mut out = Vec::new();
    match frame {
        AppFrame::Welcome { channel_version, instance, manifest } => {
            out.push(0);
            crate::os_spr::write_varint_u64(&mut out, *channel_version as u64);
            crate::os_spr::write_varint_u64(&mut out, *instance as u64);
            crate::os_spr::write_bytes(&mut out, manifest);
        }
        AppFrame::Done { in_reply_to } => {
            out.push(1);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
        }
        AppFrame::Invocation { in_reply_to, output, diagnostics, ui_scope, history_patch } => {
            out.push(2);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, output);
            crate::os_spr::write_bytes(&mut out, diagnostics);
            crate::os_spr::write_bytes(&mut out, ui_scope);
            crate::os_spr::write_bytes(&mut out, history_patch);
        }
        AppFrame::UiSection { in_reply_to, kind, key, hash, body } => {
            out.push(3);
            write_opt_u64(&mut out, in_reply_to);
            out.push(*kind);
            crate::os_spr::write_str(&mut out, key);
            crate::os_spr::write_varint_u64(&mut out, *hash);
            write_opt_bytes(&mut out, body);
        }
        AppFrame::Effects { in_reply_to, effects } => {
            out.push(4);
            write_opt_u64(&mut out, in_reply_to);
            write_vec_bytes(&mut out, effects);
        }
        AppFrame::Events { in_reply_to, events } => {
            out.push(5);
            write_opt_u64(&mut out, in_reply_to);
            write_vec_bytes(&mut out, events);
        }
        AppFrame::DocumentChanged { envelopes, origin } => {
            out.push(6);
            write_vec_envelope(&mut out, envelopes);
            crate::os_spr::write_str(&mut out, origin);
        }
        AppFrame::Document { in_reply_to, pack, spr, ops } => {
            out.push(7);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::Config { in_reply_to, pack, spr, ops } => {
            out.push(8);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::ConfigChanged { envelopes, origin } => {
            out.push(9);
            write_vec_envelope(&mut out, envelopes);
            crate::os_spr::write_str(&mut out, origin);
        }
        AppFrame::ContextMenu { in_reply_to, items } => {
            out.push(10);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, items);
        }
        AppFrame::Media { in_reply_to, port, descriptor, data } => {
            out.push(11);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, descriptor);
            crate::os_spr::write_bytes(&mut out, data);
        }
        AppFrame::MediaFingerprint { in_reply_to, port, fingerprint } => {
            out.push(12);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, fingerprint);
        }
        AppFrame::Error { in_reply_to, fault } => {
            out.push(13);
            write_opt_u64(&mut out, in_reply_to);
            crate::os_spr::write_bytes(&mut out, fault);
        }
        AppFrame::Emit { in_reply_to, document_ops, config_ops, draft_ops, output, diagnostics } => {
            out.push(14);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, document_ops);
            crate::os_spr::write_bytes(&mut out, config_ops);
            crate::os_spr::write_bytes(&mut out, draft_ops);
            crate::os_spr::write_bytes(&mut out, output);
            crate::os_spr::write_bytes(&mut out, diagnostics);
        }
        AppFrame::Draft { in_reply_to, pack, spr, ops } => {
            out.push(15);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::Children { in_reply_to, entries } => {
            out.push(16);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            write_vec_child_pack(&mut out, entries);
        }
        AppFrame::Ephemeral { presence, presence_generation, transient_generation } => {
            out.push(17);
            crate::os_spr::write_bytes(&mut out, presence);
            crate::os_spr::write_varint_u64(&mut out, *presence_generation);
            crate::os_spr::write_varint_u64(&mut out, *transient_generation);
        }
        AppFrame::HistorySnapshot { in_reply_to, history_patch } => {
            out.push(18);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, history_patch);
        }
        AppFrame::TransactionProposal { in_reply_to, proposal_id, local_ops, description, coalesce_key, foreign } => {
            out.push(19);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, proposal_id);
            write_vec_bytes(&mut out, local_ops);
            crate::os_spr::write_str(&mut out, description);
            crate::os_spr::write_str(&mut out, coalesce_key);
            write_vec_bytes(&mut out, foreign);
        }
        AppFrame::TransactionPrepared { txn_id, foreign, rejection } => {
            out.push(20);
            crate::os_spr::write_str(&mut out, txn_id);
            write_vec_bytes(&mut out, foreign);
            crate::os_spr::write_bytes(&mut out, rejection);
        }
        AppFrame::TransactionCommitted { txn_id, edit_id } => {
            out.push(21);
            crate::os_spr::write_str(&mut out, txn_id);
            crate::os_spr::write_str(&mut out, edit_id);
        }
        AppFrame::TransactionRolledBack { txn_id } => {
            out.push(22);
            crate::os_spr::write_str(&mut out, txn_id);
        }
    }
    out
}

/// @emoji 📥️ Decodes one `AppFrame`, the inverse of [`encode_app_frame`].
pub fn decode_app_frame(bytes: &[u8]) -> Result<AppFrame, crate::os_spr::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-frame tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let frame = match tag {
        0 => AppFrame::Welcome { channel_version: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32, instance: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32, manifest: crate::os_spr::read_bytes(bytes, &mut pos)? },
        1 => AppFrame::Done { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        2 => AppFrame::Invocation { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, output: crate::os_spr::read_bytes(bytes, &mut pos)?, diagnostics: crate::os_spr::read_bytes(bytes, &mut pos)?, ui_scope: crate::os_spr::read_bytes(bytes, &mut pos)?, history_patch: crate::os_spr::read_bytes(bytes, &mut pos)? },
        3 => {
            let in_reply_to = read_opt_u64(bytes, &mut pos)?;
            let kind = *bytes.get(pos).ok_or_else(|| malformed("channel ui-section kind", pos as u64, "truncated"))?;
            pos += 1;
            let key = crate::os_spr::read_str(bytes, &mut pos)?;
            let hash = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let body = read_opt_bytes(bytes, &mut pos)?;
            AppFrame::UiSection { in_reply_to, kind, key, hash, body }
        }
        4 => AppFrame::Effects { in_reply_to: read_opt_u64(bytes, &mut pos)?, effects: read_vec_bytes(bytes, &mut pos)? },
        5 => AppFrame::Events { in_reply_to: read_opt_u64(bytes, &mut pos)?, events: read_vec_bytes(bytes, &mut pos)? },
        6 => AppFrame::DocumentChanged { envelopes: read_vec_envelope(bytes, &mut pos)?, origin: crate::os_spr::read_str(bytes, &mut pos)? },
        7 => AppFrame::Document { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)?, ops: crate::os_spr::read_str(bytes, &mut pos)? },
        8 => AppFrame::Config { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)?, ops: crate::os_spr::read_str(bytes, &mut pos)? },
        9 => AppFrame::ConfigChanged { envelopes: read_vec_envelope(bytes, &mut pos)?, origin: crate::os_spr::read_str(bytes, &mut pos)? },
        10 => AppFrame::ContextMenu { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, items: crate::os_spr::read_bytes(bytes, &mut pos)? },
        11 => {
            AppFrame::Media { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, descriptor: crate::os_spr::read_bytes(bytes, &mut pos)?, data: crate::os_spr::read_bytes(bytes, &mut pos)? }
        }
        12 => AppFrame::MediaFingerprint { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, fingerprint: crate::os_spr::read_bytes(bytes, &mut pos)? },
        13 => AppFrame::Error { in_reply_to: read_opt_u64(bytes, &mut pos)?, fault: crate::os_spr::read_bytes(bytes, &mut pos)? },
        14 => AppFrame::Emit {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            document_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            output: crate::os_spr::read_bytes(bytes, &mut pos)?,
            diagnostics: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        15 => AppFrame::Draft {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            pack: crate::os_spr::read_bytes(bytes, &mut pos)?,
            spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            ops: crate::os_spr::read_str(bytes, &mut pos)?,
        },
        16 => AppFrame::Children { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos)? },
        17 => AppFrame::Ephemeral {
            presence: crate::os_spr::read_bytes(bytes, &mut pos)?,
            presence_generation: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            transient_generation: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
        },
        18 => AppFrame::HistorySnapshot { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, history_patch: crate::os_spr::read_bytes(bytes, &mut pos)? },
        19 => AppFrame::TransactionProposal {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            proposal_id: crate::os_spr::read_str(bytes, &mut pos)?,
            local_ops: read_vec_bytes(bytes, &mut pos)?,
            description: crate::os_spr::read_str(bytes, &mut pos)?,
            coalesce_key: crate::os_spr::read_str(bytes, &mut pos)?,
            foreign: read_vec_bytes(bytes, &mut pos)?,
        },
        20 => AppFrame::TransactionPrepared {
            txn_id: crate::os_spr::read_str(bytes, &mut pos)?,
            foreign: read_vec_bytes(bytes, &mut pos)?,
            rejection: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        21 => AppFrame::TransactionCommitted { txn_id: crate::os_spr::read_str(bytes, &mut pos)?, edit_id: crate::os_spr::read_str(bytes, &mut pos)? },
        22 => AppFrame::TransactionRolledBack { txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
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
    fn app_command_hello_round_trips() {
        assert_command_round_trips(&AppCommand::Hello { channel_version: CHANNEL_VERSION, app_id: "app-1".to_string(), actor: "actor-1".to_string(), config: vec![1, 2, 3] });
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
    fn app_command_refresh_ui_round_trips() {
        assert_command_round_trips(&AppCommand::RefreshUi { seq: 4, sections: vec![SectionProbe { kind: 1, key: "outline".to_string(), hash: Some(42) }, SectionProbe { kind: 2, key: "inspector".to_string(), hash: None }], view_state: vec![] });
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
    fn app_command_attach_backbone_round_trips() {
        assert_command_round_trips(&AppCommand::AttachBackbone { seq: 12, uri: "backbone://host/doc".to_string() });
    }

    #[test]
    fn app_command_detach_backbone_round_trips() {
        assert_command_round_trips(&AppCommand::DetachBackbone { seq: 13 });
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
    fn app_command_bye_round_trips() {
        assert_command_round_trips(&AppCommand::Bye);
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
    //#endregion 🔖️AppCommand

    //#region 🔖️AppFrame
    fn assert_frame_round_trips(frame: &AppFrame) {
        let bytes = encode_app_frame(frame);
        let decoded = decode_app_frame(&bytes).expect("decode must succeed");
        assert_eq!(&decoded, frame);
    }

    #[test]
    fn app_frame_welcome_round_trips() {
        assert_frame_round_trips(&AppFrame::Welcome { channel_version: CHANNEL_VERSION, instance: 1, manifest: vec![1, 2, 3] });
    }

    #[test]
    fn app_frame_done_round_trips() {
        assert_frame_round_trips(&AppFrame::Done { in_reply_to: 1 });
    }

    #[test]
    fn app_frame_invocation_round_trips() {
        assert_frame_round_trips(&AppFrame::Invocation { in_reply_to: 2, output: vec![1], diagnostics: vec![2], ui_scope: vec![3], history_patch: vec![4] });
    }

    #[test]
    fn app_frame_ui_section_round_trips_with_and_without_optionals() {
        assert_frame_round_trips(&AppFrame::UiSection { in_reply_to: Some(3), kind: 1, key: "outline".to_string(), hash: 99, body: Some(vec![1, 2]) });
        assert_frame_round_trips(&AppFrame::UiSection { in_reply_to: None, kind: 1, key: "outline".to_string(), hash: 99, body: None });
    }

    #[test]
    fn app_frame_effects_round_trips() {
        assert_frame_round_trips(&AppFrame::Effects { in_reply_to: Some(4), effects: vec![vec![1], vec![2, 2]] });
    }

    #[test]
    fn app_frame_events_round_trips() {
        assert_frame_round_trips(&AppFrame::Events { in_reply_to: None, events: vec![vec![9]] });
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
        assert_frame_round_trips(&AppFrame::Error { in_reply_to: Some(9), fault: b"rejected:bad command".to_vec() });
        assert_frame_round_trips(&AppFrame::Error { in_reply_to: None, fault: b"rejected:bad command".to_vec() });
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
        assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4 });
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
    //#endregion 🔖️AppFrame

    //#region 🔖️SectionProbe
    #[test]
    fn section_probe_round_trips_standalone_and_with_and_without_hash() {
        for probe in [SectionProbe { kind: 3, key: "tree".to_string(), hash: Some(7) }, SectionProbe { kind: 3, key: "tree".to_string(), hash: None }] {
            let mut out = Vec::new();
            encode_section_probe(&probe, &mut out);
            let mut pos = 0;
            assert_eq!(decode_section_probe(&out, &mut pos).unwrap(), probe);
            assert_eq!(pos, out.len());
        }
    }
    //#endregion 🔖️SectionProbe

    //#region 🔖️Codec
    #[test]
    fn encoding_is_deterministic() {
        let command = AppCommand::RefreshUi { seq: 1, sections: vec![SectionProbe { kind: 1, key: "a".to_string(), hash: Some(1) }], view_state: vec![] };
        assert_eq!(encode_app_command(&command), encode_app_command(&command));

        let frame = AppFrame::Error { in_reply_to: Some(1), fault: b"e:m".to_vec() };
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
        let bytes = encode_app_frame(&AppFrame::Error { in_reply_to: Some(1), fault: b"e:message".to_vec() });
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
            // 📌️ A LITERAL version, never `CHANNEL_VERSION`: this corpus pins the CODEC's bytes, and a
            // frame carrying the live constant rewrites its own golden on every wire bump — which is
            // exactly how two unrelated tickets each broke this test. The constant has its own pin in
            // `channel_version_matches_the_shared_cross_language_pin`.
            ("Hello", AppCommand::Hello { channel_version: 1, app_id: "app".to_string(), actor: "actor".to_string(), config: vec![1, 2] }),
            ("ConfigCommand", AppCommand::ConfigCommand { seq: 1, command: vec![9] }),
            ("Command", AppCommand::Command { seq: 1, command: vec![1], view_state: vec![] }),
            ("CommandText", AppCommand::CommandText { seq: 1, line: "go".to_string() }),
            ("RefreshUi", AppCommand::RefreshUi { seq: 1, sections: vec![SectionProbe { kind: 1, key: "a".to_string(), hash: Some(1) }], view_state: vec![] }),
            ("ContextMenu", AppCommand::ContextMenu { seq: 1, request: vec![1] }),
            ("ArtifactCommand", AppCommand::ArtifactCommand { seq: 1, command: vec![1] }),
            ("ApplyEnvelopes", AppCommand::ApplyEnvelopes { seq: 1, envelopes: Vec::new() }),
            ("LoadDocument", AppCommand::LoadDocument { seq: 1, pack: vec![1], spr: vec![2] }),
            ("ReadDocument", AppCommand::ReadDocument { seq: 1 }),
            ("LoadConfig", AppCommand::LoadConfig { seq: 1, pack: vec![1], spr: vec![2] }),
            ("ReadConfig", AppCommand::ReadConfig { seq: 1 }),
            ("AttachBackbone", AppCommand::AttachBackbone { seq: 1, uri: "u".to_string() }),
            ("DetachBackbone", AppCommand::DetachBackbone { seq: 1 }),
            ("MediaIn", AppCommand::MediaIn { seq: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
            ("MediaOut", AppCommand::MediaOut { seq: 1, port: "p".to_string(), request: vec![1] }),
            ("MediaFingerprint", AppCommand::MediaFingerprint { seq: 1, port: "p".to_string() }),
            ("Bye", AppCommand::Bye),
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
        ]
    }

    /// @emoji 🧾️ Named `AppFrame` fixture corpus, one entry per variant.
    fn channel_frame_fixture_corpus() -> Vec<(&'static str, AppFrame)> {
        vec![
            // 📌️ Literal version — see the sibling note on `AppCommand::Hello`'s corpus entry.
            ("Welcome", AppFrame::Welcome { channel_version: 1, instance: 1, manifest: vec![1] }),
            ("Done", AppFrame::Done { in_reply_to: 1 }),
            ("Invocation", AppFrame::Invocation { in_reply_to: 1, output: vec![1], diagnostics: vec![], ui_scope: vec![], history_patch: vec![] }),
            ("UiSection", AppFrame::UiSection { in_reply_to: Some(1), kind: 1, key: "k".to_string(), hash: 1, body: None }),
            ("Effects", AppFrame::Effects { in_reply_to: None, effects: vec![vec![1]] }),
            ("Events", AppFrame::Events { in_reply_to: None, events: vec![] }),
            ("DocumentChanged", AppFrame::DocumentChanged { envelopes: vec![], origin: "o".to_string() }),
            ("Document", AppFrame::Document { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "o".to_string() }),
            ("Config", AppFrame::Config { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "c".to_string() }),
            ("ConfigChanged", AppFrame::ConfigChanged { envelopes: vec![], origin: "o".to_string() }),
            ("ContextMenu", AppFrame::ContextMenu { in_reply_to: 1, items: vec![1] }),
            ("Media", AppFrame::Media { in_reply_to: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
            ("MediaFingerprint", AppFrame::MediaFingerprint { in_reply_to: 1, port: "p".to_string(), fingerprint: vec![1] }),
            ("Error", AppFrame::Error { in_reply_to: None, fault: vec![99] }),
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
            ("Ephemeral", AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4 }),
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
        ]
    }

    /// @emoji 🔒️ Golden hex per `AppCommand` fixture-corpus label — sourced by actually running
    /// `encode_app_command` over `channel_command_fixture_corpus()` (never hand-computed), then
    /// committed here as the drift guard: any future codec change that shifts these bytes fails
    /// this test, forcing a deliberate update of both this table and the TS-side twin (WP-0B).
    fn channel_command_fixture_hex(label: &str) -> &'static str {
        match label {
            "Hello" => "000103617070056163746f72020102",
            "ConfigCommand" => "01010109",
            "Command" => "0201010100",
            "CommandText" => "030102676f",
            "RefreshUi" => "040101010161010100",
            "ContextMenu" => "05010101",
            "ArtifactCommand" => "06010101",
            "ApplyEnvelopes" => "070100",
            "LoadDocument" => "080101010102",
            "ReadDocument" => "0901",
            "LoadConfig" => "0a0101010102",
            "ReadConfig" => "0b01",
            "AttachBackbone" => "0c010175",
            "DetachBackbone" => "0d01",
            "MediaIn" => "0e01017001010102",
            "MediaOut" => "0f0101700101",
            "MediaFingerprint" => "10010170",
            "Bye" => "11",
            "PureCommand" => "12010101010201030104010501060107",
            "LoadChildren" => "1301010173016301640101",
            "ReadChildren" => "1401",
            "ReadHistory" => "1501",
            "TransactionPrepareOwner" => "16010174016d0109000000",
            "TransactionPreparePrePlanned" => "160201740000020101020202016c0109",
            "TransactionCommit" => "17030174",
            "TransactionRollback" => "18040174",
            "TransactionUndo" => "19050167",
            "TransactionRedo" => "1a060167",
            "OpenArtifactResolve" => "1b0114732e6361642e63616440312f2a23766965776572000000",
            "OpenArtifactExplicit" => "1b0214732e6361642e63616440312f2a23656469746f72010363616414732e6361642e63616440312f2a23656469746f72",
            "SetDefaultApp" => "1c0309732e6361642e6361640131012a010363616414732e6361642e63616440312f2a23656469746f72",
            "ClearDefaultApp" => "1d0409732e6361642e6361640131012a00",
            other => panic!("channel_command_fixture_hex: no golden hex registered for label {other:?}"),
        }
    }

    /// @emoji 🔒️ Golden hex per `AppFrame` fixture-corpus label — see
    /// `channel_command_fixture_hex`'s docstring for provenance/drift-guard rationale.
    fn channel_frame_fixture_hex(label: &str) -> &'static str {
        match label {
            "Welcome" => "0001010101",
            "Done" => "0101",
            "Invocation" => "02010101000000",
            "UiSection" => "03010101016b0100",
            "Effects" => "0400010101",
            "Events" => "050000",
            "DocumentChanged" => "0600016f",
            "Document" => "070101010102016f",
            "Config" => "0801010101020163",
            "ConfigChanged" => "0900016f",
            "ContextMenu" => "0a010101",
            "Media" => "0b01017001010102",
            "MediaFingerprint" => "0c0101700101",
            "Error" => "0d000163",
            "Emit" => "0e0101010000010200",
            "Draft" => "0f01010101020164",
            "Children" => "1001010173016301640101",
            "Ephemeral" => "110201020304",
            "HistorySnapshot" => "12010101",
            "TransactionProposal" => "130101700101010164016b00",
            "TransactionPrepared" => "14017401010100",
            "TransactionCommitted" => "1501740165",
            "TransactionRolledBack" => "160174",
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

    /// @emoji 🔗️ Cross-language drift guard for the M2 transaction variants (tags 22-26/19-22): the
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

    /// @emoji 🔗️ Cross-language drift guard for the C3 opening variants (tags 27-29): the JSON file
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
    //#endregion 🔖️Corpus
}
//#endregion 🧪️Tests
