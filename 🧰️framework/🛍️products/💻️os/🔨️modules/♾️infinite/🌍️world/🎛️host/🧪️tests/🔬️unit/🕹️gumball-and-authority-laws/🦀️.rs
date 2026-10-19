/// 🕹️ Laws of the gumball and brush jobs, the wheel, drag and paint plans, the ray pick cursor and the authority.

#[test]
fn world_gumball_cursor_caps_selected_tokens_and_closes_one_owner_per_grant() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 9;
    state.interaction.interaction_objects.revision = 9;
    for index in 0..=WORLD_GUMBALL_SELECTED_CAPACITY {
        let mut model = Mat4::identity();
        model.cols[3][0] = index as f32;
        assert!(state.interaction.interaction_objects.admit(9, WorldInteractionObjectKind::Instance, &format!("selected-{index}"), None, model, [0.0, 0.0, 1.0, index as f32, 0.0, 0.0, 0.0, 0.0], None).is_some());
    }
    let mut cursor = WorldGumballPickCursor::new(&state.session, &state.selection, 12, 0.0, 0.0);
    for _ in 0..WORLD_INTERACTION_OBJECT_CAPACITY + 1 {
        let step = with_world_step_context(1, |context| cursor.step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene, 12, context));
        if step == WorldInteractionStep::Fault {
            break;
        }
    }
    assert!(cursor.faulted);
    assert_eq!(cursor.selected_len as usize, WORLD_GUMBALL_SELECTED_CAPACITY);
    let mut turns = 0;
    while !cursor.close_step() {
        turns += 1;
        assert!(turns <= WORLD_GUMBALL_SELECTED_CAPACITY + 1);
    }
    assert_eq!(turns, WORLD_GUMBALL_SELECTED_CAPACITY);
    assert!(cursor.selected.iter().all(Option::is_none));
}

#[test]
fn world_gumball_update_validates_one_selected_aba_token_per_turn() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 3;
    state.interaction.interaction_objects.revision = 3;
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 100.0, h: 100.0 };
    let token = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, Mat4::identity(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], None).expect("selected token");
    let mut selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
    selected[0] = Some(WorldGumballTarget::new(token, None, "selected").expect("original selected target"));
    let mut gesture = WorldGumballGesture {
        revision: 3,
        start_generation: 5,
        handle: GumballHandle::MoveX,
        pivot: Vec3::ZERO,
        anchor: 0.0,
        start: Vec3::ZERO,
        translate: Vec3::ZERO,
        angle: 0.0,
        scale: Vec3::new(1.0, 1.0, 1.0),
        mode: None,
        selected,
        selected_len: 1,
        selected_bytes: "selected".len() as u16,
        validation: 0,
        pending: None,
        live: false,
        streamed: false,
        sent_translate: Vec3::ZERO,
        sent_angle: 0.0,
        sent_scale: Vec3::new(1.0, 1.0, 1.0),
    };
    assert_eq!(gesture.begin_update(6, 50.0, 50.0), WorldInteractionStep::Pending);
    assert_eq!(gesture.update_step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene), WorldInteractionStep::Pending);
    let mut replacement = Mat4::identity();
    replacement.cols[3][0] = 1.0;
    let next = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, replacement, [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0], None).expect("replacement token");
    assert_ne!(next, token);
    gesture.validation = 0;
    assert_eq!(gesture.update_step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene), WorldInteractionStep::Stale);
    assert!(!gesture.close_step());
    assert!(!gesture.close_step());
    assert!(gesture.close_step());
}

fn world_gumball_commit_host_snapshot() -> (World3dState, WorldGumballGesture) {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 3;
    state.interaction.interaction_objects.revision = 3;
    let token = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, Mat4::identity(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], None).expect("selected token");
    let mut selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
    selected[0] = Some(WorldGumballTarget::new(token, None, "selected").expect("original selected target"));
    let gesture = WorldGumballGesture {
        revision: 3,
        start_generation: 5,
        handle: GumballHandle::MoveX,
        pivot: Vec3::ZERO,
        anchor: 0.0,
        start: Vec3::ZERO,
        translate: Vec3::new(2.0, 0.0, 0.0),
        angle: 0.0,
        scale: Vec3::new(1.0, 1.0, 1.0),
        mode: None,
        selected,
        selected_len: 1,
        selected_bytes: "selected".len() as u16,
        validation: 0,
        pending: None,
        live: false,
        streamed: false,
        sent_translate: Vec3::ZERO,
        sent_angle: 0.0,
        sent_scale: Vec3::new(1.0, 1.0, 1.0),
    };
    (state, gesture)
}

/// 🪟️ A transform verb is window-owned: it addresses its own window instance rather than
/// whatever has focus — see `WorldGumballCommitJob::step`'s `windowId` stage.
#[test]
fn world_gumball_commit_builds_one_flat_node_per_grant_then_retires_tokens() {
    let (state, gesture) = world_gumball_commit_host_snapshot();
    let mut job = WorldGumballCommitJob::new(8, gesture, WorldGumballPhase::Once);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(0, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).expect("bounded gumball commit");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 32);
    }
    assert!(job.terminal_is_empty());
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "translateSelection");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("windowId")).and_then(|value| value.as_str()), Some(state.session.surface_id.as_str()));
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(|value| value.as_str()), Some(state.session.surface_id.as_str()));
    assert!(turns > 8);
}

/// 🪟️ Five staged nodes precede the first selected-id resolve: `{`, `surfaceId`, `windowId`,
/// `mode`, `ids[`.
#[test]
fn world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority() {
    let (mut state, gesture) = world_gumball_commit_host_snapshot();
    let mut job = WorldGumballCommitJob::new(8, gesture, WorldGumballPhase::Once);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut blockers = Vec::new();
    while let Ok(claim) = input.claim_action(1) {
        blockers.push(claim);
    }
    assert!(matches!(with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)), Err(semio_framework_ui_action::BoundedActionFault::ItemCredits)));
    assert!(job.claim.is_none());
    input.release_action_claim(blockers.pop().expect("retry credit")).expect("release retry credit");
    assert_eq!(with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(job.claim.is_some());
    assert_eq!(with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(job.draft.is_some());
    let mut replacement = Mat4::identity();
    replacement.cols[3][0] = 1.0;
    state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, replacement, [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0], None).expect("ABA replacement");
    for _ in 0..5 {
        let _ = with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context));
    }
    assert!(matches!(with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)), Err(semio_framework_ui_action::BoundedActionFault::Structure)));
    let mut close_turns = 0;
    while !job.close_step(&mut input) {
        close_turns += 1;
        assert!(close_turns < 8);
    }
    for claim in blockers {
        input.release_action_claim(claim).expect("release blocker");
    }
    assert!(take_actions(&mut input).is_empty());
    assert_eq!(job.gesture.selected_len, 0);
    assert_eq!(job.gesture.selected_bytes, 0);
}

#[test]
fn world_gumball_fixed_gesture_projects_preview_without_mutating_source_draw() {
    let (mut state, gesture) = world_gumball_commit_host_snapshot();
    state.authority.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    let source = Mat4::identity();
    let preview = retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source);
    assert_eq!(source.cols[3], [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(preview.cols[3], [2.0, 0.0, 0.0, 1.0]);
    let unmatched = retained_gumball_preview_model(&state.interaction, &state.authority, 0, 1, source);
    assert_eq!(unmatched.cols, source.cols);
}

/// 📨️ Drives one gumball publication job to completion and answers what it published (nothing for a `Skip`).
fn world_gumball_publish(state: &World3dState, gesture: WorldGumballGesture, phase: WorldGumballPhase) -> Option<ActionDescriptor> {
    let mut job = WorldGumballCommitJob::new(8, gesture, phase);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    for _ in 0..4096 {
        if with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).expect("bounded gumball publication") == WorldInteractionStep::Complete {
            break;
        }
    }
    assert!(job.terminal_is_empty(), "the publication retires its tokens");
    let mut actions = take_actions(&mut input);
    assert!(actions.len() <= 1, "one publication is at most one dispatch");
    actions.pop()
}

/// 🛠️ LAW (`🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`, the protocol React's `worldGumballStep` answers
/// step by step): the wgpu gesture publishes every case's terminal dispatch — the local ONE net delta, the live commit
/// tail (the identity when it moved nothing), the abort with its reason, or nothing — exactly as the fixture states it,
/// never streams a local gesture, ignores a cancel before the first tick, swallows the drag after an abort, and its stream
/// ticks (one per completed pose update; the intent queue already coalesces moves, so there is no in-flight hold) sum to
/// exactly the motion React streamed.
#[test]
fn world_gumball_gesture_answers_the_shared_live_protocol() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🛠️gumball-live-protocol.json")).expect("the protocol fixture parses");
    let vector = |value: &serde_json::Value| Vec3::new(value[0].as_f64().expect("x") as f32, value[1].as_f64().expect("y") as f32, value[2].as_f64().expect("z") as f32);
    let handle = |name: &str| match name {
        "moveX" => GumballHandle::MoveX,
        "rotateZ" => GumballHandle::RotateZ,
        "scaleUniform" => GumballHandle::ScaleX,
        other => panic!("no wgpu handle for fixture handle {other}"),
    };
    let motion_keys = ["dx", "dy", "dz", "ax", "ay", "az", "angle", "sx", "sy", "sz"];
    let number = |args: &serde_json::Value, key: &str| args.get(key).and_then(serde_json::Value::as_f64);
    let mut cases = 0;
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let live = case["live"].as_bool().expect("live");
        let grabbed = !case["targets"]["ids"].as_array().expect("ids").is_empty();
        let (state, template) = world_gumball_commit_host_snapshot();
        let mut gesture: Option<WorldGumballGesture> = None;
        let mut swallowed = false;
        let (mut start_translate, mut start_scale) = (Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0));
        let mut published: Vec<ActionDescriptor> = Vec::new();
        let mut expected: Vec<serde_json::Value> = Vec::new();
        for step in case["steps"].as_array().expect("steps") {
            if !step["dispatch"].is_null() {
                expected.push(step["dispatch"].clone());
            }
            let event = &step["event"];
            let pose = |key: &str| (vector(&event[key]["position"]), vector(&event[key]["scale"]));
            match event["kind"].as_str().expect("event kind") {
                "start" => {
                    let (translate, scale) = pose("pose");
                    (start_translate, start_scale) = (translate, scale);
                    swallowed = false;
                    let mut grab = template.publication(world_gumball_identity_motion());
                    grab.handle = handle(event["handle"].as_str().expect("handle"));
                    grab.live = live;
                    grab.streamed = false;
                    if !grabbed {
                        grab.selected_len = 0;
                        grab.selected_bytes = 0;
                        grab.selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
                    }
                    gesture = Some(grab);
                }
                "drag" if !swallowed => {
                    let (translate, scale) = pose("pose");
                    let current = gesture.as_mut().expect("a drag after its grab");
                    current.translate = translate.sub(start_translate);
                    current.scale = Vec3::new(scale.x / start_scale.x, scale.y / start_scale.y, scale.z / start_scale.z);
                    if let Some((phase, motion)) = current.stream() {
                        published.extend(world_gumball_publish(&state, current.publication(motion), phase));
                    }
                }
                "release" if !swallowed => {
                    let (translate, scale) = pose("after");
                    let mut released = gesture.take().expect("a release after its grab");
                    released.translate = translate.sub(start_translate);
                    released.scale = Vec3::new(scale.x / start_scale.x, scale.y / start_scale.y, scale.z / start_scale.z);
                    let (phase, (translate, angle, scale)) = released.release();
                    published.extend(world_gumball_publish(&state, WorldGumballGesture { translate, angle, scale, ..released }, phase));
                }
                "release" => swallowed = false,
                "cancel" => {
                    let reason = if event["reason"].as_str() == Some("blur") { WorldCancelReason::Blur } else { WorldCancelReason::CaptureLost };
                    if let Some((phase, (translate, angle, scale))) = gesture.as_ref().and_then(|current| current.cancel(reason)) {
                        let cancelled = gesture.take().expect("the cancelled gesture");
                        swallowed = true;
                        published.extend(world_gumball_publish(&state, WorldGumballGesture { translate, angle, scale, ..cancelled }, phase));
                    }
                }
                _ => {}
            }
        }
        let terminal = |dispatches: &mut Vec<serde_json::Value>| dispatches.last().filter(|dispatch| dispatch["args"]["phase"].as_str() != Some("stream")).cloned().inspect(|_| {
            dispatches.pop();
        });
        let expected_terminal = terminal(&mut expected);
        let mut actual: Vec<serde_json::Value> = published.iter().map(|action| serde_json::json!({ "action": action.action, "args": serde_json::to_value(action.args.as_ref().expect("args")).expect("args as JSON") })).collect();
        let actual_terminal = terminal(&mut actual);
        match (&expected_terminal, &actual_terminal) {
            (None, None) => {}
            (Some(expected), Some(actual)) => {
                assert_eq!(actual["action"], expected["action"], "{name}: the terminal verb");
                for key in motion_keys.iter().filter(|key| expected["args"].get(**key).is_some()) {
                    assert_eq!(number(&actual["args"], key), number(&expected["args"], key), "{name}: terminal {key}");
                }
                assert_eq!(actual["args"].get("phase"), expected["args"].get("phase"), "{name}: terminal phase");
                assert_eq!(actual["args"].get("reason"), expected["args"].get("reason"), "{name}: terminal reason");
            }
            _ => panic!("{name}: terminal dispatch {actual_terminal:?} where the protocol states {expected_terminal:?}"),
        }
        assert!(live || actual.is_empty(), "{name}: a local gesture never streams");
        let summed = |dispatches: &[serde_json::Value], key: &str, product: bool| dispatches.iter().filter_map(|dispatch| number(&dispatch["args"], key)).fold(if product { 1.0 } else { 0.0 }, |total, value| if product { total * value } else { total + value });
        for (key, product) in [("dx", false), ("dy", false), ("dz", false), ("sx", true), ("sy", true), ("sz", true)] {
            assert!((summed(&actual, key, product) - summed(&expected, key, product)).abs() < 1e-6, "{name}: the stream ticks carry the streamed {key}");
        }
        assert!(actual.iter().all(|dispatch| dispatch["args"]["phase"].as_str() == Some("stream")), "{name}: only ticks stream");
        cases += 1;
    }
    assert!(cases >= 10, "every protocol case is consumed: {cases}");
}

/// 🎚️ A turn or a scale previews IN PLACE, exactly as the release commits it (every 3D gumball leaf turns and
/// scales each target about its own origin): an instance far from the gumball pivot keeps its translation while its
/// basis turns about the handle axis, and a scale stretches only its own local axis — the selection never orbits
/// the pivot, so a multi-object preview equals the committed state.
#[test]
fn world_gumball_turn_and_scale_preview_each_instance_in_place() {
    let (mut state, mut gesture) = world_gumball_commit_host_snapshot();
    let mut source = Mat4::identity();
    source.cols[3] = [5.0, 0.0, 0.0, 1.0];
    gesture.translate = Vec3::ZERO;
    gesture.handle = GumballHandle::RotateZ;
    gesture.angle = std::f32::consts::FRAC_PI_2;
    state.authority.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    let turned = retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source);
    assert_eq!(turned.cols[3], [5.0, 0.0, 0.0, 1.0], "a turned instance keeps its own origin; it never orbits the pivot at the world origin");
    let near = |column: [f32; 4], expected: [f32; 3]| (0..3).all(|axis| (column[axis] - expected[axis]).abs() < 1e-6);
    assert!(near(turned.cols[0], [0.0, 1.0, 0.0]) && near(turned.cols[1], [-1.0, 0.0, 0.0]) && near(turned.cols[2], [0.0, 0.0, 1.0]), "the basis turns a quarter about +z: {:?}", turned.cols);
    let gesture = state.authority.interaction_authority.as_mut().unwrap().gumball.as_mut().unwrap();
    gesture.handle = GumballHandle::ScaleY;
    gesture.angle = 0.0;
    gesture.scale = Vec3::new(1.0, 3.0, 1.0);
    let scaled = retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source);
    assert_eq!((scaled.cols[0], scaled.cols[1], scaled.cols[2], scaled.cols[3]), ([1.0, 0.0, 0.0, 0.0], [0.0, 3.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [5.0, 0.0, 0.0, 1.0]), "a scale stretches the instance's own y axis and keeps its origin");
}

fn world_brush_commit_host_snapshot(target: String) -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 4;
    state.gumball.brush_preview = Some(WorldBrushPreviewRecord {
        target_vortex_full_id: Some(target),
        object_kind_id: Some("kind".into()),
        source_vortex_index: Some(7),
        origin: Some([1.0, 2.0, 3.0]),
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: Some(serde_json::json!([1.0, 2.0, 3.0])),
        ..Default::default()
    });
    state
}

#[test]
fn world_brush_commit_copies_and_revalidates_fixed_chunks_before_claimed_publication() {
    let state = world_brush_commit_host_snapshot("v".repeat(WORLD_BRUSH_COPY_CHUNK_BYTES * 2 + 1));
    let mut job = WorldBrushCommitJob::new(&state.session, &state.gumball, 9).unwrap().expect("brush job");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state.session, &state.gumball, 9, &mut input, context)).expect("bounded brush step");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 64);
    }
    assert!(turns > 20);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "addBrushObject");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("targetVortexFullId")).and_then(semio_framework_value::DslValue::as_str).map(str::len), Some(WORLD_BRUSH_COPY_CHUNK_BYTES * 2 + 1));
}

#[test]
fn world_brush_validation_detects_same_length_replacement_and_close_releases_draft_claim() {
    let mut state = world_brush_commit_host_snapshot("first".into());
    let mut stale = WorldBrushCommitJob::new(&state.session, &state.gumball, 9).unwrap().expect("brush job");
    while !stale.validating {
        assert_eq!(with_world_step_context(1, |context| stale.step(&state.session, &state.gumball, 9, &mut semio_framework_ui_wgpu_input::InputState::default(), context)).unwrap(), WorldInteractionStep::Pending);
    }
    state.gumball.brush_preview.as_mut().unwrap().target_vortex_full_id = Some("other".into());
    assert!(matches!(with_world_step_context(1, |context| stale.step(&state.session, &state.gumball, 9, &mut semio_framework_ui_wgpu_input::InputState::default(), context)), Err(semio_framework_ui_action::BoundedActionFault::Structure)));

    let state = world_brush_commit_host_snapshot("target".into());
    let mut interrupted = WorldBrushCommitJob::new(&state.session, &state.gumball, 10).unwrap().expect("brush job");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    while !interrupted.complete {
        let _ = with_world_step_context(1, |context| interrupted.step(&state.session, &state.gumball, 10, &mut input, context));
    }
    assert_eq!(with_world_step_context(1, |context| interrupted.step(&state.session, &state.gumball, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(interrupted.claim.is_some());
    assert_eq!(with_world_step_context(1, |context| interrupted.step(&state.session, &state.gumball, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(interrupted.draft.is_some());
    assert!(!interrupted.close_step(&mut input));
    assert!(!interrupted.close_step(&mut input));
    assert!(interrupted.close_step(&mut input));
    assert!(take_actions(&mut input).is_empty());
}

fn world_intent(generation: u64, delta: f32) -> WorldInteractionIntent {
    WorldInteractionIntent { phase: WorldInteractionPhase::Wheel, generation, x: 0.0, y: 0.0, dx: 0.0, dy: 0.0, delta, button: 0, down: false, shift: false, ctrl: false, alt: false, meta: false }
}

/// 🧭️ A navigation step moves the orbit and publishes NOTHING: the wire report is the debounced
/// settle's, one per gesture (see `WorldCameraSync`).
#[test]
fn world_wheel_plan_revalidates_before_mutation_and_publishes_flat_action() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let original_distance = state.camera.orbit.distance;
    let mut stale = plan_world3d_wheel(&state.session, &state.camera, 7, 20.0).expect("bounded wheel plan");
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let stale_step = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut stale, 7, &mut input, context)).unwrap();
    assert_eq!(stale_step, WorldInteractionStep::Stale);
    assert_eq!(state.camera.orbit.distance, original_distance);
    assert!(take_actions(&mut input).is_empty());

    let mut plan = plan_world3d_wheel(&state.session, &state.camera, 8, 200.0).expect("bounded wheel plan");
    let pending = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 8, &mut input, context)).unwrap();
    assert_eq!(pending, WorldInteractionStep::Pending);
    assert_ne!(state.camera.orbit.distance, original_distance);
    assert!(take_actions(&mut input).is_empty());
    let complete = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 8, &mut input, context)).unwrap();
    assert_eq!(complete, WorldInteractionStep::Complete);
    assert!(plan.terminal_is_empty());

    let mut settle = plan_world3d_camera_settle(&state.session, &state.camera, 9).expect("a moved camera owes a settle");
    let mut turns = 0;
    while with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut settle, 9, &mut input, context)).unwrap() != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 8, "a bounded settle terminates");
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec!["noteWorldNavigation", "setCamera"]);
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("gestures")).and_then(semio_framework_value::DslValue::as_array).map(|ids| ids.iter().filter_map(semio_framework_value::DslValue::as_str).collect::<Vec<_>>()), Some(vec!["zoom"]));
}

#[test]
fn world_drag_and_paint_plans_reserve_before_exact_mutation() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let original_target = state.camera.orbit.target;
    let modifiers = PointerModifiers { shift: false, ctrl: false, alt: false, meta: false };
    let mut drag = plan_world3d_drag(&state.session, &state.camera, 3, 8.0, -4.0, 1, &modifiers).expect("middle drag plan");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let step = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut drag, 3, &mut input, context)).unwrap();
    assert_eq!(step, WorldInteractionStep::Pending);
    assert_ne!(state.camera.orbit.target, original_target);
    assert!(take_actions(&mut input).is_empty(), "a pan step moves the orbit and owes the settle, it never publishes per move");

    state.selection.interaction_mode = "paint".into();
    assert!(plan_world3d_paint_release(&state.session, &state.selection, 4, 0).is_none(), "a release without an open paint gesture commits nothing");
    state.selection.paint_stroke_active = true;
    let mut release = plan_world3d_paint_release(&state.session, &state.selection, 4, 0).expect("paint release plan");
    let zero = with_world_step_context(0, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut release, 4, &mut input, context)).unwrap();
    assert_eq!(zero, WorldInteractionStep::Pending);
    assert!(state.selection.paint_stroke_active, "the gesture stays open until the commit is reserved");
    let published = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut release, 4, &mut input, context)).unwrap();
    assert_eq!(published, WorldInteractionStep::Pending);
    assert!(!state.selection.paint_stroke_active);
    let actions = take_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec!["paintAt"]);
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework_value::DslValue::as_str), Some("commit"), "the release commits the app's open paint transaction");

    state.selection.paint_stroke_active = true;
    let mut end = plan_world3d_paint_release(&state.session, &state.selection, 5, 0).expect("paint release plan");
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);
    let stale = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut end, 5, &mut input, context)).unwrap();
    assert_eq!(stale, WorldInteractionStep::Stale);
    assert!(state.selection.paint_stroke_active);
    assert!(take_actions(&mut input).is_empty());
}

/// 🧯️ LAW: a host cancel aborts an open paint stroke with ONE `paintAt{phase:"abort", reason}` and clears the stroke only
/// once the abort is reserved; with no stroke open it owes nothing.
#[test]
fn world_paint_cancel_aborts_the_open_stroke_with_its_reason() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.selection.interaction_mode = "paint".into();
    assert!(plan_world3d_paint_abort(&state.session, &state.selection, 4, WorldCancelReason::Blur).is_none(), "no open stroke, nothing to abort");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    for (generation, reason) in [(4, WorldCancelReason::Blur), (5, WorldCancelReason::CaptureLost)] {
        state.selection.paint_stroke_active = true;
        let mut abort = plan_world3d_paint_abort(&state.session, &state.selection, generation, reason).expect("paint abort plan");
        let published = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut abort, generation, &mut input, context)).unwrap();
        assert_eq!(published, WorldInteractionStep::Pending);
        assert!(!state.selection.paint_stroke_active, "the abort closes the stroke");
        let actions = take_actions(&mut input);
        assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec!["paintAt"]);
        let arg = |key: &str| actions[0].args.as_ref().and_then(|args| args.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        assert_eq!((arg("phase"), arg("reason")), (Some("abort".to_string()), Some(reason.code().to_string())));
    }
}

/// ⚡️ LAW: a live gesture never previews locally — the guest previews the open transaction it streamed — while a local
/// gesture keeps the host's in-place preview.
#[test]
fn world_live_gumball_skips_the_local_preview() {
    let (mut state, mut gesture) = world_gumball_commit_host_snapshot();
    gesture.live = true;
    state.authority.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    let source = Mat4::identity();
    assert_eq!(retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source).cols, source.cols, "the guest draws the live answer");
    state.authority.interaction_authority.as_mut().unwrap().gumball.as_mut().expect("gesture").live = false;
    assert_eq!(retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source).cols[3], [2.0, 0.0, 0.0, 1.0], "a local drag previews itself");
}

/// 🧯️ LAW: a cancel is owed exactly while a live gumball gesture has streamed or a paint stroke is open.
#[test]
fn world_cancel_is_owed_only_by_a_streamed_live_gesture_or_an_open_stroke() {
    let (mut state, mut gesture) = world_gumball_commit_host_snapshot();
    assert!(!world3d_cancel_owed(&state.selection, &state.authority), "nothing open, nothing owed");
    gesture.live = true;
    state.authority.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    assert!(!world3d_cancel_owed(&state.selection, &state.authority), "a live gesture that streamed nothing owes nothing");
    state.authority.interaction_authority.as_mut().unwrap().gumball.as_mut().expect("gesture").streamed = true;
    assert!(world3d_cancel_owed(&state.selection, &state.authority));
    state.authority.interaction_authority.as_mut().unwrap().gumball = None;
    state.selection.interaction_mode = "paint".into();
    state.selection.paint_stroke_active = true;
    assert!(world3d_cancel_owed(&state.selection, &state.authority), "an open stroke owes its abort");
}

fn world_pick_fixture() -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut data = triangle_mesh_oracle();
    data.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), publish_oracle_mesh(data));
    let mesh_version = *state.scene.mesh_versions.get("mesh").expect("mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { component_source: None, id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state
}

#[test]
fn world_ray_pick_cursor_advances_one_triangle_or_boundary_per_grant() {
    let state = world_pick_fixture();
    let mut cursor = WorldRayPickCursor {
        revision: state.session.interaction_revision,
        generation: 9,
        purpose: WorldRayPickPurpose::Paint,
        origin: Vec3::new(0.0, 0.0, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        local_x: 0.0,
        local_y: 0.0,
        viewport: render_pick_viewport(&state.session),
        view_projection: ui_scene::math::projection_spec_view_proj(&state.camera.orbit.to_camera(), state.camera.projection_spec, render_pick_viewport(&state.session).w, render_pick_viewport(&state.session).h),
        projection_spec: state.camera.projection_spec,
        draw: 0,
        instance: 0,
        primitive: 0,
        mesh: None,
        mesh_probe: 0,
        merge: 0,
        best: None,
        pick_target: 0,
        best_target: None,
        complete: false,
        faulted: false,
    };
    let zero = with_world_step_context(0, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context));
    assert_eq!(zero, WorldInteractionStep::Pending);
    assert_eq!(cursor.primitive, 0);
    let first = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context));
    assert_eq!(first, WorldInteractionStep::Pending);
    assert_eq!(cursor.primitive, 0);
    assert!(cursor.mesh.is_some());
    let triangle = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context));
    assert_eq!(triangle, WorldInteractionStep::Pending);
    assert_eq!(cursor.primitive, 1);
    assert!(cursor.best.is_some());
    for _ in 0..3 {
        let _ = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context));
    }
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context)), WorldInteractionStep::Complete);
    let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 9).expect("live cursor").expect("paint hit");
    let mut state = state;
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 9, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["paintAt"]);
}

#[test]
fn world_ray_pick_cursor_stale_and_interrupted_close_do_not_publish() {
    let mut state = world_pick_fixture();
    let mut cursor = WorldRayPickCursor {
        revision: state.session.interaction_revision,
        generation: 12,
        purpose: WorldRayPickPurpose::Surface,
        origin: Vec3::new(0.0, 0.0, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        local_x: 0.0,
        local_y: 0.0,
        viewport: render_pick_viewport(&state.session),
        view_projection: ui_scene::math::projection_spec_view_proj(&state.camera.orbit.to_camera(), state.camera.projection_spec, render_pick_viewport(&state.session).w, render_pick_viewport(&state.session).h),
        projection_spec: state.camera.projection_spec,
        draw: 0,
        instance: 0,
        primitive: 0,
        mesh: None,
        mesh_probe: 0,
        merge: 0,
        best: None,
        pick_target: 0,
        best_target: None,
        complete: false,
        faulted: false,
    };
    let _ = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context));
    let _ = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context));
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context)), WorldInteractionStep::Stale);
    assert_eq!(cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 12).err().expect("stale cursor"), WorldInteractionStep::Stale);
    assert!(!cursor.close_step());
    assert!(cursor.close_step());
    assert!(cursor.terminal_is_empty());
}

/// 🧭️ **The settle-saturation law.** A navigation intent publishes nothing at all — it moves the
/// orbit and leaves the debt — so the plan that can saturate the output is the SETTLE's, and a
/// saturated settle is RETAINED and retried in its own order (`noteWorldNavigation`, then
/// `setCamera`), never dropped. See `WorldCameraSync`.
#[test]
fn world_authority_retains_front_plan_across_output_saturation_and_retries_in_order() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let original_distance = state.camera.orbit.distance;
    enqueue_world3d_intent(&mut state.authority, world_intent(1, 200.0)).expect("wheel intent");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)) != WorldInteractionAuthorityStep::Complete {
        turns += 1;
        assert!(turns < 8, "a wheel intent settles in a bounded number of steps");
    }
    assert_ne!(state.camera.orbit.distance, original_distance);
    assert!(take_actions(&mut input).is_empty(), "a navigation step publishes nothing");

    let settle = world3d_interaction_front_generation(&state.camera, &state.authority).expect("a drained queue still owes the settle");
    let mut claims = Vec::new();
    while let Ok(claim) = input.claim_action(1) {
        claims.push(claim);
    }
    assert!(!claims.is_empty());
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_interaction(&mut state, settle, &mut input, context)) != WorldInteractionAuthorityStep::OutputBlocked {
        turns += 1;
        assert!(turns < 16, "a settle with no output credit blocks rather than dropping its plan");
    }
    assert!(take_actions(&mut input).is_empty());
    for claim in claims {
        input.release_action_claim(claim).expect("release retained credit");
    }
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_interaction(&mut state, settle, &mut input, context)) != WorldInteractionAuthorityStep::Complete {
        turns += 1;
        assert!(turns < 8, "a retried settle completes in a bounded number of steps");
    }
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["noteWorldNavigation", "setCamera"]);
}

#[test]
fn world_authority_close_drains_active_and_queued_fixed_owners_to_terminal() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    enqueue_world3d_intent(&mut state.authority, world_intent(1, 10.0)).expect("active intent");
    enqueue_world3d_intent(&mut state.authority, world_intent(2, 20.0)).expect("queued intent");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
    begin_world3d_interaction_close(&mut state.authority);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert!(!with_world_step_context(0, |context| close_world3d_interaction_step(&mut state.authority, &mut input, context)));
    assert!(!world3d_interaction_terminal_is_empty(&state.authority));
    let mut turns = 0;
    while !with_world_step_context(1, |context| close_world3d_interaction_step(&mut state.authority, &mut input, context)) {
        turns += 1;
        assert!(turns < 8);
    }
    assert!(world3d_interaction_terminal_is_empty(&state.authority));
    assert!(take_actions(&mut input).is_empty());
}

/// 🗃️ The blocked owner transfers on the FIRST turn that finds a freed slot, whatever else that
/// turn's authority owes first (a moved camera re-dirties the object registry, and its rebuild is
/// itself several turns).
#[test]
fn world_saturation_owner_blocks_new_ingress_until_exact_fifo_transfer() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    for generation in 1..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
        enqueue_world3d_intent(&mut state.authority, world_intent(generation, generation as f32)).expect("queue or retained saturation owner");
    }
    let rejected_generation = WORLD_INTERACTION_INTENT_CAPACITY as u64 + 2;
    assert_eq!(enqueue_world3d_intent(&mut state.authority, world_intent(rejected_generation, rejected_generation as f32)).expect_err("blocked owner seals ingress").generation, rejected_generation);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while state.authority.interaction_authority.as_ref().and_then(|authority| authority.queue.front()).map(|intent| intent.generation) == Some(1) {
        let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
        turns += 1;
        assert!(turns < 16, "the front intent is answered in a bounded number of turns");
    }
    assert!(take_actions(&mut input).is_empty(), "a navigation intent moves the orbit and owes the settle, it publishes nothing");
    let mut turns = 0;
    while state.authority.interaction_authority.as_ref().is_some_and(|authority| authority.blocked.is_some()) {
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
        turns += 1;
        assert!(turns < 16, "a freed slot admits the retained saturation owner");
    }
    let authority = state.authority.interaction_authority.as_mut().expect("authority");
    for generation in 2..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
        assert_eq!(authority.queue.front().map(|intent| intent.generation), Some(generation));
        assert!(authority.queue.retire_front(generation));
    }
    assert!(authority.queue.front().is_none());
}
