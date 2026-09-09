use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioQuaternion;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn dialect(subset: &str) -> store::os_io::ArtifactDialect {
    store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() }
}

/// 🌱 A hand-built, non-identity, fully-populated object: all three child handles present, a
/// non-origin translation — exercises every field of the census at once.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioObjectSnapshot {
    SemioObjectSnapshot {
        schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(),
        transform: SemioTransform { translation: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } },
        brep: Some(store::ArtifactChild::new("crate-brep".into(), store::os_io::ArtifactRef { artifact_id: "crate-brep".into(), dialect: dialect("brep") })),
        mesh: Some(store::ArtifactChild::new("crate-mesh".into(), store::os_io::ArtifactRef { artifact_id: "crate-mesh".into(), dialect: dialect("mesh") })),
        properties: Some(store::ArtifactChild::new("crate-props".into(), store::os_io::ArtifactRef { artifact_id: "crate-props".into(), dialect: dialect("value") })),
    }
}

#[semio_framework_async_macros::async_test]
async fn census_reflects_child_presence_and_own_translation() {
    let composition = compute_semio_object_composition(&populated());
    assert!(composition.has_brep);
    assert!(composition.has_mesh);
    assert!(composition.has_properties);
    assert_eq!(composition.position, SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 });
}

#[semio_framework_async_macros::async_test]
async fn absent_children_are_honestly_false() {
    let composition = compute_semio_object_composition(&SemioObjectSnapshot::default());
    assert!(!composition.has_brep);
    assert!(!composition.has_mesh);
    assert!(!composition.has_properties);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = populated();
    assert_eq!(compute_semio_object_composition(&snapshot), compute_semio_object_composition(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_object_composition(&SemioObjectSnapshot::default()), SemioObjectComposition::default());
}
