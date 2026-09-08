
use super::*;

struct LegacyMeshOracleData {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
    face_ids: Vec<u32>,
    vertex_ids: Vec<u32>,
    edges: Vec<[[f32; 3]; 2]>,
    edge_ids: Vec<u32>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
}

impl LegacyMeshOracleData {
    fn triangle() -> Self {
        Self {
            positions: vec![[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: vec![0, 1, 2],
            face_ids: Vec::new(),
            vertex_ids: Vec::new(),
            edges: Vec::new(),
            edge_ids: Vec::new(),
            uvs: Vec::new(),
            colors: Vec::new(),
        }
    }
}

fn paged_mesh_fixture(data: LegacyMeshOracleData) -> Mesh3dLease {
    let schema = Mesh3dSchema {
        vertices: data.positions.len() as u32,
        indices: data.indices.len() as u32,
        face_ids: data.face_ids.len() as u32,
        vertex_ids: data.vertex_ids.len() as u32,
        edges: data.edges.len() as u32,
        edge_ids: data.edge_ids.len() as u32,
        uvs: data.uvs.len() as u32,
        colors: data.colors.len() as u32,
    };
    let token = mesh3d_begin(1, 1, schema).expect("test mesh claim");
    while !mesh3d_allocate_step(token).expect("test mesh page allocation") {}
    for value in data.positions {
        mesh3d_write_vec3(token, Mesh3dField::Positions, value).unwrap();
    }
    for value in data.normals {
        mesh3d_write_vec3(token, Mesh3dField::Normals, value).unwrap();
    }
    for value in data.indices {
        mesh3d_write_u32(token, Mesh3dField::Indices, value).unwrap();
    }
    for value in data.face_ids {
        mesh3d_write_u32(token, Mesh3dField::FaceIds, value).unwrap();
    }
    for value in data.vertex_ids {
        mesh3d_write_u32(token, Mesh3dField::VertexIds, value).unwrap();
    }
    for value in data.edges {
        mesh3d_write_edge(token, value).unwrap();
    }
    for value in data.edge_ids {
        mesh3d_write_u32(token, Mesh3dField::EdgeIds, value).unwrap();
    }
    for value in data.uvs {
        mesh3d_write_vec2(token, Mesh3dField::Uvs, value).unwrap();
    }
    for value in data.colors {
        mesh3d_write_vec4(token, Mesh3dField::Colors, value).unwrap();
    }
    mesh3d_seal(token).expect("test mesh publication")
}

#[test]
fn paged_mesh_authority_preserves_order_aba_and_interrupted_close() {
    let mut authority = Mesh3dAuthority::new();
    let schema = Mesh3dSchema::triangle_mesh(3, 3);
    let token = authority.begin(7, 11, schema).expect("fixed mesh claim");
    assert!(authority.writing(token).unwrap().allocate_step());
    for position in [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        let mut bytes = [0; 12];
        for (index, value) in position.into_iter().enumerate() {
            bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        authority.writing(token).unwrap().write(Mesh3dField::Positions, &bytes).unwrap();
    }
    for _ in 0..3 {
        let mut bytes = [0; 12];
        bytes[8..].copy_from_slice(&1.0f32.to_le_bytes());
        authority.writing(token).unwrap().write(Mesh3dField::Normals, &bytes).unwrap();
    }
    for index in [0u32, 1, 2] {
        authority.writing(token).unwrap().write(Mesh3dField::Indices, &index.to_le_bytes()).unwrap();
    }
    let lease = authority.seal(token).expect("terminal mesh publication");
    let first = authority.ready(lease).unwrap().item_bytes::<12>(Mesh3dField::Positions, 0).unwrap();
    assert_eq!(f32::from_le_bytes(first[..4].try_into().unwrap()), 0.0);
    authority.begin_close(lease).unwrap();
    assert!(!authority.close_step(lease.slot, lease.epoch).unwrap());
    assert!(authority.close_step(lease.slot, lease.epoch).unwrap());
    let replacement = authority.begin(8, 12, schema).expect("reused fixed slot");
    assert_eq!(replacement.slot, token.slot);
    assert_ne!(replacement.epoch, token.epoch);
    assert!(matches!(authority.ready(lease), Err(Mesh3dFault::Stale)));
    authority.begin_close_write(replacement).unwrap();
    assert!(authority.close_step(replacement.slot, replacement.epoch).unwrap());
}

#[test]
fn paged_mesh_authority_rejects_aggregate_page_plus_one_before_allocation() {
    let mut authority = Mesh3dAuthority::new();
    let vertices = ((MESH3D_OWNER_BYTE_CAPACITY / 28) / 3 * 3) as u32;
    let indices = vertices;
    let schema = Mesh3dSchema::triangle_mesh(vertices, indices);
    let mut tokens = Vec::new();
    for generation in 1..=MESH3D_AUTHORITY_PAGE_CAPACITY / MESH3D_OWNER_PAGE_CAPACITY {
        tokens.push(authority.begin(generation as u64, 1, schema).expect("aggregate admitted mesh"));
    }
    assert_eq!(authority.begin(99, 1, Mesh3dSchema::triangle_mesh(3, 3)).unwrap_err(), Mesh3dFault::PageCapacity);
    for token in tokens {
        authority.begin_close_write(token).unwrap();
        assert!(authority.close_step(token.slot, token.epoch).unwrap());
    }
    assert_eq!(authority.reserved_pages, 0);
}

fn test_box_mesh() -> Mesh3dLease {
    paged_mesh_fixture(LegacyMeshOracleData::triangle())
}

#[test]
fn orbit_round_trip() {
    let camera = Camera3d::default();
    let orbit = OrbitController::from_camera(&camera);
    let next = orbit.to_camera();
    assert!((next.position.x - camera.position.x).abs() < 0.5);
}

#[test]
fn point_in_square() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    assert!(point_in_polygon([5.0, 5.0], &square));
    assert!(!point_in_polygon([20.0, 5.0], &square));
}

#[test]
fn mat4_inverse_round_trips_to_identity() {
    let m = mat4_translation_m(vec3_new_m(3.0, -2.0, 5.0)).mul_m(mat4_from_quat_m(0.1, 0.2, 0.05, 0.9701425)).mul_m(mat4_scale_vec_m(vec3_new_m(2.0, 1.5, 0.5)));
    let round_trip = m.mul_m(m.inverse_m());
    let identity = mat4_identity_m();
    for col in 0..4 {
        for row in 0..4 {
            assert!((round_trip.cols[col][row] - identity.cols[col][row]).abs() < 1e-4, "mismatch at col={col} row={row}: {} vs {}", round_trip.cols[col][row], identity.cols[col][row]);
        }
    }
}

#[test]
fn mat4_inverse_undoes_view_projection() {
    let camera = Camera3d::default();
    let view = mat4_look_at_m(camera.position, camera.target, camera.up);
    let proj = mat4_perspective_m(camera.fov_y, 1.5, camera.near, camera.far);
    let view_proj = proj.mul_m(view);
    let inv = view_proj.inverse_m();
    let world_point = vec3_new_m(1.0, 2.0, 0.5);
    let clip_point = view_proj.transform_point_m(world_point);
    let unprojected = inv.transform_point_m(clip_point);
    assert!((unprojected.x - world_point.x).abs() < 1e-3, "x mismatch: {}", unprojected.x);
    assert!((unprojected.y - world_point.y).abs() < 1e-3, "y mismatch: {}", unprojected.y);
    assert!((unprojected.z - world_point.z).abs() < 1e-3, "z mismatch: {}", unprojected.z);
}

#[test]
fn ray_from_screen_center_points_at_target() {
    let camera = Camera3d::default();
    let aspect = 1.6;
    let (origin, dir) = camera.ray_from_screen(aspect, 400.0, 300.0, 800.0, 600.0);
    assert!((origin.x - camera.position.x).abs() < 1e-4);
    let to_target = camera.target.sub_m(camera.position).normalize_m();
    let dot = dir.dot_m(to_target);
    assert!(dot > 0.999, "ray from screen center should point at target, dot={dot}");
}

#[test]
fn ray_hits_triangle_direct() {
    let hit = ray_triangle(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(-1.0, -1.0, 0.0), vec3_new_m(1.0, -1.0, 0.0), vec3_new_m(0.0, 1.0, 0.0));
    assert!(hit.is_some());
}

#[test]
fn ray_hits_box() {
    let mesh = test_box_mesh();
    let instance = Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false };
    let hit = ray_pick_instance(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance);
    assert!(hit.is_some());
}

#[test]
fn ray_aabb_misses_offset_box() {
    let mesh = test_box_mesh();
    let instance = Instance3d { id: "box".into(), model: mat4_translation_m(vec3_new_m(100.0, 0.0, 0.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false };
    let hit = ray_pick_instance(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance);
    assert!(hit.is_none());
}

#[test]
fn frustum_contains_origin_box() {
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let planes = frustum_planes(view_proj);
    assert!(aabb_intersects_frustum(&planes, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]));
}

#[test]
fn frustum_culls_behind_camera_box() {
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let planes = frustum_planes(view_proj);
    assert!(aabb_intersects_frustum(&planes, [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]));
    let behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(2.0));
    let min = [behind.x - 0.1, behind.y - 0.1, behind.z - 0.1];
    let max = [behind.x + 0.1, behind.y + 0.1, behind.z + 0.1];
    assert!(!aabb_intersects_frustum(&planes, min, max));
}

fn concrete_forest_camera() -> Camera3d {
    Camera3d { position: vec3_new_m(30.0, -30.0, 20.0), target: vec3_new_m(7.0, 0.0, 3.0), up: vec3_new_m(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0 }
}

#[test]
fn concrete_forest_frustum_contains_target_box() {
    let camera = concrete_forest_camera();
    let view_proj = camera.view_proj(1.0);
    let planes = frustum_planes(view_proj);
    let target = camera.target;
    for plane in &planes {
        let distance = plane.normal.dot_m(target) + plane.distance;
        assert!(distance >= -1e-3, "look-at must be inside frustum, distance={distance}");
    }
    assert!(aabb_intersects_frustum(&planes, [6.0, -1.0, 2.0], [8.0, 1.0, 4.0]));
}

#[test]
fn concrete_forest_frustum_culls_off_axis_boxes() {
    let camera = concrete_forest_camera();
    let view_proj = camera.view_proj(1.0);
    let planes = frustum_planes(view_proj);
    assert!(!aabb_intersects_frustum(&planes, [7.0, 0.0, 200.0], [8.0, 1.0, 201.0]));
    let behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(4.0));
    let min = [behind.x - 0.25, behind.y - 0.25, behind.z - 0.25];
    let max = [behind.x + 0.25, behind.y + 0.25, behind.z + 0.25];
    assert!(!aabb_intersects_frustum(&planes, min, max));
}

#[test]
fn perspective_maps_depth_to_wgpu_ndc() {
    let near = 0.1_f32;
    let far = 100.0_f32;
    let proj = mat4_perspective_m(45.0_f32.to_radians(), 1.0, near, far);
    let near_pt = proj.transform_point_m(vec3_new_m(0.0, 0.0, -near));
    let far_pt = proj.transform_point_m(vec3_new_m(0.0, 0.0, -far));
    assert!((near_pt.z - 0.0).abs() < 1e-4, "near z={}", near_pt.z);
    assert!((far_pt.z - 1.0).abs() < 1e-3, "far z={}", far_pt.z);
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
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "far".into(), model: mat4_translation_m(vec3_new_m(0.0, 0.0, -500.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let ids = screen_select_instances(&lookup, &draws, view_proj, 200.0, 200.0, &[[0.0, 0.0], [200.0, 0.0], [200.0, 200.0], [0.0, 200.0]], true, true);
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
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "partial".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
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
    let window_ids = screen_select_instances(&lookup, &draws, view_proj, width, height, &partial, true, false);
    let crossing_ids = screen_select_instances(&lookup, &draws, view_proj, width, height, &partial, true, true);
    assert!(window_ids.is_empty());
    assert_eq!(crossing_ids, vec!["partial".to_string()]);
}

#[test]
fn lod_from_camera_distance_scales() {
    assert!((lod_from_camera_distance(100.0, 100.0) - 1.0).abs() < 1e-6);
    assert!((lod_from_camera_distance(20000.0, 100.0) - 200.0).abs() < 1e-6);
}

#[test]
fn lod_progressive_grid_layers_adds_bands() {
    assert!(lod_progressive_grid_layers(5000.0, 10.0).is_empty());
    assert_eq!(lod_progressive_grid_layers(500.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0]);
    assert_eq!(lod_progressive_grid_layers(50.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0, 25.0]);
    assert_eq!(lod_progressive_grid_layers(2.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0, 25.0, 5.0, 1.0]);
}

#[test]
fn lod_progressive_grid_layer_key_stable_within_band() {
    let key_a = lod_progressive_grid_layer_key(50.0, 10.0);
    let key_b = lod_progressive_grid_layer_key(49.2, 10.0);
    let key_c = lod_progressive_grid_layer_key(11.4, 10.0);
    assert_eq!(key_a, key_b);
    assert_eq!(key_a, key_c);
    assert_eq!(lod_progressive_grid_layer_key(9.8, 10.0), "100|25|5");
}

#[test]
fn pick_closest_lod_prefers_more_detailed_on_tie() {
    let picked = pick_closest_lod(&[1.0, 2.0, 4.0], 2.0).unwrap();
    assert!((picked - 2.0).abs() < 1e-6);
}

#[test]
fn floating_origin_rebase_subtracts_anchor() {
    let rebased = floating_origin_rebase(vec3_new_m(10.0, 20.0, 30.0), vec3_new_m(1.0, 2.0, 3.0));
    assert_eq!(rebased, vec3_new_m(9.0, 18.0, 27.0));
}

#[test]
fn mesh_schema_has_vertex_colors_only_for_a_complete_vertex_field() {
    let without = test_box_mesh().schema().unwrap();
    assert_eq!(without.colors, 0);
    let mut data = LegacyMeshOracleData::triangle();
    data.colors = vec![[1.0; 4]; data.positions.len()];
    let with = paged_mesh_fixture(data).schema().unwrap();
    assert_eq!(with.colors, with.vertices);
}

#[test]
fn instance_model_from_trs_translates_and_scales_point() {
    let model = Instance3d::model_from_trs([1.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0], [2.0, 2.0, 2.0]);
    let point = model.transform_point_m(vec3_new_m(1.0, 0.0, 0.0));
    assert!((point.x - 3.0).abs() < 1e-5, "x={}", point.x);
    assert!((point.y - 2.0).abs() < 1e-5, "y={}", point.y);
    assert!((point.z - 3.0).abs() < 1e-5, "z={}", point.z);
}

#[test]
fn orbit_controller_orbit_clamps_pitch() {
    let mut orbit = OrbitController { pitch: 1.49, ..Default::default() };
    orbit.orbit(0.0, 1000.0);
    assert!((orbit.pitch - 1.5).abs() < 1e-5, "pitch={}", orbit.pitch);
    orbit.pitch = -1.49;
    orbit.orbit(0.0, -1000.0);
    assert!((orbit.pitch + 1.5).abs() < 1e-5, "pitch={}", orbit.pitch);
}

#[test]
fn orbit_controller_pan_moves_target_away_from_origin() {
    let mut orbit = OrbitController::default();
    let start = orbit.target;
    orbit.pan(50.0, 0.0);
    assert!(orbit.target.sub_m(start).length_m() > 0.0);
}

#[test]
fn orbit_controller_zoom_clamps_distance_bounds() {
    let mut orbit = OrbitController { distance: 1.0, ..Default::default() };
    orbit.zoom(100_000.0);
    assert!((orbit.distance - 0.5).abs() < 1e-4, "distance={}", orbit.distance);
    orbit.distance = 400.0;
    orbit.zoom(-100_000.0);
    assert!((orbit.distance - 500.0).abs() < 1e-4, "distance={}", orbit.distance);
}

#[test]
fn ray_pick_mesh_detail_returns_triangle_index_and_barycentrics() {
    let mesh = test_box_mesh();
    let instance = Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false };
    let hit = ray_pick_mesh_detail(vec3_new_m(0.0, -0.5, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance).expect("hit");
    assert_eq!(hit.triangle_index, 0);
    assert!(hit.bary_u >= 0.0 && hit.bary_v >= 0.0 && hit.bary_u + hit.bary_v <= 1.0);
}

#[test]
fn ray_pick_mesh_detail_misses_when_aabb_not_hit() {
    let mesh = test_box_mesh();
    let instance = Instance3d { id: "box".into(), model: mat4_translation_m(vec3_new_m(50.0, 0.0, 0.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false };
    assert!(ray_pick_mesh_detail(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance).is_none());
}

#[test]
fn interpolate_mesh_uv_none_when_uvs_missing() {
    let mesh = test_box_mesh();
    assert!(interpolate_mesh_uv(mesh, 0, 0.25, 0.25).is_none());
}

#[test]
fn interpolate_mesh_uv_blends_triangle_corners() {
    let mut data = LegacyMeshOracleData::triangle();
    data.uvs = vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
    let mesh = paged_mesh_fixture(data);
    let (u, v) = interpolate_mesh_uv(mesh, 0, 0.0, 0.0).expect("uv");
    assert!((u - 0.0).abs() < 1e-6 && (v - 0.0).abs() < 1e-6);
    let (u, v) = interpolate_mesh_uv(mesh, 0, 1.0, 0.0).expect("uv");
    assert!((u - 1.0).abs() < 1e-6 && (v - 0.0).abs() < 1e-6);
}

#[test]
fn interpolate_mesh_uv_none_when_triangle_out_of_range() {
    let mut data = LegacyMeshOracleData::triangle();
    data.uvs = vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
    let mesh = paged_mesh_fixture(data);
    assert!(interpolate_mesh_uv(mesh, 5, 0.0, 0.0).is_none());
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
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "face", None, false);
    assert_eq!(selected, vec!["42".to_string()]);
}

#[test]
fn screen_select_components_vertex_granularity_selects_ids() {
    let mut data = LegacyMeshOracleData::triangle();
    data.vertex_ids = vec![10, 11, 12];
    let mesh = paged_mesh_fixture(data);
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let mut selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "vertex", None, false);
    selected.sort();
    assert_eq!(selected, vec!["10".to_string(), "11".to_string(), "12".to_string()]);
}

#[test]
fn screen_select_components_edge_granularity_selects_ids() {
    let mut data = LegacyMeshOracleData::triangle();
    data.edges = vec![[[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0]]];
    data.edge_ids = vec![99];
    let mesh = paged_mesh_fixture(data);
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "edge", None, false);
    assert_eq!(selected, vec!["99".to_string()]);
}

#[test]
fn screen_select_components_default_granularity_selects_whole_instance() {
    let mesh = test_box_mesh();
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "whole".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "unknown", None, false);
    assert_eq!(selected, vec!["whole".to_string()]);
}

#[test]
fn screen_select_components_filters_by_active_instance_id() {
    let mesh = test_box_mesh();
    let draws = vec![SceneDraw3d {
        mesh_key: "box".into(),
        mesh_version: 0,
        instances: vec![
            Instance3d { id: "keep".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false },
            Instance3d { id: "skip".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false },
        ],
    }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "unknown", Some("keep"), false);
    assert_eq!(selected, vec!["keep".to_string()]);
}

#[test]
fn screen_select_components_skips_missing_mesh_lookup() {
    let draws = vec![SceneDraw3d { mesh_key: "missing".into(), mesh_version: 0, instances: vec![] }];
    let lookup = std::collections::HashMap::new();
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(1.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "face", None, false);
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
    let view_proj = camera.view_proj(1.0);
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

#[test]
fn vec3_from_f64_converts_components() {
    let v = vec3_from_f64([1.5, -2.5, 3.5]);
    assert_eq!(v, vec3_new_m(1.5, -2.5, 3.5));
}

#[test]
fn gumball_extent_clamps_to_bounds() {
    assert!((gumball_extent(0.0) - 0.25).abs() < 1e-6);
    assert!((gumball_extent(1000.0) - 2.5).abs() < 1e-6);
    assert!((gumball_extent(10.0) - 1.5).abs() < 1e-6);
}

#[test]
fn gumball_eye_points_from_pivot_to_camera() {
    let camera = Camera3d { position: vec3_new_m(0.0, 0.0, 10.0), target: Vec3::ZERO, up: vec3_new_m(0.0, 1.0, 0.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 100.0 };
    let eye = gumball_eye(&camera, Vec3::ZERO);
    assert!((eye.length_m() - 1.0).abs() < 1e-5);
    assert!(eye.z > 0.99);
}

#[test]
fn ray_plane_point_hits_plane_ahead_and_misses_parallel() {
    let hit = ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).expect("hit");
    assert!((hit.z - 0.0).abs() < 1e-5, "z={}", hit.z);
    assert!(ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(1.0, 0.0, 0.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).is_none());
}

#[test]
fn ray_plane_point_rejects_intersection_behind_origin() {
    assert!(ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, -1.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).is_none());
}

#[test]
fn gumball_axis_drag_plane_normal_is_perpendicular_to_axis() {
    let axis = vec3_new_m(1.0, 0.0, 0.0);
    let eye = vec3_new_m(0.0, 0.0, 1.0);
    let normal = gumball_axis_drag_plane_normal(axis, eye);
    assert!(normal.dot_m(axis).abs() < 1e-5, "dot={}", normal.dot_m(axis));
}

#[test]
fn gumball_axis_drag_plane_normal_handles_axis_aligned_with_eye() {
    let axis = vec3_new_m(0.0, 0.0, 1.0);
    let eye = vec3_new_m(0.0, 0.0, 1.0);
    let normal = gumball_axis_drag_plane_normal(axis, eye);
    assert!(normal.dot_m(axis).abs() < 1e-5, "dot={}", normal.dot_m(axis));
    assert!((normal.length_m() - 1.0).abs() < 1e-5);
}

#[test]
fn gumball_project_ray_onto_axis_measures_signed_offset() {
    let pivot = Vec3::ZERO;
    let axis = vec3_new_m(1.0, 0.0, 0.0);
    let eye = vec3_new_m(0.0, 0.0, 1.0);
    let offset = gumball_project_ray_onto_axis(vec3_new_m(3.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), pivot, axis, eye).expect("offset");
    assert!((offset - 3.0).abs() < 1e-4, "offset={offset}");
}

#[test]
fn ray_segment_distance_measures_perpendicular_gap() {
    let dist = ray_segment_distance(vec3_new_m(3.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(0.0, -5.0, 0.0), vec3_new_m(0.0, 5.0, 0.0)).expect("distance");
    assert!((dist - 3.0).abs() < 1e-4, "dist={dist}");
}

#[test]
fn ray_segment_distance_none_for_degenerate_segment() {
    assert!(ray_segment_distance(Vec3::ZERO, vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(5.0, 5.0, 5.0), vec3_new_m(5.0, 5.0, 5.0)).is_none());
}

#[test]
fn quat_from_basis_identity_axes_yields_identity_quaternion() {
    let q = quat_from_basis(vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(0.0, 1.0, 0.0), vec3_new_m(0.0, 0.0, 1.0));
    assert!((q[0]).abs() < 1e-5 && (q[1]).abs() < 1e-5 && (q[2]).abs() < 1e-5, "q={q:?}");
    assert!((q[3] - 1.0).abs() < 1e-5, "q={q:?}");
}

#[test]
fn quat_from_basis_round_trips_through_mat4_from_quat() {
    let q = quat_from_basis(vec3_new_m(0.0, 1.0, 0.0), vec3_new_m(-1.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0));
    let m = mat4_from_quat_m(q[0], q[1], q[2], q[3]);
    let rotated = m.transform_point_m(vec3_new_m(1.0, 0.0, 0.0));
    assert!((rotated.x - 0.0).abs() < 1e-4 && (rotated.y - 1.0).abs() < 1e-4, "rotated={rotated:?}");
}

#[test]
fn rotate_vector_by_90_degrees_around_z_axis() {
    let rotated = rotate_vector(vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0), std::f32::consts::FRAC_PI_2);
    assert!((rotated.x - 0.0).abs() < 1e-4, "x={}", rotated.x);
    assert!((rotated.y - 1.0).abs() < 1e-4, "y={}", rotated.y);
}

#[test]
fn axis_rotate_angle_measures_signed_rotation() {
    let axis = vec3_new_m(0.0, 0.0, 1.0);
    let start = vec3_new_m(1.0, 0.0, 0.0);
    let current = vec3_new_m(0.0, 1.0, 0.0);
    let angle = axis_rotate_angle(start, current, axis);
    assert!((angle - std::f32::consts::FRAC_PI_2).abs() < 1e-3, "angle={angle}");
    let angle_reverse = axis_rotate_angle(start, vec3_new_m(0.0, -1.0, 0.0), axis);
    assert!((angle_reverse + std::f32::consts::FRAC_PI_2).abs() < 1e-3, "angle={angle_reverse}");
}

#[test]
fn pick_closest_mesh_url_returns_fallback_when_no_entries() {
    let entries: [(f64, &str); 0] = [];
    assert_eq!(pick_closest_mesh_url(&entries, 5.0, Some("fallback.glb")), Some("fallback.glb"));
}

#[test]
fn pick_closest_mesh_url_selects_nearest_lod_entry() {
    let entries = [(1.0, "hi.glb"), (10.0, "mid.glb"), (100.0, "lo.glb")];
    assert_eq!(pick_closest_mesh_url(&entries, 8.0, None), Some("mid.glb"));
}

#[test]
fn pick_closest_mesh_url_filters_out_non_finite_and_negative_lods() {
    let entries = [(-1.0, "bad.glb"), (f64::NAN, "nan.glb"), (5.0, "good.glb")];
    assert_eq!(pick_closest_mesh_url(&entries, 3.0, None), Some("good.glb"));
}

#[test]
fn lod_grid_step_world_returns_finest_active_band() {
    assert_eq!(lod_grid_step_world(5000.0, 10.0), None);
    let step = lod_grid_step_world(1.0, 10.0).expect("step");
    assert!((step - 1.0).abs() < 1e-9, "step={step}");
}

#[test]
fn grid_placement_anchor_uses_orbit_xy_and_datum_z() {
    let anchor = grid_placement_anchor(vec3_new_m(3.0, 4.0, 999.0), [0.0, 0.0, 12.5]);
    assert_eq!(anchor, vec3_new_m(3.0, 4.0, 12.5));
}
