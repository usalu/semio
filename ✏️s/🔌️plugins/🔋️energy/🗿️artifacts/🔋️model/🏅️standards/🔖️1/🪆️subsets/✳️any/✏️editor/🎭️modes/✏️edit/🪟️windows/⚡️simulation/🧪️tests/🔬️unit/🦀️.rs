use super::*;

fn text(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node), |root| serde_json::to_string(root).expect("window projects"))
}
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity};
use std::collections::BTreeSet;
use std::sync::Arc;

struct RunViewFixture{tool_id:String,identity:ToolRunIdentity,state:ToolRunState,progress:semio_framework_tool_run::ToolRunProgress,entities:semio_framework_tool_run_entities::EntityMap}
impl RunViewFixture{
 fn borrowed(&self)->ToolRunView<'_>{ToolRunView{tool_id:&self.tool_id,identity:self.identity,state:self.state,provisional_entities:&self.entities,progress:&self.progress,payload:None}}
}
fn view(state:ToolRunState)->RunViewFixture{
 let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:1,run:4},[0;32]);
 RunViewFixture{tool_id:tools::simulation::TOOL_ID.into(),identity,state,entities:Default::default(),progress:semio_framework_tool_run::ToolRunProgress{identity,sequence:0,state,stage:0,completed:0,total:None,counters:Vec::new(),units_per_second:0.0,conflicts:0,steps:semio_framework_tool_run::ToolRunStepRing::new()}}
}

#[test]
fn actions_are_localized_and_registered_as_interactive() {
    let definition = definition();
    let own: Vec<_> = crate::window_shared_action_definitions().into_iter().filter(|action| [SET_SETTINGS_ACTION_ID, SET_RESULT_FIELD_ACTION_ID, crate::editor_commands::SET_RUN_PERIOD_ACTION_ID].contains(&action.id.as_str())).collect();
    assert_eq!(own.iter().map(|action| action.id.as_str()).collect::<Vec<_>>(), [SET_SETTINGS_ACTION_ID, SET_RESULT_FIELD_ACTION_ID, crate::editor_commands::SET_RUN_PERIOD_ACTION_ID]);
    assert!(own.iter().all(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated));
    assert_eq!(definition.label, LocalizedLabel::native("Energy simulation", "Energiesimulation"));
    for action in &own {
        assert!(
            semio_framework_ui_locale::Terminology::ALL.iter().all(|&terminology| action.label.resolve(terminology, Locale::En) != action.label.resolve(terminology, Locale::De)),
            "action {} is not really translated",
            action.id
        );
    }
}

#[test]
fn an_idle_window_still_renders_the_editable_run_settings_and_the_framework_start_chord() {
    let model = crate::model::Model::default();
    let text = text(render(None, &EnergyModelConfig::default(), &model, None, Locale::En));
    assert!(text.contains("energy-settings"), "the settings block must render without a run");
    assert!(text.contains("energy-setting-run-period"));
    assert!(text.contains("energy-simulation-start-hint"));
    assert!(text.contains("mod+enter"));
}

#[test]
fn the_window_reads_the_run_state_from_the_framework_ledger_view() {
    let model = crate::model::Model::default();
    for state in ToolRunState::ALL {
        for locale in [Locale::En, Locale::De] {
            let text = text(render(Some(view(state).borrowed()), &EnergyModelConfig::default(), &model, None, locale));
            assert!(text.contains(state.label().text(locale)), "state {state:?} renders its framework label in {locale:?}");
            assert!(!text.contains("aria-live="));
            assert!(!text.contains("role=status"));
            let tree: serde_json::Value = serde_json::from_str(&text).unwrap();
            fn status(node: &serde_json::Value) -> Option<&serde_json::Value> {
                if node["key"] == "energy-simulation-live-region" { return Some(node); }
                node["children"].as_array()?.iter().find_map(status)
            }
            let status = status(&tree).expect("status node exists");
            assert_eq!(status["accessibility"]["live"], "polite");
            if !state.is_terminal() { assert_eq!(status["activity"], "loading"); }
        }
    }
    let foreign = RunViewFixture { tool_id: "otherTool".into(), ..view(ToolRunState::Running) };
    assert!(text(render(Some(foreign.borrowed()), &EnergyModelConfig::default(), &model, None, Locale::En)).contains("No energy simulation run"));
}

#[test]
fn both_authored_languages_produce_different_text_and_the_framework_chords() {
    let model = crate::model::Model::default();
    let english = text(render(None, &EnergyModelConfig::default(), &model, None, Locale::En));
    let german = text(render(None, &EnergyModelConfig::default(), &model, None, Locale::De));
    assert_ne!(english, german);
    assert!(german.contains("Laufeinstellungen"));
    assert!(english.contains("Run settings"));
    for action in [ToolRunAction::Start, ToolRunAction::Pause, ToolRunAction::Step, ToolRunAction::Abort, ToolRunAction::Finalize] {
        assert!(english.contains(action.chord()), "chord of {action:?} is rendered");
    }
}

#[test]
fn the_result_field_action_offers_exactly_the_published_fields_and_the_window_renders_the_current_one() {
    use crate::editor_config::ResultField;
    let declared = crate::window_shared_action_definitions();
    let action = declared.iter().find(|action| action.id == SET_RESULT_FIELD_ACTION_ID).expect("the editor declares set-result-field");
    assert_eq!(action.kind, ActionKind::View, "recolouring reads the document, it never mutates it");
    let arg = action.args.iter().find(|arg| arg.id == "field").expect("the field argument");
    let options: Vec<&str> = match &arg.schema {
        semio_framework_plugin::ArgSchema::String { options, .. } => options.iter().map(|option| option.value.as_str()).collect(),
        other => panic!("the field argument is a single-choice select, found {other:?}"),
    };
    assert_eq!(options, ResultField::ALL.iter().map(|field| field.id()).collect::<Vec<_>>(), "every published field is offerable and nothing else is");

    let model = crate::model::Model::default();
    let mut settings = EnergyModelConfig::default();
    settings.result_field = ResultField::SolarTransmitted.id().into();
    let english = text(render(None, &settings, &model, None, Locale::En));
    assert!(english.contains("Solar transmitted"), "the window reads back the field it will be recoloured by: {english}");
    assert!(english.contains(SET_RESULT_FIELD_ACTION_ID), "the settings node names the verb that changes it");
    let german = text(render(None, &settings, &model, None, Locale::De));
    assert!(german.contains("Solare Transmission"), "and in German: {german}");
}

#[test]
fn the_window_kind_and_body_key_equal_the_run_session_literal() {
    assert_eq!(crate::session::ENERGY_SIMULATION_WINDOW_KIND_ID, crate::windows::simulation::WINDOW_KIND_ID);
    assert_eq!(crate::session::ENERGY_SIMULATION_WINDOW_KIND_ID, crate::windows::simulation::BODY_KEY, "the simulation window's body key IS its kind id");
}

#[test]
fn actual_weather_identity_and_native_select_clear_bindings_are_rendered(){
 let law:serde_json::Value=serde_json::from_str(include_str!("weather.json")).unwrap();
 let link=store::ArtifactLink { target:semio_framework_artifact_reference::ArtifactRef::parse_uri(law["target"].as_str().unwrap()).unwrap(),pin:store::LinkPin::Head,role:law["role"].as_str().unwrap().into() };
 let model=crate::model::Model::default();
 for locale in [Locale::En,Locale::De]{
  let rendered=text(render(None,&EnergyModelConfig::default(),&model,Some(&link),locale));
  assert!(rendered.contains(law["target"].as_str().unwrap()));
  assert!(rendered.contains(law["selectAction"].as_str().unwrap()));
  assert!(rendered.contains(law["clearAction"].as_str().unwrap()));
  assert!(rendered.contains("energy-weather-address-input"));
  assert!(rendered.contains("energy-weather-clear"));
 }
}

#[test]
fn actual_meter_counters_render_kwh_in_both_languages(){
 let law:serde_json::Value=serde_json::from_str(include_str!("meters.json")).unwrap();
 let mut run=view(ToolRunState::Running);
 for (counter,key) in [(crate::session::EnergySimulationRunCounter::HeatingWh,"heatingWh"),(crate::session::EnergySimulationRunCounter::CoolingWh,"coolingWh"),(crate::session::EnergySimulationRunCounter::FacilityElectricityWh,"facilityWh")]{
  run.progress.counters.push(semio_framework_tool_run::ToolRunCounter{counter:counter.index(),value:law[key].as_u64().unwrap()});
 }
 for locale in [Locale::En,Locale::De]{
  let rendered=text(render(Some(run.borrowed()),&EnergyModelConfig::default(),&crate::model::Model::default(),None,locale));
  for key in ["heatingKwh","coolingKwh","facilityKwh"]{assert!(rendered.contains(law[key].as_str().unwrap()),"{rendered}");}
  assert!(rendered.contains(say(locale,"Heating","Heizen")));
  assert!(rendered.contains(say(locale,"Cooling","Kühlen")));
 }
}
