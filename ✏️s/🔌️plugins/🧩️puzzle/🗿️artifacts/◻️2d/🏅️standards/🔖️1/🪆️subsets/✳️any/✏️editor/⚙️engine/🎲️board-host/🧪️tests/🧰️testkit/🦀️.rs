//! 🧪️ The one board-scene test harness — `🦀️linking.rs` and `🦀️brush.rs` build on it instead of
//! re-deriving a camera/LOD/scene scaffold of their own.
use crate::editor::puzzle2d::engine::{BoardHost, EdgeDescriptor, HandleDescriptor, NodeDescriptor, SceneDescriptor};
use semio_framework_pack_json::json;

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
pub const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

pub trait BoardHostTestEvents {
    fn drain_events_json(&mut self) -> String;
}

impl BoardHostTestEvents for BoardHost {
    fn drain_events_json(&mut self) -> String {
        crate::editor::puzzle2d::engine::board_host::drain_board_events_json(self)
    }
}

/// ♻️ Closes a board host through its own retirement ladder before release, exactly as the
/// production owner does (`EngineCanvas`'s wgpu target wraps the host in `BoardHostRetirement`
/// and pumps `close_step` until it reports terminal-empty). A host that ADMITTED resources — an
/// icon paint, a world scene, a preview — must reach terminal-empty through `close_step`, and
/// releasing one raw trips the framework's own witness ("IconPaintCache with admitted resources
/// must reach terminal-empty through close_step before release"). A host that never admitted
/// anything is allowed to be dropped, which is why only the laws that paint really need this.
pub fn close_board_host(host: BoardHost) {
    let mut retirement = crate::editor::puzzle2d::engine::BoardHostRetirement::new(host);
    let mut original = semio_s_plugin_puzzle_jobs::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1),
        semio_s_plugin_puzzle_jobs::testing::fixture_stock(), semio_framework_job::default_now_us,
        u64::MAX, 1 << 48,
    );
    for turn in 0..1_u32 << 20 {
        if original.turn(semio_s_plugin_puzzle_jobs::testing::fixture_stock(), |cx| Ok(retirement.close_step(cx))).expect("original BoardHost retirement") {
            assert!(turn > 0, "a board host that admitted resources takes more than one close turn");
            original.finish();
            return;
        }
    }
    panic!("board host never reached terminal-empty through its retirement ladder");
}

pub fn set_detail_lod(h: &mut BoardHost) {
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 2.0);
}

/// 🗂️ Board kind-catalog JSON for a compile-time manifest id — the catalogs live in the manifest
/// registry (`semio_framework_graph_engine::manifest`), not in snapshot `meta.kindCatalogs`, so tests that
/// need real node/handle kinds read them from there. Each catalog row is the manifest row's
/// `id`/`name` merged with its flattened `presentation` object.
pub fn catalogs_json_from_manifest_id(manifest_id: &str) -> String {
    let manifest = crate::standards::v1::subsets::any::schema::graph_manifest::manifest_by_id(manifest_id).unwrap_or_else(|| panic!("unknown manifest id {manifest_id}"));
    let rows = |kinds: &[semio_framework_graph_manifest::KindDef]| -> Vec<semio_framework_pack_json::Value> {
        kinds
            .iter()
            .map(|kind| {
                let mut row = semio_framework_pack_json::Object::new();
                row.insert("id".to_string(), json!(kind.id));
                row.insert("name".to_string(), json!(kind.name));
                if let Some(presentation) = kind.presentation.as_ref().and_then(|value| value.as_object()) {
                    for (key, value) in presentation {
                        row.insert(key.clone(), semio_framework_pack_json::from_json_str(&semio_framework_pack_json::from_dsl_value(value).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("kind presentation JSON"));
                    }
                }
                semio_framework_pack_json::Value::Object(row)
            })
            .collect()
    };
    let visual_port_kinds: Vec<semio_framework_graph_manifest::KindDef> = manifest.port_kinds.iter().filter(|kind| kind.presentation.as_ref().is_some_and(|p| p.get("color").is_some())).cloned().collect();
    json!({ "handleKinds": rows(&visual_port_kinds), "nodeKinds": rows(&manifest.node_kinds) }).to_string()
}

pub fn set_micro_lod(h: &mut BoardHost) {
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 60.0, 4.5);
}

pub fn set_overview_lod(h: &mut BoardHost) {
    board_set_camera(&mut h.scene, &mut h.outbox, 0.0, 0.0, 0.25);
}

pub fn sample_scene() -> SceneDescriptor {
    SceneDescriptor {
        nodes: vec![NodeDescriptor {
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
        }].into(),
        handles: vec![
            HandleDescriptor { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, selected: None, style: None, handle_kind: Some("port".into()), color: None, icon_kind: None, user_data: None, visible: None, locked: None, scale: None },
            HandleDescriptor {
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
        ].into(),
        edges: vec![EdgeDescriptor { id: "e1".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None }].into(),
        wires: vec![].into(),
        regions: vec![],
        selection_exit_highlight_ids: vec![],
    }
}

pub fn link_test_scene_no_edge() -> SceneDescriptor {
    SceneDescriptor {
        nodes: vec![
            NodeDescriptor {
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
            NodeDescriptor {
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
        ].into(),
        handles: vec![
            HandleDescriptor { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, selected: None, style: None, handle_kind: Some("parent".into()), color: None, icon_kind: None, user_data: None, visible: None, locked: None, scale: None },
            HandleDescriptor {
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
        ].into(),
        edges: vec![].into(),
        wires: vec![].into(),
        regions: vec![],
        selection_exit_highlight_ids: vec![],
    }
}

pub fn link_test_scene_no_edge_non_draggable_nodes() -> SceneDescriptor {
    let mut s = link_test_scene_no_edge();
    for n in &mut s.nodes {
        n.draggable = Some(false);
    }
    s
}

pub fn link_test_scene_node_a_two_free_handles() -> SceneDescriptor {
    let mut s = link_test_scene_no_edge();
    s.handles.push(HandleDescriptor {
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

pub fn link_test_scene_b_two_free_child_handles() -> SceneDescriptor {
    let mut s = link_test_scene_no_edge();
    s.handles.push(HandleDescriptor {
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

pub fn link_test_scene_target_b_handle_busy() -> SceneDescriptor {
    let mut s = link_test_scene_no_edge();
    s.nodes.push(NodeDescriptor {
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
    s.handles.push(HandleDescriptor {
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
    s.edges.push(EdgeDescriptor { id: "e-bc".into(), source: "b:h0".into(), target: "c:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None });
    s
}

pub fn link_test_scene_a_to_b_linked() -> SceneDescriptor {
    let mut s = link_test_scene_no_edge();
    s.edges.push(EdgeDescriptor { id: "e-ab".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None });
    s
}

pub fn link_test_scene_node_a_two_handles_one_busy() -> SceneDescriptor {
    let mut s = link_test_scene_a_to_b_linked();
    s.handles.push(HandleDescriptor {
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
