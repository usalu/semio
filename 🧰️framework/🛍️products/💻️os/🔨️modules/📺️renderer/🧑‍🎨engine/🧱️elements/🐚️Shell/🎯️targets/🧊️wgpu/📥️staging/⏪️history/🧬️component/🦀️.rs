//! ⏪️ `shell-history` component structs: `HistoryState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// ⏪️ `HistoryState`: history cursor, entries, time travel, reprojection and the remembered local folders.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `history`.
pub struct HistoryState {
    /// 🧾️ ticket §C5 — this session's live history projection (`history_cursor`/`history_entries`
    /// mirror the React shell's `historyProjection` reducer), fed by `ReadHistory`
    /// (session/document mount, `replace=true`) and `AppFrame::Invocation.history_patch` (every
    /// dispatch response, `replace=false`) — see `fold_history_patch`/`observe_invocation_history`.
    pub history_cursor: u64,
    pub history_entries: BTreeMap<String, semio_framework::kernel::HistoryEntry>,
    /// 🧾️ The most recent checkpoint id `HistoryPatch.currentCheckpointId` reported — compared against
    /// its previous value to detect "a checkpoint landed" (§C5 item 6, `TouchArtifact`).
    pub history_current_checkpoint_id: Option<String>,
    /// ⏪️ The live history-edit session `HistoryPatch.timeTravel` last carried (`None` = no session) — what the
    /// time-travel band, the pane chips and the `ui.timeTravel.*` chords read (see `⏪️time-travel`).
    pub history_time_travel: Option<semio_framework::kernel::HistoryTimeTravel>,
    /// 📡️ The history change `HistoryPatch.reprojection` last said replays before adoption (`None` = none waits) — what the
    /// reprojection band and its live status read (see `⏪️time-travel`, audit W1E-3).
    pub history_reprojection: Option<semio_framework::kernel::HistoryReprojection>,
    /// 🎯️ Where keyboard focus goes once the session edge that named it reaches the screen, and until when its target
    /// may take before the band fallback (`⏪️time-travel` `TimeTravelFocus`).
    pub time_travel_focus: Option<(time_travel::TimeTravelFocus, f64)>,
    /// ⏱️ When the native shell last re-read the history for replay progress (`TIME_TRAVEL_POLL_MS` apart).
    #[cfg(not(target_arch = "wasm32"))]
    pub time_travel_polled_at_ms: f64,
    /// 📁️ This device's remembered folder bindings (`os.config.local-folders`), loaded once and kept current by every
    /// commit (see `📎️local-folders`).
    pub local_folder_bindings: Option<semio_framework_os_config::opening_config::mutations::LocalFolderBindings>,
    /// 🔁️ The document whose remembered folder the direct reattach already tried, so a failure is offered, not retried.
    pub local_folder_reattach_tried: Option<String>,
    /// 🚪️ Whether this build reopens a remembered folder by itself (native) or offers it for the person's gesture
    /// (browser) — `📎️local-folders`'s `LOCAL_FOLDER_DIRECT_REATTACH`.
    pub local_folder_direct_reattach: bool,
    /// 🌐️ The folder- or file-bound document the browser build keeps through the dev host's backbone route.
    #[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
    pub host_route_folder: Option<local_folders::HostRouteFolder>,
}
