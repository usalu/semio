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
        // 🩺 A p-curve is always parametrized in the EDGE's own curve order — never reversed
        // to match a particular coedge's traversal direction (W1-E's binding convention, see
        // 📓️w1e-primitives.md §"p-curve convention"): both coedges sharing an edge get the
        // IDENTICAL `(pcurve, prange)`. `pcurve`/`local_o`/`local_d` above are built to trace
        // A→B exactly as `curve3` does over `edge.range = (0.0, 1.0)`, so `prange` must match
        // that same order regardless of `co.forward` — reversing it here (the previous bug)
        // desynchronized `check_same_parameter`'s per-`s` correspondence for every backward
        // coedge, by a full edge length at each sample's far end.
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
    // Corrupt the ring: point the first coedge's `next` at itself instead of its real neighbor.
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
    // Build a free-standing edge with three coedges referencing it (impossible in a clean
    // 2-manifold build, so constructed directly).
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
    // Attach a pcurve that does NOT correspond to the face's surface at all — a constant,
    // clearly-wrong 2D point far from where the 3D edge actually projects.
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
    eprintln!("[DEBUG] original adaptive same-parameter samples={samples} worstDeviation={deviation} worstParameter={parameter} maximumPerTurn=1");
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
    eprintln!("[DEBUG] cold validation original volume={volume} issues={}", issues.len());
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
    eprintln!("[DEBUG] original validation loop probes={probes} maximumPerTurn={maximum}");
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
    eprintln!("[DEBUG] original validation valence probes={probes} maximumPerTurn={maximum}");
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
    eprintln!("[DEBUG] original validation pcurve presence probes={probes} maximumPerTurn={maximum}");
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
    eprintln!("[DEBUG] original validation tolerance probes={probes} maximumPerTurn={maximum}");
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
    eprintln!("[DEBUG] original validation same-parameter samples={samples} maximumPerTurn={maximum}");
}

#[test]
fn original_validation_planning_quotes_actual_backing_before_entity_copy() {
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
            let(result,heap)=observe(||job.step_planning_funded(&body,0,grant).unwrap());assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));
            for body_bytes in law["bodyBytes"].as_array().unwrap(){let body_bytes=body_bytes.as_u64().unwrap()as usize;if body_bytes<demand.copy_bytes{let(result,heap)=observe(||job.step_planning_funded(&body,1,RetainedCloneGrant{maximum_copy_bytes:body_bytes,..grant}).unwrap());assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));denials+=1;}}
            for axis in 0..5{
                let amount=match axis{0=>1,1=>demand.copy_bytes,2=>demand.capacity_bytes,3=>demand.release_bytes,_=>demand.depth};if amount==0{continue;}
                let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1};
                let before=(job.units_done,job.planning_stage,job.planning_slot,job.edges.len(),job.faces.len(),job.shells.len());
                let(result,heap)=observe(||job.step_planning_funded(&body,1,denied).unwrap());assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));assert_eq!(before,(job.units_done,job.planning_stage,job.planning_slot,job.edges.len(),job.faces.len(),job.shells.len()));denials+=1;
            }
            let((receipt,units),heap)=observe(||job.step_planning_funded(&body,1,grant).unwrap());assert!(receipt.fits(grant));assert_eq!(units,1);assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;reserves+=usize::from(heap.0!=0);copies+=usize::from(receipt.copied_bytes!=0);assert_eq!(body.edges.slot_at(0).unwrap().1 as *const _,original);turns+=1;assert!(turns<10000);
        }
        if stop.is_null(){assert_eq!(job.edges.len(),fixture["expected"]["edges"].as_u64().unwrap()as usize*count);assert_eq!(job.faces.len(),fixture["expected"]["faces"].as_u64().unwrap()as usize*count);assert_eq!(job.shells.len(),count);assert!(job.edges.iter().copied().eq(body.edges.iter().map(|(id,_)|id)));assert!(job.faces.iter().copied().eq(body.faces.iter().map(|(id,_)|id)));for shell in job.shells.iter(){assert_eq!(shell.faces,body.shells.get(shell.shell).unwrap().faces);}assert!(reserves>0&&copies>0);}
        let edge_pointer=job.edges.as_ptr();let(mut closing,heap)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original validation planning close")));assert_eq!(heap,(0,0));assert_eq!(closing.original().unwrap().edges.as_ptr(),edge_pointer);
        while !closing.terminal_is_empty(){let copy=closing.next_copy_byte_demand().unwrap();let demand=closing.selected_demand(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,heap)=observe(||closing.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
        let(_,heap)=observe(||drop(closing));assert_eq!(heap,(0,0));assert_eq!(born,freed);eprintln!("[DEBUG] Original validation planning sourceTetrahedra={count} stop={stop} sourcePointer=true turns={turns} deniedAxes={denials} reserves={reserves} copies={copies} born={born} released={freed} terminalDrop=0");
    }
    }
}


#[test]
fn original_validation_loop_frontier_quotes_source_and_diagnostic_events(){
    use crate::test_support::observe_tessellation_system as observe;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();let law=&fixture["retainedLoops"];
    for count in law["sourceTetrahedra"].as_array().unwrap(){let count=count.as_u64().unwrap()as usize;for broken in [false,true]{for stop in law["cancelStops"].as_array().unwrap(){
        let mut body=Body::new();let mut rec=OpRecorder::new();for _ in 0..count{build_tetrahedron(&mut body,&mut rec);}
        if broken{let first=body.loops.slot_at(0).unwrap().1.first;let next=body.coedges.get(first).unwrap().next;body.coedges.get_mut(next).unwrap().prev=next;}
        let source=&body as*const _;let mut job=BodyValidationJob::new(&body);let(mut born,mut freed)=(0,0);
        while job.phase==BodyValidationPhase::CollectingTopology{let demand=job.funded_demands(&body).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let((receipt,_),heap)=observe(||job.step_funded(&body,1,grant).unwrap());assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;}
        let(mut turns,mut denies,mut backing,mut texts)=(0,0,0,0);
        while job.phase==BodyValidationPhase::LoopRings&&stop.as_u64().is_none_or(|stop|turns<stop as usize){
            let(demand,heap)=observe(||job.funded_demands(&body).unwrap());assert_eq!(heap,(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let before=(job.progress(),job.loop_slot,job.ring.as_ptr(),job.ring.len(),job.ring_probe,job.loop_finding_cursor,job.issues.len());
            for axis in 0..5{let amount=match axis{0=>1,1=>demand.copy_bytes,2=>demand.capacity_bytes,3=>demand.release_bytes,_=>demand.depth};if amount==0{continue}let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1};let(result,heap)=observe(||job.step_funded(&body,1,denied).unwrap());assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));assert_eq!(before,(job.progress(),job.loop_slot,job.ring.as_ptr(),job.ring.len(),job.ring_probe,job.loop_finding_cursor,job.issues.len()));denies+=1;}
            let(result,heap)=observe(||job.step_funded(&body,0,grant).unwrap());assert_eq!(result,(Default::default(),0));assert_eq!(heap,(0,0));
            let((receipt,units),heap)=observe(||job.step_funded(&body,1,grant).unwrap());assert_eq!(units,1);assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;backing+=usize::from(heap.0!=0);texts+=usize::from(receipt.copied_bytes==1);turns+=1;assert!(turns<100000);assert_eq!(&body as*const _,source);
        }
        if stop.is_null(){assert_eq!(job.phase,BodyValidationPhase::EdgeValence);assert!(backing>0);if broken{let finding=job.issues.iter().next().unwrap();assert_eq!(job.issues.len(),1);assert_eq!(finding.entity,law["brokenNeighbor"]["entity"]);assert_eq!(finding.code,law["brokenNeighbor"]["code"]);assert_eq!(finding.message,law["brokenNeighbor"]["message"]);assert!(texts>0);}else{assert!(job.issues.is_empty());}}
        let pointer=job.ring.as_ptr();let(mut close,heap)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original loop custody")));assert_eq!(heap,(0,0));assert_eq!(close.original().unwrap().ring.as_ptr(),pointer);
        while !close.terminal_is_empty(){let copy=close.next_copy_byte_demand().unwrap();let demand=close.selected_demand(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,heap)=observe(||close.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
        let(_,heap)=observe(||drop(close));assert_eq!(heap,(0,0));assert_eq!(born,freed);eprintln!("[DEBUG] Original validation loops sourceTetrahedra={count} broken={broken} stop={stop} turns={turns} denials={denies} backingEvents={backing} textEvents={texts} born={born} released={freed} terminalDrop=0");
    }}}
}


#[test]
fn original_validation_topology_frontier_quotes_each_probe_and_utf8_finding(){
 use crate::test_support::observe_tessellation_system as observe;
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();let law=&fixture["retainedTopology"];
 for malformed in [false,true]{for stop in law["cancelStops"].as_array().unwrap(){
  let mut body=Body::new();build_tetrahedron(&mut body,&mut OpRecorder::new());
  if malformed{let id=body.coedges.slot_at(0).unwrap().0;body.coedges.get_mut(id).unwrap().pcurve=None;let id=body.vertices.slot_at(0).unwrap().0;body.vertices.get_mut(id).unwrap().tol=Tol::new(10.0);}
  let mut job=BodyValidationJob::new(&body);let(mut born,mut freed,mut turns,mut denials,mut utf8)=(0,0,0,0,0);
  while job.phase!=BodyValidationPhase::EdgeValence{let d=job.funded_demands(&body).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let((r,_),h)=observe(||job.step_funded(&body,1,g).unwrap());assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));born+=h.0;freed+=h.1;}
  while job.phase!=BodyValidationPhase::SameParameter&&stop.as_u64().is_none_or(|stop|turns<stop as usize){
   let(d,h)=observe(||job.funded_demands(&body).unwrap());assert_eq!(h,(0,0));let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
   let before=(job.progress(),job.valence_edge_slot,job.valence_coedge_slot,job.pcurve_slot,job.tolerance_edge_cursor,job.tolerance_vertex,job.issues.len());
   for axis in 0..5{let n=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,3=>d.release_bytes,_=>d.depth};if n==0{continue}let mut denied=g;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1};let(r,h)=observe(||job.step_funded(&body,1,denied).unwrap());assert_eq!(r,(Default::default(),0));assert_eq!(h,(0,0));assert_eq!(before,(job.progress(),job.valence_edge_slot,job.valence_coedge_slot,job.pcurve_slot,job.tolerance_edge_cursor,job.tolerance_vertex,job.issues.len()));denials+=1;}
   for width in law["bodyBytes"].as_array().unwrap(){let width=width.as_u64().unwrap()as usize;if width<d.copy_bytes{let(r,h)=observe(||job.step_funded(&body,1,RetainedCloneGrant{maximum_copy_bytes:width,..g}).unwrap());assert_eq!(r,(Default::default(),0));assert_eq!(h,(0,0));}}
   let((r,u),h)=observe(||job.step_funded(&body,1,g).unwrap());assert_eq!(u,1);assert!(r.fits(g));assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));born+=h.0;freed+=h.1;utf8+=usize::from(r.copied_bytes==3);turns+=1;assert!(turns<10000);
  }
  if stop.is_null(){if malformed{let missing=job.issues.iter().find(|i|i.code=="missing-pcurve").unwrap();assert_eq!(missing.entity,law["missingPcurve"]["entity"]);assert_eq!(missing.message,law["missingPcurve"]["message"]);let tolerance=job.issues.iter().find(|i|i.entity==law["tolerance"]["entity"]&&i.message==law["tolerance"]["message"]).unwrap();assert_eq!(tolerance.code,law["tolerance"]["code"]);assert!(utf8>0);}else{assert!(job.issues.is_empty());}}
  let(mut close,h)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original topology custody")));assert_eq!(h,(0,0));
  while !close.terminal_is_empty(){let d=close.selected_demand(close.next_copy_byte_demand().unwrap()).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(s,h)=observe(||close.step(g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));born+=h.0;freed+=h.1;}
  let(_,h)=observe(||drop(close));assert_eq!(h,(0,0));assert_eq!(born,freed);eprintln!("[DEBUG] Original topology malformed={malformed} stop={stop} turns={turns} denied={denials} utf8={utf8} born={born} released={freed} terminalDrop0");
 }}
}

#[test]
fn original_validation_same_parameter_retains_scratch_samples_and_one_original_caller(){
 use crate::test_support::observe_tessellation_system as observe;
 use semio_framework_job::{OriginalJobFixtureCaller,OriginalTurnFailure,OperationId,Generation};
 use semio_framework_value::{retained_clone::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress},retirement::controlled::ControlledRetirement};
 fn clock()->Option<u64>{Some(1000)}
 fn grant(row:&serde_json::Value)->Grant{Grant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:row["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:row["release"].as_u64().unwrap()as usize,maximum_depth:row["depth"].as_u64().unwrap()as usize}}
 fn turn(job:&mut BodyValidationJob,body:&Body,cx:&mut semio_framework_job::StepContext<'_>)->Result<Progress,OriginalTurnFailure>{
  let g=cx.retained_grant();if cx.should_yield()||g.maximum_items==0{return Ok(Default::default())}let d=job.funded_demands(body)?;if g.maximum_copy_bytes<d.copy_bytes||g.maximum_capacity_bytes<d.capacity_bytes||g.maximum_release_bytes<d.release_bytes||g.maximum_depth<d.depth{return Ok(Default::default())}cx.consume_fuel(1);let(r,u)=job.step_funded(body,1,g).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;assert_eq!(u,1);cx.consume_retained(r)?;Ok(r)
 }
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧊️cold-planning/🔣️.json")).unwrap();let law=&fixture["retainedSameParameter"];
 for width in [1,3,64]{for malformed in [false,true]{for stop in law["cancelStops"].as_array().unwrap(){
  let mut body=Body::new();let solid=build_tetrahedron(&mut body,&mut OpRecorder::new());let face=body.solid_faces(solid)[0];let coedge=body.face_coedges(face)[0];if malformed{let pcurve=body.coedges.get(coedge).unwrap().pcurve.unwrap();let semio_framework_3d_brep_curve::Curve2::Line{dir,..}=body.curves2.get_mut(pcurve).unwrap()else{panic!("original planar fixture")};*dir=semio_framework_3d_brep_vector::Vec2::new(0.0,0.0);}
  let source=&body as*const Body;let policy=grant(&law["policy"]);let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(37),grant(&law["stock"]),clock,law["deadline"].as_u64().unwrap(),law["fuel"].as_u64().unwrap());let(mut job,h)=observe(||BodyValidationJob::new(&body));assert_eq!(h,(0,0));
  while job.phase!=BodyValidationPhase::SameParameter{let(r,h)=observe(||caller.turn(policy,|cx|turn(&mut job,&body,cx)));let r=r.unwrap();assert_ne!(r,Default::default());assert_eq!(r,caller.receipt());assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));}
  let policy=Grant{maximum_copy_bytes:width,..policy};let mut samples=0;let mut events=0;let mut born=0;let mut denied=0;let mut recipient=0;
  while job.phase==BodyValidationPhase::SameParameter&&stop.as_u64().is_none_or(|stop|events<stop as usize){
   let(d,h)=observe(||job.funded_demands(&body).unwrap());assert_eq!(h,(0,0));let before=(job.progress(),job.same_stage,job.same_cursor,job.same_samples.len(),job.same_midpoints.len(),job.same_sort_cursor,job.issues.len());
   for axis in [0,1,2,3,4]{let n=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,3=>d.release_bytes,_=>d.depth};if n==0{continue}let mut g=policy;match axis{0=>g.maximum_items=0,1=>g.maximum_copy_bytes=n-1,2=>g.maximum_capacity_bytes=n-1,3=>g.maximum_release_bytes=n-1,_=>g.maximum_depth=n-1};let fuel=caller.fuel_remaining();let cumulative=caller.cumulative_receipt();let(r,h)=observe(||caller.turn(g,|cx|turn(&mut job,&body,cx)));assert_eq!(r.unwrap(),Default::default());assert_eq!(h,(0,0));assert_eq!(fuel,caller.fuel_remaining());assert_eq!(cumulative,caller.cumulative_receipt());assert_eq!(before,(job.progress(),job.same_stage,job.same_cursor,job.same_samples.len(),job.same_midpoints.len(),job.same_sort_cursor,job.issues.len()));denied+=1;}
   for(fuel,deadline)in[(0,1000000),(1,1000)]{let original=caller.fuel_remaining();let(r,h)=observe(||caller.turn(policy,|cx|cx.with_original_work_loan(fuel,deadline,|same|turn(&mut job,&body,same))));assert_eq!(r.unwrap(),Default::default());assert_eq!(h,(0,0));assert_eq!(original,caller.fuel_remaining());}
   SAME_PARAMETER_SAMPLES.with(|value|value.set(0));let(r,h)=observe(||caller.turn(policy,|cx|{if recipient==0{recipient=cx.original_recipient_identity()}assert_eq!(recipient,cx.original_recipient_identity());turn(&mut job,&body,cx)}));let r=r.unwrap();assert_ne!(r,Default::default());assert_eq!(r,caller.receipt());assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));let actual=SAME_PARAMETER_SAMPLES.with(std::cell::Cell::get);assert!(actual<=1);samples+=actual;born+=h.0;events+=1;assert!(events<10000);assert_eq!(&body as*const _,source);
  }
  if stop.is_null(){assert_eq!(job.phase,BodyValidationPhase::ShellClosure);if malformed{let issue=job.issues.iter().find(|issue|issue.code=="same-parameter-violated").unwrap();assert_eq!(issue.entity,format!("coedge-{}",coedge.raw_index()));assert!(issue.message.starts_with("pcurve and 3D curve disagree by 1 at s=1 "));assert_eq!(samples,fixture["sameParameterProbe"]["totalBodySamples"].as_u64().unwrap()as usize);}else{assert!(job.issues.is_empty());assert_eq!(samples,12*17);}assert!(born>0);}
  semio_framework_async::poll::resolve_ready(caller.cancel().cancel());let close_policy=grant(&law["policy"]);let mut source=Some(job);
  let(r,h)=observe(||caller.turn(close_policy,|cx|{cx.consume_retained(Progress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);match ControlledRetirement::new(source.take().unwrap()){Ok(close)=>Ok(close),Err((error,original))=>{source=Some(original);Err(error.into())}}}));let mut close=r.unwrap();assert!(source.is_none());assert_eq!(h,(0,0));assert_eq!(caller.receipt(),Progress{copied_items:1,..Default::default()});
  while !close.terminal_is_empty(){let(r,h)=observe(||caller.turn(close_policy,|cx|{let g=cx.retained_grant();let d=close.selected_demand(g.maximum_copy_bytes)?;assert!(d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth);cx.consume_fuel(1);let r=close.step(g).map_err(|error|OriginalTurnFailure::debit_original(error.with_retained_progress(close.step_progress()),cx))?.progress();cx.consume_retained(r)?;Ok(r)}));let r=r.unwrap();assert_eq!(r,caller.receipt());assert_eq!(h,(r.retained_capacity_bytes,r.released_bytes));}
  let(_,h)=observe(||drop(close));assert_eq!(h,(0,0));let cumulative=caller.cumulative_receipt();let roots=caller.born_bytes();assert_eq!(cumulative.retained_capacity_bytes,cumulative.released_bytes+roots);let(released,h)=observe(||caller.retire(close_policy));assert_eq!(released,roots);assert_eq!(h,(0,roots));eprintln!("[DEBUG] original same-parameter body{width} malformed={malformed} stop={stop} samples={samples} events={events} denied={denied} SAME caller and actual scratch conservation/Drop0");
 }}}
}
