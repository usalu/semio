use super::*;
use super::*;
use std::collections::HashSet;

fn shoelace_area(points: &[[f64; 2]]) -> f64 {
    let mut sum = 0.0;
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        sum += a[0] * b[1] - b[0] * a[1];
    }
    (sum * 0.5).abs()
}

fn tri_area(mesh: &TriMesh2, tri: &[u32; 3]) -> f64 {
    shoelace_area(&[mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]])
}

fn total_area(mesh: &TriMesh2) -> f64 {
    mesh.tris.iter().map(|t| tri_area(mesh, t)).sum()
}

fn no_refine() -> MeshOpts {
    MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }
}

fn square(side: f64) -> Vec<[f64; 2]> {
    vec![[0.0, 0.0], [side, 0.0], [side, side], [0.0, side]]
}

#[test]
fn owned_bowyer_watson_replays_domain_invariants() {
    let fixtures = [
        (PlanarDomain { outer: square(10.0), holes: vec![] }, 100.0),
        (PlanarDomain { outer: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [5.0, 5.0], [5.0, 10.0], [0.0, 10.0]], holes: vec![] }, 75.0),
        (PlanarDomain { outer: square(10.0), holes: vec![vec![[3.0, 3.0], [7.0, 3.0], [7.0, 7.0], [3.0, 7.0]]] }, 84.0),
    ];
    for (domain, expected_area) in fixtures {
        let first = owned_triangulate(&domain, &no_refine()).expect("owned triangulates");
        let second = owned_triangulate(&domain, &no_refine()).expect("owned replay triangulates");
        assert_eq!(first, second);
        assert!((total_area(&first) - expected_area).abs() < 1e-9);
        assert!(tri_mesh_quality(&first).min_jacobian_sign_positive);
    }
}

#[test]
fn refined_mesh_respects_min_angle() {
    let outer = vec![[0.0, 0.0], [20.0, 0.0], [20.0, 1.0], [0.0, 1.0]];
    let domain = PlanarDomain { outer, holes: vec![] };
    let opts = MeshOpts { max_edge: 1.0, min_angle_deg: 25.0 };
    let mesh = triangulate(&domain, &opts).expect("triangulates");
    let quality = tri_mesh_quality(&mesh);
    let epsilon = 2.0;
    assert!(quality.min_angle_deg >= opts.min_angle_deg - epsilon, "min_angle={}", quality.min_angle_deg);
}

#[test]
fn quad_grid_has_expected_topology() {
    let mesh = quad_grid(0.0, 0.0, 3.0, 2.0, 3, 2);
    assert_eq!(mesh.quads.len(), 6);
    assert_eq!(mesh.points.len(), 12);
    assert_eq!(mesh.points[0], [0.0, 0.0]);
    assert_eq!(mesh.points[3], [3.0, 0.0]);
    assert_eq!(mesh.points[11], [3.0, 2.0]);
    assert_eq!(mesh.quads[0], [0, 1, 5, 4]);
    assert_eq!(mesh.quads[5], [6, 7, 11, 10]);
}

#[test]
fn to_quadratic_welds_shared_edges() {
    let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
    assert!(mesh.tris.len() >= 2);

    let mut unique_edges: HashSet<(u32, u32)> = HashSet::new();
    for tri in &mesh.tris {
        for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            unique_edges.insert(if a < b { (a, b) } else { (b, a) });
        }
    }

    let quadratic = to_quadratic(&mesh);
    let new_points = quadratic.points.len() - mesh.points.len();
    assert_eq!(new_points, unique_edges.len());
}

#[test]
fn extrude_tri_mesh_volume_matches_area_times_height() {
    let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
    let area = total_area(&mesh);
    let height = 3.0;
    let volume_mesh = extrude_tri_mesh(&mesh, height, 2);
    let tets = split_to_tets(&volume_mesh);
    let total: f64 = tets.cells.iter().map(|c| cell_signed_volume(&tets.points, c).abs()).sum();
    assert!((total - area * height).abs() < 1e-6, "total={} expected={}", total, area * height);
}

#[test]
fn extrude_quad_mesh_volume_matches_area_times_height() {
    let mesh = quad_grid(0.0, 0.0, 4.0, 3.0, 4, 3);
    let area = 12.0;
    let height = 2.5;
    let volume_mesh = extrude_quad_mesh(&mesh, height, 3);
    let tets = split_to_tets(&volume_mesh);
    let total: f64 = tets.cells.iter().map(|c| cell_signed_volume(&tets.points, c).abs()).sum();
    assert!((total - area * height).abs() < 1e-6, "total={} expected={}", total, area * height);
}

#[test]
fn split_to_tets_preserves_volume() {
    let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
    let wedge_mesh = extrude_tri_mesh(&mesh, 2.0, 2);
    let pre_wedge: f64 = wedge_mesh.cells.iter().map(|c| cell_signed_volume(&wedge_mesh.points, c).abs()).sum();
    let post_wedge = split_to_tets(&wedge_mesh);
    let post_wedge_total: f64 = post_wedge.cells.iter().map(|c| cell_signed_volume(&post_wedge.points, c).abs()).sum();
    assert!((pre_wedge - post_wedge_total).abs() < 1e-9);

    let quad_mesh = quad_grid(0.0, 0.0, 4.0, 4.0, 2, 2);
    let hex_mesh = extrude_quad_mesh(&quad_mesh, 2.0, 2);
    let pre_hex: f64 = hex_mesh.cells.iter().map(|c| cell_signed_volume(&hex_mesh.points, c).abs()).sum();
    let post_hex = split_to_tets(&hex_mesh);
    let post_hex_total: f64 = post_hex.cells.iter().map(|c| cell_signed_volume(&post_hex.points, c).abs()).sum();
    assert!((pre_hex - post_hex_total).abs() < 1e-9);
}

#[test]
fn split_to_tets_shared_faces_are_parity_consistent() {
    let points = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
        [2.0, 0.0, 0.0],
        [2.0, 1.0, 0.0],
        [2.0, 0.0, 1.0],
        [2.0, 1.0, 1.0],
    ];
    let cell_a = Cell::Hex8([0, 1, 2, 3, 4, 5, 6, 7]);
    let cell_b = Cell::Hex8([1, 8, 9, 2, 5, 10, 11, 6]);
    let shared_face: HashSet<u32> = [1, 2, 6, 5].into_iter().collect();

    let mesh_a = VolumeMesh { points: points.clone(), cells: vec![cell_a] };
    let mesh_b = VolumeMesh { points: points.clone(), cells: vec![cell_b] };
    let tets_a = split_to_tets(&mesh_a);
    let tets_b = split_to_tets(&mesh_b);

    let face_triangles = |vm: &VolumeMesh| -> HashSet<[u32; 3]> {
        let mut out = HashSet::new();
        for cell in &vm.cells {
            if let Cell::Tet4(t) = cell {
                let faces = [[t[0], t[1], t[2]], [t[0], t[1], t[3]], [t[0], t[2], t[3]], [t[1], t[2], t[3]]];
                for mut f in faces {
                    if f.iter().all(|v| shared_face.contains(v)) {
                        f.sort_unstable();
                        out.insert(f);
                    }
                }
            }
        }
        out
    };

    let from_a = face_triangles(&tets_a);
    let from_b = face_triangles(&tets_b);
    assert_eq!(from_a.len(), 2, "expected the shared quad face split into 2 triangles from cell A");
    assert_eq!(from_a, from_b, "shared face must split identically from both cells");
}

#[test]
fn volume_mesh_quality_detects_inverted_cell() {
    let points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let good = VolumeMesh { points: points.clone(), cells: vec![Cell::Tet4([0, 1, 2, 3])] };
    assert!(volume_mesh_quality(&good).min_jacobian_sign_positive);

    let inverted = VolumeMesh { points, cells: vec![Cell::Tet4([1, 0, 2, 3])] };
    assert!(!volume_mesh_quality(&inverted).min_jacobian_sign_positive);
}

/// 🧱️ A `side`x`side` square extruded `height` tall, 1 layer, split to tets — `boundary_faces`'s
/// total triangle area must equal the analytic box surface `2*side² + 4*side*height` (top + bottom
/// + 4 sides), which also confirms every internal (shared, appears-twice) face was excluded.
#[test]
fn boundary_faces_area_matches_extruded_box_surface() {
    let side = 4.0;
    let height = 3.0;
    let domain = PlanarDomain { outer: square(side), holes: vec![] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
    let volume_mesh = extrude_tri_mesh(&mesh, height, 1);
    let tets = split_to_tets(&volume_mesh);

    let faces = boundary_faces(&tets);
    let tri_area = |f: &[u32; 3]| -> f64 {
        let (a, b, c) = (tets.points[f[0] as usize], tets.points[f[1] as usize], tets.points[f[2] as usize]);
        let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let cross = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
        0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
    };
    let total_area: f64 = faces.iter().map(tri_area).sum();
    let expected = 2.0 * side * side + 4.0 * side * height;
    assert!((total_area - expected).abs() < 1e-6, "total={total_area} expected={expected}");

    for f in &faces {
        if tets.points[f[0] as usize][2] < 1e-9 && tets.points[f[1] as usize][2] < 1e-9 && tets.points[f[2] as usize][2] < 1e-9 {
            let (a, b, c) = (tets.points[f[0] as usize], tets.points[f[1] as usize], tets.points[f[2] as usize]);
            let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let normal_z = e0[0] * e1[1] - e0[1] * e1[0];
            assert!(normal_z < 0.0, "bottom face normal should point outward (-z), got normal_z={normal_z}");
        }
    }
}

/// 📊️ `tri_mesh_quality` flags a clockwise-wound (negative signed area) triangle via
/// `min_jacobian_sign_positive`, and reports `0.0` angle bounds for an empty mesh instead of the
/// unhelpful `f64::INFINITY`/`NEG_INFINITY` an empty min/max fold would otherwise leave behind.
#[test]
fn tri_mesh_quality_detects_inverted_winding_and_handles_empty_mesh() {
    let ccw = TriMesh2 { points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], tris: vec![[0, 1, 2]] };
    assert!(tri_mesh_quality(&ccw).min_jacobian_sign_positive);

    let cw = TriMesh2 { points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], tris: vec![[0, 2, 1]] };
    assert!(!tri_mesh_quality(&cw).min_jacobian_sign_positive);

    let empty = TriMesh2 { points: vec![], tris: vec![] };
    let quality = tri_mesh_quality(&empty);
    assert_eq!(quality.min_angle_deg, 0.0);
    assert_eq!(quality.max_angle_deg, 0.0);
    assert_eq!(quality.element_count, 0);
}

/// 🔺️ A `Cell::Tet4` already present in the input `VolumeMesh` passes through `split_to_tets`
/// completely unchanged — the only cell kind besides `Wedge6`/`Hex8` `split_to_tets` accepts.
#[test]
fn split_to_tets_passes_through_existing_tet4_cells() {
    let points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let tet = Cell::Tet4([0, 1, 2, 3]);
    let mesh = VolumeMesh { points, cells: vec![tet] };
    let result = split_to_tets(&mesh);
    assert_eq!(result.cells.len(), 1);
    match result.cells[0] {
        Cell::Tet4(nodes) => assert_eq!(nodes, [0, 1, 2, 3]),
        _ => panic!("expected the Tet4 cell to pass through unchanged"),
    }
}
