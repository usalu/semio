use super::*;
use crate::catalog::tests::fixture;
use serde_json::json;

fn rooms() -> Rooms {
    Rooms::of(&fixture())
}

fn presence(place: &OpaqueJson) -> OpaqueJson {
    json!({ "tag": "0a1b2c3d", "identity": { "kind": "pseudonym", "handle": "Ada" }, "place": place, "active": true })
}

fn thinking(answers: &OpaqueJson) -> OpaqueJson {
    json!({ "tag": "0a1b2c3d", "answers": answers })
}

#[test]
fn the_catalog_has_a_roster_four_pages_and_a_quiz_and_thinking_room_per_quiz() {
    let rooms = rooms();
    assert_eq!(
        rooms.scopes(),
        [
            "proctor-fixture",
            "proctor-fixture/badges",
            "proctor-fixture/home",
            "proctor-fixture/introduction",
            "proctor-fixture/leaderboard",
            "proctor-fixture/quiz/homes",
            "proctor-fixture/quiz/homes/thinking",
            "proctor-fixture/quiz/power",
            "proctor-fixture/quiz/power/thinking"
        ]
    );
    assert_eq!(rooms.room("proctor-fixture"), Some(Room::Roster));
    assert_eq!(rooms.room("proctor-fixture/badges"), Some(Room::Page));
    assert_eq!(rooms.room("proctor-fixture/quiz/power"), Some(Room::Quiz("power".into())));
    assert_eq!(rooms.room("proctor-fixture/quiz/power/thinking"), Some(Room::Thinking("power".into())));
    for scope in ["other-catalog", "other-catalog/home", "proctor-fixture/identity", "proctor-fixture/learner", "proctor-fixture/preferences", "proctor-fixture/quiz/cooling", "proctor-fixture/quiz/cooling/thinking", "proctor-fixture/quiz", "proctor-fixture/", ""] {
        assert_eq!(rooms.room(scope), None, "{scope}");
    }
}

#[test]
fn the_roster_admits_a_valid_presence_state_only() {
    let rooms = rooms();
    for place in [json!({ "screen": "home" }), json!({ "screen": "quiz", "quiz": "power" }), json!({ "screen": "run", "quiz": "power", "task": "sources" }), json!({ "screen": "results", "quiz": "homes" }), json!({ "screen": "badges" })] {
        assert_eq!(rooms.admit("proctor-fixture", &presence(&place)), Ok(()), "{place}");
    }
    assert_eq!(rooms.admit("proctor-fixture", &presence(&json!({ "screen": "run", "quiz": "cooling" }))), Err("quiz-unknown /place/quiz".to_string()));
    assert_eq!(rooms.admit("proctor-fixture", &presence(&json!({ "screen": "run", "quiz": "power", "task": "rooms" }))), Err("task-unknown /place/task".to_string()));
    assert_eq!(rooms.admit("proctor-fixture", &presence(&json!({ "screen": "home", "task": "sources" }))), Err("task-without-run /place/task".to_string()));
    assert_eq!(rooms.admit("proctor-fixture", &presence(&json!({ "screen": "quiz" }))), Err("required /place/quiz".to_string()));
    let mut shouting = presence(&json!({ "screen": "home" }));
    shouting["tag"] = json!("0A1B2C3D");
    assert_eq!(rooms.admit("proctor-fixture", &shouting), Err("tag-invalid /tag".to_string()));
    assert_eq!(rooms.admit("proctor-fixture", &json!({ "tag": "0a1b2c3d", "cursor": { "anchor": "home", "x": 0.5, "y": 0.5 } })), Err(STATE_INVALID.to_string()), "a cursor is no roster state");
    assert_eq!(rooms.admit("proctor-fixture", &OpaqueJson::Null), Err(STATE_INVALID.to_string()));
}

#[test]
fn a_page_admits_a_cursor_without_a_drag() {
    let rooms = rooms();
    for scope in ["proctor-fixture/home", "proctor-fixture/leaderboard", "proctor-fixture/introduction", "proctor-fixture/badges"] {
        assert_eq!(rooms.admit(scope, &json!({ "tag": "0a1b2c3d", "cursor": { "anchor": "card:power", "x": 0.25, "y": 1.0 }, "focus": "card:power" })), Ok(()), "{scope}");
        assert_eq!(rooms.admit(scope, &json!({ "tag": "0a1b2c3d" })), Ok(()), "{scope}");
        assert_eq!(rooms.admit(scope, &json!({ "tag": "0a1b2c3d", "drag": { "item": "kettle" } })), Err("id-unknown /drag/item".to_string()), "{scope}");
    }
    assert_eq!(rooms.admit("proctor-fixture/home", &json!({ "tag": "0a1b2c3d", "cursor": { "anchor": "card:power", "x": 1.5, "y": 0.5 } })), Err("out-of-range /cursor/x".to_string()));
    assert_eq!(rooms.admit("proctor-fixture/home", &json!({ "tag": "0a1b2c3d", "focus": "Card Power" })), Err("anchor-invalid /focus".to_string()));
    assert_eq!(rooms.admit("proctor-fixture/home", &json!({ "tag": "0a1b2c3d", "answer": ["kettle"] })), Err(STATE_INVALID.to_string()));
    assert_eq!(rooms.admit("proctor-fixture/home", &presence(&json!({ "screen": "home" }))), Err(STATE_INVALID.to_string()), "a presence state is no cursor state");
}

#[test]
fn a_quiz_room_admits_item_anchors_and_drags_of_its_own_items() {
    let rooms = rooms();
    let drag = |item: &str| json!({ "tag": "0a1b2c3d", "cursor": { "anchor": "item:kettle", "x": 0.5, "y": 0.5 }, "focus": "category:heat-pump", "drag": { "item": item } });
    assert_eq!(rooms.admit("proctor-fixture/quiz/power", &drag("kettle")), Ok(()));
    assert_eq!(rooms.admit("proctor-fixture/quiz/power", &drag("pellets")), Err("id-unknown /drag/item".to_string()), "an item of another quiz");
    assert_eq!(rooms.admit("proctor-fixture/quiz/homes", &drag("pellets")), Ok(()));
    assert_eq!(rooms.admit("proctor-fixture/quiz/homes", &drag("Pellets")), Err("slug-invalid /drag/item".to_string()));
    assert_eq!(rooms.admit("proctor-fixture/quiz/homes", &thinking(&json!({}))), Err(STATE_INVALID.to_string()), "a thinking state belongs to the thinking room");
}

#[test]
fn a_thinking_room_admits_drafts_over_the_ids_its_quiz_renders() {
    let rooms = rooms();
    let power = "proctor-fixture/quiz/power/thinking";
    let homes = "proctor-fixture/quiz/homes/thinking";
    assert_eq!(rooms.admit(power, &thinking(&json!({}))), Ok(()), "nothing drafted yet");
    let drafts = json!({
        "appliances": { "kind": "sorting", "order": ["kettle", "laptop"] },
        "sources": { "kind": "matching", "values": { "capacity": { "rooftop-pv": 10000, "wind-turbine": 1_400_000_000.0 } } }
    });
    assert_eq!(rooms.admit(power, &thinking(&drafts)), Ok(()));
    assert_eq!(rooms.admit(homes, &thinking(&json!({ "systems": { "kind": "classification", "assignments": { "pellets": "wood-stove" } } }))), Ok(()));
    assert_eq!(rooms.admit(power, &thinking(&json!({ "rooms": { "kind": "sorting", "order": [] } }))), Err("task-unknown /answers/rooms".to_string()));
    assert_eq!(rooms.admit(power, &thinking(&json!({ "appliances": { "kind": "classification", "assignments": {} } }))), Err("kind-mismatch /answers/appliances".to_string()));
    assert_eq!(rooms.admit(power, &thinking(&json!({ "appliances": { "kind": "sorting", "order": ["kettle", "pellets"] } }))), Err("id-unknown /answers/appliances/order/1".to_string()));
    assert_eq!(rooms.admit(power, &thinking(&json!({ "sources": { "kind": "matching", "values": { "price": {} } } }))), Err("id-unknown /answers/sources/values/price".to_string()));
    assert_eq!(rooms.admit(power, &thinking(&json!({ "sources": { "kind": "matching", "values": { "hours": { "kettle": 950 } } } }))), Err("id-unknown /answers/sources/values/hours/kettle".to_string()));
    assert_eq!(rooms.admit(power, &thinking(&json!({ "sources": { "kind": "matching", "values": { "hours": { "rooftop-pv": 10000 } } } }))), Err("value-unknown /answers/sources/values/hours/rooftop-pv".to_string()), "a capacity is no hours card");
    assert_eq!(rooms.admit(power, &thinking(&json!({ "sources": { "kind": "matching", "assignments": { "hours": { "rooftop-pv": 0 } } } }))), Err(STATE_INVALID.to_string()), "card indices mean nothing to peers");
    assert_eq!(rooms.admit(homes, &thinking(&json!({ "systems": { "kind": "classification", "assignments": { "pellets": "coal-oven" } } }))), Err("id-unknown /answers/systems/assignments/pellets".to_string()));
    assert_eq!(rooms.admit(power, &thinking(&json!({ "appliances": { "kind": "sorting", "order": ["kettle", "kettle"] } }))), Err("duplicate-id /answers/appliances/order/1".to_string()), "the core's rules come first");
    assert_eq!(rooms.admit(power, &json!({ "tag": "nope", "answers": {} })), Err("tag-invalid /tag".to_string()));
    assert_eq!(rooms.admit(power, &json!({ "tag": "0a1b2c3d", "cursor": { "anchor": "item:kettle", "x": 0.5, "y": 0.5 } })), Err(STATE_INVALID.to_string()), "a cursor belongs to the quiz room");
}

#[test]
fn a_scope_outside_the_catalog_admits_nothing() {
    let rooms = rooms();
    assert_eq!(rooms.admit("other-catalog", &presence(&json!({ "screen": "home" }))), Err(SCOPE_UNKNOWN.to_string()));
    assert_eq!(rooms.admit("proctor-fixture/quiz/cooling", &json!({ "tag": "0a1b2c3d" })), Err(SCOPE_UNKNOWN.to_string()));
    assert_eq!(rooms.admit("proctor-fixture/quiz/cooling/thinking", &thinking(&json!({}))), Err(SCOPE_UNKNOWN.to_string()));
}
