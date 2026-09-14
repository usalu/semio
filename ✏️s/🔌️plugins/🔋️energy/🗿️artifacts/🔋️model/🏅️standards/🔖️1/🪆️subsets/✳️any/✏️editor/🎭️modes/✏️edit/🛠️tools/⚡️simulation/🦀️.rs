//! ⚡️ Edit-mode tool — Energy simulation: a whole-document, read-only algorithm the framework runs as a
//! `ToolRun` (`📋️tool-run-contract.md` §2.4). Start, pause, step, abort and finalize are the framework's
//! reserved actions and chords; the tool only declares its run and supplies the run job
//! (`crate::energy_simulation_session::EnergySimulationRunJob`).

use crate::energy_simulation_session::energy_simulation_run_definition;
use semio_framework_plugin::{LocalizedLabel, ToolDefinition};

//#region 🔖️Constants
pub const TOOL_ID: &str = "energySimulation";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor`.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(energy_simulation_run_definition()), ..semio_framework_plugin::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Energy simulation", "Energiesimulation"), "activity")) }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
