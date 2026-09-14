//! 🌉️ A mesh update never quarantines the surface.
//!
//! 🧫️ Both halves are driven from the committed `🧫️fixtures/🌉️scene-bridge/🔣️.json` payload — the real
//! `s.procedural.generation3d` preview wire, whose `parry3d` oracle independently confirms the solid
//! — and from the DEGENERATE republication `🧫️fixtures/🌉️bridge-empty-build/🔣️.json` derives from it.
//!
//! ⚖️ The defect: `publish_world3d_scene_bridge_snapshot` answers `World3dSnapshotFault::Capacity`
//! when a build yields no pages, and the wgpu host quarantines the surface on ANY bridge fault. A
//! snapshot is at least one page by construction (`world3d_snapshot_begin` refuses `page_count == 0`),
//! so "the guest published nothing drawable this turn" was indistinguishable from "credits
//! exhausted". Measured on 6118 after the third Form slider edit: the guest's intermediate wire was
//! `meshes=169b instances=267b` against `3644b`/`4786b` for every settled sample, and the preview
//! quarantined with `fault=Some(Capacity) state-meshes=3`
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.3).
//! The FIRST build of a surface survived it only by accident — its camera digest is always new, so it
//! always carried a camera page. Every case below pins that accident away.

use super::tests::{drive_scene_bridge, scene_bridge_fixture, scene_from_bridge_fixture, with_world_step_context};
use super::*;

const EMPTY_BUILD: &str = include_str!("../../🧫️fixtures/🌉️bridge-empty-build/🔣️.json");

fn empty_build_fixture() -> serde_json::Value {
    serde_json::from_str(EMPTY_BUILD).expect("bridge empty-build fixture parses")
}

fn bounds() -> Rect {
    Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }
}

/// ♻️ Drives one surface's retirement ladder to terminal. `World3dState` carries no `Drop` under
/// `cfg(test)`, and snapshot slots are a PROCESS-wide fixed store — a test that walks away from a
/// sealed lease starves every other test in the binary, which is exactly what
/// `scene bridge stopped at Fault … (fault Some(Unavailable))` reads like from the other side.
fn retire(mut state: World3dState) {
    begin_world3d_dynamic_retirement(&mut state);
    for turn in 0..8_192 {
        if with_world_step_context(64, |context| step_world3d_dynamic_retirement(&mut state, context)) {
            break;
        }
        assert!(turn < 8_191, "the retirement ladder reaches terminal");
    }
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state), "the surface released every retained owner — {}", state.ingest_census());
}

/// 🌉️ Drives one staged build to a terminal step WITHOUT asserting which one, so the law reads the
/// answer rather than presuming it.
fn drive_bridge_to_terminal(state: &mut World3dState) -> World3dSceneBridgeStep {
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_scene_bridge(state, context)) {
            World3dSceneBridgeStep::Pending => {}
            terminal => return terminal,
        }
        assert!(turn < 4_095, "the bridge terminates within its turn ceiling");
    }
    unreachable!("the bridge terminates")
}

#[test]
fn a_republication_with_nothing_drawable_completes_and_keeps_the_last_geometry() {
    for case in empty_build_fixture()["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let settled = scene_bridge_fixture();
        let mut state = World3dState::new("generation3d-generate-preview".into(), "procedural.play".into());
        let scene = scene_from_bridge_fixture(&settled);
        drive_scene_bridge(&mut state, &scene, bounds());
        let settled_draws = state.draws.len;
        let settled_meshes = state.meshes.len;
        assert!(settled_draws > 0, "{name}: the committed payload publishes draws");

        // 🩸️ The republication the guest emits while it recomputes — same camera, so no camera page
        // can carry the build, which is exactly the shape that used to fault.
        let mut degenerate = scene_from_bridge_fixture(&settled);
        let world = degenerate.world_3d.as_mut().expect("world scene");
        world.meshes_json = case["meshesJson"].to_string();
        world.instances_json = case["instancesJson"].to_string();
        sync_world3d_state(&mut state, &degenerate, bounds());
        let step = drive_bridge_to_terminal(&mut state);

        assert_ne!(step, World3dSceneBridgeStep::Fault, "{name}: a mesh update never faults the bridge — {}", state.ingest_census());
        assert_eq!(state.snapshot_fault(), None, "{name}: and records no snapshot fault");
        assert_eq!(state.draws.len, settled_draws, "{name}: the surface keeps the geometry it last had");
        assert_eq!(state.meshes.len, settled_meshes, "{name}: and keeps its published meshes");
        println!("[DEBUG] bridge-empty-build {name}: step={step:?} {}", state.ingest_census());
        retire(state);
    }
}

#[test]
fn the_same_degenerate_payload_is_never_restaged() {
    let case = empty_build_fixture()["cases"].as_array().expect("cases").first().expect("at least one case").clone();
    let settled = scene_bridge_fixture();
    let mut state = World3dState::new("generation3d-generate-preview".into(), "procedural.play".into());
    let scene = scene_from_bridge_fixture(&settled);
    drive_scene_bridge(&mut state, &scene, bounds());
    let mut degenerate = scene_from_bridge_fixture(&settled);
    let world = degenerate.world_3d.as_mut().expect("world scene");
    world.meshes_json = case["meshesJson"].to_string();
    world.instances_json = case["instancesJson"].to_string();
    sync_world3d_state(&mut state, &degenerate, bounds());
    assert_ne!(drive_bridge_to_terminal(&mut state), World3dSceneBridgeStep::Fault, "the first build completes");
    sync_world3d_state(&mut state, &degenerate, bounds());
    assert_eq!(drive_bridge_to_terminal(&mut state), World3dSceneBridgeStep::Idle, "an unchanged payload stages nothing — {}", state.ingest_census());
    retire(state);
}

#[test]
fn the_settled_payload_returns_after_a_degenerate_one() {
    let case = empty_build_fixture()["cases"].as_array().expect("cases").first().expect("at least one case").clone();
    let settled = scene_bridge_fixture();
    let mut state = World3dState::new("generation3d-generate-preview".into(), "procedural.play".into());
    let scene = scene_from_bridge_fixture(&settled);
    drive_scene_bridge(&mut state, &scene, bounds());
    let mut degenerate = scene_from_bridge_fixture(&settled);
    let world = degenerate.world_3d.as_mut().expect("world scene");
    world.meshes_json = case["meshesJson"].to_string();
    world.instances_json = case["instancesJson"].to_string();
    sync_world3d_state(&mut state, &degenerate, bounds());
    assert_ne!(drive_bridge_to_terminal(&mut state), World3dSceneBridgeStep::Fault, "the degenerate build completes");
    drive_scene_bridge(&mut state, &scene, bounds());
    assert!(state.draws.len > 0, "the recomputed payload paints again — {}", state.ingest_census());
    assert_eq!(state.snapshot_fault(), None, "and the surface never quarantined — {}", state.ingest_census());
    retire(state);
}
