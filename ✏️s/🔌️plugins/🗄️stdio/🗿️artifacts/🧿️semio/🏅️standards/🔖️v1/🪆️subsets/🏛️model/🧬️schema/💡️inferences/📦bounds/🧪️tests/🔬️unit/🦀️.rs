use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, SemioModelElement, SpatialKind, SpatialNode, STDIO_SEMIOMODEL_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn placed(x: f64, y: f64, z: f64) -> SemioTransform {
    SemioTransform { translation: SemioPoint3 { x, y, z }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioModelSnapshot {
    SemioModelSnapshot {
        schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
        spatial: vec![
            SpatialNode { id: "site-1".into(), kind: SpatialKind::Site, name: "Site".into(), parent_id: None, placement: placed(-5.0, 0.0, 0.0) },
            SpatialNode { id: "storey-1".into(), kind: SpatialKind::Storey, name: "Ground".into(), parent_id: Some("site-1".into()), placement: placed(0.0, 0.0, 3.0) },
        ],
        elements: vec![SemioModelElement { id: "wall-1".into(), class: ElementClass::Wall, placement: placed(10.0, -2.0, 1.0), geometry: GeometryRef::None, spatial_id: Some("storey-1".into()), psets: Vec::new() }],
        relations: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn folds_min_max_across_spatial_and_element_placements() {
    let bounds = compute_semio_model_bounds(&populated());
    assert_eq!(bounds.entity_count, 3);
    assert_eq!(bounds.min, SemioPoint3 { x: -5.0, y: -2.0, z: 0.0 });
    assert_eq!(bounds.max, SemioPoint3 { x: 10.0, y: 0.0, z: 3.0 });
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = populated();
    assert_eq!(compute_semio_model_bounds(&snapshot), compute_semio_model_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_model_bounds(&SemioModelSnapshot::default()), SemioModelBounds::default());
}
