//! 🧲 Kernel-level regression for the two BOOLEAN procedural-3d examples — the only two of the
//! eight `generation3d` example graphs whose op chain reaches `diff::boolean`:
//!
//! * `🧲️sphere-box-fuse` — `brep.prim3d.sphere(radius = 1.2) + brep.prim3d.box(1.5³)` →
//!   `brep.bool.fuse`. The flow-brep extension maps those neuron kinds onto `sphere_prim`/
//!   `box_prim`/`fuse`, which are `make_sphere`/`make_box`/`boolean_solid(Unite)` verbatim, so
//!   this file drives the same code path the playground does, one layer below the graph.
//! * `🍩️sphere-cut-with-torus` — `brep.prim3d.sphere(radius = 2.2) + brep.prim3d.torus(major =
//!   2.0, minor = 0.5)` → `brep.bool.cut` → `brep.measure.volume` (the extension's own declared
//!   torus channel defaults are `major = 2.0`, `minor = 0.5`; the example wires only the sphere's
//!   radius).
//!
//! Both cases assert the FOUR properties a boolean result must have, not just "it returned":
//! closed + 2-manifold + consistently oriented (`validate_body` reports nothing), Euler
//! characteristic χ = 2 for the genus-0 result, a positive volume matching an INDEPENDENT
//! oracle, and — as the third-party cross-check the standard requires — the same volume measured
//! by `parry3d`'s own `trimesh_signed_volume_and_center_of_mass` over the tessellated result.
//! `parry3d` is a `[dev-dependencies]`-only oracle here (same role and the same crate version it
//! already plays for `semio-framework-3d`'s collision module) and is never reachable from any
//! production target.
//!
//! The expected volumes are derived here, never read back from the kernel:
//! * fuse — closed form. The box occupies `[0, a]³` and the sphere is centred on the box's own
//!   corner, so with `r < a` the box contains exactly one octant of the sphere:
//!   `V = V_sphere + a³ − V_sphere / 8`.
//! * cut — an independent axisymmetric quadrature ([`torus_inside_sphere_volume`]) over the
//!   analytic implicit definitions of both primitives, with the radial extent integrated in
//!   closed form per `z` slice so only one dimension is ever discretised.

// #region 🔖️Imports
use parry3d::mass_properties::details::trimesh_signed_volume_and_center_of_mass;
use parry3d::na::Point3;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::boolean::{boolean_solid, BooleanOp};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_sphere, make_torus};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::MeshTransfer;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::arena::{EdgeId, SolidId, VertexId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use std::collections::HashSet;
use std::f64::consts::PI;
// #endregion 🔖️Imports

// #region 🔖️Oracles

/// 🔮 Independent volume of the region a torus and a concentric sphere share, by axisymmetric
/// quadrature: for each height `z` the tube occupies the radial band
/// `[major − √(minor² − z²), major + √(minor² − z²)]`, the sphere caps it at `√(r² − z²)`, and
/// `∫ ρ dρ` over that band is closed form — so only `z` is discretised. Written from the two
/// primitives' own implicit definitions; it never calls the kernel.
fn torus_inside_sphere_volume(sphere_radius: f64, major: f64, minor: f64) -> f64 {
    const SLICES: usize = 2_000_000;
    let dz = 2.0 * minor / SLICES as f64;
    let mut moment = 0.0;
    for i in 0..SLICES {
        let z = -minor + (i as f64 + 0.5) * dz;
        let half_band = (minor * minor - z * z).max(0.0).sqrt();
        let lo = major - half_band;
        let hi = (major + half_band).min((sphere_radius * sphere_radius - z * z).max(0.0).sqrt());
        if hi > lo {
            moment += 0.5 * (hi * hi - lo * lo) * dz;
        }
    }
    2.0 * PI * moment
}

/// 🔮 `parry3d`'s own signed volume of a tessellated solid — the third-party cross-check. The
/// SIGN matters as much as the magnitude: a shell whose faces are wound inward integrates to a
/// negative volume, and an unclosed one to a value that matches nothing.
fn parry_signed_volume(mesh: &MeshTransfer) -> f64 {
    let points: Vec<Point3<f32>> = mesh.position.chunks_exact(3).map(|p| Point3::new(p[0], p[1], p[2])).collect();
    let indices: Vec<[u32; 3]> = mesh.index.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
    assert!(!points.is_empty() && !indices.is_empty(), "tessellation produced an empty mesh");
    let (volume, _) = trimesh_signed_volume_and_center_of_mass(&points, &indices);
    f64::from(volume)
}

// #endregion 🔖️Oracles

// #region 🔖️TopologyProbes

/// 🧮 `true` for a POINT edge — a collapsed iso-line closer (a sphere's poles): one vertex, and a
/// 3D curve whose whole range evaluates to that one position. Such an edge bounds no surface
/// strip, so it is excluded from the Euler count exactly as it is excluded from the shell-closure
/// rule.
fn is_point_edge(body: &Body, edge_id: EdgeId) -> bool {
    let Some(edge) = body.edges.get(edge_id) else { return false };
    if edge.v0 != edge.v1 {
        return false;
    }
    let Some(curve) = body.curves3.get(edge.curve) else { return false };
    let anchor = curve.eval(edge.range.0);
    (0..=8).all(|i| curve.eval(edge.range.0 + (edge.range.1 - edge.range.0) * f64::from(i) / 8.0).distance(anchor) <= edge.tol.value())
}

/// 🧮 `V − E + F` over `solid`'s own faces, skipping point edges (and any vertex only they use).
fn euler_characteristic(body: &Body, solid: SolidId) -> i64 {
    let faces = body.solid_faces(solid);
    let mut edges: HashSet<EdgeId> = HashSet::new();
    for &face in &faces {
        for coedge_id in body.face_coedges(face) {
            if let Some(coedge) = body.coedges.get(coedge_id) {
                if !is_point_edge(body, coedge.edge) {
                    edges.insert(coedge.edge);
                }
            }
        }
    }
    let mut vertices: HashSet<VertexId> = HashSet::new();
    for &edge_id in &edges {
        if let Some(edge) = body.edges.get(edge_id) {
            vertices.insert(edge.v0);
            vertices.insert(edge.v1);
        }
    }
    vertices.len() as i64 - edges.len() as i64 + faces.len() as i64
}

/// 🧮 Asserts the whole health contract for one boolean result: no validation issue anywhere in
/// the body, χ = 2 (a genus-0 closed surface), our own volume within `rel` of `expected`, and
/// `parry3d`'s independent signed volume agreeing — positive, so the shell is outward-oriented.
fn assert_boolean_result(body: &Body, solid: SolidId, expected: f64, rel: f64, label: &str) {
    let issues = validate_body(body);
    assert!(issues.is_empty(), "{label}: validate_body reported {} issue(s): {:?}", issues.len(), issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
    assert_eq!(euler_characteristic(body, solid), 2, "{label}: expected a genus-0 closed shell (χ = 2)");
    let ours = solid_volume(body, solid, 1e-4).unwrap_or_else(|error| panic!("{label}: solid_volume failed: {error}"));
    assert!((ours - expected).abs() <= rel * expected, "{label}: kernel volume {ours}, oracle {expected}");
    let mesh = tessellate_solid(body, solid, 5e-3).unwrap_or_else(|error| panic!("{label}: tessellate failed: {error}"));
    let theirs = parry_signed_volume(&mesh);
    assert!(theirs > 0.0, "{label}: parry3d signed volume {theirs} is not positive — the tessellated shell is inverted or open");
    assert!((theirs - expected).abs() <= 2e-2 * expected, "{label}: parry3d volume {theirs}, oracle {expected}");
}

// #endregion 🔖️TopologyProbes

// #region 🔖️Examples

#[test]
fn sphere_box_fuse_example_is_a_closed_oriented_solid() {
    let (radius, size) = (1.2_f64, 1.5_f64);
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let sphere = make_sphere(&mut body, radius, &mut rec).expect("sphere");
    let cube = make_box(&mut body, size, size, size, &mut rec).expect("box");
    let fused = boolean_solid(&mut body, sphere, cube, BooleanOp::Unite, 1e-6, &mut rec).expect("fuse");
    let sphere_volume = 4.0 / 3.0 * PI * radius.powi(3);
    let expected = sphere_volume + size.powi(3) - sphere_volume / 8.0;
    assert_boolean_result(&body, fused, expected, 5e-3, "sphere-box-fuse");
}

#[test]
fn sphere_cut_with_torus_example_is_a_closed_oriented_solid() {
    let (radius, major, minor) = (2.2_f64, 2.0_f64, 0.5_f64);
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let sphere = make_sphere(&mut body, radius, &mut rec).expect("sphere");
    let torus = make_torus(&mut body, major, minor, &mut rec).expect("torus");
    let carved = boolean_solid(&mut body, sphere, torus, BooleanOp::Cut, 1e-6, &mut rec).expect("cut");
    let expected = 4.0 / 3.0 * PI * radius.powi(3) - torus_inside_sphere_volume(radius, major, minor);
    assert_boolean_result(&body, carved, expected, 5e-3, "sphere-cut-with-torus");
}

// #endregion 🔖️Examples
