use super::*;
use semio_framework_ui_declaration::ui::{build_table_scene, build_world_3d_scene, UiStackNode};
use semio_framework_ui_scene_views::TableScene;
use semio_framework_ui_scene_world3d::World3dScene;

#[test]
fn validate_ui_node_rejects_oversized_json_payload() {
    let limits = RenderPlanLimits { max_json_payload_bytes: 16, ..RenderPlanLimits::default() };
    let node = build_table_scene("table", "controller", TableScene::base("[]", "x".repeat(32)));
    let error = validate_ui_node(&node, &limits).expect_err("oversized payload should be rejected");
    assert!(error.contains("table.rows"));
    assert!(error.contains("32 bytes"));
}

fn empty_stack(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

#[test]
fn validate_component_scene_rejects_oversized_mesh_count() {
    let limits = RenderPlanLimits { max_mesh_count: 2, ..RenderPlanLimits::default() };
    let meshes_json = serde_json::to_string(&vec![serde_json::json!({"id": "m"}); 3]).unwrap();
    let node = build_world_3d_scene(
        "world",
        "controller",
        World3dScene {
            snapshot: None,
            camera_json: "{}".into(),
            meshes_json,
            instances_json: "[]".into(),
            instances_delta_json: None,
            selection_json: "{}".into(),
            vortices_json: None,
            attractions_json: None,
            target_volumes_json: None,
            references_json: None,
            brush_preview_json: None,
            interaction_json: None,
            engagement_preview_json: None,
            pick_targets_json: None,
            lod_json: None,
            presentation_json: None,
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            tool_run_trace: None,
            annotations: None,
            scalar_field: None,
            modelling_options: None,
            domain_id: None,
            domain_granularity_id: None,
            lanes: Vec::new(),
        },
    );
    let error = validate_ui_node(&node, &limits).expect_err("oversized mesh count should be rejected");
    assert!(error.contains("mesh count 3 exceeds max 2"));
}

#[test]
fn validate_component_scene_bounds_the_tool_run_trace_lane_on_every_scene_kind() {
    assert_eq!(RenderPlanLimits::default().max_tool_run_trace_bytes, infinite_world::world::tool_run_trace::TOOL_RUN_TRACE_LANE_BYTES_MAX);
    let limits = RenderPlanLimits { max_tool_run_trace_bytes: 8, ..RenderPlanLimits::default() };
    let mut world = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    world.tool_run_trace = Some("AAQBDQQB".into());
    assert!(validate_ui_node(&build_world_3d_scene("world", "controller", world.clone()), &limits).is_ok());
    world.tool_run_trace = Some("AAQBDQQBA".into());
    let error = validate_ui_node(&build_world_3d_scene("world", "controller", world), &limits).expect_err("oversized trace lane should be rejected");
    assert!(error.contains("world3d.toolRunTrace has 9 bytes (max 8)"), "{error}");
    let canvas = semio_framework_ui_scene_plane::Canvas2dScene { tool_run_trace: Some("A".repeat(9)), ..ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into()) };
    let node = semio_framework_ui_declaration::ui::build_canvas_2d_scene("canvas", "controller", canvas);
    let error = validate_ui_node(&node, &limits).expect_err("oversized canvas trace lane should be rejected");
    assert!(error.contains("canvas2d.toolRunTrace has 9 bytes (max 8)"), "{error}");
    let board = semio_framework_ui_scene_plane::Board2dScene { tool_run_trace: Some("A".repeat(9)), ..ui_wgpu::wgpu::Board2dScene::base("{}".into(), "{}".into(), true) };
    let error = validate_ui_node(&semio_framework_ui_declaration::ui::build_board2d_scene("board", "controller", board), &limits).expect_err("oversized board trace lane should be rejected");
    assert!(error.contains("board2d.toolRunTrace has 9 bytes (max 8)"), "{error}");
}

#[test]
fn validate_ui_node_rejects_oversized_node_count() {
    let limits = RenderPlanLimits { max_node_count: 3, ..RenderPlanLimits::default() };
    let tree = empty_stack(vec![empty_stack(vec![]), empty_stack(vec![]), empty_stack(vec![])]);
    let error = validate_ui_node(&tree, &limits).expect_err("oversized node count should be rejected");
    assert!(error.contains("node count 4 exceeds max 3"));
}

#[test]
fn validate_ui_node_rejects_oversized_tree_depth() {
    let limits = RenderPlanLimits { max_tree_depth: 2, ..RenderPlanLimits::default() };
    let mut tree = empty_stack(vec![]);
    for _ in 0..4 {
        tree = empty_stack(vec![tree]);
    }
    let error = validate_ui_node(&tree, &limits).expect_err("oversized tree depth should be rejected");
    assert!(error.contains("tree depth"));
    assert!(error.contains("exceeds max 2"));
}
