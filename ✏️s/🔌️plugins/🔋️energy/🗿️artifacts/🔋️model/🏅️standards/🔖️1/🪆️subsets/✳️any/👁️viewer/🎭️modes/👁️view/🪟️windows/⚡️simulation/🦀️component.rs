//! ⚡️ Read-only Energy simulation result window for the viewer surface.

use crate::energy_simulation_session::EnergySimulationProjection;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

pub const WINDOW_KIND_ID: &str = "energy.simulation.viewer";
pub const BODY_KEY: &str = "energy.simulation.viewer";

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
        artifact_snapshot_schema: Some(crate::artifacts::model::ENERGY_MODEL_DOCUMENT_SCHEMA.into()),
        input_event_schema: None,
        output_schema: Some("SMENERGY/1".into()),
        capabilities: Vec::new(),
    }
}

pub fn render(projection: Option<&EnergySimulationProjection>) -> BuiltNode {
    let roots = if let Some(projection) = projection {
        let mut tiers = Vec::with_capacity(4);
        for (index, label) in ["Steady-state / Stationär", "Design day / Auslegungstag", "Coarse / Grob", "Final / Endgültig"].into_iter().enumerate() {
            let value = projection.tiers[index].map_or_else(|| format!("{label}: —"), |tier| format!("{label}: {} / {} · {:.3} kWh", tier.timestep, tier.total_timesteps, tier.facility_electricity_kwh));
            tiers.push(TreeNodeView { id: format!("energy-viewer-tier-{index}"), label: value, children: Vec::new() });
        }
        vec![TreeNodeView { id: "energy-viewer-result-status".into(), label: "role=status · aria-live=polite · Adopted final result / Übernommenes Endergebnis".into(), children: tiers }]
    } else {
        vec![
            TreeNodeView { id: "energy-viewer-result-status".into(), label: "role=status · aria-live=polite · No adopted final result · Kein übernommenes Endergebnis".into(), children: Vec::new() },
            TreeNodeView {
                id: "energy-viewer-result-help".into(),
                label: "Start a simulation in the editor and explicitly adopt the final result. · Eine Simulation im Editor starten und das Endergebnis ausdrücklich übernehmen.".into(),
                children: Vec::new(),
            },
        ]
    };
    TreeWindowKit::render(&TreeView { roots }).unwrap_or_else(|_| semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data("Energy results unavailable")).expect("static label is valid"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewer_is_structurally_read_only_and_localized() {
        let definition = definition();
        assert!(definition.actions.is_empty());
        assert_eq!(definition.label, LocalizedLabel::native("Energy results", "Energieergebnisse"));
    }
}
