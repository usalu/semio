//! 🧲️ `shell-plugins` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🔖️IdentityPure
/// 📇️ §5/§6 — a direct manifest scan for the one app a plugin declares for a given
/// `(dialect, role)`, mirroring the React shell's `findDialectApp`.
pub fn find_dialect_app<'a>(program: &'a ProgramBridgeEntry, dialect: &semio_framework_artifact_reference::ArtifactDialect, role: semio_framework::manifest::AppRole) -> Option<&'a AppDefinition> {
    program.manifest.apps.iter().find(|app| &app.dialect == dialect && app.role == role)
}

/// 📌️ The pinned `(plugin_id, app_id)` of one `(dialect coordinate, role)` in the shell's default-app pin store, whose keys are
/// `"<dialect>/<role>"` and values `"<plugin_id> <app_id>"` (the Settings row encoding); `None` when unpinned or malformed.
pub fn pinned_app_for(pins: &std::collections::HashMap<String, String>, dialect_coordinate: &str, role: &str) -> Option<(String, String)> {
    let (plugin_id, app_id) = pins.get(&format!("{dialect_coordinate}/{role}"))?.split_once(' ')?;
    (!plugin_id.is_empty() && !app_id.is_empty()).then(|| (plugin_id.to_string(), app_id.to_string()))
}

/// 🚪️ What an `os.open-artifact` relay does once its owner app is resolved: `Refuse` when no owner was found or installed (the
/// mounted session is NEVER a fallback target), `SeedMounted` when the owner is already the mounted app, `Switch` otherwise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelayOpenPlan {
    Refuse,
    SeedMounted,
    Switch,
}

/// 🚪️ Pure decision of [`RelayOpenPlan`] from the resolved `(plugin_id, app_id)` and the mounted `(plugin_id, app_id)`.
pub fn relay_open_plan(resolved: Option<(&str, &str)>, mounted: Option<(&str, &str)>) -> RelayOpenPlan {
    match resolved {
        None => RelayOpenPlan::Refuse,
        Some(owner) if mounted == Some(owner) => RelayOpenPlan::SeedMounted,
        Some(_) => RelayOpenPlan::Switch,
    }
}

/// 🎬️ The phase a lazy plugin install is in, the one value chrome reads while it runs. `Failed`
/// keeps the message so a refused open explains itself instead of vanishing into the debug log. Both
/// targets run the same phase machine — only the step that actually acquires the module differs
/// ([`ShellState::acquire_plugin_program`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellPluginInstallPhase {
    Resolving,
    Loading,
    Cancelled,
    Failed(String),
}

/// 🎬️ One retained lazy plugin install. Minted by [`ShellState::install_plugin`], cleared by the same
/// call once it settles; `cancel` is the token every step of the install checks, so a cancel that
/// arrives mid-read stops the install at the next step boundary rather than after it has committed.
pub struct ShellPluginInstall {
    pub plugin_id: String,
    pub phase: ShellPluginInstallPhase,
    pub cancel: CancelToken,
}

/// 📁️ The plugin-modules root every resident program's artifact path sits two levels under —
/// `<modules_root>/<plugin_id>/<file>.wasm` is `load_wasm_plugins`'s own layout convention, already
/// relied on by the renderer's extension activation (`🧊️renderer/🦀️.rs`'s `activate_extensions_of`).
/// Reading it back off a resident entry is what lets the shell install a plugin without the renderer
/// having to hand it a second copy of a path it already owns.
#[cfg(not(target_arch = "wasm32"))]
pub fn plugin_modules_root_of(programs: &[ProgramBridgeEntry]) -> Option<std::path::PathBuf> {
    programs.iter().find_map(|entry| entry.wasm_artifact_path().and_then(|path| path.parent()).and_then(|dir| dir.parent()).map(std::path::Path::to_path_buf))
}

//#endregion 🔖️IdentityPure

