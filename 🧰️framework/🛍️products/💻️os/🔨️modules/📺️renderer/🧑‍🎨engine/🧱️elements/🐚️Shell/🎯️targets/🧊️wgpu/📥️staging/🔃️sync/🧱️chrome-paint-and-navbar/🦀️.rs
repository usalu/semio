//! 🔃️ `shell-sync` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🚦️SyncPill
/// 🚦️ The remote half of the pill vocabulary — React's
/// `SyncPillState["remote"]` (`🛠️ShellHelpers/🟦️.tsx`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellSyncRemote {
    Connected,
    Connecting,
    Backoff,
    Detached,
}

/// 🚦️ The `#s-sync-status` pill's state — React's `SyncPillState`, plus this renderer's own
/// `Recovering` arm for the native bootstrap-progress readout React has no twin for (it is produced
/// only by `ShellState::sync_pill`'s native branch and is documented as a declared divergence).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellSyncPill {
    Persisted,
    Pending(usize),
    Remote(ShellSyncRemote),
    Recovering { received_bytes: u64, total_bytes: u64, received_chunks: u32, total_chunks: u32 },
}

//#endregion 🚦️SyncPill
pub use semio_framework_os_renderer_wgpu_contracts::ShellHubRemoteV1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellHubDocumentV1 {
    pub document_key: String,
    pub remote: ShellHubRemoteV1,
}

/// 📶️ The target-neutral hub projection (`🧬️schema/🔗️hub-projection`'s `projection`): the shell's
/// session and link axes and every attached document's remote state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellHubProjectionV1 {
    pub session: HubSessionPresence,
    pub link: HubLink,
    pub documents: Vec<ShellHubDocumentV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellHubSummaryV1 {
    pub state: HubConnectionState,
    pub peer_count: usize,
    pub document_count: usize,
}

pub(crate) fn shell_hub_connection_summary_v1(projection: &ShellHubProjectionV1) -> ShellHubSummaryV1 {
    let document_count = projection.documents.len();
    let statuses = projection
        .documents
        .iter()
        .map(|document| match document.remote {
            ShellHubRemoteV1::Detached => HubDocumentRemote::Detached,
            ShellHubRemoteV1::Connecting => HubDocumentRemote::Connecting,
            ShellHubRemoteV1::Live { peer_count } => HubDocumentRemote::Live { peer_count },
            ShellHubRemoteV1::Backoff { .. } => HubDocumentRemote::Backoff,
        })
        .collect::<Vec<_>>();
    let state = crate::hub_connection::hub_connection_summary(&statuses, projection.session, projection.link).state;
    let peer_count = match state {
        HubConnectionState::Live { peer_count } => peer_count,
        _ => 0,
    };
    ShellHubSummaryV1 { state, peer_count, document_count }
}

pub(crate) fn shell_hub_document_key(document_key: &ArtifactDocumentKey) -> String {
    match document_key {
        ArtifactDocumentKey::Local { document_id } => format!("local:{document_id}"),
        ArtifactDocumentKey::Hub { space_id, document_id } => format!("hub:{space_id}/{document_id}"),
    }
}

/// 📶️ The footer hub badge's text for one fold state, en + de, term for term with React's
/// `HubConnectionIndicator` (`ui.sync.*`).
pub(crate) fn shell_hub_connection_text(state: HubConnectionState, is_de: bool) -> String {
    match state {
        HubConnectionState::Local => shell_chrome_string("hub.local", is_de).to_string(),
        HubConnectionState::SignedOut => shell_chrome_string("hub.signedOut", is_de).to_string(),
        HubConnectionState::Live { peer_count } => format!("{} · {peer_count} {}", shell_chrome_string("hub.live", is_de), if peer_count == 1 { shell_chrome_string("hub.peerOne", is_de) } else { shell_chrome_string("hub.peerMany", is_de) }),
        HubConnectionState::Online => shell_chrome_string("hub.online", is_de).to_string(),
        HubConnectionState::Connecting => shell_chrome_string("hub.connecting", is_de).to_string(),
        HubConnectionState::Reconnecting => shell_chrome_string("hub.reconnecting", is_de).to_string(),
    }
}

//#region 📌️HubCheckIn
/// ⏳️ How long a requested Check In waits for the document's edits to be acknowledged.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const HUB_CHECK_IN_QUIESCENCE_MS: u64 = 15_000;

/// ⏱️ The whole Check In, from request to a terminal status.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const HUB_CHECK_IN_DEADLINE_MS: u64 = 180_000;

/// 📌️ One native hub Check In owner (see `ShellState::hub_check_in`).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub struct ShellHubCheckInV1 {
    pub space_id: String,
    pub document_id: String,
    pub request_id: String,
    pub head: Option<semio_framework_os_kernel::os_directory::EditedArtifactFrontierV1>,
    pub quiescent_by_ms: u64,
    pub deadline_at_ms: u64,
    pub next_poll_at_ms: u64,
    pub submitted: bool,
    pub cancel_requested: bool,
    pub cancel_sent: bool,
    pub status: Option<semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1>,
}

/// 📌️ One sentence for a hub Check In status (the React twin is `checkinStatusText`).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn hub_check_in_status_text(status: &semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1, is_de: bool) -> String {
    use semio_framework_os_kernel::os_directory::{DocumentCheckInPhaseV1 as Phase, DocumentCheckInRefusalV1 as Refusal};
    let key = match (status.phase, status.refusal) {
        (Phase::Ready, _) => "checkIn.ready",
        (Phase::Cancelled, _) => "checkIn.cancelled",
        (Phase::Failed, Some(Refusal::UnknownHead)) => "checkIn.unknownHead",
        (Phase::Failed, Some(Refusal::StaleHead)) => "checkIn.staleHead",
        (Phase::Failed, Some(Refusal::ActiveCheckpointChanged)) => "checkIn.activeCheckpointChanged",
        (Phase::Failed, Some(Refusal::LedgerNotReplayable)) => "checkIn.ledgerNotReplayable",
        (Phase::Failed, Some(Refusal::CodecRefused)) => "checkIn.codecRefused",
        (Phase::Failed, Some(Refusal::AuthorityChanged)) => "checkIn.authorityChanged",
        (Phase::Failed, _) => "checkIn.unavailable",
        (Phase::Accepted | Phase::Materializing | Phase::Publishing, _) => return format!("{} {}/{}", shell_chrome_string("checkIn.running", is_de), status.progress.completed_units, status.progress.total_units),
    };
    shell_chrome_string(key, is_de).to_string()
}

//#endregion 📌️HubCheckIn

