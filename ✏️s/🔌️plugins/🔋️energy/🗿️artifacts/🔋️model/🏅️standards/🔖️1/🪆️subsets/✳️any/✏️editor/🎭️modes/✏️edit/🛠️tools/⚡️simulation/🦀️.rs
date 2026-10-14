//! ⚡️ Edit-mode tool — Energy simulation: a whole-document, read-only algorithm the framework runs as a
//! `ToolRun` (`📋️tool-run-contract.md` §2.4). Start, pause, step, abort and finalize are the framework's
//! reserved actions and chords; the tool only declares its run and supplies the run job
//! (`semio_s_artifact_energy_model_session::EnergySimulationRunJob`).

use crate::session::energy_simulation_run_definition;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ToolDefinition;

//#region 🔖️Constants
pub const TOOL_ID: &str = "energySimulation";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor`.
pub fn definition() -> ToolDefinition {
    ToolDefinition { run: Some(energy_simulation_run_definition()), ..::semio_framework_async::poll::resolve_ready(ToolDefinition::new(TOOL_ID, LocalizedLabel::native("Energy simulation", "Energiesimulation"), "activity")) }
}
//#endregion 🔖️Definition

//#region 🔓️RunMap
/// 🔓️ The live energy simulation run's per-surface map, if there is one. Filters on
/// `tool_id == energySimulation` so another plugin's run in the same document never colours this one.
pub fn surface_energy_from_run(run: Option<&semio_framework_plugin::ToolRunView>) -> Option<crate::editor_results::SurfaceEnergyMap> {
    let run = run.filter(|run| run.tool_id == TOOL_ID)?;
    crate::editor_results::decode_run_payload(run.payload.as_deref()?)
}
//#endregion 🔓️RunMap

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
