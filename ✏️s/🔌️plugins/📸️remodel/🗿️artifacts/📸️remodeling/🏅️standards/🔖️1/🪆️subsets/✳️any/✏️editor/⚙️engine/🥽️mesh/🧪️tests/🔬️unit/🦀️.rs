
use super::*;

fn lcg_next(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
    (*state >> 11) as f64 / (1u64 << 53) as f64
}

fn intrinsics_for(width: u32, height: u32) -> remodeling_camera::Intrinsics {
    remodeling_camera::Intrinsics { fx: 0.55 * f64::from(width), fy: 0.55 * f64::from(width), cx: f64::from(width) / 2.0, cy: f64::from(height) / 2.0, skew: 0.0, distortion: remodeling_camera::Distortion::None }
}

fn look_at_pose(eye: [f64; 3], target: [f64; 3]) -> remodeling_camera::CameraPose {
    let forward = normalize3(sub3(target, eye));
    let world_up = if forward[1].abs() > 0.95 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let right = normalize3(cross3(forward, world_up));
    let up = cross3(right, forward);
    let rotation = crate::algebra::Mat3d::from_axes(right, up, forward).transpose();
    let translation = scale3(rotation.mul_vec3(eye), -1.0);
    remodeling_camera::CameraPose(crate::lie::Se3 { r: crate::lie::So3(rotation), t: translation })
}

fn checkerboard_image(width: u32, height: u32, cell: u32) -> remodeling_image::ImageRgba8 {
    let mut img = remodeling_image::ImageRgba8::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let on = ((x / cell) + (y / cell)).is_multiple_of(2);
            let v = if on { 220u8 } else { 30u8 };
            let idx = ((y * width + x) * 4) as usize;
            img.data[idx] = v;
            img.data[idx + 1] = v;
            img.data[idx + 2] = v;
            img.data[idx + 3] = 255;
        }
    }
    img
}

fn vertical_edge_image(width: u32, height: u32) -> remodeling_image::ImageRgba8 {
    let mut img = remodeling_image::ImageRgba8::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let v = if x < width / 2 { 0u8 } else { 255u8 };
            let idx = ((y * width + x) * 4) as usize;
            img.data[idx] = v;
            img.data[idx + 1] = v;
            img.data[idx + 2] = v;
            img.data[idx + 3] = 255;
        }
    }
    img
}

fn solid_color_image(width: u32, height: u32, value: u8) -> remodeling_image::ImageRgba8 {
    let mut img = remodeling_image::ImageRgba8::new(width, height);
    for px in img.data.chunks_mut(4) {
        px[0] = value;
        px[1] = value;
        px[2] = value;
        px[3] = 255;
    }
    img
}

fn sphere_trace(origin: [f64; 3], dir: [f64; 3], sdf: impl Fn([f64; 3]) -> f64, max_t: f64) -> Option<f64> {
    let mut t = 0.0;
    for _ in 0..128 {
        let p = add3(origin, scale3(dir, t));
        let d = sdf(p);
        if d < 1e-4 {
            return Some(t);
        }
        t += d.max(1e-4);
        if t > max_t {
            return None;
        }
    }
    None
}

fn render_sdf_depth_map(width: u32, height: u32, intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, sdf: impl Fn([f64; 3]) -> f64 + Copy) -> remodeling_dense::DepthMap {
    let mut dm = remodeling_dense::DepthMap::new(width, height);
    let to_world = pose.0.inverse();
    let origin_world = to_world.act([0.0, 0.0, 0.0]);
    for y in 0..height {
        for x in 0..width {
            let ray_cam = intr.unproject_ray([f64::from(x), f64::from(y)]);
            let ray_world = normalize3(sub3(to_world.act(ray_cam), origin_world));
            if let Some(t_world) = sphere_trace(origin_world, ray_world, sdf, 20.0) {
                let hit_world = add3(origin_world, scale3(ray_world, t_world));
                let depth = pose.0.act(hit_world)[2];
                if depth > 0.0 {
                    let idx = (y * width + x) as usize;
                    dm.depth[idx] = depth as f32;
                    dm.confidence[idx] = 1.0;
                }
            }
        }
    }
    dm
}

/// 🌐️ Fibonacci-sphere view directions: far denser and more uniform coverage than a handful of
/// axis-aligned/equatorial views, which otherwise leave thin uncovered strips between views
/// wide enough to show up as spurious TSDF boundary defects.
fn orbit_views(radius: f64) -> Vec<remodeling_camera::CameraPose> {
    let n = 40;
    let golden_angle = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    (0..n)
        .map(|i| {
            let y = 1.0 - 2.0 * (f64::from(i) + 0.5) / f64::from(n);
            let r = (1.0 - y * y).max(0.0).sqrt();
            let theta = golden_angle * f64::from(i);
            let dir = [r * theta.cos(), y, r * theta.sin()];
            look_at_pose(scale3(dir, radius), [0.0, 0.0, 0.0])
        })
        .collect()
}

fn build_tsdf_from_sdf(voxel: f64, truncation: f64, radius: f64, sdf: impl Fn([f64; 3]) -> f64 + Copy) -> remodeling_dense::TsdfVolume {
    let mut vol = remodeling_dense::TsdfVolume::new(voxel, truncation);
    let intr = intrinsics_for(96, 96);
    for pose in orbit_views(radius * 2.5) {
        let dm = render_sdf_depth_map(96, 96, &intr, &pose, sdf);
        vol.integrate(&dm, &(pose, intr), false);
    }
    vol
}

fn sphere_sdf(radius: f64) -> impl Fn([f64; 3]) -> f64 + Copy {
    move |p: [f64; 3]| norm3(p) - radius
}

fn torus_sdf(major: f64, minor: f64) -> impl Fn([f64; 3]) -> f64 + Copy {
    move |p: [f64; 3]| {
        let q = ((p[0] * p[0] + p[2] * p[2]).sqrt() - major, p[1]);
        (q.0 * q.0 + q.1 * q.1).sqrt() - minor
    }
}

/// 🌐️ Watertight UV sphere: a single north-pole vertex, `stacks - 1` interior latitude rings of
/// `slices` vertices each, and a single south-pole vertex — poles are *not* duplicated per
/// slice (an earlier version did, which left every pole-cap triangle touching its neighbors at
/// an isolated coincident-position vertex rather than a shared edge, producing hundreds of
/// spurious boundary edges). Winding is auto-corrected via [`TriMesh::signed_volume`] so this
/// helper never depends on getting the hand-derived winding right by inspection.
fn make_uv_sphere(radius: f64, stacks: usize, slices: usize) -> TriMesh {
    let mut positions = vec![[0.0, radius, 0.0]];
    for i in 1..stacks {
        let phi = std::f64::consts::PI * i as f64 / stacks as f64;
        for j in 0..slices {
            let theta = std::f64::consts::TAU * j as f64 / slices as f64;
            positions.push([radius * phi.sin() * theta.cos(), radius * phi.cos(), radius * phi.sin() * theta.sin()]);
        }
    }
    let south = positions.len() as u32;
    positions.push([0.0, -radius, 0.0]);
    let ring_start = |i: usize| 1 + (i - 1) * slices;
    let ring_vertex = |i: usize, j: usize| (ring_start(i) + j % slices) as u32;
    let mut triangles = Vec::new();
    for j in 0..slices {
        triangles.push([0u32, ring_vertex(1, j + 1), ring_vertex(1, j)]);
    }
    for i in 1..(stacks - 1) {
        for j in 0..slices {
            let (a, b, c, d) = (ring_vertex(i, j), ring_vertex(i, j + 1), ring_vertex(i + 1, j), ring_vertex(i + 1, j + 1));
            triangles.push([a, d, b]);
            triangles.push([a, c, d]);
        }
    }
    for j in 0..slices {
        triangles.push([south, ring_vertex(stacks - 1, j), ring_vertex(stacks - 1, j + 1)]);
    }
    let mut mesh = TriMesh { positions, triangles };
    orient_consistently(&mut mesh).expect("hand-built UV sphere topology is always orientable");
    orient_outward(&mut mesh);
    mesh
}

// #region 🔖️TriMeshTests
#[test]
fn edge_map_counts_faces_per_edge() {
    let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]], triangles: vec![[0, 1, 2], [1, 3, 2]] };
    let edges = mesh.edge_map();
    assert_eq!(edges.get(&(1, 2)).map(Vec::len), Some(2));
    assert_eq!(edges.get(&(0, 1)).map(Vec::len), Some(1));
}

#[test]
fn signed_volume_positive_for_outward_sphere() {
    let mesh = make_uv_sphere(1.0, 12, 16);
    assert!(mesh.signed_volume() > 0.0, "expected outward-wound sphere to have positive signed volume");
}
// #endregion 🔖️TriMeshTests

// #region 🔖️TopologyTests
#[test]
fn topology_error_display_messages() {
    assert_eq!(TopologyError::NonManifoldEdge { a: 1, b: 2, face_count: 3 }.to_string(), "edge (1,2) has 3 incident faces");
    assert_eq!(TopologyError::InconsistentOrientation { a: 1, b: 2 }.to_string(), "edge (1,2) is traversed the same direction by two faces");
    assert_eq!(TopologyError::NonManifoldVertex(5).to_string(), "vertex 5 has more than one incident fan");
    assert_eq!(TopologyError::DegenerateTriangle(7).to_string(), "triangle 7 is degenerate");
}

#[test]
fn orient_error_display_message() {
    assert_eq!(OrientError::UnresolvableConflict.to_string(), "could not consistently orient mesh after retry");
}

#[test]
fn halfedge_topology_build_rejects_degenerate_triangle() {
    let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], triangles: vec![[0, 0, 1]] };
    assert_eq!(HalfedgeTopology::build(&mesh).err().expect("degenerate triangle must be rejected"), TopologyError::DegenerateTriangle(0));
}

#[test]
fn halfedge_topology_build_rejects_non_manifold_edge() {
    let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]], triangles: vec![[0, 1, 2], [0, 1, 3], [0, 1, 4]] };
    let err = HalfedgeTopology::build(&mesh).err().expect("edge shared by 3 faces must be rejected");
    assert!(matches!(err, TopologyError::NonManifoldEdge { a: 0, b: 1, face_count: 3 }), "unexpected error: {err:?}");
}

#[test]
fn halfedge_topology_build_rejects_inconsistent_orientation() {
    let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0]], triangles: vec![[0, 1, 2], [0, 1, 3]] };
    let err = HalfedgeTopology::build(&mesh).err().expect("same-direction shared edge must be rejected");
    assert!(matches!(err, TopologyError::InconsistentOrientation { a: 0, b: 1 }), "unexpected error: {err:?}");
}

#[test]
fn halfedge_topology_build_rejects_non_manifold_vertex() {
    let mesh = two_spheres_sharing_vertex();
    let err = HalfedgeTopology::build(&mesh).err().expect("pinch vertex must be rejected");
    assert!(matches!(err, TopologyError::NonManifoldVertex(_)), "unexpected error: {err:?}");
}
// #endregion 🔖️TopologyTests

// #region 🔖️CloseUnitTest
#[test]
fn close_voxel_alone_is_always_closed_and_manifold() {
    let mut state = 42u64;
    let mut positions = Vec::new();
    for _ in 0..40 {
        positions.push([lcg_next(&mut state) * 2.0 - 1.0, lcg_next(&mut state) * 2.0 - 1.0, lcg_next(&mut state) * 2.0 - 1.0]);
    }
    let mut triangles = Vec::new();
    for _ in 0..60 {
        let a = (lcg_next(&mut state) * 40.0) as u32 % 40;
        let b = (lcg_next(&mut state) * 40.0) as u32 % 40;
        let c = (lcg_next(&mut state) * 40.0) as u32 % 40;
        if a != b && b != c && a != c {
            triangles.push([a, b, c]);
        }
    }
    let soup = TriMesh { positions, triangles };
    let closed = close_voxel(&soup, 0.15);
    let report = validate_watertight(&closed, false);
    assert!(report.is_closed, "close_voxel output must have zero boundary edges, got {}", report.boundary_edge_count);
    assert!(report.is_two_manifold, "close_voxel output must be a 2-manifold, non-manifold edges={} vertices={}", report.non_manifold_edge_count, report.non_manifold_vertex_count);
}
// #endregion 🔖️CloseUnitTest

// #region 🔖️DenseFieldTests
#[test]
fn dense_field_get_set_out_of_range_are_noops() {
    let mut field = DenseField::new(2, 2, 2, [0.0; 3], 1.0, 0.0);
    assert_eq!(field.get(-1, 0, 0), None);
    assert_eq!(field.get(5, 0, 0), None);
    assert_eq!(field.get(0, 5, 0), None);
    field.set(-1, 0, 0, 9.0);
    field.set(0, 0, 0, 9.0);
    assert_eq!(field.get(0, 0, 0), Some(9.0));
}
// #endregion 🔖️DenseFieldTests

// #region 🔖️WatertightSuite
#[test]
fn tsdf_sphere_across_blocks_is_watertight_and_correct() {
    let voxel = 0.05;
    let radius = 0.6;
    let vol = build_tsdf_from_sdf(voxel, voxel * 3.0, radius, sphere_sdf(radius));
    let bound = ((radius + voxel * 4.0) / voxel).ceil() as i32;
    let mesh = extract_tsdf(&vol, 0.0, [-bound, -bound, -bound], [bound, bound, bound]);
    let report = validate_watertight(&mesh, false);
    assert_eq!(report.boundary_edge_count, 0, "expected a fully seam-welded sphere with zero boundary edges");
    assert!(report.is_watertight, "sphere across blocks report: {report:?}");
    assert_eq!(report.euler_characteristic, 2, "sphere euler characteristic report: {report:?}");
    let volume = report.signed_volume.abs();
    let expected = 4.0 / 3.0 * std::f64::consts::PI * radius.powi(3);
    let error = (volume - expected).abs() / expected;
    assert!(error < 0.03, "sphere volume error {error} too large: got {volume}, expected {expected}");
}

#[test]
fn tsdf_torus_across_blocks_has_genus_one() {
    let voxel = 0.05;
    let (major, minor) = (0.5, 0.2);
    let vol = build_tsdf_from_sdf(voxel, voxel * 3.0, major + minor, torus_sdf(major, minor));
    let bound = (((major + minor) + voxel * 4.0) / voxel).ceil() as i32;
    let bound_y = ((minor + voxel * 4.0) / voxel).ceil() as i32;
    let mesh = extract_tsdf(&vol, 0.0, [-bound, -bound_y, -bound], [bound, bound_y, bound]);
    let report = validate_watertight(&mesh, false);
    assert!(report.is_watertight, "torus across blocks report: {report:?}");
    assert_eq!(report.euler_characteristic, 0, "torus euler characteristic report: {report:?}");
    assert_eq!(report.genus, Some(1), "torus genus report: {report:?}");
}

fn delete_patch(mesh: &mut TriMesh, center_face: usize, target_boundary_verts: usize) {
    let edges = mesh.edge_map();
    let mut face_adjacency: Vec<Vec<u32>> = vec![Vec::new(); mesh.triangles.len()];
    for faces in edges.values() {
        if faces.len() == 2 {
            face_adjacency[faces[0] as usize].push(faces[1]);
            face_adjacency[faces[1] as usize].push(faces[0]);
        }
    }
    let tri_edges = |mesh: &TriMesh, f: u32| -> [(u32, u32); 3] {
        let t = mesh.triangles[f as usize];
        [sorted_edge(t[0], t[1]), sorted_edge(t[1], t[2]), sorted_edge(t[2], t[0])]
    };
    let mut removed: HashSet<u32> = HashSet::new();
    let mut frontier: HashSet<(u32, u32)> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(center_face as u32);
    removed.insert(center_face as u32);
    for e in tri_edges(mesh, center_face as u32) {
        frontier.insert(e);
    }
    let max_removed_faces = (target_boundary_verts * target_boundary_verts).max(64);
    while frontier.len() < target_boundary_verts && removed.len() < max_removed_faces {
        let Some(f) = queue.pop_front() else { break };
        for &nb in &face_adjacency[f as usize] {
            if !removed.insert(nb) {
                continue;
            }
            queue.push_back(nb);
            for e in tri_edges(mesh, nb) {
                if !frontier.remove(&e) {
                    frontier.insert(e);
                }
            }
            if frontier.len() >= target_boundary_verts || removed.len() >= max_removed_faces {
                break;
            }
        }
    }
    mesh.triangles = mesh.triangles.iter().enumerate().filter(|&(f, _)| !removed.contains(&(f as u32))).map(|(_, t)| *t).collect();
}

#[test]
fn planted_holes_fill_and_trigger_all_three_strategies() {
    let mut mesh = make_uv_sphere(1.0, 100, 150);
    let original_volume = mesh.signed_volume().abs();
    delete_patch(&mut mesh, 2850, 6);
    delete_patch(&mut mesh, 25350, 35);
    delete_patch(&mut mesh, 14550, 70);
    clean_mesh(&mut mesh, 0, 0.0);
    repair_non_manifold(&mut mesh);
    orient_consistently(&mut mesh).expect("planted-hole sphere stays orientable");
    let stats = fill_holes(&mut mesh, &HoleFillParams::default());
    assert!(stats.ear_fan_used >= 1, "expected ear-fan strategy to trigger, stats={stats:?}");
    assert!(stats.min_weight_dp_used >= 1, "expected min-weight DP strategy to trigger, stats={stats:?}");
    assert!(stats.advancing_front_used >= 1, "expected advancing-front strategy to trigger, stats={stats:?}");
    let report = validate_watertight(&mesh, false);
    assert!(report.is_watertight, "planted-hole sphere after fill_holes report: {report:?}");
    let volume_error = (mesh.signed_volume().abs() - original_volume).abs() / original_volume;
    assert!(volume_error < 0.02, "volume error {volume_error} after hole filling too large");
}
// #endregion 🔖️WatertightSuite

// #region 🔖️HoleFillStatsTests
#[test]
fn fill_holes_skips_loops_larger_than_max_boundary_verts() {
    let mut mesh = make_uv_sphere(1.0, 30, 40);
    delete_patch(&mut mesh, 0, 50);
    clean_mesh(&mut mesh, 0, 0.0);
    repair_non_manifold(&mut mesh);
    orient_consistently(&mut mesh).expect("planted-hole sphere stays orientable");
    let stats = fill_holes(&mut mesh, &HoleFillParams { max_boundary_verts: 10 });
    assert!(stats.holes_skipped_too_large >= 1, "stats={stats:?}");
    assert_eq!(stats.holes_filled, 0, "stats={stats:?}");
}
// #endregion 🔖️HoleFillStatsTests

// #region 🔖️RepairTests
fn two_spheres_sharing_edge() -> TriMesh {
    let a = make_uv_sphere(1.0, 10, 10);
    let mut b = make_uv_sphere(1.0, 10, 10);
    for p in &mut b.positions {
        p[0] += 2.0;
    }
    let shift = a.positions.len() as u32;
    let shared_a = a.triangles[0][0];
    let shared_a2 = a.triangles[0][1];
    for tri in &mut b.triangles {
        for v in tri.iter_mut() {
            *v += shift;
        }
    }
    let shared_b = b.triangles[0][0];
    let shared_b2 = b.triangles[0][1];
    let mut mesh = a;
    mesh.positions.extend(b.positions);
    mesh.triangles.extend(b.triangles);
    for tri in &mut mesh.triangles {
        for v in tri.iter_mut() {
            if *v == shared_b {
                *v = shared_a;
            } else if *v == shared_b2 {
                *v = shared_a2;
            }
        }
    }
    mesh
}

fn two_spheres_sharing_vertex() -> TriMesh {
    let a = make_uv_sphere(1.0, 10, 10);
    let mut b = make_uv_sphere(1.0, 10, 10);
    for p in &mut b.positions {
        p[0] += 2.0;
    }
    let shift = a.positions.len() as u32;
    let shared_a = a.triangles[0][0];
    for tri in &mut b.triangles {
        for v in tri.iter_mut() {
            *v += shift;
        }
    }
    let shared_b = b.triangles[0][0];
    let mut mesh = a;
    mesh.positions.extend(b.positions);
    mesh.triangles.extend(b.triangles);
    for tri in &mut mesh.triangles {
        for v in tri.iter_mut() {
            if *v == shared_b {
                *v = shared_a;
            }
        }
    }
    mesh
}

#[test]
fn repair_splits_bowtie_edge_into_two_components() {
    let mut mesh = two_spheres_sharing_edge();
    let stats = repair_non_manifold(&mut mesh);
    assert!(stats.non_manifold_edges_split >= 1, "expected at least one non-manifold edge split, stats={stats:?}");
    let report = validate_watertight(&mesh, false);
    assert_eq!(report.non_manifold_edge_count, 0, "report: {report:?}");
    assert_eq!(report.non_manifold_vertex_count, 0, "report: {report:?}");
    assert_eq!(report.connected_components, 2, "report: {report:?}");
}

#[test]
fn repair_splits_pinch_vertex_into_two_components() {
    let mut mesh = two_spheres_sharing_vertex();
    let stats = repair_non_manifold(&mut mesh);
    assert!(stats.non_manifold_vertices_split >= 1, "expected at least one pinch vertex split, stats={stats:?}");
    let report = validate_watertight(&mesh, false);
    assert_eq!(report.non_manifold_edge_count, 0, "report: {report:?}");
    assert_eq!(report.non_manifold_vertex_count, 0, "report: {report:?}");
    assert_eq!(report.connected_components, 2, "report: {report:?}");
}

#[test]
fn orient_consistently_recovers_from_flipped_windings() {
    let mut mesh = make_uv_sphere(1.0, 20, 30);
    let mut state = 7u64;
    for tri in &mut mesh.triangles {
        if lcg_next(&mut state) < 0.3 {
            tri.swap(1, 2);
        }
    }
    orient_consistently(&mut mesh).expect("sphere with flipped windings is still orientable");
    orient_outward(&mut mesh);
    let report = validate_watertight(&mesh, false);
    assert!(report.consistently_oriented, "report: {report:?}");
    assert!(report.signed_volume > 0.0, "report: {report:?}");
}
// #endregion 🔖️RepairTests

// #region 🔖️CleanTests
#[test]
fn clean_mesh_welds_duplicate_vertices() {
    let mut mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]], triangles: vec![[0, 1, 2], [3, 1, 2]] };
    let stats = clean_mesh(&mut mesh, 0, 0.0);
    assert_eq!(stats.vertices_welded, 1, "stats={stats:?}");
    assert_eq!(mesh.vertex_count(), 3);
}

#[test]
fn clean_mesh_removes_degenerate_triangles() {
    let mut mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 1.0, 0.0]], triangles: vec![[0, 1, 2], [0, 1, 3]] };
    let stats = clean_mesh(&mut mesh, 0, 0.0);
    assert_eq!(stats.degenerate_triangles_removed, 1, "stats={stats:?}");
    assert_eq!(mesh.triangle_count(), 1);
}

#[test]
fn clean_mesh_collapses_near_zero_length_edges() {
    let mut mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1e-13, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], triangles: vec![[0, 1, 3], [1, 2, 3]] };
    let stats = clean_mesh(&mut mesh, 0, 0.0);
    assert!(stats.zero_length_edges_collapsed >= 1, "stats={stats:?}");
}

#[test]
fn clean_mesh_removes_small_disconnected_components() {
    let mut mesh = make_uv_sphere(1.0, 10, 10);
    let shift = mesh.positions.len() as u32;
    mesh.positions.extend([[100.0, 100.0, 100.0], [100.01, 100.0, 100.0], [100.0, 100.01, 100.0]]);
    mesh.triangles.push([shift, shift + 1, shift + 2]);
    let stats = clean_mesh(&mut mesh, 8, 0.02);
    assert_eq!(stats.small_components_removed, 1, "stats={stats:?}");
}
// #endregion 🔖️CleanTests

// #region 🔖️TaubinTests
#[test]
fn taubin_smooth_noop_on_empty_mesh() {
    let mut mesh = TriMesh::new();
    taubin_smooth(&mut mesh, 0.5, -0.53, 5);
    assert!(mesh.positions.is_empty());
}

#[test]
fn taubin_smooth_reduces_vertex_noise_amplitude() {
    let mut state = 99u64;
    let mut mesh = make_uv_sphere(1.0, 20, 30);
    for p in &mut mesh.positions {
        let n = normalize3(*p);
        *p = add3(*p, scale3(n, (lcg_next(&mut state) - 0.5) * 0.05));
    }
    let radius_variance = |positions: &[[f64; 3]]| -> f64 {
        let radii: Vec<f64> = positions.iter().map(|p| norm3(*p)).collect();
        let mean = radii.iter().sum::<f64>() / radii.len() as f64;
        radii.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / radii.len() as f64
    };
    let noisy_variance = radius_variance(&mesh.positions);
    taubin_smooth(&mut mesh, 0.5, -0.53, 10);
    let smoothed_variance = radius_variance(&mesh.positions);
    assert!(smoothed_variance < noisy_variance, "expected taubin smoothing to reduce radius variance: before={noisy_variance} after={smoothed_variance}");
}
// #endregion 🔖️TaubinTests

// #region 🔖️SimplifyTests
#[test]
fn qem_simplification_preserves_watertight_invariant() {
    let mut mesh = make_uv_sphere(1.0, 30, 45);
    let before = validate_watertight(&mesh, false);
    assert!(before.is_watertight, "sanity: uv sphere must start watertight, report: {before:?}");
    let target = (mesh.triangles.len() as f64 * 0.2) as usize;
    let stats = simplify_qem(&mut mesh, target, &SimplifyParams::default());
    assert!(stats.collapses_performed > 0, "expected simplification to perform collapses, stats={stats:?}");
    let after = validate_watertight(&mesh, false);
    assert!(after.is_watertight, "simplified sphere must stay watertight, report: {after:?}");
}

#[test]
fn simplify_qem_rejects_collapses_that_violate_link_condition() {
    let mut mesh = two_spheres_sharing_edge();
    let stats = simplify_qem(&mut mesh, 4, &SimplifyParams::default());
    assert!(stats.collapses_rejected_by_link_condition > 0, "expected the shared bowtie edge to force at least one link-condition rejection, stats={stats:?}");
}

#[test]
fn simplify_qem_stops_when_error_exceeds_max_error() {
    let mut mesh = make_uv_sphere(1.0, 20, 30);
    let total_before = mesh.triangle_count();
    let stats = simplify_qem(&mut mesh, 0, &SimplifyParams { max_error: 1e-6 });
    assert!(stats.collapses_performed > 0, "some cheap collapses should still happen, stats={stats:?}");
    assert!(mesh.triangle_count() > 0, "max_error should halt simplification before the mesh disappears, stats={stats:?}");
    assert!(mesh.triangle_count() < total_before, "expected at least some simplification, stats={stats:?}");
}
// #endregion 🔖️SimplifyTests

// #region 🔖️SegmentChartsTests
#[test]
fn segment_charts_keeps_coplanar_faces_in_one_chart() {
    let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], triangles: vec![[0, 1, 2], [0, 2, 3]] };
    let charts = segment_charts(&mesh, 10.0);
    assert_eq!(charts.len(), 1, "coplanar faces within threshold should stay in one chart");
    assert_eq!(charts[0].faces.len(), 2);
}

#[test]
fn segment_charts_cuts_at_sharp_dihedral_angle() {
    let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]], triangles: vec![[0, 1, 2], [0, 2, 3], [0, 3, 5], [0, 5, 4]] };
    let charts = segment_charts(&mesh, 45.0);
    assert_eq!(charts.len(), 2, "a 90-degree fold above a 45-degree threshold should split into two charts");
    for chart in &charts {
        assert_eq!(chart.faces.len(), 2);
    }
}
// #endregion 🔖️SegmentChartsTests

// #region 🔖️UnwrapTests
#[test]
fn lscm_unwrap_is_bijective_per_chart() {
    let mut mesh = make_uv_sphere(1.0, 12, 18);
    let target = (mesh.triangles.len() as f64 * 0.5) as usize;
    simplify_qem(&mut mesh, target, &SimplifyParams::default());
    let report = validate_watertight(&mesh, false);
    assert!(report.is_watertight, "sanity: simplified sphere must be watertight before unwrap, report: {report:?}");
    let charts = segment_charts(&mesh, 60.0);
    let uvs = unwrap_mesh(&mut mesh, &charts);
    for chart in &charts {
        let mut sign: Option<f64> = None;
        for &f in &chart.faces {
            let tri = mesh.triangles[f as usize];
            let (a, b, c) = (uvs[tri[0] as usize], uvs[tri[1] as usize], uvs[tri[2] as usize]);
            let area2 = f64::from(b[0] - a[0]) * f64::from(c[1] - a[1]) - f64::from(c[0] - a[0]) * f64::from(b[1] - a[1]);
            if area2.abs() < 1e-12 {
                continue;
            }
            let s = area2.signum();
            if let Some(prev) = sign {
                assert_eq!(prev, s, "chart has inconsistent UV triangle winding (an inversion)");
            }
            sign = Some(s);
        }
    }
}
// #endregion 🔖️UnwrapTests

// #region 🔖️SelfIntersectionTests
#[test]
fn validate_watertight_detects_self_intersections_when_requested() {
    let mesh = TriMesh { positions: vec![[-1.0, 0.0, -1.0], [1.0, 0.0, -1.0], [0.0, 0.0, 2.0], [0.0, -1.0, -0.5], [0.0, 1.0, -0.5], [0.0, 0.0, 1.0]], triangles: vec![[0, 1, 2], [3, 4, 5]] };
    let report = validate_watertight(&mesh, true);
    assert_eq!(report.self_intersection_pairs, Some(1), "report: {report:?}");
}

#[test]
fn validate_watertight_reports_no_self_intersections_for_disjoint_triangles() {
    let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [10.0, 10.0, 10.0], [11.0, 10.0, 10.0], [10.0, 11.0, 10.0]], triangles: vec![[0, 1, 2], [3, 4, 5]] };
    let report = validate_watertight(&mesh, true);
    assert_eq!(report.self_intersection_pairs, Some(0), "report: {report:?}");
}
// #endregion 🔖️SelfIntersectionTests

// #region 🔖️ContractTest
#[test]
fn pipeline_falls_back_to_close_on_pathological_input() {
    let mut mesh = make_uv_sphere(1.0, 40, 60);
    delete_patch(&mut mesh, 0, 60);
    let mut overlapping = make_uv_sphere(1.0, 8, 8);
    for p in &mut overlapping.positions {
        p[0] += 0.3;
    }
    let shift = mesh.positions.len() as u32;
    for tri in &overlapping.triangles {
        mesh.triangles.push([tri[0] + shift, tri[1] + shift, tri[2] + shift]);
    }
    mesh.positions.extend(overlapping.positions);
    let params = MeshParams { guarantee_watertight: true, hole_fill_max_boundary_verts: 20, ..MeshParams::default() };
    let mut pipeline = MeshPipeline::from_mesh(mesh, params);
    let mut status = mesh_pipeline_step(&mut pipeline, 1);
    let mut guard = 0;
    while !matches!(status, MeshPipelineStatus::Done | MeshPipelineStatus::Failed(_)) {
        status = mesh_pipeline_step(&mut pipeline, 1);
        guard += 1;
        assert!(guard < 100, "pipeline did not terminate within a reasonable number of steps");
    }
    assert!(matches!(status, MeshPipelineStatus::Done), "pipeline status: {status:?}");
    let report = pipeline.report().expect("pipeline recorded a watertight report");
    assert!(report.closed_fallback_used, "expected the pathological mesh to trigger close_voxel, report: {report:?}");
    assert!(report.is_watertight, "expected close_voxel's output to be watertight, report: {report:?}");
}

/// 🪸️ A tiny UV-sphere island with one small planted hole (an ear-fannable, individually
/// closable defect on its own), positioned far from the origin so it shares no vertices or
/// bounding volume with any other island.
fn make_tiny_holed_island(index: usize) -> TriMesh {
    let mut island = make_uv_sphere(0.05, 6, 6);
    delete_patch(&mut island, 0, 6);
    let offset = [(index % 12) as f64 * 0.6, ((index / 12) % 12) as f64 * 0.6, (index / 144) as f64 * 0.6];
    for p in &mut island.positions {
        *p = add3(*p, offset);
    }
    island
}

/// 🪸️ Simulates a badly under-registered SfM/TSDF reconstruction reaching `remodeling_mesh`: not
/// one hand-crafted pathological shape, but hundreds of small, mutually disjoint, individually
/// well-formed islands — the real-world failure mode a fresh 48-frame orbiting-cube end-to-end
/// diagnostic actually produced (34280 vertices / 44244 triangles / 1321 components / 22234
/// boundary edges going into this pipeline).
fn make_scattered_fragment_soup(count: usize) -> TriMesh {
    let mut mesh = TriMesh::new();
    for i in 0..count {
        let island = make_tiny_holed_island(i);
        let shift = mesh.positions.len() as u32;
        for tri in &island.triangles {
            mesh.triangles.push([tri[0] + shift, tri[1] + shift, tri[2] + shift]);
        }
        mesh.positions.extend(island.positions);
    }
    mesh
}

#[test]
fn pipeline_falls_back_to_close_on_catastrophically_fragmented_reconstruction() {
    let mesh = make_scattered_fragment_soup(300);
    let raw_report = validate_watertight(&mesh, false);
    assert!(raw_report.connected_components >= 250, "expected the fragment soup to actually be badly fragmented, report: {raw_report:?}");
    assert!(raw_report.boundary_edge_count > 0, "expected the raw fragment soup to have real boundary edges, report: {raw_report:?}");
    let params = MeshParams::default();
    assert!(params.guarantee_watertight, "sanity: the guarantee must default on, exactly as the real diagnostic observed");
    let mut pipeline = MeshPipeline::from_mesh(mesh, params);
    let mut status = mesh_pipeline_step(&mut pipeline, 1);
    let mut guard = 0;
    while !matches!(status, MeshPipelineStatus::Done | MeshPipelineStatus::Failed(_)) {
        status = mesh_pipeline_step(&mut pipeline, 1);
        guard += 1;
        assert!(guard < 100, "pipeline did not terminate within a reasonable number of steps");
    }
    assert!(matches!(status, MeshPipelineStatus::Done), "pipeline status: {status:?}");
    let report = pipeline.report().expect("pipeline recorded a watertight report");
    assert!(
        report.closed_fallback_used,
        "expected hundreds of small individually-closable islands to still trip the close_voxel guarantee (fill_holes can legitimately seal every island's tiny hole one at a time without the aggregate mesh ever being one coherent solid), report: {report:?}"
    );
    assert!(report.is_watertight, "expected close_voxel's output to be watertight, report: {report:?}");
}
// #endregion 🔖️ContractTest

// #region 🔖️TextureTests
#[test]
fn image_gradient_magnitude_zero_on_flat_image() {
    let img = solid_color_image(32, 32, 100);
    assert_eq!(image_gradient_magnitude(&img, 16.0, 16.0), 0.0);
}

#[test]
fn image_gradient_magnitude_detects_vertical_edge() {
    let img = vertical_edge_image(32, 32);
    let at_edge = image_gradient_magnitude(&img, 16.0, 16.0);
    let away_from_edge = image_gradient_magnitude(&img, 4.0, 16.0);
    assert!(at_edge > 0.9, "expected a near-maximal normalized gradient right at the edge, got {at_edge}");
    assert_eq!(away_from_edge, 0.0, "expected zero gradient away from the edge, got {away_from_edge}");
}

#[test]
fn face_projected_area_none_behind_camera_some_in_front() {
    let intr = intrinsics_for(64, 64);
    let identity_pose = remodeling_camera::CameraPose(crate::lie::Se3 { r: crate::lie::So3(crate::algebra::Mat3d::IDENTITY), t: [0.0, 0.0, 0.0] });
    let behind = TriMesh { positions: vec![[-0.1, -0.1, -1.0], [0.1, -0.1, -1.0], [0.0, 0.1, -1.0]], triangles: vec![[0, 1, 2]] };
    assert!(face_projected_area(&behind, 0, &intr, &identity_pose).is_none());
    let front = TriMesh { positions: vec![[-0.1, -0.1, 2.0], [0.1, -0.1, 2.0], [0.0, 0.1, 2.0]], triangles: vec![[0, 1, 2]] };
    let area = face_projected_area(&front, 0, &intr, &identity_pose).expect("triangle in front of camera projects");
    assert!(area > 0.0, "expected a positive projected area, got {area}");
}

#[test]
fn level_seam_solves_gain_and_offset() {
    let a: Vec<[f32; 3]> = (0..5).map(|i| [f64::from(i) as f32 * 10.0; 3]).collect();
    let b: Vec<[f32; 3]> = a.iter().map(|p| [p[0] * 2.0 + 5.0, p[1] * 2.0 + 5.0, p[2] * 2.0 + 5.0]).collect();
    let fit = level_seam(&a, &b);
    for (gain, offset) in fit {
        assert!((gain - 2.0).abs() < 1e-3, "fit={fit:?}");
        assert!((offset - 5.0).abs() < 1e-2, "fit={fit:?}");
    }
}

#[test]
fn level_seam_returns_identity_for_insufficient_samples() {
    assert_eq!(level_seam(&[[1.0, 2.0, 3.0]], &[[4.0, 5.0, 6.0]]), [(1.0, 0.0); 3]);
}

#[test]
fn bake_texture_paints_atlas_from_multiple_views() {
    let mut mesh = make_uv_sphere(0.4, 10, 14);
    let target = (mesh.triangle_count() as f64 * 0.3) as usize;
    simplify_qem(&mut mesh, target, &SimplifyParams::default());
    let charts = segment_charts(&mesh, 60.0);
    let uvs = unwrap_mesh(&mut mesh, &charts);
    let intr = intrinsics_for(64, 64);
    let views: Vec<TextureView> = orbit_views(2.0).into_iter().take(8).map(|pose| TextureView { pose, intrinsics: intr, image: checkerboard_image(64, 64, 8) }).collect();
    let atlas = bake_texture(&mesh, &uvs, 64, &views);
    let painted = atlas.data.chunks(4).filter(|px| px[3] == 255).count();
    assert!(painted > 0, "expected bake_texture to paint at least some atlas pixels from {} views", views.len());
}

#[test]
fn bake_texture_is_empty_with_no_views() {
    let mut mesh = make_uv_sphere(0.4, 6, 8);
    let charts = segment_charts(&mesh, 60.0);
    let uvs = unwrap_mesh(&mut mesh, &charts);
    let atlas = bake_texture(&mesh, &uvs, 32, &[]);
    assert!(atlas.data.iter().all(|&b| b == 0), "expected an untouched (fully transparent) atlas with no views");
}

#[test]
fn accepted_interactive_mesh_postprocess_steps_stay_below_hard_ceiling() {
    let mesh = make_uv_sphere(0.4, 12, 22);
    assert!(mesh.positions.len() <= 512);
    assert!(mesh.triangles.len() <= 512);
    let params = MeshParams { atlas_size: 64, taubin_iterations: 4, ..MeshParams::default() };
    let mut pipeline = MeshPipeline::from_mesh(mesh, params);
    pipeline.interactive = true;
    let mut steps = 0usize;
    loop {
        let started = std::time::Instant::now();
        let status = mesh_pipeline_step(&mut pipeline, 1);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "interactive mesh worker step {steps} exceeded 8 ms: {status:?}");
        steps += 1;
        match status {
            MeshPipelineStatus::Done => break,
            MeshPipelineStatus::Failed(message) => panic!("accepted watertight mesh failed: {message}"),
            MeshPipelineStatus::Working { .. } => {}
        }
        assert!(steps < 200_000, "interactive mesh pipeline failed to terminate");
    }
    assert!(pipeline.result().is_some());
}

#[test]
fn accepted_tsdf_extraction_and_envelope_rejection_steps_stay_below_hard_ceiling() {
    let volume = build_tsdf_from_sdf(0.16, 0.24, 0.38, sphere_sdf(0.38));
    let mut accepted = TsdfExtractionPreparation::new(0.0, [-4; 3], [4; 3]);
    let mut steps = 0usize;
    loop {
        let started = std::time::Instant::now();
        let complete = accepted.advance(&volume, 1);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "accepted TSDF extraction worker step {steps} exceeded 8 ms");
        steps += 1;
        if complete {
            break;
        }
        assert!(steps < 2_000, "accepted TSDF extraction failed to terminate");
    }
    assert!(!accepted.exceeded(), "accepted TSDF fixture must stay inside the 512-element envelope");
    let accepted = accepted.finish().expect("accepted TSDF extraction");
    assert!(accepted.positions.len() <= 512);
    assert!(accepted.triangles.len() <= 512);

    let dense_volume = build_tsdf_from_sdf(0.05, 0.2, 0.5, sphere_sdf(0.5));
    let mut rejected = TsdfExtractionPreparation::new(0.0, [-12; 3], [12; 3]);
    let mut steps = 0usize;
    while !rejected.exceeded() {
        let started = std::time::Instant::now();
        let _ = rejected.advance(&dense_volume, 1);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "TSDF envelope-rejection worker step {steps} exceeded 8 ms");
        steps += 1;
        assert!(steps < 20_000, "oversized TSDF extraction was not rejected");
    }
    assert!(rejected.finish().is_none());
}

#[test]
fn accepted_texture_bake_and_png_publication_steps_stay_below_hard_ceiling() {
    let mesh = make_uv_sphere(0.4, 12, 22);
    let mut unwrap = BoundedUnwrapPreparation::new(&mesh);
    while !unwrap.advance(&mesh, 64) {}
    let intrinsics = intrinsics_for(64, 64);
    let views = vec![TextureView { pose: look_at_pose([0.0, 0.0, -1.5], [0.0; 3]), intrinsics, image: checkerboard_image(64, 64, 8) }];
    let mut preparation = BoundedTexturePreparation::new(64);
    let mut steps = 0usize;
    loop {
        let started = std::time::Instant::now();
        let complete = preparation.advance(&mesh, &unwrap.uvs, &views, 64);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "texture bake step {steps} exceeded 8 ms");
        steps += 1;
        if complete {
            break;
        }
        assert!(steps < 200_000, "bounded texture preparation failed to terminate");
    }
    let texture = preparation.finish();
    let mut png = BoundedTexturePngPreparation::new(&texture);
    loop {
        let started = std::time::Instant::now();
        let complete = png.advance(&texture);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "texture PNG row exceeded 8 ms");
        if complete {
            break;
        }
    }
    let encoded = png.finish();
    let bytes = base64_codec::base64_standard_decode(encoded).expect("streamed texture PNG base64");
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
}
// #endregion 🔖️TextureTests

mod long {
    use super::*;

    #[test]
    fn full_pipeline_from_tsdf_sphere_is_watertight() {
        let voxel = 0.05;
        let radius = 0.6;
        let vol = build_tsdf_from_sdf(voxel, voxel * 3.0, radius, sphere_sdf(radius));
        let bound = ((radius + voxel * 4.0) / voxel).ceil() as i32;
        let params = MeshParams { target_triangles: 400, ..MeshParams::default() };
        let mut pipeline = MeshPipeline::new(&vol, 0.0, [-bound, -bound, -bound], [bound, bound, bound], params);
        let mut status = mesh_pipeline_step(&mut pipeline, 1);
        let mut guard = 0;
        while !matches!(status, MeshPipelineStatus::Done | MeshPipelineStatus::Failed(_)) {
            status = mesh_pipeline_step(&mut pipeline, 1);
            guard += 1;
            assert!(guard < 100, "pipeline did not terminate");
        }
        assert!(matches!(status, MeshPipelineStatus::Done), "pipeline status: {status:?}");
        let report = pipeline.report().expect("pipeline recorded a report");
        assert!(report.is_watertight, "full pipeline report: {report:?}");
        let mesh_data = pipeline.result().expect("pipeline produced mesh data");
        assert!(!mesh_data.positions.is_empty());
        assert!(!mesh_data.uvs.is_empty());
    }

    #[test]
    fn full_pipeline_with_views_bakes_and_encodes_texture() {
        let mesh = make_uv_sphere(0.4, 10, 14);
        let intr = intrinsics_for(48, 48);
        let views: Vec<TextureView> = orbit_views(1.5).into_iter().take(6).map(|pose| TextureView { pose, intrinsics: intr, image: checkerboard_image(48, 48, 6) }).collect();
        let params = MeshParams { target_triangles: 150, ..MeshParams::default() };
        let mut pipeline = MeshPipeline::from_mesh(mesh, params).with_views(views);
        let mut status = mesh_pipeline_step(&mut pipeline, 1);
        let mut guard = 0;
        while !matches!(status, MeshPipelineStatus::Done | MeshPipelineStatus::Failed(_)) {
            status = mesh_pipeline_step(&mut pipeline, 1);
            guard += 1;
            assert!(guard < 100, "pipeline did not terminate");
        }
        assert!(matches!(status, MeshPipelineStatus::Done), "pipeline status: {status:?}");
        let mesh_data = pipeline.result().expect("pipeline produced mesh data");
        assert!(mesh_data.paint_texture_base64.is_some(), "expected texture bake+encode to populate paint_texture_base64");
    }
}
