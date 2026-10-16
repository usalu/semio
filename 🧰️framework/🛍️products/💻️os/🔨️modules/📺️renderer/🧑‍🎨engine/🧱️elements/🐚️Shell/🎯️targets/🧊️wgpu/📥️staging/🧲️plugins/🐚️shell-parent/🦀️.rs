//! 🧲️ `shell-plugins` staging of `shell-parent`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
pub const SHELL_EXTENSION_LEDGER_CAPACITY: usize = 256;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShellExtensionStoreRecord {
    pub extension_id: String,
    pub directory_name: String,
    pub version: String,
    pub label: String,
    #[serde(rename = "extends")]
    pub extends_host: String,
    pub module_url: String,
    pub package_hash: String,
    pub installed_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellExtensionLoadStatus {
    Available,
    Loaded,
    Failed(String),
}

#[derive(Clone)]
pub struct ShellExtensionProjection {
    pub record: ShellExtensionStoreRecord,
    pub enabled: bool,
    pub load_status: ShellExtensionLoadStatus,
    pub program: Option<ProgramBridgeEntry>,
}

//#endregion ShellTypes

