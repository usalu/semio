//! ✏️ The reserved history-edit verbs and their finalize dialog against the language-agnostic fixture
//! `🧫️fixtures/🧫️history-edit-actions/🔣️.json` (TypeScript twin `🟦️.ts` beside this file, Ajv oracle), plus the
//! finalize dialog against W1-E's renderer fixture `🧫️fixtures/🧫️dialog-choices/🔣️.json`.

use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧫️history-edit-actions/🔣️.json")).expect("history-edit-actions fixture")
}

fn arg_schema_kind(schema: &ArgSchema) -> &'static str {
    match schema {
        ArgSchema::String { options, .. } if !options.is_empty() => "select",
        ArgSchema::String { .. } => "text",
        ArgSchema::Number { integer: true, .. } => "integer",
        ArgSchema::Any => "any",
        other => panic!("unexpected history-edit arg schema {other:?}"),
    }
}

#[test]
fn history_edit_verbs_match_the_fixture_rows_in_lifecycle_order() {
    let fixture = fixture();
    let actions = history_edit_action_definitions();
    let rows = fixture["actions"].as_array().expect("actions");
    assert_eq!(actions.iter().map(|action| action.id.as_str()).collect::<Vec<_>>(), HISTORY_EDIT_ACTION_IDS.to_vec());
    assert_eq!(actions.len(), rows.len());
    for (action, row) in actions.iter().zip(rows) {
        let id = action.id.as_str();
        assert_eq!(id, row["id"].as_str().expect("row id"));
        assert_eq!(action.icon_id.as_str(), row["iconId"].as_str().expect("icon"), "{id}");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), row["label"]["en"].as_str().expect("en"), "{id}");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), row["label"]["de"].as_str().expect("de"), "{id}");
        assert_eq!(action.kind, ActionKind::History, "{id}");
        assert!(!action.in_palette && action.keys.is_none(), "{id}: never in the palette, no chord");
        assert_eq!(action.semantics.effects.destructive, row["destructive"].as_bool().expect("destructive"), "{id}");
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{id}");
        assert_eq!(resolve_audience(action), CapabilityAudience::Agent, "{id}: agent-addressable like undo");
        let description = action.semantics.description.as_ref().unwrap_or_else(|| panic!("{id}: describe"));
        assert!(!description.resolve(Terminology::Native, Locale::En).is_empty() && !description.resolve(Terminology::Native, Locale::De).is_empty(), "{id}: EN and DE description");
        assert!(!action.semantics.use_when.is_empty(), "{id}: use_when");
        let args: Vec<serde_json::Value> = action
            .args
            .iter()
            .map(|arg| serde_json::json!({ "id": arg.id, "schema": arg_schema_kind(&arg.schema), "required": arg.required, "hidden": arg.presentation == Some(ArgPresentation::Hidden) }))
            .collect();
        assert_eq!(serde_json::Value::Array(args), row["args"], "{id}: args");
    }
}

#[test]
fn history_action_definitions_end_with_the_history_edit_verbs() {
    let ids: Vec<String> = history_action_definitions().into_iter().map(|action| action.id).collect();
    assert_eq!(ids[ids.len() - HISTORY_EDIT_ACTION_IDS.len()..], HISTORY_EDIT_ACTION_IDS.map(str::to_string));
    assert!(ids.contains(&REVERT_TO_COMMAND_ACTION_ID.to_string()));
}

#[test]
fn constants_match_the_fixture_vocabulary() {
    let constants = &fixture()["constants"];
    for (key, value) in [
        ("mutationId", HISTORY_EDIT_ARG_MUTATION_ID),
        ("store", HISTORY_EDIT_ARG_STORE),
        ("path", HISTORY_EDIT_ARG_PATH),
        ("value", HISTORY_EDIT_ARG_VALUE),
        ("edit", HISTORY_EDIT_ARG_EDIT),
        ("insert", HISTORY_EDIT_INPUT_INSERT),
        ("remove", HISTORY_EDIT_INPUT_REMOVE),
        ("generation", HISTORY_EDIT_ARG_GENERATION),
        ("name", HISTORY_EDIT_ARG_NAME),
        ("choice", DIALOG_CHOICE_ARG),
        ("overwrite", HISTORY_EDIT_CHOICE_OVERWRITE),
        ("dialogId", HISTORY_EDIT_FINALIZE_DIALOG_ID),
    ] {
        assert_eq!(constants[key].as_str(), Some(value), "{key}");
    }
}

#[test]
fn the_finalize_dialog_submits_an_alternative_offers_a_destructive_overwrite_and_cancels_back() {
    let fixture = fixture();
    let expected = &fixture["finalizeDialog"];
    let dialog = history_edit_finalize_dialog();
    assert_eq!(dialog.validate_choices(), Ok(()));
    assert_eq!(dialog.id, expected["id"].as_str().expect("id"));
    assert_eq!(dialog.submit_action.as_str(), expected["submitAction"].as_str().expect("submit"));
    assert_eq!(dialog.cancel_action.as_ref().map(ActionRef::as_str), expected["cancelAction"].as_str());
    assert_eq!(serde_json::Value::Array(dialog.args.iter().map(|arg| serde_json::Value::String(arg.id.clone())).collect()), expected["args"]);
    let choices: Vec<serde_json::Value> = dialog.choices.iter().map(|choice| serde_json::json!({ "id": choice.id, "action": choice.action.as_str(), "tone": serde_json::to_value(choice.tone).expect("tone"), "destructive": choice.destructive })).collect();
    assert_eq!(serde_json::Value::Array(choices), expected["choices"]);
    let verbs = history_edit_action_definitions();
    for action in [dialog.submit_action.as_str()].into_iter().chain(dialog.cancel_action.as_ref().map(ActionRef::as_str)).chain(dialog.choices.iter().map(|choice| choice.action.as_str())) {
        assert!(verbs.iter().any(|verb| verb.id == action), "{action} is a declared history-edit verb");
    }
}

#[test]
fn the_finalize_dialog_is_the_renderer_dialog_choices_fixture_without_a_literal_default_name() {
    let renderer: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️dialog-choices/🔣️.json")).expect("dialog-choices fixture");
    let mut expected: DialogDefinition = serde_json::from_value(renderer["dialog"].clone()).expect("fixture dialog");
    for arg in &mut expected.args {
        arg.default = None;
    }
    assert_eq!(history_edit_finalize_dialog(), expected);
}

/// 🌿️ `switchAlternative` declares its one required, visible `alternativeId` for agents: an EN and DE description on the
/// verb and on the argument, `use_when` phrases, the History kind and the agent audience.
#[test]
fn switch_alternative_declares_its_alternative_id_for_agents() {
    let action = history_action_definitions().into_iter().find(|action| action.id == SWITCH_ALTERNATIVE_ACTION_ID).expect("switchAlternative");
    assert_eq!((action.kind, resolve_audience(&action)), (ActionKind::History, CapabilityAudience::Agent));
    let [arg] = action.args.as_slice() else { panic!("one argument: {:?}", action.args) };
    assert_eq!((arg.id.as_str(), arg_schema_kind(&arg.schema), arg.required, arg.presentation.as_ref()), (SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID, "text", true, None));
    for description in [action.semantics.description.as_ref().expect("verb description"), arg.description.as_ref().expect("argument description")] {
        for locale in [Locale::En, Locale::De] {
            assert!(!description.resolve(Terminology::Native, locale).is_empty(), "{locale:?}");
        }
    }
    assert!(action.semantics.use_when.len() >= 2, "use_when phrases");
}
