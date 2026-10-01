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

/// ⚖️ LAW: the validation gate judges the SHAPE it was asked about, never the arena around it.
///
/// 🐛️ It used to run `validate_body` over the whole body and block on any error-class issue found
/// anywhere, and this kernel is process-wide: one invalid solid — a boolean result whose void shell
/// is not inverted, say — refused the tessellation of every other live handle for as long as it
/// stayed live. In the procedural playground that read as six of eight examples settling on an empty
/// preview payload reported as `idle` (`📓️slider-reevaluation-correctness-2026-09-15.md`).
#[semio_framework_async_macros::async_test]
async fn the_validate_gate_judges_the_shape_it_was_asked_about_and_not_the_arena_around_it() {
    let mut kernel = Brep::new();
    let healthy = kernel.box_prim_sync(1.0, 1.0, 1.0).expect("a plain box is valid geometry");
    assert!(kernel.validate_gate_sync(&healthy).is_ok(), "a plain box must pass its own gate");
    let solid = kernel.solid_id(&healthy).expect("the box is a solid");
    let stranger = kernel.box_prim_sync(2.0, 2.0, 2.0).expect("a second box");
    let stranger_solid = kernel.solid_id(&stranger).expect("the second box is a solid");
    assert_ne!(solid, stranger_solid, "the two boxes are different solids");
    // 🧨️ Break the STRANGER's outer shell orientation, which `validate_body` reports as a blocking
    // `shell-orientation-inward` issue against that solid alone.
    let outer = kernel.body.solids.get(stranger_solid).expect("stranger solid").outer;
    let faces = kernel.body.shells.get(outer).expect("stranger shell").faces.clone();
    for face in faces {
        if let Some(entry) = kernel.body.faces.get_mut(face) {
            entry.flipped = !entry.flipped;
        }
    }
    let stranger_issues = kernel.validate_gate_sync(&stranger).expect_err("the broken box must fail its own gate");
    assert!(!stranger_issues.is_empty(), "the broken box's gate must name at least one issue");
    assert!(kernel.validate_gate_sync(&healthy).is_ok(), "the healthy box must still pass while a STRANGER is broken: {stranger_issues:?}");
    eprintln!("scoped gate: healthy=ok stranger={:?}", stranger_issues.iter().map(|issue| format!("{}:{}", issue.entity, issue.code)).collect::<Vec<_>>());
}

/// ⚖️ LAW: a boolean leaves BOTH its inputs usable — it owns copies of everything it consumes.
///
/// 🐛️ The exact imprint engine used to imprint ON the operands' own faces and then
/// `remove_solid_and_orphans` both of them, and the trivial fast paths used to alias the operands'
/// faces into a second shell. Either way the sibling input's handle named a freed arena slot the
/// moment the boolean answered, so the next evaluation of the same graph — where the unchanged
/// input node is served from the evaluator's cache — failed with `missing handle: <digest>` on a
/// node whose own parameters never moved. Witnessed as `sphere-box-fuse` and `sphere-cut-with-torus`
/// refusing to follow their slider after the first edit
/// (`📓️brep-boolean-input-lifetime-2026-09-15.md`).
#[semio_framework_async_macros::async_test]
async fn a_boolean_leaves_both_of_its_input_solids_alive_for_the_next_evaluation() {
    for op in [BooleanOp::Unite, BooleanOp::Cut, BooleanOp::Intersect] {
        let mut kernel = Brep::new();
        let sphere = kernel.sphere_prim_sync(1.2).expect("sphere");
        let cube = kernel.box_prim_sync(1.5, 1.5, 1.5).expect("box");
        let sphere_volume = kernel.volume_sync(&sphere).expect("sphere volume");
        let cube_volume = kernel.volume_sync(&cube).expect("box volume");
        let result = kernel.boolean_sync(&sphere, &cube, op).expect("the boolean itself must answer");
        let after_sphere = kernel.volume_sync(&sphere).unwrap_or_else(|error| panic!("{op:?} freed its first input: {error}"));
        let after_cube = kernel.volume_sync(&cube).unwrap_or_else(|error| panic!("{op:?} freed its second input: {error}"));
        assert!((after_sphere - sphere_volume).abs() < 1e-9, "{op:?} changed its first input: {sphere_volume} → {after_sphere}");
        assert!((after_cube - cube_volume).abs() < 1e-9, "{op:?} changed its second input: {cube_volume} → {after_cube}");
        let live: std::collections::HashSet<String> = [&sphere, &cube, &result].iter().map(|handle| handle.as_str().to_string()).collect();
        kernel.retain(&live);
        assert_eq!(kernel.registry_len(), 3, "{op:?}: the claim named three live handles and the compaction must keep all three");
        kernel.tessellate_sync(&sphere, 0.5).unwrap_or_else(|error| panic!("{op:?}: the first input must still tessellate after a compaction: {error}"));
        kernel.tessellate_sync(&cube, 0.5).unwrap_or_else(|error| panic!("{op:?}: the second input must still tessellate after a compaction: {error}"));
        kernel.tessellate_sync(&result, 0.5).unwrap_or_else(|error| panic!("{op:?}: the result must still tessellate after a compaction: {error}"));
        let again = kernel.boolean_sync(&sphere, &cube, op).unwrap_or_else(|error| panic!("{op:?}: the same graph must evaluate a second time from the same cached inputs: {error}"));
        let first = kernel.volume_sync(&result).expect("first result volume");
        let second = kernel.volume_sync(&again).expect("second result volume");
        assert!((first - second).abs() < 1e-6, "{op:?}: re-evaluating the same inputs must answer the same solid: {first} vs {second}");
    }
}

// #region 🔁️AffineTransforms
/// 🔁️ The kernel-neutral affine-transform vectors — the same rows the `brep_invoke` bridge law
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/📐️brep-invoke`), the CAD `SemioBrepKernel` and OpenCascade (the
/// third-party oracle, `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio` suite) answer.
const AFFINE_TRANSFORMS_FIXTURE: &str = include_str!("../../../🧫️fixtures/🔁️affine-transforms/🔣️.json");

/// 📐️ What the laws read off a solid: exact mass properties, the tessellation's extent, planar faces' outward normals,
/// topology counts, and the tessellated soup's own signed volume and centroid as `parry3d` (third party) integrates them.
struct AffineMeasure {
    volume: f64,
    center_of_mass: EVec3,
    min: EVec3,
    max: EVec3,
    normals: Vec<EVec3>,
    soup_volume: f64,
    soup_centroid: EVec3,
    topology: [usize; 3],
}

/// 🧮️ A fixture step as `(linear, translation)`, derived here (Rodrigues rotation, Householder reflection) independently of
/// the kernel's own `Affine3`, so the exactness law never checks the kernel against itself.
type AffineMap = ([[f64; 3]; 3], EVec3);

fn affine_fixture() -> serde_json::Value {
    let root: serde_json::Value = serde_json::from_str(AFFINE_TRANSFORMS_FIXTURE).expect("affine-transforms fixture parses");
    assert_eq!(root["schema"].as_str(), Some("semio.geometry.brep.affine-transforms/v1"), "fixture schema");
    assert!(!root["cases"].as_array().expect("cases").is_empty(), "the fixture declares at least one case");
    assert!(!root["refusals"].as_array().expect("refusals").is_empty(), "the fixture declares at least one refusal");
    root
}

fn affine_number(value: &serde_json::Value, key: &str) -> f64 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture field {key} is a number"))
}

fn affine_vec3(value: &serde_json::Value) -> EVec3 {
    let items = value.as_array().expect("fixture vector is an array");
    assert_eq!(items.len(), 3, "fixture vector has three components");
    [items[0].as_f64().expect("x"), items[1].as_f64().expect("y"), items[2].as_f64().expect("z")]
}

fn affine_make(kernel: &mut Brep, solid: &serde_json::Value) -> GeometryHandle {
    match solid["kind"].as_str() {
        Some("box") => kernel.box_prim(affine_number(solid, "width"), affine_number(solid, "depth"), affine_number(solid, "height")),
        Some("sphere") => kernel.sphere_prim(affine_number(solid, "radius")),
        Some("cylinder") => kernel.cylinder_prim(affine_number(solid, "radius"), affine_number(solid, "height")),
        Some("cone") => kernel.cone_prim(affine_number(solid, "radius"), affine_number(solid, "height")),
        other => panic!("unknown fixture solid {other:?}"),
    }
    .expect("fixture primitive builds")
}

fn affine_apply(kernel: &mut Brep, shape: &GeometryHandle, step: &serde_json::Value) -> Result<GeometryHandle, BrepError> {
    match step["kind"].as_str() {
        Some("translate") => kernel.translate(shape, affine_vec3(&step["offset"])),
        Some("rotate") => kernel.rotate(shape, affine_vec3(&step["axis"]), affine_number(step, "angle")),
        Some("rotateAbout") => kernel.rotate_about(shape, affine_vec3(&step["origin"]), affine_vec3(&step["axis"]), affine_number(step, "angle")),
        Some("scale") => kernel.scale(shape, affine_number(step, "factor"), affine_vec3(&step["center"])),
        Some("mirror") => kernel.mirror(shape, affine_vec3(&step["origin"]), affine_vec3(&step["normal"])),
        other => panic!("unknown fixture step {other:?}"),
    }
}

fn affine_unit(v: EVec3) -> EVec3 {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / length, v[1] / length, v[2] / length]
}

fn affine_linear_about(linear: [[f64; 3]; 3], fixed: EVec3) -> AffineMap {
    let moved = affine_times(&linear, fixed);
    (linear, [fixed[0] - moved[0], fixed[1] - moved[1], fixed[2] - moved[2]])
}

fn affine_rotation(axis: EVec3, angle: f64) -> [[f64; 3]; 3] {
    let [x, y, z] = affine_unit(axis);
    let (s, c) = angle.sin_cos();
    let t = 1.0 - c;
    [[t * x * x + c, t * x * y - s * z, t * x * z + s * y], [t * x * y + s * z, t * y * y + c, t * y * z - s * x], [t * x * z - s * y, t * y * z + s * x, t * z * z + c]]
}

fn affine_step_map(step: &serde_json::Value) -> AffineMap {
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    match step["kind"].as_str() {
        Some("translate") => (identity, affine_vec3(&step["offset"])),
        Some("rotate") => (affine_rotation(affine_vec3(&step["axis"]), affine_number(step, "angle")), [0.0; 3]),
        Some("rotateAbout") => affine_linear_about(affine_rotation(affine_vec3(&step["axis"]), affine_number(step, "angle")), affine_vec3(&step["origin"])),
        Some("scale") => {
            let f = affine_number(step, "factor");
            affine_linear_about([[f, 0.0, 0.0], [0.0, f, 0.0], [0.0, 0.0, f]], affine_vec3(&step["center"]))
        }
        Some("mirror") => {
            let n = affine_unit(affine_vec3(&step["normal"]));
            let row = |i: usize| [0, 1, 2].map(|j| if i == j { 1.0 } else { 0.0 } - 2.0 * n[i] * n[j]);
            affine_linear_about([row(0), row(1), row(2)], affine_vec3(&step["origin"]))
        }
        other => panic!("unknown fixture step {other:?}"),
    }
}

fn affine_times(m: &[[f64; 3]; 3], v: EVec3) -> EVec3 {
    [0, 1, 2].map(|i| m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2])
}

fn affine_compose(outer: &AffineMap, inner: &AffineMap) -> AffineMap {
    let linear = [0, 1, 2].map(|i| [0, 1, 2].map(|j| (0..3).map(|k| outer.0[i][k] * inner.0[k][j]).sum::<f64>()));
    let moved = affine_times(&outer.0, inner.1);
    (linear, [moved[0] + outer.1[0], moved[1] + outer.1[1], moved[2] + outer.1[2]])
}

fn affine_point(map: &AffineMap, p: EVec3) -> EVec3 {
    let moved = affine_times(&map.0, p);
    [moved[0] + map.1[0], moved[1] + map.1[1], moved[2] + map.1[2]]
}

fn affine_determinant(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// 🧭️ An outward normal maps by the inverse transpose — `cofactor / det`, so a reflection keeps it outward.
fn affine_normal(map: &AffineMap, n: EVec3) -> EVec3 {
    let m = &map.0;
    let cofactor = [
        [m[1][1] * m[2][2] - m[1][2] * m[2][1], m[1][2] * m[2][0] - m[1][0] * m[2][2], m[1][0] * m[2][1] - m[1][1] * m[2][0]],
        [m[0][2] * m[2][1] - m[0][1] * m[2][2], m[0][0] * m[2][2] - m[0][2] * m[2][0], m[0][1] * m[2][0] - m[0][0] * m[2][1]],
        [m[0][1] * m[1][2] - m[0][2] * m[1][1], m[0][2] * m[1][0] - m[0][0] * m[1][2], m[0][0] * m[1][1] - m[0][1] * m[1][0]],
    ];
    let det = affine_determinant(m);
    affine_unit(affine_times(&cofactor, n).map(|component| component / det))
}

fn affine_measure(kernel: &mut Brep, shape: &GeometryHandle, tessellation: f64) -> AffineMeasure {
    let mesh = kernel.tessellate(shape, tessellation).expect("the solid tessellates");
    let points: Vec<parry3d::na::Point3<f32>> = mesh.position.chunks_exact(3).map(|p| parry3d::na::Point3::new(p[0], p[1], p[2])).collect();
    let triangles: Vec<[u32; 3]> = mesh.index.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
    assert!(!points.is_empty() && !triangles.is_empty(), "tessellation produced an empty mesh");
    let (soup_volume, soup_centroid) = parry3d::mass_properties::details::trimesh_signed_volume_and_center_of_mass(&points, &triangles);
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for point in &points {
        for (axis, value) in [point.x, point.y, point.z].into_iter().enumerate() {
            min[axis] = min[axis].min(f64::from(value));
            max[axis] = max[axis].max(f64::from(value));
        }
    }
    let topology = kernel.deconstruct(shape).expect("the solid deconstructs");
    AffineMeasure {
        volume: kernel.volume(shape).expect("volume"),
        center_of_mass: kernel.center_of_mass(shape).expect("centre of mass"),
        min,
        max,
        normals: mesh.face_infos.iter().filter(|info| matches!(info.surface_kind, SurfaceKind::Plane)).map(|info| info.normal).collect(),
        soup_volume: f64::from(soup_volume),
        soup_centroid: [f64::from(soup_centroid.x), f64::from(soup_centroid.y), f64::from(soup_centroid.z)],
        topology: [topology.faces.len(), topology.edges.len(), topology.vertices.len()],
    }
}

/// 🧷️ Moves every normal of `expected` onto its match in `got` (within `tolerance`), reporting what is left on either side.
fn affine_normal_sets(id: &str, got: &[EVec3], expected: &[EVec3], tolerance: f64, failures: &mut Vec<String>) {
    let mut unmatched = got.to_vec();
    for want in expected {
        match unmatched.iter().position(|n| (0..3).all(|axis| (n[axis] - want[axis]).abs() <= tolerance)) {
            Some(index) => {
                unmatched.remove(index);
            }
            None => failures.push(format!("{id}: outward face normal {want:?} missing from {got:?}")),
        }
    }
    if !unmatched.is_empty() {
        failures.push(format!("{id}: unexpected face normals {unmatched:?}"));
    }
}

/// 🔁️ LAW (exactness): every step maps the solid EXACTLY by its affine map — the tessellated soup's signed volume scales by
/// `|det|` and stays POSITIVE (to f32 precision under an isometry; a non-unit scale re-samples curved faces at the same
/// absolute chord, so there the chordal tolerances apply) (`volume` reports a magnitude, so a reflection that left the solid inside out fails here on the
/// sign alone), its centroid lands where the map sends it, every planar face's outward normal lands where the inverse
/// transpose sends it, and face/edge/vertex counts are preserved. The map is derived in this law, not read from the kernel.
#[test]
fn every_affine_transform_maps_the_solid_exactly() {
    let root = affine_fixture();
    let tessellation = affine_number(&root, "tessellationTolerance");
    let normal_tolerance = affine_number(&root, "normalTolerance");
    let soup_volume_tolerance = affine_number(&root, "soupVolumeRelativeTolerance");
    let soup_centroid_tolerance = affine_number(&root, "soupCentroidTolerance");
    let mut failures = Vec::new();
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let mut kernel = Brep::new();
        let mut shape = affine_make(&mut kernel, &case["solid"]);
        let before = affine_measure(&mut kernel, &shape, tessellation);
        let mut map: AffineMap = ([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], [0.0; 3]);
        for step in case["steps"].as_array().expect("steps") {
            shape = affine_apply(&mut kernel, &shape, step).unwrap_or_else(|error| panic!("{id}: {error}"));
            map = affine_compose(&affine_step_map(step), &map);
        }
        let after = affine_measure(&mut kernel, &shape, tessellation);
        let scale = affine_determinant(&map.0).abs();
        let isometry = (scale - 1.0).abs() <= 1e-12;
        let (volume_tolerance, centroid_tolerance) = if isometry { (1e-5, 1e-5) } else { (soup_volume_tolerance, soup_centroid_tolerance) };
        if !(after.soup_volume > 0.0 && (after.soup_volume - scale * before.soup_volume).abs() <= volume_tolerance * scale * before.soup_volume) {
            failures.push(format!("{id}: soup signed volume {} != |det| · {} = {}", after.soup_volume, before.soup_volume, scale * before.soup_volume));
        }
        let centroid = affine_point(&map, before.soup_centroid);
        let extent = (0..3).map(|axis| after.max[axis] - after.min[axis]).fold(1.0, f64::max);
        if (0..3).any(|axis| (after.soup_centroid[axis] - centroid[axis]).abs() > centroid_tolerance * extent) {
            failures.push(format!("{id}: soup centroid {:?} != mapped {centroid:?}", after.soup_centroid));
        }
        if before.topology != after.topology {
            failures.push(format!("{id}: faces/edges/vertices {:?} -> {:?}", before.topology, after.topology));
        }
        let mapped: Vec<EVec3> = before.normals.iter().map(|n| affine_normal(&map, *n)).collect();
        affine_normal_sets(id, &after.normals, &mapped, normal_tolerance, &mut failures);
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// 📐️ LAW (closed form): every vector answers its closed form — volume and centre of mass within the fixture's
/// mass-property tolerances, the tessellation's extent within `boundsTolerance`, the soup (parry3d) within its chordal
/// tolerances, every planar face's outward normal exactly — the same rows OpenCascade answers in the `🧠️semio` suite.
#[test]
fn every_affine_transform_vector_meets_its_closed_form() {
    let root = affine_fixture();
    let tessellation = affine_number(&root, "tessellationTolerance");
    let volume_tolerance = affine_number(&root, "volumeRelativeTolerance");
    let center_tolerance = affine_number(&root, "centerOfMassTolerance");
    let bounds_tolerance = affine_number(&root, "boundsTolerance");
    let normal_tolerance = affine_number(&root, "normalTolerance");
    let soup_volume_tolerance = affine_number(&root, "soupVolumeRelativeTolerance");
    let soup_centroid_tolerance = affine_number(&root, "soupCentroidTolerance");
    let mut failures = Vec::new();
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let expect = &case["expect"];
        let mut kernel = Brep::new();
        let mut shape = affine_make(&mut kernel, &case["solid"]);
        for step in case["steps"].as_array().expect("steps") {
            shape = affine_apply(&mut kernel, &shape, step).unwrap_or_else(|error| panic!("{id}: {error}"));
        }
        let after = affine_measure(&mut kernel, &shape, tessellation);
        let volume = affine_number(expect, "volume");
        let mut near = |label: String, got: f64, want: f64, tolerance: f64| {
            if !((got - want).abs() <= tolerance) {
                failures.push(format!("{id}: {label} {got} != {want} (±{tolerance})"));
            }
        };
        near("volume".into(), after.volume, volume, volume_tolerance * volume.abs());
        near("soup signed volume".into(), after.soup_volume, volume, soup_volume_tolerance * volume.abs());
        let center = affine_vec3(&expect["centerOfMass"]);
        let min = affine_vec3(&expect["bounds"]["min"]);
        let max = affine_vec3(&expect["bounds"]["max"]);
        for axis in 0..3 {
            near(format!("centerOfMass[{axis}]"), after.center_of_mass[axis], center[axis], center_tolerance);
            near(format!("soup centroid[{axis}]"), after.soup_centroid[axis], center[axis], soup_centroid_tolerance);
            near(format!("bounds.min[{axis}]"), after.min[axis], min[axis], bounds_tolerance);
            near(format!("bounds.max[{axis}]"), after.max[axis], max[axis], bounds_tolerance);
        }
        if let Some(normals) = expect["faceNormals"].as_array() {
            let expected: Vec<EVec3> = normals.iter().map(affine_vec3).collect();
            affine_normal_sets(id, &after.normals, &expected, normal_tolerance, &mut failures);
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// 🧭️ LAW (refusal): a degenerate step — a zero axis or normal, a zero factor — is refused as invalid input and never
/// answered with a substituted direction; OpenCascade refuses every one of them too (`🧠️semio` suite).
#[test]
fn every_degenerate_affine_transform_is_refused() {
    let root = affine_fixture();
    let mut failures = Vec::new();
    for refusal in root["refusals"].as_array().expect("refusals") {
        let id = refusal["id"].as_str().expect("refusal id");
        let mut kernel = Brep::new();
        let shape = affine_make(&mut kernel, &refusal["solid"]);
        match affine_apply(&mut kernel, &shape, &refusal["step"]) {
            Err(BrepError::InvalidInput(_)) => {}
            other => failures.push(format!("{id}: expected an invalid-input refusal, got {other:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
// #endregion 🔁️AffineTransforms
