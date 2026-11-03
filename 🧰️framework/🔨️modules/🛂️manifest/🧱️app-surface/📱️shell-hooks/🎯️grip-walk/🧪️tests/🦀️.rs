use super::*;

const FIXTURE: &str = include_str!("../../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎯️grip-focus/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).unwrap()
}

fn grips_of(value: &serde_json::Value) -> Vec<GripRef> {
    value.as_array().unwrap().iter().map(|grip| GripRef { id: grip["id"].as_str().unwrap().to_owned(), entity: grip["entity"].as_str().unwrap().to_owned() }).collect()
}

fn walk_of(name: &str) -> GripWalk {
    match name {
        "forward" => GripWalk::Forward,
        "backward" => GripWalk::Backward,
        "forwardInEntity" => GripWalk::ForwardInEntity,
        "backwardInEntity" => GripWalk::BackwardInEntity,
        other => panic!("unknown walk {other}"),
    }
}

fn step_of(value: &serde_json::Value) -> GripStep {
    match value.as_str() {
        Some("leave") => GripStep::Leave,
        Some("nothing") => GripStep::Nothing,
        _ => GripStep::To(value["to"].as_str().unwrap().to_owned()),
    }
}

#[test]
fn the_walk_answers_every_row_of_the_language_agnostic_fixture() {
    let law = fixture();
    for row in law["walks"].as_array().unwrap() {
        let grips = match row["grips"].as_str() {
            Some(name) => grips_of(&law[name]),
            None => Vec::new(),
        };
        assert_eq!(walk_grips(&grips, row["current"].as_str(), walk_of(row["how"].as_str().unwrap())), step_of(&row["step"]), "{}", row["id"]);
    }
}

#[test]
fn the_registry_keeps_one_focus_per_domain_and_drops_it_when_the_grip_disappears_like_the_fixture_sequence() {
    let law = fixture();
    let domain = |name: &str| format!("framework.test-grip-focus.{name}");
    for (index, op) in law["registry"].as_array().unwrap().iter().enumerate() {
        let name = domain(op["domain"].as_str().unwrap());
        match op["op"].as_str().unwrap() {
            "note" => note_grips(&name, match op["grips"].as_str() {
                Some(key) => grips_of(&law[key]),
                None => grips_of(&op["grips"]),
            }),
            "walk" => assert_eq!(walk_focus(&name, walk_of(op["how"].as_str().unwrap())), step_of(&op["step"]), "step {index}"),
            "focused" => assert_eq!(focused_grip(&name).as_deref(), op["expect"].as_str(), "step {index}"),
            other => panic!("unknown op {other}"),
        }
    }
}
