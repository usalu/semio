//! 🩻️ Unit tests of the schema twin: serde round trips of every shared fixture document and the shared test kit.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::path::PathBuf;

const TOLERANCE: f64 = 1e-12;

pub(crate) fn fixture(case: &str) -> Value {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures");
    let directory = std::fs::read_dir(&root).into_iter().flatten().flatten().map(|entry| entry.path()).find(|path| path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.ends_with(case)));
    let path = directory.unwrap_or_else(|| panic!("no fixture case {case} under {}", root.display())).join("🔣️.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub(crate) fn typed<T: DeserializeOwned>(value: &Value) -> T {
    serde_json::from_value(value.clone()).unwrap_or_else(|error| panic!("{error}: {value}"))
}

pub(crate) fn json<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).unwrap_or_else(|error| panic!("{error}"))
}

pub(crate) fn entries(value: &Value) -> &[Value] {
    value.as_array().map_or(&[], Vec::as_slice)
}

pub(crate) fn assert_close(context: &str, produced: &Value, expected: &Value) {
    match (produced, expected) {
        (Value::Number(left), Value::Number(right)) => {
            let (left, right) = (left.as_f64().unwrap_or(f64::NAN), right.as_f64().unwrap_or(f64::NAN));
            assert!((left - right).abs() <= TOLERANCE, "{context}: {left} ≠ {right}");
        }
        (Value::Object(left), Value::Object(right)) => {
            assert_eq!(left.keys().collect::<Vec<_>>(), right.keys().collect::<Vec<_>>(), "{context}: keys");
            for (key, value) in left {
                assert_close(&format!("{context}/{key}"), value, &right[key]);
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            assert_eq!(left.len(), right.len(), "{context}: length");
            for (index, (left, right)) in left.iter().zip(right).enumerate() {
                assert_close(&format!("{context}/{index}"), left, right);
            }
        }
        _ => assert_eq!(produced, expected, "{context}"),
    }
}

fn round_trips<T: Serialize + DeserializeOwned>(context: &str, value: &Value) {
    assert_close(context, &json(&typed::<T>(value)), value);
}

#[test]
fn every_fixture_document_round_trips_through_the_twin() {
    for case in ["sheet-assembly", "badge-rules", "learner-lifecycle", "leaderboard"] {
        for (index, quiz) in entries(&fixture(case)["quizzes"]).iter().enumerate() {
            round_trips::<Quiz>(&format!("{case}/quizzes/{index}"), quiz);
        }
    }
    for case in ["learner-lifecycle", "leaderboard"] {
        round_trips::<Catalog>(&format!("{case}/catalog"), &fixture(case)["catalog"]);
    }
    for sheet in entries(&fixture("sheet-assembly")["sheets"]) {
        round_trips::<Sheet>("sheet-assembly/sheet", &sheet["sheet"]);
    }
    for case in ["sorting-concordance", "matching-concordance", "profile-similarity"] {
        let vectors = fixture(case);
        for task in entries(&vectors["tasks"]) {
            round_trips::<Task>(&format!("{case}/task"), task);
        }
        for vector in entries(&vectors["vectors"]) {
            round_trips::<SheetTask>(&format!("{case}/sheetTask"), &vector["sheetTask"]);
            round_trips::<Answer>(&format!("{case}/answer"), &vector["answer"]);
            round_trips::<TaskResult>(&format!("{case}/expected"), &vector["expected"]);
        }
    }
    let badges = fixture("badge-rules");
    for badge in entries(&badges["badges"]) {
        round_trips::<Badge>("badge-rules/badge", badge);
    }
    for result in entries(&badges["vectors"]).iter().flat_map(|vector| entries(&vector["results"])) {
        round_trips::<RunResult>("badge-rules/result", result);
    }
    let lifecycle = fixture("learner-lifecycle");
    for sequence in entries(&lifecycle["roster"]).iter().chain(entries(&lifecycle["learners"])) {
        for event in entries(&sequence["given"]) {
            round_trips::<Event>("learner-lifecycle/given", event);
        }
        for step in entries(&sequence["steps"]) {
            round_trips::<Command>("learner-lifecycle/command", &step["command"]);
            for event in entries(&step["expected"]["events"]) {
                round_trips::<Event>("learner-lifecycle/event", event);
            }
        }
        if let Some(views) = sequence.get("views") {
            round_trips::<LearnerView>("learner-lifecycle/learner", &views["expected"]["learner"]);
            for view in views["expected"]["runs"].as_object().into_iter().flat_map(|runs| runs.values()) {
                round_trips::<RunView>("learner-lifecycle/run", view);
            }
        }
    }
    for vector in entries(&fixture("leaderboard")["vectors"]) {
        round_trips::<Leaderboard>("leaderboard/leaderboard", &vector["expected"]["leaderboard"]);
        for view in vector["expected"]["learnerViews"].as_object().into_iter().flat_map(|views| views.values()) {
            round_trips::<LearnerView>("leaderboard/learner", view);
        }
    }
}

#[test]
fn queries_and_identities_use_their_wire_tags() {
    for query in [json!({"type": "catalog"}), json!({"type": "learner", "learner": "0123456789abcdef0123456789abcdef"}), json!({"type": "run", "run": "0123456789abcdef0123456789abcdef"}), json!({"type": "leaderboard"})] {
        let typed: Query = typed(&query);
        assert_eq!(Some(typed.type_name()), query["type"].as_str());
        assert_close("query", &json(&typed), &query);
    }
    assert_eq!(json(&Identity::Name { handle: "Ada".to_string() }), json!({"kind": "name", "handle": "Ada"}));
    assert_eq!(json(&BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None }), json!({"kind": "perfect-tasks", "taskKind": "sorting"}));
    assert_eq!(Rejection::QuizRevised.as_str(), "quiz-revised");
    assert_eq!(json(&Rejection::HandleInvalid), json!("handle-invalid"));
}

#[test]
fn undeclared_members_are_refused_like_additional_properties_false() {
    let task = json!({"kind": "sorting", "id": "s", "title": {"en": "S", "de": "S"}, "prompt": {"en": "S", "de": "S"}, "quantity": {"label": {"en": "W", "de": "W"}, "unit": "W", "scale": "linear", "prefixed": false}, "items": []});
    assert!(serde_json::from_value::<Task>(task.clone()).is_ok());
    let mut extra = task;
    extra["colour"] = json!("red");
    assert!(serde_json::from_value::<Task>(extra).is_err());
    assert!(serde_json::from_value::<Command>(json!({"type": "submit-run", "id": "a", "learner": "b", "run": "c", "extra": 1})).is_err());
    assert!(serde_json::from_value::<BadgeRule>(json!({"kind": "perfect-quiz"})).is_err());
    assert!(serde_json::from_value::<Text>(json!({"en": "a", "de": "b", "fr": "c"})).is_err());
}
