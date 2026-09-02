//! ⚡️ Accessible four-tier Energy simulation progress and result window.

use crate::energy_simulation_session::{EnergySimulationProjection, EnergySimulationStatus};
use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{ActionDefinition, ActionKind, BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

pub const WINDOW_KIND_ID: &str = "energy.simulation";
pub const BODY_KEY: &str = "energy.simulation";
pub const START_ACTION_ID: &str = "start-energy-simulation";
pub const CANCEL_ACTION_ID: &str = "cancel-energy-simulation";
pub const RETRY_ACTION_ID: &str = "retry-energy-simulation";
pub const DISCARD_ACTION_ID: &str = "discard-energy-simulation";
pub const ADOPT_ACTION_ID: &str = "adopt-energy-simulation";

fn action(id: &str, en: &str, de: &str) -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::View);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Energy simulation", "Energiesimulation"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "activity".into(),
        options: WindowOptions::default(),
        actions: vec![
            action(START_ACTION_ID, "Start simulation", "Simulation starten"),
            action(CANCEL_ACTION_ID, "Cancel simulation", "Simulation abbrechen"),
            action(RETRY_ACTION_ID, "Retry simulation", "Simulation wiederholen"),
            action(DISCARD_ACTION_ID, "Discard result", "Ergebnis verwerfen"),
            action(ADOPT_ACTION_ID, "Adopt final result", "Endergebnis übernehmen"),
        ],
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: Some(crate::energy_simulation_session::ENERGY_SIMULATION_EVENT_SCHEMA.into()),
        artifact_snapshot_schema: Some(crate::artifacts::model::ENERGY_MODEL_DOCUMENT_SCHEMA.into()),
        input_event_schema: Some(crate::energy_simulation_session::ENERGY_SIMULATION_EVENT_SCHEMA.into()),
        output_schema: Some("SMENERGY/1".into()),
        capabilities: Vec::new(),
    }
}

fn status_text(status: EnergySimulationStatus) -> (&'static str, &'static str, bool) {
    match status {
        EnergySimulationStatus::Idle => ("Idle", "Bereit", false),
        EnergySimulationStatus::Admitting => ("Admitting snapshot", "Snapshot wird zugelassen", true),
        EnergySimulationStatus::Queued => ("Queued", "Eingereiht", true),
        EnergySimulationStatus::Running => ("Running", "Läuft", true),
        EnergySimulationStatus::Cancelled => ("Cancelled", "Abgebrochen", false),
        EnergySimulationStatus::Faulted => ("Faulted", "Fehlgeschlagen", false),
        EnergySimulationStatus::FinalReady => ("Final result ready", "Endergebnis bereit", false),
        EnergySimulationStatus::Adopted => ("Final result adopted", "Endergebnis übernommen", false),
        EnergySimulationStatus::Closing => ("Closing", "Wird geschlossen", true),
    }
}

pub fn render(projection: Option<&EnergySimulationProjection>, german: bool) -> BuiltNode {
    let roots = match projection {
        None => vec![TreeNodeView { id: "energy-simulation-empty".into(), label: if german { "Keine aktive Energiesimulation" } else { "No active Energy simulation" }.into(), children: Vec::new() }],
        Some(projection) => {
            let (en, de, busy) = status_text(projection.status);
            let mut tiers = Vec::with_capacity(4);
            for (index, name) in [
                (0, if german { "Stationäre Schätzung (vorläufig)" } else { "Steady-state estimate (provisional)" }),
                (1, if german { "Auslegungstag (vorläufig)" } else { "Design day (provisional)" }),
                (2, if german { "Grober Zeitschritt (vorläufig)" } else { "Coarse timestep (provisional)" }),
                (3, if german { "Endgültig" } else { "Final" }),
            ] {
                let label = projection.tiers[index]
                    .map_or_else(|| format!("{name}: —"), |tier| format!("{name}: {} / {} · {:.3} kWh · {}", tier.timestep, tier.total_timesteps, tier.facility_electricity_kwh, if german { "Anlagenelektrizität" } else { "facility electricity" }));
                tiers.push(TreeNodeView { id: format!("energy-tier-{index}"), label, children: Vec::new() });
            }
            vec![
                TreeNodeView {
                    id: "energy-simulation-live-region".into(),
                    label: format!("aria-live=polite · role=status · busy={busy} · {}", if german { de } else { en }),
                    children: vec![
                        TreeNodeView {
                            id: "energy-operation".into(),
                            label: format!("{}: {} · {}: {}", if german { "Vorgang" } else { "Operation" }, projection.operation.0, if german { "Generation" } else { "Generation" }, projection.generation.0),
                            children: Vec::new(),
                        },
                        TreeNodeView { id: "energy-checkpoint".into(), label: format!("{}: {}", if german { "Checkpoint" } else { "Checkpoint" }, projection.checkpoint_ready), children: Vec::new() },
                        TreeNodeView { id: "energy-fault".into(), label: format!("{}: {}", if german { "Fehler" } else { "Fault" }, projection.fault_ready), children: Vec::new() },
                        TreeNodeView { id: "energy-final".into(), label: format!("{}: {}", if german { "Endergebnis" } else { "Final result" }, projection.final_ready), children: Vec::new() },
                    ],
                },
                TreeNodeView { id: "energy-quality-tiers".into(), label: if german { "Qualitätsstufen" } else { "Quality tiers" }.into(), children: tiers },
                TreeNodeView {
                    id: "energy-keyboard-help".into(),
                    label: if german { "Tastatur: Starten, Abbrechen, Wiederholen, Verwerfen oder Übernehmen über die Fensteraktionen" } else { "Keyboard: start, cancel, retry, discard, or adopt through the window actions" }.into(),
                    children: Vec::new(),
                },
            ]
        }
    };
    TreeWindowKit::render(&TreeView { roots }).unwrap_or_else(|_| semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data("Energy simulation UI unavailable")).expect("static label is valid"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions_are_localized_and_registered_as_interactive() {
        let definition = definition();
        assert_eq!(definition.actions.len(), 5);
        assert!(definition.actions.iter().all(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated));
        assert_eq!(definition.label, LocalizedLabel::native("Energy simulation", "Energiesimulation"));
    }
}
