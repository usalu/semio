//! 🐚 Kernel law for `↔️offset::shell_solid`'s SCAFFOLD — the defect
//! `📓️box-shell-orientation-wasm-2026-09-12.md` measured through the served
//! `flow-extension-brep` component on `?plugin=generation3d`: picking `Box Shell Preview` ended in
//! `phase: invalid` with
//! `{code: "shell-orientation-inward", entity: "solid-2", message: "outer shell's signed volume is
//! negative (-4.096000000003073)"}`, while the shelled solid itself measured the correct `+3.904`
//! and the native `example-geometry` oracle was green.
//!
//! Nothing about that was wasm-specific. `shell_solid` built the `-thickness` offset through
//! `offset_solid_with_corner`, which wraps its faces in a `Solid` of its OWN, then flipped every
//! one of those faces to face into the cavity — and left the wrapper in the body. `−4.096` is
//! exactly `−1.6³`: the scaffold's own outer shell, now inside out. `validate_body` walks every
//! solid in the body and `Brep::validate_gate_sync` refuses on any error-class issue anywhere in
//! it, so ONE orphan refused every preview taken from that kernel afterwards — including handles
//! that never went near the shell. The native oracle stayed green only because its harness
//! disposes handles between examples, and `dispose` runs the arena GC that swept the scaffold.
//!
//! So the law is deliberately about the BODY, not only about the returned solid. Each fixture case
//! asserts, from one `Brep` that is never disposed and never GC-ed:
//!
//! 1. `validate_gate_sync` admits the shelled solid AND the untouched box handle — the gate is
//!    whole-body, so a scaffold poisons both, and only checking the shelled one would miss it.
//! 2. `body.solids` holds exactly the two solids the operation is allowed to leave (the input box
//!    and the shell) and `body.shells` exactly three — the assertion that actually pins the fix;
//!    every volume below was already correct while the scaffold was there.
//! 3. `validate_body` is empty — closed, 2-manifold, coherently oriented, no degenerate topology.
//! 4. The outer shell's signed volume is the OUTER box's, positive; the single void shell's is the
//!    cavity's, negative; the solid's is their sum. All three from closed form, none read back
//!    from the kernel.
//! 5. `parry3d` — the third-party oracle, `[dev-dependencies]`-only, the same crate and version
//!    `🧲️procedural-example-booleans` already uses — recomputes that signed volume from the
//!    tessellated triangle soup alone, so the sign this file is named after never rests on our own
//!    arithmetic.
//!
//! Both targets, one law. `cargo test -p semio-s-artifact-stdio-semio --test
//! brep_shell_orientation` runs it natively; the same invocation with
//! `--target wasm32-wasip2` and `CARGO_TARGET_WASM32_WASIP2_RUNNER=wasmtime` runs the identical
//! cases inside a guest, which is the target the served component is built for. The fixture is
//! `include_str!`-ed rather than read from disk precisely so the wasm run needs no preopened
//! directory.
//!
//! @see ../../🧬️schema/🔺️diff/↔️offset/🦀️.rs — `shell_solid`.
//! @see ../../🧬️schema/🔺️diff/🔺️euler/🦀️.rs — `retire_solid_scaffold`.

// #region 🔖️Imports
use parry3d::mass_properties::details::trimesh_signed_volume_and_center_of_mass;
use parry3d::na::Point3;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::offset::shell_solid;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::{Brep, MeshTransfer};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::mass_properties::{shell_signed_volume, solid_signed_volume};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use serde_json::Value;
// #endregion 🔖️Imports

// #region 🔖️Fixture

const FIXTURE: &str = include_str!("../../🧫️fixtures/🐚️shell-orientation/🔣️.json");
const FIXTURE_SCHEMA: &str = "s.stdio.semio.brep.shell-orientation/v1";

/// 🧫 The fixture's own root, schema-checked so a renamed contract fails loudly instead of
/// silently running zero cases.
fn fixture() -> Value {
    let root: Value = serde_json::from_str(FIXTURE).expect("shell-orientation fixture parses");
    assert_eq!(root["schema"].as_str(), Some(FIXTURE_SCHEMA), "fixture schema");
    assert!(!root["cases"].as_array().expect("cases").is_empty(), "fixture declares at least one case");
    root
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn number(value: &Value, key: &str) -> f64 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture field {key} is a number"))
}

// #endregion 🔖️Fixture

// #region 🔖️Oracle

/// 🔮 `parry3d`'s own signed volume of the tessellated shell — the third-party cross-check. The
/// SIGN carries the whole claim: a shell wound inward integrates negative, and a body that still
/// carries the scaffold tessellates to a soup whose sign disagrees with the closed form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parry_signed_volume(mesh: &MeshTransfer) -> f64 {
    let points: Vec<Point3<f32>> = mesh.position.chunks_exact(3).map(|p| Point3::new(p[0], p[1], p[2])).collect();
    let indices: Vec<[u32; 3]> = mesh.index.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
    assert!(!points.is_empty() && !indices.is_empty(), "tessellation produced an empty mesh");
    let (volume, _) = trimesh_signed_volume_and_center_of_mass(&points, &indices);
    f64::from(volume)
}

// #endregion 🔖️Oracle

// #region 🔖️Laws

/// 🐚 Every fixture case, measured on a body that is never disposed and never GC-ed.
#[test]
fn shelling_a_box_leaves_no_scaffold_solid_behind() {
    let root = fixture();
    let chord_tolerance = number(&root, "chordTolerance");
    let deflection = number(&root, "deflection");
    let volume_tolerance = number(&root, "volumeTolerance");
    let oracle_tolerance = number(&root, "oracleTolerance");
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let dims: Vec<f64> = case["box"].as_array().expect("box").iter().map(|v| v.as_f64().expect("box dimension")).collect();
        assert_eq!(dims.len(), 3, "{id}: a box has three dimensions");
        let thickness = number(case, "thickness");
        let expect = &case["expect"];

        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let input = make_box(&mut body, dims[0], dims[1], dims[2], &mut rec).unwrap_or_else(|error| panic!("{id}: box: {error}"));
        let shelled = shell_solid(&mut body, input, thickness, &mut rec).unwrap_or_else(|error| panic!("{id}: shell: {error}"));

        let issues = validate_body(&body);
        let expected_issues: Vec<&str> = expect["validationIssues"].as_array().expect("validationIssues").iter().map(|v| v.as_str().expect("issue code")).collect();
        let actual_codes: Vec<&str> = issues.iter().map(|i| i.code).collect();
        assert_eq!(actual_codes, expected_issues, "{id}: validate_body reported {issues:?}");

        assert_eq!(body.solids.len(), expect["bodySolids"].as_u64().expect("bodySolids") as usize, "{id}: the body must hold only the input box and the shelled solid — a leftover offset scaffold is the defect this law exists for");
        assert_eq!(body.shells.len(), expect["bodyShells"].as_u64().expect("bodyShells") as usize, "{id}: shells in the body");

        let solid = body.solids.get(shelled).unwrap_or_else(|| panic!("{id}: shelled solid is live")).clone();
        let outer = shell_signed_volume(&body, solid.outer, chord_tolerance).unwrap_or_else(|error| panic!("{id}: outer shell volume: {error}"));
        assert!((outer - number(expect, "outerShellSignedVolume")).abs() <= volume_tolerance, "{id}: outer shell signed volume {outer}");
        assert!(outer > 0.0, "{id}: the outer shell must wind outward, got {outer}");

        let expected_voids: Vec<f64> = expect["voidShellSignedVolumes"].as_array().expect("voidShellSignedVolumes").iter().map(|v| v.as_f64().expect("void volume")).collect();
        assert_eq!(solid.inners.len(), expected_voids.len(), "{id}: void shells on the shelled solid");
        for (shell, expected) in solid.inners.iter().zip(expected_voids) {
            let measured = shell_signed_volume(&body, *shell, chord_tolerance).unwrap_or_else(|error| panic!("{id}: void shell volume: {error}"));
            assert!((measured - expected).abs() <= volume_tolerance, "{id}: void shell signed volume {measured}, expected {expected}");
            assert!(measured < 0.0, "{id}: a cavity must be inverted relative to the exterior, got {measured}");
        }

        let expected_volume = number(expect, "solidSignedVolume");
        let measured = solid_signed_volume(&body, shelled, chord_tolerance).unwrap_or_else(|error| panic!("{id}: solid volume: {error}"));
        assert!((measured - expected_volume).abs() <= volume_tolerance, "{id}: solid signed volume {measured}, expected {expected_volume}");

        let mesh = tessellate_solid(&body, shelled, deflection).unwrap_or_else(|error| panic!("{id}: tessellate: {error}"));
        let triangles = mesh.index.len() / 3;
        assert!(triangles >= expect["minTriangles"].as_u64().expect("minTriangles") as usize, "{id}: {triangles} triangles");
        let oracle = parry_signed_volume(&mesh);
        assert!((oracle - expected_volume).abs() <= oracle_tolerance, "{id}: parry3d measured {oracle} over the tessellated shell, expected {expected_volume}");
    }
}

/// 🚦 The gate the served preview actually runs, through the same `Brep` façade the
/// `flow-extension-brep` component calls — whole-body, so it is asserted on BOTH the shelled
/// handle and the untouched input handle. Before the fix this refused both with
/// `shell-orientation-inward`.
#[test]
fn the_preview_validate_gate_admits_a_shelled_box_and_its_input() {
    let root = fixture();
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let dims: Vec<f64> = case["box"].as_array().expect("box").iter().map(|v| v.as_f64().expect("box dimension")).collect();
        let mut kernel = Brep::new();
        let input = kernel.box_prim_sync(dims[0], dims[1], dims[2]).unwrap_or_else(|error| panic!("{id}: box: {error:?}"));
        let shelled = kernel.shell_sync(&input, number(case, "thickness"), &[]).unwrap_or_else(|error| panic!("{id}: shell: {error:?}"));
        assert_eq!(kernel.validate_gate_sync(&shelled), Ok(()), "{id}: the preview gate must admit the shelled solid");
        assert_eq!(kernel.validate_gate_sync(&input), Ok(()), "{id}: one orphan solid refuses every OTHER handle in the same kernel too — that cascade is the whole defect");
    }
}

// #endregion 🔖️Laws
