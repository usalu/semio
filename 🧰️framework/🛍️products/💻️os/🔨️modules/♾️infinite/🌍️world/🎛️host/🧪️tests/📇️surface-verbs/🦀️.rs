//! 🧪️ host tests (📇️surface-verbs): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::WorldInteractionIntent;
use semio_framework_os_infinite_world_components::{GumballHandle, set_world3d_declared_actions, world3d_declares_action, world3d_offers_transform_gumball};
use semio_framework_os_infinite_world_cursors::{gumball_handle_action_id, WorldInteractionAuthorityStep};
use semio_framework_os_infinite_world_interaction::{enqueue_world3d_event, world3d_interaction_front_generation};
use semio_framework_os_infinite_world_host::{step_world3d_interaction, World3dState};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_wgpu_input::PointerModifiers;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

pub(crate) fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let grant=world_original_fixture_grant();
    let mut receipt=semio_framework_value::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX,grant),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
        &mut receipt,
    );
    let result=step(&mut context);
    assert!(context.retained_progress().fits(grant));
    result
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}

#[allow(non_snake_case)]
fn SURFACE_VERBS() -> &'static str {
    fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/📇️surface-verbs.json")
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(SURFACE_VERBS()).expect("surface verbs fixture parses")
}

fn cases() -> Vec<serde_json::Value> {
    fixture()["cases"].as_array().expect("fixture declares cases").clone()
}

fn declared(case: &serde_json::Value) -> Vec<String> {
    case["declaredActionIds"].as_array().expect("declaredActionIds").iter().map(|value| value.as_str().expect("action id").to_string()).collect()
}

fn expected_handle_verbs(case: &serde_json::Value) -> Vec<String> {
    case["offeredHandleVerbs"].as_array().expect("offeredHandleVerbs").iter().map(|value| value.as_str().expect("verb").to_string()).collect()
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

/// 🧫️ A surface with one selected object, the state a gumball is offered on.
fn selected_surface(window_kind_id: &str, action_ids: &[String]) -> World3dState {
    let mut state = World3dState::new(window_kind_id.to_string(), "procedural.play".to_string());
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = state.session.bounds;
    state.selection.selected_ids = vec!["extrude@solid".to_string()];
    state.session.active_utility = "select".to_string();
    set_world3d_declared_actions(&mut state.session, action_ids);
    state
}

/// 🔍️ The pre-fix shape, re-derived from the same fixture: the gumball was offered on every
/// World3d surface and every handle could commit. A case that answers `false` is what makes
/// this fixture discriminating rather than merely agreeing.
#[test]
fn a_surface_offers_a_gumball_only_when_its_window_kind_declares_a_transform_verb() {
    let mut discriminating = 0;
    for case in cases() {
        let name = case["name"].as_str().expect("case name");
        let action_ids = declared(&case);
        let state = selected_surface(case["windowKindId"].as_str().expect("windowKindId"), &action_ids);
        let offers = world3d_offers_transform_gumball(&state.session);
        assert_eq!(offers, case["offersGumball"].as_bool().expect("offersGumball"), "{name}");

        let offered: Vec<String> = ALL_HANDLES.iter().map(|handle| gumball_handle_action_id(*handle).to_string()).filter(|verb| world3d_declares_action(&state.session, verb)).collect();
        let mut unique = Vec::new();
        for verb in offered {
            if !unique.contains(&verb) {
                unique.push(verb);
            }
        }
        assert_eq!(unique, expected_handle_verbs(&case), "{name}: a handle is offered iff its own verb is declared");

        if !offers {
            discriminating += 1;
        }
    }
    assert!(discriminating >= 2, "the fixture pins at least two surfaces the pre-fix shape got wrong");
}

#[test]
fn a_primary_press_enters_gumball_pick_only_on_a_surface_that_declares_one() {
    for case in cases() {
        let name = case["name"].as_str().expect("case name");
        let mut state = selected_surface(case["windowKindId"].as_str().expect("windowKindId"), &declared(&case));
        let modifiers = PointerModifiers::default();
        assert!(enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_button(200.0, 200.0, true, 0, &modifiers)).is_ok(), "{name}: the intent is admitted");
        let generation = world3d_interaction_front_generation(&state.camera, &state.authority).expect("a queued intent carries a generation");
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        let mut turns = 0;
        while !state.interaction_census().contains("active=GumballPick") {
            let step = with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context));
            turns += 1;
            assert!(turns < 256, "{name}: the authority terminates");
            if matches!(step, WorldInteractionAuthorityStep::Complete | WorldInteractionAuthorityStep::Idle | WorldInteractionAuthorityStep::Stale) {
                break;
            }
            assert_ne!(step, WorldInteractionAuthorityStep::Fault, "{name}: the authority never faults");
        }
        let entered = state.interaction_census().contains("active=GumballPick");
        assert_eq!(entered, case["offersGumball"].as_bool().expect("offersGumball"), "{name}: {}", state.interaction_census());
    }
}
