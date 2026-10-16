//! 🔀️ `shell-actions` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities

//#region 🔀️ChromeParity

/// 👁️✏️ The app a role switch would open, or `None` when the role is already mounted or the dialect
/// declares no such sibling. Twin of `roleSwitchTargetV1`.
pub(crate) fn role_switch_target<'a>(apps: &'a [AppDefinition], dialect: &semio_framework_artifact_reference::ArtifactDialect, current_role: semio_framework::manifest::AppRole, requested: semio_framework::manifest::AppRole) -> Option<&'a AppDefinition> {
    if current_role == requested {
        return None;
    }
    let (editor, viewer) = surface_role_apps(apps, dialect)?;
    Some(match requested {
        semio_framework::manifest::AppRole::Editor => editor,
        semio_framework::manifest::AppRole::Viewer => viewer,
    })
}

//#endregion 🔀️ChromeParity
//#endregion ActionPanelAndUtilities

