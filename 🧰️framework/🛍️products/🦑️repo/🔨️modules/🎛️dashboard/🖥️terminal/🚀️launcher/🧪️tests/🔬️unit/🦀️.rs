use super::*;
use dashboard_registry::Facts;
use semio_framework_ui_tui_widgets::widget::{TreeState, WidgetState};

fn source_launcher() -> Harness {
    let facts: Facts = serde_json::from_value(serde_json::json!({"projects":[{"name":"workspace","root":".","dashboard":{"tools":[{"id":"source","command":["bun","{project}"],"parameters":[{"id":"project","kind":"text","required":true,"suggestions":{"source":"projects"}}]}]}},{"name":"app","root":"app"}],"playgrounds":[],"tickets":[],"problems":[]})).unwrap();
    let registry = Arc::new(Registry::build(std::path::Path::new("workspace"), &facts));
    assert!(registry.problems().is_empty(), "{:?}", registry.problems());
    let entry = registry.entries().iter().position(|entry| entry.id == "tool:workspace/source").unwrap();
    let mut state = LauncherState::new(Preferences {language:Some("en".into()),..Default::default()}, Locale::En);
    state.set_registry(registry); state.set_page(40); state.open_form(entry); state.edited("app".into());
    let widget = WidgetState::Tree(state.rebuild_tree(None));
    Harness { state, widget }
}

#[test]
fn editable_project_suggestions_share_keyboard_and_pointer_acceptance() {
    let mut keyboard = source_launcher(); let mut pointer = source_launcher();
    assert_eq!(keyboard.state.apply(&mut keyboard.widget, Input::SuggestAccept), Outcome::Idle);
    let row = pointer.state.screen().items.iter().position(|row| row.contains("app · app · app")).unwrap();
    assert_eq!(pointer.state.click(&mut pointer.widget, row), Outcome::Idle);
    for state in [&keyboard.state, &pointer.state] { let form = state.form.as_ref().unwrap(); assert_eq!(Form::effective(&form.fields[0]), Some("app")); assert_eq!(form.focus, 1); }
}

#[test]
fn suggestion_help_uses_the_selected_locale_and_customized_bindings() {
    let mut harness = source_launcher();
    harness.state.preferences.bindings.insert("view.suggest-accept".into(), vec!["alt+y".into()]);
    assert!(harness.state.screen().caption.contains("Alt+Y"));
    harness.state.set_locale(Locale::De);
    let caption = harness.state.screen().caption;
    assert!(caption.contains("Alt+Y")); assert!(caption.contains("übernimmt"));
}

fn fixture() -> serde_json::Value { serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🧫️fixtures/🔎️launcher/🔣️.json")).unwrap() }

fn registry() -> Arc<Registry> {
    let facts: Facts = serde_json::from_value(fixture()["workspace"].clone()).unwrap();
    Arc::new(Registry::build(std::path::Path::new("workspace"), &facts))
}

struct Harness { state: LauncherState, widget: WidgetState }

fn launcher() -> Harness {
    let mut state = LauncherState::new(Preferences {language:Some("en".into()),..Default::default()}, Locale::En);
    state.set_registry(registry());
    state.set_page(40);
    let widget = WidgetState::Tree(state.rebuild_tree(None));
    Harness { state, widget }
}

impl Harness {
    fn tree(&self) -> &TreeState { match &self.widget { WidgetState::Tree(tree) => tree, _ => unreachable!("the harness holds a tree") } }

    fn ids(&self) -> Vec<String> { self.state.visible_ids(self.tree()) }
}

fn steps(harness: &mut Harness, script: &[serde_json::Value]) -> Outcome {
    let mut outcome = Outcome::Idle;
    for step in script {
        let step = step.as_str().unwrap();
        let inputs: Vec<Input> = match step {
            "up" => vec![Input::Up], "down" => vec![Input::Down], "left" => vec![Input::Left], "right" => vec![Input::Right], "pgup" => vec![Input::PageUp], "pgdn" => vec![Input::PageDown],
            "enter" => vec![Input::Activate], "esc" => vec![Input::Back], "backspace" => vec![Input::Backspace], "clear" => vec![Input::Clear],
            typed => typed.strip_prefix("type:").unwrap_or_else(|| panic!("unknown step {typed}")).chars().map(Input::Char).collect(),
        };
        for input in inputs { outcome = harness.state.apply(&mut harness.widget, input); }
    }
    outcome
}

fn describe(outcome: &Outcome) -> serde_json::Value {
    match outcome {
        Outcome::Idle => serde_json::Value::Null,
        Outcome::Leave => "leave".into(),
        Outcome::Start(start) => serde_json::json!({ "start": { "id": start.id, "chosen": start.chosen, "extra": start.extra, "env": start.env } }),
    }
}

#[test]
fn the_fixture_workspace_offers_exactly_the_pinned_commands() {
    let registry = registry();
    let offered: Vec<serde_json::Value> = registry.entries().iter().filter(|entry| entry.listed).map(|entry| serde_json::json!({ "id": entry.id, "label": entry.label })).collect();
    assert_eq!(serde_json::Value::Array(offered), fixture()["entries"]);
    assert!(registry.problems().is_empty(), "{:?}", registry.problems());
}

#[test]
fn journeys_match_the_shared_vectors() {
    let fixture = fixture();
    for journey in fixture["journeys"].as_array().unwrap() {
        let mut harness = launcher();
        let outcome = steps(&mut harness, journey["steps"].as_array().unwrap());
        let screen = harness.state.screen();
        let browse = harness.state.stage() == Stage::Browse;
        let items = if browse { harness.state.rows(harness.tree()) } else { screen.items.clone() };
        let actual = serde_json::json!({ "stage": format!("{:?}", harness.state.stage()).to_lowercase(), "outcome": describe(&outcome), "caption": if browse { harness.state.caption(harness.tree()) } else { screen.caption.clone() }, "input": if browse { harness.tree().query().to_string() } else { screen.input.clone() }, "ids": harness.ids(), "items": items });
        let expect = &journey["expect"];
        for (key, value) in expect.as_object().unwrap() {
            match key.as_str() {
                "contains" => for needle in value.as_array().unwrap() { assert!(items.iter().any(|item| item.contains(needle.as_str().unwrap())), "{}: {needle} missing from {items:?}", journey["name"]); },
                "lacks" => for needle in value.as_array().unwrap() { assert!(!items.iter().any(|item| item.contains(needle.as_str().unwrap())), "{}: {needle} present in {items:?}", journey["name"]); },
                other => assert_eq!(&actual[other], value, "{}: {other}", journey["name"]),
            }
        }
    }
}

#[test]
fn selection_search_and_open_groups_survive_a_registry_update() {
    let mut harness = launcher();
    steps(&mut harness, &[serde_json::json!("type:quiz"), serde_json::json!("down")]);
    let before = (harness.tree().query().to_string(), harness.ids(), harness.tree().selected_item().and_then(|item| harness.tree().item(item).map(|found| found.id.clone())));
    harness.state.set_registry(registry());
    let rebuilt = harness.state.rebuild_tree(Some(harness.tree()));
    harness.widget = WidgetState::Tree(rebuilt);
    let after = (harness.tree().query().to_string(), harness.ids(), harness.tree().selected_item().and_then(|item| harness.tree().item(item).map(|found| found.id.clone())));
    assert_eq!(after, before);
    assert_eq!(harness.state.stage(), Stage::Browse);
}

#[test]
fn the_verbs_are_filed_in_the_language_of_the_dashboard() {
    let mut german = LauncherState::new(Preferences {language:Some("de".into()),..Default::default()}, Locale::De);
    german.set_registry(registry());
    let rows = german.rows(&german.rebuild_tree(None)).join("\n");
    assert!(rows.contains("testen") && rows.contains("bauen"), "{rows}");
}

#[test]
fn a_collapsed_tree_lists_only_verbs_and_expanding_reveals_children() {
    let mut harness = launcher();
    assert!(harness.state.rows(harness.tree()).iter().all(|row| row.starts_with("▸ ")), "{:?}", harness.state.rows(harness.tree()));
    steps(&mut harness, &[serde_json::json!("right")]);
    assert!(harness.state.rows(harness.tree()).len() > 9);
    steps(&mut harness, &[serde_json::json!("left")]);
    assert!(harness.state.rows(harness.tree()).iter().all(|row| row.starts_with("▸ ")));
}

#[test]
fn every_scenario_of_the_launcher_feature_is_proved_by_a_test() {
    dashboard_core::testkit::assert_proved(semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🚀️launcher/🥒️.feature"), &[semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🔬️unit/🦀️.rs")], &[
        ("The fixture workspace offers exactly the pinned commands", &["the_fixture_workspace_offers_exactly_the_pinned_commands"]),
        ("Journeys match the shared vectors", &["journeys_match_the_shared_vectors"]),
        ("A registry update keeps the search, the selection and the open groups", &["selection_search_and_open_groups_survive_a_registry_update"]),
        ("The verbs are filed in the language of the dashboard", &["the_verbs_are_filed_in_the_language_of_the_dashboard"]),
        ("A collapsed tree lists only verbs", &["a_collapsed_tree_lists_only_verbs_and_expanding_reveals_children"]),
    ]);
}

#[test]
fn localized_metadata_is_visible_in_command_search_and_parameter_forms() {
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧫️fixtures/🌐️metadata/🔣️.json")).unwrap();
    let facts: Facts = serde_json::from_value(fixture["workspace"].clone()).unwrap();
    let registry = Arc::new(Registry::build(std::path::Path::new("workspace"), &facts));
    for row in fixture["display"].as_array().unwrap() {
        let locale = if row["locale"] == "en" { Locale::En } else { Locale::De };
        let mut state = LauncherState::new(Preferences {language:Some(locale.as_str().into()),..Default::default()}, locale);
        state.set_registry(registry.clone());
        state.set_page(40);
        let mut harness = Harness { widget: WidgetState::Tree(state.rebuild_tree(None)), state };
        steps(&mut harness, &[serde_json::json!(format!("type:{}", row["title"].as_str().unwrap()))]);
        assert_eq!(harness.ids(), ["tool:workspace/source-open"]);
        assert!(harness.state.rows(harness.tree()).join(" ").contains(row["title"].as_str().unwrap()));
        assert!(harness.state.caption(harness.tree()).contains(row["description"].as_str().unwrap()));
        steps(&mut harness, &[serde_json::json!("enter")]);
        let screen = harness.state.screen();
        assert!(screen.items.iter().any(|item| item == row["title"].as_str().unwrap()));
        assert!(screen.items.iter().any(|item| item == row["description"].as_str().unwrap()));
        assert!(screen.items.iter().any(|item| item.contains(row["parameterTitle"].as_str().unwrap())));
        assert!(screen.caption.contains(row["parameterDescription"].as_str().unwrap()));
        assert_eq!(screen.placeholder, row["parameterTitle"].as_str().unwrap());
        steps(&mut harness, &[serde_json::json!("type:a b.rs")]);
        let start = harness.state.form.as_ref().unwrap().start().unwrap();
        assert_eq!(start.id, "tool:workspace/source-open");
        assert_eq!(start.chosen, [("path".to_string(), "a b.rs".to_string())]);
    }
}

#[test]
fn launcher_viewport_and_default_text_follow_the_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
    for row in fixture["viewportCases"].as_array().unwrap() {
        let mut harness = launcher();
        harness.state.set_page(row["initialPage"].as_u64().unwrap() as usize);
        steps(&mut harness, &[serde_json::json!("type:serve docs"), serde_json::json!("enter")]);
        harness.state.set_page(row["page"].as_u64().unwrap() as usize);
        assert_eq!(harness.state.offset, row["expectedTop"].as_u64().unwrap() as usize);
        assert!(harness.state.screen().items.iter().any(|item| item == "tool:workspace/docs"));
    }
    for row in fixture["defaultTextCases"].as_array().unwrap() {
        let mut harness = launcher();
        steps(&mut harness, &[serde_json::json!("type:serve docs"), serde_json::json!("enter")]);
        let form = harness.state.form.as_mut().unwrap();
        form.fields[0].parameter.default = row["default"].as_str().map(str::to_string);
        let screen = harness.state.screen();
        assert_eq!(screen.input, row["expected"].as_str().unwrap());
        if row["default"].is_string() { assert!(screen.items.iter().any(|item| item.contains(&format!("dir: {}", row["expected"].as_str().unwrap())))); }
    }
}

#[test]
fn launcher_pointer_rows_follow_localized_description_layout() {
    let vectors: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧫️fixtures/⌨️batch/🔣️.json")).unwrap();
    let metadata: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧫️fixtures/🌐️metadata/🔣️.json")).unwrap();
    let facts: Facts = serde_json::from_value(metadata["workspace"].clone()).unwrap();
    for row in vectors["pointerCases"].as_array().unwrap() {
        let locale = if row["locale"] == "en" { Locale::En } else { Locale::De };
        let mut state = LauncherState::new(Preferences {language:Some(locale.as_str().into()),..Default::default()}, locale);
        state.set_registry(Arc::new(Registry::build(std::path::Path::new("workspace"), &facts)));
        state.set_page(40);
        let mut harness = Harness { widget:WidgetState::Tree(state.rebuild_tree(None)), state };
        steps(&mut harness, &[serde_json::json!("type:source-open"), serde_json::json!("enter"), serde_json::json!("down")]);
        let screen = harness.state.screen();
        let clicked = screen.items.iter().position(|line| line.contains(row["expectedPlaceholder"].as_str().unwrap())).unwrap();
        harness.state.click(&mut harness.widget, clicked);
        assert_eq!(harness.state.screen().placeholder, row["expectedPlaceholder"].as_str().unwrap());
    }
}
