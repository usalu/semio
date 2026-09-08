//! 🏛️ IFC4 spatial structure + placement matrices + property sets — analyzer view derived from
//! the shared generic Part-21 graph (`step::engine::part21`, reused verbatim: IFC is STEP syntax
//! + a different EXPRESS schema). Walks `IfcRelAggregates`/`IfcRelContainedInSpatialStructure`
//! for the spatial tree, composes `IfcLocalPlacement`→`IfcAxis2Placement3D` chains into real 4x4
//! world matrices, and `IfcRelDefinesByProperties`→`IfcPropertySet`→`IfcPropertySingleValue` for
//! property sets. Matrix composition order was pre-verified via a standalone scratch binary per
//! this session's own convention (ticket `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`).

use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Value};
use std::collections::{HashMap, HashSet};

//#region 🔖️Model
/// 🌳️ One node of the `IfcRelAggregates`/`IfcRelContainedInSpatialStructure` decomposition tree.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(transparent)]
pub struct Mat4(pub [[f64; 4]; 4]);

impl Default for Mat4 {
    fn default() -> Self {
        Mat4::identity()
    }
}

impl Mat4 {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn identity() -> Self {
        Mat4([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]])
    }
    /// ✖️ `self * other` (self applied after other — i.e. `self` is the outer/parent transform).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn mul(&self, other: &Mat4) -> Mat4 {
        let (a, b) = (&self.0, &other.0);
        let mut out = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                out[i][j] = (0..4).map(|k| a[i][k] * b[k][j]).sum();
            }
        }
        Mat4(out)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let m = &self.0;
        [m[0][0] * p[0] + m[0][1] * p[1] + m[0][2] * p[2] + m[0][3], m[1][0] * p[0] + m[1][1] * p[1] + m[1][2] * p[2] + m[1][3], m[2][0] * p[0] + m[2][1] * p[1] + m[2][2] * p[2] + m[2][3]]
    }
}

/// 🏷️ One `IfcPropertySingleValue` — value kept as the raw generic `Part21Value` (may be a
/// `Typed` wrapper like `IFCLENGTHMEASURE(3000.)`), nothing schema-narrowed away.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PropertyValue {
    pub name: String,
    pub value: Part21Value,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PropertySet {
    pub id: u64,
    pub name: String,
    pub properties: Vec<PropertyValue>,
}

/// 🧐️ Full spatial/placement/pset analysis of an IFC4 Part-21 document.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arg_ref(args: &[Part21Value], idx: usize) -> Option<u64> {
    args.get(idx).and_then(Part21Value::as_ref_id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arg_refs(args: &[Part21Value], idx: usize) -> Vec<u64> {
    args.get(idx).and_then(Part21Value::as_list).map(|items| items.iter().filter_map(Part21Value::as_ref_id).collect()).unwrap_or_default()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arg_str(args: &[Part21Value], idx: usize) -> Option<String> {
    args.get(idx).and_then(Part21Value::as_str).map(str::to_string)
}
//#endregion 🔖️ArgHelpers

//#region 🔖️SpatialTree
/// 🔗️ Builds parent->children edges from both relationship kinds — `IfcRelAggregates`
/// (project→site→building→storey) and `IfcRelContainedInSpatialStructure` (storey→elements).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn collect_children(doc: &Part21Document) -> HashMap<u64, Vec<u64>> {
    let mut children: HashMap<u64, Vec<u64>> = HashMap::new();
    for rel in doc.by_type("IFCRELAGGREGATES") {
        if let Some(args) = rel.entity("IFCRELAGGREGATES") {
            if let Some(parent) = arg_ref(args, 4) {
                children.entry(parent).or_default().extend(arg_refs(args, 5));
            }
        }
    }
    for rel in doc.by_type("IFCRELCONTAINEDINSPATIALSTRUCTURE") {
        if let Some(args) = rel.entity("IFCRELCONTAINEDINSPATIALSTRUCTURE") {
            if let Some(parent) = arg_ref(args, 5) {
                children.entry(parent).or_default().extend(arg_refs(args, 4));
            }
        }
    }
    children
}

/// 🌳️ Recursively builds a `SpatialNode` — `IfcRoot`'s `Name` is always attribute index 2 and
/// `IfcProduct`'s `ObjectPlacement` is always index 5, regardless of which concrete entity type
/// (both are supertype attributes declared before any subtype-specific ones).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_node(doc: &Part21Document, id: u64, children_map: &HashMap<u64, Vec<u64>>, seen: &mut HashSet<u64>) -> Option<SpatialNode> {
    if !seen.insert(id) {
        return None; // cycle guard: never revisit the same instance
    }
    let inst = doc.instance(id)?;
    let (ifc_type, args) = inst.primary()?;
    let name = arg_str(args, 2).filter(|n| !n.is_empty());
    let object_placement = arg_ref(args, 5);
    let children = children_map.get(&id).into_iter().flatten().filter_map(|&kid| build_node(doc, kid, children_map, seen)).collect();
    Some(SpatialNode { id, ifc_type: ifc_type.to_string(), name, object_placement, children })
}
//#endregion 🔖️SpatialTree

//#region 🔖️Placements
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cartesian_point(doc: &Part21Document, id: u64) -> Option<[f64; 3]> {
    let args = doc.instance(id)?.entity("IFCCARTESIANPOINT")?;
    let coords = args.first()?.as_list()?;
    Some([coords.first().and_then(Part21Value::as_real).unwrap_or(0.0), coords.get(1).and_then(Part21Value::as_real).unwrap_or(0.0), coords.get(2).and_then(Part21Value::as_real).unwrap_or(0.0)])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn direction(doc: &Part21Document, id: u64) -> Option<[f64; 3]> {
    let args = doc.instance(id)?.entity("IFCDIRECTION")?;
    let r = args.first()?.as_list()?;
    Some([r.first().and_then(Part21Value::as_real).unwrap_or(0.0), r.get(1).and_then(Part21Value::as_real).unwrap_or(0.0), r.get(2).and_then(Part21Value::as_real).unwrap_or(0.0)])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn norm3(a: [f64; 3]) -> f64 {
    dot3(a, a).sqrt()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-12 {
        a
    } else {
        scale3(a, 1.0 / n)
    }
}

/// 🧭️ Builds a local transform from an `IfcAxis2Placement3D` (Location + optional Axis=local Z,
/// RefDirection=local X hint) via Gram-Schmidt, matching the pre-verified scratch algorithm.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_axis2placement(location: [f64; 3], axis_z: Option<[f64; 3]>, ref_x: Option<[f64; 3]>) -> Mat4 {
    let z = normalize3(axis_z.unwrap_or([0.0, 0.0, 1.0]));
    let x_hint = ref_x.unwrap_or([1.0, 0.0, 0.0]);
    let x_proj = sub3(x_hint, scale3(z, dot3(x_hint, z)));
    let x = if norm3(x_proj) < 1e-9 {
        let fallback = if z[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        normalize3(sub3(fallback, scale3(z, dot3(fallback, z))))
    } else {
        normalize3(x_proj)
    };
    let y = cross3(z, x);
    Mat4([[x[0], y[0], z[0], location[0]], [x[1], y[1], z[1], location[1]], [x[2], y[2], z[2], location[2]], [0.0, 0.0, 0.0, 1.0]])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn axis2placement3d_matrix(doc: &Part21Document, id: u64) -> Option<Mat4> {
    let args = doc.instance(id)?.entity("IFCAXIS2PLACEMENT3D")?;
    let location = cartesian_point(doc, arg_ref(args, 0)?)?;
    let axis_z = arg_ref(args, 1).and_then(|r| direction(doc, r));
    let ref_x = arg_ref(args, 2).and_then(|r| direction(doc, r));
    Some(build_axis2placement(location, axis_z, ref_x))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_placement(doc: &Part21Document, id: u64, memo: &mut HashMap<u64, Mat4>, visiting: &mut Vec<u64>, issues: &mut Vec<String>) -> Mat4 {
    if let Some(m) = memo.get(&id) {
        return m.clone();
    }
    if visiting.contains(&id) {
        issues.push(format!("cyclic IfcLocalPlacement chain detected at #{id}"));
        return Mat4::identity();
    }
    visiting.push(id);
    let result = resolve_placement_inner(doc, id, memo, visiting, issues);
    visiting.pop();
    memo.insert(id, result.clone());
    result
}

/// ➡️ `world = parent_world * local` — parent transforms local's coordinate frame (proven the
/// correct order, not `local * parent`, by the standalone scratch binary's order-discriminator case).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_placement_inner(doc: &Part21Document, id: u64, memo: &mut HashMap<u64, Mat4>, visiting: &mut Vec<u64>, issues: &mut Vec<String>) -> Mat4 {
    let Some(inst) = doc.instance(id) else {
        issues.push(format!("missing instance #{id}"));
        return Mat4::identity();
    };
    let Some(args) = inst.entity("IFCLOCALPLACEMENT") else {
        issues.push(format!("#{id} is not an IFCLOCALPLACEMENT"));
        return Mat4::identity();
    };
    let rel_to = arg_ref(args, 0);
    let Some(rel_placement_ref) = arg_ref(args, 1) else {
        issues.push(format!("#{id} missing RelativePlacement"));
        return Mat4::identity();
    };
    let Some(local) = axis2placement3d_matrix(doc, rel_placement_ref) else {
        issues.push(format!("#{id} could not resolve RelativePlacement geometry"));
        return Mat4::identity();
    };
    let parent = match rel_to {
        Some(parent_id) => resolve_placement(doc, parent_id, memo, visiting, issues),
        None => Mat4::identity(),
    };
    parent.mul(&local)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn compute_all_placements(doc: &Part21Document) -> (HashMap<u64, Mat4>, Vec<String>) {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn single_value_property(doc: &Part21Document, id: u64) -> Option<PropertyValue> {
    let args = doc.instance(id)?.entity("IFCPROPERTYSINGLEVALUE")?;
    let name = arg_str(args, 0)?;
    let value = args.get(2)?.clone();
    Some(PropertyValue { name, value })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn property_set(doc: &Part21Document, id: u64) -> Option<PropertySet> {
    let args = doc.instance(id)?.entity("IFCPROPERTYSET")?;
    let name = arg_str(args, 2).unwrap_or_default();
    let properties = arg_refs(args, 4).into_iter().filter_map(|pid| single_value_property(doc, pid)).collect();
    Some(PropertySet { id, name, properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn compute_property_sets(doc: &Part21Document) -> HashMap<u64, Vec<PropertySet>> {
    let mut out: HashMap<u64, Vec<PropertySet>> = HashMap::new();
    for rel in doc.by_type("IFCRELDEFINESBYPROPERTIES") {
        let Some(args) = rel.entity("IFCRELDEFINESBYPROPERTIES") else { continue };
        let Some(pset) = arg_ref(args, 5).and_then(|pid| property_set(doc, pid)) else { continue };
        for obj in arg_refs(args, 4) {
            out.entry(obj).or_default().push(pset.clone());
        }
    }
    out
}
//#endregion 🔖️PropertySets

//#region 🔖️Analyze
/// 🧐️ Full spatial-structure + placement + property-set analysis of an IFC4 document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn analyze_spatial(doc: &Part21Document) -> SpatialAnalysis {
    let children_map = collect_children(doc);
    let mut seen = HashSet::new();
    let roots: Vec<SpatialNode> = doc.by_type("IFCPROJECT").filter_map(|p| build_node(doc, p.id, &children_map, &mut seen)).collect();
    let (placements, mut issues) = compute_all_placements(doc);
    let property_sets = compute_property_sets(doc);
    if roots.is_empty() && doc.by_type("IFCPROJECT").next().is_none() && !doc.instances.is_empty() {
        issues.push("no IFCPROJECT root found".into());
    }
    SpatialAnalysis { roots, placements, property_sets, issues }
}
//#endregion 🔖️Analyze

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
