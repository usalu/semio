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
    for sequence in entries(&lifecycle["registrations"]).iter().chain(entries(&lifecycle["learners"])).chain(entries(&lifecycle["malformed"]["registrations"])).chain(entries(&lifecycle["malformed"]["learners"])) {
        for event in sequence.get("given").map(entries).unwrap_or_default() {
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
        for board in vector["expected"]["leaderboards"].as_object().into_iter().flat_map(|boards| boards.values()).flat_map(|callers| callers.as_object().into_iter().flat_map(|callers| callers.values())) {
            round_trips::<Leaderboard>("leaderboard/leaderboard", board);
        }
        for view in vector["expected"]["learnerViews"].as_object().into_iter().flat_map(|views| views.values()) {
            round_trips::<LearnerView>("leaderboard/learner", view);
        }
    }
}

#[test]
fn queries_and_identities_use_their_wire_tags() {
    for query in [json!({"type": "catalog"}), json!({"type": "learner", "learner": "0123456789abcdef0123456789abcdef"}), json!({"type": "run", "run": "0123456789abcdef0123456789abcdef"}), json!({"type": "leaderboard", "period": "all-time"}), json!({"type": "leaderboard", "period": "daily", "quiz": "physics", "learner": "0123456789abcdef0123456789abcdef"}), json!({"type": "leaderboard", "period": "weekly", "learner": "0123456789abcdef0123456789abcdef"}), json!({"type": "leaderboard", "period": "monthly", "quiz": "physics"}), json!({"type": "crowd", "quiz": "physics"}), json!({"type": "handle", "handle": " Ada "})] {
        let typed: Query = typed(&query);
        assert_eq!(Some(typed.type_name()), query["type"].as_str());
        assert_close("query", &json(&typed), &query);
    }
    assert_eq!(json(&Identity::Name { handle: "Ada".to_string() }), json!({"kind": "name", "handle": "Ada"}));
    assert_eq!(json(&BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None, challenge: None }), json!({"kind": "perfect-tasks", "taskKind": "sorting"}));
    assert_eq!(json(&BadgeRule::PerfectTasks { task_kind: None, quiz: Some("physics".to_string()), challenge: Some(Challenge::Hard) }), json!({"kind": "perfect-tasks", "quiz": "physics", "challenge": "hard"}));
    assert_eq!(typed::<BadgeRule>(&json!({"kind": "perfect-quiz", "quiz": "physics", "challenge": "medium"})), BadgeRule::PerfectQuiz { quiz: "physics".to_string(), challenge: Some(Challenge::Medium) });
    assert!(serde_json::from_value::<BadgeRule>(json!({"kind": "perfect-quiz", "quiz": "physics", "challenge": "insane"})).is_err());
    assert!(serde_json::from_value::<BadgeRule>(json!({"kind": "completed-quizzes", "challenge": "hard"})).is_err());
    assert_eq!(Rejection::QuizRevised.as_str(), "quiz-revised");
    assert_eq!(json(&Rejection::HandleInvalid), json!("handle-invalid"));
    for rejection in [Rejection::HandleClaimed, Rejection::IdInvalid, Rejection::LearnerExists, Rejection::RosterFull, Rejection::RunsExhausted, Rejection::AnswersExhausted, Rejection::RunUntimed, Rejection::TaskUnopened, Rejection::TimeUp, Rejection::AlreadyOpened] {
        assert_eq!(json(&rejection), json!(rejection.as_str()));
        assert_eq!(typed::<Rejection>(&json!(rejection.as_str())), rejection);
    }
    assert_eq!([Rejection::RunUntimed, Rejection::TaskUnopened, Rejection::TimeUp, Rejection::AlreadyOpened].map(|rejection| rejection.as_str()), ["run-untimed", "task-unopened", "time-up", "already-opened"]);
    assert_eq!(json(&DEFAULT_LIMITS), json!({"learners": 100000, "runsPerQuiz": 200, "runs": 1000, "answersPerRun": 2000}));
    assert_eq!(json(&HandleView { display: "Ada".to_string(), holder: None }), json!({"display": "Ada"}));
}

#[test]
fn undeclared_members_are_refused_like_additional_properties_false() {
    let task = json!({"kind": "sorting", "id": "s", "title": {"en": "S", "de": "S"}, "prompt": {"en": "S", "de": "S"}, "quantity": {"label": {"en": "W", "de": "W"}, "unit": "W", "scale": "linear", "prefixed": false, "additive": false}, "items": []});
    assert!(serde_json::from_value::<Task>(task.clone()).is_ok());
    let mut extra = task;
    extra["colour"] = json!("red");
    assert!(serde_json::from_value::<Task>(extra).is_err());
    assert!(serde_json::from_value::<Command>(json!({"type": "submit-run", "id": "a", "learner": "b", "run": "c", "extra": 1})).is_err());
    assert!(serde_json::from_value::<BadgeRule>(json!({"kind": "perfect-quiz"})).is_err());
    assert!(serde_json::from_value::<Text>(json!({"en": "a", "de": "b", "fr": "c"})).is_err());
}

#[test]
fn sorting_answers_carry_optional_guesses_per_item() {
    let bare = json!({"kind": "sorting", "order": ["a", "b"]});
    assert_close("bare", &json(&typed::<Answer>(&bare)), &bare);
    let Answer::Sorting(answer) = typed::<Answer>(&bare) else { unreachable!() };
    assert_eq!(answer.guesses, None);
    let guessed = json!({"kind": "sorting", "order": ["b", "a"], "guesses": {"a": 1500.5, "b": 0.002}});
    assert_close("guessed", &json(&typed::<Answer>(&guessed)), &guessed);
    let Answer::Sorting(answer) = typed::<Answer>(&guessed) else { unreachable!() };
    assert_eq!(answer.guesses, Some(BTreeMap::from([("a".to_string(), 1500.5), ("b".to_string(), 0.002)])));
    let empty = json!({"kind": "sorting", "order": [], "guesses": {}});
    assert_close("empty", &json(&typed::<Answer>(&empty)), &empty);
    assert!(serde_json::from_value::<Answer>(json!({"kind": "sorting", "order": [], "guesses": {"a": "1"}})).is_err());
    assert!(serde_json::from_value::<Answer>(json!({"kind": "sorting", "order": [], "guesses": {"a": 1}, "extra": 1})).is_err());
}

#[test]
fn matching_answers_carry_assignments_or_guesses() {
    for answer in [
        json!({"kind": "matching", "assignments": {"load": {"m0": 1, "m1": 0}}}),
        json!({"kind": "matching", "guesses": {"load": {"m0": 12.5, "m1": -3}, "demand": {}}}),
        json!({"kind": "matching", "assignments": {}}),
        json!({"kind": "matching", "guesses": {}}),
        json!({"kind": "matching"}),
        json!({"kind": "matching", "assignments": {"load": {"m0": 1}}, "guesses": {"load": {"m0": 1.5}}}),
    ] {
        assert_close("matching", &json(&typed::<Answer>(&answer)), &answer);
    }
    assert_eq!(typed::<Answer>(&json!({"kind": "matching"})), Answer::Matching(MatchingAnswer { assignments: None, guesses: None }));
    assert!(serde_json::from_value::<Answer>(json!({"kind": "matching", "guesses": {"load": {"m0": "1"}}})).is_err());
    assert!(serde_json::from_value::<Answer>(json!({"kind": "matching", "values": {}})).is_err());
}

#[test]
fn every_challenge_uses_its_kebab_case_name() {
    assert_eq!(json(&CHALLENGES), json!(["easy", "medium", "hard", "expert"]));
    for challenge in CHALLENGES {
        assert_eq!(typed::<Challenge>(&json(&challenge)), challenge);
    }
    for refused in [json!("Easy"), json!("extreme"), json!(""), json!(1), json!(null)] {
        assert!(serde_json::from_value::<Challenge>(refused.clone()).is_err(), "{refused}");
    }
    assert!(Challenge::Easy < Challenge::Medium && Challenge::Medium < Challenge::Hard && Challenge::Hard < Challenge::Expert);
}

const ID: &str = "0123456789abcdef0123456789abcdef";

#[test]
fn the_commands_of_a_run_carry_its_challenge_and_the_instants_a_learner_acted() {
    let start = json!({"type": "start-run", "id": ID, "learner": ID, "run": ID, "quiz": "physics", "challenge": "expert", "at": MAX_TIMESTAMP});
    let open = json!({"type": "open-task", "id": ID, "learner": ID, "run": ID, "task": "loads", "at": 1_700_000_000_000u64});
    let record = json!({"type": "record-answer", "id": ID, "learner": ID, "run": ID, "task": "loads", "answer": {"kind": "sorting", "order": ["a"], "guesses": {"a": 2}}, "at": 0});
    for (command, name) in [(&start, "start-run"), (&open, "open-task"), (&record, "record-answer")] {
        let typed: Command = typed(command);
        assert_eq!((typed.type_name(), typed.id().as_str(), typed.learner().as_str()), (name, ID, ID));
        assert_close(name, &json(&typed), command);
    }
    assert_eq!(typed::<Command>(&open), Command::OpenTask { id: ID.to_string(), learner: ID.to_string(), run: ID.to_string(), task: "loads".to_string(), at: 1_700_000_000_000 });
    assert_eq!(typed::<Command>(&start), Command::StartRun { id: ID.to_string(), learner: ID.to_string(), run: ID.to_string(), quiz: "physics".to_string(), challenge: Challenge::Expert, at: 9_007_199_254_740_991 });
    for (command, member) in [(&start, "challenge"), (&start, "at"), (&open, "at"), (&record, "at"), (&open, "task")] {
        let mut without = command.clone();
        if let Some(object) = without.as_object_mut() {
            object.remove(member);
        }
        assert!(serde_json::from_value::<Command>(without).is_err(), "{member} is required");
    }
    let mut extra = open;
    extra["answer"] = json!({"kind": "sorting", "order": []});
    assert!(serde_json::from_value::<Command>(extra).is_err());
}

#[test]
fn the_facts_of_a_run_carry_its_challenge_and_the_opening_of_its_tasks() {
    let started = json!({"type": "run-started", "learner": ID, "run": ID, "quiz": "physics", "challenge": "hard", "revision": "1".repeat(64), "seed": 4_294_967_295u32, "at": 5});
    let opened = json!({"type": "task-opened", "learner": ID, "run": ID, "task": "loads", "at": 7});
    for (event, name, at) in [(&started, "run-started", 5), (&opened, "task-opened", 7)] {
        let typed: Event = typed(event);
        assert_eq!((typed.type_name(), typed.learner().as_str(), typed.at()), (name, ID, at));
        assert_close(name, &json(&typed), event);
    }
    let mut unchallenged = started;
    if let Some(object) = unchallenged.as_object_mut() {
        object.remove("challenge");
    }
    assert!(serde_json::from_value::<Event>(unchallenged).is_err());
}

#[test]
fn sheets_results_and_views_omit_what_their_challenge_does_not_carry() {
    let sheet = json!({
        "quiz": "physics",
        "seed": 1,
        "challenge": "expert",
        "title": {"en": "P", "de": "P"},
        "description": {"en": "P", "de": "P"},
        "tasks": [
            {"kind": "sorting", "id": "s", "title": {"en": "S", "de": "S"}, "prompt": {"en": "S", "de": "S"}, "quantity": {"label": {"en": "W", "de": "W"}, "unit": "W", "scale": "logarithmic", "prefixed": true, "additive": true}, "items": [], "seconds": 54},
            {"kind": "matching", "id": "m", "title": {"en": "M", "de": "M"}, "prompt": {"en": "M", "de": "M"}, "dimensions": [{"id": "load", "quantity": {"label": {"en": "W", "de": "W"}, "unit": "W", "scale": "linear", "prefixed": false, "additive": false}}], "items": [], "seconds": 30},
            {"kind": "classification", "id": "c", "title": {"en": "C", "de": "C"}, "prompt": {"en": "C", "de": "C"}, "axes": [{"id": "heat", "label": {"en": "H", "de": "H"}}], "categories": [], "items": [], "seconds": 30}
        ]
    });
    assert_close("expert", &json(&typed::<Sheet>(&sheet)), &sheet);
    let mut shown = sheet.clone();
    shown["challenge"] = json!("easy");
    shown["tasks"][0]["keys"] = json!([1, 2.5, 1e9]);
    shown["tasks"][1]["dimensions"][0]["cards"] = json!([3, 1]);
    shown["tasks"][2]["axes"][0] = json!({"id": "heat", "label": {"en": "H", "de": "H"}, "unit": "kW", "min": 0, "max": 10});
    for task in shown["tasks"].as_array_mut().into_iter().flatten().filter_map(Value::as_object_mut) {
        task.remove("seconds");
    }
    assert_close("easy", &json(&typed::<Sheet>(&shown)), &shown);
    let mut short = shown.clone();
    short["tasks"][0]["quantity"]["short"] = json!({"en": "power", "de": "Leistung"});
    short["tasks"][0]["items"] = json!([{"id": "a", "label": {"en": "A long label", "de": "Ein langes Label"}, "short": {"en": "A", "de": "A"}}]);
    short["tasks"][2]["axes"][0]["short"] = json!({"en": "heat", "de": "Wärme"});
    short["tasks"][2]["categories"] = json!([{"id": "x", "label": {"en": "X", "de": "X"}, "short": {"en": "x", "de": "x"}}]);
    assert_close("short", &json(&typed::<Sheet>(&short)), &short);
    short["tasks"][0]["items"][0]["familiar"] = json!(true);
    assert!(serde_json::from_value::<Sheet>(short).is_err(), "a sheet item never says whether it is familiar");
    let mut unchallenged = sheet.clone();
    if let Some(object) = unchallenged.as_object_mut() {
        object.remove("challenge");
    }
    assert!(serde_json::from_value::<Sheet>(unchallenged).is_err());
    let result = json!({"quiz": "physics", "challenge": "hard", "score": 0.5, "points": 150, "tasks": [
        {"kind": "sorting", "task": "s", "score": 0.5, "items": [{"item": "a", "value": 1, "position": 0, "rank": 0, "guess": 2, "miss": false}, {"item": "b", "value": 9, "position": 1, "rank": 1, "miss": true}]},
        {"kind": "matching", "task": "m", "score": 0.5, "dimensions": [{"dimension": "load", "score": 0.5, "items": [{"item": "a", "assigned": 4, "correct": 3, "miss": false}, {"item": "b", "correct": 3, "miss": true}]}]},
        {"kind": "classification", "task": "c", "score": 0.5, "items": [{"item": "a", "assigned": "x", "correct": "x", "credit": 1}, {"item": "b", "correct": "x", "credit": 0}]}
    ]});
    assert_close("result", &json(&typed::<RunResult>(&result)), &result);
    for member in ["challenge", "points"] {
        let mut without = result.clone();
        if let Some(object) = without.as_object_mut() {
            object.remove(member);
        }
        assert!(serde_json::from_value::<RunResult>(without).is_err(), "{member} is required");
    }
    let summary = json!({"run": ID, "quiz": "physics", "challenge": "medium", "status": "submitted", "startedAt": 1, "score": 0.5, "points": 100, "submittedAt": 2});
    assert_close("summary", &json(&typed::<RunSummary>(&summary)), &summary);
    let best = json!({"challenge": "expert", "score": 0.25, "points": 100});
    assert_close("best", &json(&typed::<Best>(&best)), &best);
    assert!(serde_json::from_value::<Best>(json!({"challenge": "expert", "score": 0.25})).is_err());
    let view = json!({"run": ID, "learner": ID, "quiz": "physics", "status": "open", "sheet": sheet, "answers": {}, "startedAt": 1, "opened": {"s": 3}, "hints": {"s": [{"kind": "compare", "item": "a", "other": "b", "factor": 0.25, "verdict": "under"}, {"kind": "compare", "item": "b", "other": "a", "factor": 4, "verdict": "reversed"}], "m": [{"kind": "compare", "item": "a", "other": "b", "dimension": "load", "difference": -2.5, "verdict": "over"}], "c": [{"kind": "profile", "item": "a", "category": "x", "axis": "heat"}, {"kind": "profile", "item": "e", "category": "x", "axis": "heat", "other": "b", "above": false}, {"kind": "group", "item": "b", "other": "a", "together": true}, {"kind": "category", "item": "d", "category": "x"}]}});
    assert_close("view", &json(&typed::<RunView>(&view)), &view);
    let plain = typed::<RunView>(&json!({"run": ID, "learner": ID, "quiz": "physics", "status": "open", "sheet": shown, "answers": {}, "startedAt": 1}));
    assert_eq!((plain.opened.as_ref(), plain.hints.as_ref()), (None, None));
    assert!(!serde_json::to_string(&plain).unwrap_or_default().contains("opened"));
}
