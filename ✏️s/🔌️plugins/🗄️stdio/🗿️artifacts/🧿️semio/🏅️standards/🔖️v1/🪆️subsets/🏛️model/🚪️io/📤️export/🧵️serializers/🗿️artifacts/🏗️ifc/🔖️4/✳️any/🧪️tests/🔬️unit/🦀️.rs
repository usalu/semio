use super::*;
use crate::standards::v1::subsets::model::io::import::deserializers::artifacts::ifc::v4::any::model_from_ifc;
use crate::standards::v1::subsets::model::schema::snapshot::{GeometryRef, ModelRelation, Property, PropertySet, RelationKind, SemioModelElement, SpatialNode};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn rich_model() -> SemioModelSnapshot {
    SemioModelSnapshot {
        schema: crate::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
        spatial: vec![
            SpatialNode { id: "site-1".into(), kind: SpatialKind::Site, name: "Site One".into(), parent_id: None, placement: SemioTransform::identity() },
            SpatialNode {
                id: "storey-1".into(),
                kind: SpatialKind::Storey,
                name: "Ground Floor".into(),
                parent_id: Some("site-1".into()),
                placement: SemioTransform {
                    translation: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: 0.0, y: 0.0, z: 3.0 },
                    rotation: SemioQuaternion::default(),
                    scale: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 },
                },
            },
        ],
        elements: vec![SemioModelElement {
            id: "wall-1".into(),
            class: ElementClass::Wall,
            placement: SemioTransform::identity(),
            geometry: GeometryRef::None,
            spatial_id: Some("storey-1".into()),
            psets: vec![PropertySet {
                name: "Pset_WallCommon".into(),
                properties: vec![Property { key: "IsExternal".into(), value: PsetValue::Boolean { value: true } }, Property { key: "FireRating".into(), value: PsetValue::Text { value: "REI60".into() } }],
            }],
        }],
        relations: vec![ModelRelation { id: "rel-1".into(), kind: RelationKind::ContainedIn, from: "wall-1".into(), to: "storey-1".into() }],
    }
}

/// 🧪️ Required proof: model -> ifc -> model round trip preserves everything `model` can
/// represent (documented lossy fields excepted — none of which this fixture exercises: no
/// element name, unit scale throughout).
#[semio_framework_async_macros::async_test]
async fn model_to_ifc_to_model_round_trips() {
    let s1 = rich_model();
    let ifc = ifc_from_model(&s1);
    let s2 = model_from_ifc(&ifc);

    assert_eq!(s1.spatial.len(), s2.spatial.len());
    for original in &s1.spatial {
        let back = s2.spatial.iter().find(|n| n.id == original.id).expect("spatial node survives by id");
        assert_eq!(back.kind, original.kind);
        assert_eq!(back.name, original.name);
        assert_eq!(back.parent_id, original.parent_id);
        assert!((back.placement.translation.x - original.placement.translation.x).abs() < 1e-9);
        assert!((back.placement.translation.y - original.placement.translation.y).abs() < 1e-9);
        assert!((back.placement.translation.z - original.placement.translation.z).abs() < 1e-9);
    }

    assert_eq!(s1.elements.len(), s2.elements.len());
    let original = &s1.elements[0];
    let back = s2.elements.iter().find(|e| e.id == original.id).expect("element survives by id");
    assert_eq!(back.class, original.class);
    assert_eq!(back.spatial_id, original.spatial_id);
    assert_eq!(back.geometry, GeometryRef::None);
    assert_eq!(back.psets, original.psets, "pset name/key/value round-trips through the IFCTEXT/IFCREAL/IFCBOOLEAN rewrap");

    // relations are re-derived (not read) on serialize, and re-derived identically on
    // deserialize — the ContainedIn edge from s1 (hand-authored) still exists in s2, now
    // via the SAME deterministic id-synthesis formula the deserializer always uses.
    assert!(s2.relations.iter().any(|r| r.kind == RelationKind::ContainedIn && r.from == "wall-1" && r.to == "storey-1"));
}

#[semio_framework_async_macros::async_test]
async fn non_unit_rotation_round_trips_through_the_quaternion_matrix_conversion() {
    // 45 degree rotation about Z: (0, 0, sin(22.5deg), cos(22.5deg)).
    let half = std::f64::consts::FRAC_PI_8;
    let rotation = SemioQuaternion { x: 0.0, y: 0.0, z: half.sin(), w: half.cos() };
    let s1 = SemioModelSnapshot {
        schema: crate::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
        spatial: vec![SpatialNode {
            id: "site-1".into(),
            kind: SpatialKind::Site,
            name: "Rotated Site".into(),
            parent_id: None,
            placement: SemioTransform {
                translation: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: 5.0, y: -2.0, z: 0.0 },
                rotation,
                scale: crate::standards::v1::subsets::base::schema::geometry::SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 },
            },
        }],
        elements: vec![],
        relations: vec![],
    };
    let ifc = ifc_from_model(&s1);
    let s2 = model_from_ifc(&ifc);
    let back = &s2.spatial[0];
    assert!((back.placement.rotation.z - rotation.z).abs() < 1e-9, "rotation.z: {:?}", back.placement.rotation);
    assert!((back.placement.rotation.w - rotation.w).abs() < 1e-9, "rotation.w: {:?}", back.placement.rotation);
    assert!((back.placement.translation.x - 5.0).abs() < 1e-9);
}
