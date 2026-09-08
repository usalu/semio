
use super::*;
use crate::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Header, Part21Instance};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn point_instance(id: u64, x: f64, y: f64, z: f64) -> Part21Instance {
    Part21Instance { id, entities: vec![("IFCCARTESIANPOINT".into(), vec![Part21Value::List(vec![Part21Value::Real(x.into()), Part21Value::Real(y.into()), Part21Value::Real(z.into())])])] }
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_point_extent() {
    let snapshot = Ifc2x3Snapshot {
        schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(),
        document: Part21Document {
            header: Part21Header::default(),
            instances: vec![point_instance(1, 0.0, 0.0, 0.0), point_instance(2, -2.0, 5.0, 10.0), point_instance(3, 8.0, 1.0, -4.0), Part21Instance { id: 4, entities: vec![("IFCOWNERHISTORY".into(), vec![Part21Value::Unset])] }],
        },
        edm_preamble: None,
    };
    let bounds = compute_ifc2x3_bounds(&snapshot);
    assert_eq!(bounds.min, [-2.0, 0.0, -4.0]);
    assert_eq!(bounds.max, [8.0, 5.0, 10.0]);
    assert_eq!(bounds.point_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document: Part21Document { header: Part21Header::default(), instances: vec![point_instance(1, 1.0, 1.0, 1.0)] }, edm_preamble: None };
    assert_eq!(compute_ifc2x3_bounds(&snapshot), compute_ifc2x3_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_ifc2x3_bounds(&Ifc2x3Snapshot::default()), Ifc2x3Bounds::default());
}
