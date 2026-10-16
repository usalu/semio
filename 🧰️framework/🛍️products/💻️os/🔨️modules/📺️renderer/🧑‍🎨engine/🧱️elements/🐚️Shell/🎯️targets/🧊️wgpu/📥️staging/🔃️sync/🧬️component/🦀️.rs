//! 🔃️ `shell-sync` component structs: `SyncState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 🔃️ `SyncState`: the document sync backbone, channel, status, link, hub documents, reseed and auto check-in.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `sync`.
pub struct SyncState {
    pub sync_backbone_uri: Option<String>,
    pub sync_card_kind: Option<String>,
    pub sync_card_draft: String,
    pub sync_card_anchor: Option<(f32, f32)>,
    /// 🏛️ Shell-lifetime document-host actor registry, the kernel's `ArtifactHost` on both
    /// targets ([`SHELL_DOCUMENT_TRANSPORTS`] names which transports its actor serves).
    pub document_host: ArtifactHost,
    /// 🧵️ The currently attached document's live actor channel.
    pub sync_channel: Option<ShellSyncChannel>,
    pub next_sync_binding_generation: u64,
    pub sync_terminal_fault: Option<String>,
    /// 🚦️ Latest sync health for the active document's status badge.
    pub sync_status: Option<ArtifactSyncStatus>,
    /// 🔌️ The hub link of the document this shell last attached ([`store_sync::sync::DocumentLink`]'s status):
    /// the Sync card speaks it while it is short and after it turned terminal, until the next open.
    pub sync_link: DocumentLinkStatus,
    /// 📶️ Target-neutral remote state for every document still attached to this shell. Native actor
    /// status events and browser-host publications enter through the same bounded projection.
    pub hub_documents: BTreeMap<String, ShellHubRemoteV1>,
    pub sync_bootstrap_progress: Option<(u64, u64, u32, u32)>,
    /// 🛟️ The canonical-pair reseed a rebootstrapping hub document is waiting for ([`ShellState::advance_sync_reseed`]).
    pub sync_reseed: Option<ShellSyncReseed>,
    /// 📌️ Ms-since-epoch of the last uncommitted-count CHANGE (any direction — mirrors
    /// `AutoCheckinScheduler::notify` resetting its idle timer on every call, not just increases);
    /// `None` whenever nothing is uncommitted.
    pub last_uncommitted_edit_at_ms: Option<i64>,
    /// 📌️ The auto check-in poll's own `pending` latch (`AutoCheckinScheduler`'s twin): set the
    /// instant an auto checkpoint is dispatched, cleared the moment `uncommitted_edit_count` returns
    /// to 0 (a landed checkpoint) — the storm guard `auto_checkin_should_fire` reads.
    pub auto_checkin_pending: bool,
    /// 📌️ §C5 item 6 — set right before ANY checkpoint (auto/explicit/close) THIS shell itself asked
    /// for; cleared once the resulting `history_current_checkpoint_id` change is observed and
    /// `TouchArtifact` fires — distinguishes "a checkpoint we asked for landed" from "the session just
    /// mounted with a pre-existing checkpoint" (mirrors React's `checkpointDispatchedRef`).
    pub checkpoint_dispatched: bool,
    /// 📌️ ticket §C5 item 3 — explicit check-in's own message-prompt dialog: `None` when closed,
    /// `Some(draft)` while open. This hand-painted footer chrome has no generic text-input widget (the
    /// `🟦️Interpreter` pipeline's `UiEvent::TextInput` is deliberately bypassed here, same as
    /// `render_presence_bar`'s own design note) — mirrors the pre-existing `sync_card_kind`/
    /// `sync_card_draft` shell-owned keyboard-routed draft-field idiom instead of inventing a new one.
    pub checkin_dialog_draft: Option<String>,
    /// 📌️ The one hub Check In this shell drives for its mounted hub document: requested the moment
    /// a checkpoint this shell asked for lands, submitted once the document's sync status names an
    /// acknowledged head, then polled to a terminal status (the React twin is the worker's
    /// `driveDocumentCheckIn`). Its status is the footer's hub badge suffix.
    #[cfg(not(target_arch = "wasm32"))]
    pub hub_check_in: Option<ShellHubCheckInV1>,
}
