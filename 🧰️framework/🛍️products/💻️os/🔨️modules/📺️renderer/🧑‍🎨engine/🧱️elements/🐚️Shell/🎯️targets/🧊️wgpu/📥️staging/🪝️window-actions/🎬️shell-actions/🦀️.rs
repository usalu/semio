//! 🪝️ `shell-window-actions` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl<'ctx> WindowActionsCtx<'ctx> {
    //#region 🔖️NativeBackboneSync
    //#endregion 🔖️NativeBackboneSync
}

/// 🧭️ Parses a local sync-card uri into the `framework/sync` persistence bindings a
/// document actor opens. `folder://` → the multi-document append-only event log; `file://x.json` → its
/// parent folder's store (single-blob export demoted per the plan). A `remote://` uri names a hub
/// document of its own and is read by [`parse_remote_backbone_uri`]; one missing a part is refused.
pub fn parse_persistence_binding(uri: &str) -> Result<Vec<PersistenceBinding>, String> {
    if uri.starts_with("remote://") {
        return Err(format!("a remote backbone names host, space and document: {uri}"));
    }
    if let Some(path) = uri.strip_prefix("folder://") {
        return Ok(vec![PersistenceBinding::Folder { path: std::path::PathBuf::from(path) }]);
    }
    if let Some(path) = uri.strip_prefix("file://") {
        let parent = std::path::Path::new(path).parent().map(std::path::Path::to_path_buf).unwrap_or_else(|| std::path::PathBuf::from("."));
        return Ok(vec![PersistenceBinding::Folder { path: parent }]);
    }
    Err(format!("unsupported backbone uri: {uri}"))
}

//#endregion ShellActions

