//! ⏯️ Tool run action injection over `🧫️fixtures/⏯️tool-run-actions.json`: the seven framework-reserved
//! actions appear exactly when a tool or utility declares `run`, with the §2.5 ids, args, chords and
//! EN/DE labels, and their chords obey the framework chord law.
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` §2.4, §2.5.

use super::app_label_tests::app_with;
use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fixture {
    kind: ActionKind,
    actions: Vec<ActionRow>,
    reserved_chords: Vec<String>,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ActionRow {
    id: String,
    icon_id: String,
    #[serde(default)]
    keys: Option<String>,
    #[serde(default)]
    panel_chord: Option<String>,
    label: LocaleRow,
    args: Vec<ArgRow>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LocaleRow {
    en: String,
    de: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ArgRow {
    id: String,
    schema: String,
    required: bool,
    label: LocaleRow,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    name: String,
    tools: Vec<ToolDefinition>,
    utilities: Vec<UtilityDefinition>,
    injected: bool,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/⏯️tool-run-actions.json")).unwrap()
}

fn arg_schema_name(schema: &ArgSchema) -> &'static str {
    match schema {
        ArgSchema::String { .. } => "text",
        ArgSchema::Number { integer: true, .. } => "integer",
        _ => "other",
    }
}

#[semio_framework_async_macros::async_test]
async fn tool_run_actions_are_injected_exactly_when_a_tool_or_utility_declares_run() {
    let fixture = fixture();
    assert_eq!(fixture.cases.len(), 4);
    for case in fixture.cases {
        let mut app = app_with(vec![], vec![]).await;
        app.tools = case.tools;
        app.utilities = case.utilities;
        let ids: Vec<String> = tool_run_action_definitions(&app).into_iter().map(|action| action.id).collect();
        let expected: Vec<String> = if case.injected { TOOL_RUN_ACTION_IDS.iter().map(|id| id.to_string()).collect() } else { Vec::new() };
        assert_eq!(ids, expected, "{}", case.name);
    }
}

#[semio_framework_async_macros::async_test]
async fn tool_run_actions_match_the_shared_fixture_rows() {
    let fixture = fixture();
    let mut app = app_with(vec![], vec![]).await;
    app.tools = vec![ToolDefinition { run: Some(sample_run()), ..ToolDefinition::new("fill", LocalizedLabel::native("Fill", "Füllen"), "paint-bucket").await }];
    let actions = tool_run_action_definitions(&app);
    assert_eq!(actions.len(), fixture.actions.len());
    for (action, row) in actions.iter().zip(&fixture.actions) {
        assert_eq!(action.id, row.id);
        assert_eq!(action.kind, fixture.kind, "{}", row.id);
        assert_eq!(action.icon_id.as_str(), row.icon_id, "{}", row.id);
        assert_eq!(action.keys, row.keys, "{}", row.id);
        assert!(!action.in_palette, "{} is dispatched by the run panel and chords, never the palette", row.id);
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{}", row.id);
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), row.label.en, "{}", row.id);
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), row.label.de, "{}", row.id);
        assert_eq!(action.args.len(), row.args.len(), "{}", row.id);
        for (arg, arg_row) in action.args.iter().zip(&row.args) {
            assert_eq!((arg.id.as_str(), arg.required, arg_schema_name(&arg.schema)), (arg_row.id.as_str(), arg_row.required, arg_row.schema.as_str()), "{}", row.id);
            assert_eq!(arg.presentation, Some(ArgPresentation::Hidden), "{}.{} is filled by the host", row.id, arg.id);
            assert_eq!(arg.label.resolve(Terminology::Native, Locale::En), arg_row.label.en);
            assert_eq!(arg.label.resolve(Terminology::Native, Locale::De), arg_row.label.de);
        }
        let contract = ToolRunAction::from_id(&row.id).unwrap();
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), contract.label().text(Locale::En));
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), contract.label().text(Locale::De));
        let contract_args: Vec<(&str, bool)> = contract.args().iter().map(|arg| (arg.name, arg.required)).collect();
        let declared_args: Vec<(&str, bool)> = action.args.iter().map(|arg| (arg.id.as_str(), arg.required)).collect();
        assert_eq!(declared_args, contract_args, "{}", row.id);
        assert_eq!(row.keys.as_deref().or(row.panel_chord.as_deref()), Some(contract.chord()), "{}", row.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn tool_run_chords_obey_the_framework_chord_law() {
    let fixture = fixture();
    let mut app = app_with(vec![], vec![]).await;
    app.utilities = vec![UtilityDefinition { run: Some(sample_run()), ..UtilityDefinition::new("brush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush") }];
    let actions = tool_run_action_definitions(&app);
    let keys_of = |action: ToolRunAction| actions.iter().find(|entry| entry.id == action.id()).unwrap().keys.clone();
    for action in ToolRunAction::ALL {
        match action {
            ToolRunAction::Dismiss => assert_eq!(keys_of(action), None, "escape dismisses only with panel focus, never as an app-wide binding"),
            _ => assert_eq!(keys_of(action).as_deref(), Some(action.chord()), "{}", action.id()),
        }
    }
    assert_eq!(keys_of(ToolRunAction::Pause), keys_of(ToolRunAction::Resume), "pause and resume share one toggle chord");
    let global: Vec<String> = actions.iter().filter_map(|action| action.keys.clone()).collect();
    let mut distinct = global.clone();
    distinct.sort();
    distinct.dedup();
    assert_eq!(distinct.len(), 5, "only the pause/resume toggle shares a chord: {global:?}");
    let mut framework_chords: Vec<String> = history_action_definitions().into_iter().chain(clipboard_action_definitions()).filter_map(|action| action.keys).collect();
    framework_chords.extend([CLEAR_SELECTION_ACTION_ID, SELECT_ALL_ACTION_ID].map(|id| {
        let mut interactive = app.clone();
        interactive.interactions = vec![InteractionDefinition { id: "d".into(), label: LocalizedLabel::data("D"), granularities: vec![], hierarchy: crate::HierarchyProvider::Flat, hover: crate::HoverSpec::default(), selection: crate::SelectionSpec { modes: vec![], methods: vec![], merges: vec![], transitive: false, broadcast: false } }];
        interaction_action_definitions(&interactive).into_iter().find(|action| action.id == id).and_then(|action| action.keys).unwrap()
    }));
    framework_chords.sort();
    let mut reserved = fixture.reserved_chords.clone();
    reserved.sort();
    assert_eq!(framework_chords, reserved, "the fixture pins every other framework-injected chord");
    for chord in &global {
        assert!(!reserved.contains(chord), "{chord} collides with a framework-injected chord");
        let key = chord.rsplit('+').next().unwrap();
        let shifted = chord.split('+').any(|modifier| modifier == "shift");
        assert!(!(shifted && key.chars().all(|character| character.is_ascii_punctuation())), "{chord}: shifted punctuation differs by keyboard layout");
        assert_eq!(chord, &chord.to_lowercase(), "{chord}: chords are lower-case");
    }
}

#[semio_framework_async_macros::async_test]
async fn tool_and_utility_run_round_trips_through_serde_and_the_value_codec() {
    let fixture = fixture();
    for case in fixture.cases.into_iter().filter(|case| case.injected) {
        for tool in &case.tools {
            let json = serde_json::to_value(tool).unwrap();
            assert_eq!(serde_json::from_value::<ToolDefinition>(json.clone()).unwrap(), *tool, "{}", case.name);
            assert_eq!(ToolDefinition::from_value(tool.to_value()).unwrap(), *tool, "{}", case.name);
            assert_eq!(json.get("run").is_some(), tool.run.is_some(), "{}", case.name);
        }
        for utility in &case.utilities {
            assert_eq!(serde_json::from_value::<UtilityDefinition>(serde_json::to_value(utility).unwrap()).unwrap(), *utility, "{}", case.name);
            assert_eq!(UtilityDefinition::from_value(utility.to_value()).unwrap(), *utility, "{}", case.name);
            utility.run.as_ref().unwrap().validate().unwrap();
        }
    }
}

fn sample_run() -> ToolRunDefinition {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⏯️tool-run-actions.json")).unwrap();
    serde_json::from_value(fixture["cases"][2]["tools"][1]["run"].clone()).unwrap()
}
