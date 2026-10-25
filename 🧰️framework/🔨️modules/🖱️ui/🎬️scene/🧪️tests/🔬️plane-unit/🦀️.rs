//! 🪧️ Unit proofs of the flat-surface scenes: canvas, board, tiled map, ink canvas and paint lane contracts and spine bounds.

use super::*;
use serde_json::Value;

//#region 🚚️World3dSceneLanes

#[test]
fn a_scene_without_lanes_still_publishes_its_whole_doc() {
    let scene = Canvas2dScene::base(1.0, 2.0, 3.0, "[]".into());
    let (spine, lanes) = scene.split_lanes();
    assert!(lanes.is_empty());
    assert_eq!(spine, scene);
}

//#endregion 🚚️World3dSceneLanes

//#region 🚚️Canvas2dSceneLanes
/// 🚚️ The canvas-2d twin of [`WORLD3D_SCENE_LANE_CONTRACT`].
const CANVAS2D_SCENE_LANE_CONTRACT: &str = include_str!("../🧫️fixtures/🚚️canvas2d-scene-lanes/🔣️.json");

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
    assert_eq!(Canvas2dSceneLane::from_body_key("framework.scene.world3d.toolRunTrace"), None);
    let spine_fields: Vec<&str> = contract["spineFields"].as_array().expect("spineFields").iter().map(|field| field.as_str().expect("spine field")).collect();
    let mut probe = Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    probe.snapshot = Some(Canvas2dSnapshotLease::default());
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
    assert!(!merged.merge_lane("framework.scene.world3d.toolRunTrace", String::new()));
    merged.lanes = Vec::new();
    assert_eq!(merged, assembled);
}
//#endregion 🚚️Canvas2dSceneLanes

//#region 🚚️Board2dSceneLanes
/// 📜️ The board-2d twin of [`CANVAS2D_SCENE_LANE_CONTRACT`].
const BOARD2D_SCENE_LANE_CONTRACT: &str = include_str!("../🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json");

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
    probe.placement_naming = Some(String::new());
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
    let idle_lanes = idle.split_lanes().1;
    assert_eq!(idle_lanes.len(), 1, "an idle board publishes its fixture carrier and no trace carrier");
    assert_eq!(idle_lanes[0].key, Board2dSceneLane::Snapshot.body_key());
}
//#endregion 🚚️Board2dSceneLanes

//#region 🚚️TiledMapSceneLanes
/// 🀄️ The tiled-map twin of [`BOARD2D_SCENE_LANE_CONTRACT`].
const TILEDMAP_SCENE_LANE_CONTRACT: &str = include_str!("../🧫️fixtures/🚚️tiledmap-scene-lanes/🔣️.json");

fn tiledmap_lane_contract() -> Value {
    serde_json::from_str(TILEDMAP_SCENE_LANE_CONTRACT).expect("tiled-map lane contract parses")
}

#[test]
fn tiledmap_scene_lanes_mirror_the_language_neutral_declaration() {
    let contract = tiledmap_lane_contract();
    assert_eq!(contract["schema"].as_str(), Some(TiledMapScene::SCHEMA));
    assert_eq!(contract["laneKeyPrefix"].as_str(), Some(TILEDMAP_SCENE_LANE_KEY_PREFIX));
    let declared = contract["lanes"].as_array().expect("lanes array");
    assert_eq!(declared.len(), TiledMapSceneLane::ALL.len());
    for (lane, entry) in TiledMapSceneLane::ALL.into_iter().zip(declared) {
        assert_eq!(entry["lane"].as_str(), Some(lane.name()));
        assert_eq!(entry["field"].as_str(), Some(lane.field()));
        assert_eq!(entry["bodyKey"].as_str(), Some(lane.body_key()));
        assert_eq!(entry["optional"].as_bool(), Some(lane.optional()));
        assert_eq!(lane.body_key(), format!("{TILEDMAP_SCENE_LANE_KEY_PREFIX}{}", lane.name()));
        assert_eq!(TiledMapSceneLane::from_body_key(lane.body_key()), Some(lane));
        assert_eq!(TiledMapSceneLane::from_name(lane.name()), Some(lane));
    }
    assert_eq!(TiledMapSceneLane::from_body_key(Board2dSceneLane::Snapshot.body_key()), None);
    let spine_fields: Vec<&str> = contract["spineFields"].as_array().expect("spineFields").iter().map(|field| field.as_str().expect("spine field")).collect();
    let mut probe = TiledMapScene::base("{}".into(), "{}".into());
    probe.lanes = vec![SceneLaneRef::default()];
    for key in serde_json::to_value(&probe).expect("serialize probe").as_object().expect("object").keys() {
        assert!(spine_fields.contains(&key.as_str()) || TILEDMAP_SCENE_LANE_FIELDS.contains(&key.as_str()), "scene field {key} is declared neither spine nor lane");
    }
}

/// 🗺️ The gis `demo` map is 59 667 bytes of descriptor — the surface doc is a hard 32 KiB that cannot
/// page, so before this lane `scene_surface.encode` refused the whole surface and the gis window
/// never published. The spine has to stay bounded no matter how large the fixture gets.
#[test]
fn tiledmap_scene_splits_into_the_declared_lanes_and_merges_back() {
    let contract = tiledmap_lane_contract();
    let round_trip = &contract["roundTrip"];
    let assembled: TiledMapScene = serde_json::from_value(round_trip["assembled"].clone()).expect("assembled scene");
    let (spine, lanes) = assembled.split_lanes();
    let expected_texts = round_trip["laneTexts"].as_object().expect("laneTexts object");
    assert_eq!(lanes.len(), expected_texts.len());
    for lane in &lanes {
        assert_eq!(Some(lane.payload.as_str()), expected_texts.get(lane.key).and_then(Value::as_str), "lane {} payload", lane.key);
    }
    assert_eq!(serde_json::to_value(&spine).expect("serialize spine"), round_trip["spine"]);
    assert_eq!(TiledMapScene::decode_pack(&spine.encode_pack().expect("spine packs")).expect("spine unpacks"), spine);
    assert_eq!(TiledMapScene::from_value(assembled.to_value()).expect("value round trip"), assembled);
    let mut merged = spine.clone();
    for lane in &lanes {
        assert!(merged.merge_lane(lane.key, lane.payload.clone()));
    }
    assert!(!merged.merge_lane(Board2dSceneLane::Snapshot.body_key(), String::new()));
    merged.lanes = Vec::new();
    assert_eq!(merged, assembled);
    let huge = TiledMapScene::base("x".repeat(ui_contract::UI_FIXED_BYTES * 2), "{}".into());
    let (huge_spine, huge_lanes) = huge.split_lanes();
    assert_eq!(huge_lanes.len(), 1);
    assert!(huge_spine.encode_pack().expect("spine packs").len() < ui_contract::UI_FIXED_BYTES, "the spine stays admissible however large the map fixture is");
}
//#endregion 🚚️TiledMapSceneLanes

/// ⚖️ Law: `InkCanvasScene`'s TWO encoders — serde (`rename_all = "camelCase"`, the pack wire every
/// producer actually travels) and its hand-written `ToValue`/`FromValue` — must spell the document
/// payload identically. They did not: `ToValue` wrote `"snapshotJson"` while serde wrote
/// `"documentJson"`, which is the key React's `InkCanvasHost` reads (`parseInkScene(scene?.documentJson)`,
/// `🖋️InkCanvasHost/🟦️.tsx:971`). A scene handed to the DSL-value transport therefore carried a key
/// nothing read, and the document silently vanished.
#[test]
fn ink_canvas_document_payload_is_document_json_on_both_encoders() {
    let mut scene = InkCanvasScene::base("{\"items\":[]}".into(), "selectDirect".into(), "composite".into(), true);
    scene.interaction_domain = Some(InkCanvasInteractionDomain { id: "blocks".into(), granularity_id: "block".into() });
    let json = serde_json::to_value(&scene).expect("serialize");
    assert_eq!(json.get("documentJson").and_then(Value::as_str), Some("{\"items\":[]}"), "serde must spell React's own key");
    assert!(json.get("snapshotJson").is_none(), "the camelCase holdout must be gone from the serde wire");
    let value = scene.to_value();
    let entries = match &value {
        DslValue::Object(entries) => entries.clone(),
        other => panic!("ink canvas scene encodes as an object, got {other:?}"),
    };
    assert!(entries.iter().any(|(key, _)| key == "documentJson"), "ToValue must spell the same key as serde: {:?}", entries.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>());
    assert!(!entries.iter().any(|(key, _)| key == "snapshotJson"));
    assert_eq!(InkCanvasScene::from_value(value).expect("value round trip"), scene);
    assert_eq!(json["interactionDomain"], serde_json::json!({ "id": "blocks", "granularityId": "block" }));
    assert_eq!(serde_json::from_value::<InkCanvasScene>(json).expect("serde round trip"), scene);

    let bare = InkCanvasScene::base("{}".into(), "selectDirect".into(), "composite".into(), true);
    assert!(serde_json::to_value(&bare).expect("serialize bare").get("interactionDomain").is_none());
    for invalid in [
        serde_json::json!({ "id": "blocks" }),
        serde_json::json!({ "granularityId": "block" }),
        serde_json::json!({ "id": "", "granularityId": "block" }),
        serde_json::json!({ "id": "blocks", "granularityId": "" }),
        serde_json::json!({ "id": "blocks", "granularityId": "block", "domainId": "legacy" }),
    ] {
        assert!(serde_json::from_value::<InkCanvasInteractionDomain>(invalid).is_err());
    }
    for invalid in [
        DslValue::Object(vec![("id".into(), DslValue::String("blocks".into()))]),
        DslValue::Object(vec![("id".into(), DslValue::String(String::new())), ("granularityId".into(), DslValue::String("block".into()))]),
        DslValue::Object(vec![("id".into(), DslValue::String("blocks".into())), ("granularityId".into(), DslValue::String("block".into())), ("domainId".into(), DslValue::String("legacy".into()))]),
        DslValue::Object(vec![("id".into(), DslValue::String("blocks".into())), ("id".into(), DslValue::String("duplicate".into())), ("granularityId".into(), DslValue::String("block".into()))]),
    ] {
        assert!(InkCanvasInteractionDomain::from_value(invalid).is_err());
    }
}

//#region 🚚️Paint2dSceneLanes
/// 🖌️ The paint-2d twin of [`CANVAS2D_SCENE_LANE_CONTRACT`].
const PAINT2D_SCENE_LANE_CONTRACT: &str = include_str!("../🧫️fixtures/🚚️paint2d-scene-lanes/🔣️.json");

fn paint2d_lane_contract() -> Value {
    serde_json::from_str(PAINT2D_SCENE_LANE_CONTRACT).expect("paint-2d lane contract parses")
}

#[test]
fn paint2d_scene_lanes_mirror_the_language_neutral_declaration() {
    let contract = paint2d_lane_contract();
    assert_eq!(contract["schema"].as_str(), Some(Paint2dScene::SCHEMA));
    assert_eq!(contract["laneKeyPrefix"].as_str(), Some(PAINT2D_SCENE_LANE_KEY_PREFIX));
    let declared = contract["lanes"].as_array().expect("lanes array");
    assert_eq!(declared.len(), Paint2dSceneLane::ALL.len());
    for (lane, entry) in Paint2dSceneLane::ALL.into_iter().zip(declared) {
        assert_eq!(entry["lane"].as_str(), Some(lane.name()));
        assert_eq!(entry["field"].as_str(), Some(lane.field()));
        assert_eq!(entry["bodyKey"].as_str(), Some(lane.body_key()));
        assert_eq!(entry["optional"].as_bool(), Some(lane.optional()));
        assert_eq!(lane.body_key(), format!("{PAINT2D_SCENE_LANE_KEY_PREFIX}{}", lane.name()));
        assert_eq!(Paint2dSceneLane::from_body_key(lane.body_key()), Some(lane));
        assert_eq!(Paint2dSceneLane::from_name(lane.name()), Some(lane));
    }
    assert_eq!(Paint2dSceneLane::from_body_key(Canvas2dSceneLane::ToolRunTrace.body_key()), None);
    let spine_fields: Vec<&str> = contract["spineFields"].as_array().expect("spineFields").iter().map(|field| field.as_str().expect("spine field")).collect();
    let mut probe = paint2d_probe_scene();
    probe.hovered_id = Some(String::new());
    probe.composite_viewport_json = Some(String::new());
    probe.lanes = vec![SceneLaneRef::default()];
    for key in serde_json::to_value(&probe).expect("serialize probe").as_object().expect("object").keys() {
        assert!(spine_fields.contains(&key.as_str()) || PAINT2D_SCENE_LANE_FIELDS.contains(&key.as_str()), "scene field {key} is declared neither spine nor lane");
    }
}

fn paint2d_probe_scene() -> Paint2dScene {
    Paint2dScene {
        document_sync_json: "{}".into(),
        assets_json: "[]".into(),
        camera_json: "{}".into(),
        selection_json: "[]".into(),
        hovered_id: None,
        active_utility: "brush".into(),
        brush_size: 4.0,
        brush_opacity: 1.0,
        brush_color: "#e07020".into(),
        brush_hardness: 0.25,
        paint_target:"pixels".into(),mask_value:255,fill_tolerance:24,pixel_selection_json:None,
        view_mode: "composite".into(),
        composite_viewport_json: None,
        lanes: Vec::new(),
    }
}

/// ✂️ Document, assets and optional pixel coverage preserve their exact paged payloads.
#[test]
fn paint2d_scene_splits_into_the_declared_lanes_and_merges_back() {
    let contract = paint2d_lane_contract();
    let round_trip = &contract["roundTrip"];
    let assembled: Paint2dScene = serde_json::from_value(round_trip["assembled"].clone()).expect("assembled scene");
    let (spine, lanes) = assembled.split_lanes();
    let expected_texts = round_trip["laneTexts"].as_object().expect("laneTexts object");
    assert_eq!(lanes.len(), expected_texts.len());
    for lane in &lanes {
        assert_eq!(Some(lane.payload.as_str()), expected_texts.get(lane.key).and_then(Value::as_str), "lane {} payload", lane.key);
        use std::hash::Hasher;
        let mut oracle=fnv::FnvHasher::default();oracle.write(lane.payload.as_bytes());let expected=spine.lanes.iter().find(|reference|lane.key.ends_with(&reference.lane)).unwrap();assert_eq!(expected.hash,format!("{:016x}",oracle.finish()));assert_eq!(expected.bytes as usize,lane.payload.as_bytes().len());
    }
    assert_eq!(serde_json::to_value(&spine).expect("serialize spine"), round_trip["spine"]);
    assert_eq!(Paint2dScene::decode_pack(&spine.encode_pack().expect("spine packs")).expect("spine unpacks"), spine);
    assert_eq!(Paint2dScene::from_value(assembled.to_value()).expect("value round trip"), assembled);
    let mut merged = spine.clone();
    for lane in &lanes {
        assert!(merged.merge_lane(lane.key, lane.payload.clone()));
    }
    assert!(!merged.merge_lane(Canvas2dSceneLane::ToolRunTrace.body_key(), String::new()));
    merged.lanes = Vec::new();
    assert_eq!(merged, assembled);
}

/// 📕️ The defect this lane set closes: a raster document larger than the fixed surface doc used to
/// refuse admission entirely, because `Paint2dScene` had no `split_lanes` override at all.
#[test]
fn paint2d_spine_stays_inside_the_fixed_surface_doc_for_an_oversized_document() {
    let mut oversized = paint2d_probe_scene();
    oversized.document_sync_json = "x".repeat(64 * 1024);
    oversized.assets_json = "y".repeat(64 * 1024);
    oversized.pixel_selection_json=Some("s".repeat(40000));
    let (spine, lanes) = oversized.split_lanes();
    assert_eq!(lanes.len(), 3);
    let packed = spine.encode_pack().expect("oversized spine still packs");
    assert!(packed.len() < 4096, "the paint-2d spine must stay tiny whatever the document size, got {} bytes", packed.len());
    assert!(ui_contract::UiFixedBytes::try_from_vec(packed).is_ok(), "the spine must fit the fixed surface doc");
}
//#endregion 🚚️Paint2dSceneLanes

#[test]
fn paint2d_absent_pixel_selection_emits_no_coverage_lane(){
    let (spine,lanes)=paint2d_probe_scene().split_lanes();
    assert!(spine.pixel_selection_json.is_none());
    assert_eq!(lanes.len(),2);
    assert!(!spine.lanes.iter().any(|lane|lane.lane=="pixelSelection"));
}
