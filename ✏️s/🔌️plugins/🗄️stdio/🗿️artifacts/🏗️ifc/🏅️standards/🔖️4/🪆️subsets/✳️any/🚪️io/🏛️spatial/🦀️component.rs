//! 🏛️ IFC4 spatial structure + placement matrices + property sets — analyzer view derived from
//! the shared generic Part-21 graph (`step::engine::part21`, reused verbatim: IFC is STEP syntax
//! + a different EXPRESS schema). Walks `IfcRelAggregates`/`IfcRelContainedInSpatialStructure`
//! for the spatial tree, composes `IfcLocalPlacement`→`IfcAxis2Placement3D` chains into real 4x4
//! world matrices, and `IfcRelDefinesByProperties`→`IfcPropertySet`→`IfcPropertySingleValue` for
//! property sets. Matrix composition order was pre-verified via a standalone scratch binary per
//! this session's own convention (ticket `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`).

use crate::artifacts::step::engine::part21::{Part21Document, Part21Value};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

//#region 🔖️Model
/// 🌳️ One node of the `IfcRelAggregates`/`IfcRelContainedInSpatialStructure` decomposition tree.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpatialNode {
    pub id: u64,
    pub ifc_type: String,
    pub name: Option<String>,
    /// 🔗️ `#id` of this node's own `IfcLocalPlacement`, if it has one — look it up in
    /// `SpatialAnalysis::placements` for its composed world matrix.
    pub object_placement: Option<u64>,
    pub children: Vec<SpatialNode>,
}

/// 🧮️ Row-major affine 4x4 matrix; point transform is `p' = M * p`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mat4(pub [[f64; 4]; 4]);

impl Default for Mat4 {
    fn default() -> Self {
        Mat4::identity()
    }
}

impl Mat4 {
    pub async fn identity() -> Self {
        Mat4([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]])
    }
    /// ✖️ `self * other` (self applied after other — i.e. `self` is the outer/parent transform).
    pub async fn mul(&self, other: &Mat4) -> Mat4 {
        let (a, b) = (&self.0, &other.0);
        let mut out = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                out[i][j] = (0..4).map(|k| a[i][k] * b[k][j]).sum();
            }
        }
        Mat4(out)
    }
    pub async fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let m = &self.0;
        [m[0][0] * p[0] + m[0][1] * p[1] + m[0][2] * p[2] + m[0][3], m[1][0] * p[0] + m[1][1] * p[1] + m[1][2] * p[2] + m[1][3], m[2][0] * p[0] + m[2][1] * p[1] + m[2][2] * p[2] + m[2][3]]
    }
}

/// 🏷️ One `IfcPropertySingleValue` — value kept as the raw generic `Part21Value` (may be a
/// `Typed` wrapper like `IFCLENGTHMEASURE(3000.)`), nothing schema-narrowed away.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyValue {
    pub name: String,
    pub value: Part21Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertySet {
    pub id: u64,
    pub name: String,
    pub properties: Vec<PropertyValue>,
}

/// 🧐️ Full spatial/placement/pset analysis of an IFC4 Part-21 document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpatialAnalysis {
    pub roots: Vec<SpatialNode>,
    /// 🗺️ `IfcLocalPlacement` id -> composed world matrix.
    pub placements: HashMap<u64, Mat4>,
    /// 🗺️ Element id -> property sets attached to it via `IfcRelDefinesByProperties`.
    pub property_sets: HashMap<u64, Vec<PropertySet>>,
    pub issues: Vec<String>,
}
//#endregion 🔖️Model

//#region 🔖️ArgHelpers
async fn arg_ref(args: &[Part21Value], idx: usize) -> Option<u64> {
    args.get(idx).and_then(Part21Value::as_ref_id)
}
async fn arg_refs(args: &[Part21Value], idx: usize) -> Vec<u64> {
    args.get(idx).and_then(Part21Value::as_list).map(|items| items.iter().filter_map(Part21Value::as_ref_id).collect()).unwrap_or_default()
}
async fn arg_str(args: &[Part21Value], idx: usize) -> Option<String> {
    args.get(idx).and_then(Part21Value::as_str).map(str::to_string)
}
//#endregion 🔖️ArgHelpers

//#region 🔖️SpatialTree
/// 🔗️ Builds parent->children edges from both relationship kinds — `IfcRelAggregates`
/// (project→site→building→storey) and `IfcRelContainedInSpatialStructure` (storey→elements).
async fn collect_children(doc: &Part21Document) -> HashMap<u64, Vec<u64>> {
    let mut children: HashMap<u64, Vec<u64>> = HashMap::new();
    for rel in doc.by_type("IFCRELAGGREGATES") {
        if let Some(args) = rel.entity("IFCRELAGGREGATES") {
            if let Some(parent) = arg_ref(args, 4).await {
                children.entry(parent).or_default().extend(arg_refs(args, 5));
            }
        }
    }
    for rel in doc.by_type("IFCRELCONTAINEDINSPATIALSTRUCTURE") {
        if let Some(args) = rel.entity("IFCRELCONTAINEDINSPATIALSTRUCTURE") {
            if let Some(parent) = arg_ref(args, 5).await {
                children.entry(parent).or_default().extend(arg_refs(args, 4));
            }
        }
    }
    children
}

/// 🌳️ Recursively builds a `SpatialNode` — `IfcRoot`'s `Name` is always attribute index 2 and
/// `IfcProduct`'s `ObjectPlacement` is always index 5, regardless of which concrete entity type
/// (both are supertype attributes declared before any subtype-specific ones).
async fn build_node(doc: &Part21Document, id: u64, children_map: &HashMap<u64, Vec<u64>>, seen: &mut HashSet<u64>) -> Option<SpatialNode> {
    if !seen.insert(id) {
        return None; // cycle guard: never revisit the same instance
    }
    let inst = doc.instance(id).await?;
    let (ifc_type, args) = inst.primary().await?;
    let name = arg_str(args, 2).await.filter(|n| !n.is_empty());
    let object_placement = arg_ref(args, 5);
    let children = children_map.get(&id).into_iter().flatten().filter_map(|&kid| semio_framework_plugin::resolve_ready(build_node(doc, kid, children_map, seen))).collect();
    Some(SpatialNode { id, ifc_type: ifc_type.to_string(), name, object_placement, children })
}
//#endregion 🔖️SpatialTree

//#region 🔖️Placements
async fn cartesian_point(doc: &Part21Document, id: u64) -> Option<[f64; 3]> {
    let args = doc.instance(id).await?.entity("IFCCARTESIANPOINT").await?;
    let coords = args.first()?.as_list().await?;
    Some([coords.first().and_then(Part21Value::as_real).unwrap_or(0.0), coords.get(1).and_then(Part21Value::as_real).unwrap_or(0.0), coords.get(2).and_then(Part21Value::as_real).unwrap_or(0.0)])
}

async fn direction(doc: &Part21Document, id: u64) -> Option<[f64; 3]> {
    let args = doc.instance(id).await?.entity("IFCDIRECTION").await?;
    let r = args.first()?.as_list().await?;
    Some([r.first().and_then(Part21Value::as_real).unwrap_or(0.0), r.get(1).and_then(Part21Value::as_real).unwrap_or(0.0), r.get(2).and_then(Part21Value::as_real).unwrap_or(0.0)])
}

async fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
async fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
async fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
async fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
async fn norm3(a: [f64; 3]) -> f64 {
    dot3(a, a).await.sqrt()
}
async fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-12 {
        a
    } else {
        scale3(a, 1.0 / n).await
    }
}

/// 🧭️ Builds a local transform from an `IfcAxis2Placement3D` (Location + optional Axis=local Z,
/// RefDirection=local X hint) via Gram-Schmidt, matching the pre-verified scratch algorithm.
async fn build_axis2placement(location: [f64; 3], axis_z: Option<[f64; 3]>, ref_x: Option<[f64; 3]>) -> Mat4 {
    let z = normalize3(axis_z.unwrap_or([0.0, 0.0, 1.0]));
    let x_hint = ref_x.unwrap_or([1.0, 0.0, 0.0]);
    let x_proj = sub3(x_hint, scale3(z.await, dot3(x_hint, z.await).await).await);
    let x = if norm3(x_proj.await) < 1e-9 {
        let fallback = if z[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        normalize3(sub3(fallback, scale3(z.await, dot3(fallback, z.await).await).await).await)
    } else {
        normalize3(x_proj.await)
    };
    let y = cross3(z.await, x);
    Mat4([[x[0], y[0], z[0], location[0]], [x[1], y[1], z[1], location[1]], [x[2], y[2], z[2], location[2]], [0.0, 0.0, 0.0, 1.0]])
}

async fn axis2placement3d_matrix(doc: &Part21Document, id: u64) -> Option<Mat4> {
    let args = doc.instance(id).await?.entity("IFCAXIS2PLACEMENT3D").await?;
    let location = cartesian_point(doc, arg_ref(args, 0).await?).await?;
    let axis_z = arg_ref(args, 1).await.and_then(|r| direction(doc, r));
    let ref_x = arg_ref(args, 2).await.and_then(|r| direction(doc, r));
    Some(build_axis2placement(location, axis_z, ref_x).await)
}

async fn resolve_placement(doc: &Part21Document, id: u64, memo: &mut HashMap<u64, Mat4>, visiting: &mut Vec<u64>, issues: &mut Vec<String>) -> Mat4 {
    if let Some(m) = memo.get(&id) {
        return m.clone();
    }
    if visiting.contains(&id) {
        issues.push(format!("cyclic IfcLocalPlacement chain detected at #{id}"));
        return Mat4::identity().await;
    }
    visiting.push(id);
    let result = resolve_placement_inner(doc, id, memo, visiting, issues);
    visiting.pop();
    memo.insert(id, result.clone());
    result.await
}

/// ➡️ `world = parent_world * local` — parent transforms local's coordinate frame (proven the
/// correct order, not `local * parent`, by the standalone scratch binary's order-discriminator case).
async fn resolve_placement_inner(doc: &Part21Document, id: u64, memo: &mut HashMap<u64, Mat4>, visiting: &mut Vec<u64>, issues: &mut Vec<String>) -> Mat4 {
    let Some(inst) = doc.instance(id).await else {
        issues.push(format!("missing instance #{id}"));
        return Mat4::identity().await;
    };
    let Some(args) = inst.entity("IFCLOCALPLACEMENT").await else {
        issues.push(format!("#{id} is not an IFCLOCALPLACEMENT"));
        return Mat4::identity().await;
    };
    let rel_to = arg_ref(args, 0);
    let Some(rel_placement_ref) = arg_ref(args, 1).await else {
        issues.push(format!("#{id} missing RelativePlacement"));
        return Mat4::identity().await;
    };
    let Some(local) = axis2placement3d_matrix(doc, rel_placement_ref).await else {
        issues.push(format!("#{id} could not resolve RelativePlacement geometry"));
        return Mat4::identity().await;
    };
    let parent = match rel_to.await {
        Some(parent_id) => resolve_placement(doc, parent_id, memo, visiting, issues).await,
        None => Mat4::identity().await,
    };
    parent.mul(&local).await
}

async fn compute_all_placements(doc: &Part21Document) -> (HashMap<u64, Mat4>, Vec<String>) {
    let mut memo = HashMap::new();
    let mut visiting = Vec::new();
    let mut issues = Vec::new();
    for inst in doc.by_type("IFCLOCALPLACEMENT") {
        resolve_placement(doc, inst.id, &mut memo, &mut visiting, &mut issues);
    }
    (memo, issues)
}
//#endregion 🔖️Placements

//#region 🔖️PropertySets
async fn single_value_property(doc: &Part21Document, id: u64) -> Option<PropertyValue> {
    let args = doc.instance(id).await?.entity("IFCPROPERTYSINGLEVALUE").await?;
    let name = arg_str(args, 0).await?;
    let value = args.get(2)?.clone();
    Some(PropertyValue { name, value })
}

async fn property_set(doc: &Part21Document, id: u64) -> Option<PropertySet> {
    let args = doc.instance(id).await?.entity("IFCPROPERTYSET").await?;
    let name = arg_str(args, 2).await.unwrap_or_default();
    let properties = arg_refs(args, 4).into_iter().filter_map(|pid| single_value_property(doc, pid)).collect();
    Some(PropertySet { id, name, properties })
}

async fn compute_property_sets(doc: &Part21Document) -> HashMap<u64, Vec<PropertySet>> {
    let mut out: HashMap<u64, Vec<PropertySet>> = HashMap::new();
    for rel in doc.by_type("IFCRELDEFINESBYPROPERTIES") {
        let Some(args) = rel.entity("IFCRELDEFINESBYPROPERTIES") else { continue };
        let Some(pset) = arg_ref(args, 5).await.and_then(|pid| property_set(doc, pid)) else { continue };
        for obj in arg_refs(args, 4) {
            out.entry(obj).or_default().push(pset.clone());
        }
    }
    out
}
//#endregion 🔖️PropertySets

//#region 🔖️Analyze
/// 🧐️ Full spatial-structure + placement + property-set analysis of an IFC4 document.
pub async fn analyze_spatial(doc: &Part21Document) -> SpatialAnalysis {
    let children_map = collect_children(doc);
    let mut seen = HashSet::new();
    let roots: Vec<SpatialNode> = doc.by_type("IFCPROJECT").filter_map(|p| build_node(doc, p.id, &children_map, &mut seen)).collect();
    let (placements, mut issues) = compute_all_placements(doc).await;
    let property_sets = compute_property_sets(doc);
    if roots.is_empty() && doc.by_type("IFCPROJECT").next().is_none() && !doc.instances.is_empty() {
        issues.push("no IFCPROJECT root found".into());
    }
    SpatialAnalysis { roots, placements, property_sets, issues }
}
//#endregion 🔖️Analyze

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::engine::part21::parse_part21;

    const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('gid-project',#2,'Demo Project',$,$,$,$,(#10),#11);\n#2=IFCOWNERHISTORY($,$,$,$,$,$,$,0);\n#10=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#40,$);\n#40=IFCAXIS2PLACEMENT3D(#42,$,$);\n#42=IFCCARTESIANPOINT((0.,0.,0.));\n#11=IFCUNITASSIGNMENT((#41));\n#41=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n#3=IFCSITE('gid-site',#2,'Demo Site',$,$,#50,$,$,.ELEMENT.,$,$,$,$,$);\n#50=IFCLOCALPLACEMENT($,#51);\n#51=IFCAXIS2PLACEMENT3D(#42,$,$);\n#4=IFCBUILDING('gid-building',#2,'Demo Building',$,$,#60,$,$,.ELEMENT.,$,$,$);\n#60=IFCLOCALPLACEMENT(#50,#61);\n#61=IFCAXIS2PLACEMENT3D(#62,$,$);\n#62=IFCCARTESIANPOINT((0.,0.,10.));\n#5=IFCBUILDINGSTOREY('gid-storey',#2,'Ground Floor',$,$,#70,$,$,.ELEMENT.,3.);\n#70=IFCLOCALPLACEMENT(#60,#71);\n#71=IFCAXIS2PLACEMENT3D(#72,$,$);\n#72=IFCCARTESIANPOINT((0.,0.,0.));\n#6=IFCWALL('gid-wall',#2,'Wall-01',$,$,#80,$,$,$);\n#80=IFCLOCALPLACEMENT(#70,#81);\n#81=IFCAXIS2PLACEMENT3D(#82,$,$);\n#82=IFCCARTESIANPOINT((1.,2.,0.));\n#100=IFCRELAGGREGATES('agg-1',#2,$,$,#1,(#3));\n#101=IFCRELAGGREGATES('agg-2',#2,$,$,#3,(#4));\n#102=IFCRELAGGREGATES('agg-3',#2,$,$,#4,(#5));\n#103=IFCRELCONTAINEDINSPATIALSTRUCTURE('cont-1',#2,$,$,(#6),#5);\n#200=IFCPROPERTYSET('pset-1',#2,'Pset_WallCommon',$,(#201));\n#201=IFCPROPERTYSINGLEVALUE('IsExternal',$,IFCBOOLEAN(.T.),$);\n#202=IFCRELDEFINESBYPROPERTIES('rel-1',#2,$,$,(#6),#200);\nENDSEC;\nEND-ISO-10303-21;\n";

    async fn fixture_doc() -> Part21Document {
        parse_part21(FIXTURE).expect("parse ifc fixture")
    }

    #[semio_framework_async_macros::async_test]
    async fn spatial_hierarchy_matches_real_chain() {
        let doc = fixture_doc();
        let analysis = analyze_spatial(&doc);
        assert!(analysis.issues.is_empty(), "unexpected issues: {:?}", analysis.issues);
        assert_eq!(analysis.roots.len(), 1);
        let project = &analysis.roots[0];
        assert_eq!(project.ifc_type, "IFCPROJECT");
        assert_eq!(project.name.as_deref(), Some("Demo Project"));
        let site = &project.children[0];
        assert_eq!(site.ifc_type, "IFCSITE");
        let building = &site.children[0];
        assert_eq!(building.ifc_type, "IFCBUILDING");
        let storey = &building.children[0];
        assert_eq!(storey.ifc_type, "IFCBUILDINGSTOREY");
        let wall = &storey.children[0];
        assert_eq!(wall.ifc_type, "IFCWALL");
        assert_eq!(wall.name.as_deref(), Some("Wall-01"));
    }

    #[semio_framework_async_macros::async_test]
    async fn placement_matrix_composes_across_four_levels() {
        let doc = fixture_doc();
        let analysis = analyze_spatial(&doc);
        let wall_placement_id = analysis.roots[0].children[0].children[0].children[0].children[0].object_placement.expect("wall placement");
        assert_eq!(wall_placement_id, 80);
        let world = analysis.placements.get(&80).expect("world matrix for wall placement");
        let origin = world.transform_point([0.0, 0.0, 0.0]);
        assert!((origin[0] - 1.0).abs() < 1e-9, "x: {origin:?}");
        assert!((origin[1] - 2.0).abs() < 1e-9, "y: {origin:?}");
        assert!((origin[2] - 10.0).abs() < 1e-9, "z: {origin:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn property_set_attached_to_wall() {
        let doc = fixture_doc();
        let analysis = analyze_spatial(&doc);
        let psets = analysis.property_sets.get(&6).expect("wall property sets");
        assert_eq!(psets.len(), 1);
        assert_eq!(psets[0].name, "Pset_WallCommon");
        assert_eq!(psets[0].properties.len(), 1);
        assert_eq!(psets[0].properties[0].name, "IsExternal");
        let (typed_name, inner) = psets[0].properties[0].value.as_typed().expect("typed value");
        assert_eq!(typed_name, "IFCBOOLEAN");
        assert_eq!(inner[0].as_enum(), Some("T"));
    }

    #[semio_framework_async_macros::async_test]
    async fn cyclic_placement_is_flagged_not_infinite_loop() {
        let cyclic = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCLOCALPLACEMENT(#2,#3);\n#2=IFCLOCALPLACEMENT(#1,#3);\n#3=IFCAXIS2PLACEMENT3D(#4,$,$);\n#4=IFCCARTESIANPOINT((0.,0.,0.));\nENDSEC;\nEND-ISO-10303-21;\n";
        let doc = parse_part21(cyclic).expect("parse cyclic fixture");
        let analysis = analyze_spatial(&doc);
        assert!(analysis.issues.iter().any(|i| i.contains("cyclic")));
    }
}
//#endregion 🧪️Tests
