//! 🖱️ The wgpu World3d authority answers the SAME gesture→action law as the React `World3dHost`.
//!
//! 🧫️ Every expectation is read from `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`, the
//! language-neutral oracle React's own suites already answer (`🧑‍🎨engine/🧪️tests/🖱️world3d-interaction/🟦️.tsx`
//! mounted in jsdom, and `🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`'s React-free Node
//! oracle). This file is the THIRD reader of that file and the first one that drives the wgpu
//! implementation: `WorldRayPickCursor` → `finish_plan` → `publish_world3d_plan_step`, and
//! `WorldMarqueePublishJob` / `plan_world3d_wheel` for the two gestures that do not go through a ray.
//!
//! ⚖️ Each law also re-derives the PRE-FIX shape from the same fixture and asserts it DIFFERS, so the
//! fixture is held to discriminating rather than merely agreeing. Before ticket
//! 26/09/09/PROCEDURAL-3D-END-TO-END's `wgpu-world3d-interaction` lane this surface published
//! `targets` as a JSON ARRAY (the framework decodes it with `DslValue::as_str`), stamped the scene's
//! hover granularity on a pick, sent the RENDER id instead of the topology id, had no `subtractive`
//! merge at all, and addressed `setCamera` with `surfaceId` instead of `windowId`.

use super::tests::{mesh_oracle_from_buffers, publish_oracle_mesh, take_actions, with_world_step_context};
use super::*;

const POINTER_GESTURES: &str = include_str!("../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(POINTER_GESTURES).expect("pointer gesture fixture parses")
}

fn gesture(id: &str) -> serde_json::Value {
    fixture()["gestures"].as_array().expect("gesture list").iter().find(|case| case["id"] == id).unwrap_or_else(|| panic!("fixture case {id}")).clone()
}

/// 🧫️ A `World3dState` carrying the fixture's own scene: its mesh, both rendered instances, their
/// shared topology target and the bound interaction domain.
fn fixture_state() -> World3dState {
    let scene = fixture()["scene"].clone();
    let mut state = World3dState::new(scene["surfaceId"].as_str().expect("surfaceId").into(), scene["controllerId"].as_str().expect("controllerId").into());
    state.bound_domain_id = Some(scene["domainId"].as_str().expect("domainId").into());
    state.bound_domain_granularity_id = Some(scene["domainGranularityId"].as_str().expect("domainGranularityId").into());
    let numbers = |value: &serde_json::Value| value.as_array().expect("number list").iter().map(|entry| entry.as_f64().expect("number") as f32).collect::<Vec<f32>>();
    let mesh = scene["meshes"].as_array().expect("mesh list")[0].clone();
    let data = mesh_oracle_from_buffers(
        numbers(&mesh["data"]["positions"]),
        numbers(&mesh["data"]["normals"]),
        mesh["data"]["indices"].as_array().expect("index list").iter().map(|entry| entry.as_u64().expect("index") as u32).collect(),
    );
    let mesh_key = mesh["id"].as_str().expect("mesh id").to_string();
    store_mesh(&mut state, mesh_key.clone(), publish_oracle_mesh(data));
    let mesh_version = *state.mesh_versions.get(&mesh_key).expect("mesh version");
    let mut instances = Vec::new();
    for instance in scene["instances"].as_array().expect("instance list") {
        let id = instance["id"].as_str().expect("instance id").to_string();
        let target = instance["interactionId"].as_str().expect("instance interactionId").to_string();
        let position = numbers(&instance["position"]);
        let mut model = Mat4::identity();
        model.cols[3][0] = position[0];
        model.cols[3][1] = position[1];
        model.cols[3][2] = position[2];
        state.instance_interaction_ids.insert(id.clone(), target);
        instances.push(Instance3d { id, model, color: [1.0; 4], selected: false, hovered: false });
    }
    state.draws.push(SceneDraw3d { mesh_key, mesh_version, instances });
    state
}

/// 🎯️ A ray straight down onto one rendered instance, in the mesh's own world space — the fixture's
/// quad lies in the `z = 0` plane, so `-Z` from above hits whichever instance the model sits under.
fn instance_ray(state: &World3dState, instance_id: &str) -> (Vec3, Vec3) {
    let instance = state.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == instance_id).expect("rendered instance");
    let translation = instance.model.cols[3];
    (Vec3::new(translation[0], translation[1], translation[2] + 4.0), Vec3::new(0.0, 0.0, -1.0))
}

fn ray_cursor(state: &World3dState, purpose: WorldRayPickPurpose, origin: Vec3, direction: Vec3, merge: u8) -> WorldRayPickCursor {
    WorldRayPickCursor { revision: state.interaction_revision, generation: 1, purpose, origin, direction, draw: 0, instance: 0, triangle: 0, mesh: None, mesh_probe: 0, merge, best: None, complete: false, faulted: false }
}

/// 🏁️ Drives one bounded pick to completion and returns the descriptor it published, if any.
fn publish_pick(state: &mut World3dState, mut cursor: WorldRayPickCursor) -> Option<ActionDescriptor> {
    let mut turns = 0;
    while with_world_step_context(1, |context| cursor.step(state, 1, context)) != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 256, "bounded pick terminates");
    }
    let mut plan = cursor.finish_plan(state, 1).expect("live pick")?;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| publish_world3d_plan_step(state, &mut plan, 1, &mut input, context)).expect("bounded publication") != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 64, "bounded publication terminates");
    }
    take_actions(&mut input).into_iter().next()
}

fn arg<'a>(action: &'a ActionDescriptor, key: &str) -> &'a str {
    action.args.as_ref().and_then(|args| args.get(key)).and_then(|value| value.as_str()).unwrap_or_else(|| panic!("action {} carries a text arg {key}, got {:?}", action.action, action.args))
}

/// 🎯️ The `targets` arg as the framework's own decoder reads it: `DslValue::as_str` then
/// `serde_json`. A structured array answers `as_str` with `None`, which is exactly the defect.
fn decoded_targets(action: &ActionDescriptor) -> serde_json::Value {
    serde_json::from_str(arg(action, "targets")).expect("targets is JSON text")
}

#[test]
fn an_instance_pick_selects_the_topology_target_at_object_granularity() {
    for id in ["instance-pick-replaces", "instance-pick-additive", "instance-pick-subtractive", "instance-pick-subtractive-on-command", "instance-pick-invertive", "second-instance-of-one-topology-id-pick"] {
        let case = gesture(id);
        let expect = case["expect"].clone();
        let mut state = fixture_state();
        let (origin, direction) = instance_ray(&state, case["instanceId"].as_str().expect("instanceId"));
        let merge = world_merge_code(
            case["modifiers"]["shiftKey"].as_bool().unwrap_or(false),
            case["modifiers"]["ctrlKey"].as_bool().unwrap_or(false),
            case["modifiers"]["metaKey"].as_bool().unwrap_or(false),
        );
        let cursor = ray_cursor(&state, WorldRayPickPurpose::Instance, origin, direction, merge);
        let action = publish_pick(&mut state, cursor).unwrap_or_else(|| panic!("{id} publishes an action"));
        assert_eq!(action.action, expect["action"].as_str().expect("action id"), "{id}");
        assert_eq!(arg(&action, "domainId"), expect["domainId"].as_str().expect("domainId"), "{id}");
        assert_eq!(arg(&action, "merge"), expect["merge"].as_str().expect("merge"), "{id}");
        assert_eq!(arg(&action, "method"), expect["method"].as_str().expect("method"), "{id}");
        assert_eq!(decoded_targets(&action), expect["targets"], "{id}");
        println!("[DEBUG] pointer-gestures {id}: targets={} merge={}", arg(&action, "targets"), arg(&action, "merge"));

        // 🔍️ The pre-fix shape, re-derived from the same fixture: the RENDER id at the SCENE
        // granularity, with ctrl spelled `invertive`. It must differ on every case that carries one.
        let render_id = case["instanceId"].as_str().expect("instanceId");
        let topology_id = expect["targets"][0]["id"].as_str().expect("expected target id");
        assert_ne!(render_id, topology_id, "{id}: the fixture's own scene renders the target under a different id");
        assert_ne!(expect["targets"][0]["granularity"].as_str().expect("granularity"), fixture()["scene"]["domainGranularityId"].as_str().expect("scene granularity"), "{id}: a pick's granularity is not the scene's");
    }
}

#[test]
fn an_instance_hover_reports_the_scene_granularity_on_the_pointer_channel() {
    let case = gesture("instance-hover");
    let expect = case["expect"].clone();
    let mut state = fixture_state();
    let (origin, direction) = instance_ray(&state, case["instanceId"].as_str().expect("instanceId"));
    let cursor = ray_cursor(&state, WorldRayPickPurpose::Hover, origin, direction, 0);
    let action = publish_pick(&mut state, cursor).expect("hover publishes an action");
    assert_eq!(action.action, expect["action"].as_str().expect("action id"));
    assert_eq!(arg(&action, "domainId"), expect["domainId"].as_str().expect("domainId"));
    assert_eq!(arg(&action, "channel"), expect["channel"].as_str().expect("channel"));
    assert_eq!(decoded_targets(&action), expect["targets"]);
    assert_eq!(state.local_hover_id.as_deref(), expect["targets"][0]["id"].as_str(), "the surface's own hover marker is the topology target too");
    println!("[DEBUG] pointer-gestures instance-hover: targets={} hover={:?}", arg(&action, "targets"), state.local_hover_id);
}

#[test]
fn a_background_click_clears_with_an_empty_target_list_rather_than_a_silence() {
    let case = gesture("background-click-clears");
    let expect = case["expect"].clone();
    let mut state = fixture_state();
    let cursor = ray_cursor(&state, WorldRayPickPurpose::Instance, Vec3::new(500.0, 500.0, 4.0), Vec3::new(0.0, 0.0, -1.0), 0);
    let action = publish_pick(&mut state, cursor).expect("a background click still dispatches");
    assert_eq!(action.action, expect["action"].as_str().expect("action id"));
    assert_eq!(arg(&action, "merge"), expect["merge"].as_str().expect("merge"));
    assert_eq!(arg(&action, "method"), expect["method"].as_str().expect("method"));
    assert_eq!(decoded_targets(&action), expect["targets"]);
    println!("[DEBUG] pointer-gestures background-click: targets={}", arg(&action, "targets"));
}

#[test]
fn a_marquee_release_replaces_with_the_deduplicated_topology_targets() {
    let case = gesture("marquee-release-replaces");
    let expect = case["expect"].clone();
    let mut state = fixture_state();
    state.interaction_revision = 3;
    state.interaction_objects.revision = 3;
    let mut results = WorldMarqueeResultPages::default();
    for instance in state.draws[0].instances.clone() {
        let token = state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, &instance.id, None, instance.model, [0.0; 8]).expect("marquee object token");
        assert!(results.push(WorldMarqueeResult::Object(token), instance.id.len()), "both rendered instances are admitted");
    }
    let gesture_state = WorldMarqueeGesture::new(3, 1, [0.0, 0.0]);
    let mut job = WorldMarqueePublishJob::new(2, gesture_state, results, false, false);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| job.step(&state, 2, &mut input, context)).expect("bounded marquee publication") != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 128, "bounded marquee publication terminates");
    }
    let action = take_actions(&mut input).into_iter().next().expect("marquee publishes an action");
    assert_eq!(action.action, expect["action"].as_str().expect("action id"));
    assert_eq!(arg(&action, "domainId"), expect["domainId"].as_str().expect("domainId"));
    assert_eq!(arg(&action, "merge"), expect["merge"].as_str().expect("merge"));
    assert_eq!(decoded_targets(&action), expect["targets"], "two rendered instances of ONE channel collapse onto one target");
    println!("[DEBUG] pointer-gestures marquee-release-replaces: targets={} instances={}", arg(&action, "targets"), state.draws[0].instances.len());
}

#[test]
fn a_camera_gesture_addresses_the_window_that_owns_the_surface() {
    let case = gesture("orbit-completes-into-one-setcamera");
    let mut state = fixture_state();
    let mut plan = plan_world3d_wheel(&state, 1, 20.0).expect("bounded wheel plan");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 1, &mut input, context)).expect("bounded publication") != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 64, "bounded publication terminates");
    }
    let action = take_actions(&mut input).into_iter().next().expect("a camera gesture publishes an action");
    assert_eq!(action.action, case["expect"]["action"].as_str().expect("action id"));
    // 🪟️ `windowId`, never `surfaceId`: the shell resolves `ActionAddress::window_instance_id` from
    // this argument, and a World3d surface IS keyed by its window instance id.
    assert_eq!(arg(&action, "windowId"), fixture()["scene"]["surfaceId"].as_str().expect("surfaceId"));
    assert!(action.args.as_ref().and_then(|args| args.get("surfaceId")).is_none(), "the pre-fix `surfaceId` address is gone, not merely joined by `windowId`");
    let camera = action.args.as_ref().and_then(|args| args.get("camera")).expect("the pose nests under `camera`");
    assert!(camera.get("position").is_some() && camera.get("target").is_some() && camera.get("fov").is_some(), "the pose carries position/target/fov");
    println!("[DEBUG] pointer-gestures orbit-completes-into-one-setcamera: windowId={} camera={:?}", arg(&action, "windowId"), camera);
}

/// 🧮️ A marquee page with NO targets still writes the two bytes of `[]`, and its reservation says so.
///
/// ⚖️ `targets` became JSON TEXT, so the array's own punctuation is part of the action's byte
/// credits. `page_credit` charged the delimiters PER TARGET, which is zero for an empty page — and
/// the two-byte shortfall surfaced on the LAST string of the action (`method`), faulting the whole
/// publish. Measured on 6118 as `world3d interaction … leave step=Fault` with
/// `active=MarqueePublish[page=0 stage=5 targets=2b]`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`).
#[test]
fn an_empty_marquee_page_reserves_the_bytes_of_its_own_empty_array() {
    let state = fixture_state();
    let job = WorldMarqueePublishJob::new(2, WorldMarqueeGesture::new(state.interaction_revision, 1, [0.0, 0.0]), WorldMarqueeResultPages::default(), false, false);
    let base = ui_wgpu::wgpu::checked_action_string_bytes(&[
        state.controller_id.as_str(),
        "interactionSelect",
        "domainId",
        resolved_domain_id(&state),
        "targets",
        "merge",
        job.merge,
        "method",
        selection_method_wire_str(SelectionMethod::Rectangle),
    ])
    .expect("base action bytes");
    let credit = job.page_credit(&state, 0).expect("empty page credit");
    assert_eq!(credit - base, INTERACTION_TARGETS_EMPTY.len(), "an empty page reserves exactly the `[]` it writes");
    println!("[DEBUG] pointer-gestures empty-marquee-credit: base={base} credit={credit} array={}", INTERACTION_TARGETS_EMPTY.len());
}

/// 🕹️ The modifier → merge rule, in the ONE vocabulary, read off the fixture's own cases.
#[test]
fn the_merge_vocabulary_is_not_translated() {
    for id in ["instance-pick-replaces", "instance-pick-additive", "instance-pick-subtractive", "instance-pick-subtractive-on-command", "instance-pick-invertive"] {
        let case = gesture(id);
        let code = world_merge_code(
            case["modifiers"]["shiftKey"].as_bool().unwrap_or(false),
            case["modifiers"]["ctrlKey"].as_bool().unwrap_or(false),
            case["modifiers"]["metaKey"].as_bool().unwrap_or(false),
        );
        assert_eq!(world_merge_wire_label(code), case["expect"]["merge"].as_str().expect("merge"), "{id}");
    }
    // 🔍️ The pre-fix rule — `shift → additive, ctrl → INVERTIVE, else replace` — has no
    // `subtractive` at all, so it disagrees on the two ctrl/cmd cases.
    let pre_fix = |shift: bool, ctrl: bool| if shift { MergeMode::Additive } else if ctrl { MergeMode::Invertive } else { MergeMode::Replace };
    assert_ne!(pre_fix(false, true).wire_label(), gesture("instance-pick-subtractive")["expect"]["merge"].as_str().expect("merge"));
    assert_ne!(pre_fix(true, true).wire_label(), gesture("instance-pick-invertive")["expect"]["merge"].as_str().expect("merge"));
}
