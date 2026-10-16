//! 🔀️ Unit proofs of the node-graph scene: hover and interaction records, highlighted ids and the neutral lane contract.

use super::*;
use serde_json::Value;

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
fn node_graph_interaction_domain_is_closed_non_empty_and_identical_on_both_encoders() {
    let mut scene = NodeGraphScene::base(Vec::new(), Vec::new(), Viewport2d::default());
    scene.interaction_domain = Some(NodeGraphInteractionDomain {
        id: "graph".into(),
        node_target_prefix: "flow-play-document.widget.".into(),
        edge_target_prefix: "flow-play-document.synapse.".into(),
        handle_target_prefix: "flow-play-document.handle.".into(),
    });
    let json = serde_json::to_value(&scene).expect("serialize");
    assert_eq!(
        json["interactionDomain"],
        serde_json::json!({
            "id": "graph",
            "nodeTargetPrefix": "flow-play-document.widget.",
            "edgeTargetPrefix": "flow-play-document.synapse.",
            "handleTargetPrefix": "flow-play-document.handle."
        })
    );
    assert_eq!(serde_json::from_value::<NodeGraphScene>(json).expect("serde round trip"), scene);
    assert_eq!(NodeGraphScene::from_value(scene.to_value()).expect("value round trip"), scene);
    assert!(serde_json::to_value(NodeGraphScene::base(Vec::new(), Vec::new(), Viewport2d::default())).expect("serialize bare").get("interactionDomain").is_none());
    for invalid in [
        serde_json::json!({ "id": "graph", "nodeTargetPrefix": "", "edgeTargetPrefix": "edge.", "handleTargetPrefix": "handle." }),
        serde_json::json!({ "id": "graph", "nodeTargetPrefix": "node.", "edgeTargetPrefix": "edge." }),
        serde_json::json!({ "id": "graph", "nodeTargetPrefix": "node.", "edgeTargetPrefix": "edge.", "handleTargetPrefix": "handle.", "legacyPrefix": "legacy." }),
    ] {
        assert!(serde_json::from_value::<NodeGraphInteractionDomain>(invalid).is_err());
    }
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

#[test]
fn node_graph_lanes_pin_the_neutral_contract_and_restore_oversized_documents() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🚚️node-graph-scene-lanes/🔣️.json")).unwrap();
    let definitions = fixture["lanes"].as_array().unwrap();
    assert_eq!(definitions.len(), NodeGraphSceneLane::ALL.len());
    for (lane, definition) in NodeGraphSceneLane::ALL.into_iter().zip(definitions) {
        assert_eq!(lane.name(), definition["lane"].as_str().unwrap());
        assert_eq!(lane.field(), definition["field"].as_str().unwrap());
        assert_eq!(lane.body_key(), definition["bodyKey"].as_str().unwrap());
        assert_eq!(NodeGraphSceneLane::from_body_key(lane.body_key()), Some(lane));
    }
    let sample: NodeGraphScene = serde_json::from_value(fixture["sample"].clone()).unwrap();
    let (sample_spine, sample_lanes) = sample.split_lanes();
    assert_eq!(sample_lanes.len(), NodeGraphSceneLane::ALL.len());
    let mut sample_restored = sample_spine;
    for lane in sample_lanes { assert!(sample_restored.merge_lane(lane.key, lane.payload)); }
    sample_restored.lanes.clear();
    assert_eq!(sample_restored, sample);
    let count = fixture["nodeCount"].as_u64().unwrap() as usize;
    let node: NodeGraphNodeRecord = serde_json::from_value(fixture["node"].clone()).unwrap();
    let nodes = (0..count).map(|index| NodeGraphNodeRecord { id: index.to_string(), ..node.clone() }).collect();
    let mut scene = NodeGraphScene::base(nodes, Vec::new(), Viewport2d::default());
    scene.host_snapshot_json = Some(serde_json::json!({ "label": fixture["hostSnapshotLabel"].as_str().unwrap().repeat(fixture["repeatCount"].as_u64().unwrap() as usize) }).to_string());
    scene.selection = vec!["4095".into()];
    let original_bytes = scene.encode_pack().unwrap().len();
    assert!(original_bytes > ui_contract::UI_FIXED_BYTES);
    let (spine, lanes) = scene.split_lanes();
    let spine_bytes = spine.encode_pack().unwrap().len();
    assert!(spine_bytes < ui_contract::UI_FIXED_BYTES);
    assert!(spine.nodes.is_empty());
    assert!(spine.host_snapshot_json.is_none());
    assert_eq!(NodeGraphScene::from_value(spine.to_value()).unwrap(), spine);
    assert_eq!(NodeGraphScene::decode_pack(&spine.encode_pack().unwrap()).unwrap(), spine);
    let mut restored = spine.clone();
    for lane in &lanes {
        let reference = spine.lanes.iter().find(|reference| NodeGraphSceneLane::from_body_key(lane.key).unwrap().name() == reference.lane).unwrap();
        assert_eq!(reference.bytes as usize, lane.payload.len());
        assert_eq!(reference.hash, scene_lane_hash(&lane.payload));
        let definition = definitions.iter().find(|definition| definition["bodyKey"] == lane.key).unwrap();
        if definition["encoding"] == "json" {
            let oracle: Value = serde_json::from_str(&lane.payload).unwrap();
            let mut expected = serde_json::to_value(&scene).unwrap();
            expected[definition["field"].as_str().unwrap()] = oracle;
            assert_eq!(serde_json::from_value::<NodeGraphScene>(expected).unwrap(), scene);
        }
        assert!(restored.merge_lane(lane.key, lane.payload.clone()));
    }
    restored.lanes.clear();
    assert_eq!(restored, scene);
    assert!(!restored.merge_lane("framework.scene.nodeGraph.nodes", "[truncated".into()));
    assert!(!restored.merge_lane("framework.scene.world3d.meshes", "[]".into()));
    assert_eq!(restored, scene);
    println!("[DEBUG] node graph lane transport: nodes={count}, full={original_bytes}B, spine={spine_bytes}B, carriers={}", lanes.len());
}
