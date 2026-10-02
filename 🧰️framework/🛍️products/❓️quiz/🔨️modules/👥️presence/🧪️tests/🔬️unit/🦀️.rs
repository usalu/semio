//! 🫂️ Unit tests of the presence: scopes of every screen, presence and cursor admission problems, wire shapes.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::schema::Anchor;
use crate::views::learner_tag;
use serde_json::json;

fn place(screen: Screen, quiz: Option<&str>, task: Option<&str>) -> Place {
    Place { screen, quiz: quiz.map(str::to_string), task: task.map(str::to_string) }
}

fn presence(tag: &str, identity: Identity, place: Place) -> PresenceState {
    PresenceState { tag: tag.to_string(), identity, place, active: true }
}

fn cursor(tag: &str, point: Option<(&str, f64, f64)>, focus: Option<&str>) -> CursorState {
    CursorState { tag: tag.to_string(), cursor: point.map(|(anchor, x, y)| Cursor { anchor: anchor.to_string(), x, y }), focus: focus.map(Anchor::from), drag: None }
}

fn problem(path: &str, code: IssueCode) -> ValidationIssue {
    ValidationIssue { path: path.to_string(), code }
}

#[test]
fn every_screen_has_one_room_and_the_task_never_splits_a_quiz_room() {
    assert_eq!(roster_scope("architecture"), "architecture");
    let rooms = [
        (place(Screen::Introduction, None, None), Some("architecture/introduction")),
        (place(Screen::Identity, None, None), None),
        (place(Screen::Home, None, None), Some("architecture/home")),
        (place(Screen::Leaderboard, None, None), Some("architecture/leaderboard")),
        (place(Screen::Run, Some("physics"), Some("power")), Some("architecture/quiz/physics")),
        (place(Screen::Run, Some("physics"), Some("energy")), Some("architecture/quiz/physics")),
        (place(Screen::Results, Some("physics"), None), Some("architecture/quiz/physics")),
        (place(Screen::Run, None, Some("power")), None),
        (place(Screen::Results, None, None), None),
        (place(Screen::Quiz, Some("physics"), None), Some("architecture/quiz/physics")),
        (place(Screen::Quiz, None, None), None),
        (place(Screen::Badges, None, None), Some("architecture/badges")),
        (place(Screen::Learner, None, None), None),
        (place(Screen::Preferences, None, None), None),
    ];
    for (place, expected) in rooms {
        assert_eq!(room_scope("architecture", &place).as_deref(), expected, "{place:?}");
    }
}

#[test]
fn presence_states_are_admitted_only_with_a_tag_a_valid_handle_and_a_complete_place() {
    let tag = learner_tag("0123456789abcdef0123456789abcdef");
    assert!(is_tag(&tag));
    assert_eq!(presence_problem(&presence(&tag, Identity::Anonymous, place(Screen::Home, None, None))), None);
    assert_eq!(presence_problem(&presence(&tag, Identity::Name { handle: "Ada Lovelace".to_string() }, place(Screen::Run, Some("physics"), Some("power")))), None);
    for bad in ["", "0a1b2c3", "0A1B2C3D", "0a1b2c3g", "0a1b2c3d4"] {
        assert_eq!(presence_problem(&presence(bad, Identity::Anonymous, place(Screen::Home, None, None))), Some(problem("/tag", IssueCode::TagInvalid)), "{bad}");
    }
    for handle in [String::new(), "ü".repeat(65), " Ada".to_string(), "A\u{200b}da".to_string(), "\u{202e}adA".to_string(), "Ade\u{301}".to_string(), "\u{410}da".to_string()] {
        assert_eq!(presence_problem(&presence(&tag, Identity::Pseudonym { handle }, place(Screen::Home, None, None))), Some(problem("/identity/handle", IssueCode::HandleInvalid)));
    }
    assert_eq!(presence_problem(&presence(&tag, Identity::Anonymous, place(Screen::Results, Some("Physics"), None))), Some(problem("/place/quiz", IssueCode::SlugInvalid)));
    assert_eq!(presence_problem(&presence(&tag, Identity::Anonymous, place(Screen::Run, Some("physics"), Some("Power")))), Some(problem("/place/task", IssueCode::SlugInvalid)));
    assert_eq!(presence_problem(&presence(&tag, Identity::Anonymous, place(Screen::Run, None, Some("power")))), Some(problem("/place/quiz", IssueCode::Required)));
    assert_eq!(presence_problem(&presence("nope", Identity::Name { handle: String::new() }, place(Screen::Run, None, None))), Some(problem("/identity/handle", IssueCode::HandleInvalid)));
}

#[test]
fn only_a_run_or_its_results_name_a_quiz_and_only_a_run_names_a_task() {
    let tag = "0a1b2c3d";
    let issues = |place: Place| presence_issues(&presence(tag, Identity::Anonymous, place));
    assert_eq!(issues(place(Screen::Home, Some("physics"), None)), [problem("/place/quiz", IssueCode::QuizOutsideRun)]);
    assert_eq!(issues(place(Screen::Quiz, Some("physics"), None)), []);
    assert_eq!(issues(place(Screen::Quiz, None, None)), [problem("/place/quiz", IssueCode::Required)]);
    assert_eq!(issues(place(Screen::Quiz, Some("physics"), Some("power"))), [problem("/place/task", IssueCode::TaskWithoutRun)]);
    for screen in [Screen::Badges, Screen::Learner, Screen::Preferences] {
        assert_eq!(issues(place(screen, None, None)), [], "{screen:?}");
        assert_eq!(issues(place(screen, Some("physics"), None)), [problem("/place/quiz", IssueCode::QuizOutsideRun)], "{screen:?}");
    }
    assert_eq!(issues(place(Screen::Results, Some("physics"), Some("power"))), [problem("/place/task", IssueCode::TaskWithoutRun)]);
    assert_eq!(
        issues(place(Screen::Leaderboard, Some("Bad"), Some("power"))),
        [problem("/place/quiz", IssueCode::QuizOutsideRun), problem("/place/quiz", IssueCode::SlugInvalid), problem("/place/task", IssueCode::TaskWithoutRun)]
    );
    assert_eq!(cursor_issues(&cursor("X", Some(("Bad", 2.0, -1.0)), Some("Bad"))), [problem("/cursor/anchor", IssueCode::AnchorInvalid), problem("/cursor/x", IssueCode::OutOfRange), problem("/cursor/y", IssueCode::OutOfRange), problem("/focus", IssueCode::AnchorInvalid), problem("/tag", IssueCode::TagInvalid)]);
    assert_eq!(serde_json::to_value(IssueCode::QuizOutsideRun).ok(), Some(json!("quiz-outside-run")));
    assert_eq!(IssueCode::TaskWithoutRun.as_str(), "task-without-run");
}

#[test]
fn cursor_states_are_admitted_only_inside_their_anchor_box() {
    let tag = "0a1b2c3d";
    for (anchor, x, y) in [("task:power", 0.0, 1.0), ("home", 0.5, 0.25), ("quiz-card:physics", 1.0, 0.0)] {
        assert_eq!(cursor_problem(&cursor(tag, Some((anchor, x, y)), Some(anchor))), None, "{anchor}");
    }
    assert_eq!(cursor_problem(&cursor(tag, None, None)), None);
    for anchor in ["", "Task:power", "task::power", "task:", "-home", "a b", &"a".repeat(65)] {
        assert_eq!(cursor_problem(&cursor(tag, Some((anchor, 0.5, 0.5)), None)), Some(problem("/cursor/anchor", IssueCode::AnchorInvalid)), "{anchor}");
        assert_eq!(cursor_problem(&cursor(tag, None, Some(anchor))), Some(problem("/focus", IssueCode::AnchorInvalid)), "{anchor}");
    }
    for value in [-0.001, 1.001, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(cursor_problem(&cursor(tag, Some(("home", value, 0.5)), None)), Some(problem("/cursor/x", IssueCode::OutOfRange)), "{value}");
        assert_eq!(cursor_problem(&cursor(tag, Some(("home", 0.5, value)), None)), Some(problem("/cursor/y", IssueCode::OutOfRange)), "{value}");
    }
    assert_eq!(cursor_problem(&cursor("X", Some(("home", 2.0, 0.5)), Some("Bad"))), Some(problem("/cursor/x", IssueCode::OutOfRange)));
}

#[test]
fn shared_presence_vectors_of_the_python_reference_hold() {
    use crate::schema::tests::{entries, fixture, typed};
    let vectors = fixture("shared-presence");
    for vector in entries(&vectors["scopes"]) {
        let catalog = vector["catalog"].as_str().unwrap_or_default();
        let place: Place = typed(&vector["place"]);
        assert_eq!(Some(roster_scope(catalog).as_str()), vector["expected"]["roster"].as_str(), "{}", vector["id"]);
        assert_eq!(room_scope(catalog, &place).as_deref(), vector["expected"]["room"].as_str(), "{}", vector["id"]);
    }
    for vector in entries(&vectors["presence"]) {
        let admitted = serde_json::from_value::<PresenceState>(vector["state"].clone()).is_ok_and(|state| presence_problem(&state).is_none());
        assert_eq!(Some(admitted), vector["expected"].as_bool(), "presence {} ({})", vector["id"], vector["rule"]);
    }
    for vector in entries(&vectors["cursors"]) {
        let admitted = serde_json::from_value::<CursorState>(vector["state"].clone()).is_ok_and(|state| cursor_problem(&state).is_none());
        assert_eq!(Some(admitted), vector["expected"].as_bool(), "cursor {} ({})", vector["id"], vector["rule"]);
    }
    for vector in entries(&vectors["thinkingScopes"]) {
        assert_eq!(Some(thinking_scope(vector["catalog"].as_str().unwrap_or_default(), vector["quiz"].as_str().unwrap_or_default()).as_str()), vector["expected"].as_str(), "{}", vector["id"]);
    }
    for vector in entries(&vectors["thinking"]) {
        let admitted = serde_json::from_value::<ThinkingState>(vector["state"].clone()).is_ok_and(|state| thinking_problem(&state).is_none());
        assert_eq!(Some(admitted), vector["expected"].as_bool(), "thinking {} ({})", vector["id"], vector["rule"]);
    }
}

#[test]
fn presence_and_cursor_states_use_their_wire_shapes() {
    let presence = json!({"tag": "0a1b2c3d", "identity": {"kind": "pseudonym", "handle": "Ada"}, "place": {"screen": "run", "quiz": "physics", "task": "power"}, "active": false});
    let typed: PresenceState = serde_json::from_value(presence.clone()).unwrap_or_else(|error| panic!("{error}"));
    assert_eq!((typed.place.screen, typed.active), (Screen::Run, false));
    assert_eq!(serde_json::to_value(&typed).ok(), Some(presence));
    let cursor = json!({"tag": "0a1b2c3d", "cursor": {"anchor": "task:power", "x": 0.25, "y": 1.0}, "focus": "task:power"});
    let typed: CursorState = serde_json::from_value(cursor.clone()).unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(serde_json::to_value(&typed).ok(), Some(cursor));
    assert_eq!(serde_json::to_value(CursorState { tag: "0a1b2c3d".to_string(), cursor: None, focus: None, drag: None }).ok(), Some(json!({"tag": "0a1b2c3d"})));
    assert!(serde_json::from_value::<PresenceState>(json!({"tag": "0a1b2c3d", "identity": {"kind": "anonymous"}, "place": {"screen": "lobby"}, "active": true})).is_err());
    assert!(serde_json::from_value::<PresenceState>(json!({"tag": "0a1b2c3d", "identity": {"kind": "anonymous"}, "place": {"screen": "home"}, "active": true, "learner": "0123456789abcdef0123456789abcdef"})).is_err());
    assert!(serde_json::from_value::<CursorState>(json!({"tag": "0a1b2c3d", "cursor": {"anchor": "home", "x": 0.5}})).is_err());
    assert!(serde_json::from_value::<CursorState>(json!({"tag": "0a1b2c3d", "item": "answer"})).is_err());
}

fn dragging(anchor: &str, item: Option<&str>) -> CursorState {
    CursorState { drag: item.map(|item| crate::schema::CursorDrag { item: item.to_string() }), ..cursor("0a1b2c3d", Some((anchor, 0.5, 0.5)), Some(anchor)) }
}

#[test]
fn item_and_category_anchors_follow_the_anchor_pattern_and_a_drag_names_an_item() {
    for anchor in ["item:tea-light", "category:passive", "item", "item:a:b", "task:power", "category-card:x"] {
        assert_eq!(cursor_problem(&dragging(anchor, Some("tea-light"))), None, "{anchor}");
    }
    for anchor in ["item:Tea-Light", "category:heat pump", "item:", "category:"] {
        assert_eq!(cursor_issues(&dragging(anchor, None)), [problem("/cursor/anchor", IssueCode::AnchorInvalid), problem("/focus", IssueCode::AnchorInvalid)], "{anchor}");
    }
    assert_eq!(cursor_problem(&dragging("home", Some("Tea Light"))), Some(problem("/drag/item", IssueCode::SlugInvalid)));
    let typed: CursorState = serde_json::from_value(json!({"tag": "0a1b2c3d", "cursor": {"anchor": "category:passive", "x": 0.5, "y": 0.5}, "drag": {"item": "tea-light"}})).unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(typed.drag.map(|drag| drag.item).as_deref(), Some("tea-light"));
    assert!(serde_json::from_value::<CursorState>(json!({"tag": "0a1b2c3d", "drag": {"item": "tea-light", "target": "joule"}})).is_err());
}

fn thinking(answers: &serde_json::Value) -> ThinkingState {
    serde_json::from_value(json!({"tag": "0a1b2c3d", "answers": answers})).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn thinking_states_share_draft_answers_within_bounds() {
    assert_eq!(thinking_scope("architecture", "physics"), "architecture/quiz/physics/thinking");
    let draft = thinking(&json!({
        "standards": {"kind": "classification", "assignments": {"a": "low"}},
        "power": {"kind": "sorting", "order": ["s1", "s0"]},
        "buildings": {"kind": "matching", "values": {"load": {"m0": 10, "m1": 42.6}, "demand": {"m0": -0.5}}},
        "empty": {"kind": "classification", "assignments": {}}
    }));
    assert_eq!(thinking_problem(&draft), None);
    let broken = ThinkingState { tag: "X".to_string(), ..thinking(&json!({
        "Power": {"kind": "sorting", "order": ["s1", "s1", "Bad"]},
        "standards": {"kind": "classification", "assignments": {"A": "low", "b": "Low", "c": "old"}},
        "buildings": {"kind": "matching", "values": {"Load": {"m0": 0}, "demand": {"M0": 64, "m1": 1}}},
        "a/b": {"kind": "sorting", "order": []}
    })) };
    assert_eq!(
        thinking_issues(&broken),
        [
            problem("/answers/Power", IssueCode::SlugInvalid),
            problem("/answers/Power/order/1", IssueCode::DuplicateId),
            problem("/answers/Power/order/2", IssueCode::SlugInvalid),
            problem("/answers/a~1b", IssueCode::SlugInvalid),
            problem("/answers/buildings/values/Load", IssueCode::SlugInvalid),
            problem("/answers/buildings/values/demand/M0", IssueCode::SlugInvalid),
            problem("/answers/standards/assignments/A", IssueCode::SlugInvalid),
            problem("/answers/standards/assignments/b", IssueCode::SlugInvalid),
            problem("/tag", IssueCode::TagInvalid),
        ]
    );
    assert_eq!(thinking_problem(&broken), Some(problem("/answers/Power", IssueCode::SlugInvalid)));
    let mut infinite = thinking(&json!({"buildings": {"kind": "matching", "values": {"load": {"m0": 1}}}}));
    if let Some(ThinkingAnswer::Matching(answer)) = infinite.answers.get_mut("buildings") {
        answer.values.entry("load".to_string()).or_default().insert("m1".to_string(), f64::NAN);
        answer.values.entry("load".to_string()).or_default().insert("m2".to_string(), f64::INFINITY);
    }
    assert_eq!(thinking_issues(&infinite), [problem("/answers/buildings/values/load/m1", IssueCode::TypeInvalid), problem("/answers/buildings/values/load/m2", IssueCode::TypeInvalid)]);
    assert!(serde_json::from_value::<ThinkingState>(json!({"tag": "0a1b2c3d", "answers": {"buildings": {"kind": "matching", "assignments": {"load": {"m0": 0}}}}})).is_err(), "card indices are publisher-local and never shared");
}

#[test]
fn thinking_sorting_drafts_carry_guesses_validated_like_values() {
    let guessed = thinking(&json!({"power": {"kind": "sorting", "order": ["s1", "s0"], "guesses": {"s0": 90, "s1": 4.5}}}));
    assert_eq!(thinking_problem(&guessed), None, "the order of guesses is not checked on drafts");
    assert_eq!(thinking_problem(&thinking(&json!({"power": {"kind": "sorting", "order": [], "guesses": {}}}))), None);
    let slugs = thinking(&json!({"power": {"kind": "sorting", "order": ["s0"], "guesses": {"S0": 1, "s-1": 2}}}));
    assert_eq!(thinking_issues(&slugs), [problem("/answers/power/guesses/S0", IssueCode::SlugInvalid)]);
    let mut infinite = thinking(&json!({"power": {"kind": "sorting", "order": ["s0"], "guesses": {"s0": 1}}}));
    if let Some(ThinkingAnswer::Sorting(answer)) = infinite.answers.get_mut("power") {
        answer.guesses.insert("s1".to_string(), f64::NAN);
        answer.guesses.insert("s2".to_string(), f64::NEG_INFINITY);
    }
    assert_eq!(thinking_issues(&infinite), [problem("/answers/power/guesses/s1", IssueCode::TypeInvalid), problem("/answers/power/guesses/s2", IssueCode::TypeInvalid)]);
    let many: serde_json::Map<String, serde_json::Value> = (0..=THINKING_LIMIT).map(|index| (format!("s{index}"), json!(index as f64 * 1.5))).collect();
    assert_eq!(thinking_issues(&thinking(&json!({"power": {"kind": "sorting", "order": [], "guesses": many}}))), [problem("/answers/power/guesses", IssueCode::TooMany)]);
    let at_limit: serde_json::Map<String, serde_json::Value> = (0..THINKING_LIMIT).map(|index| (format!("s{index}"), json!(index as f64 * 1.5))).collect();
    assert_eq!(thinking_problem(&thinking(&json!({"power": {"kind": "sorting", "order": [], "guesses": at_limit}}))), None);
    assert!(serde_json::from_value::<ThinkingState>(json!({"tag": "0a1b2c3d", "answers": {"power": {"kind": "sorting", "order": [], "guesses": {"s0": "9"}}}})).is_err());
    assert!(serde_json::from_value::<ThinkingState>(json!({"tag": "0a1b2c3d", "answers": {"power": {"kind": "classification", "assignments": {}, "guesses": {}}}})).is_err());
}

#[test]
fn thinking_states_refuse_more_than_the_limit() {
    let many = |prefix: &str, count: usize| (0..count).map(|index| format!("{prefix}{index}")).collect::<Vec<_>>();
    let tasks: serde_json::Map<String, serde_json::Value> = many("t", THINKING_LIMIT + 1).into_iter().map(|task| (task, json!({"kind": "sorting", "order": []}))).collect();
    assert_eq!(thinking_issues(&thinking(&serde_json::Value::Object(tasks))), [problem("/answers", IssueCode::TooMany)]);
    let at_limit: serde_json::Map<String, serde_json::Value> = many("t", THINKING_LIMIT).into_iter().map(|task| (task, json!({"kind": "sorting", "order": []}))).collect();
    assert_eq!(thinking_problem(&thinking(&serde_json::Value::Object(at_limit))), None);
    let order = many("s", THINKING_LIMIT + 1);
    assert_eq!(thinking_issues(&thinking(&json!({"power": {"kind": "sorting", "order": order}}))), [problem("/answers/power/order", IssueCode::TooMany)]);
    let assignments: serde_json::Map<String, serde_json::Value> = many("i", THINKING_LIMIT + 1).into_iter().map(|item| (item, json!("low"))).collect();
    assert_eq!(thinking_issues(&thinking(&json!({"standards": {"kind": "classification", "assignments": assignments}}))), [problem("/answers/standards/assignments", IssueCode::TooMany)]);
    let values: serde_json::Map<String, serde_json::Value> = many("m", THINKING_LIMIT + 1).into_iter().enumerate().map(|(index, item)| (item, json!(index as f64 * 1.5))).collect();
    assert_eq!(thinking_issues(&thinking(&json!({"buildings": {"kind": "matching", "values": {"load": values}}}))), [problem("/answers/buildings/values/load", IssueCode::TooMany)]);
    let dimensions: serde_json::Map<String, serde_json::Value> = many("d", THINKING_LIMIT + 1).into_iter().map(|dimension| (dimension, json!({}))).collect();
    assert_eq!(thinking_issues(&thinking(&json!({"buildings": {"kind": "matching", "values": dimensions}}))), [problem("/answers/buildings/values", IssueCode::TooMany)]);
    assert!(serde_json::from_value::<ThinkingState>(json!({"tag": "0a1b2c3d", "answers": {}, "learner": "0123456789abcdef0123456789abcdef"})).is_err());
}
