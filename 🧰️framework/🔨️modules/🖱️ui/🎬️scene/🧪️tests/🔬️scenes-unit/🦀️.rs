
use super::*;
use serde_json::Value;

#[test]
fn world3d_scene_domain_id_round_trips_as_camel_case_and_omits_when_none() {
    let mut scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    scene.domain_id = Some("cad".into());
    scene.domain_granularity_id = Some("handle".into());
    let value = serde_json::to_value(&scene).expect("serialize");
    assert_eq!(value.get("domainId").and_then(Value::as_str), Some("cad"));
    assert_eq!(value.get("domainGranularityId").and_then(Value::as_str), Some("handle"));
    let back: World3dScene = serde_json::from_value(value).expect("deserialize");
    assert_eq!(back, scene);

    let bare = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    let bare_value = serde_json::to_value(&bare).expect("serialize");
    assert!(bare_value.get("domainId").is_none());
    assert!(bare_value.get("domainGranularityId").is_none());
}

#[test]
fn node_graph_hover_port_id_round_trips_as_camel_case_and_omits_when_none() {
    let hover = NodeGraphHover { node_id: Some("combine".into()), port_id: Some("b".into()) };
    let value = serde_json::to_value(&hover).expect("serialize");
    assert_eq!(value.get("nodeId").and_then(Value::as_str), Some("combine"));
    assert_eq!(value.get("portId").and_then(Value::as_str), Some("b"));
    let back: NodeGraphHover = serde_json::from_value(value).expect("deserialize");
    assert_eq!(back, hover);

    let bare = NodeGraphHover { node_id: Some("combine".into()), port_id: None };
    let bare_value = serde_json::to_value(&bare).expect("serialize");
    assert!(bare_value.get("portId").is_none());
}

#[test]
fn node_graph_scene_highlighted_round_trips_and_omits_when_empty() {
    let viewport = semio_framework_ui_viewport::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 };
    let mut scene = NodeGraphScene { highlighted: vec!["a".into(), "b@out".into()], ..NodeGraphScene::base(Vec::new(), Vec::new(), viewport.clone()) };
    let value = serde_json::to_value(&scene).expect("serialize");
    assert_eq!(value.get("highlighted").and_then(Value::as_array).map(Vec::len), Some(2));
    let back: NodeGraphScene = serde_json::from_value(value).expect("deserialize");
    assert_eq!(back, scene);

    scene.highlighted = Vec::new();
    let bare_value = serde_json::to_value(&scene).expect("serialize");
    assert!(bare_value.get("highlighted").is_none());
}

//#region 🚚️World3dSceneLanes
/// 🚚️ The ONE language-neutral declaration both this crate and `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`
/// are pinned against — see `World3dSceneLane`'s docstring.
const WORLD3D_SCENE_LANE_CONTRACT: &str = include_str!("../../🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json");

fn world3d_lane_contract() -> Value {
    serde_json::from_str(WORLD3D_SCENE_LANE_CONTRACT).expect("world-3d lane contract parses")
}

fn world3d_scene_from_json(value: &Value) -> World3dScene {
    serde_json::from_value(value.clone()).expect("contract scene deserializes")
}

#[test]
fn world3d_scene_lanes_mirror_the_language_neutral_declaration() {
    let contract = world3d_lane_contract();
    assert_eq!(contract["schema"].as_str(), Some(World3dScene::SCHEMA));
    assert_eq!(contract["laneKeyPrefix"].as_str(), Some(WORLD3D_SCENE_LANE_KEY_PREFIX));
    let declared = contract["lanes"].as_array().expect("lanes array");
    assert_eq!(declared.len(), World3dSceneLane::ALL.len());
    for (lane, entry) in World3dSceneLane::ALL.into_iter().zip(declared) {
        assert_eq!(entry["lane"].as_str(), Some(lane.name()));
        assert_eq!(entry["field"].as_str(), Some(lane.field()));
        assert_eq!(entry["bodyKey"].as_str(), Some(lane.body_key()));
        assert_eq!(entry["optional"].as_bool(), Some(lane.optional()));
        assert_eq!(lane.body_key(), format!("{WORLD3D_SCENE_LANE_KEY_PREFIX}{}", lane.name()));
        assert_eq!(World3dSceneLane::from_body_key(lane.body_key()), Some(lane));
        assert_eq!(World3dSceneLane::from_name(lane.name()), Some(lane));
    }
    assert_eq!(World3dSceneLane::from_body_key("puzzle3d-main-top"), None);
    let spine_fields: Vec<&str> = contract["spineFields"].as_array().expect("spineFields").iter().map(|field| field.as_str().expect("spine field")).collect();
    for field in &spine_fields {
        assert!(!WORLD3D_SCENE_LANE_FIELDS.contains(field), "spine field {field} must not also be a lane");
    }
    let base = serde_json::to_value(World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into())).expect("serialize base");
    let keys: Vec<&str> = base.as_object().expect("object").keys().map(String::as_str).collect();
    for key in &keys {
        assert!(spine_fields.contains(key) || WORLD3D_SCENE_LANE_FIELDS.contains(key), "scene field {key} is declared neither spine nor lane");
    }
}

#[test]
fn world3d_scene_splits_into_the_declared_lanes_and_merges_back() {
    let contract = world3d_lane_contract();
    let round_trip = &contract["roundTrip"];
    let assembled = world3d_scene_from_json(&round_trip["assembled"]);
    let (spine, lanes) = assembled.split_lanes();

    let expected_texts = round_trip["laneTexts"].as_object().expect("laneTexts object");
    assert_eq!(lanes.len(), expected_texts.len());
    for lane in &lanes {
        assert_eq!(Some(lane.payload.as_str()), expected_texts.get(lane.key).and_then(Value::as_str), "lane {} payload", lane.key);
    }
    assert_eq!(serde_json::to_value(&spine).expect("serialize spine"), round_trip["spine"]);

    let mut merged = spine.clone();
    for lane in &lanes {
        assert!(merged.merge_lane(lane.key, lane.payload.clone()), "lane {} merges", lane.key);
    }
    assert!(!merged.merge_lane("puzzle3d-main-top", String::new()));
    merged.lanes = Vec::new();
    assert_eq!(merged, assembled);
    println!("[DEBUG] world-3d scene split into {} lanes and merged back byte-exactly", lanes.len());
}

#[test]
fn world3d_scene_spine_changes_only_for_the_lanes_that_changed() {
    let contract = world3d_lane_contract();
    let assembled = world3d_scene_from_json(&contract["roundTrip"]["assembled"]);
    let (first_spine, first_lanes) = assembled.split_lanes();

    let mut camera_moved = assembled.clone();
    camera_moved.camera_json = r#"{"position":[9,9,9],"target":[0,0,0]}"#.into();
    let (camera_spine, camera_lanes) = camera_moved.split_lanes();
    assert_ne!(camera_spine.camera_json, first_spine.camera_json);
    assert_eq!(camera_spine.lanes, first_spine.lanes, "no lane ref may change when only the camera moved");
    assert_eq!(camera_lanes, first_lanes, "no lane payload may change when only the camera moved");

    let mut selection_changed = assembled.clone();
    selection_changed.selection_json = r#"{"method":"rectangle","mode":"replace","ids":["a"],"hoveredId":null}"#.into();
    let (selection_spine, selection_lanes) = selection_changed.split_lanes();
    let changed: Vec<&str> = first_spine.lanes.iter().zip(&selection_spine.lanes).filter(|(before, after)| before != after).map(|(before, _)| before.lane.as_str()).collect();
    assert_eq!(changed, vec!["selection"]);
    let changed_payloads: Vec<&str> = first_lanes.iter().zip(&selection_lanes).filter(|(before, after)| before != after).map(|(before, _)| before.key).collect();
    assert_eq!(changed_payloads, vec![World3dSceneLane::Selection.body_key()]);
    println!("[DEBUG] a selection edit moved exactly 1 of {} lane refs; a camera move moved none", first_spine.lanes.len());
}

#[test]
fn scene_lane_hash_is_fnv1a64_hex_and_pins_the_declared_digests() {
    let contract = world3d_lane_contract();
    assert_eq!(contract["carrier"]["hash"].as_str(), Some("fnv1a64-hex"));
    assert_eq!(scene_lane_hash(""), "cbf29ce484222325");
    for entry in contract["roundTrip"]["spine"]["lanes"].as_array().expect("spine lanes") {
        let name = entry["lane"].as_str().expect("lane name");
        let lane = World3dSceneLane::from_name(name).expect("declared lane");
        let payload = contract["roundTrip"]["laneTexts"][lane.body_key()].as_str().expect("lane text");
        assert_eq!(scene_lane_hash(payload), entry["hash"].as_str().expect("hash"), "lane {name} hash");
        assert_eq!(payload.len() as u64, entry["bytes"].as_u64().expect("bytes"), "lane {name} bytes");
    }
}

#[test]
fn world3d_scene_spine_survives_the_pack_and_value_codecs_with_its_lane_manifest() {
    let contract = world3d_lane_contract();
    let assembled = world3d_scene_from_json(&contract["roundTrip"]["assembled"]);
    let (spine, _) = assembled.split_lanes();
    let packed = spine.encode_pack().expect("spine packs");
    assert!(packed.len() <= contract["carrier"]["docBytesMax"].as_u64().expect("docBytesMax") as usize);
    assert_eq!(World3dScene::decode_pack(&packed).expect("spine unpacks"), spine);
    assert_eq!(World3dScene::from_value(spine.to_value()).expect("spine round-trips as a value"), spine);
    println!("[DEBUG] world-3d spine packs to {} bytes carrying {} lane refs", packed.len(), spine.lanes.len());
}

#[test]
fn a_nakagin_scale_world3d_scene_pages_per_lane_and_reassembles_losslessly() {
    let contract = world3d_lane_contract();
    let leaf_bytes = contract["carrier"]["leafBytes"].as_u64().expect("leafBytes") as usize;
    let doc_max = contract["carrier"]["docBytesMax"].as_u64().expect("docBytesMax") as usize;
    let instances = (0..720)
        .map(|index| format!(r#"{{"id":"capsule-{index:04}","meshId":"box","position":[{index},0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}}"#))
        .collect::<Vec<_>>()
        .join(",");
    let vortices = (0..80)
        .map(|index| format!(r#"{{"id":"vortex-{index}","position":[0,{index},0],"strength":1}}"#))
        .collect::<Vec<_>>()
        .join(",");
    let references = (0..40).map(|index| format!(r#""ref-{index:03}""#)).collect::<Vec<_>>().join(",");
    let mut scene = World3dScene::base(
        r#"{"position":[4,4,4],"target":[0,0,0]}"#.into(),
        r#"[{"id":"box","kind":"box"}]"#.into(),
        format!("[{instances}]"),
        r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#.into(),
    );
    scene.vortices_json = Some(format!("[{vortices}]"));
    scene.references_json = Some(format!("[{references}]"));
    scene.interaction_json = Some(r#"{"hoveredId":null,"gumball":null}"#.into());
    scene.lod_json = Some(r#"{"maxInstances":8000}"#.into());
    scene.chunking_json = Some(r#"{"chunkSize":64,"radius":8000}"#.into());
    scene.environment_json = Some(r#"{"exposure":1}"#.into());
    scene.domain_id = Some("puzzle3d".into());
    let packed = scene.encode_pack().expect("assembled packs");
    let measured_fault = contract["carrier"]["measuredFaultBytes"].as_u64().expect("measuredFaultBytes") as usize;
    assert!(packed.len() > measured_fault, "unsplit Nakagin-scale scene must exceed the measured {measured_fault}-byte fault, got {}", packed.len());

    let (spine, lanes) = scene.split_lanes();
    let spine_pack = spine.encode_pack().expect("spine packs");
    assert!(spine_pack.len() <= doc_max, "spine {} must fit the {}-byte fixed doc", spine_pack.len(), doc_max);
    assert!(!lanes.is_empty());
    for lane in &lanes {
        assert!(lane.payload.len() <= packed.len());
        let pages = lane.payload.as_bytes().chunks(leaf_bytes).count();
        assert!(pages >= 1);
        for page in lane.payload.as_bytes().chunks(leaf_bytes) {
            assert!(page.len() <= leaf_bytes, "lane {} page {} exceeds leafBytes", lane.key, page.len());
        }
    }
    let mut merged = spine.clone();
    for lane in &lanes {
        assert!(merged.merge_lane(lane.key, lane.payload.clone()));
    }
    merged.lanes = Vec::new();
    assert_eq!(merged, scene);
    println!("[DEBUG] Nakagin-scale scene packed {} bytes into a {}-byte spine plus {} lane pages", packed.len(), spine_pack.len(), lanes.len());
}

#[test]
fn a_scene_without_lanes_still_publishes_its_whole_doc() {
    let scene = Canvas2dScene::base(1.0, 2.0, 3.0, "[]".into());
    let (spine, lanes) = scene.split_lanes();
    assert!(lanes.is_empty());
    assert_eq!(spine, scene);
}
#[test]
fn world3d_empty_brush_preview_still_publishes_a_lane() {
    let mut assembled = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    assembled.brush_preview_json = Some(String::new());
    let (spine, lanes) = assembled.split_lanes();
    assert_eq!(spine.brush_preview_json.as_deref(), Some(""));
    let preview = lanes.iter().find(|lane| lane.key == World3dSceneLane::BrushPreview.body_key()).expect("empty preview still splits");
    assert_eq!(preview.payload, "");
    assert!(spine.lanes.iter().any(|lane| lane.lane == "brushPreview" && lane.bytes == 0));
}

#[test]
fn world3d_tool_run_trace_rides_its_own_optional_lane_and_merges_back() {
    let mut assembled = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    assembled.tool_run_trace = Some("AAQBDQQB".into());
    let (spine, lanes) = assembled.split_lanes();
    assert_eq!(spine.tool_run_trace, None);
    let trace = lanes.iter().find(|lane| lane.key == "framework.scene.world3d.toolRunTrace").expect("trace lane splits out");
    assert_eq!(trace.payload, "AAQBDQQB");
    assert!(spine.lanes.iter().any(|lane| lane.lane == "toolRunTrace" && lane.bytes == 8 && lane.hash == scene_lane_hash("AAQBDQQB")));
    let mut merged = spine.clone();
    for lane in &lanes {
        assert!(merged.merge_lane(lane.key, lane.payload.clone()));
    }
    merged.lanes = Vec::new();
    assert_eq!(merged, assembled);
    let idle = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    assert!(idle.split_lanes().1.iter().all(|lane| lane.key != World3dSceneLane::ToolRunTrace.body_key()), "an idle run publishes no trace carrier");
}
//#endregion 🚚️World3dSceneLanes

//#region 🚚️Canvas2dSceneLanes
/// 🚚️ The canvas-2d twin of [`WORLD3D_SCENE_LANE_CONTRACT`].
const CANVAS2D_SCENE_LANE_CONTRACT: &str = include_str!("../../🧫️fixtures/🚚️canvas2d-scene-lanes/🔣️.json");

fn canvas2d_lane_contract() -> Value {
    serde_json::from_str(CANVAS2D_SCENE_LANE_CONTRACT).expect("canvas-2d lane contract parses")
}

#[test]
fn canvas2d_scene_lanes_mirror_the_language_neutral_declaration() {
    let contract = canvas2d_lane_contract();
    assert_eq!(contract["schema"].as_str(), Some(Canvas2dScene::SCHEMA));
    assert_eq!(contract["laneKeyPrefix"].as_str(), Some(CANVAS2D_SCENE_LANE_KEY_PREFIX));
    let declared = contract["lanes"].as_array().expect("lanes array");
    assert_eq!(declared.len(), Canvas2dSceneLane::ALL.len());
    for (lane, entry) in Canvas2dSceneLane::ALL.into_iter().zip(declared) {
        assert_eq!(entry["lane"].as_str(), Some(lane.name()));
        assert_eq!(entry["field"].as_str(), Some(lane.field()));
        assert_eq!(entry["bodyKey"].as_str(), Some(lane.body_key()));
        assert_eq!(entry["optional"].as_bool(), Some(lane.optional()));
        assert_eq!(lane.body_key(), format!("{CANVAS2D_SCENE_LANE_KEY_PREFIX}{}", lane.name()));
        assert_eq!(Canvas2dSceneLane::from_body_key(lane.body_key()), Some(lane));
        assert_eq!(Canvas2dSceneLane::from_name(lane.name()), Some(lane));
    }
    assert_eq!(Canvas2dSceneLane::from_body_key(World3dSceneLane::ToolRunTrace.body_key()), None);
    let spine_fields: Vec<&str> = contract["spineFields"].as_array().expect("spineFields").iter().map(|field| field.as_str().expect("spine field")).collect();
    let mut probe = Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    probe.snapshot = Some(crate::Canvas2dSnapshotLease::default());
    probe.tool_run_trace = Some(String::new());
    probe.lanes = vec![SceneLaneRef::default()];
    for key in serde_json::to_value(&probe).expect("serialize probe").as_object().expect("object").keys() {
        assert!(spine_fields.contains(&key.as_str()) || CANVAS2D_SCENE_LANE_FIELDS.contains(&key.as_str()), "scene field {key} is declared neither spine nor lane");
    }
}

#[test]
fn canvas2d_scene_splits_into_the_declared_lanes_and_merges_back() {
    let contract = canvas2d_lane_contract();
    let round_trip = &contract["roundTrip"];
    let assembled: Canvas2dScene = serde_json::from_value(round_trip["assembled"].clone()).expect("assembled scene");
    let (spine, lanes) = assembled.split_lanes();
    let expected_texts = round_trip["laneTexts"].as_object().expect("laneTexts object");
    assert_eq!(lanes.len(), expected_texts.len());
    for lane in &lanes {
        assert_eq!(Some(lane.payload.as_str()), expected_texts.get(lane.key).and_then(Value::as_str), "lane {} payload", lane.key);
    }
    assert_eq!(serde_json::to_value(&spine).expect("serialize spine"), round_trip["spine"]);
    assert_eq!(Canvas2dScene::decode_pack(&spine.encode_pack().expect("spine packs")).expect("spine unpacks"), spine);
    let mut merged = spine.clone();
    for lane in &lanes {
        assert!(merged.merge_lane(lane.key, lane.payload.clone()));
    }
    assert!(!merged.merge_lane(World3dSceneLane::ToolRunTrace.body_key(), String::new()));
    merged.lanes = Vec::new();
    assert_eq!(merged, assembled);
}
//#endregion 🚚️Canvas2dSceneLanes

//#region 🚚️Board2dSceneLanes
/// 🚚️ The board-2d twin of [`CANVAS2D_SCENE_LANE_CONTRACT`].
const BOARD2D_SCENE_LANE_CONTRACT: &str = include_str!("../../🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json");

fn board2d_lane_contract() -> Value {
    serde_json::from_str(BOARD2D_SCENE_LANE_CONTRACT).expect("board-2d lane contract parses")
}

#[test]
fn board2d_scene_lanes_mirror_the_language_neutral_declaration() {
    let contract = board2d_lane_contract();
    assert_eq!(contract["schema"].as_str(), Some(Board2dScene::SCHEMA));
    assert_eq!(contract["laneKeyPrefix"].as_str(), Some(BOARD2D_SCENE_LANE_KEY_PREFIX));
    let declared = contract["lanes"].as_array().expect("lanes array");
    assert_eq!(declared.len(), Board2dSceneLane::ALL.len());
    for (lane, entry) in Board2dSceneLane::ALL.into_iter().zip(declared) {
        assert_eq!(entry["lane"].as_str(), Some(lane.name()));
        assert_eq!(entry["field"].as_str(), Some(lane.field()));
        assert_eq!(entry["bodyKey"].as_str(), Some(lane.body_key()));
        assert_eq!(entry["optional"].as_bool(), Some(lane.optional()));
        assert_eq!(lane.body_key(), format!("{BOARD2D_SCENE_LANE_KEY_PREFIX}{}", lane.name()));
        assert_eq!(Board2dSceneLane::from_body_key(lane.body_key()), Some(lane));
        assert_eq!(Board2dSceneLane::from_name(lane.name()), Some(lane));
    }
    assert_eq!(Board2dSceneLane::from_body_key(Canvas2dSceneLane::ToolRunTrace.body_key()), None);
    let spine_fields: Vec<&str> = contract["spineFields"].as_array().expect("spineFields").iter().map(|field| field.as_str().expect("spine field")).collect();
    let mut probe = Board2dScene::base("{}".into(), "{}".into(), true);
    probe.hovered_id = Some(String::new());
    probe.active_utility = Some(String::new());
    probe.tool_run_trace = Some(String::new());
    probe.lanes = vec![SceneLaneRef::default()];
    for key in serde_json::to_value(&probe).expect("serialize probe").as_object().expect("object").keys() {
        assert!(spine_fields.contains(&key.as_str()) || BOARD2D_SCENE_LANE_FIELDS.contains(&key.as_str()), "scene field {key} is declared neither spine nor lane");
    }
}

#[test]
fn board2d_scene_splits_into_the_declared_lanes_and_merges_back() {
    let contract = board2d_lane_contract();
    let round_trip = &contract["roundTrip"];
    let assembled: Board2dScene = serde_json::from_value(round_trip["assembled"].clone()).expect("assembled scene");
    let (spine, lanes) = assembled.split_lanes();
    let expected_texts = round_trip["laneTexts"].as_object().expect("laneTexts object");
    assert_eq!(lanes.len(), expected_texts.len());
    for lane in &lanes {
        assert_eq!(Some(lane.payload.as_str()), expected_texts.get(lane.key).and_then(Value::as_str), "lane {} payload", lane.key);
    }
    assert_eq!(serde_json::to_value(&spine).expect("serialize spine"), round_trip["spine"]);
    assert_eq!(Board2dScene::decode_pack(&spine.encode_pack().expect("spine packs")).expect("spine unpacks"), spine);
    assert_eq!(Board2dScene::from_value(assembled.to_value()).expect("value round trip"), assembled);
    let mut merged = spine.clone();
    for lane in &lanes {
        assert!(merged.merge_lane(lane.key, lane.payload.clone()));
    }
    assert!(!merged.merge_lane(Canvas2dSceneLane::ToolRunTrace.body_key(), String::new()));
    merged.lanes = Vec::new();
    assert_eq!(merged, assembled);
    let idle = Board2dScene::base("{}".into(), "{}".into(), false);
    assert!(idle.split_lanes().1.is_empty(), "an idle board publishes no trace carrier");
}
//#endregion 🚚️Board2dSceneLanes
