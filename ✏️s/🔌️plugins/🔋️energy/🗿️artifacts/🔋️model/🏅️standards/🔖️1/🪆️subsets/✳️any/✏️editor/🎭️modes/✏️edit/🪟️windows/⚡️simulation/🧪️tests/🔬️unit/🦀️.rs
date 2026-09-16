use super::*;

fn text(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("window projects")
}
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity};
use std::collections::BTreeSet;
use std::sync::Arc;

fn view(state: ToolRunState) -> ToolRunView {
    ToolRunView::new(tools::simulation::TOOL_ID, ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 4 }, [0; 32]), state)
}

#[test]
fn actions_are_localized_and_registered_as_interactive() {
    let definition = definition();
    assert_eq!(definition.actions.iter().map(|action| action.id.as_str()).collect::<Vec<_>>(), [SET_SETTINGS_ACTION_ID, SET_RESULT_FIELD_ACTION_ID, crate::editor::model::SET_RUN_PERIOD_ACTION_ID]);
    assert!(definition.actions.iter().all(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated));
    assert_eq!(definition.label, LocalizedLabel::native("Energy simulation", "Energiesimulation"));
    for action in &definition.actions {
        assert!(
            semio_framework::Terminology::ALL.iter().all(|&terminology| action.label.resolve(terminology, Locale::En) != action.label.resolve(terminology, Locale::De)),
            "action {} is not really translated",
            action.id
        );
    }
}

#[test]
fn an_idle_window_still_renders_the_editable_run_settings_and_the_framework_start_chord() {
    let model = crate::model::Model::default();
    let text = text(render(None, &EnergyModelConfig::default(), &model, Locale::En));
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
            let text = text(render(Some(&view(state)), &EnergyModelConfig::default(), &model, locale));
            assert!(text.contains(state.label().text(locale)), "state {state:?} renders its framework label in {locale:?}");
            assert!(text.contains(&format!("busy={}", !state.is_terminal())));
        }
    }
    let foreign = ToolRunView { tool_id: "otherTool".into(), ..view(ToolRunState::Running) };
    assert!(text(render(Some(&foreign), &EnergyModelConfig::default(), &model, Locale::En)).contains("No energy simulation run"));
}

#[test]
fn both_authored_languages_produce_different_text_and_the_framework_chords() {
    let model = crate::model::Model::default();
    let english = text(render(None, &EnergyModelConfig::default(), &model, Locale::En));
    let german = text(render(None, &EnergyModelConfig::default(), &model, Locale::De));
    assert_ne!(english, german);
    assert!(german.contains("Laufeinstellungen"));
    assert!(english.contains("Run settings"));
    for action in [ToolRunAction::Start, ToolRunAction::Pause, ToolRunAction::Step, ToolRunAction::Abort, ToolRunAction::Finalize] {
        assert!(english.contains(action.chord()), "chord of {action:?} is rendered");
    }
}

#[test]
fn the_result_field_action_offers_exactly_the_published_fields_and_the_window_renders_the_current_one() {
    use crate::editor::model::results::ResultField;
    let definition = definition();
    let action = definition.actions.iter().find(|action| action.id == SET_RESULT_FIELD_ACTION_ID).expect("the window declares set-result-field");
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
    let english = text(render(None, &settings, &model, Locale::En));
    assert!(english.contains("Solar transmitted"), "the window reads back the field it will be recoloured by: {english}");
    assert!(english.contains(SET_RESULT_FIELD_ACTION_ID), "the settings node names the verb that changes it");
    let german = text(render(None, &settings, &model, Locale::De));
    assert!(german.contains("Solare Transmission"), "and in German: {german}");
}
