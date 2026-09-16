use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::make_box;

#[semio_framework_async_macros::async_test]
async fn box_round_trip_topology_counts() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
    let step = write_step(&body, &[solid]).unwrap();
    assert!(step.contains("MANIFOLD_SOLID_BREP"));
    assert!(step.contains("ADVANCED_FACE"));
    let read = read_step(&step).unwrap();
    assert_eq!(read.vertices.len(), 8);
    assert_eq!(read.edges.len(), 12);
    assert_eq!(read.faces.len(), 6);
    assert_eq!(read.solids.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn write_step_rejects_empty_solids() {
    let body = Body::new();
    assert!(write_step(&body, &[]).is_err());
}

#[semio_framework_async_macros::async_test]
async fn read_step_rejects_missing_data_section() {
    assert!(read_step("ISO-10303-21;").is_err());
}

//#region 🔖️RealExports
/// 🌲️ The committed Rhino/ST-Developer export of the hexagonal-cut concrete forest piece — one
/// manifold solid, 57 planar faces, every edge a degree-1 or degree-3 B-spline with a collinear
/// control polygon, every `AXIS2_PLACEMENT_3D` leaving its `ref_direction` unset (`$`).
const CONCRETE_FOREST_STEP: &str = include_str!("../../../../../../../../../../📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧫️fixtures/🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp");

/// 🧊️ The piece's volume as an independent oracle: the signed-tetrahedron sum over the committed
/// GLB export's 958 triangles (`♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/◀️hexagonal-cut-concrete-forest-left.glb`),
/// computed outside this kernel — `14.0998185 m³`.
const CONCRETE_FOREST_VOLUME: f64 = 14.099_818_5;

#[semio_framework_async_macros::async_test]
async fn real_export_reads_optional_placement_slots_and_straight_bsplines_as_lines() {
    let body = read_step(CONCRETE_FOREST_STEP).expect("a real Rhino export reads");
    assert_eq!(body.solids.len(), 1);
    assert_eq!(body.faces.len(), 57);
    assert_eq!(body.edges.len(), 126);
    assert!(body.edges.iter().all(|(_, edge)| matches!(body.curves3.get(edge.curve), Some(Curve3::Line { .. }))), "every collinear B-spline edge is read as the line it is");
    let solid = body.solids.iter().next().map(|(id, _)| id).expect("solid");
    let volume = crate::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume(&body, solid, 1e-3).expect("volume");
    assert!((volume - CONCRETE_FOREST_VOLUME).abs() < 1e-4, "volume {volume} matches the GLB oracle {CONCRETE_FOREST_VOLUME}");
}

#[semio_framework_async_macros::async_test]
async fn read_faces_carry_pcurves_and_validate_clean() {
    let body = read_step(CONCRETE_FOREST_STEP).expect("read");
    assert!(body.coedges.iter().all(|(_, coedge)| coedge.pcurve.is_some()), "every imported coedge carries a p-curve");
    let issues = crate::standards::v1::subsets::brep::schema::inferences::validation_report::body::validate_body(&body);
    assert!(issues.is_empty(), "a valid export validates clean: {issues:?}");
}

#[semio_framework_async_macros::async_test]
async fn read_solids_take_booleans() {
    use crate::standards::v1::subsets::brep::schema::engine::{Brep, BrepKernel};
    let mut kernel = Brep::new();
    let piece = kernel.import_step_sync(CONCRETE_FOREST_STEP).expect("import").into_iter().next().expect("solid");
    let before = kernel.volume(&piece).expect("volume");
    let core = kernel.cylinder_prim(0.051, 0.4).expect("core");
    let core = kernel.translate(&core, [6.0, 3.6, 2.8]).expect("translate");
    let cut = kernel.cut(&piece, &core).expect("a blind core cuts the imported piece");
    let after = kernel.volume(&cut).expect("volume");
    let removed = std::f64::consts::PI * 0.051 * 0.051 * 0.2;
    assert!((before - after - removed).abs() < 1e-5, "the core removes exactly its own volume: {before} → {after} (expected −{removed})");
}
//#endregion 🔖️RealExports
