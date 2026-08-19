//! 📤️ `SemioModelToIfc` — mirror image of the `SemioModelFromIfc` deserializer. Builds a real
//! `Part21Document` (fresh sequential entity ids; every placement re-derived as an ABSOLUTE
//! `IfcAxis2Placement3D` with no `PlacementRelTo` chaining — a legitimate IFC4 encoding, and the
//! simplest one that reproduces the exact WORLD transform this subset already stores) and hands it
//! to ifc's OWN `schema::snapshot::from_part21_document` builder — this file constructs the
//! abstract Part-21 graph only, it never re-implements Part-21 TEXT encoding.
//!
//! Documented lossy/best-effort choices (mirrors the deserializer's own doc comment):
//! - `SemioModelElement` has no `name` field — every generated `IfcElement`'s `Name` attribute is
//!   emitted empty.
//! - `model.relations` is NOT read here: every `Aggregates`/`ContainedIn` edge this bridge ever
//!   produces is already fully implied by `SpatialNode.parent_id`/`SemioModelElement.spatial_id`,
//!   so regenerating `IfcRelAggregates`/`IfcRelContainedInSpatialStructure` straight from those
//!   fields is exact and avoids a second, redundant source of truth. A hand-authored relation of
//!   any OTHER kind (`ConnectsTo`/`FillsVoid`/`VoidsElement`/`Other`) has no IFC counterpart this
//!   bridge builds and is silently dropped — same honesty boundary as the deserializer's.
//! - `PsetValue` is re-wrapped as a generic `IFCTEXT`/`IFCREAL`/`IFCBOOLEAN` defined-type value —
//!   the ORIGINAL wrapper keyword (`IFCLABEL` vs `IFCIDENTIFIER` vs `IFCTEXT`, etc.) is not
//!   preserved (never captured by the deserializer's own `PsetValue` in the first place).
//! - `GlobalId`/relation-guid text is passed through opaquely (this bridge does not validate or
//!   regenerate real IFC 22-character base64 GUIDs).
//! - `geometry`/non-unit `scale` on a placement have no IFC representation in this analyzer's
//!   model and are dropped (`GeometryRef` is never read; `SemioTransform.scale` is ignored).

use crate::artifacts::ifc::IfcSnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioQuaternion, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, PsetValue, SemioModelSnapshot, SpatialKind};
use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use std::collections::HashMap;

//#region 🔖️Serializer
pub struct SemioModelToIfc;

impl ArtifactSerializer for SemioModelToIfc {
    type From = SemioModelSnapshot;
    type Into = IfcSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(ifc_from_model(from))
    }
}

pub async fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Classify
async fn ifc_type_of_spatial_kind(kind: SpatialKind) -> &'static str {
    match kind {
        SpatialKind::Site => "IFCSITE",
        SpatialKind::Building => "IFCBUILDING",
        SpatialKind::Storey => "IFCBUILDINGSTOREY",
        SpatialKind::Space => "IFCSPACE",
    }
}

async fn ifc_type_of_element_class(class: &ElementClass) -> String {
    match class {
        ElementClass::Wall => "IFCWALL".into(),
        ElementClass::Slab => "IFCSLAB".into(),
        ElementClass::Column => "IFCCOLUMN".into(),
        ElementClass::Beam => "IFCBEAM".into(),
        ElementClass::Door => "IFCDOOR".into(),
        ElementClass::Window => "IFCWINDOW".into(),
        ElementClass::Roof => "IFCROOF".into(),
        ElementClass::Stair => "IFCSTAIR".into(),
        ElementClass::Furniture => "IFCFURNISHINGELEMENT".into(),
        ElementClass::Other { name } => name.clone(),
    }
}
//#endregion 🔖️Classify

//#region 🔖️Geometry
/// 🧭️ Inverse of the deserializer's `quat_from_rotation_columns` — standard quaternion -> 3x3
/// rotation matrix, columns = x/y/z basis vectors (matches `Mat4`'s own layout).
async fn quat_to_rotation_columns(q: &SemioQuaternion) -> [[f64; 3]; 3] {
    let (x, y, z, w) = (q.x, q.y, q.z, q.w);
    [[1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y - z * w), 2.0 * (x * z + y * w)], [2.0 * (x * y + z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z - x * w)], [2.0 * (x * z - y * w), 2.0 * (y * z + x * w), 1.0 - 2.0 * (x * x + y * y)]]
}
//#endregion 🔖️Geometry

//#region 🔖️IdAlloc
struct IdAlloc(u64);
impl IdAlloc {
    async fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}
//#endregion 🔖️IdAlloc

//#region 🔖️Builders
async fn owner_history_instance(id: u64) -> Part21Instance {
    Part21Instance { id, entities: vec![("IFCOWNERHISTORY".into(), vec![Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Int(0)])] }
}

async fn project_instance(id: u64, owner_id: u64) -> Part21Instance {
    Part21Instance {
        id,
        entities: vec![(
            "IFCPROJECT".into(),
            vec![Part21Value::Str("semio-model".into()), Part21Value::Ref(owner_id), Part21Value::Str("model".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::List(vec![]), Part21Value::Unset],
        )],
    }
}

/// 📍️ Builds `IfcCartesianPoint`/`IfcDirection`×2/`IfcAxis2Placement3D`/`IfcLocalPlacement`
/// (absolute — `PlacementRelTo` unset) for `transform`, returns the `IfcLocalPlacement` id.
async fn build_placement(instances: &mut Vec<Part21Instance>, alloc: &mut IdAlloc, transform: &SemioTransform) -> u64 {
    let r = quat_to_rotation_columns(&transform.rotation);
    let loc_id = alloc.next();
    instances.push(Part21Instance {
        id: loc_id,
        entities: vec![("IFCCARTESIANPOINT".into(), vec![Part21Value::List(vec![Part21Value::Real(transform.translation.x.into()), Part21Value::Real(transform.translation.y.into()), Part21Value::Real(transform.translation.z.into())])])],
    });
    let axis_id = alloc.next();
    instances.push(Part21Instance { id: axis_id, entities: vec![("IFCDIRECTION".into(), vec![Part21Value::List(vec![Part21Value::Real(r[0][2].into()), Part21Value::Real(r[1][2].into()), Part21Value::Real(r[2][2].into())])])] });
    let refdir_id = alloc.next();
    instances.push(Part21Instance { id: refdir_id, entities: vec![("IFCDIRECTION".into(), vec![Part21Value::List(vec![Part21Value::Real(r[0][0].into()), Part21Value::Real(r[1][0].into()), Part21Value::Real(r[2][0].into())])])] });
    let placement3d_id = alloc.next();
    instances.push(Part21Instance { id: placement3d_id, entities: vec![("IFCAXIS2PLACEMENT3D".into(), vec![Part21Value::Ref(loc_id), Part21Value::Ref(axis_id), Part21Value::Ref(refdir_id)])] });
    let local_id = alloc.next();
    instances.push(Part21Instance { id: local_id, entities: vec![("IFCLOCALPLACEMENT".into(), vec![Part21Value::Unset, Part21Value::Ref(placement3d_id)])] });
    local_id
}

async fn spatial_instance(id: u64, ifc_type: &str, guid: &str, owner_id: u64, name: &str, placement_id: u64) -> Part21Instance {
    Part21Instance {
        id,
        entities: vec![(
            ifc_type.to_string(),
            vec![
                Part21Value::Str(guid.to_string()),
                Part21Value::Ref(owner_id),
                Part21Value::Str(name.to_string()),
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Ref(placement_id),
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Enum("ELEMENT".into()),
            ],
        )],
    }
}

async fn element_instance(id: u64, ifc_type: &str, guid: &str, owner_id: u64, placement_id: u64) -> Part21Instance {
    Part21Instance {
        id,
        entities: vec![(
            ifc_type.to_string(),
            vec![
                Part21Value::Str(guid.to_string()),
                Part21Value::Ref(owner_id),
                // 🕳️ `SemioModelElement` has no `name` field — always empty (documented above).
                Part21Value::Str(String::new()),
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Ref(placement_id),
                Part21Value::Unset,
                Part21Value::Unset,
                Part21Value::Unset,
            ],
        )],
    }
}

async fn rel_aggregates_instance(id: u64, owner_id: u64, parent_id: u64, children: &[u64]) -> Part21Instance {
    Part21Instance {
        id,
        entities: vec![(
            "IFCRELAGGREGATES".into(),
            vec![Part21Value::Str(format!("agg-{id}")), Part21Value::Ref(owner_id), Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(parent_id), Part21Value::List(children.iter().map(|c| Part21Value::Ref(*c)).collect())],
        )],
    }
}

async fn rel_contained_instance(id: u64, owner_id: u64, spatial_id: u64, elements: &[u64]) -> Part21Instance {
    Part21Instance {
        id,
        entities: vec![(
            "IFCRELCONTAINEDINSPATIALSTRUCTURE".into(),
            vec![Part21Value::Str(format!("cont-{id}")), Part21Value::Ref(owner_id), Part21Value::Unset, Part21Value::Unset, Part21Value::List(elements.iter().map(|c| Part21Value::Ref(*c)).collect()), Part21Value::Ref(spatial_id)],
        )],
    }
}

async fn part21_value_of_pset_value(v: &PsetValue) -> Part21Value {
    match v {
        PsetValue::Text { value } => Part21Value::Typed("IFCTEXT".into(), vec![Part21Value::Str(value.clone())]),
        PsetValue::Number { value } => Part21Value::Typed("IFCREAL".into(), vec![Part21Value::Real((*value).into())]),
        PsetValue::Boolean { value } => Part21Value::Typed("IFCBOOLEAN".into(), vec![Part21Value::Enum(if *value { "T" } else { "F" }.into())]),
    }
}

async fn build_pset(instances: &mut Vec<Part21Instance>, alloc: &mut IdAlloc, owner_id: u64, element_id: u64, pset: &crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::PropertySet) {
    let mut prop_ids = Vec::new();
    for prop in &pset.properties {
        let pid = alloc.next();
        instances.push(Part21Instance { id: pid, entities: vec![("IFCPROPERTYSINGLEVALUE".into(), vec![Part21Value::Str(prop.key.clone()), Part21Value::Unset, part21_value_of_pset_value(&prop.value), Part21Value::Unset])] });
        prop_ids.push(pid);
    }
    let pset_id = alloc.next();
    instances.push(Part21Instance {
        id: pset_id,
        entities: vec![(
            "IFCPROPERTYSET".into(),
            vec![Part21Value::Str(format!("pset-{pset_id}")), Part21Value::Ref(owner_id), Part21Value::Str(pset.name.clone()), Part21Value::Unset, Part21Value::List(prop_ids.iter().map(|p| Part21Value::Ref(*p)).collect())],
        )],
    });
    let rel_id = alloc.next();
    instances.push(Part21Instance {
        id: rel_id,
        entities: vec![(
            "IFCRELDEFINESBYPROPERTIES".into(),
            vec![Part21Value::Str(format!("rel-{rel_id}")), Part21Value::Ref(owner_id), Part21Value::Unset, Part21Value::Unset, Part21Value::List(vec![Part21Value::Ref(element_id)]), Part21Value::Ref(pset_id)],
        )],
    });
}
//#endregion 🔖️Builders

//#region 🔖️Entry
pub async fn ifc_from_model(from: &SemioModelSnapshot) -> IfcSnapshot {
    let mut instances: Vec<Part21Instance> = Vec::new();
    let mut alloc = IdAlloc(0);
    let owner_id = alloc.next();
    instances.push(owner_history_instance(owner_id));
    let project_id = alloc.next();
    instances.push(project_instance(project_id, owner_id));

    let mut spatial_ids: HashMap<String, u64> = HashMap::new();
    for node in &from.spatial {
        let placement_id = build_placement(&mut instances, &mut alloc, &node.placement);
        let numeric_id = alloc.next();
        instances.push(spatial_instance(numeric_id, ifc_type_of_spatial_kind(node.kind), &node.id, owner_id, &node.name, placement_id));
        spatial_ids.insert(node.id.clone(), numeric_id);
    }

    let mut project_children = Vec::new();
    let mut spatial_children: HashMap<String, Vec<u64>> = HashMap::new();
    for node in &from.spatial {
        let nid = spatial_ids[&node.id];
        match &node.parent_id {
            Some(p) if spatial_ids.contains_key(p) => spatial_children.entry(p.clone()).or_default().push(nid),
            _ => project_children.push(nid),
        }
    }
    if !project_children.is_empty() {
        let rel_id = alloc.next();
        instances.push(rel_aggregates_instance(rel_id, owner_id, project_id, &project_children));
    }
    for (parent_guid, children) in &spatial_children {
        let rel_id = alloc.next();
        instances.push(rel_aggregates_instance(rel_id, owner_id, spatial_ids[parent_guid], children));
    }

    let mut element_ids: HashMap<String, u64> = HashMap::new();
    for el in &from.elements {
        let placement_id = build_placement(&mut instances, &mut alloc, &el.placement);
        let numeric_id = alloc.next();
        instances.push(element_instance(numeric_id, &ifc_type_of_element_class(&el.class), &el.id, owner_id, placement_id));
        element_ids.insert(el.id.clone(), numeric_id);
        for pset in &el.psets {
            build_pset(&mut instances, &mut alloc, owner_id, numeric_id, pset);
        }
    }
    let mut spatial_elements: HashMap<String, Vec<u64>> = HashMap::new();
    for el in &from.elements {
        if let Some(sid) = &el.spatial_id {
            if spatial_ids.contains_key(sid) {
                spatial_elements.entry(sid.clone()).or_default().push(element_ids[&el.id]);
            }
        }
    }
    for (spatial_guid, els) in &spatial_elements {
        let rel_id = alloc.next();
        instances.push(rel_contained_instance(rel_id, owner_id, spatial_ids[spatial_guid], els));
    }

    let doc = Part21Document {
        header: Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str(String::new())]), Part21Value::Str("2;1".into())],
            file_name: vec![
                Part21Value::Str("semio-model.ifc".into()),
                Part21Value::Str(String::new()),
                Part21Value::List(vec![]),
                Part21Value::List(vec![]),
                Part21Value::Str("semio".into()),
                Part21Value::Str("semio".into()),
                Part21Value::Str(String::new()),
            ],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC4".into())])],
        },
        instances,
    };
    crate::artifacts::ifc::schema::snapshot::from_part21_document(crate::artifacts::ifc::STDIO_IFC_DOCUMENT_SCHEMA, &doc)
}
//#endregion 🔖️Entry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::model::io::import::deserializers::artifacts::ifc::v4::any::model_from_ifc;
    use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{GeometryRef, ModelRelation, Property, PropertySet, RelationKind, SemioModelElement, SpatialNode};

    async fn rich_model() -> SemioModelSnapshot {
        SemioModelSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
            spatial: vec![
                SpatialNode { id: "site-1".into(), kind: SpatialKind::Site, name: "Site One".into(), parent_id: None, placement: SemioTransform::identity() },
                SpatialNode {
                    id: "storey-1".into(),
                    kind: SpatialKind::Storey,
                    name: "Ground Floor".into(),
                    parent_id: Some("site-1".into()),
                    placement: SemioTransform {
                        translation: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: 0.0, y: 0.0, z: 3.0 },
                        rotation: SemioQuaternion::default(),
                        scale: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 },
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
            schema: crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
            spatial: vec![SpatialNode {
                id: "site-1".into(),
                kind: SpatialKind::Site,
                name: "Rotated Site".into(),
                parent_id: None,
                placement: SemioTransform {
                    translation: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: 5.0, y: -2.0, z: 0.0 },
                    rotation,
                    scale: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 },
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
}
//#endregion 🧪️Tests
