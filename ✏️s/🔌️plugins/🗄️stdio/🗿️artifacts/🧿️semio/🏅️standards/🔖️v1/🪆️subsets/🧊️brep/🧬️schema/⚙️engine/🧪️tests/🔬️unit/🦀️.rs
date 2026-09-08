
use super::*;

#[test]
fn brep_error_contract_is_owned_and_stable() {
    let errors = [(BrepError::InvalidInput("mesh".into()), "invalid input: mesh"), (BrepError::MissingHandle("a1".into()), "missing handle: a1"), (BrepError::Operation("split".into()), "operation failed: split")];
    for (error, message) in errors {
        assert_eq!(error.to_string(), message);
        assert!(std::error::Error::source(&error).is_none());
    }
}

#[semio_framework_async_macros::async_test]
async fn native_box_volume() {
    let mut k = Brep::new();
    let solid = k.box_prim(1.0, 1.0, 1.0).unwrap();
    let v = k.volume(&solid).unwrap();
    assert!((v - 1.0).abs() < 1e-3, "volume {v}");
}

#[semio_framework_async_macros::async_test]
async fn native_fuse_disjoint() {
    let mut k = Brep::new();
    let a = k.box_prim(1.0, 1.0, 1.0).unwrap();
    let b = k.convex_hull(&[[2.0, 0.0, 0.0], [3.0, 0.0, 0.0], [3.0, 1.0, 0.0], [2.0, 1.0, 0.0], [2.0, 0.0, 1.0], [3.0, 0.0, 1.0], [3.0, 1.0, 1.0], [2.0, 1.0, 1.0]]).unwrap();
    let u = k.fuse(&a, &b).unwrap();
    let v = k.volume(&u).unwrap();
    assert!((v - 2.0).abs() < 1e-2, "volume {v}");
}

#[semio_framework_async_macros::async_test]
async fn wire_tessellate_preserves_edge_positions() {
    let mut k = Brep::new();
    let wire = k.rectangle_wire(2.0, 1.5).expect("wire");
    let transfer = k.tessellate_sync(&wire, 0.1).expect("tessellate");
    let data = mesh_data_from_mesh_transfer(&transfer);
    assert!(data.edge_positions.len() >= 24, "edge_positions {}", data.edge_positions.len());
    assert!(data.indices.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn box_shell_produces_positive_volume() {
    let mut k = Brep::new();
    let box_h = k.box_prim(2.0, 2.0, 2.0).expect("box");
    let shelled = k.shell(&box_h, 0.2, &[]).expect("shell");
    let vol = k.volume(&shelled).expect("shell volume");
    assert!(vol > 0.0, "shelled volume {vol}");
    let mesh = k.tessellate_sync(&shelled, 0.1).expect("tessellate shell");
    assert!(!mesh.position.is_empty() || !mesh.edges.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn sphere_torus_cut_produces_preview_mesh() {
    let mut k2 = Brep::new();
    let sphere = k2.sphere_prim(2.2).expect("sphere");
    let torus = k2.torus_prim(2.0, 0.5).expect("torus");
    let tv = k2.volume(&torus).expect("torus volume");
    assert!(tv > 0.5, "torus volume too small: {tv}");
    let tmesh = k2.tessellate_sync(&torus, 0.15).expect("tessellate torus");
    assert!(tmesh.position.len() >= 9 && tmesh.index.len() >= 3, "torus mesh empty");
    let cut = k2.cut(&sphere, &torus).expect("cut");
    let mesh = k2.tessellate_sync(&cut, 0.15).expect("tessellate cut");
    assert!(mesh.position.len() >= 9 && mesh.index.len() >= 3, "cut mesh empty: pos={} idx={}", mesh.position.len(), mesh.index.len());
}

#[semio_framework_async_macros::async_test]
async fn arc_curve_respects_start_end_angles() {
    let mut k = Brep::new();
    let start = 0.0;
    let end = std::f64::consts::FRAC_PI_2;
    let radius = 2.0;
    let arc = k.arc_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], radius, start, end).expect("arc");
    let domain = k.curve_domain_sync(&arc).expect("domain");
    assert!((domain.min - start).abs() < 1e-9 && (domain.max - end).abs() < 1e-9);
    let p0 = k.curve_point_sync(&arc, start).expect("p0");
    let p1 = k.curve_point_sync(&arc, end).expect("p1");
    let r0 = (p0[0] * p0[0] + p0[1] * p0[1] + p0[2] * p0[2]).sqrt();
    let r1 = (p1[0] * p1[0] + p1[1] * p1[1] + p1[2] * p1[2]).sqrt();
    assert!((r0 - radius).abs() < 1e-4, "start radius {r0} from {p0:?}");
    assert!((r1 - radius).abs() < 1e-4, "end radius {r1} from {p1:?}");
    let chord = ((p1[0] - p0[0]).powi(2) + (p1[1] - p0[1]).powi(2) + (p1[2] - p0[2]).powi(2)).sqrt();
    let expected_chord = (2.0 * radius * radius * (1.0 - (end - start).cos())).sqrt();
    assert!((chord - expected_chord).abs() < 1e-3, "chord {chord} expected {expected_chord}");
    // Full circle would land start≈end; a quarter arc must keep endpoints distinct.
    assert!(chord > radius * 0.5);
    let kappa = k.curve_curvature_sync(&arc, (start + end) * 0.5).expect("kappa");
    assert!((kappa - 1.0 / radius).abs() < 5e-2, "kappa {kappa}");
}

#[semio_framework_async_macros::async_test]
async fn solid_face_loops_returns_a_quad_per_box_face() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(2.0, 3.0, 4.0).expect("box");
    let (positions, face_loops) = k.solid_face_loops_sync(&solid).expect("loops");
    assert_eq!(positions.len(), 8, "a box has 8 distinct vertices");
    assert_eq!(face_loops.len(), 6, "a box has 6 faces");
    for (outer, holes) in &face_loops {
        assert_eq!(outer.len(), 4, "each box face is a quad");
        assert!(holes.is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn validate_returns_structured_json_report() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let report = k.validate_sync(&solid).expect("validate");
    let value: serde_json::Value = serde_json::from_str(&report).expect("json");
    assert_eq!(value["ok"], true);
    assert_eq!(value["issueCount"], 0);
    assert!(value["issues"].as_array().unwrap().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn deconstruct_includes_vertices_edges_and_faces() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let topo = k.deconstruct_sync(&solid).expect("deconstruct");
    assert_eq!(topo.faces.len(), 6);
    assert_eq!(topo.edges.len(), 12);
    assert_eq!(topo.vertices.len(), 8);
}

/// 🏷️ Deconstruct must be idempotent: minting from each entity's [`PersistentLabel`] (not a
/// session counter) means calling it twice on the same untouched shape yields byte-identical
/// handles, and shells are now included (audit §5.4 — shell was not a first-class handle kind).
#[semio_framework_async_macros::async_test]
async fn deconstruct_twice_yields_identical_handles_and_includes_shells() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let first = k.deconstruct_sync(&solid).expect("first deconstruct");
    let second = k.deconstruct_sync(&solid).expect("second deconstruct");
    assert_eq!(first.shells.len(), 1, "a box has exactly one outer shell");
    assert_eq!(first.vertices, second.vertices, "deconstruct must be idempotent per PersistentLabel");
    assert_eq!(first.edges, second.edges);
    assert_eq!(first.faces, second.faces);
    assert_eq!(first.shells, second.shells);
}

/// 🏷️ Registering unrelated geometry between the two calls must not perturb a single handle —
/// under the old counter-based `mint`, this alone would have changed every handle the second
/// `deconstruct` produces (audit §5.1: "deconstructing the same body repeatedly can mint new
/// handles repeatedly").
#[semio_framework_async_macros::async_test]
async fn deconstruct_handles_are_unaffected_by_unrelated_registrations_between_calls() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let before = k.deconstruct_sync(&solid).expect("deconstruct before");
    let _ = k.line_curve_sync([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).expect("unrelated curve");
    let _ = k.sphere_prim_sync(0.5).expect("unrelated sphere");
    let after = k.deconstruct_sync(&solid).expect("deconstruct after");
    assert_eq!(before.vertices, after.vertices);
    assert_eq!(before.edges, after.edges);
    assert_eq!(before.faces, after.faces);
    assert_eq!(before.shells, after.shells);
}

/// ♻️ Disposing the only handle reaching a body's geometry must actually free it (audit §5.3:
/// "dispose is not equivalent to deleting geometry" under the old registry-only dispose).
#[semio_framework_async_macros::async_test]
async fn dispose_reclaims_unreferenced_topology() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    k.dispose(&solid);
    assert_eq!(k.registry_len(), 0);
    let counts = k.body.entity_counts();
    assert_eq!(counts.vertices, 0);
    assert_eq!(counts.edges, 0);
    assert_eq!(counts.faces, 0);
    assert_eq!(counts.shells, 0);
    assert_eq!(counts.solids, 0);
}

/// ♻️ `retain` is dispose-of-everything-else-then-compact: the dropped solid's own geometry
/// must be freed while the kept solid stays fully intact and resolvable.
#[semio_framework_async_macros::async_test]
async fn retain_compacts_everything_not_kept() {
    let mut k = Brep::new();
    let keep = k.box_prim_sync(1.0, 1.0, 1.0).expect("keep");
    let drop = k.box_prim_sync(2.0, 2.0, 2.0).expect("drop");
    let mut live = std::collections::HashSet::new();
    live.insert(keep.as_str().to_string());
    k.retain(&live);
    assert_eq!(k.registry_len(), 1);
    assert!(k.kind(&keep).is_ok());
    assert!(k.kind(&drop).is_err(), "the dropped handle must no longer resolve");
    let vol = k.volume(&keep).expect("kept solid stays intact");
    assert!((vol - 1.0).abs() < 1e-6, "volume {vol}");
}

/// 🧩️ `import_step` merges into the existing body (audit §5.2's required fix) rather than
/// replacing it: a handle minted before the import must still resolve, and the original
/// solid's own geometry must be untouched by the merge.
#[semio_framework_async_macros::async_test]
async fn import_step_merges_and_keeps_prior_handles_resolvable() {
    let mut k = Brep::new();
    let original = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let step_text = k.export_step_sync(std::slice::from_ref(&original)).expect("export");
    let imported = k.import_step_sync(&step_text).expect("import");
    assert!(k.kind(&original).is_ok(), "a handle minted before import must still resolve");
    assert!((k.volume(&original).unwrap() - 1.0).abs() < 1e-6, "the original solid's own geometry must be untouched");
    assert_eq!(imported.len(), 1);
    assert!((k.volume(&imported[0]).unwrap() - 1.0).abs() < 1e-2, "the round-tripped solid should have the same volume");
}

/// 🧩️ Shell/compound are first-class handle kinds now (audit §5.4); `explode` is `compound`'s
/// inverse.
#[semio_framework_async_macros::async_test]
async fn solid_shells_compound_and_explode_round_trip() {
    let mut k = Brep::new();
    let a = k.box_prim_sync(1.0, 1.0, 1.0).expect("a");
    let b = k.box_prim_sync(1.0, 1.0, 1.0).expect("b");

    let shells = k.solid_shells(&a).expect("shells");
    assert_eq!(shells.len(), 1);
    assert_eq!(k.kind(&shells[0]).unwrap(), GeometryKind::Shell);

    let compound = k.compound(&[a.clone(), b.clone()]).expect("compound");
    assert_eq!(k.kind(&compound).unwrap(), GeometryKind::Compound);

    let members = k.explode(&compound).expect("explode");
    assert_eq!(members.len(), 2);
    for m in &members {
        assert!((k.volume(m).unwrap() - 1.0).abs() < 1e-6);
    }
}

/// 🏷️ `label`/`handle_for_label` are the ephemeral-handle↔persistent-label bridge the audit's
/// §5.5 required fix asks for.
#[semio_framework_async_macros::async_test]
async fn label_and_handle_for_label_round_trip() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let label = k.label(&solid).expect("solid must carry a label");
    let via_label = k.handle_for_label(PersistentLabel(label));
    assert_eq!(via_label, Some(solid));
}
