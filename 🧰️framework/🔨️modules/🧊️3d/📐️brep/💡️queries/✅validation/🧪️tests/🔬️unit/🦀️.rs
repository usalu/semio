use super::*;
use {semio_framework_3d_brep_construction::euler::{add_face, add_shell, add_solid, make_edge, make_vertex}, semio_framework_3d_brep_representation::topology::make_loop};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn attach_planar_pcurves(body: &mut Body, face: FaceId, frame: &Frame3) {
    for coedge_id in body.face_coedges(face) {
        let co = body.coedges.get(coedge_id).unwrap();
        let edge = body.edges.get(co.edge).unwrap();
        let Curve3::Line { origin, dir } = *body.curves3.get(edge.curve).unwrap() else { continue };
        let local_o = frame.to_local(origin);
        let local_d = frame.to_local_vector(dir);
        let pcurve = body.curves2.insert(semio_framework_3d_brep_curve::Curve2::Line {
            origin: semio_framework_3d_brep_vector::Pnt2::new(local_o.x, local_o.y),
            dir: semio_framework_3d_brep_vector::Vec2::new(local_d.x, local_d.y),
        });
        let prange = (0.0, 1.0);
        let co = body.coedges.get_mut(coedge_id).unwrap();
        co.pcurve = Some(pcurve);
        co.prange = prange;
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_tetrahedron_with_windings(body: &mut Body, rec: &mut OpRecorder, face_defs: [[usize; 3]; 4]) -> semio_framework_3d_brep_representation::arena::SolidId {
    let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
    let vertices: Vec<_> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
    let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
    let mut edges = std::collections::HashMap::new();
    for &(a, b) in &edge_pairs {
        let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
        let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
        edges.insert((a, b), edge);
        edges.insert((b, a), edge);
    }
    let mut faces = Vec::new();
    for tri in face_defs {
        let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
        let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let members: Vec<(EdgeId, bool)> = (0..3)
            .map(|i| {
                let a = tri[i];
                let b = tri[(i + 1) % 3];
                let edge = edges[&(a, b)];
                let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                (edge, forward)
            })
            .collect();
        let outer = make_loop(body, ArenaId::from_raw(0, 0), &members);
        let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
        body.loops.get_mut(outer).unwrap().face = face;
        attach_planar_pcurves(body, face, &frame);
        faces.push(face);
    }
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
// 🐛 FIX (ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` FX-5): these two windings were
// swapped. For vertices `0=(0,0,0), 1=(1,0,0), 2=(0,1,0), 3=(0,0,1)`, `[[0,1,2],[0,3,1],[1,3,2],
// [2,3,0]]` — the OLD `build_tetrahedron` — has every face's `(v1-v0)×(v2-v0)` normal pointing
// TOWARD the face's own excluded (4th) vertex, i.e. INWARD (verified algebraically per face,
// not just by the validator's own say-so): e.g. face `[0,1,2]` lies in `z=0` with normal
// `(0,0,1)`, the SAME side as the excluded vertex `3=(0,0,1)`. `[[0,2,1],[0,1,3],[1,2,3],
// [2,0,3]]` — the OLD `build_tetrahedron_globally_reversed` — is each of those faces with its
// last two vertices swapped, i.e. the genuinely OUTWARD-facing tetrahedron. The fixtures were
// simply mislabeled; the validator's `shell-orientation-inward` check was correct both times.
fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> semio_framework_3d_brep_representation::arena::SolidId {
    build_tetrahedron_with_windings(body, rec, [[0, 2, 1], [0, 1, 3], [1, 2, 3], [2, 0, 3]])
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_tetrahedron_globally_reversed(body: &mut Body, rec: &mut OpRecorder) -> semio_framework_3d_brep_representation::arena::SolidId {
    build_tetrahedron_with_windings(body, rec, [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]])
}

#[semio_framework_async_macros::async_test]
async fn a_cleanly_built_tetrahedron_validates_with_no_issues() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "unexpected issues on a clean solid: {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn a_broken_ring_pointer_is_detected() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedges = body.loop_coedges(outer);
    let first = coedges[0];
    body.coedges.get_mut(first).unwrap().next = first;
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "broken-ring" || i.code == "next-prev-mismatch"), "expected a ring issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn a_vertex_tolerance_exceeding_its_edge_tolerance_is_detected() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let (vertex_id, _) = body.vertices.iter().next().unwrap();
    body.vertices.get_mut(vertex_id).unwrap().tol = Tol::new(10.0);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "tolerance-containment-violated"), "expected a tolerance issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn a_non_manifold_edge_is_flagged() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let v1 = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
    let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, Tol::DEFAULT, &mut rec);
    for _ in 0..3 {
        body.coedges.insert(semio_framework_3d_brep_representation::topology::Coedge {
            edge,
            forward: true,
            pcurve: None,
            prange: (0.0, 1.0),
            loop_id: ArenaId::from_raw(0, 0),
            next: ArenaId::from_raw(0, 0),
            prev: ArenaId::from_raw(0, 0),
        });
    }
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "non-manifold-edge"), "expected a non-manifold-edge issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn same_parameter_violation_is_detected_when_pcurve_disagrees_with_3d_curve() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedge_id = body.loop_coedges(outer)[0];
    let bad_pcurve = body.curves2.insert(semio_framework_3d_brep_curve::Curve2::Line {
        origin: semio_framework_3d_brep_vector::Pnt2::new(500.0, 500.0),
        dir: semio_framework_3d_brep_vector::Vec2::new(0.0, 0.0),
    });
    let coedge = body.coedges.get_mut(coedge_id).unwrap();
    coedge.pcurve = Some(bad_pcurve);
    coedge.prange = (0.0, 1.0);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "same-parameter-violated"), "expected a same-parameter issue, got {issues:?}");
}

#[test]
fn validation_same_parameter_retains_all_original_adaptive_passes_and_verdict() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let probe = &fixture["sameParameterProbe"];
    let mut body = Body::new();
    let solid = build_tetrahedron(&mut body, &mut OpRecorder::new());
    let face = body.solid_faces(solid)[probe["face"].as_u64().unwrap() as usize];
    let coedge = body.face_coedges(face)[probe["coedge"].as_u64().unwrap() as usize];
    let pcurve = body.coedges.get(coedge).unwrap().pcurve.unwrap();
    let semio_framework_3d_brep_curve::Curve2::Line { dir, .. } = body.curves2.get_mut(pcurve).unwrap() else { panic!("neutral original planar edge"); };
    *dir = semio_framework_3d_brep_vector::Vec2::new(0.0, 0.0);
    let mut job = BodyValidationJob::new(&body);
    let mut samples = 0;
    while !job.is_complete() {
        SAME_PARAMETER_SAMPLES.with(|value| value.set(0));
        job.step(&body, fixture["budget"]["turn"].as_u64().unwrap() as usize);
        let actual = SAME_PARAMETER_SAMPLES.with(std::cell::Cell::get);
        assert!(actual <= fixture["budget"]["maximumSameParameterSamples"].as_u64().unwrap() as usize);
        samples += actual;
    }
    let issues = job.into_issues();
    let verdict = issues.iter().find(|issue| issue.code == "same-parameter-violated" && issue.entity == format!("coedge-{}", coedge.raw_index())).expect("original adaptive verdict");
    let deviation = probe["maximumDeviation"].as_f64().unwrap();
    let parameter = probe["worstParameter"].as_f64().unwrap();
    assert!(verdict.message.starts_with(&format!("pcurve and 3D curve disagree by {deviation} at s={parameter} ")), "{verdict:?}");
    assert_eq!(samples, probe["totalBodySamples"].as_u64().unwrap() as usize);
}

#[semio_framework_async_macros::async_test]
async fn missing_pcurve_is_an_error_not_a_skip() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedge_id = body.loop_coedges(outer)[0];
    body.coedges.get_mut(coedge_id).unwrap().pcurve = None;
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "missing-pcurve"), "expected a missing-pcurve issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn shell_not_closed_is_detected_when_a_face_is_removed() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let shell_id = body.solid_shells(solid)[0];
    body.shells.get_mut(shell_id).unwrap().faces.pop();
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "shell-not-closed"), "expected a shell-not-closed issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn orientation_inconsistent_is_detected_when_a_shared_edge_sense_is_flipped() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedge_id = body.loop_coedges(outer)[0];
    let co = body.coedges.get_mut(coedge_id).unwrap();
    co.forward = !co.forward;
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "orientation-inconsistent"), "expected an orientation-inconsistent issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn shell_orientation_inward_is_detected_on_a_globally_reversed_tetrahedron() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron_globally_reversed(&mut body, &mut rec);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "shell-orientation-inward"), "expected a shell-orientation-inward issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn degenerate_edge_is_detected_when_tolerance_exceeds_edge_length() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let (edge_id, _) = body.edges.iter().next().unwrap();
    body.edges.get_mut(edge_id).unwrap().tol = Tol::new(10.0);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "degenerate-edge"), "expected a degenerate-edge issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn sliver_face_is_detected_when_tolerance_exceeds_face_area() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let (face_id, _) = body.faces.iter().next().unwrap();
    body.faces.get_mut(face_id).unwrap().tol = Tol::new(10.0);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "sliver-face"), "expected a sliver-face issue, got {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn self_intersection_probe_warns_on_overlapping_non_adjacent_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = semio_framework_3d_brep_construction::primitives::make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let b = semio_framework_3d_brep_construction::primitives::make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let mut faces = body.solid_faces(a);
    faces.extend(body.solid_faces(b));
    let shell = add_shell(&mut body, faces, &mut rec);
    add_solid(&mut body, shell, vec![], &mut rec);
    let issues = validate_body(&body);
    assert!(issues.iter().any(|i| i.code == "warning-possible-self-intersection"), "expected a self-intersection warning, got {issues:?}");
}


#[test]
fn validation_cold_planning_collects_original_ids_only_after_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let faces: [[usize; 3]; 4] = serde_json::from_value(fixture["faces"].clone()).unwrap();
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let solid = build_tetrahedron_with_windings(&mut body, &mut recorder, faces);
    let mut job = BodyValidationJob::new(&body);
    let collected = |job: &BodyValidationJob| job.edges.len() + job.faces.len() + job.shells.iter().map(|shell| 1 + shell.faces.len()).sum::<usize>();
    assert_eq!(collected(&job), fixture["initialCollectedIds"].as_u64().unwrap() as usize);
    let initial = job.progress();
    assert_eq!(job.step(&body, fixture["budget"]["probe"].as_u64().unwrap() as usize), initial);
    assert_eq!(collected(&job), 0);
    for _ in 0..fixture["budget"]["planningTurns"].as_u64().unwrap() {
        let before = collected(&job);
        let progress = job.step(&body, fixture["budget"]["turn"].as_u64().unwrap() as usize);
        assert!(collected(&job).saturating_sub(before) <= 1, "one grant collected multiple original ids");
        assert!(!job.is_complete());
        assert!(progress.units_done <= progress.units_total);
    }
    let issues = job.run_to_completion(&body);
    let codes: Vec<_> = issues.iter().map(|issue| issue.code).collect();
    assert_eq!(serde_json::to_value(&codes).unwrap(), fixture["expected"]["issueCodes"]);
    let volume = mass_properties::solid_signed_volume(&body, solid, 1e-3).unwrap();
    assert!((volume - fixture["expected"]["volume"].as_f64().unwrap()).abs() < 1e-10);
}


#[test]
fn validation_loop_probe_spends_only_the_original_granted_coedge_work() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    build_tetrahedron_with_windings(&mut body, &mut recorder, serde_json::from_value(fixture["faces"].clone()).unwrap());
    let mut job = BodyValidationJob::new(&body);
    while job.progress().phase == BodyValidationPhase::CollectingTopology { job.step(&body, 1); }
    let maximum = fixture["budget"]["maximumCoedgeProbes"].as_u64().unwrap() as usize;
    let mut probes = 0;
    while job.progress().phase == BodyValidationPhase::LoopRings {
        LOOP_PROBES.with(|value| value.set(0));
        job.step(&body, 0);
        assert_eq!(LOOP_PROBES.with(std::cell::Cell::get), 0);
        job.step(&body, 1);
        let actual = LOOP_PROBES.with(std::cell::Cell::get);
        assert!(actual <= maximum, "one original grant performed {actual} coedge probes");
        probes += actual;
    }
    assert_eq!(probes, fixture["expected"]["faces"].as_u64().unwrap() as usize * 3);
    assert!(job.run_to_completion(&body).is_empty());
}

#[test]
fn validation_valence_spends_only_the_original_granted_coedge_work() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    build_tetrahedron_with_windings(&mut body, &mut recorder, serde_json::from_value(fixture["faces"].clone()).unwrap());
    let mut job = BodyValidationJob::new(&body);
    while matches!(job.progress().phase, BodyValidationPhase::CollectingTopology | BodyValidationPhase::LoopRings) { job.step(&body, 1); }
    let maximum = fixture["budget"]["maximumValenceProbes"].as_u64().unwrap() as usize;
    let mut probes = 0;
    while job.progress().phase == BodyValidationPhase::EdgeValence {
        EDGE_USE_PROBES.with(|value| value.set(0));
        job.step(&body, 0);
        assert_eq!(EDGE_USE_PROBES.with(std::cell::Cell::get), 0);
        job.step(&body, 1);
        let actual = EDGE_USE_PROBES.with(std::cell::Cell::get);
        assert!(actual <= maximum, "one original grant performed {actual} edge-use probes");
        probes += actual;
    }
    assert_eq!(probes, fixture["expected"]["edges"].as_u64().unwrap() as usize * fixture["expected"]["faces"].as_u64().unwrap() as usize * 3);
    assert!(job.run_to_completion(&body).is_empty());
}

#[test]
fn validation_pcurve_presence_spends_only_the_original_granted_coedge_work() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    build_tetrahedron_with_windings(&mut body, &mut recorder, serde_json::from_value(fixture["faces"].clone()).unwrap());
    let mut job = BodyValidationJob::new(&body);
    while job.progress().phase != BodyValidationPhase::MissingPcurves { job.step(&body, 1); }
    let maximum = fixture["budget"]["maximumPCurveProbes"].as_u64().unwrap() as usize;
    let mut probes = 0;
    while job.progress().phase == BodyValidationPhase::MissingPcurves {
        PCURVE_PROBES.with(|value| value.set(0));
        job.step(&body, 0);
        assert_eq!(PCURVE_PROBES.with(std::cell::Cell::get), 0);
        job.step(&body, 1);
        let actual = PCURVE_PROBES.with(std::cell::Cell::get);
        assert!(actual <= maximum, "one original grant performed {actual} pcurve-presence probes");
        probes += actual;
    }
    assert_eq!(probes, fixture["expected"]["faces"].as_u64().unwrap() as usize * 3);
    assert!(job.run_to_completion(&body).is_empty());
}

#[test]
fn validation_tolerance_spends_only_the_original_granted_containment_work() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    build_tetrahedron_with_windings(&mut body, &mut recorder, serde_json::from_value(fixture["faces"].clone()).unwrap());
    let mut job = BodyValidationJob::new(&body);
    while job.progress().phase != BodyValidationPhase::ToleranceContainment { job.step(&body, 1); }
    let maximum = fixture["budget"]["maximumToleranceProbes"].as_u64().unwrap() as usize;
    let mut probes = 0;
    while job.progress().phase == BodyValidationPhase::ToleranceContainment {
        TOLERANCE_PROBES.with(|value| value.set(0));
        job.step(&body, 0);
        assert_eq!(TOLERANCE_PROBES.with(std::cell::Cell::get), 0);
        job.step(&body, 1);
        let actual = TOLERANCE_PROBES.with(std::cell::Cell::get);
        assert!(actual <= maximum, "one original grant performed {actual} tolerance-containment probes");
        probes += actual;
    }
    assert_eq!(probes, fixture["expected"]["edges"].as_u64().unwrap() as usize * 2 + fixture["expected"]["faces"].as_u64().unwrap() as usize * 3);
    assert!(job.run_to_completion(&body).is_empty());
}

#[test]
fn validation_same_parameter_spends_only_the_original_granted_geometry_sample() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    build_tetrahedron_with_windings(&mut body, &mut recorder, serde_json::from_value(fixture["faces"].clone()).unwrap());
    let mut job = BodyValidationJob::new(&body);
    while job.progress().phase != BodyValidationPhase::SameParameter { job.step(&body, 1); }
    let maximum = fixture["budget"]["maximumSameParameterSamples"].as_u64().unwrap() as usize;
    let mut samples = 0;
    while job.progress().phase == BodyValidationPhase::SameParameter {
        SAME_PARAMETER_SAMPLES.with(|value| value.set(0));
        job.step(&body, 0);
        assert_eq!(SAME_PARAMETER_SAMPLES.with(std::cell::Cell::get), 0);
        job.step(&body, 1);
        let actual = SAME_PARAMETER_SAMPLES.with(std::cell::Cell::get);
        assert!(actual <= maximum, "one original grant performed {actual} same-parameter geometry samples");
        samples += actual;
    }
    assert_eq!(samples, fixture["expected"]["faces"].as_u64().unwrap() as usize * 3 * (fixture["budget"]["sameParameterBaseSamples"].as_u64().unwrap() as usize + 1));
    assert!(job.run_to_completion(&body).is_empty());
}


type ValidationGrant = semio_framework_value::retained_clone::RetainedCloneGrant;
type ValidationReceipt = semio_framework_value::retained_clone::RetainedCloneProgress;

fn validation_fixture_caller() -> semio_framework_job::OriginalJobFixtureCaller {
    let stock = ValidationGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 256 };
    semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(943), semio_framework_job::Generation(1), stock, || Some(1), 1_000_000, 100_000_000)
}

fn observed_validation_turn(
    caller: &mut semio_framework_job::OriginalJobFixtureCaller,
    grant: ValidationGrant,
    run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<(ValidationReceipt, usize),semio_framework_value::ValueError>,
) -> ((ValidationReceipt, usize), (usize, usize)) {
    caller.with_effect(grant, |effect| crate::test_support::observe_tessellation_system(|| run(effect).unwrap()))
        .unwrap_or_else(|_| ((Default::default(),0),(0,0)))
}

fn validation_close_turn(
    close: &mut semio_framework_value::retirement::controlled::ControlledRetirement<BodyValidationJob>,
    effect: &mut semio_framework_value::RetainedAllocationEffect<'_>,
) -> Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError> {
    use semio_framework_value::{retained_clone::RetainedCloneStep, RetainedWorkPoll};
    match close.step(effect.grant().maximum_copy_bytes, effect) {
        RetainedWorkPoll::Pending{..} => Ok(RetainedCloneStep::Progress(Default::default())),
        RetainedWorkPoll::Complete(turn) => turn.result.map_err(|failure| failure.into_value_error()),
    }
}

#[test]
fn original_validation_planning_quotes_actual_backing_before_entity_copy() {
    let mut fixture_caller = validation_fixture_caller();
    use crate::test_support::observe_tessellation_system as observe;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();let law=&fixture["retainedPlanning"];
    assert_eq!(std::mem::size_of::<EdgeId>(),law["entityCopyBytes"].as_u64().unwrap()as usize);
    for count in law["sourceTetrahedra"].as_array().unwrap(){let count=count.as_u64().unwrap()as usize;
    for stop in law["cancelStops"].as_array().unwrap(){
        let mut body=Body::new();let mut rec=OpRecorder::new();for _ in 0..count{build_tetrahedron(&mut body,&mut rec);}
        let original=body.edges.slot_at(0).unwrap().1 as *const _;
        let(mut job,heap)=observe(||BodyValidationJob::new(&body));assert_eq!(heap,(0,0));let(mut born,mut freed,mut turns,mut denials,mut reserves,mut copies)=(0,0,0,0,0,0);
        let other=Body::new();let(result,heap)=observe(||job.planning_demands(&other));assert!(result.is_err());assert_eq!(heap,(0,0));
        while job.phase==BodyValidationPhase::CollectingTopology&&stop.as_u64().is_none_or(|stop|turns<stop as usize){
            let(demand,heap)=observe(||job.planning_demands(&body).unwrap());assert_eq!(heap,(0,0));
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let(result,heap)=observed_validation_turn(&mut fixture_caller,grant,|effect|job.step_planning_funded(&body,0,effect));assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));
            for body_bytes in law["bodyBytes"].as_array().unwrap(){let body_bytes=body_bytes.as_u64().unwrap()as usize;if body_bytes<demand.copy_bytes{let(result,heap)=observed_validation_turn(&mut fixture_caller,RetainedCloneGrant{maximum_copy_bytes:body_bytes,..grant},|effect|job.step_planning_funded(&body,1,effect));assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));denials+=1;}}
            for axis in 0..5{
                let amount=match axis{0=>1,1=>demand.copy_bytes,2=>demand.capacity_bytes,3=>demand.release_bytes,_=>demand.depth};if amount==0{continue;}
                let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1};
                let before=(job.units_done,job.planning_stage,job.planning_slot,job.edges.len(),job.faces.len(),job.shells.len());
                let(result,heap)=observed_validation_turn(&mut fixture_caller,denied,|effect|job.step_planning_funded(&body,1,effect));assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));assert_eq!(before,(job.units_done,job.planning_stage,job.planning_slot,job.edges.len(),job.faces.len(),job.shells.len()));denials+=1;
            }
            let((receipt,units),heap)=observed_validation_turn(&mut fixture_caller,grant,|effect|job.step_planning_funded(&body,1,effect));assert!(receipt.fits(grant));assert_eq!(units,1);assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;reserves+=usize::from(heap.0!=0);copies+=usize::from(receipt.copied_bytes!=0);assert_eq!(body.edges.slot_at(0).unwrap().1 as *const _,original);turns+=1;assert!(turns<10000);
        }
        if stop.is_null(){assert_eq!(job.edges.len(),fixture["expected"]["edges"].as_u64().unwrap()as usize*count);assert_eq!(job.faces.len(),fixture["expected"]["faces"].as_u64().unwrap()as usize*count);assert_eq!(job.shells.len(),count);assert!(job.edges.iter().copied().eq(body.edges.iter().map(|(id,_)|id)));assert!(job.faces.iter().copied().eq(body.faces.iter().map(|(id,_)|id)));for shell in job.shells.iter(){assert_eq!(shell.faces,body.shells.get(shell.shell).unwrap().faces);}assert!(reserves>0&&copies>0);}
        let edge_pointer=job.edges.as_ptr();let(mut closing,heap)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original validation planning close")));assert_eq!(heap,(0,0));assert_eq!(closing.original().unwrap().edges.as_ptr(),edge_pointer);
        while !closing.terminal_is_empty(){let demand=closing.next_demand(0).and_then(|first|closing.next_demand(first.copy_bytes)).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,heap)=fixture_caller.with_effect(grant,|effect|observe(||validation_close_turn(&mut closing,effect).unwrap())).unwrap_or_else(|_|panic!("expected funded retirement turn"));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
        let(_,heap)=observe(||drop(closing));assert_eq!(heap,(0,0));assert_eq!(born,freed);
    }
    }
    fixture_caller.retire_exact();
}


#[test]
fn original_validation_loop_frontier_quotes_source_and_diagnostic_events() {
    let mut fixture_caller = validation_fixture_caller();
    use crate::test_support::observe_tessellation_system as observe;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();let law=&fixture["retainedLoops"];
    for count in law["sourceTetrahedra"].as_array().unwrap(){let count=count.as_u64().unwrap()as usize;for broken in [false,true]{for stop in law["cancelStops"].as_array().unwrap(){
        let mut body=Body::new();let mut rec=OpRecorder::new();for _ in 0..count{build_tetrahedron(&mut body,&mut rec);}
        if broken{let first=body.loops.slot_at(0).unwrap().1.first;let next=body.coedges.get(first).unwrap().next;body.coedges.get_mut(next).unwrap().prev=next;}
        let source=&body as*const _;let mut job=BodyValidationJob::new(&body);let(mut born,mut freed)=(0,0);
        while job.phase==BodyValidationPhase::CollectingTopology{let demand=job.funded_demands(&body).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let((receipt,_),heap)=observed_validation_turn(&mut fixture_caller,grant,|effect|job.step_funded(&body,1,effect));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;}
        let(mut turns,mut denies,mut backing,mut texts)=(0,0,0,0);
        while job.phase==BodyValidationPhase::LoopRings&&stop.as_u64().is_none_or(|stop|turns<stop as usize){
            let(demand,heap)=observe(||job.funded_demands(&body).unwrap());assert_eq!(heap,(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let before=(job.progress(),job.loop_slot,job.ring.as_ptr(),job.ring.len(),job.ring_probe,job.loop_finding_cursor,job.issues.len());
            for axis in 0..5{let amount=match axis{0=>1,1=>demand.copy_bytes,2=>demand.capacity_bytes,3=>demand.release_bytes,_=>demand.depth};if amount==0{continue}let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1};let(result,heap)=observed_validation_turn(&mut fixture_caller,denied,|effect|job.step_funded(&body,1,effect));assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));assert_eq!(before,(job.progress(),job.loop_slot,job.ring.as_ptr(),job.ring.len(),job.ring_probe,job.loop_finding_cursor,job.issues.len()));denies+=1;}
            let(result,heap)=observed_validation_turn(&mut fixture_caller,grant,|effect|job.step_funded(&body,0,effect));assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));
            let((receipt,units),heap)=observed_validation_turn(&mut fixture_caller,grant,|effect|job.step_funded(&body,1,effect));assert_eq!(units,1);assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;backing+=usize::from(heap.0!=0);texts+=usize::from(receipt.copied_bytes==1);turns+=1;assert!(turns<100000);assert_eq!(&body as*const _,source);
        }
        if stop.is_null(){assert_eq!(job.phase,BodyValidationPhase::EdgeValence);assert!(backing>0);if broken{let finding=job.issues.iter().next().unwrap();assert_eq!(job.issues.len(),1);assert_eq!(finding.entity,law["brokenNeighbor"]["entity"]);assert_eq!(finding.code,law["brokenNeighbor"]["code"]);assert_eq!(finding.message,law["brokenNeighbor"]["message"]);assert!(texts>0);}else{assert!(job.issues.is_empty());}}
        let pointer=job.ring.as_ptr();let(mut close,heap)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original loop custody")));assert_eq!(heap,(0,0));assert_eq!(close.original().unwrap().ring.as_ptr(),pointer);
        while !close.terminal_is_empty(){let demand=close.next_demand(0).and_then(|first|close.next_demand(first.copy_bytes)).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,heap)=fixture_caller.with_effect(grant,|effect|observe(||validation_close_turn(&mut close,effect).unwrap())).unwrap_or_else(|_|panic!("expected funded retirement turn"));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
        let(_,heap)=observe(||drop(close));assert_eq!(heap,(0,0));assert_eq!(born,freed);
    }}}
    fixture_caller.retire_exact();
}


#[test]
fn original_validation_topology_frontier_quotes_each_probe_and_utf8_finding() {
    let mut fixture_caller = validation_fixture_caller();
 use crate::test_support::observe_tessellation_system as observe;
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();let law=&fixture["retainedTopology"];
 for malformed in [false,true]{for stop in law["cancelStops"].as_array().unwrap(){
  let mut body=Body::new();build_tetrahedron(&mut body,&mut OpRecorder::new());
  if malformed{let id=body.coedges.slot_at(0).unwrap().0;body.coedges.get_mut(id).unwrap().pcurve=None;let id=body.vertices.slot_at(0).unwrap().0;body.vertices.get_mut(id).unwrap().tol=Tol::new(10.0);}
  let mut job=BodyValidationJob::new(&body);let(mut born,mut freed,mut turns,mut denials,mut utf8)=(0,0,0,0,0);
  while job.phase!=BodyValidationPhase::EdgeValence{let d=job.funded_demands(&body).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let((r,_),h)=observed_validation_turn(&mut fixture_caller,g,|effect|job.step_funded(&body,1,effect));assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));born+=h.0;freed+=h.1;}
  while job.phase!=BodyValidationPhase::SameParameter&&stop.as_u64().is_none_or(|stop|turns<stop as usize){
   let(d,h)=observe(||job.funded_demands(&body).unwrap());assert_eq!(h,(0,0));let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
   let before=(job.progress(),job.valence_edge_slot,job.valence_coedge_slot,job.pcurve_slot,job.tolerance_edge_cursor,job.tolerance_vertex,job.issues.len());
   for axis in 0..5{let n=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,3=>d.release_bytes,_=>d.depth};if n==0{continue}let mut denied=g;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1};let(r,h)=observed_validation_turn(&mut fixture_caller,denied,|effect|job.step_funded(&body,1,effect));assert_eq!(r,(Default::default(),0));assert_eq!(h,(0,0));assert_eq!(before,(job.progress(),job.valence_edge_slot,job.valence_coedge_slot,job.pcurve_slot,job.tolerance_edge_cursor,job.tolerance_vertex,job.issues.len()));denials+=1;}
   for width in law["bodyBytes"].as_array().unwrap(){let width=width.as_u64().unwrap()as usize;if width<d.copy_bytes{let(r,h)=observed_validation_turn(&mut fixture_caller,RetainedCloneGrant{maximum_copy_bytes:width,..g},|effect|job.step_funded(&body,1,effect));assert_eq!(r,(Default::default(),0));assert_eq!(h,(0,0));}}
   let((r,u),h)=observed_validation_turn(&mut fixture_caller,g,|effect|job.step_funded(&body,1,effect));assert_eq!(u,1);assert!(r.fits(g));assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));born+=h.0;freed+=h.1;utf8+=usize::from(r.copied_bytes==3);turns+=1;assert!(turns<10000);
  }
  if stop.is_null(){if malformed{let missing=job.issues.iter().find(|i|i.code=="missing-pcurve").unwrap();assert_eq!(missing.entity,law["missingPcurve"]["entity"]);assert_eq!(missing.message,law["missingPcurve"]["message"]);let tolerance=job.issues.iter().find(|i|i.entity==law["tolerance"]["entity"]&&i.message==law["tolerance"]["message"]).unwrap();assert_eq!(tolerance.code,law["tolerance"]["code"]);assert!(utf8>0);}else{assert!(job.issues.is_empty());}}
  let(mut close,h)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original topology custody")));assert_eq!(h,(0,0));
  while !close.terminal_is_empty(){let d=close.next_demand(0).and_then(|first|close.next_demand(first.copy_bytes)).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(s,h)=fixture_caller.with_effect(g,|effect|observe(||validation_close_turn(&mut close,effect).unwrap())).unwrap_or_else(|_|panic!("expected funded retirement turn"));assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));born+=h.0;freed+=h.1;}
  let(_,h)=observe(||drop(close));assert_eq!(h,(0,0));assert_eq!(born,freed);
 }}
    fixture_caller.retire_exact();
}

#[test]
fn original_validation_same_parameter_retains_scratch_samples_and_one_original_caller(){
 use crate::test_support::observe_tessellation_system as observe;
 use semio_framework_job::{OriginalJobFixtureCaller,OriginalTurnFailure,OperationId,Generation};
 use semio_framework_value::{retained_clone::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress},retirement::controlled::ControlledRetirement};
 fn clock()->Option<u64>{Some(1000)}
 fn grant(row:&serde_json::Value)->Grant{Grant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:row["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:row["release"].as_u64().unwrap()as usize,maximum_depth:row["depth"].as_u64().unwrap()as usize}}
 fn turn(job:&mut BodyValidationJob,body:&Body,cx:&mut semio_framework_job::StepContext<'_>)->Result<Progress,OriginalTurnFailure>{
  let g=cx.retained_grant();if cx.should_yield()||g.maximum_items==0{return Ok(Default::default())}let d=job.funded_demands(body)?;if g.maximum_copy_bytes<d.copy_bytes||g.maximum_capacity_bytes<d.capacity_bytes||g.maximum_release_bytes<d.release_bytes||g.maximum_depth<d.depth{return Ok(Default::default())}cx.consume_fuel(1);let Some((outcome,work))=cx.with_original_effect(g,|_,effect| job.step_funded(body,1,effect))? else { return Ok(Default::default()); };cx.consume_work(work);let(r,u)=outcome.map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;assert_eq!(u,1);cx.consume_retained(r)?;Ok(r)
 }
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();let law=&fixture["retainedSameParameter"];
 for width in [1,3,64]{for malformed in [false,true]{for stop in law["cancelStops"].as_array().unwrap(){
  let mut body=Body::new();let solid=build_tetrahedron(&mut body,&mut OpRecorder::new());let face=body.solid_faces(solid)[0];let coedge=body.face_coedges(face)[0];if malformed{let pcurve=body.coedges.get(coedge).unwrap().pcurve.unwrap();let semio_framework_3d_brep_curve::Curve2::Line{dir,..}=body.curves2.get_mut(pcurve).unwrap()else{panic!("original planar fixture")};*dir=semio_framework_3d_brep_vector::Vec2::new(0.0,0.0);}
  let source=&body as*const Body;let policy=grant(&law["policy"]);let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(37),grant(&law["stock"]),clock,law["deadline"].as_u64().unwrap(),law["fuel"].as_u64().unwrap());let(mut job,h)=observe(||BodyValidationJob::new(&body));assert_eq!(h,(0,0));
  while job.phase!=BodyValidationPhase::SameParameter{let(r,h)=observe(||caller.turn(policy,|cx|turn(&mut job,&body,cx)));let r=r.unwrap();assert_ne!(r,Default::default(),"phase {:?}, progress {:?}, demand {:?}, policy {:?}",job.phase,job.progress(),job.funded_demands(&body).unwrap(),policy);assert_eq!(r,caller.receipt());assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));}
  let policy=Grant{maximum_copy_bytes:width,..policy};let mut samples=0;let mut events=0;let mut born=0;let mut denied=0;let mut recipient=0;
  while job.phase==BodyValidationPhase::SameParameter&&stop.as_u64().is_none_or(|stop|events<stop as usize){
   let(d,h)=observe(||job.funded_demands(&body).unwrap());assert_eq!(h,(0,0));let before=(job.progress(),job.same_stage,job.same_cursor,job.same_samples.len(),job.same_midpoints.len(),job.same_sort_cursor,job.issues.len());
   for axis in [0,1,2,3,4]{let n=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,3=>d.release_bytes,_=>d.depth};if n==0{continue}let mut g=policy;match axis{0=>g.maximum_items=0,1=>g.maximum_copy_bytes=n-1,2=>g.maximum_capacity_bytes=n-1,3=>g.maximum_release_bytes=n-1,_=>g.maximum_depth=n-1};let fuel=caller.fuel_remaining();let cumulative=caller.cumulative_receipt();let(r,h)=observe(||caller.turn(g,|cx|turn(&mut job,&body,cx)));assert_eq!(r.unwrap(),Default::default());assert_eq!(h,(0,0));assert_eq!(fuel,caller.fuel_remaining());assert_eq!(cumulative,caller.cumulative_receipt());assert_eq!(before,(job.progress(),job.same_stage,job.same_cursor,job.same_samples.len(),job.same_midpoints.len(),job.same_sort_cursor,job.issues.len()));denied+=1;}
   for(fuel,deadline)in[(0,1000000),(1,1000)]{let original=caller.fuel_remaining();let(r,h)=observe(||caller.turn(policy,|cx|cx.with_original_work_loan(fuel,deadline,|same|turn(&mut job,&body,same))));assert_eq!(r.unwrap(),Default::default());assert_eq!(h,(0,0));assert_eq!(original,caller.fuel_remaining());}
   SAME_PARAMETER_SAMPLES.with(|value|value.set(0));let(r,h)=observe(||caller.turn(policy,|cx|{if recipient==0{recipient=cx.original_recipient_identity()}assert_eq!(recipient,cx.original_recipient_identity());turn(&mut job,&body,cx)}));let r=r.unwrap();assert_ne!(r,Default::default());assert_eq!(r,caller.receipt());assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));let actual=SAME_PARAMETER_SAMPLES.with(std::cell::Cell::get);assert!(actual<=1);samples+=actual;born+=h.0;events+=1;assert!(events<10000);assert_eq!(&body as*const _,source);
  }
  if stop.is_null(){assert_eq!(job.phase,BodyValidationPhase::ShellClosure);if malformed{let issue=job.issues.iter().find(|issue|issue.code=="same-parameter-violated").unwrap();assert_eq!(issue.entity,format!("coedge-{}",coedge.raw_index()));assert!(issue.message.starts_with("pcurve and 3D curve disagree by 1 at s=1 "));assert_eq!(samples,fixture["sameParameterProbe"]["totalBodySamples"].as_u64().unwrap()as usize);}else{assert!(job.issues.is_empty());assert_eq!(samples,12*17);}assert!(born>0);}
  semio_framework_async::poll::resolve_ready(caller.cancel().cancel());let close_policy=Grant{maximum_copy_bytes:law["retirementCopyBytes"].as_u64().unwrap()as usize,..grant(&law["policy"])};let mut source=Some(job);
  let(r,h)=observe(||caller.turn(close_policy,|cx|{cx.consume_retained(Progress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);match ControlledRetirement::new(source.take().unwrap()){Ok(close)=>Ok(close),Err((error,original))=>{source=Some(original);Err(error.into())}}}));let mut close=r.unwrap();assert!(source.is_none());assert_eq!(h,(0,0));assert_eq!(caller.receipt(),Progress{copied_items:1,..Default::default()});
  while !close.terminal_is_empty(){let(r,h)=observe(||caller.turn(close_policy,|cx|{let g=cx.retained_grant();let d=close.selected_demand(g.maximum_copy_bytes)?;assert!(d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth);cx.consume_fuel(1);let Some((result,work))=cx.with_original_effect(g,|_,effect| validation_close_turn(&mut close,effect))? else {return Ok(Default::default());};cx.consume_work(work);let r=result.map_err(|error|OriginalTurnFailure::debit_original(error.with_retained_progress(close.step_progress()),cx))?.progress();cx.consume_retained(r)?;Ok(r)}));let r=r.unwrap();assert_eq!(r,caller.receipt());assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));}
  let(_,h)=observe(||drop(close));assert_eq!(h,(0,0));let cumulative=caller.cumulative_receipt();let roots=caller.born_bytes();assert_eq!(cumulative.retained_capacity_bytes,cumulative.released_bytes+roots);let(released,h)=observe(||caller.retire(close_policy));assert_eq!(released,roots);assert_eq!(h,(0,roots));
 }}}
}

#[test]
fn retained_shell_traversal_agrees_with_independent_face_incidence() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    for (case, code) in [("closedBoundaryEdges", "shell-not-closed"), ("missingFaceBoundaryEdges", "shell-not-closed"), ("reversedFaceOrientationEdges", "orientation-inconsistent")] {
        let mut body = Body::new();
        let solid = build_tetrahedron(&mut body, &mut OpRecorder::new());
        let face = body.solid_faces(solid)[0];
        let shell = body.shells.iter().next().unwrap().0;
        if case == "missingFaceBoundaryEdges" { body.shells.get_mut(shell).unwrap().faces.retain(|id| *id != face); }
        if case == "reversedFaceOrientationEdges" { let row = body.faces.get_mut(face).unwrap(); row.flipped = !row.flipped; }
        let mut job = BodyValidationJob::new(&body);
        job.live_coedges = body.coedges.iter().count();
        job.phase = BodyValidationPhase::ShellClosure;
        let mut observed = 0;
        while job.phase == BodyValidationPhase::ShellClosure {
            job.probe_shell_closure_funded(&body,None).unwrap();
            if let Some(finding) = job.loop_findings[0].take() { assert_eq!(finding.code(), code); observed += 1; }
        }
        assert_eq!(observed, fixture["shellClosure"][case].as_u64().unwrap());
        let mut reference = Vec::new();
        check_shell_closure_and_orientation(&body, &mut reference);
        assert_eq!(observed as usize, reference.iter().filter(|issue| issue.code == code).count());
    }
}

#[test]
fn original_preview_validation_reaches_the_planar_gate() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let mut caller = validation_fixture_caller();
    for reversed in [false, true] {
        let mut faces: [[usize;3];4] = serde_json::from_value(fixture["faces"].clone()).unwrap();
        if reversed { for face in &mut faces { face.swap(1, 2); } }
        let mut body = Body::new();
        build_tetrahedron_with_windings(&mut body, &mut OpRecorder::new(), faces);
        let mut job = BodyValidationJob::for_preview_gate(&body);
        let mut turns = 0;
        while !job.is_complete() {
            let d = job.funded_demands(&body).unwrap();
            let grant = ValidationGrant { maximum_items: 1, maximum_copy_bytes: d.copy_bytes, maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: d.release_bytes, maximum_depth: d.depth };
            let ((receipt, units), heap) = observed_validation_turn(&mut caller, grant, |effect| job.step_funded(&body, 1, effect));
            assert_eq!(units, 1);
            assert_eq!(heap, (receipt.retained_capacity_bytes, receipt.released_bytes));
            turns += 1;
            assert!(turns < 50_000);
        }
        assert_eq!(job.passed(), !reversed);
        let volume = job.shells[0].total.unwrap();
        let expected = fixture["expected"]["volume"].as_f64().unwrap();
        assert!((volume - if reversed { -expected } else { expected }).abs() < 1e-12);
        assert_eq!(job.issues.iter().any(|issue| issue.code == "shell-orientation-inward"), reversed);
    }
    caller.retire_exact();
}

#[test]
fn scoped_open_shells_keep_free_edges_and_solids_require_closed_boundaries(){
    use semio_framework_3d_brep_representation::topology::EntityRef;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let mut body=Body::new();
    let solid=build_tetrahedron(&mut body,&mut OpRecorder::new());
    let shell=body.solids.get(solid).unwrap().outer;
    body.shells.get_mut(shell).unwrap().faces.remove(0);
    let mut caller=validation_fixture_caller();
    for (root,key) in [(EntityRef::Shell(shell),"openShellPasses"),(EntityRef::Solid(solid),"openSolidPasses")]{
        let scope=body.reachable_from(&[root]);
        let mut job=BodyValidationJob::for_preview_gate(&body);
        for _ in 0..50_000{
            if job.is_complete(){break;}
            let demand=job.funded_demands_for(&body,Some(&scope)).unwrap();
            let grant=ValidationGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,
                maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let ((receipt,units),heap)=observed_validation_turn(&mut caller,grant,|effect|job.step_funded_for(&body,Some(&scope),1,effect));
            assert_eq!(units,1);
            assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));
        }
        assert!(job.is_complete());
        assert_eq!(job.passed(),fixture["shellClosure"][key].as_bool().unwrap(),"{key}");
        if !job.passed(){assert_eq!(job.issue(0).unwrap().code,"shell-not-closed");}
    }
    caller.retire_exact();
}

#[test]
fn original_triangular_face_intersections_match_the_neutral_contract(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/triangular-intersection.json")).unwrap();
    let points=|row:&serde_json::Value|{
        let coordinates:[[f64;3];3]=serde_json::from_value(row.clone()).unwrap();
        coordinates.map(|[x,y,z]|Pnt3::new(x,y,z))
    };
    let a=points(&fixture["triangle"]);
    for row in fixture["cases"].as_array().unwrap(){
        let b=points(&row["triangle"]);
        let expected=row["intersects"].as_bool().unwrap();
        assert_eq!(triangular_intersection::triangles_intersect(a,b),expected);
        assert_eq!(triangular_intersection::triangles_intersect(b,a),expected);
        assert_eq!(triangular_intersection::triangles_intersect([a[1],a[0],a[2]],b),expected);
    }
}


#[test]
fn original_same_parameter_accepts_rational_surface_curve_and_trim() {
    use semio_framework_3d_brep_curve::{Curve2,bspline::KnotVector};
    use semio_framework_3d_brep_vector::Pnt2;
    let f:serde_json::Value=serde_json::from_str(include_str!("../../../../📸️representation/🏄️surface/🧪️tests/🧫️fixtures/rational-patch.json")).unwrap();
    let xyz:Vec<Vec<[f64;3]>>=serde_json::from_value(f["controls"].clone()).unwrap();
    let weights:Vec<Vec<f64>>=serde_json::from_value(f["weights"].clone()).unwrap();
    let u=KnotVector::new(serde_json::from_value(f["uKnots"].clone()).unwrap(),2,3).unwrap();
    let v=KnotVector::new(serde_json::from_value(f["vKnots"].clone()).unwrap(),1,2).unwrap();
    let point=|p:[f64;3]|Pnt3::new(p[0],p[1],p[2]);
    let curve=Curve3::Nurbs{knots:u.clone(),controls:xyz.iter().map(|row|point(row[0])).collect(),weights:weights.iter().map(|row|row[0]).collect()};
    let surface=Surface::Nurbs{u_knots:u,v_knots:v.clone(),controls:xyz.into_iter().map(|row|row.into_iter().map(point).collect()).collect(),weights};
    let trim=Curve2::Nurbs{knots:v,controls:vec![Pnt2::new(0.0,0.0),Pnt2::new(1.0,0.0)],weights:vec![1.0,1.0]};
    for i in 0..=16 {
        assert!(retained_parameter::funded_deviation(&surface,&trim,&curve,(0.0,1.0),(0.0,1.0),i as f64/16.0).unwrap()<1e-12);
    }
}

#[test]
fn original_scoped_validation_ignores_an_unrelated_inward_solid() {
    use semio_framework_3d_brep_representation::topology::EntityRef;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();
    let law=&fixture["scopeSelection"];
    let faces:[[usize;3];4]=serde_json::from_value(fixture["faces"].clone()).unwrap();
    let mut body=Body::new();
    let mut rec=OpRecorder::new();
    let outward=build_tetrahedron_with_windings(&mut body,&mut rec,faces);
    let inward=build_tetrahedron_with_windings(&mut body,&mut rec,faces.map(|[a,b,c]|[a,c,b]));
    let mut caller=validation_fixture_caller();
    for (id,passes) in [(outward,law["selectedPasses"].as_bool().unwrap()),(inward,law["unrelatedPasses"].as_bool().unwrap())]{
        let scope=body.reachable_from(&[EntityRef::Solid(id)]);
        let mut job=BodyValidationJob::for_preview_gate(&body);
        let mut turns=0;
        while !job.is_complete(){
            let demand=job.funded_demands_for(&body,Some(&scope)).unwrap();
            let grant=ValidationGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,
                maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let ((receipt,units),heap)=observed_validation_turn(&mut caller,grant,|effect|job.step_funded_for(&body,Some(&scope),1,effect));
            assert_eq!(units,1);
            assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));
            turns+=1;
            assert!(turns<50_000);
        }
        assert_eq!(job.passed(),passes);
        assert_eq!(job.shells.len(),1);
        if !passes{
            let message=job.take_first_issue_message().unwrap();
            assert!(!message.is_empty());
        }
    }
    caller.retire_exact();
}

#[test]
fn original_solid_healing_validates_private_coordinates_before_funded_commit() {
    use semio_framework_3d_brep_construction::primitives::make_box;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_value::{RetainedWorkPoll, retirement::controlled::ControlledRetirement};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️retained-heal.json")).unwrap();
    let axes = |key: &str| serde_json::from_value::<[f64; 3]>(fixture[key].clone()).unwrap();
    let [x, y, z] = axes("original");
    let [dx, dy, dz] = axes("shift");
    let [width, depth, height] = axes("size");
    let restored = Pnt3::new(x, y, z);
    let displaced = Pnt3::new(x + dx, y + dy, z + dz);
    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let selected = make_box(&mut body, width, depth, height, &mut recorder).unwrap();
    let unrelated = make_box(&mut body, width, depth, height, &mut recorder).unwrap();
    let find_corner = |solid| body.reachable_from(&[EntityRef::Solid(solid)]).vertices.iter()
        .copied().find(|vertex| body.vertices.get(*vertex).unwrap().position == restored).unwrap();
    let selected_corner = find_corner(selected);
    let unrelated_corner = find_corner(unrelated);
    body.vertices.get_mut(selected_corner).unwrap().position = displaced;
    body.vertices.get_mut(unrelated_corner).unwrap().position = displaced;

    let revision = (0, 0, body.labels.next());
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    let mut job = SolidHealing::new(&body, selected, revision, false, tolerance).unwrap();
    let mut caller = validation_fixture_caller();
    let grant_for = |demand: semio_framework_value::RetirementDemand| ValidationGrant {
        maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth,
    };
    let mut turns = 0;
    while !job.ready() {
        let copy = job.demand(&body, revision, 0).unwrap().copy_bytes;
        let grant = grant_for(job.demand(&body, revision, copy).unwrap());
        let ((receipt, _), heap) = observed_validation_turn(&mut caller, grant,
            |effect| job.step(&body, revision, effect).map(|receipt| (receipt, 1)));
        assert!(receipt.fits(grant));
        assert_eq!(heap, (receipt.retained_capacity_bytes, receipt.released_bytes));
        assert_eq!(body.vertices.get(selected_corner).unwrap().position, displaced);
        turns += 1;
        assert!(turns < 10000);
    }

    let grant = grant_for(job.demand(&body, revision, 0).unwrap());
    let weak = ValidationGrant { maximum_copy_bytes: grant.maximum_copy_bytes - 1, ..grant };
    let declined = caller.with_effect(weak, |effect| job.commit(&mut body, revision, effect).unwrap()).ok().unwrap();
    assert_eq!(declined, (false, false, Default::default()));
    let (ready, changed, receipt) = caller.with_effect(grant, |effect| job.commit(&mut body, revision, effect).unwrap()).ok().unwrap();
    assert!(ready && changed && receipt.fits(grant));
    assert_eq!(body.vertices.get(selected_corner).unwrap().position, restored);
    assert_eq!(body.vertices.get(unrelated_corner).unwrap().position, displaced);

    let mut close = ControlledRetirement::new(job).unwrap_or_else(|_| panic!("expected funded healing retirement"));
    while !close.terminal_is_empty() {
        let demand = close.next_demand(0).and_then(|first| close.next_demand(first.copy_bytes)).unwrap();
        let grant = grant_for(demand);
        let result = caller.with_effect(grant, |effect| close.step(grant.maximum_copy_bytes, effect)).ok().unwrap();
        if let RetainedWorkPoll::Complete(turn) = result { assert!(turn.result.is_ok()); }
    }
    caller.retire_exact();
}
