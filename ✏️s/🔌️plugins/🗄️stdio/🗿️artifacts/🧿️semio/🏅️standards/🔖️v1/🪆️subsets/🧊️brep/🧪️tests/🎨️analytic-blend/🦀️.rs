//! 🎨 Kernel-level contract for the ANALYTIC rolling-ball fillet and cutting-plane chamfer
//! (`🧬️schema/🔺️diff/🎨️blend`), from the crate's PUBLIC surface — the lane
//! `📓️boolean-kernel-2026-09-09.md` §5.6 left open, closed in
//! `📓️blend-kernel-2026-09-09.md`.
//!
//! Every case asserts the same five independent things about one blended box:
//!
//! 1. `validate_body` is silent over the WHOLE body — the surgery is non-destructive, so the input
//!    solid is still there and still valid too.
//! 2. `V − E + F = 2` over the result's own entities: the shell stayed a topological sphere across
//!    12 patch insertions and 8 corner insertions.
//! 3. The face count is exactly what the analytic construction predicts (26 for a fully blended
//!    box: 6 shrunken originals + 12 edge patches + 8 corner patches), which is what distinguishes
//!    a finished surgery from one that silently dropped a corner.
//! 4. The kernel's own `solid_volume` equals the closed form — derived here from Steiner's formula
//!    rather than reused from the kernel's tests, and each one algebraically re-stated as a
//!    subtraction from the sharp box so a sign error in either statement shows up.
//! 5. `parry3d`'s `trimesh_signed_volume_and_center_of_mass` over the TESSELLATED result agrees,
//!    positive — so no number here rests on our own arithmetic, and the mesh the viewer sees is
//!    the solid the kernel measured.
//!
//! Plus the interactive budget: a 12-edge fillet AND its tessellation must finish well inside a
//! frame's worth of work. Before this rewrite the same call spent minutes in
//! `Surface::project_curve` fitting p-curves onto NURBS blend patches.
//!
//! @see ../../🧬️schema/🔺️diff/🎨️blend/🦀️.rs — the patch construction under test.

// #region 🔖️Imports
use parry3d::mass_properties::details::trimesh_signed_volume_and_center_of_mass;
use parry3d::na::Point3;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::blend::{chamfer_edges, fillet_edges};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::MeshTransfer;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::mass_properties::{solid_signed_volume, solid_volume};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::arena::{EdgeId, SolidId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use std::collections::BTreeSet;
use std::f64::consts::PI;
use std::time::Instant;
// #endregion 🔖️Imports

// #region 🔖️Budget

/// ⏱️ Chord tolerance every tessellation here runs at — the viewer's own preview deflection.
const DEFLECTION: f64 = 1e-3;
/// 📏 Chord tolerance the mass-properties integrator runs at. A filleted or chamfered BOX has no
/// full-circle boundary, so its faces' `(u, v)` regions are straight-sided and the integrator
/// carries no polygonisation bias — the comparison below is against the exact closed form.
const MEASURE_TOLERANCE: f64 = 1e-5;
/// 🔮 How far `parry3d`'s `f32` integrator over a `1e-3` chord soup may sit from the closed form.
const ORACLE_TOLERANCE: f64 = 2e-3;
/// ⏱️ The interactive budget for a 12-edge fillet AND its tessellation, measured natively in a
/// debug build — which is the pessimistic end, the shipped build is optimised.
const BUDGET: std::time::Duration = std::time::Duration::from_millis(200);

// #endregion 🔖️Budget

// #region 🔖️Probes

/// 🧊 Every edge of `solid`, which for a fresh box is exactly its 12.
fn solid_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    let mut edges = BTreeSet::new();
    for face in body.solid_faces(solid) {
        for coedge in body.face_coedges(face) {
            edges.insert(body.coedges.get(coedge).expect("coedge").edge);
        }
    }
    edges.into_iter().collect()
}

/// 🔺 `V − E + F` over the solid's own entities — 2 for every sphere-like closed shell.
fn euler_characteristic(body: &Body, solid: SolidId) -> i64 {
    let faces = body.solid_faces(solid);
    let mut edges = BTreeSet::new();
    let mut vertices = BTreeSet::new();
    for &face in &faces {
        for coedge in body.face_coedges(face) {
            let entity = body.coedges.get(coedge).expect("coedge");
            edges.insert(entity.edge);
            let edge = body.edges.get(entity.edge).expect("edge");
            vertices.insert(edge.v0);
            vertices.insert(edge.v1);
        }
    }
    vertices.len() as i64 - edges.len() as i64 + faces.len() as i64
}

/// 🔮 `parry3d`'s own signed volume over a tessellated soup — the third-party half of every case.
fn parry_signed_volume(mesh: &MeshTransfer) -> f64 {
    let points: Vec<Point3<f32>> = mesh.position.chunks_exact(3).map(|p| Point3::new(p[0], p[1], p[2])).collect();
    let indices: Vec<[u32; 3]> = mesh.index.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
    assert!(!points.is_empty() && !indices.is_empty(), "tessellation produced an empty mesh");
    let (volume, _) = trimesh_signed_volume_and_center_of_mass(&points, &indices);
    f64::from(volume)
}

// #endregion 🔖️Probes

// #region 🔖️Assertions

/// ✅ Everything one finished blend must satisfy at once.
///
/// The `warning-` codes are excluded by the validator's own advisory contract, and a rounded solid
/// produces one legitimately: a corner patch meets each of its three planar neighbours at exactly
/// one point — their shared VERTEX — which the self-intersection probe's edge-adjacency test does
/// not recognise as adjacency.
fn assert_blend_contract(body: &Body, solid: SolidId, faces: usize, expected: f64, label: &str) {
    let issues: Vec<_> = validate_body(body).into_iter().filter(|issue| !issue.code.starts_with("warning-")).collect();
    assert!(issues.is_empty(), "{label}: validate_body reported {} error(s): {:?}", issues.len(), issues.iter().map(|issue| format!("{}:{}:{}", issue.entity, issue.code, issue.message)).collect::<Vec<_>>());
    assert_eq!(euler_characteristic(body, solid), 2, "{label}: the blended shell is no longer a topological sphere");
    assert_eq!(body.solid_faces(solid).len(), faces, "{label}: the surgery did not mint every patch");

    let signed = solid_signed_volume(body, solid, MEASURE_TOLERANCE).expect("kernel signed volume");
    assert!(signed > 0.0, "{label}: the blended shell is inward-oriented, signed volume {signed}");
    let ours = solid_volume(body, solid, MEASURE_TOLERANCE).expect("kernel volume");
    assert!((ours - expected).abs() <= 1e-6 * expected, "{label}: kernel volume {ours} vs closed form {expected}");

    let mesh = tessellate_solid(body, solid, DEFLECTION).expect("tessellate");
    let theirs = parry_signed_volume(&mesh);
    assert!(theirs > 0.0, "{label}: parry3d reads the tessellated soup as inward-wound, {theirs}");
    assert!((theirs - expected).abs() <= ORACLE_TOLERANCE * expected, "{label}: parry3d volume {theirs} vs closed form {expected}");
}

/// 🧊 The rounded box's volume, from Steiner's formula for the Minkowski sum of the INNER box
/// `(a−2r)(b−2r)(c−2r)` with a ball of radius `r`: the inner box, six slabs of thickness `r`,
/// twelve quarter-cylinders along its edges, and eight sphere octants at its corners.
fn rounded_box_volume(a: f64, b: f64, c: f64, r: f64) -> f64 {
    let (x, y, z) = (a - 2.0 * r, b - 2.0 * r, c - 2.0 * r);
    x * y * z + 2.0 * r * (x * y + y * z + z * x) + PI * r * r * (x + y + z) + (4.0 / 3.0) * PI * r * r * r
}

/// 🧊 The same number stated the other way round — the sharp box MINUS what the rolling ball takes
/// out of it: each of the twelve edges loses the `r²(1 − π/4)` corner sliver over the length its
/// two end corners left it (`a − 2r`), and each of the eight vertices loses the cube corner beyond
/// those slivers, `r³(1 − π/6)`.
///
/// Expanding `a³ = (a−2r)³ + 6r(a−2r)² + 12r²(a−2r) + 8r³` against [`rounded_box_volume`] turns
/// `12r²(a−2r) − 3πr²(a−2r)` into the edge term and `8r³ − (4/3)πr³` into the vertex term, so the
/// two statements are algebraically the same and a sign error in either fails this assertion.
fn rounded_cube_volume_by_subtraction(a: f64, r: f64) -> f64 {
    a * a * a - 12.0 * (1.0 - PI / 4.0) * r * r * (a - 2.0 * r) - 8.0 * (1.0 - PI / 6.0) * r * r * r
}

// #endregion 🔖️Assertions

// #region 🔖️Fillet

#[test]
fn fillet_of_all_twelve_box_edges_matches_the_rounded_box_closed_form() {
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let (a, b, c, r) = (2.6, 3.6, 2.1, 0.3);
    let solid = make_box(&mut body, a, b, c, &mut recorder).expect("box");
    let edges = solid_edges(&body, solid);
    assert_eq!(edges.len(), 12, "a box has twelve edges");
    let out = fillet_edges(&mut body, solid, &edges, r, &mut recorder).expect("twelve-edge fillet");
    assert_blend_contract(&body, out, 26, rounded_box_volume(a, b, c, r), "box fillet ×12");
}

#[test]
fn the_two_rounded_cube_closed_forms_agree_and_the_kernel_meets_both() {
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let (a, r) = (2.0, 0.25);
    let steiner = rounded_box_volume(a, a, a, r);
    let subtraction = rounded_cube_volume_by_subtraction(a, r);
    assert!((steiner - subtraction).abs() <= 1e-12 * steiner, "the two statements of the rounded cube disagree: {steiner} vs {subtraction}");
    let solid = make_box(&mut body, a, a, a, &mut recorder).expect("cube");
    let edges = solid_edges(&body, solid);
    let out = fillet_edges(&mut body, solid, &edges, r, &mut recorder).expect("twelve-edge fillet");
    assert_blend_contract(&body, out, 26, subtraction, "cube fillet ×12");
}

#[test]
fn fillet_of_one_box_edge_removes_exactly_its_corner_sliver() {
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let (a, b, c, r) = (2.0, 2.0, 2.0, 0.3);
    let solid = make_box(&mut body, a, b, c, &mut recorder).expect("box");
    let edge = solid_edges(&body, solid)[0];
    let length = {
        let entity = body.edges.get(edge).expect("edge");
        let (p, q) = (body.vertices.get(entity.v0).expect("v0").position, body.vertices.get(entity.v1).expect("v1").position);
        p.distance(q)
    };
    let out = fillet_edges(&mut body, solid, &[edge], r, &mut recorder).expect("single-edge fillet");
    // 🧊 A lone blended edge keeps its two end vertices trihedral and unblended, so the surgery
    // adds one cylinder patch and no corner patch: 7 faces, and the sliver runs the full edge.
    assert_blend_contract(&body, out, 7, a * b * c - length * r * r * (1.0 - PI / 4.0), "box fillet ×1");
}

#[test]
fn fillet_of_four_parallel_box_edges_matches_the_rounded_prism_closed_form() {
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let (a, b, c, r) = (2.4, 3.2, 1.8, 0.35);
    let solid = make_box(&mut body, a, b, c, &mut recorder).expect("box");
    // 🧊 The four edges parallel to `Z` — the classic "round the uprights" selection, and the
    // partial set whose eight end vertices each keep exactly one blended edge, so every one of them
    // takes the `cap`-face branch rather than a corner patch.
    let vertical: Vec<EdgeId> = solid_edges(&body, solid)
        .into_iter()
        .filter(|&edge| {
            let entity = body.edges.get(edge).expect("edge");
            let (p, q) = (body.vertices.get(entity.v0).expect("v0").position, body.vertices.get(entity.v1).expect("v1").position);
            (p.x - q.x).abs() < 1e-9 && (p.y - q.y).abs() < 1e-9
        })
        .collect();
    assert_eq!(vertical.len(), 4, "a box has four edges parallel to Z");
    let out = fillet_edges(&mut body, solid, &vertical, r, &mut recorder).expect("four-edge fillet");
    // 🧊 A prism on a rounded rectangle: the `(a−2r)×(b−2r)` core, two slabs, and one full disc of
    // radius `r` assembled from the four quarter-cylinders — all of height `c`.
    let section = (a - 2.0 * r) * (b - 2.0 * r) + 2.0 * r * ((a - 2.0 * r) + (b - 2.0 * r)) + PI * r * r;
    assert_blend_contract(&body, out, 10, section * c, "box fillet ×4 vertical");
}

// #endregion 🔖️Fillet

// #region 🔖️Chamfer

#[test]
fn chamfer_of_all_twelve_box_edges_matches_the_closed_form() {
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let (a, b, c, d) = (2.0, 3.0, 1.5, 0.25);
    let solid = make_box(&mut body, a, b, c, &mut recorder).expect("box");
    let edges = solid_edges(&body, solid);
    let out = chamfer_edges(&mut body, solid, &edges, d, d, &mut recorder).expect("twelve-edge chamfer");
    // 🔻 The same Steiner decomposition with the ball replaced by the CUBE `[−d, d]³`'s own
    // corner-cutting octahedron: twelve triangular prisms of section `d²/2` over the full edges,
    // corrected at the eight corners where three of them overlap.
    let closed_form = a * b * c - 2.0 * d * d * (a + b + c) + (16.0 / 3.0) * d * d * d;
    assert_blend_contract(&body, out, 26, closed_form, "box chamfer ×12");
}

#[test]
fn chamfer_of_one_box_edge_removes_exactly_its_wedge() {
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let (a, b, c, d0, d1) = (2.0, 2.0, 2.0, 0.2, 0.35);
    let solid = make_box(&mut body, a, b, c, &mut recorder).expect("box");
    let edge = solid_edges(&body, solid)[0];
    let length = {
        let entity = body.edges.get(edge).expect("edge");
        let (p, q) = (body.vertices.get(entity.v0).expect("v0").position, body.vertices.get(entity.v1).expect("v1").position);
        p.distance(q)
    };
    let out = chamfer_edges(&mut body, solid, &[edge], d0, d1, &mut recorder).expect("single-edge chamfer");
    assert_blend_contract(&body, out, 7, a * b * c - 0.5 * d0 * d1 * length, "box chamfer ×1");
}

// #endregion 🔖️Chamfer

// #region 🔖️Budget

/// ⏱️ The whole point of carrying every blend patch on an ANALYTIC support: no boundary of one
/// needs `Surface::project_curve`, so a twelve-edge fillet and its tessellation are a frame's work
/// rather than minutes of NURBS inversion. Measured over three runs (the first pays the arena's
/// own warm-up), each one held to the budget on its own.
#[test]
fn fillet_of_all_twelve_box_edges_stays_inside_the_interactive_budget() {
    for attempt in 0..3 {
        let mut body = Body::new();
        let mut recorder = OpRecorder::new();
        let solid = make_box(&mut body, 2.6, 3.6, 2.1, &mut recorder).expect("box");
        let edges = solid_edges(&body, solid);
        let started = Instant::now();
        let out = fillet_edges(&mut body, solid, &edges, 0.3, &mut recorder).expect("twelve-edge fillet");
        let filleted = started.elapsed();
        let mesh = tessellate_solid(&body, out, DEFLECTION).expect("tessellate");
        let total = started.elapsed();
        assert!(!mesh.index.is_empty(), "the filleted solid tessellated to nothing");
        eprintln!("⏱️ attempt {attempt}: fillet {filleted:?}, fillet + tessellation {total:?}, {} triangles", mesh.index.len() / 3);
        assert!(total <= BUDGET, "attempt {attempt}: a twelve-edge fillet plus tessellation took {total:?}, over the {BUDGET:?} interactive budget (fillet alone {filleted:?})");
    }
}

// #endregion 🔖️Budget
