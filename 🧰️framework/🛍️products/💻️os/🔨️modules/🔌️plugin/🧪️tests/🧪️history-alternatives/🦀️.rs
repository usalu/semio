//! 🌿️ The alternatives list of the history body, driven by the language-agnostic fixture
//! `🧫️fixtures/🧫️history-alternatives/🔣️.json` whose cases the TypeScript twin (`🟦️.ts` beside this file) validates with
//! Ajv and derives with its own implementation, the branch time through JavaScript's `Date` as the third-party oracle.

use super::*;
use serde_json::{json, Value};

const HISTORY_ALTERNATIVES_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🧫️history-alternatives/🔣️.json");

fn fact(value: &Value) -> AlternativeFact<'_> {
    match value["kind"].as_str() {
        Some("branch") => AlternativeFact::Branch { alternative_id: value["alternative"].as_str().expect("alternative"), actor: value["actor"].as_str().expect("actor"), physical_ms: value["at"].as_u64().expect("at") },
        Some("supersede") => AlternativeFact::Supersede { scope: value["scope"].as_str() },
        other => panic!("unknown fact kind {other:?}"),
    }
}

/// ⚖️ LAW: every fixture case derives exactly its expected alternatives list.
#[test]
fn every_fixture_case_lists_its_alternatives() {
    let fixture: Value = serde_json::from_str(HISTORY_ALTERNATIVES_FIXTURE_JSON).expect("history-alternatives fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let facts: Vec<AlternativeFact<'_>> = case["facts"].as_array().expect("facts").iter().map(fact).collect();
        let alternatives = case["alternatives"].as_array().expect("alternatives").iter().map(|alternative| (alternative["id"].as_str().expect("id"), alternative["name"].as_str().expect("name")));
        let rows: Vec<Value> = history_alternative_views(alternatives, case["active"].as_str(), &facts)
            .into_iter()
            .map(|view| json!({ "id": view.id, "name": view.name, "current": view.current, "author": view.author, "branchedAt": view.branched_at, "edited": view.edited }))
            .collect();
        assert_eq!(Value::Array(rows), case["expected"], "{id}");
    }
}
