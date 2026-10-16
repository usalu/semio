//! 🪝️ `shell-window-actions` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🔀️ChromeParity
/// 🔁️ The wrapping mode cycle `mod+alt+→`/`mod+alt+←` steps through — `None` for a single-mode app
/// (nothing to step to). An active id the app no longer declares steps from the first. Twin of
/// `stepModeIdV1`, pinned by the `modeSteps` fixture rows.
pub fn step_mode_id(mode_ids: &[String], active_mode_id: Option<&str>, step: i32) -> Option<String> {
    if mode_ids.len() < 2 {
        return None;
    }
    let index = active_mode_id.and_then(|id| mode_ids.iter().position(|candidate| candidate == id)).unwrap_or(0) as i32;
    let len = mode_ids.len() as i32;
    Some(mode_ids[(index + step).rem_euclid(len) as usize].clone())
}

//#endregion 🔀️ChromeParity
//#endregion ActionPanelAndUtilities

/// 🔌️ The link status an actor's sync status implies: live relinks, a backoff means the link is short; a
/// terminal link stays terminal until the next open (only the actor's coded terminal message ends a link).
pub fn shell_sync_link_after_status(current: DocumentLinkStatus, remote: &RemoteState) -> DocumentLinkStatus {
    match (current, remote) {
        (DocumentLinkStatus::LinkExpired | DocumentLinkStatus::AccessRevoked, _) => current,
        (_, RemoteState::Live { .. }) => DocumentLinkStatus::Linked,
        (_, RemoteState::Backoff { .. }) => DocumentLinkStatus::Reconnecting,
        _ => current,
    }
}

/// 🔌️ The terminal link a document actor's coded conflict names, if it names one.
pub fn shell_sync_link_terminal(code: &str) -> Option<DocumentLinkStatus> {
    [DocumentLinkStatus::LinkExpired, DocumentLinkStatus::AccessRevoked].into_iter().find(|status| status.code() == code)
}

pub fn shell_hub_remote(remote: &RemoteState) -> ShellHubRemoteV1 {
    match remote {
        RemoteState::Detached => ShellHubRemoteV1::Detached,
        RemoteState::Connecting => ShellHubRemoteV1::Connecting,
        RemoteState::Live { peer_count } => ShellHubRemoteV1::Live { peer_count: *peer_count },
        RemoteState::Backoff { retry_in_ms } => ShellHubRemoteV1::Backoff { retry_in_ms: *retry_in_ms },
    }
}

//#region ActionPanelAndUtilities
/// 🪟️ Resolves an action only from its addressed window kind owner.
pub fn window_action_definition<'a>(app: &'a AppDefinition, window_kind_id: &str, action_id: &str) -> Option<&'a semio_framework::ActionDefinition> {
    semio_framework::window_kind_actions(app, app.window_kinds.iter().find(|kind| kind.id == window_kind_id)?).into_iter().find(|action| action.id == action_id)
}
//#endregion ActionPanelAndUtilities

//#region ActionPanelAndUtilities
//#region 🔀️ChromeParity
/// 👁️✏️ Both surfaces of ONE dialect, in `editor → viewer` focus order, or `None` when the plugin
/// declares fewer than two — the roles group's whole render gate. Twin of `surfaceRoleAppsV1`
/// (`🏛️ShellHost/🔀️surface-switch/🟦️.ts`), pinned by that lane's `group` fixture rows.
pub fn surface_role_apps<'a>(apps: &'a [AppDefinition], dialect: &semio_framework_artifact_reference::ArtifactDialect) -> Option<(&'a AppDefinition, &'a AppDefinition)> {
    let editor = apps.iter().find(|app| &app.dialect == dialect && app.role == semio_framework::manifest::AppRole::Editor)?;
    let viewer = apps.iter().find(|app| &app.dialect == dialect && app.role == semio_framework::manifest::AppRole::Viewer)?;
    Some((editor, viewer))
}
//#endregion 🔀️ChromeParity
//#endregion ActionPanelAndUtilities
