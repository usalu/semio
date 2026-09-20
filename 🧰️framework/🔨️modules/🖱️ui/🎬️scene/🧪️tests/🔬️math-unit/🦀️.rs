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
    let (origin, dir) = camera.ray_from_screen(400.0, 300.0, 800.0, 600.0);
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
    let instance = Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    let hit = ray_pick_instance(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance);
    assert!(hit.is_some());
}

#[test]
fn ray_aabb_misses_offset_box() {
    let mesh = test_box_mesh();
    let instance = Instance3d { id: "box".into(), model: mat4_translation_m(vec3_new_m(100.0, 0.0, 0.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    let hit = ray_pick_instance(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance);
    assert!(hit.is_none());
}

#[test]
fn frustum_contains_origin_box() {
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let planes = frustum_planes(view_proj);
    assert!(aabb_intersects_frustum(&planes, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]));
}

#[test]
fn frustum_culls_behind_camera_box() {
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let planes = frustum_planes(view_proj);
    assert!(aabb_intersects_frustum(&planes, [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]));
    let behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(2.0));
    let min = [behind.x - 0.1, behind.y - 0.1, behind.z - 0.1];
    let max = [behind.x + 0.1, behind.y + 0.1, behind.z + 0.1];
    assert!(!aabb_intersects_frustum(&planes, min, max));
}

fn concrete_forest_camera() -> Camera3d {
    Camera3d { position: vec3_new_m(30.0, -30.0, 20.0), target: vec3_new_m(7.0, 0.0, 3.0), up: vec3_new_m(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0, projection: CameraProjection3d::Perspective, zoom: 1.0 }
}

#[test]
fn concrete_forest_frustum_contains_target_box() {
    let camera = concrete_forest_camera();
    let view_proj = camera.view_proj(800.0, 800.0);
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
    let view_proj = camera.view_proj(800.0, 800.0);
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
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "far".into(), model: mat4_translation_m(vec3_new_m(0.0, 0.0, -500.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
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
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "partial".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
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

/// 📐️ LAW: the grid's fade radius follows the camera and stays inside the far plane it is drawn
/// against, so it is neither clipped away nor drawn with a hard square edge — React's
/// `cameraGridFadeDistance` (`🎨️r3f/🟦️.tsx`), viewport branch and all, now that the orbit camera
/// grows its own far plane the way React's does
/// (`📓️w7b-presenter-one-frame-per-boot.md` §5, `📓️w8b-orthographic-camera-and-3d-parity.md` §2).
#[test]
fn the_grid_fade_radius_follows_the_camera_and_stays_inside_the_far_plane() {
    let orbit = OrbitController::default();
    let camera = orbit.to_camera();
    assert_eq!(camera.far, adaptive_orbit_camera_far(orbit.distance));
    assert_eq!(camera.far, WORLD_ORBIT_CAMERA_MIN_FAR, "a near orbit sits on React's floor");
    assert_eq!(adaptive_orbit_camera_far(4_096.0), 2.0_f32.powf((4_096.0 * WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR).log2().ceil()), "and a far one follows the distance, quantized to a power of two");

    let fade = camera_grid_fade_distance(&camera, 0.0, 10.0, 800.0, 600.0);
    assert!(fade > 0.0, "the grid always covers something");
    assert!(fade <= f32::from(camera.far) * 0.25 + 1e-3, "and never reaches past a quarter of the far plane while the coverage floor allows it");
    let higher = Camera3d { position: vec3_new_m(camera.position.x, camera.position.y, camera.position.z * 20.0), ..camera.clone() };
    assert!(camera_grid_fade_distance(&higher, 0.0, 10.0, 800.0, 600.0) >= fade, "a camera further above the plane covers at least as much");
    assert_eq!(camera_grid_fade_distance(&camera, 0.0, 0.0, 800.0, 600.0), 0.0, "a degenerate step draws nothing");

    assert_eq!(lod_grid_fade_alpha(0.0, 100.0), 1.0, "the grid is opaque at its centre");
    assert_eq!(lod_grid_fade_alpha(100.0, 100.0), 0.0, "and gone at its rim");
    assert!(lod_grid_fade_alpha(50.0, 100.0) < 1.0 && lod_grid_fade_alpha(50.0, 100.0) > 0.0, "with a monotone fade between");
    assert_eq!(lod_grid_fade_alpha(10.0, 0.0), 0.0, "a zero fade radius draws nothing");
}

/// 🎥️ The puzzle 3d `Top` pane's camera, verbatim off the wire
/// (`🗑️generated/w7b-diag/dumps.json`): an orthographic plan view whose `up` is `+Y`.
fn puzzle3d_top_camera(zoom: f32) -> Camera3d {
    Camera3d {
        position: vec3_new_m(3.5, 0.0, 9.455),
        target: vec3_new_m(3.5, 0.0, 0.005),
        up: vec3_new_m(0.0, 1.0, 0.0),
        fov_y: 45.0_f32.to_radians(),
        near: WORLD_ORBIT_CAMERA_NEAR,
        far: WORLD_ORBIT_CAMERA_MIN_FAR,
        projection: CameraProjection3d::Orthographic,
        zoom,
    }
}

/// 🎥️ The puzzle 3d `Perspective` pane's camera, verbatim off the same wire.
fn puzzle3d_perspective_camera() -> Camera3d {
    Camera3d {
        position: vec3_new_m(9.17, -5.67, 4.2575),
        target: vec3_new_m(3.5, 0.0, 0.005),
        up: vec3_new_m(0.0, 0.0, 1.0),
        fov_y: 50.0_f32.to_radians(),
        near: WORLD_ORBIT_CAMERA_NEAR,
        far: WORLD_ORBIT_CAMERA_MIN_FAR,
        projection: CameraProjection3d::Perspective,
        zoom: 1.0,
    }
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

/// 📡️ LAW: the perspective pick ray survives React's adaptive far plane. It is three's
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

/// 🎯️ LAW: React's `WorldAutoFit` uses the sphere-fit eye distance in both camera families and
/// preserves parallel zoom from the independent projection-content frame.
#[test]
fn auto_fit_moves_both_camera_families_eye_distance_and_preserves_parallel_zoom() {
    let (width, height) = (478.0_f32, 814.0_f32);
    let minimum = [0.0_f32, -2.0, 0.0];
    let maximum = [7.0_f32, 2.0, 1.0];

    let perspective = OrbitController::default();
    let framed = frame_orbit_to_bounds(&perspective, minimum, maximum, width, height, WORLD_FRAME_BOUNDS_MARGIN);
    assert_eq!(framed.target, vec3_new_m(3.5, 0.0, 0.5), "both families centre on the box");
    assert!((framed.distance - perspective.distance).abs() > 1e-3, "a perspective fit dollies");
    assert!((framed.zoom - 1.0).abs() < 1e-6, "and leaves the identity zoom alone");

    let parallel = OrbitController { projection: CameraProjection3d::Orthographic, zoom: 7.7595, ..OrbitController::default() };
    let parallel_framed = frame_orbit_to_bounds(&parallel, minimum, maximum, width, height, WORLD_FRAME_BOUNDS_MARGIN);
    assert_eq!(parallel_framed.target, vec3_new_m(3.5, 0.0, 0.5));
    assert!((parallel_framed.distance - 11.888316).abs() < 1e-5, "Three OrthographicCamera has no aspect property, so WorldAutoFit uses one");
    let parallel_wide = frame_orbit_to_bounds(&parallel, minimum, maximum, height, width, WORLD_FRAME_BOUNDS_MARGIN);
    assert!((parallel_wide.distance - parallel_framed.distance).abs() < 1e-6, "parallel eye distance is independent of viewport aspect");
    assert!((parallel_framed.zoom - parallel.zoom).abs() < 1e-6, "and preserves projection-content zoom");
}

/// 📐️ LAW: a projection pane frames on React's CARDINAL half-extent, not on the box's projected
/// screen extent — `worldProjectionViewHalfExtent` reads `(hx, hy)` for a plan view, `(hx, hz)` for
/// front/back, `(hy, hz)` for left/right, `(hx, hz)` for a free oblique, and the ISOTROPIC span
/// `max(hx, hy, hz)` for everything else (`🎨️r3f/🟦️.tsx`).
///
/// 🧪️ The numbers are React's own, ported from `🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx`: a 50-wide
/// reference at `[7, 0, 0.01]` gives `halfExtent [25, 25, 0.5]`, and a `400 × 800` top pane frames
/// it at `zoom = 200 / (25 · 1.35)`. There was no Rust mirror of that fixture, which is why the
/// cardinal-versus-projected divergence went unpinned.
#[test]
fn a_projection_frame_reads_reacts_cardinal_half_extent() {
    let half = [25.0_f32, 25.0, 0.5];
    let cardinal = |view| world_projection_view_half_extent(WorldProjectionOrientation::Cardinal(view), false, half);
    assert_eq!(cardinal(WorldCardinalView::Top), (25.0, 25.0), "📐️ a plan view spans (hx, hy)");
    assert_eq!(cardinal(WorldCardinalView::Bottom), (25.0, 25.0));
    assert_eq!(cardinal(WorldCardinalView::Front), (25.0, 0.5), "📐️ front/back span (hx, hz)");
    assert_eq!(cardinal(WorldCardinalView::Back), (25.0, 0.5));
    assert_eq!(cardinal(WorldCardinalView::Left), (25.0, 0.5), "📐️ left/right span (hy, hz)");
    assert_eq!(cardinal(WorldCardinalView::Right), (25.0, 0.5));
    assert_eq!(world_projection_view_half_extent(WorldProjectionOrientation::Free, true, half), (25.0, 0.5), "📐️ a free oblique measures in (x, z)");
    assert_eq!(world_projection_view_half_extent(WorldProjectionOrientation::Free, false, half), (25.0, 25.0), "📐️ and every other free orientation takes the isotropic span");
    assert_eq!(world_projection_view_half_extent(WorldProjectionOrientation::Free, false, [1.0, 2.0, 9.0]), (9.0, 9.0), "📐️ isotropic means the LARGEST axis on both, React's `max(hx, hy, hz)`");

    let parallel = OrbitController { projection: CameraProjection3d::Orthographic, zoom: 1.0, ..OrbitController::default() };
    let (minimum, maximum) = ([7.0_f32 - 25.0, -25.0, 0.01 - 0.5], [7.0_f32 + 25.0, 25.0, 0.01 + 0.5]);
    let framed = frame_projection_orbit_to_bounds(&parallel, WorldProjectionOrientation::Cardinal(WorldCardinalView::Top), false, minimum, maximum, 400.0, 800.0, WORLD_PROJECTION_FRAME_PADDING);
    assert!((framed.target.x - 7.0).abs() < 1e-5 && framed.target.y.abs() < 1e-5 && (framed.target.z - 0.01).abs() < 1e-5, "📐️ the frame centres on the content box, as React's `computeWorldProjectionPose` does: {:?}", framed.target);
    assert!((framed.zoom - 200.0 / (25.0 * WORLD_PROJECTION_FRAME_PADDING)).abs() < 1e-3, "📐️ React's own chunkkey number, to the digit: {}", framed.zoom);
    assert!((framed.distance - parallel.distance).abs() < 1e-6, "📐️ a parallel frame never dollies");

    // 📐️ The divergence the cardinal rule exists to remove: a box seen down a corner. React frames
    // it on the isotropic span; the projected screen extent of the same box is strictly smaller.
    let corner = OrbitController { projection: CameraProjection3d::Orthographic, zoom: 1.0, yaw: 0.7, pitch: 0.6, ..OrbitController::default() };
    let (thin_min, thin_max) = ([-8.0_f32, -1.0, -1.0], [8.0_f32, 1.0, 1.0]);
    let isotropic = frame_projection_orbit_to_bounds(&corner, WorldProjectionOrientation::Free, false, thin_min, thin_max, 400.0, 800.0, WORLD_PROJECTION_FRAME_PADDING);
    let centered = OrbitController { target: vec3_new_m(0.0, 0.0, 0.0), ..corner };
    let (projected_half_width, projected_half_height) = screen_half_extent(&centered.to_camera(), thin_min, thin_max);
    let projected_zoom = world_projection_ortho_zoom(projected_half_width, projected_half_height, 400.0, 800.0, WORLD_PROJECTION_FRAME_PADDING);
    assert!(isotropic.zoom < projected_zoom, "📐️ React's isotropic span frames WIDER than the projected box extent: {} vs {}", isotropic.zoom, projected_zoom);
}

/// 🔎️ LAW: the wheel moves whichever number the family owns — three's `OrbitControls` dollies a
/// perspective camera and scales an orthographic one's `zoom` (`captureNavigationSnapshot` reports
/// exactly that split).
#[test]
fn the_wheel_dollies_a_perspective_orbit_and_zooms_a_parallel_one() {
    let mut perspective = OrbitController::default();
    let (distance, zoom) = (perspective.distance, perspective.zoom);
    perspective.zoom(-200.0);
    assert!(perspective.distance > distance, "a perspective wheel dollies");
    assert!((perspective.zoom - zoom).abs() < 1e-6, "and never touches the zoom");

    let mut parallel = OrbitController { projection: CameraProjection3d::Orthographic, zoom: WORLD_ORBIT_PARALLEL_DEFAULT_ZOOM, ..OrbitController::default() };
    let distance = parallel.distance;
    parallel.zoom(200.0);
    assert!(parallel.zoom > WORLD_ORBIT_PARALLEL_DEFAULT_ZOOM, "a parallel wheel scales the zoom");
    assert!((parallel.distance - distance).abs() < 1e-6, "and never dollies");
    parallel.zoom(-1e9);
    assert!(parallel.zoom >= WORLD_ORBIT_PARALLEL_ZOOM_MIN, "the frustum never inverts");
}

/// 🎥️ LAW: the two puzzle 3d panes' delivered cameras survive the orbit round-trip intact — this is
/// the fixture the whole packet is measured against. The `Top` pane's `up` is `+Y`, and the Z-up the
/// orbit used to hard-code made its `look_at` cross product degenerate, which is why that pane
/// rendered nothing useful (`📓️w8b-orthographic-camera-and-3d-parity.md` §1).
#[test]
fn the_puzzle3d_pane_cameras_survive_the_orbit_round_trip() {
    let top = puzzle3d_top_camera(1.0);
    let orbit = OrbitController::from_camera(&top);
    assert_eq!(orbit.projection, CameraProjection3d::Orthographic);
    assert_eq!(orbit.up, vec3_new_m(0.0, 1.0, 0.0));
    assert!((orbit.zoom - 1.0).abs() < 1e-6);
    let round_tripped = orbit.to_camera();
    assert!(round_tripped.position.sub_m(top.position).length_m() < 1e-3, "the plan eye survives: {:?}", round_tripped.position);
    assert_eq!(round_tripped.target, top.target);
    assert_eq!(round_tripped.up, top.up);
    let matrix = round_tripped.view_proj(478.0, 814.0);
    assert!(matrix.cols.iter().flatten().all(|value| value.is_finite()), "and the plan view-projection is finite, not NaN");

    let perspective = puzzle3d_perspective_camera();
    let orbit = OrbitController::from_camera(&perspective);
    assert_eq!(orbit.projection, CameraProjection3d::Perspective);
    assert!((orbit.distance - 9.0764).abs() < 1e-2, "distance={}", orbit.distance);
    assert!((orbit.fov_y.to_degrees() - 50.0).abs() < 1e-3);
    assert!(orbit.to_camera().position.sub_m(perspective.position).length_m() < 1e-3);
}

/// 📐️ LAW: the two `zoom` ↔ frustum mappings React uses are exact inverses, so a projection switch
/// never jumps scale (`worldProjectionMatchedOrthoZoom` / `…MatchedPerspectiveDistance`).
#[test]
fn the_matched_zoom_and_distance_mappings_invert_each_other() {
    let zoom = world_projection_matched_ortho_zoom(WORLD_LOD_REFERENCE_FOV_DEG, 9.45, 814.0);
    let distance = world_projection_matched_perspective_distance(WORLD_LOD_REFERENCE_FOV_DEG, zoom, 814.0);
    assert!((distance - 9.45).abs() < 1e-3, "distance={distance}");
    let camera = Camera3d { projection: CameraProjection3d::Orthographic, zoom, ..puzzle3d_perspective_camera() };
    assert!((lod_orbit_distance_for_camera(&camera, 0.0, 814.0) - 9.45).abs() < 1e-3, "a parallel pane bands its LOD on the matched distance");
    let perspective = puzzle3d_perspective_camera();
    assert!((lod_orbit_distance_for_camera(&perspective, 9.45, 814.0) - 9.45).abs() < 1e-6, "and a perspective one on its own");
}

/// 📐️ LAW: a parallel camera's visible radius is its own zoomed pixel extent, never the
/// height-above-plane a perspective one uses — React's `cameraGridVisibleRadius`.
#[test]
fn the_grid_visible_radius_reads_the_cameras_own_family() {
    let perspective = OrbitController::default().to_camera();
    let radius = camera_grid_visible_radius(&perspective, 0.0, 960.0, 800.0);
    assert!(radius > 0.0);

    let parallel = Camera3d { projection: CameraProjection3d::Orthographic, zoom: 1.0, ..perspective.clone() };
    let half = (960.0_f32 * 0.5).hypot(800.0 * 0.5);
    assert!((camera_grid_visible_radius(&parallel, 0.0, 960.0, 800.0) - half).abs() < 1e-3, "zoom 1 shows exactly the viewport in world units");
    let zoomed = Camera3d { zoom: 4.0, ..parallel.clone() };
    assert!((camera_grid_visible_radius(&zoomed, 0.0, 960.0, 800.0) - half / 4.0).abs() < 1e-3, "and four times the zoom shows a quarter of it");
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
    let instance = Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    let hit = ray_pick_mesh_detail(vec3_new_m(0.0, -0.5, -5.0), vec3_new_m(0.0, 0.0, 1.0), mesh, &instance).expect("hit");
    assert_eq!(hit.triangle_index, 0);
    assert!(hit.bary_u >= 0.0 && hit.bary_v >= 0.0 && hit.bary_u + hit.bary_v <= 1.0);
}

#[test]
fn ray_pick_mesh_detail_misses_when_aabb_not_hit() {
    let mesh = test_box_mesh();
    let instance = Instance3d { id: "box".into(), model: mat4_translation_m(vec3_new_m(50.0, 0.0, 0.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
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
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "face", None, false);
    assert_eq!(selected, vec!["42".to_string()]);
}

#[test]
fn screen_select_components_vertex_granularity_selects_ids() {
    let mut data = LegacyMeshOracleData::triangle();
    data.vertex_ids = vec![10, 11, 12];
    let mesh = paged_mesh_fixture(data);
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
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
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "edge", None, false);
    assert_eq!(selected, vec!["99".to_string()]);
}

#[test]
fn screen_select_components_default_granularity_selects_whole_instance() {
    let mesh = test_box_mesh();
    let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "whole".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
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
            Instance3d { id: "keep".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() },
            Instance3d { id: "skip".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() },
        ],
        shadow_role: Default::default(),
    }];
    let mut lookup = std::collections::HashMap::new();
    lookup.insert("box".into(), mesh);
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
    let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "unknown", Some("keep"), false);
    assert_eq!(selected, vec!["keep".to_string()]);
}

#[test]
fn screen_select_components_skips_missing_mesh_lookup() {
    let draws = vec![SceneDraw3d { mesh_key: "missing".into(), mesh_version: 0, instances: vec![], shadow_role: Default::default() }];
    let lookup = std::collections::HashMap::new();
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
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
    let camera = Camera3d { position: vec3_new_m(0.0, 0.0, 10.0), target: Vec3::ZERO, up: vec3_new_m(0.0, 1.0, 0.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 100.0, projection: CameraProjection3d::Perspective, zoom: 1.0 };
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
fn lod_grid_step_world_quantizes_like_reacts_helper() {
    assert_eq!(lod_grid_step_world(0.0, 10.0), None, "a degenerate LOD draws nothing");
    assert_eq!(lod_grid_step_world(-1.0, 10.0), None);
    assert_eq!(lod_grid_step_world(1.0, 0.0), None, "and so does a degenerate factor");
    let step = lod_grid_step_world(1.0, 10.0).expect("step");
    assert!((step - 10.0).abs() < 1e-9, "one factor covers everything up to the base LOD: step={step}");
    assert!((lod_grid_step_world(2.0, 10.0).expect("step") - 10.0).abs() < 1e-9);
    assert!((lod_grid_step_world(5.0, 10.0).expect("step") - 25.0).abs() < 1e-9, "2.5 quantum");
    assert!((lod_grid_step_world(9.0, 10.0).expect("step") - 50.0).abs() < 1e-9, "5 quantum");
    assert!((lod_grid_step_world(25.0, 10.0).expect("step") - 250.0).abs() < 1e-9, "and the next decade");
}

#[test]
fn grid_placement_anchor_uses_orbit_xy_and_datum_z() {
    let anchor = grid_placement_anchor(vec3_new_m(3.0, 4.0, 999.0), [0.0, 0.0, 12.5]);
    assert_eq!(anchor, vec3_new_m(3.0, 4.0, 12.5));
}
