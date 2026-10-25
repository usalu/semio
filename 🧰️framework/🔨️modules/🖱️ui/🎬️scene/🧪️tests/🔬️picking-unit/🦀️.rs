//! 🧲️ Unit proofs of ray, marquee and component screen picking.

use super::*;
use semio_framework_ui_scene_algebra::{mat4_identity_m, mat4_translation_m};
use semio_framework_ui_scene_camera::testkit::{puzzle3d_perspective_camera, puzzle3d_top_camera};
use semio_framework_ui_scene_camera::{default_projection_spec, Camera3d, WORLD_ORBIT_CAMERA_MIN_FAR};
use semio_framework_ui_scene_mesh::testkit::{close_mesh, mesh_test_attempt, paged_mesh_fixture, test_box_mesh, LegacyMeshOracleData};
use semio_framework_ui_scene_mesh::{mesh3d_begin_close, mesh3d_terminal_is_empty};

#[test]
fn point_in_square() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    assert!(point_in_polygon([5.0, 5.0], &square));
    assert!(!point_in_polygon([20.0, 5.0], &square));
}

#[test]
fn ray_hits_triangle_direct() {
    let hit = ray_triangle(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(-1.0, -1.0, 0.0), vec3_new_m(1.0, -1.0, 0.0), vec3_new_m(0.0, 1.0, 0.0));
    assert!(hit.is_some());
}

#[test]
fn ray_hits_box() {
    let mesh = test_box_mesh();
    let instance = Instance3d { component_source: None, id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    let hit = mesh_test_attempt(||ray_pick_instance(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance)).unwrap();
    assert!(hit.is_some());
}

#[test]
fn ray_aabb_misses_offset_box() {
    let mesh = test_box_mesh();
    let instance = Instance3d { component_source: None, id: "box".into(), model: mat4_translation_m(vec3_new_m(100.0, 0.0, 0.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    let hit = mesh_test_attempt(||ray_pick_instance(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance)).unwrap();
    assert!(hit.is_none());
}

#[test]
fn rectangle_marquee_bounds_use_start_and_end_corners() {
    let bounds = marquee_rect_bounds(&[[10.0, 10.0], [200.0, 10.0], [200.0, 200.0], [10.0, 200.0]]).expect("bounds");
    assert!(rect_contains(bounds, [100.0, 100.0]));
    assert!(!rect_contains(bounds, [5.0, 100.0]));
}

#[test]
fn projected_aabb_skips_far_instance() {
    let mesh = test_box_mesh();
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "far".into(), model: mat4_translation_m(vec3_new_m(0.0, 0.0, -500.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let ids = screen_select_instances(&lookup, &draws, view_proj, default_projection_spec(), 200.0, 200.0, &[[0.0, 0.0], [200.0, 0.0], [200.0, 200.0], [0.0, 200.0]], true, true);
    assert!(ids.is_empty());
}

#[test]
fn marquee_is_crossing_follows_drag_direction() {
    assert!(marquee_is_crossing(100.0, 80.0));
    assert!(!marquee_is_crossing(80.0, 100.0));
}

#[test]
fn marquee_is_crossing_from_path_lasso_uses_first_horizontal_step() {
    let left_first = [[100.0, 100.0], [80.0, 100.0], [120.0, 100.0]];
    let right_first = [[100.0, 100.0], [120.0, 100.0], [80.0, 100.0]];
    assert!(marquee_is_crossing_from_path(&left_first, true));
    assert!(!marquee_is_crossing_from_path(&right_first, true));
}

#[test]
fn screen_select_instances_window_requires_full_vertex_enclosure() {
    let mesh = test_box_mesh();
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "partial".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let width = 800.0;
    let height = 600.0;
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for corner in [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]] {
        let screen = project_point(view_proj, vec3_from_array_m(corner), width, height).expect("screen");
        min_x = min_x.min(screen[0]);
        min_y = min_y.min(screen[1]);
        max_x = max_x.max(screen[0]);
        max_y = max_y.max(screen[1]);
    }
    let center_x = (min_x + max_x) * 0.5;
    let center_y = (min_y + max_y) * 0.5;
    let partial = [[min_x, min_y], [center_x, center_y]];
    let window_ids = screen_select_instances(&lookup, &draws, view_proj, default_projection_spec(), width, height, &partial, true, false);
    let crossing_ids = screen_select_instances(&lookup, &draws, view_proj, default_projection_spec(), width, height, &partial, true, true);
    assert!(window_ids.is_empty());
    assert_eq!(crossing_ids, vec!["partial".to_string()]);
}

/// 📐️ LAW: the parallel frustum is drei's PIXEL frustum — `left = -width / 2` … divided by `zoom`
/// (`worldProjectionGoalMatrix`, `🎨️r3f/🟦️.tsx`). A point exactly `width / (2 · zoom)` to the right
/// of the target lands on the right edge of the viewport, and nothing about that depends on where
/// along the view axis the eye stands, which is what makes a parallel camera parallel.
#[test]
fn the_parallel_frustum_is_dreis_pixel_frustum_and_ignores_the_eyes_distance() {
    let (width, height) = (478.0_f32, 814.0_f32);
    let camera = puzzle3d_top_camera(1.0);
    let (half_width, half_height) = camera.orthographic_half_extent(width, height);
    assert!((half_width - width * 0.5).abs() < 1e-3, "zoom 1 shows exactly `width` world units across");
    assert!((half_height - height * 0.5).abs() < 1e-3);

    let view_proj = camera.view_proj(width, height);
    let centre = project_point(view_proj, camera.target, width, height).expect("the target projects");
    assert!((centre[0] - width * 0.5).abs() < 1e-2 && (centre[1] - height * 0.5).abs() < 1e-2, "the target sits in the middle of the pane");
    let edge = project_point(view_proj, vec3_new_m(camera.target.x + half_width, camera.target.y, camera.target.z), width, height).expect("the rim projects");
    assert!((edge[0] - width).abs() < 1e-2, "and the half-extent lands on the rim: {edge:?}");

    let pulled_back = Camera3d { position: vec3_new_m(3.5, 0.0, 900.0), ..camera.clone() };
    let pulled_edge = project_point(pulled_back.view_proj(width, height), vec3_new_m(camera.target.x + half_width, camera.target.y, camera.target.z), width, height).expect("the rim projects");
    assert!((pulled_edge[0] - edge[0]).abs() < 1e-2, "a parallel camera's scale does not change when the eye moves along its own axis");

    let zoomed = puzzle3d_top_camera(4.0);
    let (zoomed_half, _) = zoomed.orthographic_half_extent(width, height);
    assert!((zoomed_half - half_width / 4.0).abs() < 1e-3, "and four times the zoom shows a quarter as much");
}

/// 📡️ LAW: a screen pixel round-trips through the parallel camera — the pick ray it builds projects
/// back to the pixel it came from, and its direction is the view axis (three's `Raycaster` parallel
/// branch), so `pick_*` keeps working under a plan view.
#[test]
fn a_parallel_pick_ray_round_trips_through_the_pixel_it_came_from() {
    let (width, height) = (478.0_f32, 814.0_f32);
    let camera = puzzle3d_top_camera(12.0);
    let view_proj = camera.view_proj(width, height);
    let forward = camera.target.sub_m(camera.position).normalize_m();
    for pixel in [[120.0_f32, 200.0_f32], [239.0, 407.0], [400.0, 700.0]] {
        let (origin, dir) = camera.ray_from_screen(pixel[0], pixel[1], width, height);
        assert!(dir.sub_m(forward).length_m() < 1e-4, "a parallel ray runs down the view axis");
        let ahead = origin.add_m(dir.scale_m(25.0));
        let back = project_point(view_proj, ahead, width, height).expect("the ray point projects");
        assert!((back[0] - pixel[0]).abs() < 1e-2 && (back[1] - pixel[1]).abs() < 1e-2, "pixel {pixel:?} round-tripped to {back:?}");
    }
}

/// 🔦️ LAW: the perspective pick ray survives React's adaptive far plane. It is three's
/// `origin = camera.position, direction = unproject(ndc, 0.5) - origin`; the near-minus-far pair it
/// replaced was numerically empty in `f32` at a `0.2 / 524 288` ratio and missed every instance
/// (`📓️w7b-presenter-one-frame-per-boot.md` §5.1).
#[test]
fn a_perspective_pick_ray_survives_the_adaptive_far_plane() {
    let (width, height) = (956.0_f32, 814.0_f32);
    let camera = puzzle3d_perspective_camera();
    assert_eq!(camera.far, WORLD_ORBIT_CAMERA_MIN_FAR);
    let view_proj = camera.view_proj(width, height);
    for pixel in [[478.0_f32, 407.0_f32], [100.0, 120.0], [900.0, 780.0]] {
        let (origin, dir) = camera.ray_from_screen(pixel[0], pixel[1], width, height);
        assert!(origin.sub_m(camera.position).length_m() < 1e-4, "a perspective ray starts at the eye");
        assert!((dir.length_m() - 1.0).abs() < 1e-4, "and carries a unit direction, not a degenerate one");
        let back = project_point(view_proj, origin.add_m(dir.scale_m(9.0)), width, height).expect("the ray point projects");
        assert!((back[0] - pixel[0]).abs() < 1e-2 && (back[1] - pixel[1]).abs() < 1e-2, "pixel {pixel:?} round-tripped to {back:?}");
    }
}

#[test]
fn ray_pick_mesh_detail_returns_triangle_index_and_barycentrics() {
    let mesh = test_box_mesh();
    let instance = Instance3d { component_source: None, id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    let hit = mesh_test_attempt(||ray_pick_mesh_detail(vec3_new_m(0.0, -0.5, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance)).unwrap().expect("hit");
    assert_eq!(hit.triangle_index, 0);
    assert!(hit.bary_u >= 0.0 && hit.bary_v >= 0.0 && hit.bary_u + hit.bary_v <= 1.0);
}

#[test]
fn ray_pick_mesh_detail_misses_when_aabb_not_hit() {
    let mesh = test_box_mesh();
    let instance = Instance3d { component_source: None, id: "box".into(), model: mat4_translation_m(vec3_new_m(50.0, 0.0, 0.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    assert!(mesh_test_attempt(||ray_pick_mesh_detail(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance)).unwrap().is_none());
}

#[test]
fn interpolate_mesh_uv_none_when_uvs_missing() {
    let mesh = test_box_mesh();
    assert!(mesh_test_attempt(||interpolate_mesh_uv(mesh, 0, 0.25, 0.25)).unwrap().is_none());
}

#[test]
fn interpolate_mesh_uv_blends_triangle_corners() {
    let mut data = LegacyMeshOracleData::triangle();
    data.uvs = vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
    let mesh = paged_mesh_fixture(data);
    let (u, v) = mesh_test_attempt(||interpolate_mesh_uv(mesh, 0, 0.0, 0.0)).unwrap().expect("uv");
    assert!((u - 0.0).abs() < 1e-6 && (v - 0.0).abs() < 1e-6);
    let (u, v) = mesh_test_attempt(||interpolate_mesh_uv(mesh, 0, 1.0, 0.0)).unwrap().expect("uv");
    assert!((u - 1.0).abs() < 1e-6 && (v - 0.0).abs() < 1e-6);
}

#[test]
fn interpolate_mesh_uv_none_when_triangle_out_of_range() {
    let mut data = LegacyMeshOracleData::triangle();
    data.uvs = vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
    let mesh = paged_mesh_fixture(data);
    assert!(mesh_test_attempt(||interpolate_mesh_uv(mesh, 5, 0.0, 0.0)).unwrap().is_none());
}

#[test]
fn marquee_is_crossing_from_path_window_mode_uses_endpoints() {
    let path = [[100.0, 100.0], [50.0, 100.0]];
    assert!(marquee_is_crossing_from_path(&path, false));
    let path = [[50.0, 100.0], [100.0, 100.0]];
    assert!(!marquee_is_crossing_from_path(&path, false));
}

#[test]
fn marquee_is_crossing_from_path_empty_defaults_to_false() {
    let path: [[f32; 2]; 0] = [];
    assert!(!marquee_is_crossing_from_path(&path, true));
}

#[test]
fn segments_intersect_detects_proper_crossing() {
    assert!(segments_intersect([0.0, 0.0], [10.0, 10.0], [0.0, 10.0], [10.0, 0.0]));
    assert!(!segments_intersect([0.0, 0.0], [10.0, 0.0], [0.0, 5.0], [10.0, 5.0]));
}

#[test]
fn segments_intersect_detects_collinear_touch() {
    assert!(segments_intersect([0.0, 0.0], [10.0, 0.0], [5.0, 0.0], [20.0, 0.0]));
}

#[test]
fn point_on_segment_checks_bounding_box() {
    assert!(point_on_segment([5.0, 0.0], [0.0, 0.0], [10.0, 0.0]));
    assert!(!point_on_segment([20.0, 0.0], [0.0, 0.0], [10.0, 0.0]));
}

#[test]
fn segment_intersects_rect_detects_boundary_crossing() {
    let rect = [0.0, 0.0, 10.0, 10.0];
    assert!(segment_intersects_rect([-5.0, 5.0], [5.0, 5.0], rect));
    assert!(!segment_intersects_rect([-5.0, 20.0], [-1.0, 20.0], rect));
}

#[test]
fn segment_intersects_polygon_true_when_endpoint_inside() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    assert!(segment_intersects_polygon([5.0, 5.0], [50.0, 50.0], &square));
    assert!(!segment_intersects_polygon([50.0, 50.0], [60.0, 60.0], &square));
}

#[test]
fn marquee_contains_point_rectangle_vs_polygon_modes() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let bounds = marquee_rect_bounds(&square);
    assert!(marquee_contains_point([5.0, 5.0], &square, true, bounds));
    assert!(!marquee_contains_point([5.0, 5.0], &square, true, None));
    assert!(marquee_contains_point([5.0, 5.0], &square, false, bounds));
}

#[test]
fn marquee_segment_selected_window_mode_requires_both_endpoints_inside() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let bounds = marquee_rect_bounds(&square);
    assert!(marquee_segment_selected([2.0, 2.0], [8.0, 8.0], &square, true, bounds, false));
    assert!(!marquee_segment_selected([2.0, 2.0], [20.0, 20.0], &square, true, bounds, false));
}

#[test]
fn marquee_segment_selected_crossing_mode_detects_rect_edge_crossing() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let bounds = marquee_rect_bounds(&square);
    assert!(marquee_segment_selected([-5.0, 5.0], [15.0, 5.0], &square, true, bounds, true));
    assert!(!marquee_segment_selected([-5.0, 20.0], [-1.0, 20.0], &square, true, bounds, true));
}

#[test]
fn marquee_triangle_selected_window_mode_requires_all_points_inside() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let bounds = marquee_rect_bounds(&square);
    let inside = [[1.0, 1.0], [2.0, 2.0], [3.0, 1.0]];
    let partial = [[1.0, 1.0], [2.0, 2.0], [30.0, 30.0]];
    assert!(marquee_triangle_selected(&inside, &square, true, bounds, false));
    assert!(!marquee_triangle_selected(&partial, &square, true, bounds, false));
}

#[test]
fn marquee_triangle_selected_crossing_mode_true_on_partial_overlap() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let bounds = marquee_rect_bounds(&square);
    let straddling = [[5.0, 5.0], [20.0, 20.0], [20.0, 5.0]];
    assert!(marquee_triangle_selected(&straddling, &square, true, bounds, true));
}

#[test]
fn aabb_overlaps_marquee_polygon_mode_detects_corner_containment() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    assert!(aabb_overlaps_marquee([2.0, 2.0, 8.0, 8.0], &square, false));
    assert!(!aabb_overlaps_marquee([100.0, 100.0, 110.0, 110.0], &square, false));
}

#[test]
fn aabb_overlaps_marquee_rectangle_mode_returns_false_without_bounds() {
    let empty: [[f32; 2]; 0] = [];
    assert!(!aabb_overlaps_marquee([0.0, 0.0, 5.0, 5.0], &empty, true));
}

#[test]
fn screen_select_components_face_granularity_selects_visible_triangle() {
    let mut data = LegacyMeshOracleData::triangle();
    data.face_ids = vec![42];
    let mesh = paged_mesh_fixture(data);
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, default_projection_spec(), 800.0, 600.0, &full_screen, true, "face", None, false);
    assert_eq!(selected, vec!["42".to_string()]);
}

#[test]
fn screen_select_components_vertex_granularity_selects_ids() {
    let mut data = LegacyMeshOracleData::triangle();
    data.vertex_ids = vec![10, 11, 12];
    let mesh = paged_mesh_fixture(data);
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let mut selected = screen_select_components(&lookup, &draws, view_proj, default_projection_spec(), 800.0, 600.0, &full_screen, true, "vertex", None, false);
    selected.sort();
    assert_eq!(selected, vec!["10".to_string(), "11".to_string(), "12".to_string()]);
}

#[test]
fn screen_select_components_edge_granularity_selects_ids() {
    let mut data = LegacyMeshOracleData::triangle();
    data.edges = vec![[[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0]]];
    data.edge_ids = vec![99];
    let mesh = paged_mesh_fixture(data);
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, default_projection_spec(), 800.0, 600.0, &full_screen, true, "edge", None, false);
    assert_eq!(selected, vec!["99".to_string()]);
}

#[test]
fn screen_select_components_default_granularity_selects_whole_instance() {
    let mesh = test_box_mesh();
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "whole".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, default_projection_spec(), 800.0, 600.0, &full_screen, true, "unknown", None, false);
    assert_eq!(selected, vec!["whole".to_string()]);
}

#[test]
fn screen_select_components_filters_by_active_instance_id() {
    let mesh = test_box_mesh();
    let draws = vec![SceneDraw3d {
        mesh_key: "box".into(),
        mesh_version: 0,
        instances: vec![
            Instance3d { component_source: None, id: "keep".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() },
            Instance3d { component_source: None, id: "skip".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() },
        ],
        shadow_role: Default::default(),
    }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, default_projection_spec(), 800.0, 600.0, &full_screen, true, "unknown", Some("keep"), false);
    assert_eq!(selected, vec!["keep".to_string()]);
}

#[test]
fn screen_select_components_skips_missing_mesh_lookup() {
    let draws = vec![SceneDraw3d { mesh_key: "missing".into(), mesh_version: 0, instances: vec![], shadow_role: Default::default() }];
    let lookup = std::collections::HashMap::new();
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, default_projection_spec(), 800.0, 600.0, &full_screen, true, "face", None, false);
    assert!(selected.is_empty());
}

#[test]
fn screen_segment_distance_projects_point_onto_segment() {
    let dist = screen_segment_distance(5.0, 5.0, 0.0, 0.0, 10.0, 0.0);
    assert!((dist - 5.0).abs() < 1e-4, "dist={dist}");
    let dist_beyond_end = screen_segment_distance(20.0, 0.0, 0.0, 0.0, 10.0, 0.0);
    assert!((dist_beyond_end - 10.0).abs() < 1e-4, "dist={dist_beyond_end}");
}

#[test]
fn screen_segment_distance_degenerate_segment_falls_back_to_point_distance() {
    let dist = screen_segment_distance(3.0, 4.0, 0.0, 0.0, 0.0, 0.0);
    assert!((dist - 5.0).abs() < 1e-4, "dist={dist}");
}

#[test]
fn project_point_rejects_points_outside_near_far_clip() {
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let far_behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(2.0));
    assert!(project_point(view_proj, far_behind, 800.0, 600.0).is_none());
    assert!(project_point(view_proj, camera.target, 800.0, 600.0).is_some());
}

#[test]
fn ray_aabb_slab_axis_parallel_ray_outside_bounds_misses() {
    let origin = vec3_new_m(5.0, 0.0, 0.0);
    let dir = vec3_new_m(0.0, 0.0, 1.0);
    assert!(ray_aabb_slab(origin, dir, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]).is_none());
}

#[test]
fn ray_aabb_slab_returns_none_when_box_entirely_behind_origin() {
    let origin = vec3_new_m(0.0, 0.0, -10.0);
    let dir = vec3_new_m(0.0, 0.0, -1.0);
    assert!(ray_aabb_slab(origin, dir, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]).is_none());
}

/// 📍️ Rectangle and lasso selection skip sampled vertices in the original paged mesh.
#[test]
fn screen_select_original_vertices_skips_surface_samples() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️vertex-selection/🔣️.json")).unwrap();
    let mut data = LegacyMeshOracleData::triangle();
    data.positions = serde_json::from_value(fixture["positions"].clone()).unwrap();
    data.normals = vec![[0.0,0.0,1.0]; data.positions.len()];
    data.indices = serde_json::from_value(fixture["indices"].clone()).unwrap();
    data.vertex_ids = serde_json::from_value(fixture["vertexIds"].clone()).unwrap();
    let mesh = paged_mesh_fixture(data);
    let draws = vec![SceneDraw3d { mesh_key: "original".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "original".into(), model: mat4_identity_m(), color: [1.0;4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let lookup = std::collections::HashMap::from([("original".into(), mesh)]);
    let camera = Camera3d::default();
    let polygon: Vec<[f32;2]> = serde_json::from_value(fixture["polygon"].clone()).unwrap();
    let expected: Vec<String> = serde_json::from_value(fixture["expected"].clone()).unwrap();
    for method in fixture["methods"].as_array().unwrap() {
        let selected = screen_select_components(&lookup, &draws, camera.view_proj(800.0,600.0), default_projection_spec(), 800.0,600.0,&polygon, method.as_str() == Some("rectangle"),"vertex",None,false);
        assert_eq!(selected,expected,"method {method}");
    }
    mesh_test_attempt(||mesh3d_begin_close(mesh)).unwrap();
    while !close_mesh(mesh).unwrap() {}
    assert!(mesh3d_terminal_is_empty(mesh));
}
