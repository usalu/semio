//! ⚡️ Accessible Energy simulation window: run settings, four-tier progress, cancellation and the
//! live result meters of the mounted `🧵️simulation-session` worker. Every label is authored in
//! English AND German with no default — the active language comes from the session's own
//! `EnergySimulationConfigProjection::locale_de`, which `configure-energy-simulation` sets.

use crate::energy_simulation_session::{EnergySimulationConfigProjection, EnergySimulationProjection, EnergySimulationStatus};
use crate::{EnergyJobStage, EnergyQualityTier};
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "energy.simulation";
pub const BODY_KEY: &str = "energy.simulation";
pub const START_ACTION_ID: &str = "start-energy-simulation";
pub const CANCEL_ACTION_ID: &str = "cancel-energy-simulation";
pub const RETRY_ACTION_ID: &str = "retry-energy-simulation";
pub const DISCARD_ACTION_ID: &str = "discard-energy-simulation";
pub const ADOPT_ACTION_ID: &str = "adopt-energy-simulation";
pub const CONFIGURE_ACTION_ID: &str = "configure-energy-simulation";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🎬️ A window action carrying no execution authority of its own — the surface root's
/// `bounded_first_step_tool_proofs!` + registered factory is what actually classifies it, and
/// `create_energy_model_editor` stamps `Migrated` on every id in `ENERGY_MODEL_RETAINED_TOOL_IDS`.
fn action(id: &str, en: &str, de: &str, args: Vec<ActionArgDef>) -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::View).with_args(args);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}

/// 📅️ The one DOCUMENT verb this window owns. The run period is model data (W-D0 moved it onto
/// `crate::model::Model`), so it is an `ActionKind::Mutation` reduced through the semantic
/// `update-run-period` kind — but it is declared HERE because this window is where it is rendered
/// and edited. Declaring it nowhere would leave it out of `migrated_tool_ids()` while the surface
/// root still proves it, which `AppActionRegistry::validate_tool_job_rows` refuses at app
/// construction with `interactive-job.catalog-incomplete`.
fn run_period_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(crate::editor::model::SET_RUN_PERIOD_ACTION_ID, LocalizedLabel::native("Set run period", "Simulationszeitraum setzen"), ActionKind::Mutation).with_args(vec![
        ActionArgDef::slider("startMonth", LocalizedLabel::native("Start month", "Startmonat"), 1.0, 12.0).required(),
        ActionArgDef::slider("startDay", LocalizedLabel::native("Start day", "Starttag"), 1.0, 31.0).required(),
        ActionArgDef::slider("endMonth", LocalizedLabel::native("End month", "Endmonat"), 1.0, 12.0).required(),
        ActionArgDef::slider("endDay", LocalizedLabel::native("End day", "Endtag"), 1.0, 31.0).required(),
    ]);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}

/// ♻️ The four identity arguments every non-`start` session verb carries, so a cancel/retry/discard/
/// adopt can never be applied to a run other than the one the user is looking at.
fn request_identity_args() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::number("request", LocalizedLabel::native("Request", "Anforderung")).required(),
        ActionArgDef::number("operation", LocalizedLabel::native("Operation", "Vorgang")).required(),
        ActionArgDef::number("generation", LocalizedLabel::native("Generation", "Generation")).required(),
        ActionArgDef::number("configDigest", LocalizedLabel::native("Settings digest", "Einstellungs-Digest")).required(),
    ]
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
            action(START_ACTION_ID, "Start simulation", "Simulation starten", vec![ActionArgDef::number("request", LocalizedLabel::native("Request", "Anforderung")).required()]),
            action(CANCEL_ACTION_ID, "Cancel simulation", "Simulation abbrechen", request_identity_args()),
            action(RETRY_ACTION_ID, "Retry simulation", "Simulation wiederholen", request_identity_args()),
            action(DISCARD_ACTION_ID, "Discard result", "Ergebnis verwerfen", request_identity_args()),
            action(ADOPT_ACTION_ID, "Adopt final result", "Endergebnis übernehmen", request_identity_args()),
            action(
                CONFIGURE_ACTION_ID,
                "Configure run",
                "Lauf konfigurieren",
                vec![
                    ActionArgDef::text("locale", LocalizedLabel::native("Language", "Sprache")).required(),
                    ActionArgDef::slider("zoneTimestepMinutes", LocalizedLabel::native("Zone timestep (min)", "Zonen-Zeitschritt (min)"), 1.0, 60.0).required(),
                    ActionArgDef::slider("systemTimestepMinutes", LocalizedLabel::native("System timestep (min)", "Anlagen-Zeitschritt (min)"), 1.0, 60.0).required(),
                    ActionArgDef::slider("warmupDays", LocalizedLabel::native("Warmup days", "Einschwingtage"), 0.0, 365.0).required(),
                ],
            ),
            run_period_action(),
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
//#endregion 🔖️Definition

//#region 🗣️Language
/// 🗣️ Picks one of the two authored languages. There is no default language: every caller passes the
/// session's explicit `locale_de`, which `configure-energy-simulation` requires as `en` or `de`.
fn say(german: bool, en: &'static str, de: &'static str) -> &'static str {
    if german {
        de
    } else {
        en
    }
}

fn status_text(status: EnergySimulationStatus, german: bool) -> (&'static str, bool) {
    match status {
        EnergySimulationStatus::Idle => (say(german, "Idle", "Bereit"), false),
        EnergySimulationStatus::Admitting => (say(german, "Admitting snapshot", "Snapshot wird zugelassen"), true),
        EnergySimulationStatus::Queued => (say(german, "Queued", "Eingereiht"), true),
        EnergySimulationStatus::Running => (say(german, "Running", "Läuft"), true),
        EnergySimulationStatus::Cancelled => (say(german, "Cancelled", "Abgebrochen"), false),
        EnergySimulationStatus::Faulted => (say(german, "Faulted", "Fehlgeschlagen"), false),
        EnergySimulationStatus::FinalReady => (say(german, "Final result ready", "Endergebnis bereit"), false),
        EnergySimulationStatus::Adopted => (say(german, "Final result adopted", "Endergebnis übernommen"), false),
        EnergySimulationStatus::Closing => (say(german, "Closing", "Wird geschlossen"), true),
    }
}

/// 🧭️ The worker's persistent stage, in both authored languages — the coarse "what is it doing right
/// now" a progress region needs beside the numeric timestep cursor.
fn stage_text(stage: EnergyJobStage, german: bool) -> &'static str {
    match stage {
        EnergyJobStage::Validate => say(german, "Validating the model", "Modell wird geprüft"),
        EnergyJobStage::ResolveWeather => say(german, "Resolving weather", "Wetterdaten werden aufgelöst"),
        EnergyJobStage::Precompute => say(german, "Precomputing", "Vorberechnung"),
        EnergyJobStage::InitializeZones => say(german, "Initializing zones", "Zonen werden initialisiert"),
        EnergyJobStage::InitializeSurfaces => say(german, "Initializing surfaces", "Flächen werden initialisiert"),
        EnergyJobStage::InitializeWarmupHistory => say(german, "Initializing warmup history", "Einschwingverlauf wird initialisiert"),
        EnergyJobStage::WarmupTimestep => say(german, "Warming up", "Einschwingen"),
        EnergyJobStage::WarmupConvergence => say(german, "Checking warmup convergence", "Einschwing-Konvergenz wird geprüft"),
        EnergyJobStage::StartRun => say(german, "Starting the run", "Lauf wird gestartet"),
        EnergyJobStage::RunZoneTimestep => say(german, "Solving zone timesteps", "Zonen-Zeitschritte werden gelöst"),
        EnergyJobStage::AggregateZone => say(german, "Aggregating zones", "Zonen werden aggregiert"),
        EnergyJobStage::AggregateFacility => say(german, "Aggregating the facility", "Anlage wird aggregiert"),
        EnergyJobStage::PublishTimestep => say(german, "Publishing a timestep", "Zeitschritt wird veröffentlicht"),
        EnergyJobStage::Finalize => say(german, "Finalizing", "Abschluss"),
        EnergyJobStage::Size => say(german, "Sizing equipment", "Anlagen werden ausgelegt"),
        EnergyJobStage::FinalizeSummaries => say(german, "Finalizing summaries", "Zusammenfassungen werden erstellt"),
        EnergyJobStage::FinalizeMetrics => say(german, "Finalizing metrics", "Kennzahlen werden erstellt"),
        EnergyJobStage::FinalizeEconomics => say(german, "Finalizing economics", "Wirtschaftlichkeit wird berechnet"),
        EnergyJobStage::BuildResults => say(german, "Building results", "Ergebnisse werden aufgebaut"),
        EnergyJobStage::PublishFinal => say(german, "Publishing the final result", "Endergebnis wird veröffentlicht"),
        EnergyJobStage::EncodeOutput => say(german, "Encoding the output", "Ausgabe wird kodiert"),
        EnergyJobStage::Complete => say(german, "Complete", "Abgeschlossen"),
    }
}

fn tier_text(tier: EnergyQualityTier, german: bool) -> &'static str {
    match tier {
        EnergyQualityTier::SteadyStateEstimate => say(german, "Steady-state estimate (provisional)", "Stationäre Schätzung (vorläufig)"),
        EnergyQualityTier::DesignDay => say(german, "Design day (provisional)", "Auslegungstag (vorläufig)"),
        EnergyQualityTier::CoarseTimestep => say(german, "Coarse timestep (provisional)", "Grober Zeitschritt (vorläufig)"),
        EnergyQualityTier::Final => say(german, "Final", "Endgültig"),
    }
}

const TIERS: [EnergyQualityTier; 4] = [EnergyQualityTier::SteadyStateEstimate, EnergyQualityTier::DesignDay, EnergyQualityTier::CoarseTimestep, EnergyQualityTier::Final];
//#endregion 🗣️Language

//#region 🔖️Render
fn leaf(id: impl Into<String>, label: String) -> TreeNodeView {
    TreeNodeView { id: id.into(), label, children: Vec::new() }
}

/// 🎛️ The editable run settings, rendered as addressable leaves so a keyboard user can read the
/// current values before invoking `configure-energy-simulation`/`set-run-period` on them.
fn settings_nodes(settings: EnergySimulationConfigProjection, model: &crate::model::Model, german: bool) -> TreeNodeView {
    let run_period = &model.run_period;
    TreeNodeView {
        id: "energy-settings".into(),
        label: format!("{} · {CONFIGURE_ACTION_ID} · set-run-period", say(german, "Run settings (editable)", "Laufeinstellungen (bearbeitbar)")),
        children: vec![
            leaf("energy-setting-locale", format!("{}: {}", say(german, "Language", "Sprache"), if settings.locale_de { "de" } else { "en" })),
            leaf("energy-setting-zone-timestep", format!("{}: {} min", say(german, "Zone timestep", "Zonen-Zeitschritt"), settings.zone_timestep_minutes)),
            leaf("energy-setting-system-timestep", format!("{}: {} min", say(german, "System timestep", "Anlagen-Zeitschritt"), settings.system_timestep_minutes)),
            leaf("energy-setting-warmup-days", format!("{}: {}", say(german, "Warmup days", "Einschwingtage"), settings.warmup_days)),
            leaf(
                "energy-setting-run-period",
                format!("{}: {:02}-{:02} → {:02}-{:02}", say(german, "Run period (month-day)", "Simulationszeitraum (Monat-Tag)"), run_period.start_month, run_period.start_day, run_period.end_month, run_period.end_day),
            ),
            leaf("energy-setting-weather", format!("{}: {}", say(german, "Weather file", "Wetterdatei"), say(german, "bound through the model's weather link", "über die Wetterverknüpfung des Modells gebunden"))),
        ],
    }
}

/// 📊️ Per-tier progress AND result: the timestep cursor with its completed percentage, the worker
/// stage, the warmup hour, and the facility electricity meter the tier published.
fn tier_nodes(projection: &EnergySimulationProjection, german: bool) -> TreeNodeView {
    let children = TIERS
        .iter()
        .enumerate()
        .map(|(index, tier)| {
            let name = tier_text(*tier, german);
            let Some(published) = projection.tiers[index] else { return leaf(format!("energy-tier-{index}"), format!("{name}: —")) };
            let percent = if published.total_timesteps == 0 { 0.0 } else { f64::from(published.timestep) * 100.0 / f64::from(published.total_timesteps) };
            TreeNodeView {
                id: format!("energy-tier-{index}"),
                label: format!("{name}: {} / {} ({percent:.1} %)", published.timestep, published.total_timesteps),
                children: vec![
                    leaf(format!("energy-tier-{index}-stage"), format!("{}: {}", say(german, "Stage", "Phase"), stage_text(published.stage, german))),
                    leaf(format!("energy-tier-{index}-warmup"), format!("{}: {} h", say(german, "Warmup hour", "Einschwingstunde"), published.warmup_hour)),
                    leaf(format!("energy-tier-{index}-electricity"), format!("{}: {:.3} kWh", say(german, "Facility electricity", "Anlagenelektrizität"), published.facility_electricity_kwh)),
                ],
            }
        })
        .collect();
    TreeNodeView { id: "energy-quality-tiers".into(), label: say(german, "Quality tiers", "Qualitätsstufen").into(), children }
}

/// 🏁️ The headline result — the highest tier that actually published, so the window shows a real
/// number the moment the steady-state estimate lands rather than only at the end of the run.
fn result_node(projection: &EnergySimulationProjection, german: bool) -> TreeNodeView {
    let best = TIERS.iter().enumerate().rev().find_map(|(index, tier)| projection.tiers[index].map(|published| (*tier, published)));
    let label = match best {
        None => format!("{}: —", say(german, "Facility electricity", "Anlagenelektrizität")),
        Some((tier, published)) => format!("{}: {:.3} kWh · {}", say(german, "Facility electricity", "Anlagenelektrizität"), published.facility_electricity_kwh, tier_text(tier, german)),
    };
    TreeNodeView {
        id: "energy-result".into(),
        label: format!("{} · {label}", say(german, "Result", "Ergebnis")),
        children: vec![
            leaf("energy-result-final", format!("{}: {}", say(german, "Final result available", "Endergebnis verfügbar"), projection.final_ready)),
            leaf("energy-result-adopted", format!("{}: {}", say(german, "Adopted into the document", "Ins Dokument übernommen"), projection.adopted)),
            leaf("energy-result-checkpoint", format!("{}: {}", say(german, "Checkpoint available", "Checkpoint verfügbar"), projection.checkpoint_ready)),
            leaf("energy-result-fault", format!("{}: {}", say(german, "Fault reported", "Fehler gemeldet"), projection.fault_ready)),
        ],
    }
}

/// ⌨️ The keyboard contract, rendered rather than assumed — the same bindings
/// `create_energy_model_editor` registers with `.keybinding(...)`.
fn keyboard_node(german: bool) -> TreeNodeView {
    TreeNodeView {
        id: "energy-keyboard-help".into(),
        label: say(german, "Keyboard", "Tastatur").into(),
        children: vec![
            leaf("energy-keyboard-start", format!("mod+enter — {}", say(german, "start the simulation", "Simulation starten"))),
            leaf("energy-keyboard-cancel", format!("mod+period — {}", say(german, "cancel the running simulation", "laufende Simulation abbrechen"))),
            leaf("energy-keyboard-adopt", format!("mod+shift+enter — {}", say(german, "adopt the final result", "Endergebnis übernehmen"))),
        ],
    }
}

/// ⚡️ The whole window. With no live run the settings are still shown and editable, so a user can
/// configure the run before starting it.
pub fn render(projection: Option<&EnergySimulationProjection>, settings: EnergySimulationConfigProjection, model: &crate::model::Model, german: bool) -> BuiltNode {
    let mut roots = Vec::with_capacity(5);
    match projection {
        None => roots.push(TreeNodeView {
            id: "energy-simulation-live-region".into(),
            label: format!("aria-live=polite · role=status · busy=false · {}", say(german, "No active Energy simulation", "Keine aktive Energiesimulation")),
            children: vec![leaf("energy-simulation-start-hint", say(german, "Start a run with mod+enter or the Start simulation action.", "Einen Lauf mit mod+Eingabe oder der Aktion „Simulation starten“ beginnen.").to_string())],
        }),
        Some(projection) => {
            let (status, busy) = status_text(projection.status, german);
            roots.push(TreeNodeView {
                id: "energy-simulation-live-region".into(),
                label: format!("aria-live=polite · role=status · busy={busy} · {status}"),
                children: vec![
                    leaf("energy-operation", format!("{}: {} · {}: {}", say(german, "Operation", "Vorgang"), projection.operation.0, say(german, "Generation", "Generation"), projection.generation.0)),
                    leaf("energy-request", format!("{}: {} · {}: {}", say(german, "Request", "Anforderung"), projection.request, say(german, "Settings digest", "Einstellungs-Digest"), projection.config_digest)),
                    leaf("energy-sequence", format!("{}: {}", say(german, "Published previews", "Veröffentlichte Vorschauen"), projection.latest_sequence)),
                ],
            });
            roots.push(tier_nodes(projection, german));
            roots.push(result_node(projection, german));
        }
    }
    roots.push(settings_nodes(settings, model, german));
    roots.push(keyboard_node(german));
    TreeWindowKit::render(&TreeView { roots }).unwrap_or_else(|_| semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data("Energy simulation UI unavailable")).expect("static label is valid"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions_are_localized_and_registered_as_interactive() {
        let definition = definition();
        assert_eq!(definition.actions.len(), 7);
        assert!(definition.actions.iter().any(|action| action.id == crate::editor::model::SET_RUN_PERIOD_ACTION_ID), "the run period is edited here, so it must be declared here");
        assert!(definition.actions.iter().all(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated));
        assert_eq!(definition.label, LocalizedLabel::native("Energy simulation", "Energiesimulation"));
        for action in &definition.actions {
            assert!(semio_framework::Terminology::ALL.iter().all(|&terminology| action.label.resolve(terminology, semio_framework::Locale::En) != action.label.resolve(terminology, semio_framework::Locale::De)), "action {} is not really translated", action.id);
        }
    }

    #[test]
    fn an_idle_window_still_renders_the_editable_run_settings() {
        let model = crate::model::Model::default();
        let node = render(None, EnergySimulationConfigProjection::DEFAULT, &model, false);
        let text = format!("{node:?}");
        assert!(text.contains("energy-settings"), "the settings block must render without a live run");
        assert!(text.contains("energy-setting-run-period"));
        assert!(text.contains("energy-keyboard-start"));
    }

    #[test]
    fn both_authored_languages_produce_different_text() {
        let model = crate::model::Model::default();
        let english = format!("{:?}", render(None, EnergySimulationConfigProjection::DEFAULT, &model, false));
        let german = format!("{:?}", render(None, EnergySimulationConfigProjection::DEFAULT, &model, true));
        assert_ne!(english, german);
        assert!(german.contains("Laufeinstellungen"));
        assert!(english.contains("Run settings"));
    }

    #[test]
    fn every_worker_stage_is_authored_in_both_languages() {
        let stages = [
            EnergyJobStage::Validate,
            EnergyJobStage::ResolveWeather,
            EnergyJobStage::Precompute,
            EnergyJobStage::InitializeZones,
            EnergyJobStage::InitializeSurfaces,
            EnergyJobStage::InitializeWarmupHistory,
            EnergyJobStage::WarmupTimestep,
            EnergyJobStage::WarmupConvergence,
            EnergyJobStage::StartRun,
            EnergyJobStage::RunZoneTimestep,
            EnergyJobStage::AggregateZone,
            EnergyJobStage::AggregateFacility,
            EnergyJobStage::PublishTimestep,
            EnergyJobStage::Finalize,
            EnergyJobStage::Size,
            EnergyJobStage::FinalizeSummaries,
            EnergyJobStage::FinalizeMetrics,
            EnergyJobStage::FinalizeEconomics,
            EnergyJobStage::BuildResults,
            EnergyJobStage::PublishFinal,
            EnergyJobStage::EncodeOutput,
            EnergyJobStage::Complete,
        ];
        for stage in stages {
            assert_ne!(stage_text(stage, false), stage_text(stage, true), "stage {stage:?} is not really translated");
        }
    }
}
//#endregion 🧪️Tests
