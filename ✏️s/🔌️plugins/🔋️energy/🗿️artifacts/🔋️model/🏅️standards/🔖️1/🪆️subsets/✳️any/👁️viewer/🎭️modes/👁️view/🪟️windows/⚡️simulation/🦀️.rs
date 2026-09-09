//! ⚡️ Read-only Energy result window for the viewer surface — the adopted final result only. A
//! viewer declares no actions and owns no locale switch, so every row carries both authored
//! languages side by side (English then German) rather than picking a default.

use crate::energy_simulation_session::EnergySimulationProjection;
use crate::EnergyQualityTier;
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
        output_schema: Some("SMENERGY/1".into()),
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
const TIER_LABELS: [(EnergyQualityTier, &str); 4] = [
    (EnergyQualityTier::SteadyStateEstimate, "Steady-state estimate / Stationäre Schätzung"),
    (EnergyQualityTier::DesignDay, "Design day / Auslegungstag"),
    (EnergyQualityTier::CoarseTimestep, "Coarse timestep / Grober Zeitschritt"),
    (EnergyQualityTier::Final, "Final / Endgültig"),
];

fn leaf(id: impl Into<String>, label: String) -> TreeNodeView {
    TreeNodeView { id: id.into(), label, children: Vec::new() }
}

/// 👁️ Pure read: the adopted projection's per-tier meters plus the run period the model itself
/// carries, so a reader can tell which period the numbers belong to without opening the editor.
pub fn render(projection: Option<&EnergySimulationProjection>, model: &crate::model::Model) -> BuiltNode {
    let run_period = &model.run_period;
    let period = leaf("energy-viewer-run-period", format!("Run period / Simulationszeitraum: {:02}-{:02} → {:02}-{:02}", run_period.start_month, run_period.start_day, run_period.end_month, run_period.end_day));
    let roots = match projection {
        Some(projection) => {
            let tiers = TIER_LABELS
                .iter()
                .enumerate()
                .map(|(index, (_, label))| {
                    let value = projection.tiers[index].map_or_else(|| format!("{label}: —"), |tier| format!("{label}: {} / {} · {:.3} kWh", tier.timestep, tier.total_timesteps, tier.facility_electricity_kwh));
                    leaf(format!("energy-viewer-tier-{index}"), value)
                })
                .collect();
            vec![TreeNodeView { id: "energy-viewer-result-status".into(), label: "role=status · aria-live=polite · Adopted final result / Übernommenes Endergebnis".into(), children: tiers }, period]
        }
        None => vec![
            TreeNodeView { id: "energy-viewer-result-status".into(), label: "role=status · aria-live=polite · No adopted final result · Kein übernommenes Endergebnis".into(), children: Vec::new() },
            TreeNodeView {
                id: "energy-viewer-result-help".into(),
                label: "Start a simulation in the editor and explicitly adopt the final result. · Eine Simulation im Editor starten und das Endergebnis ausdrücklich übernehmen.".into(),
                children: Vec::new(),
            },
            period,
        ],
    };
    TreeWindowKit::render(&TreeView { roots }).unwrap_or_else(|_| semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data("Energy results unavailable")).expect("static label is valid"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
