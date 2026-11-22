/// ♻️ Retained map gesture lifecycle and bridge: same identity refresh, key replacement and removal, sibling rebase, the tile URL placeholders and the bridge lease revision bump.

#[test]
fn retained_map_same_identity_refresh_preserves_the_active_gesture() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(MAP_GESTURE_LIFECYCLE_FIXTURE).expect("Map gesture lifecycle contract");
    let fixture = fixture();
    let surface_id = "map.lifecycle.refresh";
    let key = contract["owner"]["key"].as_str().expect("Map owner key");
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let base = tiled_map_scene(surface_id, &fixture).tiled_map.expect("fixture Map scene");
    drop_engine_surface(surface_id);
    let mut first = retained_document(surface_id, 1, 1, vec![retained_map_record(1, key, &base)]);
    let _ = publish_retained_map_fixture(&first, surface_id, bounds, 901);
    let mut input = InputState::<ActionDescriptor>::default();
    let owner = semio_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface_id, bounds.x + 1.0, bounds.y + 1.0).expect("mounted Map exposes its exact retained target");
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&owner, "map.lifecycle.fixture", bounds, 100.0, 100.0, 0, false, false, "rectangle", &mut input), Ok(true));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&owner.host_id));
    let mut changed = base.clone();
    changed.camera_json = json!({ "x": 0.5, "y": 0.4, "zoom": 5.0 }).to_string();
    let mut refresh = retained_document(surface_id, 2, 1, vec![retained_map_record(1, key, &changed)]);
    let mut refresh_input = publish_retained_map_fixture(&refresh, surface_id, bounds, 902);
    let refresh_actions = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut refresh_input);
    assert!(refresh_actions.is_empty(), "an ordinary refresh does not synthesize Map input");
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&owner.host_id), "the same retained node/key/kind/surface preserves its active gesture");
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_pointer_cancel_into(&owner.host_id, "map.lifecycle.fixture", bounds, 100.0, 100.0, &mut input), Ok(true));
    close_retained_map_fixture(surface_id);
    while !first.close_step() {}
    while !refresh.close_step() {}
    assert!(engine_surface_token(&owner.host_id).is_none());
}

#[test]
fn retained_map_key_replacement_and_removal_retire_the_old_gesture_without_input() {
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(MAP_GESTURE_LIFECYCLE_FIXTURE).expect("Map gesture lifecycle contract");
    let fixture = fixture();
    let surface_id = "map.lifecycle.replacement";
    let original = contract["owner"]["key"].as_str().expect("Map owner key");
    let successor = contract["cases"].as_array().expect("lifecycle cases").iter().find(|row| row["name"] == "key-replacement").and_then(|row| row["next"]["key"].as_str()).expect("successor key");
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let map = tiled_map_scene(surface_id, &fixture).tiled_map.expect("fixture Map scene");
    drop_engine_surface(surface_id);
    let mut first = retained_document(surface_id, 1, 1, vec![retained_map_record(1, original, &map)]);
    let _ = publish_retained_map_fixture(&first, surface_id, bounds, 901);
    let mut gesture_input = InputState::<ActionDescriptor>::default();
    let original_owner = semio_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface_id, bounds.x + 1.0, bounds.y + 1.0).expect("mounted Map exposes its exact retained target");
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&original_owner, "map.lifecycle.fixture", bounds, 100.0, 100.0, 0, false, false, "rectangle", &mut gesture_input), Ok(true));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&original_owner.host_id));
    let mut replacement = retained_document(surface_id, 2, 1, vec![retained_map_record(1, successor, &map)]);
    let mut replacement_input = publish_retained_map_fixture(&replacement, surface_id, bounds, 902);
    let replacement_actions = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut replacement_input);
    assert!(replacement_actions.is_empty(), "key replacement cannot synthesize PointerUp or selection");
    assert!(!semio_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(&original_owner), "acknowledgement revokes the old Map receiver before successor input");
    let mut retirement_input = publish_retained_map_fixture(&replacement, surface_id, bounds, 903);
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut retirement_input).is_empty(), "bounded Map retirement cannot synthesize input");
    assert!(!semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&original_owner.host_id), "the bounded retirement lane clears the old keyed Map gesture");
    assert!(engine_surface_token(&original_owner.host_id).is_none());
    let successor_owner = semio_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(surface_id, bounds.x + 1.0, bounds.y + 1.0).expect("successor exposes its exact retained target");
    assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&successor_owner, "map.lifecycle.fixture", bounds, 140.0, 140.0, 0, false, false, "rectangle", &mut gesture_input), Ok(true));
    assert!(semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&successor_owner.host_id), "the keyed successor starts a fresh gesture");
    let mut removed = retained_document(surface_id, 3, 2, vec![retained_separator_record(2, "map.removed")]);
    let mut removal_input = publish_retained_map_fixture(&removed, surface_id, bounds, 904);
    let removal_actions = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut removal_input);
    assert!(removal_actions.is_empty(), "node removal cannot synthesize PointerUp or selection");
    assert!(!semio_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(&successor_owner), "acknowledgement revokes the removed Map receiver");
    let mut retirement_input = publish_retained_map_fixture(&removed, surface_id, bounds, 905);
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut retirement_input).is_empty());
    assert!(!semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&successor_owner.host_id), "Retire clears the removed Map gesture");
    close_retained_map_fixture(surface_id);
    while !first.close_step() {}
    while !replacement.close_step() {}
    while !removed.close_step() {}
    assert!(engine_surface_token(&original_owner.host_id).is_none());
    assert!(engine_surface_token(&successor_owner.host_id).is_none());
}

#[test]
fn retained_map_same_host_sibling_rebase_retires_engine_interaction_owner() {
    let mut caller=crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let _guard = engine_surface_law_guard();
    let contract: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).expect("scene pointer owner contract");
    let fixture = fixture();
    let law = &contract["siblingInsertion"];
    let receiver = law["receiver"].as_u64().expect("receiver protocol id");
    let window_id = "map.lifecycle.sibling-rebase";
    let wire_surface_id = "map.lifecycle.shared-wire";
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let map = tiled_map_scene(wire_surface_id, &fixture).tiled_map.expect("fixture Map scene");
    let mut documents = Vec::new();
    let mut presented = None;
    let mut accepted = None;
    let mut hosts = Vec::new();
    for (index, ids) in law["records"].as_array().expect("sibling histories").iter().enumerate() {
        let ids: Vec<_> = ids.as_array().expect("protocol node ids").iter().map(|id| id.as_u64().expect("protocol node id")).collect();
        let mut records = vec![retained_map_row_record(1, &ids)];
        records.extend(ids.iter().map(|id| retained_map_record(*id, &format!("map-{id}"), &map)));
        let document = retained_document(window_id, index as u64 + 1, 1, records);
        let mut input = paint_retained_map_document(&document, window_id, bounds);
        semio_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
        semio_renderer_wgpu_ui_engine_dump::chrome_ledger::note_accessibility_visible_document(window_id);
        let registrations = semio_renderer_wgpu_ui_engine::retained_ui_engine::register_clipped_retained_hit_targets(window_id, bounds, &mut input);
        for (_, target, _) in &registrations {
            if let Some(target) = target {
                if !hosts.contains(&target.host_id) {
                    hosts.push(target.host_id.clone());
                }
            }
        }
        let witness = 840 + index as u64;
        assert!(semio_renderer_wgpu_ui_engine_dump::chrome_ledger::seal_presented_input_candidate(witness));
        assert!(semio_renderer_wgpu_ui_engine_dump::chrome_ledger::acknowledge_presented_input(witness));
        input.publish_hits();
        semio_renderer_wgpu_ui_engine_dump::chrome_ledger::publish_accessibility_visible_documents();
        if index > 0 {
            let (target, rect) = registrations.into_iter().find_map(|(_, target, rect)| target.filter(|target| target.document_id == ui_contract::UiNodeId(receiver)).map(|target| (target, rect))).expect("the receiver Map remains mounted");
            assert!(semio_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(&target), "the accepted receiver registration names the live Map host");
            if index == 1 {
                let mut interaction = InputState::<ActionDescriptor>::default();
                assert_eq!(semio_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&target, "map.lifecycle.fixture", rect, rect.x + rect.w * 0.5, rect.y + rect.h * 0.5, 0, false, false, "rectangle", &mut interaction), Ok(true));
                assert!(semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&target.host_id));
                presented = Some(target);
            } else {
                accepted = Some(target);
            }
        }
        documents.push(document);
    }
    let presented = presented.expect("the displayed Map owns the interaction");
    let accepted = accepted.expect("the accepted sibling candidate retains the Map");
    let engine_retired = retire_map_interaction_owner(&accepted);
    let gesture_retired = semio_renderer_wgpu_scene_state::retire_tiled_map_scene_identity(&accepted);
    let gesture_live = semio_renderer_wgpu_scene_feeds::tiled_map_drag_active(&accepted.host_id);
    let duplicate_retired = retire_map_interaction_owner(&presented);
    for host in hosts {
        drop_engine_surface(&host);
    }
    assert!(semio_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(window_id));
    for _ in 0..262_144 {
        if !semio_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending() {
            break;
        }
        caller.turn(|cx|semio_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn");
        semio_renderer_wgpu_host_winit::os_host::finish_retired_cpu_only_map_fixture_close();
    }
    assert!(!semio_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending(), "the retained Map window returns every bounded owner");
    for mut document in documents {
        while !document.close_step() {}
    }
    assert!(presented.same_component_host(&accepted));
    assert_ne!(presented.node, accepted.node, "the fixture exercises a real arena-node rebase");
    assert!(engine_retired, "the accepted same-host identity retires the EngineCanvas interaction owner");
    assert!(gesture_retired, "the accepted same-host identity retires the local Map gesture");
    assert!(!gesture_live, "the accepted retirement clears the local Map gesture");
    assert!(!duplicate_retired, "the same retirement already consumed the EngineCanvas interaction owner");
    caller.finish();
}

/// 🔗️ `map_tile_url` is React's own `urlTemplate.replace("{z}", …).replace("{x}", …).replace("{y}", …)`.
#[test]
fn map_tile_url_substitutes_the_same_three_placeholders_react_does() {
    assert_eq!(map_tile_url("/osm/{z}/{x}/{y}.png", 14, 8192, 5461), "/osm/14/8192/5461.png");
    assert_eq!(map_tile_url("/vt/{z}/{x}/{y}.pbf", 0, 0, 0), "/vt/0/0/0.pbf");
    assert_eq!(map_tile_url("/no/placeholders", 3, 4, 5), "/no/placeholders", "a template without placeholders is passed through, as in JS");
}

/// 🧱️ A revision bump taken while the scene bridge's lease is IN FLIGHT must not lose the draws the
/// apply built from that lease.
///
/// 🩸️ `step_world3d_snapshot`'s completion adopted `lease.revision` verbatim, and that lease carries
/// this world's `interaction_revision` as of the bridge PUBLISH. Anything that bumps the revision
/// between the publish and the apply — a document lane the producer republished, a parallel framing,
/// a camera report — therefore rolled the counter BACKWARDS at completion, and the sealed draw
/// rebuild the same apply had just filled answered `WorldDrawRebuildStep::Stale` on its very next
/// turn, which closes it and throws every draw away. Measured on 6118: generation3d's
/// `procedural-preview` sat at `state-meshes=3 state-instances=0 state-draws=0` forever, three
/// `step=Stale` lines after `world3d delivery applied`, with the React twin showing the tessellated
/// column (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w14b-generation3d-labels-preview-layout.md`).
///
/// 1️⃣ The first paint STAGES the mesh-wire bridge; this drive seals its lease.
///
/// 2️⃣ …and only THEN does the producer republish a document lane, which bumps the revision.
///    The reference plane is `hidden`, so this is a pure revision bump and never a re-framing.
#[test]
fn a_revision_bump_while_the_bridge_lease_is_in_flight_still_publishes_the_draws() {
    let fixture = fixture();
    let surface_id = "world3d-inflight-revision";
    let scene = world3d_preview_scene(surface_id, &fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };

    let mut frame = paint_scene(&scene, bounds, semio_renderer_wgpu_contracts::AdmittedSurfaceMap::default());
    assert!(frame.world3d_states.contains_key(surface_id), "the first painted frame constructs the World3d host");
    drive_world3d_ladder(frame.world3d_states.get_mut(surface_id).expect("attached world state"));

    let mut moved = world3d_preview_scene(surface_id, &fixture);
    moved.world_3d.as_mut().expect("world scene").references_json = Some(json!([{ "id": "plan", "origin": [0.0, 0.0, 0.0], "widthWorld": 4.0, "hidden": true }]).to_string());

    let mut painted = false;
    for turn in 0..16 {
        frame = paint_scene(&moved, bounds, frame.world3d_states);
        if frame.draw.scene_passes.first().is_some_and(|pass| !pass.draws.is_empty()) {
            painted = true;
            break;
        }
        drive_world3d_ladder(frame.world3d_states.get_mut(surface_id).expect("attached world state"));
        assert!(turn < 15, "the preview settled into a drawn frame within its frame ceiling");
    }
    let state = frame.world3d_states.get(surface_id).expect("attached world state");
    assert_eq!(state.snapshot_fault(), None, "a revision bump is not a snapshot fault");
    assert!(painted, "the bridged draws survive a revision bump taken while their lease was in flight: {}", state.ingest_census());
    drop_world3d_states(frame.world3d_states);
}

/// 🩹️ Render-device losses walk a map through React's recovery: re-attach once, then unavailable with the live camera kept, restored by the retry.
#[test]
fn map_device_loss_recovery_restores_the_live_camera_on_retry() {
    use semio_renderer_wgpu_engine_board::tiled_map::{MapRecovery, MapRendererState};
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/🩹️map-recovery/🔣️.json")).expect("map recovery fixture");
    let live: [f64; 3] = serde_json::from_value(fixture["liveCamera"].clone()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut record = MapRecovery::default();
        let states: Vec<&str> = (0..case["losses"].as_u64().unwrap()).map(|_| match record.lost(Some(live)) {
            MapRendererState::Running => "running",
            MapRendererState::Reattached => "reattached",
            MapRendererState::Unavailable => "unavailable",
        }).collect();
        assert_eq!(serde_json::to_value(&states).unwrap(), case["states"], "{}", case["name"]);
        let restored = record.retry();
        assert_eq!(restored.map(|camera| serde_json::json!(camera)), (!case["restored"].is_null()).then(|| case["restored"].clone()), "{}", case["name"]);
        if restored.is_some() {
            assert_eq!(record.state, MapRendererState::Running);
        }
    }
}

/// 🩹️ A given-up map paints its notice; the staged Retry rect is the only pointer target, and the retry route leaves the unavailable state and drops the notice.
#[test]
fn map_unavailable_notice_is_staged_and_retried_through_its_button_rect() {
    use semio_renderer_wgpu_engine_board::tiled_map::{map_retry_contains, map_unavailable_layout, map_unavailable_notice, retry_tiled_map, stage_map_unavailable_notice, tiled_map_device_lost, tiled_map_unavailable, MapUnavailableNotice};
    let surface_id = "map.notice.retry";
    let bounds = Rect { x: 10.0, y: 20.0, w: 400.0, h: 200.0 };
    let (message_rect, retry_rect) = map_unavailable_layout(bounds, (120.0, 16.0), 90.0, 32.0, 8.0);
    assert_eq!(message_rect, Rect { x: 150.0, y: 20.0 + (200.0 - 56.0) * 0.5, w: 120.0, h: 16.0 });
    assert_eq!(retry_rect, Rect { x: 165.0, y: message_rect.y + 24.0, w: 90.0, h: 32.0 });
    stage_map_unavailable_notice(surface_id, MapUnavailableNotice { message: "Canvas unavailable".into(), retry_label: "Retry".into(), message_rect, retry_rect });
    assert!(map_unavailable_notice(surface_id).is_none(), "a map that still draws shows no notice");
    tiled_map_device_lost(surface_id);
    tiled_map_device_lost(surface_id);
    assert!(tiled_map_unavailable(surface_id));
    assert_eq!(map_unavailable_notice(surface_id).map(|notice| notice.retry_label), Some("Retry".to_string()));
    assert!(map_retry_contains(surface_id, retry_rect.x + 1.0, retry_rect.y + 1.0));
    assert!(!map_retry_contains(surface_id, message_rect.x, message_rect.y), "the message is not the button");
    assert!(retry_tiled_map(surface_id));
    assert!(!tiled_map_unavailable(surface_id) && map_unavailable_notice(surface_id).is_none());
    assert!(!retry_tiled_map(surface_id), "a second retry has nothing pending");
}
