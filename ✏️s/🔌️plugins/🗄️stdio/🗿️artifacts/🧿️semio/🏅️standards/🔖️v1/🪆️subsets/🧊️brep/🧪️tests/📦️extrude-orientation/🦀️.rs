//! 📦 Kernel-level regression for `➡️sweep`'s prism orientation — the defect
//! `📓️example-geometry-tests-2026-09-09.md` §7.2 measured through the real DSL (a watertight
//! extrusion whose four lateral faces all pointed INTO the solid, so its tessellated soup
//! integrated to `−V/3` while the B-Rep's own `measure.volume` read `+V`).
//!
//! The matrix is the one that makes the bug impossible to re-introduce by compensating a sign
//! somewhere else: two profile shapes (a rectangle and a regular hexagon), BOTH windings of each
//! (which flips the profile plane's own frame, hence the cap's `flipped`), and BOTH sweep
//! directions (which flips it again) — plus, for each shape, a profile whose loop traverses every
//! one of its edges BACKWARDS, the `forward = false` half of `build_prism`'s derivation that
//! `🧱️primitives::make_box`'s own side faces exercise and a freshly authored wire never does.
//!
//! Every case asserts the same five things, none of them derivable from another:
//!
//! 1. `validate_body` is silent — closed, 2-manifold, coherently oriented, non-degenerate.
//! 2. Every face's outer loop is counter-clockwise in its OWN surface frame (`Newell · frame.z >
//!    0`), the parametric-region convention a face's trim depends on.
//! 3. Every face's outward normal (`frame.z`, negated when `flipped`) points AWAY from the solid's
//!    centroid — these prisms are convex, so that is exactly outwardness, and it is measured from
//!    the geometry alone, never from the loop.
//! 4. The tessellated triangle soup is closed, has no edge traversed the same way by both its
//!    triangles, and its SIGNED divergence volume equals the closed-form volume within `1e-6` —
//!    positive, i.e. wound outward.
//! 5. `parry3d`'s own `trimesh_signed_volume_and_center_of_mass` over that same soup agrees, so no
//!    number here rests on our arithmetic alone.
//!
//! @see ../../🧬️schema/🔺️diff/➡️sweep/🧮️core/🦀️.rs — `build_prism`'s derivation.

// #region 🔖️Imports
use parry3d::mass_properties::details::trimesh_signed_volume_and_center_of_mass;
use parry3d::na::Point3;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::primitives::{make_planar_face_from_wire, make_polyline_wire, Wire};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::sweep::extrude_face;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::MeshTransfer;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_signed_volume;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::arena::SolidId;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};
use std::collections::HashMap;
use std::f64::consts::TAU;
// #endregion 🔖️Imports

// #region 🔖️Profiles

const TESSELLATION_DEFLECTION: f64 = 1e-3;
const VOLUME_TOLERANCE: f64 = 1e-6;

/// 📐 The rectangle profile, counter-clockwise about `+Z`, in the `z = 0` plane. Every coordinate
/// is exactly representable in `f32`, so the tessellated soup's own volume carries no rounding of
/// its own into the `1e-6` comparison.
fn rectangle_profile() -> Vec<Pnt3> {
    vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.5, 0.0), Pnt3::new(0.0, 1.5, 0.0)]
}

/// 📐 A regular hexagon of circumradius 1 in the `z = 0` plane, counter-clockwise about `+Z` — the
/// `🍄️hexagonal-mushroom-column` example's own profile shape, whose 12 mis-wound cap↔lateral edges
/// were the second signature of the defect.
fn hexagon_profile() -> Vec<Pnt3> {
    (0..6)
        .map(|i| {
            let angle = TAU * f64::from(i) / 6.0;
            Pnt3::new(angle.cos(), angle.sin(), 0.0)
        })
        .collect()
}

/// 📐 Closed-form area of a regular `sides`-gon of circumradius `radius`.
fn regular_polygon_area(radius: f64, sides: usize) -> f64 {
    0.5 * sides as f64 * radius * radius * (TAU / sides as f64).sin()
}

/// 📐 The same points listed the other way round — the wire is geometrically identical and its
/// plane's frame is the mirror image, which is what puts the cap's `flipped` on the other branch.
fn reversed_points(points: &[Pnt3]) -> Vec<Pnt3> {
    points.iter().rev().copied().collect()
}

/// 🧭 Newell normal of a closed 3D polygon, unnormalised (its length is twice the area).
fn newell(points: &[Pnt3]) -> Vec3 {
    let mut normal = Vec3::ZERO;
    for i in 0..points.len() {
        let p = points[i];
        let q = points[(i + 1) % points.len()];
        normal.x += (p.y - q.y) * (p.z + q.z);
        normal.y += (p.z - q.z) * (p.x + q.x);
        normal.z += (p.x - q.x) * (p.y + q.y);
    }
    normal
}

/// 🔁 The same closed wire walked backwards: every member keeps its edge and inverts its sense,
/// and the walk order reverses, so the loop is the mirror circuit through the same geometry with
/// `forward = false` everywhere. Its face is written on the OPPOSITE plane normal, so the loop
/// stays counter-clockwise in the surface's own frame.
fn reversed_wire(wire: &Wire) -> Wire {
    let members = wire.members.iter().rev().map(|&(edge, forward)| (edge, !forward)).collect();
    let mut vertices = vec![wire.vertices[0]];
    vertices.extend(wire.vertices.iter().skip(1).rev().copied());
    Wire { members, vertices, closed: wire.closed }
}

// #endregion 🔖️Profiles

// #region 🔖️MeshProbes

/// 🧮 Signed volume of a triangle soup by the divergence theorem — positive exactly when the soup
/// is wound outward. The whole defect this file guards shows up here as a NEGATIVE value with the
/// right magnitude divided by three.
fn signed_soup_volume(mesh: &MeshTransfer) -> f64 {
    let point = |index: u32| {
        let base = index as usize * 3;
        [f64::from(mesh.position[base]), f64::from(mesh.position[base + 1]), f64::from(mesh.position[base + 2])]
    };
    let mut total = 0.0;
    for triangle in mesh.index.chunks_exact(3) {
        let (a, b, c) = (point(triangle[0]), point(triangle[1]), point(triangle[2]));
        let cross = [b[1] * c[2] - b[2] * c[1], b[2] * c[0] - b[0] * c[2], b[0] * c[1] - b[1] * c[0]];
        total += (a[0] * cross[0] + a[1] * cross[1] + a[2] * cross[2]) / 6.0;
    }
    total
}

/// 🧵 Boundary edges and orientation defects of a triangle soup, on position-welded vertices (the
/// tessellator emits one vertex per face corner). An orientation defect is an edge whose two
/// triangles traverse it the SAME way — watertight, and inside out.
fn soup_edge_defects(mesh: &MeshTransfer) -> (usize, usize) {
    let key = |index: u32| {
        let base = index as usize * 3;
        let quantise = |value: f32| (f64::from(value) * 1e6).round() as i64;
        (quantise(mesh.position[base]), quantise(mesh.position[base + 1]), quantise(mesh.position[base + 2]))
    };
    type Vertex = (i64, i64, i64);
    let mut counts: HashMap<(Vertex, Vertex), (usize, usize)> = HashMap::new();
    for triangle in mesh.index.chunks_exact(3) {
        for pair in [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])] {
            let (first, second) = (key(pair.0), key(pair.1));
            let forward = first <= second;
            let entry = counts.entry(if forward { (first, second) } else { (second, first) }).or_default();
            entry.0 += 1;
            if forward {
                entry.1 += 1;
            }
        }
    }
    let boundary = counts.values().filter(|(total, _)| *total != 2).count();
    let defects = counts.values().filter(|(total, forward)| *total == 2 && *forward != 1).count();
    (boundary, defects)
}

/// 🔮 `parry3d`'s own signed volume over the same soup — the third-party half of every case.
fn parry_signed_volume(mesh: &MeshTransfer) -> f64 {
    let points: Vec<Point3<f32>> = mesh.position.chunks_exact(3).map(|p| Point3::new(p[0], p[1], p[2])).collect();
    let indices: Vec<[u32; 3]> = mesh.index.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
    assert!(!points.is_empty() && !indices.is_empty(), "tessellation produced an empty mesh");
    let (volume, _) = trimesh_signed_volume_and_center_of_mass(&points, &indices);
    f64::from(volume)
}

// #endregion 🔖️MeshProbes

// #region 🔖️Assertions

/// 🧭 Every face's own two orientation statements, checked against the geometry rather than
/// against each other: its loop must be counter-clockwise in its surface's frame, and its outward
/// normal must point away from the solid's centroid (exact for these convex prisms).
fn assert_faces_face_outward(body: &Body, solid: SolidId, label: &str) {
    let faces = body.solid_faces(solid);
    let mut centre = Vec3::ZERO;
    let mut samples = 0.0;
    let mut face_loops = Vec::new();
    for &face in &faces {
        let mut positions = Vec::new();
        for coedge in body.face_coedges(face) {
            let (start, _) = body.coedge_endpoints(coedge).expect("coedge endpoints");
            let position = body.vertices.get(start).expect("vertex").position;
            centre = centre + position.to_vec();
            samples += 1.0;
            positions.push(position);
        }
        face_loops.push((face, positions));
    }
    let centre = Pnt3::new(centre.x / samples, centre.y / samples, centre.z / samples);
    for (face, positions) in face_loops {
        let entity = body.faces.get(face).expect("face");
        let Surface::Plane { frame } = body.surfaces.get(entity.surface).expect("surface") else {
            panic!("{label}: face {face:?} of a straight-edged prism is not planar");
        };
        let winding = newell(&positions);
        assert!(winding.dot(frame.z) > 0.0, "{label}: face {face:?}'s loop is clockwise in its own (u, v) — its trimmed region is the complement of the face");
        let outward = if entity.flipped { -frame.z } else { frame.z };
        let anchor = positions.iter().fold(Vec3::ZERO, |sum, p| sum + p.to_vec()) * (1.0 / positions.len() as f64);
        assert!(outward.dot(anchor - centre.to_vec()) > 0.0, "{label}: face {face:?}'s outward normal {outward:?} points into the solid");
    }
}

/// ✅ The whole contract for one extruded prism.
fn assert_prism_is_outward(profile: &[Pnt3], reversed: bool, direction: Vec3, distance: f64, expected: f64, label: &str) {
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let wire = make_polyline_wire(&mut body, profile, true, &mut recorder).expect("profile wire");
    let normal = newell(profile);
    let (wire, normal) = if reversed { (reversed_wire(&wire), -normal) } else { (wire, normal) };
    let face = make_planar_face_from_wire(&mut body, &wire, profile[0], normal, &mut recorder).expect("profile face");
    let solid = extrude_face(&mut body, face, direction, distance, &mut recorder).expect("extrude");

    let issues = validate_body(&body);
    assert!(issues.is_empty(), "{label}: validate_body reported {} issue(s): {:?}", issues.len(), issues.iter().map(|issue| format!("{}:{}:{}", issue.entity, issue.code, issue.message)).collect::<Vec<_>>());
    assert_faces_face_outward(&body, solid, label);

    let kernel = solid_signed_volume(&body, solid, 1e-6).expect("kernel signed volume");
    assert!((kernel - expected).abs() <= VOLUME_TOLERANCE, "{label}: kernel signed volume {kernel} vs closed form {expected}");

    let mesh = tessellate_solid(&body, solid, TESSELLATION_DEFLECTION).expect("tessellate");
    let (boundary, defects) = soup_edge_defects(&mesh);
    assert_eq!(boundary, 0, "{label}: tessellated soup has {boundary} boundary edge(s)");
    assert_eq!(defects, 0, "{label}: {defects} tessellated edge(s) are traversed the same way by both their triangles — watertight but inside out");
    let soup = signed_soup_volume(&mesh);
    assert!((soup - expected).abs() <= VOLUME_TOLERANCE, "{label}: tessellated signed volume {soup} vs closed form {expected}");

    let theirs = parry_signed_volume(&mesh);
    assert!((theirs - expected).abs() <= 1e-4, "{label}: parry3d signed volume {theirs} vs closed form {expected}");
}

// #endregion 🔖️Assertions

// #region 🔖️Rectangle

#[test]
fn rectangle_profile_wound_ccw_extruded_up_faces_outward() {
    assert_prism_is_outward(&rectangle_profile(), false, Vec3::Z, 3.0, 2.0 * 1.5 * 3.0, "rectangle ccw +Z");
}

#[test]
fn rectangle_profile_wound_ccw_extruded_down_faces_outward() {
    assert_prism_is_outward(&rectangle_profile(), false, -Vec3::Z, 3.0, 2.0 * 1.5 * 3.0, "rectangle ccw -Z");
}

#[test]
fn rectangle_profile_wound_cw_extruded_up_faces_outward() {
    assert_prism_is_outward(&reversed_points(&rectangle_profile()), false, Vec3::Z, 3.0, 2.0 * 1.5 * 3.0, "rectangle cw +Z");
}

#[test]
fn rectangle_profile_wound_cw_extruded_down_faces_outward() {
    assert_prism_is_outward(&reversed_points(&rectangle_profile()), false, -Vec3::Z, 3.0, 2.0 * 1.5 * 3.0, "rectangle cw -Z");
}

#[test]
fn rectangle_profile_with_backward_coedges_extruded_up_faces_outward() {
    assert_prism_is_outward(&rectangle_profile(), true, Vec3::Z, 3.0, 2.0 * 1.5 * 3.0, "rectangle backward-coedges +Z");
}

#[test]
fn rectangle_profile_with_backward_coedges_extruded_down_faces_outward() {
    assert_prism_is_outward(&rectangle_profile(), true, -Vec3::Z, 3.0, 2.0 * 1.5 * 3.0, "rectangle backward-coedges -Z");
}

// #endregion 🔖️Rectangle

// #region 🔖️Hexagon

#[test]
fn hexagon_profile_wound_ccw_extruded_up_faces_outward() {
    assert_prism_is_outward(&hexagon_profile(), false, Vec3::Z, 2.0, regular_polygon_area(1.0, 6) * 2.0, "hexagon ccw +Z");
}

#[test]
fn hexagon_profile_wound_ccw_extruded_down_faces_outward() {
    assert_prism_is_outward(&hexagon_profile(), false, -Vec3::Z, 2.0, regular_polygon_area(1.0, 6) * 2.0, "hexagon ccw -Z");
}

#[test]
fn hexagon_profile_wound_cw_extruded_up_faces_outward() {
    assert_prism_is_outward(&reversed_points(&hexagon_profile()), false, Vec3::Z, 2.0, regular_polygon_area(1.0, 6) * 2.0, "hexagon cw +Z");
}

#[test]
fn hexagon_profile_wound_cw_extruded_down_faces_outward() {
    assert_prism_is_outward(&reversed_points(&hexagon_profile()), false, -Vec3::Z, 2.0, regular_polygon_area(1.0, 6) * 2.0, "hexagon cw -Z");
}

#[test]
fn hexagon_profile_with_backward_coedges_extruded_up_faces_outward() {
    assert_prism_is_outward(&hexagon_profile(), true, Vec3::Z, 2.0, regular_polygon_area(1.0, 6) * 2.0, "hexagon backward-coedges +Z");
}

#[test]
fn hexagon_profile_with_backward_coedges_extruded_down_faces_outward() {
    assert_prism_is_outward(&hexagon_profile(), true, -Vec3::Z, 2.0, regular_polygon_area(1.0, 6) * 2.0, "hexagon backward-coedges -Z");
}

// #endregion 🔖️Hexagon
