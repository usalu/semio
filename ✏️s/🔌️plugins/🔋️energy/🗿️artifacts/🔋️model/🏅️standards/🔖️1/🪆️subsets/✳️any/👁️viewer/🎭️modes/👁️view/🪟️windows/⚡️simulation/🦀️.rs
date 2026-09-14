//! ⚡️ Read-only Energy simulation window for the viewer surface. A simulation is a read-only framework
//! tool run of the editor (`energySimulation`): it never writes the document, so a viewer has no result
//! to show beyond the run period the model carries. A viewer declares no actions and owns no locale
//! switch, so every row carries both authored languages side by side (English then German).

use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "energy.simulation.viewer";
pub const BODY_KEY: &str = "energy.simulation.viewer";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Energy results", "Energieergebnisse"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "activity".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: Some(crate::ENERGY_MODEL_DOCUMENT_SCHEMA.into()),
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn leaf(id: impl Into<String>, label: String) -> TreeNodeView {
    TreeNodeView { id: id.into(), label, children: Vec::new() }
}

/// 👁️ Pure read of the model: the run period a simulation in the editor would cover.
pub fn render(model: &crate::model::Model) -> BuiltNode {
    let run_period = &model.run_period;
    let roots = vec![
        TreeNodeView {
            id: "energy-viewer-result-status".into(),
            label: "role=status · aria-live=polite · Simulations run in the editor and never change the document · Simulationen laufen im Editor und ändern das Dokument nie".into(),
            children: Vec::new(),
        },
        leaf("energy-viewer-run-period", format!("Run period / Simulationszeitraum: {:02}-{:02} → {:02}-{:02}", run_period.start_month, run_period.start_day, run_period.end_month, run_period.end_day)),
    ];
    TreeWindowKit::render(&TreeView { roots }).unwrap_or_else(|_| semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data("Energy results unavailable")).expect("static label is valid"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
