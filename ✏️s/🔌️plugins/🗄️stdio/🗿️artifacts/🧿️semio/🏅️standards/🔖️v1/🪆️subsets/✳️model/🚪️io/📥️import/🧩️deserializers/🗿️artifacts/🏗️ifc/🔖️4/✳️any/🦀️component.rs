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

use crate::artifacts::ifc::engine::spatial::{analyze_spatial, Mat4, PropertySet as IfcPropertySet, SpatialAnalysis, SpatialNode as IfcSpatialNode};
use crate::artifacts::ifc::IfcSnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{
    ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode, STDIO_SEMIOMODEL_DOCUMENT_SCHEMA,
};
use crate::artifacts::step::engine::part21::{Part21Document, Part21Value};
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
pub async fn register() {}
//#endregion 🔖️Deserializer

//#region 🔖️Classify
async fn spatial_kind_of(ifc_type: &str) -> Option<SpatialKind> {
    match ifc_type.to_ascii_uppercase().as_str() {
        "IFCSITE" => Some(SpatialKind::Site),
        "IFCBUILDING" => Some(SpatialKind::Building),
        "IFCBUILDINGSTOREY" => Some(SpatialKind::Storey),
        "IFCSPACE" => Some(SpatialKind::Space),
        _ => None,
    }
}

async fn element_class_from_ifc_type(ifc_type: &str) -> ElementClass {
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
async fn quat_from_rotation_columns(m: &Mat4) -> SemioQuaternion {
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

async fn transform_from_mat4(m: &Mat4) -> SemioTransform {
    SemioTransform { translation: SemioPoint3 { x: m.0[0][3], y: m.0[1][3], z: m.0[2][3] }, rotation: quat_from_rotation_columns(m), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
}
//#endregion 🔖️Geometry

//#region 🔖️PropertyValue
/// 🏷️ Best-effort `Part21Value` -> `PsetValue`: unwraps one level of `Typed(_, [inner])` (the
/// `IFCBOOLEAN(.T.)`/`IFCREAL(3000.)`/`IFCTEXT('x')` defined-type wrapper shape) before matching a
/// scalar. `List`/`Ref`/`Unset`/`Derived` (incl. a `Typed` wrapper around one of those) have no
/// `PsetValue` counterpart — `None`, never fabricated.
async fn pset_value_from_part21(v: &Part21Value) -> Option<PsetValue> {
    match v {
        Part21Value::Str(s) => Some(PsetValue::Text { value: s.clone() }),
        Part21Value::Real(r) => Some(PsetValue::Number { value: r.to_f64()? }),
        Part21Value::Int(i) => Some(PsetValue::Number { value: *i as f64 }),
        Part21Value::Enum(s) => match s.as_str() {
            "T" | "TRUE" | ".T." => Some(PsetValue::Boolean { value: true }),
            "F" | "FALSE" | ".F." => Some(PsetValue::Boolean { value: false }),
            other => Some(PsetValue::Text { value: other.to_string() }),
        },
        Part21Value::Typed(_, inner) => inner.first().and_then(pset_value_from_part21),
        Part21Value::List(_) | Part21Value::Ref(_) | Part21Value::Unset | Part21Value::Derived => None,
    }
}

async fn convert_pset(ps: &IfcPropertySet) -> PropertySet {
    PropertySet { name: ps.name.clone(), properties: ps.properties.iter().filter_map(|p| pset_value_from_part21(&p.value).map(|value| Property { key: p.name.clone(), value })).collect() }
}
//#endregion 🔖️PropertyValue

//#region 🔖️Walk
async fn guid_of(doc: &Part21Document, id: u64) -> String {
    doc.instance(id).and_then(|i| i.primary()).and_then(|(_, args)| args.first()).and_then(Part21Value::as_str).map(str::to_string).unwrap_or_else(|| format!("ifc-{id}"))
}

/// 🌳️ Recursively converts one `analyze_spatial` tree node into `spatial`/`elements`/`relations`
/// rows, tracking the nearest spatial ancestor's already-converted `model` id (`None` at the
/// project root). See the module doc comment for every documented gap this walk introduces.
#[allow(clippy::too_many_arguments)]
async fn walk(doc: &Part21Document, node: &IfcSpatialNode, parent_spatial_id: Option<String>, analysis: &SpatialAnalysis, out_spatial: &mut Vec<SpatialNode>, out_elements: &mut Vec<SemioModelElement>, out_relations: &mut Vec<ModelRelation>) {
    let placement = node.object_placement.and_then(|pid| analysis.placements.get(&pid)).map(transform_from_mat4).unwrap_or_else(SemioTransform::identity);

    if let Some(kind) = spatial_kind_of(&node.ifc_type) {
        let id = guid_of(doc, node.id);
        out_spatial.push(SpatialNode { id: id.clone(), kind, name: node.name.clone().unwrap_or_default(), parent_id: parent_spatial_id.clone(), placement });
        if let Some(parent) = &parent_spatial_id {
            out_relations.push(ModelRelation { id: format!("rel-aggregates-{parent}-{id}"), kind: RelationKind::Aggregates, from: id.clone(), to: parent.clone() });
        }
        for child in &node.children {
            walk(doc, child, Some(id.clone()), analysis, out_spatial, out_elements, out_relations);
        }
    } else if node.ifc_type.eq_ignore_ascii_case("IFCPROJECT") {
        for child in &node.children {
            walk(doc, child, parent_spatial_id.clone(), analysis, out_spatial, out_elements, out_relations);
        }
    } else {
        let id = guid_of(doc, node.id);
        let class = element_class_from_ifc_type(&node.ifc_type);
        let psets = analysis.property_sets.get(&node.id).map(|v| v.iter().map(convert_pset).collect()).unwrap_or_default();
        out_elements.push(SemioModelElement { id: id.clone(), class, placement, geometry: GeometryRef::None, spatial_id: parent_spatial_id.clone(), psets });
        if let Some(parent) = &parent_spatial_id {
            out_relations.push(ModelRelation { id: format!("rel-containedin-{id}-{parent}"), kind: RelationKind::ContainedIn, from: id.clone(), to: parent.clone() });
        }
        for child in &node.children {
            // 🧩️ Flattened: a nested element (e.g. an opening) attaches to the SAME spatial
            // ancestor, not to `id` — `model.elements` has no element-parent field.
            walk(doc, child, parent_spatial_id.clone(), analysis, out_spatial, out_elements, out_relations);
        }
    }
}
//#endregion 🔖️Walk

//#region 🔖️Entry
pub async fn model_from_ifc(from: &IfcSnapshot) -> SemioModelSnapshot {
    let doc = crate::artifacts::ifc::schema::snapshot::to_part21_document(from);
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
mod tests {
    use super::*;

    /// 🏗️ Same 4-level (project/site/building/storey) + wall + Pset_WallCommon fixture as ifc's
    /// own `engine::spatial` test module — a real, non-trivial IFC4 document.
    const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('gid-project',#2,'Demo Project',$,$,$,$,(#10),#11);\n#2=IFCOWNERHISTORY($,$,$,$,$,$,$,0);\n#10=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#40,$);\n#40=IFCAXIS2PLACEMENT3D(#42,$,$);\n#42=IFCCARTESIANPOINT((0.,0.,0.));\n#11=IFCUNITASSIGNMENT((#41));\n#41=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n#3=IFCSITE('gid-site',#2,'Demo Site',$,$,#50,$,$,.ELEMENT.,$,$,$,$,$);\n#50=IFCLOCALPLACEMENT($,#51);\n#51=IFCAXIS2PLACEMENT3D(#42,$,$);\n#4=IFCBUILDING('gid-building',#2,'Demo Building',$,$,#60,$,$,.ELEMENT.,$,$,$);\n#60=IFCLOCALPLACEMENT(#50,#61);\n#61=IFCAXIS2PLACEMENT3D(#62,$,$);\n#62=IFCCARTESIANPOINT((0.,0.,10.));\n#5=IFCBUILDINGSTOREY('gid-storey',#2,'Ground Floor',$,$,#70,$,$,.ELEMENT.,3.);\n#70=IFCLOCALPLACEMENT(#60,#71);\n#71=IFCAXIS2PLACEMENT3D(#72,$,$);\n#72=IFCCARTESIANPOINT((0.,0.,0.));\n#6=IFCWALL('gid-wall',#2,'Wall-01',$,$,#80,$,$,$);\n#80=IFCLOCALPLACEMENT(#70,#81);\n#81=IFCAXIS2PLACEMENT3D(#82,$,$);\n#82=IFCCARTESIANPOINT((1.,2.,0.));\n#100=IFCRELAGGREGATES('agg-1',#2,$,$,#1,(#3));\n#101=IFCRELAGGREGATES('agg-2',#2,$,$,#3,(#4));\n#102=IFCRELAGGREGATES('agg-3',#2,$,$,#4,(#5));\n#103=IFCRELCONTAINEDINSPATIALSTRUCTURE('cont-1',#2,$,$,(#6),#5);\n#200=IFCPROPERTYSET('pset-1',#2,'Pset_WallCommon',$,(#201));\n#201=IFCPROPERTYSINGLEVALUE('IsExternal',$,IFCBOOLEAN(.T.),$);\n#202=IFCRELDEFINESBYPROPERTIES('rel-1',#2,$,$,(#6),#200);\nENDSEC;\nEND-ISO-10303-21;\n";

    async fn fixture_snapshot() -> IfcSnapshot {
        let doc = crate::artifacts::step::engine::part21::parse_part21(FIXTURE).expect("parse fixture");
        crate::artifacts::ifc::schema::snapshot::from_part21_document(crate::artifacts::ifc::STDIO_IFC_DOCUMENT_SCHEMA, &doc)
    }

    #[test]
    async fn spatial_tree_and_element_map_from_a_real_ifc4_document() {
        let model = model_from_ifc(&fixture_snapshot());
        assert_eq!(model.spatial.len(), 3, "site/building/storey, project dropped");
        let site = model.spatial.iter().find(|n| n.kind == SpatialKind::Site).expect("site");
        assert_eq!(site.name, "Demo Site");
        assert!(site.parent_id.is_none(), "site is a root (project has no SpatialKind)");
        let building = model.spatial.iter().find(|n| n.kind == SpatialKind::Building).expect("building");
        assert_eq!(building.parent_id.as_deref(), Some(site.id.as_str()));
        let storey = model.spatial.iter().find(|n| n.kind == SpatialKind::Storey).expect("storey");
        assert_eq!(storey.parent_id.as_deref(), Some(building.id.as_str()));

        assert_eq!(model.elements.len(), 1);
        let wall = &model.elements[0];
        assert_eq!(wall.class, ElementClass::Wall);
        assert_eq!(wall.spatial_id.as_deref(), Some(storey.id.as_str()));
        assert_eq!(wall.geometry, GeometryRef::None, "geometry resolution is out of this bridge's scope");
        assert!((wall.placement.translation.x - 1.0).abs() < 1e-9, "world placement composed across 4 levels: {:?}", wall.placement);
        assert!((wall.placement.translation.z - 10.0).abs() < 1e-9);
        assert_eq!(wall.psets.len(), 1);
        assert_eq!(wall.psets[0].name, "Pset_WallCommon");
        assert_eq!(wall.psets[0].properties[0], Property { key: "IsExternal".into(), value: PsetValue::Boolean { value: true } });

        assert!(model.relations.iter().any(|r| r.kind == RelationKind::Aggregates && r.from == building.id && r.to == site.id));
        assert!(model.relations.iter().any(|r| r.kind == RelationKind::ContainedIn && r.from == wall.id && r.to == storey.id));
    }

    #[test]
    async fn unscalar_property_values_are_skipped_not_fabricated() {
        assert_eq!(pset_value_from_part21(&Part21Value::Unset), None);
        assert_eq!(pset_value_from_part21(&Part21Value::List(vec![])), None);
        assert_eq!(pset_value_from_part21(&Part21Value::Ref(1)), None);
    }
}
//#endregion 🧪️Tests
