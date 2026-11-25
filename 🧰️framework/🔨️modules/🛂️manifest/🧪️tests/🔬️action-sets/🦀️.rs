//! 🧺️ Action sets over `🧫️fixtures/🧫️action-sets/🔣️.json` — Rust twin of the TypeScript laws beside this file (`resolveManifestActionSets`, `actionSetsFault`).
use super::*;
use semio_framework_manifest_actions::ActionKind;
use semio_framework_manifest_app_surface::testkit::app_with;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🧫️action-sets/🔣️.json")).expect("action-sets fixture")
}

fn ids(value: &Value) -> Vec<String> {
    value.as_array().expect("id list").iter().map(|id| id.as_str().expect("id").to_string()).collect()
}

fn row(id: &str) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::data(id), ActionKind::View)
}

async fn app(id: &str, actions: Vec<ActionDefinition>) -> AppDefinition {
    let mut app = app_with(Vec::new(), Vec::new()).await;
    app.id = id.into();
    app.actions = actions;
    app
}

async fn manifest(case: &Value) -> PluginManifest {
    let mut apps = Vec::new();
    for (id, own) in case["apps"].as_object().expect("apps") {
        apps.push(app(id, ids(own).iter().map(|id| row(id)).collect()).await);
    }
    let action_sets = case["actionSets"].as_array().expect("actionSets").iter().map(|set| ActionSet { id: set["id"].as_str().expect("id").into(), app_ids: ids(&set["appIds"]), actions: ids(&set["actions"]).iter().map(|id| row(id)).collect() }).collect();
    PluginManifest { plugin_id: "fixture".into(), label: "fixture".into(), version: "1.0.0".into(), apps, examples: Vec::new(), capabilities: Vec::new(), topic_contributions: Vec::new(), commands: Vec::new(), artifact_kinds: Vec::new(), hosted_artifact_kinds: Vec::new(), action_sets, dependencies: Vec::new(), contributions: Vec::new() }
}

#[semio_framework_async_macros::async_test]
async fn action_sets_resolve_and_validate_the_way_every_language_does() {
    for case in fixture()["resolution"].as_array().expect("resolution") {
        let name = case["case"].as_str().expect("case");
        let manifest = manifest(case).await;
        assert_eq!(manifest.validate_action_sets().err().as_deref(), case["fault"].as_str(), "{name}: fault");
        let Some(expected) = case["resolved"].as_object() else { continue };
        let resolved = manifest.resolved();
        assert!(resolved.action_sets.is_empty(), "{name}: no set survives resolution");
        for app in &resolved.apps {
            assert_eq!(app.actions.iter().map(|action| action.id.clone()).collect::<Vec<_>>(), ids(&expected[&app.id]), "{name}: roster of {}", app.id);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn sharing_moves_the_rows_members_declare_identically_into_one_set() {
    for case in fixture()["sharing"].as_array().expect("sharing") {
        let name = case["case"].as_str().expect("case");
        let mut manifest = manifest(&serde_json::json!({ "apps": {}, "actionSets": [] })).await;
        for (id, declared) in case["rosters"].as_object().expect("rosters") {
            let actions = ids(declared).iter().map(|action| if case["differing"][id].as_str() == Some(action) { ActionDefinition::bounded_catalog(action.as_str(), LocalizedLabel::data("other"), ActionKind::View) } else { row(action) }).collect();
            manifest.apps.push(app(id, actions).await);
        }
        let mut refusal = None;
        for set in case["sets"].as_array().expect("sets") {
            let members = ids(&set["members"]);
            if let Err(error) = manifest.share_actions(set["id"].as_str().expect("id"), |app| members.contains(&app.id)) {
                refusal = Some(error);
            }
        }
        assert_eq!(refusal.as_deref(), case["refusal"].as_str(), "{name}: refusal");
        if refusal.is_some() {
            continue;
        }
        let table: Vec<(String, Vec<String>, Vec<String>)> = manifest.action_sets.iter().map(|set| (set.id.clone(), set.app_ids.clone(), set.actions.iter().map(|action| action.id.clone()).collect())).collect();
        let expected: Vec<(String, Vec<String>, Vec<String>)> = case["result"].as_array().expect("result").iter().map(|set| (set["id"].as_str().expect("id").to_string(), ids(&set["appIds"]), ids(&set["actions"]))).collect();
        assert_eq!(table, expected, "{name}: table");
        for app in &manifest.apps {
            assert_eq!(app.actions.iter().map(|action| action.id.clone()).collect::<Vec<_>>(), ids(&case["remaining"][&app.id]), "{name}: remaining roster of {}", app.id);
        }
        assert_eq!(manifest.validate_action_sets(), Ok(()), "{name}: the formed table is sound");
    }
}
