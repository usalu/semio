//! ⏯️ Lift file: 1 item(s), about 4 lines, of `⏯️tool-run/🦀️.rs` that belong to crate `job` (WP3).
//!
//! WP5 owns the source file and cuts the items here (is_tool_run_action_id); WP3 wires this file with one `mod` line in the crate root. Until `job` exists, the source module re-exports the items with `pub use`.
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
use semio_framework_ui_scene::SceneDoc;
use semio_framework_tool_run::TOOL_RUN_ACTION_IDS;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;

/// 🔖️ True for the seven framework-reserved tool run action ids (§2.5).
pub fn is_tool_run_action_id(action: &str) -> bool {
    TOOL_RUN_ACTION_IDS.contains(&action)
}
