use super::*;
use crate::standards::v1::subsets::any::schema::{AssignmentJson, Lhs, PatternJson, Rhs};
use semio_framework_graph::dsl::Completion as JackCompletion;
use semio_framework_graph::manifest::PropertyValue;
use semio_s_artifact_trinity_jack::editor::jack::lod::trinity_lod_scale_json;
use semio_s_artifact_trinity_jack::lexer::TokenSpan as JackTokenSpan;
use store::ArtifactDsl;

#[test]
fn trinity_rewriting_envelope_credits_admit_exact_caps_and_reject_zero_or_plus_one() {
    assert!(trinity_rewriting_envelope_credits_are_valid(TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES, TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES));
    assert!(!trinity_rewriting_envelope_credits_are_valid(0, TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES));
    assert!(!trinity_rewriting_envelope_credits_are_valid(TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES, 0));
    assert!(!trinity_rewriting_envelope_credits_are_valid(TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES + 1, TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES));
    assert!(!trinity_rewriting_envelope_credits_are_valid(TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES, TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES + 1));
}

#[test]
fn trinity_rewriting_rejected_page_preserves_pointer_content_and_retry_owner() {
    let caller = std::rc::Rc::new(vec![1_u8, 2, 3, 4]);
    let pointer = caller.as_ptr();
    let mut rejected = TrinityRewritingCallerPageOwner::new(std::rc::Rc::clone(&caller));
    let retry = rejected.take_page().expect("exact rejected caller page");
    assert_eq!(retry.as_ptr(), pointer);
    assert_eq!(retry.as_slice(), &[1, 2, 3, 4]);
    assert!(!rejected.has_page());
    let retried = TrinityRewritingCallerPageOwner::new(retry);
    assert_eq!(retried.page.as_ref().expect("same retry owner").as_ptr(), pointer);
}

#[test]
fn trinity_rewriting_page_cap_plus_one_rejects_before_owner_construction() {
    let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }).expect("one exact page reservation");
    let bytes_plus_one = vec![8_u8; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1].into_boxed_slice();
    let bytes_plus_one_pointer = bytes_plus_one.as_ptr();
    assert_eq!(pages.preflight_page_bytes(bytes_plus_one.len()), Err(store::OwnedSchemaDecodeAdmissionFault::ByteCapacity));
    assert_eq!(bytes_plus_one.as_ptr(), bytes_plus_one_pointer);
    assert_eq!(bytes_plus_one[0], 8);
    let bytes = [7; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    store::ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, bytes.len()).admit_preflighted_into(&mut pages);
    let caller = Box::new([9; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]);
    let pointer = caller.as_ptr();
    assert_eq!(pages.preflight_page_bytes(caller.len()), Err(store::OwnedSchemaDecodeAdmissionFault::PageCapacity));
    assert_eq!(caller.as_ptr(), pointer);
    assert_eq!(caller[0], 9);
}

#[test]
fn trinity_rewriting_stale_generation_and_slot_aba_never_match() {
    assert!(trinity_rewriting_page_handle_matches(17, 23, 17, 23));
    assert!(!trinity_rewriting_page_handle_matches(17, 23, 17, 24));
    assert!(!trinity_rewriting_page_handle_matches(17, 23, 18, 23));
}

#[test]
fn trinity_rewriting_checked_out_drop_preserves_raw_caller_authority() {
    let caller = std::rc::Rc::new(vec![5_u8; 64]);
    let pointer = caller.as_ptr();
    let checked_out = TrinityRewritingCallerPageOwner::new(std::rc::Rc::clone(&caller));
    drop(checked_out);
    assert_eq!(caller.as_ptr(), pointer);
    assert_eq!(caller.as_slice(), &[5_u8; 64]);
    assert_eq!(std::rc::Rc::strong_count(&caller), 1);
}

#[test]
fn trinity_rewriting_rejected_page_close_retires_one_owner_per_grant() {
    let mut rejected = TrinityRewritingCallerPageOwner::new(Box::new([3_u8; 32]));
    assert!(!rejected.close_step());
    assert!(rejected.close_step());
    assert!(!rejected.has_page());
}

fn nakagin_graph() -> Graph {
    let dsl = include_str!("../../../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
    Graph::from_fixture(JackSnapshot::parse_dsl(dsl).unwrap()).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn nakagin_fixture_loads() {
    let g = nakagin_graph();
    assert_eq!(g.nodes.len(), 9);
    assert_eq!(g.edges.len(), 6);
}

#[semio_framework_async_macros::async_test]
async fn nakagin_flat_position_derived() {
    let g = nakagin_graph();
    let flat = semio_s_artifact_trinity_jack::compute_flat_position(&g.to_fixture());
    let root_uv = flat.positions.get("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(root_uv.u, 0.0);
    let capsule_uv = flat.positions.get("6947a41b-8c6d-4291-bdd8-96cd535c78fc").unwrap();
    assert!(capsule_uv.v > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_rebuilds_engine() {
    let host = TrinityBridge::from_graph(&nakagin_graph()).await;
    assert_eq!(host.engine.nodes.len(), 9);
    assert!(!host.engine.edges.is_empty());
    assert!(!host.engine.enforce_acyclic);
    assert_eq!(host.board.nodes.len(), 9);
    assert!(host.board.nodes.values().all(|node| matches!(node.shape, semio_framework_os_infinite::NodeShape::Circle)));
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_reorganize_moves_nodes() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    let before: Vec<(f64, f64)> = host.graph.nodes.values().map(|n| (n.x, n.y)).collect();
    host.reorganize().await;
    let after: Vec<(f64, f64)> = host.graph.nodes.values().map(|n| (n.x, n.y)).collect();
    assert_ne!(before, after);
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_tokenize_jack_json() {
    let host = TrinityBridge::from_graph(&nakagin_graph()).await;
    let json = host.tokenize_jack_json("MATCH (a:Piece)").unwrap();
    let tokens: Vec<JackTokenSpan> = pack::from_json_str(&json).unwrap();
    assert!(tokens.iter().any(|row| row.start == 0));
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_complete_jack_json() {
    let host = TrinityBridge::from_graph(&nakagin_graph()).await;
    let json = host.complete_jack_json("MAT", 3).unwrap();
    let items: Vec<JackCompletion> = pack::from_json_str(&json).unwrap();
    assert!(items.iter().any(|row| row.label == "MATCH"));
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_jack_create_undo() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    let before = host.graph.nodes.len();
    host.run_jack("CREATE (n:Piece)").await.unwrap();
    assert_eq!(host.graph.nodes.len(), before + 1);
    host.undo().await.unwrap();
    assert_eq!(host.graph.nodes.len(), before);
}

#[semio_framework_async_macros::async_test]
async fn trinity_lod_scale_json_lists_all_six_lods() {
    let json = trinity_lod_scale_json();
    let rows: Vec<pack::JsonValue> = pack::parse_json(&json).unwrap().as_array().unwrap().to_vec();
    assert_eq!(rows.len(), 6);
    assert_eq!(rows[0]["id"], "minimap");
    assert_eq!(rows[5]["id"], "micro");
}

#[semio_framework_async_macros::async_test]
async fn trinity_abbreviate_label_short_passthrough_and_long_truncated() {
    assert_eq!(trinity_abbreviate_label("abcd"), "abcd");
    assert_eq!(trinity_abbreviate_label("  abcd  "), "abcd");
    assert_eq!(trinity_abbreviate_label("abcdef"), "abc");
}

#[semio_framework_async_macros::async_test]
async fn trinity_draw_lod_from_id_and_visibility_flags() {
    assert_eq!(TrinityDrawLod::from_id("bogus"), None);
    assert_eq!(TrinityDrawLod::from_id("micro"), Some(TrinityDrawLod::Micro));
    assert!(TrinityDrawLod::Detail.handles_visible());
    assert!(!TrinityDrawLod::Normal.handles_visible());
    assert!(!TrinityDrawLod::Minimap.labels_visible());
    assert!(TrinityDrawLod::Compact.labels_visible());
    assert!(!TrinityDrawLod::Compact.full_labels());
    assert!(TrinityDrawLod::Detail.full_labels());
    assert_eq!(TrinityDrawLod::from_scale_index(5), TrinityDrawLod::Micro);
    assert_eq!(TrinityDrawLod::from_scale_index(0), TrinityDrawLod::Minimap);
}

#[semio_framework_async_macros::async_test]
async fn trinity_node_radius_uses_dimensions_or_default() {
    let mut node = Node { id: "n".into(), kind: "Piece".into(), name: "n".into(), x: 0.0, y: 0.0, width: 0.0, height: 0.0, properties: Default::default(), ports: vec![] };
    assert_eq!(trinity_node_radius(&node), 44.0);
    node.width = 10.0;
    node.height = 10.0;
    assert_eq!(trinity_node_radius(&node), TRINITY_DEFAULT_NODE_RADIUS * 0.5);
    node.width = 200.0;
    node.height = 10.0;
    assert_eq!(trinity_node_radius(&node), 100.0);
}

#[semio_framework_async_macros::async_test]
async fn trinity_circle_port_angle_left_right_spread() {
    assert!((trinity_circle_port_angle(0, 1, true) - std::f64::consts::PI).abs() < 1e-9);
    assert_eq!(trinity_circle_port_angle(0, 1, false), 0.0);
    assert!(trinity_circle_port_angle(0, 2, false) < trinity_circle_port_angle(1, 2, false));
}

#[semio_framework_async_macros::async_test]
async fn trinity_port_endpoint_parts_splits_on_at() {
    assert_eq!(trinity_port_endpoint_parts("node1@portA"), ("node1".to_string(), "portA".to_string()));
    assert_eq!(trinity_port_endpoint_parts("no-at"), ("no-at".to_string(), String::new()));
}

#[semio_framework_async_macros::async_test]
async fn trinity_port_handle_key_direction_prefix() {
    assert_eq!(trinity_port_handle_key("n", "p", true), "n:in:p");
    assert_eq!(trinity_port_handle_key("n", "p", false), "n:out:p");
}

#[semio_framework_async_macros::async_test]
async fn trinity_graph_to_board_fixture_includes_handles_and_edges() {
    let g = nakagin_graph();
    let fixture = trinity_graph_to_board_fixture(&g);
    assert_eq!(fixture["schema"], "puzzle.2d.fixture");
    assert_eq!(fixture["nodes"].as_array().unwrap().len(), 9);
    assert_eq!(fixture["edges"].as_array().unwrap().len(), 6);
    let root_node = fixture["nodes"].as_array().unwrap().iter().find(|n| n["id"] == "7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert!(!root_node["handles"].as_array().unwrap().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn force_layout_reposition_operations_produces_repositions() {
    let fixture = nakagin_graph().to_fixture();
    let operations = force_layout_reposition_operations(&fixture).unwrap();
    assert!(!operations.is_empty());
    assert!(operations.iter().all(|op| matches!(op, TrinityGraphMutation::MoveNode(_))));
}

#[semio_framework_async_macros::async_test]
async fn apply_force_layout_positions_errors_when_nodes_missing() {
    let mut g = nakagin_graph();
    let fixture = pack::json!({});
    let err = apply_force_layout_positions_to_trinity_graph(&mut g, &fixture).unwrap_err();
    assert!(matches!(err, TrinityRewritingError::ForceLayoutFixtureMissingNodes));
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_apply_rewriting_json_end_to_end() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    let rule = Rule {
        name: "label-core".into(),
        lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
        rhs: Rhs { create: vec![], delete: vec![], set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) }], merge: vec![], parameters: vec![] },
    };
    let rule_json = pack::to_json_string(&rule);
    let out = host.apply_rewriting_json(&rule_json, "{}").await.unwrap();
    let value: pack::JsonValue = pack::parse_json(&out).unwrap();
    assert!(value.get("fixture").is_some());
    let core = host.graph.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_run_jack_json_and_with_fixture() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    let json = host.run_jack_json("MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name").await.unwrap();
    let result: QueryResult = pack::from_json_str(&json).unwrap();
    assert_eq!(result.rows.len(), 1);

    let before = host.graph.nodes.len();
    let out = host.run_jack_with_fixture_json("CREATE (n:Piece)").await.unwrap();
    let value: pack::JsonValue = pack::parse_json(&out).unwrap();
    assert!(value.get("fixtureJson").is_some());
    assert_eq!(host.graph.nodes.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_selected_and_highlighted_node_ids() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    host.set_viewport(800, 600, 1.0);
    host.pointer_down(400.0, 300.0, false);
    let json = host.selected_node_ids_json().unwrap();
    let ids: Vec<String> = pack::from_json_str(&json).unwrap();
    assert_eq!(ids, vec!["7dc5b737-3b6b-4068-b315-b7bacc91c2e1".to_string()]);
    assert!(host.set_highlighted_node_ids_json("[\"7dc5b737-3b6b-4068-b315-b7bacc91c2e1\"]").is_ok());
    assert_eq!(host.node_overlays_json().unwrap(), "[]");
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_viewport_camera_and_wheel() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    host.set_viewport(800, 600, 1.0);
    let before_zoom = host.graph.camera.zoom;
    host.wheel_screen(400.0, 300.0, -100.0);
    assert!(host.graph.camera.zoom > before_zoom);
    host.set_camera(10.0, 20.0, 2.0);
    assert_eq!((host.graph.camera.x, host.graph.camera.y, host.graph.camera.zoom), (10.0, 20.0, 2.0));
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_pointer_drag_commits_position() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    host.set_viewport(800, 600, 1.0);
    let node_id = "7dc5b737-3b6b-4068-b315-b7bacc91c2e1";
    assert_eq!((host.graph.nodes[node_id].x, host.graph.nodes[node_id].y), (0.0, 0.0));
    host.pointer_down(400.0, 300.0, false);
    host.pointer_move(460.0, 360.0);
    host.pointer_up(460.0, 360.0).await;
    let after = (host.graph.nodes[node_id].x, host.graph.nodes[node_id].y);
    assert!((after.0 - 60.0).abs() < 1e-6);
    assert!((after.1 - 60.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_commit_checkpoint_and_redo_and_store_generation() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    let gen0 = host.store_generation();
    host.run_jack("CREATE (n:Piece)").await.unwrap();
    assert!(host.store_generation() > gen0);
    let count_after_create = host.graph.nodes.len();
    host.commit_checkpoint(None).await.unwrap();
    host.undo().await.unwrap();
    assert_eq!(host.graph.nodes.len(), count_after_create - 1);
    host.redo().await.unwrap();
    assert_eq!(host.graph.nodes.len(), count_after_create);
}

#[semio_framework_async_macros::async_test]
async fn trinity_host_forced_and_automatic_draw_lod_label() {
    let mut host = TrinityBridge::from_graph(&nakagin_graph()).await;
    host.set_camera(0.0, 0.0, 0.05);
    assert_eq!(host.draw_lod_label(), "minimap");
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("micro");
    assert_eq!(host.draw_lod_label(), "micro");
    host.set_forced_draw_lod_label("");
    assert_eq!(host.draw_lod_label(), "minimap");
    host.set_forced_draw_lod_label("bogus");
    assert_eq!(host.draw_lod_label(), "minimap");
}
