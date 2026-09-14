//! 🏗️ Model-mode tool — Reconstruction: the whole photogrammetry pipeline as one mutating framework
//! `ToolRun` (`📋️tool-run-contract.md` §2.4). Start, pause, step, abort and finalize are the framework's
//! reserved actions and chords; the tool only declares its run and the app supplies the run and revalidate
//! jobs (`crate::editor::remodeling::reconstruction_session`).

use crate::editor::remodeling::reconstruction_session::reconstruction_run_definition;
use semio_framework_plugin::{LocalizedLabel, ToolDefinition};

//#region 🔖️Constants
pub const TOOL_ID: &str = "reconstruction";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::remodeling::create_remodeling_app` and listed by
/// every remodeling mode, so a run can start wherever the user is.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(reconstruction_run_definition()), ..semio_framework_plugin::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Reconstruction", "Rekonstruktion"), "remodeling-app")) }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
