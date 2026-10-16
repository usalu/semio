//! 📨️ `AppCommand`: every UI interaction forwarded to the headless engine.

use super::{ChildPackEntry, DocumentArchivePack, MediaExportHandleWire, PresenceRosterWire, WindowConfigPackEntry};

//#region 🔖️AppCommand
/// 📨️ One frame a client (UI or headless runner) sends to the app engine.
#[derive(Debug, PartialEq)]
#[expect(clippy::large_enum_variant, reason = "Presence commands transfer the admitted fixed roster slots inline without another channel allocation.")]
pub enum AppCommand {
    ConfigCommand {
        seq: u64,
        command: Vec<u8>,
    },
    Command {
        seq: u64,
        command: Vec<u8>,
        /// 🗣️ Packed `ViewModel` (see `crate::os_pack::pack_rt`) the client wants this command evaluated against.
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
        envelopes: Vec<protocol::causal::MutationEnvelope>,
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
    /// 🪟️ Restores one exact concrete window's persisted-local config envelope.
    LoadWindowConfig {
        seq: u64,
        entry: WindowConfigPackEntry,
    },
    /// 🪟️ Reads every exact concrete window config envelope owned by this app instance.
    ReadWindowConfigs {
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
    /// 🧾 Host-authoritative command: the guest evaluates `command` and returns `AppFrame::Emit` ops only (host
    /// applies). CHANNEL_VERSION 5 wire addition; CHANNEL_VERSION 21 reduced its lanes to one `head` (design §20.8).
    PureCommand {
        seq: u64,
        command: Vec<u8>,
        /// 📸️ The HEAD snapshot pack a stateless evaluation runs against with an empty history — never a fold of `.spr`
        /// history; empty evaluates against the instance's live document.
        head: Vec<u8>,
    },
    /// 🧸️ Restores a composing document's owned children into the engine, each as its own live
    /// store. Sent after the parent's document is loaded (the parent must exist before its children can be adopted).
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
    /// (`prepared_ops`+`origin` set, `mutation_id` empty); see contract-freeze.md §2 of
    /// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/`.
    /// CHANNEL_VERSION 9 wire addition; CHANNEL_VERSION 21 dropped the host-written label (a member's history row is
    /// named by its own leaves, design §20.6).
    TransactionPrepare {
        seq: u64,
        txn_id: String,
        mutation_id: String,
        payload: Vec<u8>,
        prepared_ops: Vec<Vec<u8>>,
        origin: Vec<u8>,
        /// 🧩️ The pre-planned form's owned-child op groups (CHANNEL_VERSION 18 trailing addition): the composing
        /// guest's own `ChildEmit` list as one wire pack, exactly as its `AppFrame::Emit.child_ops` preview produced
        /// it; empty when the gesture touches no owned child. The channel carries it opaque.
        prepared_child_ops: Vec<u8>,
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
    /// contract-freeze.md §3 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`.
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
    /// contract-freeze.md §C3/C8 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
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
    /// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`).
    /// `own_color` is this actor's hub-assigned palette index (`None` for a folder-only session with
    /// no hub); `peers` are `encode_presence_peer` blobs, the whole roster with the wrapper's own
    /// actor already dropped. Reply is a plain `AppFrame::Done`. CHANNEL_VERSION 12 wire addition.
    Presence {
        seq: u64,
        own_color: Option<u8>,
        peers: PresenceRosterWire,
    },
    /// 🏠️ Fixed request/token commands; page data travels only on the reply lane.
    LocalInteractionQuery {
        seq: u64,
        command: protocol::LocalInteractionQueryCommand,
    },
    /// 🗃️ Restores one root envelope and its complete recursive owned-member closure through one
    /// retained publication transaction. CHANNEL_VERSION 16 wire addition.
    LoadDocumentArchive {
        seq: u64,
        archive: DocumentArchivePack,
    },
    /// 🗃️ Reads one generation-fenced root envelope and complete recursive owned-member closure.
    /// CHANNEL_VERSION 16 wire addition.
    ReadDocumentArchive {
        seq: u64,
    },
    /// 📊️ Reads one retained archive operation without transferring or releasing its owner.
    PollDocumentArchiveLoad {
        seq: u64,
        operation: u64,
    },
    /// 🛑️ Marks one retained archive operation cancelled; maintenance owns cleanup.
    CancelDocumentArchiveLoad {
        seq: u64,
        operation: u64,
    },
    /// 📨️ Releases one terminal archive operation after its exact result was observed.
    AcknowledgeDocumentArchiveLoad {
        seq: u64,
        operation: u64,
    },
    SubmitMediaExport {
        seq: u64,
        port: String,
        expected_parent_document_id: String,
        expected_base_revision: u64,
    },
    PollMediaExport {
        seq: u64,
        handle: MediaExportHandleWire,
    },
    CancelMediaExport {
        seq: u64,
        handle: MediaExportHandleWire,
    },
    TakeMediaExportChunk {
        seq: u64,
        handle: MediaExportHandleWire,
    },
    ReadDocumentIdentity { seq: u64 },
    /// 🪆️ Reads every owned child's CURRENT head snapshot pack (nested members included) for a reader that composes
    /// parent + children on read (design §20.15). Reply `AppFrame::ChildHeads`. CHANNEL_VERSION 21 wire addition.
    ReadChildHeads { seq: u64 },
    /// 🧲️ Merges an archive of the document this program already shows: the guest ingests the events its stores lack and
    /// never replaces them or adopts the archive's viewed head (design §22.22). Same `archive` encoding and the same poll /
    /// cancel / acknowledge exchange as `LoadDocumentArchive`; an archive of another document is refused with
    /// `plugin.document-load.other-document`. CHANNEL_VERSION 22 wire addition.
    MergeDocumentArchive {
        seq: u64,
        archive: DocumentArchivePack,
    },
}
//#endregion 🔖️AppCommand
