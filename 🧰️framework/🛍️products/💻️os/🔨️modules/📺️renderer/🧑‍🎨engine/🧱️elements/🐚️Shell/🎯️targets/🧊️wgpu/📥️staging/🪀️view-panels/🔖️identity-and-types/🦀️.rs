//! 🪀️ `shell-view-panels` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🔖️IdentityPure
/// 👥️ Projects only Hub-normalized peers for the shell's currently attached surface.
pub fn presence_peer_rows_for_surface(peers: &[PresencePeer], attached_surface: Option<&str>, target_surface: &str) -> Vec<semio_framework_ui_wgpu_light::presence_bar::PresencePeerRow> {
    if attached_surface != Some(target_surface) {
        return Vec::new();
    }
    peers
        .iter()
        .filter(|peer| peer.surface.as_deref() == Some(target_surface))
        .map(|peer| semio_framework_ui_wgpu_light::presence_bar::PresencePeerRow {
            actor: peer.actor.clone(),
            user_id: peer.user_id.clone(),
            label: peer.label.clone().unwrap_or_else(|| peer.actor.clone()),
            role: match peer.role.as_deref() {
                Some("author") | Some("owner") | Some("member") => Some(semio_framework_ui_wgpu_light::presence_bar::PresenceRole::Author),
                Some("spectator") | Some("viewer") => Some(semio_framework_ui_wgpu_light::presence_bar::PresenceRole::Spectator),
                _ => None,
            },
            connected_at_ms: Some(peer.connected_at_ms),
            color: peer.color,
            is_agent: matches!(peer.principal_kind, Some(store_sync::os_spr::PresencePrincipalKind::Agent)),
            activity: None,
        })
        .collect()
}

//#endregion 🔖️IdentityPure

