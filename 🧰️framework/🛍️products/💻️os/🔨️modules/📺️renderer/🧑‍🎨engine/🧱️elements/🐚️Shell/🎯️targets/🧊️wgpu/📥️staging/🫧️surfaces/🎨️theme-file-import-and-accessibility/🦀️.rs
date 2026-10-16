//! 🫧️ `shell-surfaces` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 📤️FileOpenImport
/// 📤️ One file a picker handed back. The NAME is half the payload, not decoration: every import leaf
/// in the repo resolves the file's format from its extension (`document_io::import_row_for_name`),
/// so a shell that hands over contents alone can only guess.
pub struct OpenedFile {
    pub name: String,
    pub contents: String,
}

/// 📥️ The files one picker answered, with the verb their chunks go to.
pub(crate) struct PickedImport {
    pub controller_id: String,
    pub import_action: String,
    pub args: Option<semio_framework::DslValue>,
    pub opened: Vec<OpenedFile>,
    pub multiple: bool,
}

//#endregion 📤️FileOpenImport

