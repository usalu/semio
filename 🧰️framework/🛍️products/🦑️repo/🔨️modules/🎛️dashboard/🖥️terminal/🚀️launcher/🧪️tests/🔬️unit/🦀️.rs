use super::*;
use crate::registry::Facts;
use ui_tui::tui::widget::{TreeState, WidgetState};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../🧫️fixtures/🔎️launcher/🔣️.json")).unwrap() }

fn registry() -> Arc<Registry> {
    let facts: Facts = serde_json::from_value(fixture()["workspace"].clone()).unwrap();
    Arc::new(Registry::build(std::path::Path::new("workspace"), &facts))
}

struct Harness { state: LauncherState, widget: WidgetState }

fn launcher() -> Harness {
    let mut state = LauncherState::new(Preferences::default(), Locale::En);
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
    let mut german = LauncherState::new(Preferences::default(), Locale::De);
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
    crate::tests::assert_proved(include_str!("../../../../🧪️tests/🚀️launcher/🥒️.feature"), &[include_str!("🦀️.rs")], &[
        ("The fixture workspace offers exactly the pinned commands", &["the_fixture_workspace_offers_exactly_the_pinned_commands"]),
        ("Journeys match the shared vectors", &["journeys_match_the_shared_vectors"]),
        ("A registry update keeps the search, the selection and the open groups", &["selection_search_and_open_groups_survive_a_registry_update"]),
        ("The verbs are filed in the language of the dashboard", &["the_verbs_are_filed_in_the_language_of_the_dashboard"]),
        ("A collapsed tree lists only verbs", &["a_collapsed_tree_lists_only_verbs_and_expanding_reveals_children"]),
    ]);
}
