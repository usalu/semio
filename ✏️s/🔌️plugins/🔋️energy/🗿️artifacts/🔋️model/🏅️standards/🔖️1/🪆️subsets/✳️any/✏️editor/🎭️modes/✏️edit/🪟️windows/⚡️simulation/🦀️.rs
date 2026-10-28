//! ⚡️ Accessible Energy simulation window: the editable run settings and the state of the energy
//! simulation tool run as the framework ledger reports it (`ArtifactView::tool_run`). Progress, the
//! step log with the published quality tiers and the start/pause/step/abort/finalize buttons are the
//! framework ToolRun panel's (`📋️tool-run-contract.md` §2.5, §4.1); this window never keeps run state of
//! its own. Every plugin label is authored in English and German with no default; framework labels come
//! from the framework table for the OS-owned locale.

use crate::editor_config::EnergyModelConfig;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable,HasBase,HasChildren,InputKind};
use crate::tools;
use crate::editor_config::ResultField;
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::OriginalToolRunView as ToolRunView;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_tool_run::{ToolRunAction, ToolRunState};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "energy.simulation";
pub const BODY_KEY: &str = "energy.simulation";
pub const SET_SETTINGS_ACTION_ID: &str = semio_s_artifact_energy_model_editor_commands::SET_SIMULATION_SETTINGS_ACTION_ID;
pub const SET_RESULT_FIELD_ACTION_ID: &str = semio_s_artifact_energy_model_editor_commands::SET_RESULT_FIELD_ACTION_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// ⚙️ The config verb this window owns: it publishes the run settings into the config store, whose
/// generation change the framework applies to a live run as its `reconfigure: restart` policy.
pub fn settings_action() -> ActionDefinition {
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
pub fn result_field_action() -> ActionDefinition {
    let options = ResultField::ALL
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
/// `ActionKind::Mutation` reduced through the field-granular `change-run-period-*` kinds — declared HERE because
/// this window is where it is rendered and edited.
pub fn run_period_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(crate::editor_commands::SET_RUN_PERIOD_ACTION_ID, LocalizedLabel::native("Set run period", "Simulationszeitraum setzen"), ActionKind::Mutation).with_args(vec![
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
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Energy simulation", "Energiesimulation"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "activity".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: Some(<EnergyModelConfig as pack::snapshot::ArtifactDsl>::envelope_id().into()),
        artifact_snapshot_schema: Some(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA.into()),
        input_event_schema: None,
        output_schema: Some(crate::session::ENERGY_SIMULATION_RUN_SCHEMA.into()),
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
fn settings_nodes(settings: &EnergyModelConfig, model: &crate::model::Model, weather: Option<&store::ArtifactLink>, locale: Locale) -> TreeNodeView {
    let run_period = &model.run_period;
    TreeNodeView {
        id: "energy-settings".into(),
        label: format!("{} · {SET_SETTINGS_ACTION_ID} · {SET_RESULT_FIELD_ACTION_ID} · set-run-period", say(locale, "Run settings (editable)", "Laufeinstellungen (bearbeitbar)")),
        children: vec![
            leaf("energy-setting-zone-timestep", format!("{}: {} min", say(locale, "Zone timestep", "Zonen-Zeitschritt"), settings.zone_timestep_minutes)),
            leaf("energy-setting-system-timestep", format!("{}: {} min", say(locale, "System timestep", "Anlagen-Zeitschritt"), settings.system_timestep_minutes)),
            leaf("energy-setting-warmup-days", format!("{}: {}", say(locale, "Warmup days", "Einschwingtage"), settings.warmup_days)),
            leaf("energy-setting-run-period", format!("{}: {:02}-{:02} → {:02}-{:02}", say(locale, "Run period (month-day)", "Simulationszeitraum (Monat-Tag)"), run_period.start_month, run_period.start_day, run_period.end_month, run_period.end_day)),
            leaf("energy-setting-weather", format!("{}: {}", say(locale, "Weather file", "Wetterdatei"), weather.map_or_else(||say(locale,"No weather artifact selected","Kein Wetterartefakt ausgewählt").to_string(),|link|format!("{} · {}",link.target.to_uri(),match &link.pin{store::LinkPin::Head=>say(locale,"capture target head at run start","Zielstand beim Laufstart erfassen").to_string(),store::LinkPin::Checkpoint{id}=>format!("{} {id}",say(locale,"Checkpoint","Prüfpunkt")),store::LinkPin::Snapshot{blob}=>format!("{} {}",say(locale,"Snapshot","Momentaufnahme"),blob.hash)})))),
            leaf(
                "energy-setting-result-field",
                format!("{}: {}", say(locale, "Surfaces coloured by", "Flächen eingefärbt nach"), result_field_text(crate::editor_config::result_field(settings), locale)),
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
fn run_nodes(run: Option<ToolRunView<'_>>, locale: Locale) -> TreeNodeView {
    let Some(run) = run.filter(|run| run.tool_id == tools::simulation::TOOL_ID) else {
        return TreeNodeView {
            id: "energy-simulation-live-region".into(),
            label: say(locale, "No energy simulation run", "Kein Energiesimulationslauf").into(),
            children: vec![leaf("energy-simulation-start-hint", format!("{} — {}", ToolRunAction::Start.chord(), ToolRunAction::Start.label().text(locale)))],
        };
    };
    TreeNodeView {
        id: "energy-simulation-live-region".into(),
        label: run.state.label().text(locale).into(),
        children: vec![
            leaf("energy-run", format!("{}: {} · {}: {}", say(locale, "Run", "Lauf"), run.identity.id.run, say(locale, "Generation", "Generation"), run.identity.generation)),
            leaf("energy-result", format!("{}: {}", say(locale, "Result", "Ergebnis"), result_text(run.state, locale))),
            meter_node(&run, crate::session::EnergySimulationRunCounter::HeatingWh, "energy-meter-heating", say(locale,"Heating","Heizen"), locale),
            meter_node(&run, crate::session::EnergySimulationRunCounter::CoolingWh, "energy-meter-cooling", say(locale,"Cooling","Kühlen"), locale),
            meter_node(&run, crate::session::EnergySimulationRunCounter::FacilityElectricityWh, "energy-meter-facility", say(locale,"Facility electricity","Anlagenelektrizität"), locale),
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
pub fn render(run: Option<ToolRunView<'_>>, settings: &EnergyModelConfig, model: &crate::model::Model, weather: Option<&store::ArtifactLink>, locale: Locale) -> BuiltNode {
    let roots = vec![run_nodes(run, locale), settings_nodes(settings, model, weather, locale), keyboard_node(locale)];
    let mut tree = TreeWindowKit::render(&TreeView { roots }).unwrap_or_else(|_| semio_framework_plugin::built_text_node(semio_framework_ui_locale::Label::data("Energy simulation UI unavailable")).expect("static label is valid"));
    if let Ok((select,clear))=weather_controls(weather,locale){
        tree.children.try_push(select).expect("weather selector fits the authored simulation tree");
        tree.children.try_push(clear).expect("weather clear action fits the authored simulation tree");
    }
    announce_run(&mut tree, run.is_some_and(|run| run.tool_id == tools::simulation::TOOL_ID && !run.state.is_terminal()));
    tree
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

/// 📢️ The native accessibility record drives actual live announcements and busy attributes.
fn announce_run(node: &mut BuiltNode, busy: bool) {
    if node.key.as_str() == "energy-simulation-live-region" {
        node.accessibility.live = semio_framework_plugin::plugin_app_close_prelude::Liveness::Polite;
        if busy { node.activity = semio_framework_plugin::plugin_app_close_prelude::Activity::Loading; }
        return;
    }
    for child in node.children.iter_mut() { announce_run(child, busy); }
}

/// 🌦️ Native linked-artifact selection and clearing use the document's ordinary mutation history.
fn weather_controls(weather:Option<&store::ArtifactLink>,locale:Locale)->semio_framework_plugin::UiAssemblyResult<(BuiltNode,BuiltNode)>{
    use crate::editor_commands::{energy_model_action,ui_label,SELECT_WEATHER_FILE_ACTION_ID,CLEAR_WEATHER_FILE_ACTION_ID};
    use semio_framework_plugin::{UiText,PluginAssemblyError,Trigger};
    let refusal=||PluginAssemblyError::new("energy.weather.control","Weather artifact controls exceed their native UI extent");
    let uri=weather.map(|link|link.target.to_uri()).unwrap_or_default();
    let value=UiText::try_from_str(&uri).ok_or_else(refusal)?;
    let (action,_)=energy_model_action(SELECT_WEATHER_FILE_ACTION_ID,None)?;
    let input=semio_framework_ui_contract::input(InputKind::Text).value(value).commit(UiText::try_from_str("blur").unwrap())
        .try_id("energy-weather-address-input").map_err(|_|refusal())?.try_on(Trigger::Commit,action).map_err(|_|refusal())?.try_build().map_err(|_|refusal())?;
    let select=semio_framework_ui_contract::tree_item(ui_label(say(locale,"EPW artifact address","EPW-Artefaktadresse"))?)
        .try_id("energy-weather-address").map_err(|_|refusal())?.try_child(input).map_err(|_|refusal())?.try_build().map_err(|_|refusal())?;
    let mut clear=semio_framework_plugin::tree_item_with_action("energy-weather-clear",ui_label(say(locale,"Clear weather artifact","Wetterartefakt lösen"))?,None,energy_model_action(CLEAR_WEATHER_FILE_ACTION_ID,None)?)?;
    clear.disabled=weather.is_none();
    Ok((select,clear))
}

/// 🧮️ Reads the numerical meter counter retained by the framework ledger without inventing a missing value.
fn meter_node(run:&ToolRunView,counter:crate::session::EnergySimulationRunCounter,id:&str,label:&str,locale:Locale)->TreeNodeView{
 let value=run.progress.counters.iter().find(|value|value.counter==counter.index()).map_or_else(||say(locale,"Pending","Ausstehend").to_string(),|value|format!("{}.{:03} kWh",value.value/1000,value.value%1000));
 leaf(id,format!("{label}: {value}"))
}
