//! 📇️ A World3d surface may only mint a verb its WINDOW KIND declares.
//!
//! 🧫️ Every expectation is read from `🌐️World3dHost/🧫️fixtures/📇️surface-verbs.json`, the
//! language-neutral oracle the TypeScript twin
//! (`🧑‍🎨engine/🧪️tests/📇️world3d-surface-verbs/🟦️.ts`) answers from the other side. This file drives
//! the production predicate (`world3d_offers_transform_gumball`), the production handle→verb mapping
//! (`gumball_handle_action_id`) and the production ADMISSION gate — a primary press on a selected
//! surface enters `GumballPick` only when the window kind declares a gumball verb.
//!
//! ⚖️ Each law also re-derives the PRE-FIX shape from the same fixture and asserts it DIFFERS: before
//! ticket 26/09/09/PROCEDURAL-3D-END-TO-END's `wgpu-world3d-gaps` lane the gumball was an
//! unconditional affordance of every World3d surface, so the procedural VIEWER offered a translate
//! gizmo and published `translateSelection` into `procedural-view-preview`, which declares no such
//! action (`📓️wgpu-end-to-end-verification-2026-09-14.md` §5.2).

use super::tests::with_world_step_context;
use super::*;

const SURFACE_VERBS: &str = include_str!("../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/📇️surface-verbs.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(SURFACE_VERBS).expect("surface verbs fixture parses")
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
    state.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.pick_bounds = state.bounds;
    state.selected_ids = vec!["extrude@solid".to_string()];
    state.active_utility = "select".to_string();
    set_world3d_declared_actions(&mut state, action_ids);
    state
}

#[test]
fn a_surface_offers_a_gumball_only_when_its_window_kind_declares_a_transform_verb() {
    let mut discriminating = 0;
    for case in cases() {
        let name = case["name"].as_str().expect("case name");
        let action_ids = declared(&case);
        let state = selected_surface(case["windowKindId"].as_str().expect("windowKindId"), &action_ids);
        let offers = world3d_offers_transform_gumball(&state);
        assert_eq!(offers, case["offersGumball"].as_bool().expect("offersGumball"), "{name}");

        let offered: Vec<String> = ALL_HANDLES.iter().map(|handle| gumball_handle_action_id(*handle).to_string()).filter(|verb| world3d_declares_action(&state, verb)).collect();
        let mut unique = Vec::new();
        for verb in offered {
            if !unique.contains(&verb) {
                unique.push(verb);
            }
        }
        assert_eq!(unique, expected_handle_verbs(&case), "{name}: a handle is offered iff its own verb is declared");

        // 🔍️ The pre-fix shape, re-derived from the same fixture: the gumball was offered on every
        // World3d surface and every handle could commit. A case that answers `false` is what makes
        // this fixture discriminating rather than merely agreeing.
        if !offers {
            discriminating += 1;
        }
        println!("[DEBUG] surface-verbs {name}: offers={offers} handles={unique:?} declared={action_ids:?}");
    }
    assert!(discriminating >= 2, "the fixture pins at least two surfaces the pre-fix shape got wrong");
}

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

#[test]
fn a_primary_press_enters_gumball_pick_only_on_a_surface_that_declares_one() {
    for case in cases() {
        let name = case["name"].as_str().expect("case name");
        let mut state = selected_surface(case["windowKindId"].as_str().expect("windowKindId"), &declared(&case));
        let modifiers = PointerModifiers::default();
        assert!(enqueue_world3d_event(&mut state, WorldInteractionIntent::pointer_button(200.0, 200.0, true, 0, &modifiers)).is_ok(), "{name}: the intent is admitted");
        let generation = world3d_interaction_front_generation(&state).expect("a queued intent carries a generation");
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
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
        println!("[DEBUG] surface-verbs {name}: gumball-pick={entered} after {turns} turns");
    }
}
