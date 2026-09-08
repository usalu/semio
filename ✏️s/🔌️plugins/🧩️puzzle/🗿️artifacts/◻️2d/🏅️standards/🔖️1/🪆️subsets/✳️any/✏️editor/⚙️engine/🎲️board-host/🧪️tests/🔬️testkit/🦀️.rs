//! 🧪️ The one board-scene test harness — `🦀️linking.rs` and `🦀️brush.rs` build on it instead of
//! re-deriving a camera/LOD/scene scaffold of their own.
use crate::editor::puzzle2d::engine::{BoardHost, EdgeDescJson, HandleDescJson, NodeDescJson, SceneDescriptorJson};
use serde_json::json;

pub trait BoardHostTestEvents {
    fn drain_events_json(&mut self) -> String;
}

impl BoardHostTestEvents for BoardHost {
    fn drain_events_json(&mut self) -> String {
        crate::editor::puzzle2d::drain_board_events_json(self)
    }
}

pub fn set_detail_lod(h: &mut BoardHost) {
    h.set_camera(0.0, 0.0, 2.0);
}

/// 🗂️ Board kind-catalog JSON for a compile-time manifest id — the catalogs live in the manifest
/// registry (`semio_framework_graph::manifest`), not in fixture `meta.kindCatalogs`, so tests that
/// need real node/handle kinds read them from there. Each catalog row is the manifest row's
/// `id`/`name` merged with its flattened `presentation` object.
pub fn catalogs_json_from_manifest_id(manifest_id: &str) -> String {
    let manifest = semio_framework_graph::manifest::manifest_by_id(manifest_id).unwrap_or_else(|| panic!("unknown manifest id {manifest_id}"));
    let rows = |kinds: &[semio_framework_graph::manifest::KindDef]| -> Vec<serde_json::Value> {
        kinds
            .iter()
            .map(|kind| {
                let mut row = serde_json::Map::new();
                row.insert("id".to_string(), json!(kind.id));
                row.insert("name".to_string(), json!(kind.name));
                if let Some(presentation) = kind.presentation.as_ref().and_then(|value| value.as_object()) {
                    for (key, value) in presentation {
                        row.insert(key.clone(), serde_json::from_str(&dsl::json::from_dsl_value(value).to_string()).expect("kind presentation JSON"));
                    }
                }
                serde_json::Value::Object(row)
            })
            .collect()
    };
    let visual_port_kinds: Vec<semio_framework_graph::manifest::KindDef> = manifest.port_kinds.iter().filter(|kind| kind.presentation.as_ref().is_some_and(|p| p.get("color").is_some())).cloned().collect();
    json!({ "handleKinds": rows(&visual_port_kinds), "nodeKinds": rows(&manifest.node_kinds) }).to_string()
}

pub fn set_micro_lod(h: &mut BoardHost) {
    h.set_camera(0.0, 60.0, 4.5);
}

pub fn set_overview_lod(h: &mut BoardHost) {
    h.set_camera(0.0, 0.0, 0.25);
}

pub fn sample_scene() -> SceneDescriptorJson {
    SceneDescriptorJson {
        nodes: vec![NodeDescJson {
            id: "a".into(),
            x: 0.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        }],
        handles: vec![
            HandleDescJson { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, selected: None, style: None, handle_kind: Some("port".into()), color: None, icon_kind: None, user_data: None, visible: None, locked: None, scale: None },
            HandleDescJson {
                id: "b:h0".into(),
                node_id: "b".into(),
                angle: std::f64::consts::PI,
                radius: None,
                selected: None,
                style: None,
                handle_kind: Some("port".into()),
                color: None,
                icon_kind: None,
                user_data: None,
                visible: None,
                locked: None,
                scale: None,
            },
        ],
        edges: vec![EdgeDescJson { id: "e1".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None }],
        wires: vec![],
        selection_exit_highlight_ids: vec![],
    }
}

pub fn link_test_scene_no_edge() -> SceneDescriptorJson {
    SceneDescriptorJson {
        nodes: vec![
            NodeDescJson {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: None,
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            },
            NodeDescJson {
                id: "b".into(),
                x: 280.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: None,
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            },
        ],
        handles: vec![
            HandleDescJson { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, selected: None, style: None, handle_kind: Some("parent".into()), color: None, icon_kind: None, user_data: None, visible: None, locked: None, scale: None },
            HandleDescJson {
                id: "b:h0".into(),
                node_id: "b".into(),
                angle: std::f64::consts::PI,
                radius: None,
                selected: None,
                style: None,
                handle_kind: Some("child".into()),
                color: None,
                icon_kind: None,
                user_data: None,
                visible: None,
                locked: None,
                scale: None,
            },
        ],
        edges: vec![],
        wires: vec![],
        selection_exit_highlight_ids: vec![],
    }
}

pub fn link_test_scene_no_edge_non_draggable_nodes() -> SceneDescriptorJson {
    let mut s = link_test_scene_no_edge();
    for n in &mut s.nodes {
        n.draggable = Some(false);
    }
    s
}

pub fn link_test_scene_node_a_two_free_handles() -> SceneDescriptorJson {
    let mut s = link_test_scene_no_edge();
    s.handles.push(HandleDescJson {
        id: "a:h1".into(),
        node_id: "a".into(),
        angle: std::f64::consts::FRAC_PI_2,
        radius: None,
        selected: None,
        style: None,
        handle_kind: Some("parent".into()),
        color: None,
        icon_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        scale: None,
    });
    s
}

pub fn link_test_scene_b_two_free_child_handles() -> SceneDescriptorJson {
    let mut s = link_test_scene_no_edge();
    s.handles.push(HandleDescJson {
        id: "b:h1".into(),
        node_id: "b".into(),
        angle: 0.0,
        radius: None,
        selected: None,
        style: None,
        handle_kind: Some("child".into()),
        color: None,
        icon_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        scale: None,
    });
    s
}

pub fn link_test_scene_target_b_handle_busy() -> SceneDescriptorJson {
    let mut s = link_test_scene_no_edge();
    s.nodes.push(NodeDescJson {
        id: "c".into(),
        x: 560.0,
        y: 0.0,
        draggable: Some(true),
        selected: None,
        style: None,
        text: None,
        icon_kind: None,
        node_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        root: None,
        shape: Some("circle".into()),
        radius: Some(40.0),
        width: None,
        height: None,
        scale: None,
    });
    s.handles.push(HandleDescJson {
        id: "c:h0".into(),
        node_id: "c".into(),
        angle: std::f64::consts::PI,
        radius: None,
        selected: None,
        style: None,
        handle_kind: Some("child".into()),
        color: None,
        icon_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        scale: None,
    });
    s.edges.push(EdgeDescJson { id: "e-bc".into(), source: "b:h0".into(), target: "c:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None });
    s
}

pub fn link_test_scene_a_to_b_linked() -> SceneDescriptorJson {
    let mut s = link_test_scene_no_edge();
    s.edges.push(EdgeDescJson { id: "e-ab".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None });
    s
}

pub fn link_test_scene_node_a_two_handles_one_busy() -> SceneDescriptorJson {
    let mut s = link_test_scene_a_to_b_linked();
    s.handles.push(HandleDescJson {
        id: "a:h1".into(),
        node_id: "a".into(),
        angle: std::f64::consts::FRAC_PI_2,
        radius: None,
        selected: None,
        style: None,
        handle_kind: Some("parent".into()),
        color: None,
        icon_kind: None,
        user_data: None,
        visible: None,
        locked: None,
        scale: None,
    });
    s
}
