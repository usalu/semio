//! 🧩 `composition` — one named inference: which owned CHILD slots are present (`brep`/`mesh`/
//! `properties` — handles only, never embedded content per this subset's own module doc comment,
//! so presence is the honest limit of what a handle tells you) plus the object's own real
//! `transform.translation`, read directly (never a fabricated geometry bounding box — resolving a
//! child handle into its target snapshot is a cross-artifact read, out of scope for a pure
//! snapshot->inference fold).

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Composition
/// 🧩️ Semio object composition census.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioObjectComposition {
    pub has_brep: bool,
    pub has_mesh: bool,
    pub has_properties: bool,
    pub position: SemioPoint3,
}

/// 🧩️ Computes [`SemioObjectComposition`] — pure, total, O(1).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_object_composition(snapshot: &SemioObjectSnapshot) -> SemioObjectComposition {
    SemioObjectComposition { has_brep: snapshot.brep.is_some(), has_mesh: snapshot.mesh.is_some(), has_properties: snapshot.properties.is_some(), position: snapshot.transform.translation }
}
//#endregion 🔖️Composition

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioQuaternion;
    use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioTransform;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA;

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
            brep: Some(store::ArtifactChild::new("brep-01".into(), store::os_io::ArtifactRef { artifact_id: "crate-brep".into(), dialect: dialect("brep") })),
            mesh: Some(store::ArtifactChild::new("mesh-01".into(), store::os_io::ArtifactRef { artifact_id: "crate-mesh".into(), dialect: dialect("mesh") })),
            properties: Some(store::ArtifactChild::new("props-01".into(), store::os_io::ArtifactRef { artifact_id: "crate-props".into(), dialect: dialect("value") })),
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
}
//#endregion 🧪️Tests
