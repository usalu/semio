//! 🎬️ `AppFrame`: every engine reaction returned to the client.

use super::{AppDocumentIdentity, ChildHeadPackEntry, ChildPackEntry, DocumentArchiveLoadStatus, DocumentArchivePack, MediaExportHandleWire, MediaExportStateWire, WindowConfigPackEntry};

//#region 🔖️AppFrame
/// 📬️ One frame the app engine sends to its client.
#[derive(Clone, Debug, PartialEq)]
pub enum AppFrame {
    Done {
        in_reply_to: u64,
    },
    /// 🧾 `messages` is one packed `DispatchReport` for this dispatch — see contract-freeze.md
    /// §C8. `mutations` and `inverse_group` (CHANNEL_VERSION 14 trailing additions) retain the
    /// exact packed invocation result needed by the ordinary mutation/undo transport.
    Invocation {
        in_reply_to: u64,
        output: Vec<u8>,
        diagnostics: Vec<u8>,
        ui_scope: Vec<u8>,
        history_patch: Vec<u8>,
        messages: Vec<u8>,
        mutations: Vec<u8>,
        inverse_group: Vec<u8>,
    },
    DocumentChanged {
        envelopes: Vec<protocol::causal::MutationEnvelope>,
        origin: String,
    },
    Document {
        in_reply_to: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
        ops: String,
    },
    Config {
        in_reply_to: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
        ops: String,
    },
    /// 🪟️ Every exact concrete window config envelope owned by the app instance.
    WindowConfigs {
        in_reply_to: u64,
        entries: Vec<WindowConfigPackEntry>,
    },
    ConfigChanged {
        envelopes: Vec<protocol::causal::MutationEnvelope>,
        origin: String,
    },
    ContextMenu {
        in_reply_to: u64,
        items: Vec<u8>,
    },
    Media {
        in_reply_to: u64,
        port: String,
        descriptor: Vec<u8>,
        data: Vec<u8>,
    },
    MediaFingerprint {
        in_reply_to: u64,
        port: String,
        fingerprint: Vec<u8>,
    },
    /// 🧾 `report` (CHANNEL_VERSION 11 trailing addition) is one packed `DispatchReport` of the
    /// rejected dispatch, accompanying a `Fault.code == "app.command.rejected"` — see
    /// contract-freeze.md §C8/C9.
    Error {
        in_reply_to: Option<u64>,
        fault: Vec<u8>,
        report: Vec<u8>,
    },
    /// 📤️ Guest Emit bytes for host-applied store authority (document/config/draft op packs).
    Emit {
        in_reply_to: u64,
        document_ops: Vec<u8>,
        config_ops: Vec<u8>,
        draft_ops: Vec<u8>,
        output: Vec<u8>,
        diagnostics: Vec<u8>,
        /// 🧩️ The owned-child op groups of a previewed agent gesture (CHANNEL_VERSION 18 trailing addition): the
        /// composing guest's `ChildEmit` list as one wire pack, handed back unmodified on
        /// `AppCommand::TransactionPrepare.prepared_child_ops`; empty when the gesture touches no owned child.
        child_ops: Vec<u8>,
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
    /// `tool_run` (trailing field) is `encode_presence_tool_run` of the instance's tool run summary — empty
    /// bytes without a run (`📋️tool-run-contract.md` §3.4). `history_edit` (trailing field) is
    /// `encode_presence_history_edit` of the instance's open history edit — empty bytes while none is open.
    Ephemeral {
        presence: Vec<u8>,
        presence_generation: u64,
        transient_generation: u64,
        interaction: Vec<u8>,
        tool_run: Vec<u8>,
        history_edit: Vec<u8>,
    },
    /// 🧾️ Full history patch for initial host projection and gap recovery.
    HistorySnapshot {
        in_reply_to: u64,
        history_patch: Vec<u8>,
    },
    /// 📣️ A guest's dispatch touched a foreign artifact — the host mints `txn_id`, resolves each
    /// opaque `ForeignStep` in `foreign` (one `store::pack_rt::encode_wire_value`-encoded serde
    /// form per element; not decoded at this layer), and drives the transaction protocol (contract
    /// freeze §5). CHANNEL_VERSION 9 wire addition; CHANNEL_VERSION 21 dropped its never-read description and
    /// coalesce key (design §20.1/§20.7).
    TransactionProposal {
        in_reply_to: u64,
        proposal_id: String,
        local_ops: Vec<Vec<u8>>,
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
    /// 🏁️ One mounted typed operation reached its terminal result. Unsolicited and NEVER a reply:
    /// the command that started the operation already returned its "started" `Invocation` long
    /// before the retained job finished, so no `in_reply_to` sequence exists to correlate against —
    /// `operation` (the host-allocated `semio_framework_job::OperationId`) is the correlation key
    /// instead. `revision` is the artifact store's content revision at completion; `ui_scope` is the
    /// operation's final `kernel::UiDirtyScope` and `history_patch` its command-log delta (empty
    /// bytes when the operation recorded nothing), both `store::pack_rt::encode_wire_value`-encoded
    /// exactly like `Invocation`'s same-named fields. CHANNEL_VERSION 15 wire addition.
    OperationCompleted {
        operation: u64,
        revision: u64,
        ui_scope: Vec<u8>,
        history_patch: Vec<u8>,
    },
    /// 📃️ ACK-owned local-only pages are independent of ordinary command outcomes.
    LocalInteractionQuery {
        reply: protocol::LocalInteractionQueryReply,
    },
    /// 🗃️ Complete generation-fenced recursive document archive. CHANNEL_VERSION 16 wire addition.
    DocumentArchive {
        in_reply_to: u64,
        archive: DocumentArchivePack,
    },
    /// 📊️ Progress or terminal state for one ACK-owned archive load.
    DocumentArchiveLoad {
        in_reply_to: u64,
        status: DocumentArchiveLoadStatus,
    },
    MediaExportSubmitted {
        in_reply_to: u64,
        handle: MediaExportHandleWire,
    },
    MediaExportStatus {
        in_reply_to: u64,
        handle: MediaExportHandleWire,
        state: MediaExportStateWire,
        applied_progress: u64,
        checkpoint_available: bool,
        mime_type: String,
        total_bytes: u64,
        detail: String,
    },
    MediaExportChunk {
        in_reply_to: u64,
        handle: MediaExportHandleWire,
        data: Vec<u8>,
        terminal: bool,
    },
    DocumentIdentity { in_reply_to: u64, identity: AppDocumentIdentity },
    /// 🪆️ Reply to `AppCommand::ReadChildHeads`: one entry per owned child. CHANNEL_VERSION 21 wire addition.
    ChildHeads { in_reply_to: u64, entries: Vec<ChildHeadPackEntry> },
}
//#endregion 🔖️AppFrame
