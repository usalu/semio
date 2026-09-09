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

use crate::standards::v1::subsets::base::schema::geometry::{SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, PsetValue, SemioModelSnapshot, SpatialKind};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_ifc::IfcSnapshot;
use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Classify
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifc_type_of_spatial_kind(kind: SpatialKind) -> &'static str {
    match kind {
        SpatialKind::Site => "IFCSITE",
        SpatialKind::Building => "IFCBUILDING",
        SpatialKind::Storey => "IFCBUILDINGSTOREY",
        SpatialKind::Space => "IFCSPACE",
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifc_type_of_element_class(class: &ElementClass) -> String {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn quat_to_rotation_columns(q: &SemioQuaternion) -> [[f64; 3]; 3] {
    let (x, y, z, w) = (q.x, q.y, q.z, q.w);
    [[1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y - z * w), 2.0 * (x * z + y * w)], [2.0 * (x * y + z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z - x * w)], [2.0 * (x * z - y * w), 2.0 * (y * z + x * w), 1.0 - 2.0 * (x * x + y * y)]]
}
//#endregion 🔖️Geometry

//#region 🔖️IdAlloc
struct IdAlloc(u64);
impl IdAlloc {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}
//#endregion 🔖️IdAlloc

//#region 🔖️Builders
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn owner_history_instance(id: u64) -> Part21Instance {
    Part21Instance { id, entities: vec![("IFCOWNERHISTORY".into(), vec![Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Int(0)])] }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn project_instance(id: u64, owner_id: u64) -> Part21Instance {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_placement(instances: &mut Vec<Part21Instance>, alloc: &mut IdAlloc, transform: &SemioTransform) -> u64 {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn spatial_instance(id: u64, ifc_type: &str, guid: &str, owner_id: u64, name: &str, placement_id: u64) -> Part21Instance {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn element_instance(id: u64, ifc_type: &str, guid: &str, owner_id: u64, placement_id: u64) -> Part21Instance {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rel_aggregates_instance(id: u64, owner_id: u64, parent_id: u64, children: &[u64]) -> Part21Instance {
    Part21Instance {
        id,
        entities: vec![(
            "IFCRELAGGREGATES".into(),
            vec![Part21Value::Str(format!("agg-{id}")), Part21Value::Ref(owner_id), Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(parent_id), Part21Value::List(children.iter().map(|c| Part21Value::Ref(*c)).collect())],
        )],
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rel_contained_instance(id: u64, owner_id: u64, spatial_id: u64, elements: &[u64]) -> Part21Instance {
    Part21Instance {
        id,
        entities: vec![(
            "IFCRELCONTAINEDINSPATIALSTRUCTURE".into(),
            vec![Part21Value::Str(format!("cont-{id}")), Part21Value::Ref(owner_id), Part21Value::Unset, Part21Value::Unset, Part21Value::List(elements.iter().map(|c| Part21Value::Ref(*c)).collect()), Part21Value::Ref(spatial_id)],
        )],
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part21_value_of_pset_value(v: &PsetValue) -> Part21Value {
    match v {
        PsetValue::Text { value } => Part21Value::Typed { name: "IFCTEXT".into(), items: vec![Part21Value::Str(value.clone())] },
        PsetValue::Number { value } => Part21Value::Typed { name: "IFCREAL".into(), items: vec![Part21Value::Real((*value).into())] },
        PsetValue::Boolean { value } => Part21Value::Typed { name: "IFCBOOLEAN".into(), items: vec![Part21Value::Enum(if *value { "T" } else { "F" }.into())] },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_pset(instances: &mut Vec<Part21Instance>, alloc: &mut IdAlloc, owner_id: u64, element_id: u64, pset: &crate::standards::v1::subsets::model::schema::snapshot::PropertySet) {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ifc_from_model(from: &SemioModelSnapshot) -> IfcSnapshot {
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
    semio_s_artifact_stdio_ifc::schema::snapshot::from_part21_document(semio_s_artifact_stdio_ifc::STDIO_IFC_DOCUMENT_SCHEMA, &doc)
}
//#endregion 🔖️Entry

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
