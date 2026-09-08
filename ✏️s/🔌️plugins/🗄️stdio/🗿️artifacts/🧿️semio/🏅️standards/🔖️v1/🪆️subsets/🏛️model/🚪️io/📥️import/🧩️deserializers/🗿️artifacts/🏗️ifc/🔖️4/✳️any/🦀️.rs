//! 📥️ `SemioModelFromIfc` — real IFC4 spatial-structure/property-set bridge into `semio/v1/model`.
//! Reuses ifc's OWN `engine::spatial::analyze_spatial` (parent/child resolution, composed
//! placement matrices, property-set resolution) plus `schema::snapshot::to_part21_document` — this
//! file does ONLY the `SpatialAnalysis` -> `SemioModelSnapshot` shape mapping; it never re-parses
//! Part-21 bytes itself.
//!
//! Honest mapping, documented gaps (never silently fabricated):
//! - `IFCPROJECT` has no `SpatialKind` counterpart (model's spatial kinds are
//!   site/building/storey/space only) — its children become spatial ROOTS (`parent_id: None`); the
//!   project node itself is dropped.
//! - `IfcElement.Name`/`Description` have no home on `SemioModelElement` (this subset's schema, as
//!   built by W2a, carries no `name` field) — element names are unconditionally dropped. Spatial
//!   node names DO map (`SpatialNode.name` exists).
//! - Element GEOMETRY (`IfcShapeRepresentation`/`IfcExtrudedAreaSolid`/…) is never resolved into
//!   the sibling `brep`/`mesh` subsets by this bridge — every element decodes with
//!   `GeometryRef::None`. That is real geometric-kernel work, out of a Snapshot-to-Snapshot bridge.
//! - Nested element-under-element composition (e.g. an opening inside a wall) is FLATTENED: every
//!   non-spatial descendant becomes a sibling element attached to the nearest spatial ancestor —
//!   `model.elements` has no element-to-element parent field to preserve it in.
//! - A property value that isn't a scalar (`Str`/`Real`/`Int`/`Enum`/one-level `Typed`) — i.e. a
//!   `List`/`Ref`/`Unset`/`Derived` nominal value — has no `PsetValue` counterpart and is skipped.
//! - `relations` carries only the `Aggregates` (spatial parent/child) and `ContainedIn`
//!   (element/spatial) edges implied by the containment tree this bridge walks — other real IFC
//!   relationship kinds (`IfcRelVoidsElement`/`IfcRelConnectsElements`/…) are not read here.

use semio_s_artifact_stdio_ifc::engine::spatial::{analyze_spatial, Mat4, PropertySet as IfcPropertySet, SpatialAnalysis, SpatialNode as IfcSpatialNode};
use semio_s_artifact_stdio_ifc::IfcSnapshot;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::model::schema::snapshot::{
    ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode, STDIO_SEMIOMODEL_DOCUMENT_SCHEMA,
};
use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Value};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️Deserializer
pub struct SemioModelFromIfc;

impl ArtifactDeserializer for SemioModelFromIfc {
    type From = IfcSnapshot;
    type Into = SemioModelSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(model_from_ifc(from))
    }
}

/// 📌️ No standalone registration — this leaf is wired into the registry through `model`'s own
/// `🎹️composer::register()` via `deserializer_entry_of::<SemioModelFromIfc>()` (matches the
/// repo-wide io-leaf convention, e.g. gltf's own `register() {}` stub next to its json bridge).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
//#endregion 🔖️Deserializer

//#region 🔖️Classify
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn spatial_kind_of(ifc_type: &str) -> Option<SpatialKind> {
    match ifc_type.to_ascii_uppercase().as_str() {
        "IFCSITE" => Some(SpatialKind::Site),
        "IFCBUILDING" => Some(SpatialKind::Building),
        "IFCBUILDINGSTOREY" => Some(SpatialKind::Storey),
        "IFCSPACE" => Some(SpatialKind::Space),
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn element_class_from_ifc_type(ifc_type: &str) -> ElementClass {
    match ifc_type.to_ascii_uppercase().as_str() {
        "IFCWALL" | "IFCWALLSTANDARDCASE" => ElementClass::Wall,
        "IFCSLAB" => ElementClass::Slab,
        "IFCCOLUMN" => ElementClass::Column,
        "IFCBEAM" => ElementClass::Beam,
        "IFCDOOR" => ElementClass::Door,
        "IFCWINDOW" => ElementClass::Window,
        "IFCROOF" => ElementClass::Roof,
        "IFCSTAIR" | "IFCSTAIRFLIGHT" => ElementClass::Stair,
        "IFCFURNISHINGELEMENT" | "IFCFURNITURE" => ElementClass::Furniture,
        other => ElementClass::Other { name: other.to_string() },
    }
}
//#endregion 🔖️Classify

//#region 🔖️Geometry
/// 🧭️ `Mat4`'s rotation columns (`m[i][0]`=x-axis, `m[i][1]`=y-axis, `m[i][2]`=z-axis, per
/// `build_axis2placement`'s own layout) -> a quaternion, via the standard trace-based (Shepperd)
/// method. Scale is always unit — this analyzer's placements never carry non-uniform scale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn quat_from_rotation_columns(m: &Mat4) -> SemioQuaternion {
    let r = &m.0;
    let trace = r[0][0] + r[1][1] + r[2][2];
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        SemioQuaternion { w: 0.25 * s, x: (r[2][1] - r[1][2]) / s, y: (r[0][2] - r[2][0]) / s, z: (r[1][0] - r[0][1]) / s }
    } else if r[0][0] > r[1][1] && r[0][0] > r[2][2] {
        let s = (1.0 + r[0][0] - r[1][1] - r[2][2]).sqrt() * 2.0;
        SemioQuaternion { w: (r[2][1] - r[1][2]) / s, x: 0.25 * s, y: (r[0][1] + r[1][0]) / s, z: (r[0][2] + r[2][0]) / s }
    } else if r[1][1] > r[2][2] {
        let s = (1.0 + r[1][1] - r[0][0] - r[2][2]).sqrt() * 2.0;
        SemioQuaternion { w: (r[0][2] - r[2][0]) / s, x: (r[0][1] + r[1][0]) / s, y: 0.25 * s, z: (r[1][2] + r[2][1]) / s }
    } else {
        let s = (1.0 + r[2][2] - r[0][0] - r[1][1]).sqrt() * 2.0;
        SemioQuaternion { w: (r[1][0] - r[0][1]) / s, x: (r[0][2] + r[2][0]) / s, y: (r[1][2] + r[2][1]) / s, z: 0.25 * s }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transform_from_mat4(m: &Mat4) -> SemioTransform {
    SemioTransform { translation: SemioPoint3 { x: m.0[0][3], y: m.0[1][3], z: m.0[2][3] }, rotation: quat_from_rotation_columns(m), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
}
//#endregion 🔖️Geometry

//#region 🔖️PropertyValue
/// 🏷️ Best-effort `Part21Value` -> `PsetValue`: unwraps one level of `Typed(_, [inner])` (the
/// `IFCBOOLEAN(.T.)`/`IFCREAL(3000.)`/`IFCTEXT('x')` defined-type wrapper shape) before matching a
/// scalar. `List`/`Ref`/`Unset`/`Derived` (incl. a `Typed` wrapper around one of those) have no
/// `PsetValue` counterpart — `None`, never fabricated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pset_value_from_part21(v: &Part21Value) -> Option<PsetValue> {
    match v {
        Part21Value::Str(s) => Some(PsetValue::Text { value: s.clone() }),
        Part21Value::Real(r) => Some(PsetValue::Number { value: r.to_f64()? }),
        Part21Value::Int(i) => Some(PsetValue::Number { value: *i as f64 }),
        Part21Value::Enum(s) => match s.as_str() {
            "T" | "TRUE" | ".T." => Some(PsetValue::Boolean { value: true }),
            "F" | "FALSE" | ".F." => Some(PsetValue::Boolean { value: false }),
            other => Some(PsetValue::Text { value: other.to_string() }),
        },
        Part21Value::Typed { items: inner, .. } => inner.first().and_then(pset_value_from_part21),
        Part21Value::List(_) | Part21Value::Ref(_) | Part21Value::Unset | Part21Value::Derived => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn convert_pset(ps: &IfcPropertySet) -> PropertySet {
    PropertySet { name: ps.name.clone(), properties: ps.properties.iter().filter_map(|p| pset_value_from_part21(&p.value).map(|value| Property { key: p.name.clone(), value })).collect() }
}
//#endregion 🔖️PropertyValue

//#region 🔖️Walk
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn guid_of(doc: &Part21Document, id: u64) -> String {
    doc.instance(id).and_then(|i| i.primary()).and_then(|(_, args)| args.first()).and_then(Part21Value::as_str).map_or_else(|| format!("ifc-{id}"), str::to_string)
}

/// 🌳️ Recursively converts one `analyze_spatial` tree node into `spatial`/`elements`/`relations`
/// rows, tracking the nearest spatial ancestor's already-converted `model` id (`None` at the
/// project root). See the module doc comment for every documented gap this walk introduces.
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk(doc: &Part21Document, node: &IfcSpatialNode, parent_spatial_id: Option<&str>, analysis: &SpatialAnalysis, out_spatial: &mut Vec<SpatialNode>, out_elements: &mut Vec<SemioModelElement>, out_relations: &mut Vec<ModelRelation>) {
    let placement = node.object_placement.and_then(|pid| analysis.placements.get(&pid)).map_or_else(SemioTransform::identity, transform_from_mat4);

    if let Some(kind) = spatial_kind_of(&node.ifc_type) {
        let id = guid_of(doc, node.id);
        out_spatial.push(SpatialNode { id: id.clone(), kind, name: node.name.clone().unwrap_or_default(), parent_id: parent_spatial_id.map(str::to_owned), placement });
        if let Some(parent) = parent_spatial_id {
            out_relations.push(ModelRelation { id: format!("rel-aggregates-{parent}-{id}"), kind: RelationKind::Aggregates, from: id.clone(), to: parent.to_owned() });
        }
        for child in &node.children {
            walk(doc, child, Some(&id), analysis, out_spatial, out_elements, out_relations);
        }
    } else if node.ifc_type.eq_ignore_ascii_case("IFCPROJECT") {
        for child in &node.children {
            walk(doc, child, parent_spatial_id, analysis, out_spatial, out_elements, out_relations);
        }
    } else {
        let id = guid_of(doc, node.id);
        let class = element_class_from_ifc_type(&node.ifc_type);
        let psets = analysis.property_sets.get(&node.id).map(|v| v.iter().map(convert_pset).collect()).unwrap_or_default();
        out_elements.push(SemioModelElement { id: id.clone(), class, placement, geometry: GeometryRef::None, spatial_id: parent_spatial_id.map(str::to_owned), psets });
        if let Some(parent) = parent_spatial_id {
            out_relations.push(ModelRelation { id: format!("rel-containedin-{id}-{parent}"), kind: RelationKind::ContainedIn, from: id, to: parent.to_owned() });
        }
        for child in &node.children {
            // 🧩️ Flattened: a nested element (e.g. an opening) attaches to the SAME spatial
            // ancestor, not to `id` — `model.elements` has no element-parent field.
            walk(doc, child, parent_spatial_id, analysis, out_spatial, out_elements, out_relations);
        }
    }
}
//#endregion 🔖️Walk

//#region 🔖️Entry
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn model_from_ifc(from: &IfcSnapshot) -> SemioModelSnapshot {
    let doc = semio_s_artifact_stdio_ifc::schema::snapshot::to_part21_document(from);
    let analysis = analyze_spatial(&doc);
    let mut spatial = Vec::new();
    let mut elements = Vec::new();
    let mut relations = Vec::new();
    for root in &analysis.roots {
        walk(&doc, root, None, &analysis, &mut spatial, &mut elements, &mut relations);
    }
    SemioModelSnapshot { schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(), spatial, elements, relations }
}
//#endregion 🔖️Entry

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
