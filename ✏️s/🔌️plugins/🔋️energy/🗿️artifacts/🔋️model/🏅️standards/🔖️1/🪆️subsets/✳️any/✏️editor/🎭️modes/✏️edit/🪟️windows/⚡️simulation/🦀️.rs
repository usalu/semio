//! ⚡️ Accessible Energy simulation window: the editable run settings and the state of the energy
//! simulation tool run as the framework ledger reports it (`ArtifactView::tool_run`). Progress, the
//! step log with the published quality tiers and the start/pause/step/abort/finalize buttons are the
//! framework ToolRun panel's (`📋️tool-run-contract.md` §2.5, §4.1); this window never keeps run state of
//! its own. Every plugin label is authored in English and German with no default; framework labels come
//! from the framework table for the OS-owned locale.

use crate::editor::model::config::EnergyModelConfig;
use crate::editor::model::modes::edit::tools;
use crate::editor::model::results::ResultField;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, BuiltNode, InteractiveJobClassification, Locale, LocalizedLabel, SurfaceKind, ToolRunView, WindowKindDefinition, WindowOptions};
use semio_framework_tool_run::{ToolRunAction, ToolRunState};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "energy.simulation";
pub const BODY_KEY: &str = "energy.simulation";
pub const SET_SETTINGS_ACTION_ID: &str = "set-simulation-settings";
pub const SET_RESULT_FIELD_ACTION_ID: &str = "set-result-field";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// ⚙️ The config verb this window owns: it publishes the run settings into the config store, whose
/// generation change the framework applies to a live run as its `reconfigure: restart` policy.
fn settings_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(SET_SETTINGS_ACTION_ID, LocalizedLabel::native("Set simulation settings", "Simulationseinstellungen setzen"), ActionKind::View).with_args(vec![
        ActionArgDef::slider("zoneTimestepMinutes", LocalizedLabel::native("Zone timestep (min)", "Zonen-Zeitschritt (min)"), 1.0, 60.0).required(),
        ActionArgDef::slider("systemTimestepMinutes", LocalizedLabel::native("System timestep (min)", "Anlagen-Zeitschritt (min)"), 1.0, 60.0).required(),
        ActionArgDef::slider("warmupDays", LocalizedLabel::native("Warmup days", "Einschwingtage"), 0.0, 365.0).required(),
    ]);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}

/// 🎨️ The second config verb this window owns: which published per-surface field the 3d model window
/// colours by. Shaped exactly like `set-simulation-settings` — an `ActionKind::View` reducing to a
/// config-store mutation — but it touches no pointer in `ENERGY_SIMULATION_RUN_SETTINGS`, so a
/// recolour never restarts a live run.
fn result_field_action() -> ActionDefinition {
    let options = crate::editor::model::results::ResultField::ALL
        .iter()
        .map(|field| {
            ActionArgOption::new(
                field.id(),
                match field {
                    ResultField::ConductionLoss => LocalizedLabel::native("Conduction loss", "Transmissionsverlust"),
                    ResultField::ConductionGain => LocalizedLabel::native("Conduction gain", "Transmissionsgewinn"),
                    ResultField::SolarTransmitted => LocalizedLabel::native("Solar transmitted", "Solare Transmission"),
                    ResultField::SolarAbsorbed => LocalizedLabel::native("Solar absorbed", "Solare Absorption"),
                },
            )
        })
        .collect();
    let mut action = ActionDefinition::bounded_catalog(SET_RESULT_FIELD_ACTION_ID, LocalizedLabel::native("Set result field", "Ergebnisfeld setzen"), ActionKind::View)
        .with_args(vec![ActionArgDef::select("field", LocalizedLabel::native("Coloured by", "Eingefärbt nach"), options).required()]);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}

/// 📅️ The one DOCUMENT verb this window owns. The run period is model data, so it is an
/// `ActionKind::Mutation` reduced through the semantic `update-run-period` kind — declared HERE because
/// this window is where it is rendered and edited.
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

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Energy simulation", "Energiesimulation"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "activity".into(),
        options: WindowOptions::default(),
        actions: vec![settings_action(), result_field_action(), run_period_action()],
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: Some(<EnergyModelConfig as store::ArtifactDsl>::envelope_id().into()),
        artifact_snapshot_schema: Some(crate::ENERGY_MODEL_DOCUMENT_SCHEMA.into()),
        input_event_schema: None,
        output_schema: Some(crate::energy_simulation_session::ENERGY_SIMULATION_RUN_SCHEMA.into()),
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🗣️Language
/// 🗣️ Picks one of the two authored languages for the OS-owned locale.
fn say(locale: Locale, en: &'static str, de: &'static str) -> &'static str {
    if locale == Locale::De {
        de
    } else {
        en
    }
}

/// 🏁️ What the run's state means for the simulation result, beside the framework state label.
fn result_text(state: ToolRunState, locale: Locale) -> &'static str {
    match state {
        ToolRunState::Starting | ToolRunState::Running | ToolRunState::Paused => say(locale, "Quality tiers publish into the run steps as they complete", "Qualitätsstufen erscheinen nach Abschluss in den Laufschritten"),
        ToolRunState::Complete => say(locale, "The final result is ready to finalize", "Das Endergebnis ist bereit zum Abschließen"),
        ToolRunState::Finalizing | ToolRunState::Finalized => say(locale, "The final result is accepted; the simulation never writes the document", "Das Endergebnis ist übernommen; die Simulation schreibt nie ins Dokument"),
        ToolRunState::Aborting | ToolRunState::Aborted | ToolRunState::Faulted => say(locale, "No result was accepted", "Kein Ergebnis wurde übernommen"),
    }
}
//#endregion 🗣️Language

//#region 🔖️Render
fn leaf(id: impl Into<String>, label: String) -> TreeNodeView {
    TreeNodeView { id: id.into(), label, children: Vec::new() }
}

/// 🎛️ The editable run settings, rendered as addressable leaves so a keyboard user can read the current
/// values before invoking `set-simulation-settings`/`set-run-period` on them.
fn settings_nodes(settings: &EnergyModelConfig, model: &crate::model::Model, locale: Locale) -> TreeNodeView {
    let run_period = &model.run_period;
    TreeNodeView {
        id: "energy-settings".into(),
        label: format!("{} · {SET_SETTINGS_ACTION_ID} · {SET_RESULT_FIELD_ACTION_ID} · set-run-period", say(locale, "Run settings (editable)", "Laufeinstellungen (bearbeitbar)")),
        children: vec![
            leaf("energy-setting-zone-timestep", format!("{}: {} min", say(locale, "Zone timestep", "Zonen-Zeitschritt"), settings.zone_timestep_minutes)),
            leaf("energy-setting-system-timestep", format!("{}: {} min", say(locale, "System timestep", "Anlagen-Zeitschritt"), settings.system_timestep_minutes)),
            leaf("energy-setting-warmup-days", format!("{}: {}", say(locale, "Warmup days", "Einschwingtage"), settings.warmup_days)),
            leaf("energy-setting-run-period", format!("{}: {:02}-{:02} → {:02}-{:02}", say(locale, "Run period (month-day)", "Simulationszeitraum (Monat-Tag)"), run_period.start_month, run_period.start_day, run_period.end_month, run_period.end_day)),
            leaf("energy-setting-weather", format!("{}: {}", say(locale, "Weather file", "Wetterdatei"), say(locale, "bound through the model's weather link", "über die Wetterverknüpfung des Modells gebunden"))),
            leaf(
                "energy-setting-result-field",
                format!("{}: {}", say(locale, "Surfaces coloured by", "Flächen eingefärbt nach"), result_field_text(crate::editor::model::results::result_field(settings), locale)),
            ),
        ],
    }
}

/// 🎨️ The localized name of the field the 3d model window currently colours by.
fn result_field_text(field: ResultField, locale: Locale) -> &'static str {
    match field {
        ResultField::ConductionLoss => say(locale, "Conduction loss", "Transmissionsverlust"),
        ResultField::ConductionGain => say(locale, "Conduction gain", "Transmissionsgewinn"),
        ResultField::SolarTransmitted => say(locale, "Solar transmitted", "Solare Transmission"),
        ResultField::SolarAbsorbed => say(locale, "Solar absorbed", "Solare Absorption"),
    }
}

/// 🚦️ The run as the framework ledger reports it: its localized state and the identity the framework
/// panel's buttons carry.
fn run_nodes(run: Option<&ToolRunView>, locale: Locale) -> TreeNodeView {
    let Some(run) = run.filter(|run| run.tool_id == tools::simulation::TOOL_ID) else {
        return TreeNodeView {
            id: "energy-simulation-live-region".into(),
            label: format!("aria-live=polite · role=status · busy=false · {}", say(locale, "No energy simulation run", "Kein Energiesimulationslauf")),
            children: vec![leaf("energy-simulation-start-hint", format!("{} — {}", ToolRunAction::Start.chord(), ToolRunAction::Start.label().text(locale)))],
        };
    };
    TreeNodeView {
        id: "energy-simulation-live-region".into(),
        label: format!("aria-live=polite · role=status · busy={} · {}", !run.state.is_terminal(), run.state.label().text(locale)),
        children: vec![
            leaf("energy-run", format!("{}: {} · {}: {}", say(locale, "Run", "Lauf"), run.identity.id.run, say(locale, "Generation", "Generation"), run.identity.generation)),
            leaf("energy-result", format!("{}: {}", say(locale, "Result", "Ergebnis"), result_text(run.state, locale))),
        ],
    }
}

/// ⌨️ The framework chords that drive the run, rendered rather than assumed.
fn keyboard_node(locale: Locale) -> TreeNodeView {
    let actions = [ToolRunAction::Start, ToolRunAction::Pause, ToolRunAction::Step, ToolRunAction::Abort, ToolRunAction::Finalize];
    TreeNodeView {
        id: "energy-keyboard-help".into(),
        label: say(locale, "Keyboard", "Tastatur").into(),
        children: actions.iter().map(|action| leaf(format!("energy-keyboard-{}", action.id()), format!("{} — {}", action.chord(), action.label().text(locale)))).collect(),
    }
}

/// ⚡️ The whole window. With no run the settings are still shown and editable, so a user can configure
/// the run before starting it.
pub fn render(run: Option<&ToolRunView>, settings: &EnergyModelConfig, model: &crate::model::Model, locale: Locale) -> BuiltNode {
    let roots = vec![run_nodes(run, locale), settings_nodes(settings, model, locale), keyboard_node(locale)];
    TreeWindowKit::render(&TreeView { roots }).unwrap_or_else(|_| semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data("Energy simulation UI unavailable")).expect("static label is valid"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
