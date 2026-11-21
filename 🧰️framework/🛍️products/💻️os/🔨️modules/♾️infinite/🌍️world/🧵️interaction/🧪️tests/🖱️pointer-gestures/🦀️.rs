//! 🧪️ interaction tests (🖱️pointer-gestures): compiled against the `testkit` surface of the world crates.

use semio_os_infinite_world_components::World3dRelocateSession;
use semio_os_infinite_world_cursors::world_merge_wire_label;
use semio_os_infinite_world_interaction::{world3d_relocate_dispatch_args, world3d_relocate_drag_target, world_merge_code};
use semio_framework::MergeMode;
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
fn POINTER_GESTURES() -> &'static str {
    fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🖱️pointer-gestures.json")
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(POINTER_GESTURES()).expect("pointer gesture fixture parses")
}

fn gesture(id: &str) -> serde_json::Value {
    fixture()["gestures"].as_array().expect("gesture list").iter().find(|case| case["id"] == id).unwrap_or_else(|| panic!("fixture case {id}")).clone()
}

/// 🕹️ The modifier → merge rule, in the ONE vocabulary, read off the fixture's own cases.
///
/// 🔍️ The pre-fix rule — `shift → additive, ctrl → INVERTIVE, else replace` — has no
/// `subtractive` at all, so it disagrees on the two ctrl/cmd cases.
#[test]
fn the_merge_vocabulary_is_not_translated() {
    for id in ["instance-pick-replaces", "instance-pick-additive", "instance-pick-subtractive", "instance-pick-subtractive-on-command", "instance-pick-invertive"] {
        let case = gesture(id);
        let code = world_merge_code(case["modifiers"]["shiftKey"].as_bool().unwrap_or(false), case["modifiers"]["ctrlKey"].as_bool().unwrap_or(false), case["modifiers"]["metaKey"].as_bool().unwrap_or(false));
        assert_eq!(world_merge_wire_label(code), case["expect"]["merge"].as_str().expect("merge"), "{id}");
    }
    let pre_fix = |shift: bool, ctrl: bool| {
        if shift {
            MergeMode::Additive
        } else if ctrl {
            MergeMode::Invertive
        } else {
            MergeMode::Replace
        }
    };
    assert_ne!(pre_fix(false, true).wire_label(), gesture("instance-pick-subtractive")["expect"]["merge"].as_str().expect("merge"));
    assert_ne!(pre_fix(true, true).wire_label(), gesture("instance-pick-invertive")["expect"]["merge"].as_str().expect("merge"));
}

//#region WorldRelocateGestureLaws
/// 🚚️ React `🌐️World3dHost/🟦️.tsx` `world3dRelocateDragTargetV1`: a press on empty canvas grabs the
/// first SELECTED object; a press with no selection grabs what it landed on; a press inside the
/// selection grabs that object; a press on something OUTSIDE the selection grabs nothing, because
/// that gesture is a selection change and `clearSelection` stays the way to drop a selection.
#[test]
fn a_relocate_press_grabs_the_object_react_grabs() {
    let selection = vec!["a".to_string(), "b".to_string()];
    assert_eq!(world3d_relocate_drag_target(None, &selection).as_deref(), Some("a"), "an empty-canvas press relocates the first selected object");
    assert_eq!(world3d_relocate_drag_target(Some("c"), &[]).as_deref(), Some("c"), "with no selection the pressed object is the target");
    assert_eq!(world3d_relocate_drag_target(Some("b"), &selection).as_deref(), Some("b"), "a press inside the selection relocates that object");
    assert_eq!(world3d_relocate_drag_target(Some("c"), &selection), None, "a press outside the selection relocates nothing");
    assert_eq!(world3d_relocate_drag_target(None, &[]), None, "nothing pressed and nothing selected grabs nothing");
}

/// 🚚️ React `world3dRelocateDispatchArgsV1`: the payload is the grabbed object's ORIGIN plus the
/// ground travel `from → to`, grid-snapped — never an incremental pose delta — and a drag that lands
/// within `GUMBALL_TRANSFORM_EPSILON` of where it started answers `null`, so a click without travel
/// writes no document edit.
#[test]
fn a_relocate_commit_is_the_origin_plus_the_ground_travel_and_a_still_drag_commits_nothing() {
    let session = World3dRelocateSession { object_id: "o1".into(), origin: [10.0, 20.0, 5.0], from: [0.0, 0.0, 5.0] };
    let moved = world3d_relocate_dispatch_args(&session, [3.0, -4.0, 5.0], false, 1.0).expect("a travelled drag commits");
    assert_eq!(moved, [13.0, 16.0, 5.0], "origin + (to - from), with the grabbed object's own z preserved");
    assert_eq!(world3d_relocate_dispatch_args(&session, [0.0, 0.0, 5.0], false, 1.0), None, "a drag that never travelled commits nothing");
    let snapped = world3d_relocate_dispatch_args(&session, [3.4, -4.4, 5.0], true, 1.0).expect("a travelled drag commits");
    assert_eq!(snapped, [13.0, 16.0, 5.0], "the committed point is snapped exactly like a catalogue drop");
}

#[test]
fn authored_engagement_projects_and_publishes_bounded_pointer_commands() {
    use semio_os_infinite_world_bridge::sync_world3d_scene_selection;
    use semio_os_infinite_world_components::{WorldCameraState, WorldGumballState, WorldSceneState, WorldSelectionState};
    use semio_os_infinite_world_cursors::{plan_world3d_engagement_pointer, publish_world3d_engagement_pointer};
    use semio_os_infinite_world_model::{WorldFlatActionKind, WorldInteractionIntent, WorldSessionState};
    use semio_ui_theme::geometry::Rect;
    use semio_ui_wgpu_input::{InputState, PointerModifiers};

    let law = fixture()["engagement"].clone();
    let surface = law["surfaceId"].as_str().unwrap();
    let window = law["windowId"].as_str().unwrap();
    let commands = law["commands"].as_array().unwrap();
    let mut session = WorldSessionState::new(surface.into(), "controller".into());
    session.window_instance_id = Some(window.into());
    session.bounds = Rect::new(0.0, 0.0, 400.0, 400.0);
    session.pick_bounds = session.bounds;
    session.declared_action_ids = commands.iter().map(|row| row["action"].as_str().unwrap().into()).collect();
    let camera = WorldCameraState::new();
    let mut selection = WorldSelectionState::new();
    let mut gumball = WorldGumballState::new();
    let mut scene = WorldSceneState::new();
    for case in law["selectionCases"].as_array().unwrap() {
        sync_world3d_scene_selection(&mut session, &mut selection, &mut gumball, &mut scene, &case["wire"].to_string());
        let active = case["active"].as_bool().unwrap();
        assert_eq!(selection.engagement_session_active, active);
        for command in commands {
            let press = command["phase"] == "press";
            let modifiers = PointerModifiers { shift: true, meta: true, ..Default::default() };
            let intent = if press {
                WorldInteractionIntent::pointer_button(200.0, 200.0, true, 0, &modifiers)
            } else {
                WorldInteractionIntent::pointer_move(200.0, 200.0, 0.0, 0.0, false, 0, &modifiers)
            };
            let plan = plan_world3d_engagement_pointer(&session, &camera, &intent).expect("declared engagement command");
            let action = plan.actions[0].unwrap();
            let kind = if press { WorldFlatActionKind::EngagementPointerDown } else { WorldFlatActionKind::EngagementPointerMove };
            assert_eq!(action.kind, kind);
            let mut input = InputState::default();
            assert_eq!(publish_world3d_engagement_pointer(&session, &selection, &plan, action, &mut input).unwrap(), active);
            let published = input.take_action_step().expect("bounded input authority");
            assert_eq!(published.is_some(), active);
            if let Some(published) = published {
                let descriptor = published.into_descriptor().unwrap();
                assert_eq!(descriptor.action, command["action"].as_str().unwrap());
                let args = descriptor.args.unwrap();
                for key in ["surfaceId", "windowId", "pane"] {
                    assert_eq!(args.get(key).and_then(semio_framework::DslValue::as_str), law[key].as_str());
                }
                let point = args.get("position").and_then(semio_framework::DslValue::as_array).unwrap();
                assert_eq!(point.len(), 3);
                for (value, expected) in point.iter().zip(law["position"].as_array().unwrap()) {
                    assert!((value.as_f64().unwrap() - expected.as_f64().unwrap()).abs() < 1e-5);
                }
                for key in ["shiftKey", "ctrlKey", "metaKey"] {
                    assert_eq!(args.get(key).and_then(semio_framework::DslValue::as_bool), command["modifiers"][key].as_bool());
                }
            }
        }
    }
}

#[test]
fn only_unstarted_engagement_hover_can_fold_into_the_queue_front() {
    use semio_os_infinite_world_model::{WorldInteractionIntent, WorldInteractionIntentQueue};
    use semio_ui_wgpu_input::PointerModifiers;

    let sample = fixture()["engagement"]["queue"].clone();
    let point = |key: &str| [sample[key][0].as_f64().unwrap() as f32, sample[key][1].as_f64().unwrap() as f32];
    let [x, y] = point("first");
    let [next_x, next_y] = point("latest");
    let modifiers = PointerModifiers::default();
    let mut first = WorldInteractionIntent::pointer_move(x, y, 0.0, 0.0, false, 0, &modifiers);
    first.generation = sample["generation"].as_u64().unwrap();
    let latest = WorldInteractionIntent::pointer_move(next_x, next_y, next_x - x, next_y - y, false, 0, &modifiers);
    let mut queue = WorldInteractionIntentQueue::default();
    queue.push(first).unwrap();
    assert_eq!(queue.coalesce_tail_pointer_move(&latest, false), None);
    assert_eq!(queue.coalesce_tail_pointer_move(&latest, true), Some(first.generation));
    let front = queue.front().unwrap();
    assert_eq!([front.x, front.y], [next_x, next_y]);
    assert_eq!([front.dx, front.dy], [next_x - x, next_y - y]);
    assert_eq!(queue.len, 1);
}
