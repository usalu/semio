//! 🧪️ cursors tests (📇️surface-verbs): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_components::GumballHandle;
use semio_framework_os_infinite_world_cursors::gumball_handle_action_id;
use semio_framework_test_fixtures::fixture_text;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

#[allow(non_snake_case)]
fn SURFACE_VERBS() -> &'static str {
    fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/📇️surface-verbs.json")
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(SURFACE_VERBS()).expect("surface verbs fixture parses")
}

const ALL_HANDLES: [GumballHandle; 12] = [
    GumballHandle::MoveX,
    GumballHandle::MoveY,
    GumballHandle::MoveZ,
    GumballHandle::MoveXY,
    GumballHandle::MoveYZ,
    GumballHandle::MoveXZ,
    GumballHandle::RotateX,
    GumballHandle::RotateY,
    GumballHandle::RotateZ,
    GumballHandle::ScaleX,
    GumballHandle::ScaleY,
    GumballHandle::ScaleZ,
];

#[test]
fn the_handle_verb_mapping_is_the_one_the_fixture_declares() {
    let mapping = fixture()["rule"]["handleVerbs"].clone();
    for handle in ALL_HANDLES {
        let family = if handle.is_translate() {
            "translate"
        } else if handle.is_rotate() {
            "rotate"
        } else {
            "scale"
        };
        assert_eq!(gumball_handle_action_id(handle), mapping[family].as_str().expect("declared verb"), "{handle:?}");
    }
}
