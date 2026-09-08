
use super::*;
use semio_framework_job::{Generation, OperationId, RevisionId, StepBudget, root_cancel_token};
use std::collections::HashSet;
use std::time::{Duration, Instant};

fn mesh_operation() -> Operation {
    Operation::new(OperationId(700), RevisionId(11), Generation(3), 19)
}

fn take_payload_bytes(mut payload: RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::new();
    for page in 0..payload.page_count() {
        bytes.extend_from_slice(payload.page(page).expect("retained mesh payload page"));
    }
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, usize::MAX);
    }
    bytes
}

fn drive_mesh_job(mut job: MeshJob) -> (Vec<u8>, usize, Duration) {
    fn now() -> Option<u64> {
        Some(0)
    }
    let cancel = root_cancel_token();
    let mut sequence = 0;
    let mut previews = 0;
    let mut worst = Duration::ZERO;
    for _ in 0..10_000_000 {
        let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(64, 10), cancel.clone(), now, &mut sequence);
        let started = Instant::now();
        let outcome = job.step(&mut context);
        let elapsed = started.elapsed();
        worst = worst.max(elapsed);
        match outcome {
            StepOutcome::PreviewReady(preview) => {
                previews += 1;
                take_payload_bytes(preview);
            }
            StepOutcome::Complete(candidate) => {
                take_payload_bytes(candidate.state);
                return (take_payload_bytes(candidate.output), previews, worst);
            }
            StepOutcome::CheckpointReady(checkpoint) => {
                take_payload_bytes(checkpoint.state);
            }
            StepOutcome::Yield => {}
            other => panic!("mesh job failed: {other:?}"),
        }
    }
    panic!("mesh job did not complete")
}

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

fn square(side: f64) -> Vec<[f64; 2]> {
    vec![[0.0, 0.0], [side, 0.0], [side, side], [0.0, side]]
}

#[test]
fn triangulate_square_area_matches_input() {
    let outer = square(10.0);
    let expected = shoelace_area(&outer);
    let domain = PlanarDomain { outer, holes: vec![] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
    assert!(!mesh.tris.is_empty());
    assert!((total_area(&mesh) - expected).abs() < 1e-9);
}

#[test]
fn triangulate_respects_hole_area() {
    let outer = square(10.0);
    let hole = vec![[3.0, 3.0], [7.0, 3.0], [7.0, 7.0], [3.0, 7.0]];
    let domain = PlanarDomain { outer, holes: vec![hole.clone()] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
    let expected = 100.0 - 16.0;
    assert!((total_area(&mesh) - expected).abs() < 1e-6, "area={}", total_area(&mesh));
    for tri in &mesh.tris {
        let p0 = mesh.points[tri[0] as usize];
        let p1 = mesh.points[tri[1] as usize];
        let p2 = mesh.points[tri[2] as usize];
        let centroid = [(p0[0] + p1[0] + p2[0]) / 3.0, (p0[1] + p1[1] + p2[1]) / 3.0];
        assert!(!point_in_polygon(centroid, &hole));
    }
}

#[test]
fn triangulate_honors_constrained_boundary_edges() {
    // L-shape: non-convex outer boundary.
    let outer = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [5.0, 5.0], [5.0, 10.0], [0.0, 10.0]];
    let domain = PlanarDomain { outer: outer.clone(), holes: vec![] };
    let mesh = triangulate(&domain, &no_refine()).expect("triangulates");

    let key = |p: [f64; 2]| (p[0].to_bits(), p[1].to_bits());
    let mut edge_set: HashSet<((u64, u64), (u64, u64))> = HashSet::new();
    for tri in &mesh.tris {
        let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
        for i in 0..3 {
            let a = key(p[i]);
            let b = key(p[(i + 1) % 3]);
            let edge = if a <= b { (a, b) } else { (b, a) };
            edge_set.insert(edge);
        }
    }

    for i in 0..outer.len() {
        let a = key(outer[i]);
        let b = key(outer[(i + 1) % outer.len()]);
        let edge = if a <= b { (a, b) } else { (b, a) };
        assert!(edge_set.contains(&edge), "boundary edge {i} missing from triangulation");
    }
}

#[test]
fn refined_mesh_respects_min_angle() {
    // A long thin rectangle: all input corners are 90 degrees, while an unrefined diagonal would
    // produce slivers. Deterministic boundary subdivision plus the interior lattice removes them.
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
    // Two Hex8 cells sharing the quad face [1,2,6,5] (cell A's +x face / cell B's -x face).
    let points = vec![
        [0.0, 0.0, 0.0], // 0
        [1.0, 0.0, 0.0], // 1
        [1.0, 1.0, 0.0], // 2
        [0.0, 1.0, 0.0], // 3
        [0.0, 0.0, 1.0], // 4
        [1.0, 0.0, 1.0], // 5
        [1.0, 1.0, 1.0], // 6
        [0.0, 1.0, 1.0], // 7
        [2.0, 0.0, 0.0], // 8
        [2.0, 1.0, 0.0], // 9
        [2.0, 0.0, 1.0], // 10
        [2.0, 1.0, 1.0], // 11
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

    // Swap two nodes to invert the signed volume.
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

    // Every boundary face must be wound so its normal points away from its own tet's centroid —
    // spot-checked here on the bottom face (z=0, outward normal must have negative z).
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

#[test]
fn triangulate_rejects_degenerate_outer_boundary() {
    let domain = PlanarDomain { outer: vec![[0.0, 0.0], [1.0, 0.0]], holes: vec![] };
    match triangulate(&domain, &no_refine()) {
        Err(MeshError::DegenerateDomain) => {}
        other => panic!("expected DegenerateDomain, got {other:?}"),
    }
}

#[test]
fn triangulate_rejects_degenerate_hole() {
    let domain = PlanarDomain { outer: square(10.0), holes: vec![vec![[3.0, 3.0], [4.0, 4.0]]] };
    match triangulate(&domain, &no_refine()) {
        Err(MeshError::DegenerateDomain) => {}
        other => panic!("expected DegenerateDomain, got {other:?}"),
    }
}

#[test]
fn point_in_polygon_returns_false_for_degenerate_polygon() {
    assert!(!point_in_polygon([0.0, 0.0], &[]));
    assert!(!point_in_polygon([0.0, 0.0], &[[0.0, 0.0], [1.0, 0.0]]));
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

#[test]
fn mesh_job_is_previewing_deterministic_and_step_bounded() {
    let domain = PlanarDomain { outer: square(8.0), holes: vec![square(2.0).into_iter().map(|point| [point[0] + 3.0, point[1] + 3.0]).collect()] };
    let options = MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 };
    let first = drive_mesh_job(MeshJob::new(domain.clone(), options, mesh_operation()));
    let second = drive_mesh_job(MeshJob::new(domain, options, mesh_operation()));
    assert_eq!(first.0, second.0);
    assert!(first.1 > 0);
    assert!(first.2 < Duration::from_millis(8), "worst mesh job step was {:?}", first.2);
}

#[test]
fn mesh_job_observes_cancellation_before_mutating() {
    fn now() -> Option<u64> {
        Some(0)
    }
    let operation = mesh_operation();
    let mut job = MeshJob::new(PlanarDomain { outer: square(2.0), holes: vec![] }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation);
    let cancel = root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(64, 10), cancel, now, &mut sequence);
    assert_eq!(job.step(&mut context), StepOutcome::Cancelled);
    assert_eq!(job.stage, MeshJobStage::Validate);
    assert!(job.triangulation.is_none());
}

#[test]
fn mesh_job_large_boundary_never_runs_to_completion_in_one_step() {
    let boundary = (0..1_024)
        .map(|index| {
            let angle = index as f64 * std::f64::consts::TAU / 1_024.0;
            [angle.cos() * 100.0, angle.sin() * 100.0]
        })
        .collect();
    let (_, previews, worst) = drive_mesh_job(MeshJob::new(PlanarDomain { outer: boundary, holes: vec![] }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, mesh_operation()));
    assert!(previews > 0);
    assert!(worst < Duration::from_millis(8), "worst large-boundary mesh job step was {worst:?}");
}

#[test]
fn bounded_mesh_plus_one_fault_retains_the_exact_domain_for_cursor_close() {
    let operation = mesh_operation();
    let domain = PlanarDomain { outer: (0..65).map(|index| [index as f64, 0.0]).collect(), holes: Vec::new() };
    let mut job = MeshJob::new_bounded(domain, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation, 64, 128);
    let mut sequence = 0;
    let mut faulted = false;
    for _ in 0..256 {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        if matches!(job.step(&mut context), StepOutcome::Fault(_)) {
            faulted = true;
            break;
        }
    }
    assert!(faulted);
    assert_eq!(job.domain.outer().len(), 65, "fault retains every rejected point owner");
    let (terminal, items, _) = job.close_step(4_096);
    assert!(!terminal);
    assert_eq!(items, 1);
    assert_eq!(job.domain.outer().len(), 64, "one close grant releases one point owner");
}

#[test]
fn mesh_mounted_classification_indexes_admit_maximum_reject_plus_one_and_close_exactly() {
    let operation = Operation::new(OperationId(41), RevisionId(43), Generation(47), 53);
    let mut job = MeshJob::new_bounded(PlanarDomain { outer: square(1.0), holes: Vec::new() }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation, 8, 2);
    let edge_capacity = job.maximum_triangles * 12 + 3;
    job.indexed_edges.try_reserve_exact(edge_capacity).expect("fixed edge index backing");
    assert!(job.indexed_edges.capacity() * size_of::<Edge>() <= 4_096);
    job.indexed_edges.extend((0..edge_capacity).map(|index| Edge(index, index + 1)));
    let before = (job.indexed_edges.as_ptr(), job.indexed_edges.len(), job.indexed_edges.capacity());
    assert_eq!(job.indexed_edges.binary_search(&Edge(edge_capacity + 1, edge_capacity + 2)), Err(edge_capacity));
    assert_eq!((job.indexed_edges.as_ptr(), job.indexed_edges.len(), job.indexed_edges.capacity()), before, "plus-one preflight returns the exact index authority without insertion");
    job.stage = MeshJobStage::Complete;
    job.close_lane = 7;
    assert!(!job.close_step(4_096).0, "one grant retires one exact indexed edge");
    assert_eq!(job.indexed_edges.len(), edge_capacity - 1);
}

#[test]
fn p6h_constraint_flip_interrupts_after_every_edge_phase_and_updates_only_affected_adjacencies() {
    let operation = mesh_operation();
    let points = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let triangulation = OwnedTriangulation {
        points: points.clone(),
        triangles: vec![[0, 1, 2], [0, 2, 3]],
        input_len: 4,
        insert_cursor: 4,
        insertion_order: vec![0, 1, 2, 3],
        insertion: None,
        maximum_triangles: 4,
        allocation_fault: false,
        mounted_initialization: MountedTriangulationInitialization { stage: MountedTriangulationStage::Complete, cursor: 0, sort_outer: 4, sort_inner: 4, bounds: [0.0, 1.0, 0.0, 1.0], center: [0.5, 0.5], span: 1.0 },
        finish: TriangulationFinishCursor { stage: TriangulationFinishStage::Complete, read: 2, write: 2, sort_outer: 2, sort_inner: 2 },
    };
    let mut job = MeshJob::new_bounded(PlanarDomain { outer: points, holes: Vec::new() }, no_refine(), operation, 4, 4);
    job.triangulation = Some(triangulation);
    job.constraints = vec![Edge::new(1, 3)];
    job.stage = MeshJobStage::ConstrainBoundary;
    job.indexed_constraint_edges.try_reserve_exact(12).expect("fixed edge authority");
    for triangle in 0..2 {
        for local in 0..3 {
            assert!(job.begin_edge_index_candidate(triangle, local));
            while !job.advance_edge_index_candidate().expect("index edge") {}
        }
    }
    let mut seen = HashSet::new();
    let mut sequence = 0;
    let mut maximum_micros = 0;
    for _ in 0..512 {
        let stage = job.constraint_stage;
        seen.insert(stage);
        let before = job.triangulation.as_ref().expect("triangulation retained").triangles.clone();
        let before_cursor = (job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor);
        let mut deadline = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), root_cancel_token(), || Some(0), &mut sequence);
        assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
        assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);
        assert_eq!(job.triangulation.as_ref().expect("triangulation retained").triangles, before);

        let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
        assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);

        let token = root_cancel_token();
        semio_framework_async::block_on(token.cancel());
        let mut cancelled = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || Some(0), &mut sequence);
        assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);

        let started = Instant::now();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        assert_eq!(job.step(&mut context), StepOutcome::Yield);
        maximum_micros = maximum_micros.max(started.elapsed().as_micros());
        if job.constraint_cursor == job.constraints.len() {
            break;
        }
    }
    for stage in [
        ConstraintRecoveryStage::ReserveConstraintWorkspace,
        ConstraintRecoveryStage::IndexTriangleEdge,
        ConstraintRecoveryStage::SearchConstraintEdge,
        ConstraintRecoveryStage::ClassifyIntersection,
        ConstraintRecoveryStage::SelectDeterministicFlip,
        ConstraintRecoveryStage::ValidateFlip,
        ConstraintRecoveryStage::ApplyFlip,
        ConstraintRecoveryStage::RetireFormerEdge,
        ConstraintRecoveryStage::PublishConstraintProgress,
        ConstraintRecoveryStage::ConstraintComplete,
    ] {
        assert!(seen.contains(&stage), "missing interrupted phase {stage:?}");
    }
    assert!(job.indexed_constraint_edges.iter().any(|slot| slot.active && slot.edge == Edge::new(1, 3)));
    assert!(!job.indexed_constraint_edges.iter().any(|slot| slot.active && slot.edge == Edge::new(0, 2)));
    assert!(maximum_micros < 8_000, "constraint recovery step exceeded timing ceiling: {maximum_micros} us");

    let mut close_turns = 0;
    loop {
        close_turns += 1;
        let (terminal, released_items, _) = job.close_step(usize::MAX);
        assert!(released_items <= 1);
        if terminal {
            break;
        }
        assert!(close_turns < 20_000);
    }
    assert!(job.close_lane > 11);
}

/// 📦️ Mounted preparation, construction, finish, and payload cursors interrupt and replay exactly.
#[test]
fn p6h_mounted_mesh_preparation_initialization_finish_publication_interrupt_replay_timing_and_close() {
    fn domain() -> PlanarDomain {
        PlanarDomain { outer: square(2.0), holes: vec![square(0.5).into_iter().map(|point| [point[0] + 0.75, point[1] + 0.75]).collect()] }
    }

    fn snapshot(job: &MeshJob) -> impl PartialEq + std::fmt::Debug {
        (
            job.stage,
            job.preparation.as_ref().map(|cursor| {
                (
                    (cursor.polygon, cursor.edge, cursor.segment, cursor.point_lookup_cursor, cursor.grid_row, cursor.grid_column, cursor.grid_polygon),
                    (cursor.grid_edge, cursor.pending_point.is_some(), cursor.pending_index, cursor.grid_candidate.is_some(), cursor.points.len(), cursor.constraints.len()),
                )
            }),
            job.triangulation.as_ref().map(|cursor| {
                (
                    cursor.mounted_initialization.stage,
                    cursor.mounted_initialization.cursor,
                    cursor.mounted_initialization.sort_outer,
                    cursor.mounted_initialization.sort_inner,
                    cursor.finish.stage,
                    cursor.finish.read,
                    cursor.finish.write,
                    cursor.finish.sort_outer,
                    cursor.finish.sort_inner,
                    cursor.insert_cursor,
                    cursor.points.len(),
                    cursor.triangles.len(),
                )
            }),
            job.publication_kind,
            job.publication_cursor.stage,
            job.publication_cursor.point,
            job.publication_cursor.coordinate,
            job.publication_cursor.triangle,
            job.publication_cursor.index,
            job.publication_writer.as_ref().and_then(RetainedJobPayloadWriter::staged_page_len),
            job.mesh.points.len(),
            job.mesh.tris.len(),
        )
    }

    fn run(operation: Operation) -> (Vec<u8>, u128) {
        let mut job = MeshJob::new_bounded(domain(), MeshOpts { max_edge: 2.0, min_angle_deg: 0.0 }, operation, 128, 20);
        let mut sequence = 0;
        let mut maximum_micros = 0;
        let mut initialization_seen = HashSet::new();
        let mut finish_seen = HashSet::new();
        let mut payload_seen = HashSet::new();
        let mut preparation_lookup_seen = false;
        let mut preparation_polygon_seen = false;
        for _ in 0..1_000_000 {
            if let Some(preparation) = job.preparation.as_ref() {
                preparation_lookup_seen |= preparation.point_lookup_cursor > 0;
                preparation_polygon_seen |= preparation.grid_candidate.is_some() && preparation.grid_edge > 0;
            }
            if let Some(triangulation) = job.triangulation.as_ref() {
                initialization_seen.insert(triangulation.mounted_initialization.stage);
                finish_seen.insert(triangulation.finish.stage);
            }
            if job.publication_kind.is_some() {
                payload_seen.insert(job.publication_cursor.stage);
            }
            if matches!(job.stage, MeshJobStage::PrepareInput | MeshJobStage::Initialize | MeshJobStage::InsertBoundary | MeshJobStage::PublishPreview | MeshJobStage::PublishCheckpoint | MeshJobStage::Complete) {
                let before = snapshot(&job);
                let mut deadline = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), root_cancel_token(), || Some(0), &mut sequence);
                assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
                assert_eq!(snapshot(&job), before);
                let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
                assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
                assert_eq!(snapshot(&job), before);
                let token = root_cancel_token();
                semio_framework_async::block_on(token.cancel());
                let mut cancelled = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || Some(0), &mut sequence);
                assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
                assert_eq!(snapshot(&job), before);
            }
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
            let started = Instant::now();
            let outcome = job.step(&mut context);
            maximum_micros = maximum_micros.max(started.elapsed().as_micros());
            match outcome {
                StepOutcome::PreviewReady(preview) => {
                    take_payload_bytes(preview);
                }
                StepOutcome::CheckpointReady(checkpoint) => {
                    take_payload_bytes(checkpoint.state);
                }
                StepOutcome::Complete(candidate) => {
                    take_payload_bytes(candidate.state);
                    for stage in [
                        MountedTriangulationStage::BoundsPoint,
                        MountedTriangulationStage::ValidateBounds,
                        MountedTriangulationStage::ReserveInsertionOrder,
                        MountedTriangulationStage::BuildInsertionOrder,
                        MountedTriangulationStage::OrderInsertion,
                        MountedTriangulationStage::ReserveSuperPoints,
                        MountedTriangulationStage::AppendSuperPoint,
                        MountedTriangulationStage::ReserveTriangles,
                        MountedTriangulationStage::SeedTriangle,
                        MountedTriangulationStage::Complete,
                    ] {
                        assert!(initialization_seen.contains(&stage), "missing mounted initialization stage {stage:?}");
                    }
                    for stage in [TriangulationFinishStage::Filter, TriangulationFinishStage::TruncateTriangles, TriangulationFinishStage::OrderTriangles, TriangulationFinishStage::TruncatePoints, TriangulationFinishStage::Complete] {
                        assert!(finish_seen.contains(&stage), "missing finish stage {stage:?}");
                    }
                    for stage in [
                        MeshPayloadStage::Magic,
                        MeshPayloadStage::Tier,
                        MeshPayloadStage::Complete,
                        MeshPayloadStage::Sequence,
                        MeshPayloadStage::Refinement,
                        MeshPayloadStage::PointCount,
                        MeshPayloadStage::TriangleCount,
                        MeshPayloadStage::PointCoordinate,
                        MeshPayloadStage::TriangleIndex,
                        MeshPayloadStage::CommitPage,
                    ] {
                        assert!(payload_seen.contains(&stage), "missing payload stage {stage:?}");
                    }
                    assert!(preparation_lookup_seen && preparation_polygon_seen);
                    return (take_payload_bytes(candidate.output), maximum_micros);
                }
                StepOutcome::Yield => {}
                outcome => panic!("mounted mesh cursor law failed: {outcome:?}"),
            }
        }
        panic!("mounted mesh cursor law did not complete")
    }

    let operation = mesh_operation();
    let first = run(operation);
    let second = run(operation);
    assert_eq!(first.0, second.0);
    assert!(first.1.max(second.1) < 8_000);

    let mut interrupted = MeshJob::new_bounded(domain(), MeshOpts { max_edge: 2.0, min_angle_deg: 0.0 }, operation, 128, 20);
    let mut sequence = 0;
    for _ in 0..1_000_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        match interrupted.step(&mut context) {
            StepOutcome::PreviewReady(preview) => {
                take_payload_bytes(preview);
            }
            StepOutcome::CheckpointReady(checkpoint) => {
                take_payload_bytes(checkpoint.state);
            }
            StepOutcome::Fault(fault) => panic!("interrupted mesh fixture fault: {:?}", fault.detail),
            _ => {}
        }
        if interrupted.publication_writer.as_ref().and_then(RetainedJobPayloadWriter::staged_page_len).is_some_and(|length| length > 0) {
            break;
        }
    }
    assert!(interrupted.publication_writer.is_some());
    InteractiveJob::begin_close(&mut interrupted);
    assert_eq!(InteractiveJob::close_step(&mut interrupted, 1, 0), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    for _ in 0..100_000 {
        match InteractiveJob::close_step(&mut interrupted, 1, usize::MAX) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
            semio_framework_job::InteractiveJobCloseStep::Complete => break,
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("mounted mesh close cannot block"),
        }
    }
    assert!(InteractiveJob::terminal_is_empty(&interrupted));
}
