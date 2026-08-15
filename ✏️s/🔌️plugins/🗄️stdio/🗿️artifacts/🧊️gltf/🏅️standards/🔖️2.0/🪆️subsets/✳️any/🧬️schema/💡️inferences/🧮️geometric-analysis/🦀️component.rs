//! 🧮️ Authoritative glTF 2.0 static-pose geometric analysis.

use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{adjacency::*, area_volume::*, clearance::*, compactness::*, concavity::*, curvature::*, mass_distribution::*, orientation::*, proportion::*, roughness::*, size::*, symmetry::*, thickness::*, topology::*};
use super::super::super::modules::{measurement_contracts::*, mesh_topology::Topology};

//#region 🔖️Aggregate

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfEntityIndicators {
    pub size: GltfSizeIndicators,
    pub area_volume: GltfAreaVolumeIndicators,
    pub compactness: GltfCompactnessIndicators,
    pub proportion: GltfProportionIndicators,
    pub mass: GltfMassIndicators,
    pub curvature: GltfCurvatureIndicators,
    pub thickness: GltfThicknessIndicators,
    pub concavity: GltfConcavityIndicators,
    pub clearance: GltfClearanceIndicators,
    pub adjacency: GltfAdjacencyIndicators,
    pub orientation: GltfOrientationIndicators,
    pub symmetry: GltfSymmetryIndicators,
    pub roughness: GltfRoughnessIndicators,
    pub topology: GltfTopologyIndicators,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfPartInference {
    pub address: GltfEntityAddress,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub indicators: GltfEntityIndicators,
    pub diagnostic_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfPairInference {
    pub first: GltfEntityAddress,
    pub second: GltfEntityAddress,
    pub minimum_distance: GltfMeasure<f64>,
    pub clearance_distribution: GltfMeasure<GltfStatistics>,
    pub contact_area: GltfMeasure<f64>,
    pub interference_volume: GltfMeasure<f64>,
    pub overlap_volume: GltfMeasure<f64>,
    pub adjacent: GltfMeasure<bool>,
    pub orientation_consistency: GltfMeasure<f64>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfInferenceCounts {
    pub scene_count: u64,
    pub node_instance_count: u64,
    pub mesh_count: u64,
    pub primitive_count: u64,
    pub vertex_count: u64,
    pub triangle_count: u64,
    pub component_count: u64,
    pub surface_region_count: u64,
    pub pair_count: u64,
    pub valid_part_count: u64,
    pub invalid_part_count: u64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfGeometricInference {
    pub schema: String,
    pub schema_version: u32,
    pub policy: GltfAnalysisPolicy,
    pub counts: GltfInferenceCounts,
    pub overall: GltfEntityIndicators,
    pub parts: Vec<GltfPartInference>,
    pub pairs: Vec<GltfPairInference>,
    pub diagnostics: Vec<GltfDiagnostic>,
    pub validity: GltfValidity,
    pub quality: GltfQuality,
    pub provenance: GltfProvenance,
}
//#endregion 🔖️Aggregate

//#region 🧮️Kernel
pub(crate) type V3 = [f64; 3];
type M4 = [f64; 16];

#[derive(Clone)]
pub(crate) struct RawPart {
    pub(crate) address: GltfEntityAddress,
    pub(crate) name: Option<String>,
    pub(crate) points: Vec<V3>,
    pub(crate) triangles: Vec<[usize; 3]>,
    pub(crate) diagnostic_ids: Vec<String>,
}
fn policy() -> GltfAnalysisPolicy {
    GltfAnalysisPolicy {
        schema_version: 2,
        absolute_length_tolerance: 1e-9,
        relative_tolerance: 1e-9,
        angular_tolerance_radians: 1e-7,
        contact_tolerance: 1e-7,
        sharp_feature_angle_radians: std::f64::consts::FRAC_PI_4,
        histogram_edges: vec![0.0, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, std::f64::consts::PI],
        sampling_budget: 4096,
        sampling_seed: "s.stdio.gltf.geometry.v2".into(),
        static_pose: true,
        unit_density: true,
        fingerprint: "gltf-geometry-policy-v2-1e-9-1e-7-4096".into(),
    }
}
fn provenance(space: GltfCoordinateSpace) -> GltfProvenance {
    GltfProvenance {
        algorithm: "s.stdio.gltf.geometry".into(),
        algorithm_version: 2,
        dependency_fingerprints: Vec::new(),
        coordinate_space: space,
        tolerance_fingerprint: "gltf-geometry-policy-v2-1e-9-1e-7-4096".into(),
        sampling_seed: Some("s.stdio.gltf.geometry.v2".into()),
        pose: Some("static-node-and-mesh-morph-weights;skinning-unapplied".into()),
    }
}
fn quality(method: GltfComputationMethod, n: usize, topology: Option<Topology>) -> GltfQuality {
    let t = topology.unwrap_or(Topology { components: 0, boundary_loops: 0, chi: 0, genus: None, manifold: true, watertight: false, oriented: true });
    GltfQuality { method, coverage: if n == 0 { 0.0 } else { 1.0 }, absolute_error: None, relative_error: None, sample_count: n as u64, watertight: t.watertight, manifold: t.manifold, consistently_oriented: t.oriented, warnings: Vec::new() }
}
fn measure<T>(value: T, unit: GltfUnit, method: GltfComputationMethod, n: usize, t: Option<Topology>) -> GltfMeasure<T> {
    GltfMeasure {
        value: Some(value),
        unit,
        availability: if method == GltfComputationMethod::Exact { GltfAvailability::Available } else { GltfAvailability::Approximate },
        validity: GltfValidity::Valid,
        diagnostic_ids: Vec::new(),
        quality: quality(method, n, t),
        provenance: provenance(GltfCoordinateSpace::SceneWorld),
    }
}
pub(crate) fn unavailable<T>(unit: GltfUnit, availability: GltfAvailability, ids: Vec<String>, n: usize, t: Option<Topology>) -> GltfMeasure<T> {
    let validity = if availability == GltfAvailability::InvalidInput { GltfValidity::Invalid } else { GltfValidity::Indeterminate };
    GltfMeasure { value: None, unit, availability, validity, diagnostic_ids: ids, quality: quality(GltfComputationMethod::Exact, n, t), provenance: provenance(GltfCoordinateSpace::SceneWorld) }
}
pub(crate) fn exact<T>(v: T, u: GltfUnit, n: usize, t: Option<Topology>) -> GltfMeasure<T> {
    measure(v, u, GltfComputationMethod::Exact, n, t)
}
pub(crate) fn estimate<T>(v: T, u: GltfUnit, n: usize, t: Option<Topology>) -> GltfMeasure<T> {
    measure(v, u, GltfComputationMethod::DeterministicEstimate, n, t)
}

pub(crate) fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub(crate) fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub(crate) fn mul(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}
pub(crate) fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub(crate) fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
pub(crate) fn norm(a: V3) -> f64 {
    dot(a, a).sqrt()
}
pub(crate) fn normalize(a: V3) -> V3 {
    let n = norm(a);
    if n > 0.0 {
        mul(a, 1.0 / n)
    } else {
        [1.0, 0.0, 0.0]
    }
}
fn identity() -> M4 {
    [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
}
fn mat_mul(a: M4, b: M4) -> M4 {
    let mut c = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            c[col * 4 + row] = (0..4).map(|k| a[k * 4 + row] * b[col * 4 + k]).sum();
        }
    }
    c
}
fn transform(m: M4, p: V3) -> V3 {
    [m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12], m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13], m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14]]
}
fn node_matrix(node: &crate::artifacts::gltf::schema::snapshot::GltfNode) -> M4 {
    if let Some(m) = node.matrix {
        return m;
    }
    let t = node.translation.unwrap_or([0.0; 3]);
    let s = node.scale.unwrap_or([1.0; 3]);
    let q = node.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let l = (q.iter().map(|x| x * x).sum::<f64>()).sqrt();
    let [x, y, z, w] = if l > 0.0 { [q[0] / l, q[1] / l, q[2] / l, q[3] / l] } else { [0.0, 0.0, 0.0, 1.0] };
    [
        s[0] * (1.0 - 2.0 * (y * y + z * z)),
        s[0] * 2.0 * (x * y + z * w),
        s[0] * 2.0 * (x * z - y * w),
        0.0,
        s[1] * 2.0 * (x * y - z * w),
        s[1] * (1.0 - 2.0 * (x * x + z * z)),
        s[1] * 2.0 * (y * z + x * w),
        0.0,
        s[2] * 2.0 * (x * z + y * w),
        s[2] * 2.0 * (y * z - x * w),
        s[2] * (1.0 - 2.0 * (x * x + y * y)),
        0.0,
        t[0],
        t[1],
        t[2],
        1.0,
    ]
}
fn fingerprint(points: &[V3], triangles: &[[usize; 3]]) -> String {
    let mut h = 1469598103934665603u64;
    for p in points {
        for x in p {
            h ^= x.to_bits();
            h = h.wrapping_mul(1099511628211);
        }
    }
    for f in triangles {
        for x in f {
            h ^= *x as u64;
            h = h.wrapping_mul(1099511628211);
        }
    }
    format!("{h:016x}")
}
fn byte_fingerprint(bytes: &[u8]) -> String {
    let mut hash = 1469598103934665603u64;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{hash:016x}")
}

fn primitive_triangles(mode: u64, indices: &[usize]) -> Result<Vec<[usize; 3]>, GltfAvailability> {
    let mut out = Vec::new();
    match mode {
        4 => {
            for c in indices.chunks_exact(3) {
                out.push([c[0], c[1], c[2]])
            }
        }
        5 => {
            for i in 2..indices.len() {
                if i % 2 == 0 {
                    out.push([indices[i - 2], indices[i - 1], indices[i]])
                } else {
                    out.push([indices[i - 1], indices[i - 2], indices[i]])
                }
            }
        }
        6 => {
            for i in 2..indices.len() {
                out.push([indices[0], indices[i - 1], indices[i]])
            }
        }
        _ => return Err(GltfAvailability::UnsupportedPrimitive),
    }
    Ok(out)
}

fn decode_part(snapshot: &GltfSnapshot, mesh_index: usize, primitive_index: usize, matrix: M4, scene: Option<usize>, path: &[usize], weights: &[f64], diagnostics: &mut Vec<GltfDiagnostic>) -> Option<RawPart> {
    let mesh = snapshot.document.meshes.get(mesh_index)?;
    let primitive = mesh.primitives.get(primitive_index)?;
    let Some(position_accessor) = primitive.attributes.iter().find(|(s, _)| s == "POSITION").map(|x| x.1) else {
        let id = format!("gltf-geometry-{}", diagnostics.len());
        diagnostics.push(GltfDiagnostic {
            id,
            severity: GltfSeverity::Error,
            code: "missing-position-accessor".into(),
            message: "triangle primitive has no POSITION attribute".into(),
            paths: vec![format!("meshes/{mesh_index}/primitives/{primitive_index}/attributes")],
        });
        return None;
    };
    let Some(position_spec) = snapshot.document.accessors.get(position_accessor) else {
        let id = format!("gltf-geometry-{}", diagnostics.len());
        diagnostics.push(GltfDiagnostic {
            id,
            severity: GltfSeverity::Error,
            code: "invalid-position-accessor".into(),
            message: format!("POSITION accessor {position_accessor} is out of range"),
            paths: vec![format!("meshes/{mesh_index}/primitives/{primitive_index}/attributes/POSITION")],
        });
        return None;
    };
    if position_spec.kind != GltfAccessorType::Vec3 || position_spec.component_type != GltfComponentType::Float {
        let id = format!("gltf-geometry-{}", diagnostics.len());
        diagnostics.push(GltfDiagnostic { id, severity: GltfSeverity::Error, code: "invalid-position-accessor-type".into(), message: "POSITION must use FLOAT VEC3".into(), paths: vec![format!("accessors/{position_accessor}")] });
        return None;
    }
    let decoded = match crate::artifacts::gltf::engine::decode_accessor(&snapshot.document, &snapshot.buffers, position_accessor) {
        Ok(x) if x.components.len() % 3 == 0 => x,
        Ok(_) => return None,
        Err(message) => {
            let id = format!("gltf-geometry-{}", diagnostics.len());
            diagnostics.push(GltfDiagnostic { id, severity: GltfSeverity::Error, code: "unresolved-position-accessor".into(), message, paths: vec![format!("meshes/{mesh_index}/primitives/{primitive_index}/attributes/POSITION")] });
            return None;
        }
    };
    let mut local: Vec<V3> = decoded.components.chunks_exact(3).map(|v| [v[0], v[1], v[2]]).collect();
    for (target_index, target) in primitive.targets.iter().enumerate() {
        let weight = weights.get(target_index).copied().unwrap_or(0.0);
        if weight == 0.0 {
            continue;
        }
        let Some(accessor) = target.0.iter().find(|(s, _)| s == "POSITION").map(|x| x.1) else { continue };
        if !snapshot.document.accessors.get(accessor).is_some_and(|spec| spec.kind == GltfAccessorType::Vec3 && spec.component_type == GltfComponentType::Float && spec.count == local.len()) {
            let id = format!("gltf-geometry-{}", diagnostics.len());
            diagnostics.push(GltfDiagnostic {
                id,
                severity: GltfSeverity::Error,
                code: "invalid-morph-position-accessor-type".into(),
                message: "morph POSITION must use FLOAT VEC3 and match the base vertex count".into(),
                paths: vec![format!("accessors/{accessor}")],
            });
            continue;
        }
        match crate::artifacts::gltf::engine::decode_accessor(&snapshot.document, &snapshot.buffers, accessor) {
            Ok(delta) if delta.components.len() == local.len() * 3 => {
                for (p, d) in local.iter_mut().zip(delta.components.chunks_exact(3)) {
                    *p = add(*p, mul([d[0], d[1], d[2]], weight));
                }
            }
            Ok(_) => {}
            Err(message) => {
                let id = format!("gltf-geometry-{}", diagnostics.len());
                diagnostics.push(GltfDiagnostic { id, severity: GltfSeverity::Warning, code: "unresolved-morph-target".into(), message, paths: vec![format!("meshes/{mesh_index}/primitives/{primitive_index}/targets/{target_index}/POSITION")] });
            }
        }
    }
    let points: Vec<V3> = local.into_iter().map(|p| transform(matrix, p)).collect();
    let indices: Vec<usize> = if let Some(accessor) = primitive.indices {
        let Some(index_spec) = snapshot.document.accessors.get(accessor) else {
            let id = format!("gltf-geometry-{}", diagnostics.len());
            diagnostics.push(GltfDiagnostic {
                id,
                severity: GltfSeverity::Error,
                code: "invalid-index-accessor".into(),
                message: format!("index accessor {accessor} is out of range"),
                paths: vec![format!("meshes/{mesh_index}/primitives/{primitive_index}/indices")],
            });
            return None;
        };
        if index_spec.kind != GltfAccessorType::Scalar || !matches!(index_spec.component_type, GltfComponentType::UnsignedByte | GltfComponentType::UnsignedShort | GltfComponentType::UnsignedInt) {
            let id = format!("gltf-geometry-{}", diagnostics.len());
            diagnostics.push(GltfDiagnostic { id, severity: GltfSeverity::Error, code: "invalid-index-accessor-type".into(), message: "indices must use unsigned SCALAR components".into(), paths: vec![format!("accessors/{accessor}")] });
            return None;
        }
        match crate::artifacts::gltf::engine::decode_accessor(&snapshot.document, &snapshot.buffers, accessor) {
            Ok(v) => v.components.iter().filter_map(|x| if x.is_finite() && *x >= 0.0 && x.fract() == 0.0 { Some(*x as usize) } else { None }).collect(),
            Err(message) => {
                let id = format!("gltf-geometry-{}", diagnostics.len());
                diagnostics.push(GltfDiagnostic { id, severity: GltfSeverity::Error, code: "unresolved-index-accessor".into(), message, paths: vec![format!("meshes/{mesh_index}/primitives/{primitive_index}/indices")] });
                return None;
            }
        }
    } else {
        (0..points.len()).collect()
    };
    let triangles = match primitive_triangles(primitive.mode.unwrap_or(4), &indices) {
        Ok(t) => t.into_iter().filter(|f| f.iter().all(|i| *i < points.len()) && f[0] != f[1] && f[1] != f[2] && f[2] != f[0]).collect(),
        Err(_) => {
            let id = format!("gltf-geometry-{}", diagnostics.len());
            diagnostics.push(GltfDiagnostic {
                id,
                severity: GltfSeverity::Warning,
                code: "unsupported-primitive-mode".into(),
                message: format!("primitive mode {} is not a triangle mode", primitive.mode.unwrap_or(4)),
                paths: vec![format!("meshes/{mesh_index}/primitives/{primitive_index}/mode")],
            });
            Vec::new()
        }
    };
    let address = GltfEntityAddress {
        scope: if scene.is_some() { GltfEntityScope::NodeInstance } else { GltfEntityScope::Primitive },
        scene: scene.map(|x| x as u32),
        node_path: path.iter().map(|x| *x as u32).collect(),
        mesh: Some(mesh_index as u32),
        primitive: Some(primitive_index as u32),
        component: None,
        surface_region: None,
        content_fingerprint: fingerprint(&points, &triangles),
    };
    Some(RawPart { address, name: mesh.name.clone(), points, triangles, diagnostic_ids: Vec::new() })
}

fn collect_parts(snapshot: &GltfSnapshot, diagnostics: &mut Vec<GltfDiagnostic>) -> (Vec<RawPart>, u64) {
    fn visit(snapshot: &GltfSnapshot, scene: usize, node_index: usize, parent: M4, path: &mut Vec<usize>, stack: &mut BTreeSet<usize>, parts: &mut Vec<RawPart>, diagnostics: &mut Vec<GltfDiagnostic>, instances: &mut u64) {
        let Some(node) = snapshot.document.nodes.get(node_index) else { return };
        if !stack.insert(node_index) {
            let id = format!("gltf-geometry-{}", diagnostics.len());
            diagnostics.push(GltfDiagnostic { id, severity: GltfSeverity::Error, code: "cyclic-node-hierarchy".into(), message: format!("cycle at node {node_index}"), paths: vec![format!("nodes/{node_index}")] });
            return;
        }
        *instances += 1;
        path.push(node_index);
        let world = mat_mul(parent, node_matrix(node));
        if let Some(mesh_index) = node.mesh {
            if let Some(mesh) = snapshot.document.meshes.get(mesh_index) {
                let weights = if node.weights.is_empty() { &mesh.weights } else { &node.weights };
                for primitive_index in 0..mesh.primitives.len() {
                    if let Some(p) = decode_part(snapshot, mesh_index, primitive_index, world, Some(scene), path, weights, diagnostics) {
                        parts.push(p)
                    }
                }
            }
        }
        for child in &node.children {
            visit(snapshot, scene, *child, world, path, stack, parts, diagnostics, instances)
        }
        path.pop();
        stack.remove(&node_index);
    }
    let mut parts = Vec::new();
    let mut instances = 0;
    if snapshot.document.scenes.is_empty() {
        for (mi, mesh) in snapshot.document.meshes.iter().enumerate() {
            for pi in 0..mesh.primitives.len() {
                if let Some(p) = decode_part(snapshot, mi, pi, identity(), None, &[], &mesh.weights, diagnostics) {
                    parts.push(p)
                }
            }
        }
    } else {
        for (si, scene) in snapshot.document.scenes.iter().enumerate() {
            for root in &scene.nodes {
                visit(snapshot, si, *root, identity(), &mut Vec::new(), &mut BTreeSet::new(), &mut parts, diagnostics, &mut instances)
            }
        }
    }
    (parts, instances)
}

fn bounds(points: &[V3]) -> Option<(V3, V3, V3)> {
    let first = *points.first()?;
    let mut lo = first;
    let mut hi = first;
    for p in &points[1..] {
        for i in 0..3 {
            lo[i] = lo[i].min(p[i]);
            hi[i] = hi[i].max(p[i]);
        }
    }
    Some((lo, hi, sub(hi, lo)))
}
pub(crate) fn triangle_area(a: V3, b: V3, c: V3) -> f64 {
    0.5 * norm(cross(sub(b, a), sub(c, a)))
}

pub(crate) fn convex_hull_metrics(points: &[V3], tolerance: f64) -> Option<(f64, f64, Vec<(V3, f64)>)> {
    if points.len() < 4 {
        return None;
    }
    let mut planes = BTreeMap::<(i64, i64, i64, i64), (V3, f64)>::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                let mut n = cross(sub(points[j], points[i]), sub(points[k], points[i]));
                if norm(n) <= tolerance {
                    continue;
                }
                n = normalize(n);
                let mut d = dot(n, points[i]);
                let (mut positive, mut negative) = (false, false);
                for p in points {
                    let s = dot(n, *p) - d;
                    positive |= s > tolerance;
                    negative |= s < -tolerance;
                }
                if positive && negative {
                    continue;
                }
                if positive {
                    n = mul(n, -1.0);
                    d = -d
                }
                let q = 1e-8;
                planes.entry(((n[0] / q).round() as i64, (n[1] / q).round() as i64, (n[2] / q).round() as i64, (d / q).round() as i64)).or_insert((n, d));
            }
        }
    }
    let supporting_planes = planes.values().copied().collect::<Vec<_>>();
    let mut area = 0.0;
    let mut volume = 0.0;
    for (n, d) in &supporting_planes {
        let ids = points.iter().enumerate().filter(|(_, p)| (dot(*n, **p) - *d).abs() <= tolerance * 4.0).map(|(i, _)| i).collect::<Vec<_>>();
        if ids.len() < 3 {
            continue;
        }
        let seed = if n[0].abs() < 0.8 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let u = normalize(cross(seed, *n));
        let v = cross(*n, u);
        let mut q = ids.iter().map(|i| (dot(points[*i], u), dot(points[*i], v), *i)).collect::<Vec<_>>();
        q.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
        let turn = |a: (f64, f64, usize), b: (f64, f64, usize), c: (f64, f64, usize)| (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
        let mut h = Vec::new();
        for p in q.iter().copied() {
            while h.len() >= 2 && turn(h[h.len() - 2], h[h.len() - 1], p) <= tolerance {
                h.pop();
            }
            h.push(p)
        }
        let lower = h.len();
        for p in q.iter().rev().skip(1).copied() {
            while h.len() > lower && turn(h[h.len() - 2], h[h.len() - 1], p) <= tolerance {
                h.pop();
            }
            h.push(p)
        }
        if h.len() > 1 {
            h.pop();
        }
        if h.len() < 3 {
            continue;
        }
        let a = points[h[0].2];
        for i in 1..h.len() - 1 {
            let (b, c) = (points[h[i].2], points[h[i + 1].2]);
            area += triangle_area(a, b, c);
            volume += dot(a, cross(b, c)) / 6.0;
        }
    }
    Some((area, volume.abs(), supporting_planes))
}

pub(crate) fn hull_sample(points: &[V3], budget: usize) -> Vec<V3> {
    let limit = budget.min(32).max(4);
    if points.len() <= limit {
        return points.to_vec();
    }
    let mut selected = BTreeSet::new();
    for axis in 0..3 {
        selected.insert((0..points.len()).min_by(|a, b| points[*a][axis].total_cmp(&points[*b][axis])).unwrap());
        selected.insert((0..points.len()).max_by(|a, b| points[*a][axis].total_cmp(&points[*b][axis])).unwrap());
    }
    for slot in 0..limit {
        selected.insert(slot * (points.len() - 1) / (limit - 1));
        if selected.len() == limit {
            break;
        }
    }
    for index in 0..points.len() {
        if selected.len() == limit {
            break;
        }
        selected.insert(index);
    }
    selected.into_iter().take(limit).map(|index| points[index]).collect()
}

fn ray_triangle(origin: V3, direction: V3, a: V3, b: V3, c: V3, tolerance: f64) -> Option<f64> {
    let e1 = sub(b, a);
    let e2 = sub(c, a);
    let h = cross(direction, e2);
    let det = dot(e1, h);
    if det.abs() <= tolerance {
        return None;
    }
    let inv = 1.0 / det;
    let s = sub(origin, a);
    let u = inv * dot(s, h);
    if u < -tolerance || u > 1.0 + tolerance {
        return None;
    }
    let q = cross(s, e1);
    let v = inv * dot(direction, q);
    if v < -tolerance || u + v > 1.0 + tolerance {
        return None;
    }
    let t = inv * dot(e2, q);
    if t > tolerance {
        Some(t)
    } else {
        None
    }
}

pub(crate) fn thickness_samples(points: &[V3], faces: &[[usize; 3]], budget: usize, tolerance: f64) -> Vec<f64> {
    let mut normals = vec![[0.0; 3]; points.len()];
    for f in faces {
        let n = cross(sub(points[f[1]], points[f[0]]), sub(points[f[2]], points[f[0]]));
        for i in f {
            normals[*i] = add(normals[*i], n)
        }
    }
    let budget = budget.max(1);
    let face_target = (budget as f64).sqrt().floor().max(1.0) as usize;
    let face_step = faces.len().div_ceil(face_target).max(1);
    let sampled_faces = faces.len().div_ceil(face_step).max(1);
    let point_target = (budget / sampled_faces).max(1);
    let point_step = points.len().div_ceil(point_target).max(1);
    let mut out = Vec::new();
    for (i, p) in points.iter().enumerate().step_by(point_step) {
        let n = normalize(normals[i]);
        let mut best = f64::INFINITY;
        for direction in [n, mul(n, -1.0)] {
            for f in faces.iter().step_by(face_step) {
                if f.contains(&i) {
                    continue;
                }
                if let Some(t) = ray_triangle(*p, direction, points[f[0]], points[f[1]], points[f[2]], tolerance) {
                    best = best.min(t)
                }
            }
        }
        if best.is_finite() {
            out.push(best)
        }
    }
    out
}

pub(crate) fn roughness_samples(points: &[V3], faces: &[[usize; 3]]) -> Vec<f64> {
    let mut neighbors = vec![BTreeSet::new(); points.len()];
    for f in faces {
        for &(a, b) in &[(f[0], f[1]), (f[1], f[2]), (f[2], f[0])] {
            neighbors[a].insert(b);
            neighbors[b].insert(a);
        }
    }
    points
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if neighbors[i].is_empty() {
                return None;
            }
            let avg = mul(neighbors[i].iter().fold([0.0; 3], |s, j| add(s, points[*j])), 1.0 / neighbors[i].len() as f64);
            Some(norm(sub(*p, avg)))
        })
        .collect()
}

#[derive(Clone)]
struct Dsu {
    p: Vec<usize>,
    r: Vec<u8>,
}
impl Dsu {
    fn new(n: usize) -> Self {
        Self { p: (0..n).collect(), r: vec![0; n] }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x])
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return;
        }
        if self.r[a] < self.r[b] {
            std::mem::swap(&mut a, &mut b)
        }
        self.p[b] = a;
        if self.r[a] == self.r[b] {
            self.r[a] += 1
        }
    }
}

fn topology(points: &[V3], faces: &[[usize; 3]]) -> (Topology, Vec<V3>, Vec<[usize; 3]>, BTreeMap<(usize, usize), Vec<(usize, bool)>>) {
    let diagonal = bounds(points).map(|x| norm(x.2)).unwrap_or(0.0);
    let tol = (diagonal * 1e-9).max(1e-9);
    let mut map = BTreeMap::<(i64, i64, i64), usize>::new();
    let mut welded = Vec::new();
    let mut remap = Vec::with_capacity(points.len());
    for p in points {
        let key = ((p[0] / tol).round() as i64, (p[1] / tol).round() as i64, (p[2] / tol).round() as i64);
        let id = *map.entry(key).or_insert_with(|| {
            let i = welded.len();
            welded.push(*p);
            i
        });
        remap.push(id)
    }
    let mut clean = Vec::new();
    for f in faces {
        if f.iter().all(|i| *i < remap.len()) {
            let w = [remap[f[0]], remap[f[1]], remap[f[2]]];
            if w[0] != w[1] && w[1] != w[2] && w[2] != w[0] && triangle_area(welded[w[0]], welded[w[1]], welded[w[2]]) > tol * tol {
                clean.push(w)
            }
        }
    }
    let mut edges = BTreeMap::<(usize, usize), Vec<(usize, bool)>>::new();
    let mut used = BTreeSet::new();
    let mut dsu = Dsu::new(welded.len());
    for (fi, f) in clean.iter().enumerate() {
        for &(a, b) in &[(f[0], f[1]), (f[1], f[2]), (f[2], f[0])] {
            used.insert(a);
            used.insert(b);
            dsu.union(a, b);
            let key = (a.min(b), a.max(b));
            edges.entry(key).or_default().push((fi, a < b));
        }
    }
    let manifold = edges.values().all(|e| e.len() <= 2);
    let watertight = !clean.is_empty() && edges.values().all(|e| e.len() == 2);
    let oriented = edges.values().filter(|e| e.len() == 2).all(|e| e[0].1 != e[1].1);
    let components = used.iter().map(|i| dsu.find(*i)).collect::<BTreeSet<_>>().len() as u64;
    let mut bd = Dsu::new(welded.len());
    let mut boundary_vertices = BTreeSet::new();
    for (&(a, b), e) in &edges {
        if e.len() == 1 {
            bd.union(a, b);
            boundary_vertices.insert(a);
            boundary_vertices.insert(b);
        }
    }
    let boundary_loops = boundary_vertices.iter().map(|i| bd.find(*i)).collect::<BTreeSet<_>>().len() as u64;
    let chi = used.len() as i64 - edges.len() as i64 + clean.len() as i64;
    let g2 = 2 * components as i64 - boundary_loops as i64 - chi;
    let genus = if manifold && g2 >= 0 && g2 % 2 == 0 { Some((g2 / 2) as u64) } else { None };
    (Topology { components, boundary_loops, chi, genus, manifold, watertight, oriented }, welded, clean, edges)
}

fn point_in_closed_mesh(point: V3, points: &[V3], faces: &[[usize; 3]], tolerance: f64) -> Option<bool> {
    let directions = [normalize([1.0, 0.371, 0.529]), normalize([0.233, 1.0, 0.419]), normalize([0.317, 0.271, 1.0])];
    let mut votes = Vec::new();
    for direction in directions {
        let mut hits = faces.iter().filter_map(|face| ray_triangle(point, direction, points[face[0]], points[face[1]], points[face[2]], tolerance)).collect::<Vec<_>>();
        hits.sort_by(f64::total_cmp);
        hits.dedup_by(|a, b| (*a - *b).abs() <= tolerance * (1.0 + a.abs().max(b.abs())));
        votes.push(hits.len() % 2 == 1);
    }
    let inside = votes.iter().filter(|vote| **vote).count();
    if inside >= 2 {
        Some(true)
    } else if inside <= 1 {
        Some(false)
    } else {
        None
    }
}

fn shell_material_metrics(points: &[V3], faces: &[[usize; 3]], edge_faces: &BTreeMap<(usize, usize), Vec<(usize, bool)>>, tolerance: f64, budget: usize) -> Option<(f64, f64, f64, V3)> {
    let mut neighbors = vec![BTreeSet::new(); faces.len()];
    for incidences in edge_faces.values().filter(|incidences| incidences.len() == 2) {
        neighbors[incidences[0].0].insert(incidences[1].0);
        neighbors[incidences[1].0].insert(incidences[0].0);
    }
    let mut seen = vec![false; faces.len()];
    let mut components = Vec::<Vec<usize>>::new();
    for seed in 0..faces.len() {
        if seen[seed] {
            continue;
        }
        let mut stack = vec![seed];
        seen[seed] = true;
        let mut component = Vec::new();
        while let Some(face) = stack.pop() {
            component.push(face);
            for neighbor in &neighbors[face] {
                if !seen[*neighbor] {
                    seen[*neighbor] = true;
                    stack.push(*neighbor);
                }
            }
        }
        components.push(component);
    }
    let mut shells = Vec::<(f64, V3, V3, Vec<[usize; 3]>)>::new();
    for component in components {
        let shell_faces = component.iter().map(|index| faces[*index]).collect::<Vec<_>>();
        let mut signed_volume = 0.0;
        let mut weighted_centroid = [0.0; 3];
        for face in &shell_faces {
            let (a, b, c) = (points[face[0]], points[face[1]], points[face[2]]);
            let tetrahedron = dot(a, cross(b, c)) / 6.0;
            signed_volume += tetrahedron;
            weighted_centroid = add(weighted_centroid, mul(add(add(a, b), c), tetrahedron / 4.0));
        }
        if signed_volume.abs() <= tolerance.powi(3) {
            return None;
        }
        let centroid = mul(weighted_centroid, 1.0 / signed_volume);
        let mut representative = None;
        for face in shell_faces.iter().take(8) {
            let (a, b, c) = (points[face[0]], points[face[1]], points[face[2]]);
            let face_centroid = mul(add(add(a, b), c), 1.0 / 3.0);
            let normal = normalize(cross(sub(b, a), sub(c, a)));
            for candidate in [add(face_centroid, mul(normal, tolerance * 8.0)), sub(face_centroid, mul(normal, tolerance * 8.0))] {
                if point_in_closed_mesh(candidate, points, &shell_faces, tolerance) == Some(true) {
                    representative = Some(candidate);
                    break;
                }
            }
            if representative.is_some() {
                break;
            }
        }
        shells.push((signed_volume.abs(), centroid, representative?, shell_faces));
    }
    if shells.len().saturating_mul(faces.len()).saturating_mul(3) > budget.max(1) {
        return None;
    }
    let mut intersection_budget = budget.max(1);
    for i in 0..shells.len() {
        for j in i + 1..shells.len() {
            for first in &shells[i].3 {
                for second in &shells[j].3 {
                    if intersection_budget == 0 {
                        return None;
                    }
                    intersection_budget -= 1;
                    let first = [points[first[0]], points[first[1]], points[first[2]]];
                    let second = [points[second[0]], points[second[1]], points[second[2]]];
                    if triangle_distance(first, second) <= tolerance {
                        return None;
                    }
                }
            }
        }
    }
    let mut enclosed = 0.0;
    let mut material = 0.0;
    let mut material_centroid = [0.0; 3];
    for i in 0..shells.len() {
        let depth = (0..shells.len()).filter(|other| *other != i).filter(|other| point_in_closed_mesh(shells[i].2, points, &shells[*other].3, tolerance) == Some(true)).count();
        let sign = if depth % 2 == 0 { 1.0 } else { -1.0 };
        if depth == 0 {
            enclosed += shells[i].0
        }
        material += sign * shells[i].0;
        material_centroid = add(material_centroid, mul(shells[i].1, sign * shells[i].0));
    }
    if material <= tolerance.powi(3) {
        return None;
    }
    Some((material, enclosed, (enclosed - material).max(0.0), mul(material_centroid, 1.0 / material)))
}

pub(crate) fn statistics(values: &[f64], edges: &[f64]) -> GltfStatistics {
    if values.is_empty() {
        return GltfStatistics::default();
    }
    let mut sorted: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();
    if sorted.is_empty() {
        return GltfStatistics::default();
    }
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    let mean = sorted.iter().sum::<f64>() / n as f64;
    let variance = sorted.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / n as f64;
    let q = |p: f64| sorted[((n - 1) as f64 * p).round() as usize];
    let mut counts = vec![0u64; edges.len().saturating_sub(1)];
    for v in &sorted {
        if let Some(i) = edges.windows(2).position(|e| *v >= e[0] && *v < e[1]) {
            counts[i] += 1
        } else if *v == *edges.last().unwrap_or(v) && !counts.is_empty() {
            *counts.last_mut().unwrap() += 1
        }
    }
    GltfStatistics {
        minimum: sorted.first().copied(),
        maximum: sorted.last().copied(),
        mean: Some(mean),
        variance: Some(variance),
        standard_deviation: Some(variance.sqrt()),
        median: Some(q(0.5)),
        quantiles: vec![q(0.0), q(0.25), q(0.5), q(0.75), q(1.0)],
        histogram: Some(GltfHistogram { edges: edges.to_vec(), counts, weights: Vec::new() }),
    }
}

fn principal_frame(points: &[V3], centroid: V3) -> GltfPrincipalFrame {
    let mut a = [[0.0; 3]; 3];
    for p in points {
        let d = sub(*p, centroid);
        for i in 0..3 {
            for j in 0..3 {
                a[i][j] += d[i] * d[j] / points.len().max(1) as f64
            }
        }
    }
    let mut v = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    for _ in 0..32 {
        let mut p = 0;
        let mut q = 1;
        for i in 0..3 {
            for j in i + 1..3 {
                if a[i][j].abs() > a[p][q].abs() {
                    p = i;
                    q = j
                }
            }
        }
        if a[p][q].abs() < 1e-15 {
            break;
        }
        let phi = 0.5 * (2.0 * a[p][q]).atan2(a[q][q] - a[p][p]);
        let (c, s) = (phi.cos(), phi.sin());
        for k in 0..3 {
            let (ap, aq) = (a[k][p], a[k][q]);
            a[k][p] = c * ap - s * aq;
            a[k][q] = s * ap + c * aq
        }
        for k in 0..3 {
            let (ap, aq) = (a[p][k], a[q][k]);
            a[p][k] = c * ap - s * aq;
            a[q][k] = s * ap + c * aq;
            let (vp, vq) = (v[k][p], v[k][q]);
            v[k][p] = c * vp - s * vq;
            v[k][q] = s * vp + c * vq
        }
    }
    let mut e = [(a[0][0], [v[0][0], v[1][0], v[2][0]]), (a[1][1], [v[0][1], v[1][1], v[2][1]]), (a[2][2], [v[0][2], v[1][2], v[2][2]])];
    e.sort_by(|x, y| y.0.total_cmp(&x.0));
    for x in &mut e {
        let axis = &mut x.1;
        let k = (0..3).max_by(|i, j| axis[*i].abs().total_cmp(&axis[*j].abs())).unwrap();
        if axis[k] < 0.0 {
            *axis = mul(*axis, -1.0)
        }
    }
    GltfPrincipalFrame { centroid: GltfVec3::new(centroid), axes: e.map(|x| GltfVec3::new(normalize(x.1))), eigenvalues: e.map(|x| x.0.max(0.0)) }
}

/// 🧰️ Canonical shared geometry consumed by independent inference stages.
pub(crate) struct GltfGeometryContext<'a> {
    pub(crate) policy: &'a GltfAnalysisPolicy,
    pub(crate) topology: Topology,
    pub(crate) points: Vec<V3>,
    pub(crate) faces: Vec<[usize; 3]>,
    pub(crate) edge_faces: BTreeMap<(usize, usize), Vec<(usize, bool)>>,
    pub(crate) sample_count: usize,
    pub(crate) bounds: GltfBounds3,
    pub(crate) dimensions: V3,
    pub(crate) diagonal: f64,
    pub(crate) surface_area: f64,
    pub(crate) solid: Option<(f64, f64, f64, V3)>,
    pub(crate) volume: f64,
    pub(crate) centroid: V3,
    pub(crate) principal_frame: GltfPrincipalFrame,
    pub(crate) principal_axes: Vec<GltfDirectionScore>,
    pub(crate) oriented_bounds: GltfBounds3,
    pub(crate) oriented_extent: V3,
    pub(crate) unavailable_volume: GltfAvailability,
}

impl<'a> GltfGeometryContext<'a> {
    fn new(points: &[V3], triangles: &[[usize; 3]], policy: &'a GltfAnalysisPolicy) -> Option<Self> {
        let (topology, points, faces, edge_faces) = topology(points, triangles);
        let sample_count = points.len();
        let (lo, hi, dimensions) = bounds(&points)?;
        let bounds = GltfBounds3 { min: GltfVec3::new(lo), max: GltfVec3::new(hi), dimensions: GltfVec3::new(dimensions) };
        let diagonal = norm(dimensions);
        let surface_area = faces.iter().map(|face| triangle_area(points[face[0]], points[face[1]], points[face[2]])).sum::<f64>();
        let mut surface_centroid = [0.0; 3];
        for face in &faces {
            let (a, b, c) = (points[face[0]], points[face[1]], points[face[2]]);
            let area = triangle_area(a, b, c);
            surface_centroid = add(surface_centroid, mul(add(add(a, b), c), area / 3.0));
        }
        if surface_area > 0.0 {
            surface_centroid = mul(surface_centroid, 1.0 / surface_area);
        } else {
            surface_centroid = mul(points.iter().fold([0.0; 3], |sum, point| add(sum, *point)), 1.0 / sample_count.max(1) as f64);
        }
        let tolerance = (diagonal * policy.relative_tolerance).max(policy.absolute_length_tolerance);
        let solid = if topology.watertight && topology.manifold && topology.oriented { shell_material_metrics(&points, &faces, &edge_faces, tolerance, policy.sampling_budget as usize) } else { None };
        let volume = solid.map(|metrics| metrics.0).unwrap_or(0.0);
        let centroid = solid.map(|metrics| metrics.3).unwrap_or(surface_centroid);
        let principal_frame = principal_frame(&points, centroid);
        let principal_axes = principal_frame.axes.iter().enumerate().map(|(index, axis)| GltfDirectionScore { direction: *axis, score: principal_frame.eigenvalues[index], order: Some((index + 1) as u32) }).collect::<Vec<_>>();
        let mut oriented_extent = [0.0; 3];
        let mut oriented_min = [0.0; 3];
        let mut oriented_max = [0.0; 3];
        for (index, axis) in principal_frame.axes.iter().enumerate() {
            let axis = axis.array();
            let mut minimum = f64::INFINITY;
            let mut maximum = f64::NEG_INFINITY;
            for point in &points {
                let projection = dot(sub(*point, centroid), axis);
                minimum = minimum.min(projection);
                maximum = maximum.max(projection);
            }
            oriented_min[index] = minimum;
            oriented_max[index] = maximum;
            oriented_extent[index] = maximum - minimum;
        }
        let oriented_bounds = GltfBounds3 { min: GltfVec3::new(oriented_min), max: GltfVec3::new(oriented_max), dimensions: GltfVec3::new(oriented_extent) };
        let unavailable_volume = if !topology.manifold {
            GltfAvailability::NonManifold
        } else if !topology.watertight {
            GltfAvailability::OpenSurface
        } else {
            GltfAvailability::InvalidInput
        };
        Some(Self { policy, topology, points, faces, edge_faces, sample_count, bounds, dimensions, diagonal, surface_area, solid, volume, centroid, principal_frame, principal_axes, oriented_bounds, oriented_extent, unavailable_volume })
    }
}

fn empty_indicators(diagnostic_ids: Vec<String>) -> GltfEntityIndicators {
    GltfEntityIndicators {
        size: GltfSizeInference::unavailable(&diagnostic_ids),
        area_volume: GltfAreaVolumeInference::unavailable(&diagnostic_ids),
        compactness: GltfCompactnessInference::unavailable(&diagnostic_ids),
        proportion: GltfProportionInference::unavailable(&diagnostic_ids),
        mass: GltfMassInference::unavailable(&diagnostic_ids),
        curvature: GltfCurvatureInference::unavailable(&diagnostic_ids),
        thickness: GltfThicknessInference::unavailable(&diagnostic_ids),
        concavity: GltfConcavityInference::unavailable(&diagnostic_ids),
        clearance: GltfClearanceInference::unavailable(&diagnostic_ids),
        adjacency: GltfAdjacencyInference::unavailable(&diagnostic_ids),
        orientation: GltfOrientationInference::unavailable(&diagnostic_ids),
        symmetry: GltfSymmetryInference::unavailable(&diagnostic_ids),
        roughness: GltfRoughnessInference::unavailable(&diagnostic_ids),
        topology: GltfTopologyInference::unavailable(&diagnostic_ids),
    }
}

fn analyze(points: &[V3], triangles: &[[usize; 3]], policy: &GltfAnalysisPolicy) -> (GltfEntityIndicators, Topology) {
    let Some(context) = GltfGeometryContext::new(points, triangles, policy) else {
        return (empty_indicators(Vec::new()), topology(points, triangles).0);
    };
    let topology = context.topology;
    (
        GltfEntityIndicators {
            size: GltfSizeInference::infer(&context),
            area_volume: GltfAreaVolumeInference::infer(&context),
            compactness: GltfCompactnessInference::infer(&context),
            proportion: GltfProportionInference::infer(&context),
            mass: GltfMassInference::infer(&context),
            curvature: GltfCurvatureInference::infer(&context),
            thickness: GltfThicknessInference::infer(&context),
            concavity: GltfConcavityInference::infer(&context),
            clearance: GltfClearanceInference::infer(&context),
            adjacency: GltfAdjacencyInference::infer(&context),
            orientation: GltfOrientationInference::infer(&context),
            symmetry: GltfSymmetryInference::infer(&context),
            roughness: GltfRoughnessInference::infer(&context),
            topology: GltfTopologyInference::infer(&context),
        },
        topology,
    )
}
fn point_triangle_distance(p: V3, a: V3, b: V3, c: V3) -> f64 {
    let ab = sub(b, a);
    let ac = sub(c, a);
    let ap = sub(p, a);
    let d1 = dot(ab, ap);
    let d2 = dot(ac, ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return norm(ap);
    }
    let bp = sub(p, b);
    let d3 = dot(ab, bp);
    let d4 = dot(ac, bp);
    if d3 >= 0.0 && d4 <= d3 {
        return norm(bp);
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return norm(sub(p, add(a, mul(ab, v))));
    }
    let cp = sub(p, c);
    let d5 = dot(ab, cp);
    let d6 = dot(ac, cp);
    if d6 >= 0.0 && d5 <= d6 {
        return norm(cp);
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return norm(sub(p, add(a, mul(ac, w))));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return norm(sub(p, add(b, mul(sub(c, b), w))));
    }
    dot(ap, normalize(cross(ab, ac))).abs()
}
fn segment_distance(p1: V3, q1: V3, p2: V3, q2: V3) -> f64 {
    let d1 = sub(q1, p1);
    let d2 = sub(q2, p2);
    let r = sub(p1, p2);
    let a = dot(d1, d1);
    let e = dot(d2, d2);
    let f = dot(d2, r);
    let (mut s, mut t);
    if a <= 1e-30 && e <= 1e-30 {
        return norm(r);
    }
    if a <= 1e-30 {
        s = 0.0;
        t = (f / e).clamp(0.0, 1.0)
    } else {
        let c = dot(d1, r);
        if e <= 1e-30 {
            t = 0.0;
            s = (-c / a).clamp(0.0, 1.0)
        } else {
            let b = dot(d1, d2);
            let denom = a * e - b * b;
            s = if denom != 0.0 { ((b * f - c * e) / denom).clamp(0.0, 1.0) } else { 0.0 };
            t = (b * s + f) / e;
            if t < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0)
            } else if t > 1.0 {
                t = 1.0;
                s = ((b - c) / a).clamp(0.0, 1.0)
            }
        }
    }
    norm(sub(add(p1, mul(d1, s)), add(p2, mul(d2, t))))
}
fn triangle_distance(a: [V3; 3], b: [V3; 3]) -> f64 {
    for i in 0..3 {
        let direction = sub(a[(i + 1) % 3], a[i]);
        if ray_triangle(a[i], direction, b[0], b[1], b[2], 1e-12).is_some_and(|t| t <= 1.0 + 1e-12) {
            return 0.0;
        }
        let direction = sub(b[(i + 1) % 3], b[i]);
        if ray_triangle(b[i], direction, a[0], a[1], a[2], 1e-12).is_some_and(|t| t <= 1.0 + 1e-12) {
            return 0.0;
        }
    }
    let mut d = f64::INFINITY;
    for p in a {
        d = d.min(point_triangle_distance(p, b[0], b[1], b[2]))
    }
    for p in b {
        d = d.min(point_triangle_distance(p, a[0], a[1], a[2]))
    }
    for i in 0..3 {
        for j in 0..3 {
            d = d.min(segment_distance(a[i], a[(i + 1) % 3], b[j], b[(j + 1) % 3]))
        }
    }
    d
}

fn pair_overlap_volume(a: &RawPart, b: &RawPart, policy: &GltfAnalysisPolicy) -> Option<(f64, usize)> {
    let (topology_a, points_a, faces_a, _) = topology(&a.points, &a.triangles);
    let (topology_b, points_b, faces_b, _) = topology(&b.points, &b.triangles);
    if !topology_a.watertight || !topology_a.manifold || !topology_b.watertight || !topology_b.manifold {
        return None;
    }
    let (alo, ahi, _) = bounds(&points_a)?;
    let (blo, bhi, _) = bounds(&points_b)?;
    let lo = [alo[0].max(blo[0]), alo[1].max(blo[1]), alo[2].max(blo[2])];
    let hi = [ahi[0].min(bhi[0]), ahi[1].min(bhi[1]), ahi[2].min(bhi[2])];
    let dimensions = sub(hi, lo);
    if dimensions.iter().any(|dimension| *dimension <= policy.absolute_length_tolerance) {
        return Some((0.0, 0));
    }
    let triangle_cost = 3 * (faces_a.len() + faces_b.len()).max(1);
    let cell_budget = (policy.sampling_budget as usize / triangle_cost).max(1);
    let resolution = (cell_budget as f64).cbrt().floor().max(1.0) as usize;
    let tolerance = (norm(dimensions) * policy.relative_tolerance).max(policy.absolute_length_tolerance);
    let mut inside = 0usize;
    let mut samples = 0usize;
    for x in 0..resolution {
        for y in 0..resolution {
            for z in 0..resolution {
                let point = [lo[0] + dimensions[0] * (x as f64 + 0.5) / resolution as f64, lo[1] + dimensions[1] * (y as f64 + 0.5) / resolution as f64, lo[2] + dimensions[2] * (z as f64 + 0.5) / resolution as f64];
                samples += 1;
                if point_in_closed_mesh(point, &points_a, &faces_a, tolerance) == Some(true) && point_in_closed_mesh(point, &points_b, &faces_b, tolerance) == Some(true) {
                    inside += 1
                }
            }
        }
    }
    Some((dimensions[0] * dimensions[1] * dimensions[2] * inside as f64 / samples as f64, samples))
}

/// 🧷️ Shared pair geometry interpreted by independent pair-inference leaves.
pub(crate) struct GltfPairGeometry {
    pub(crate) first: GltfEntityAddress,
    pub(crate) second: GltfEntityAddress,
    pub(crate) minimum_distance: f64,
    pub(crate) contact_area: Option<f64>,
    pub(crate) overlap: Option<(f64, usize)>,
    pub(crate) adjacent: bool,
    pub(crate) sample_count: usize,
}

fn pair_geometry(a: &RawPart, b: &RawPart, p: &GltfAnalysisPolicy) -> Option<GltfPairGeometry> {
    let (alo, ahi, _) = bounds(&a.points)?;
    let (blo, bhi, _) = bounds(&b.points)?;
    let mut distance = f64::INFINITY;
    let mut contact_area = 0.0;
    let mut coincident_contact = false;
    let aabb_separation = (0..3)
        .map(|axis| {
            if ahi[axis] < blo[axis] {
                blo[axis] - ahi[axis]
            } else if bhi[axis] < alo[axis] {
                alo[axis] - bhi[axis]
            } else {
                0.0
            }
        })
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt();
    let budget = (p.sampling_budget as usize).max(1);
    let target_a = (budget as f64).sqrt().floor().max(1.0) as usize;
    let target_b = (budget / target_a).max(1);
    let step_a = a.triangles.len().div_ceil(target_a).max(1);
    let step_b = b.triangles.len().div_ceil(target_b).max(1);
    for fa in a.triangles.iter().step_by(step_a) {
        if !fa.iter().all(|i| *i < a.points.len()) {
            continue;
        }
        let ta = [a.points[fa[0]], a.points[fa[1]], a.points[fa[2]]];
        let mut face_distance = f64::INFINITY;
        for fb in b.triangles.iter().step_by(step_b) {
            if !fb.iter().all(|i| *i < b.points.len()) {
                continue;
            }
            let tb = [b.points[fb[0]], b.points[fb[1]], b.points[fb[2]]];
            face_distance = face_distance.min(triangle_distance(ta, tb));
            let coincident = ta.iter().all(|point| point_triangle_distance(*point, tb[0], tb[1], tb[2]) <= p.contact_tolerance) && tb.iter().all(|point| point_triangle_distance(*point, ta[0], ta[1], ta[2]) <= p.contact_tolerance);
            if coincident {
                coincident_contact = true;
                contact_area += triangle_area(ta[0], ta[1], ta[2]).min(triangle_area(tb[0], tb[1], tb[2]));
            }
        }
        distance = distance.min(face_distance);
    }
    if !distance.is_finite() {
        distance = aabb_separation
    }
    let adjacent = distance <= p.contact_tolerance;
    let n = a.points.len() + b.points.len();
    let overlap = pair_overlap_volume(a, b, p);
    Some(GltfPairGeometry {
        first: a.address.clone(),
        second: b.address.clone(),
        minimum_distance: distance,
        contact_area: if coincident_contact {
            Some(contact_area)
        } else if aabb_separation > p.contact_tolerance {
            Some(0.0)
        } else {
            None
        },
        overlap,
        adjacent,
        sample_count: n,
    })
}

/// 🧮️ Decodes and canonicalizes the static glTF pose, preserving every scene-node mesh instance.
pub fn compute_gltf_geometry(snapshot: &GltfSnapshot) -> GltfGeometricInference {
    let p = policy();
    let mut diagnostics = Vec::new();
    let (raw_parts, node_instances) = collect_parts(snapshot, &mut diagnostics);
    let mut all_points = Vec::new();
    let mut all_triangles = Vec::new();
    for part in &raw_parts {
        let offset = all_points.len();
        all_points.extend_from_slice(&part.points);
        all_triangles.extend(part.triangles.iter().map(|f| [f[0] + offset, f[1] + offset, f[2] + offset]))
    }
    let (mut overall, overall_topology) = analyze(&all_points, &all_triangles, &p);
    let mut parts = Vec::new();
    let mut component_count = 0;
    for raw in &raw_parts {
        let (indicators, t) = analyze(&raw.points, &raw.triangles, &p);
        component_count += t.components;
        parts.push(GltfPartInference { address: raw.address.clone(), name: raw.name.clone(), indicators, diagnostic_ids: raw.diagnostic_ids.clone() })
    }
    GltfSymmetryInference::infer_assembly(&mut overall.symmetry, &parts, &p, overall_topology);
    let mut pairs = Vec::new();
    for first in 0..raw_parts.len() {
        for second in first + 1..raw_parts.len() {
            if let Some(pair) = pair_geometry(&raw_parts[first], &raw_parts[second], &p) {
                let (minimum_distance, clearance_distribution, interference_volume, overlap_volume) = GltfClearanceInference::infer_pair(&pair, &p);
                pairs.push(GltfPairInference {
                    first: pair.first.clone(),
                    second: pair.second.clone(),
                    minimum_distance,
                    clearance_distribution,
                    contact_area: GltfAreaVolumeInference::infer_pair_contact(&pair),
                    interference_volume,
                    overlap_volume,
                    adjacent: GltfAdjacencyInference::infer_pair(&pair),
                    orientation_consistency: GltfOrientationInference::infer_pair(&pair),
                });
            }
        }
    }
    let distances = pairs.iter().filter_map(|pair| pair.minimum_distance.value).collect::<Vec<_>>();
    let contact_area = pairs.iter().filter_map(|pair| pair.contact_area.value).sum::<f64>();
    let contact_area_complete = pairs.iter().all(|pair| pair.contact_area.value.is_some());
    let overlap_volume = pairs.iter().filter_map(|pair| pair.overlap_volume.value).sum::<f64>();
    let overlap_complete = pairs.iter().all(|pair| pair.overlap_volume.value.is_some());
    let contacts = pairs.iter().filter(|pair| pair.adjacent.value == Some(true)).count() as u64;
    let sample_count = all_points.len();
    GltfAreaVolumeInference::infer_assembly(&mut overall.area_volume, raw_parts.len(), contact_area, contact_area_complete, sample_count, overall_topology);
    GltfAdjacencyInference::infer_assembly(&mut overall.adjacency, raw_parts.len(), contacts, sample_count, overall_topology);
    GltfOrientationInference::infer_assembly(&mut overall.orientation, raw_parts.len(), sample_count, overall_topology);
    GltfClearanceInference::infer_assembly(&mut overall.clearance, &distances, overlap_volume, overlap_complete, pairs.len(), &p, sample_count, overall_topology);
    let valid = raw_parts.iter().filter(|part| !part.triangles.is_empty()).count() as u64;
    let invalid = raw_parts.iter().filter(|part| part.triangles.is_empty()).count() as u64 + diagnostics.iter().filter(|d| d.severity == GltfSeverity::Error).count() as u64;
    let authored_primitives = snapshot.document.meshes.iter().map(|mesh| mesh.primitives.len() as u64).sum();
    let counts = GltfInferenceCounts {
        scene_count: snapshot.document.scenes.len() as u64,
        node_instance_count: node_instances,
        mesh_count: snapshot.document.meshes.len() as u64,
        primitive_count: authored_primitives,
        vertex_count: all_points.len() as u64,
        triangle_count: all_triangles.len() as u64,
        component_count,
        surface_region_count: component_count,
        pair_count: pairs.len() as u64,
        valid_part_count: valid,
        invalid_part_count: invalid,
    };
    let validity = if counts.invalid_part_count > 0 {
        GltfValidity::Invalid
    } else if counts.valid_part_count == 0 && !snapshot.document.meshes.is_empty() {
        GltfValidity::Indeterminate
    } else {
        GltfValidity::Valid
    };
    let mut q = quality(GltfComputationMethod::DeterministicEstimate, all_points.len(), Some(overall_topology));
    q.coverage = if counts.valid_part_count + counts.invalid_part_count == 0 {
        if snapshot.document.meshes.is_empty() {
            1.0
        } else {
            0.0
        }
    } else {
        counts.valid_part_count as f64 / (counts.valid_part_count + counts.invalid_part_count) as f64
    };
    let mut analysis_provenance = provenance(GltfCoordinateSpace::SceneWorld);
    analysis_provenance.dependency_fingerprints.push(format!("canonical:{}", fingerprint(&all_points, &all_triangles)));
    analysis_provenance.dependency_fingerprints.extend(snapshot.buffers.iter().enumerate().map(|(index, bytes)| format!("buffer:{index}:{}", byte_fingerprint(bytes))));
    GltfGeometricInference { schema: "s.stdio.gltf.inference".into(), schema_version: 2, policy: p, counts, overall, parts, pairs, diagnostics, validity, quality: q, provenance: analysis_provenance }
}

impl Default for GltfGeometricInference {
    fn default() -> Self {
        compute_gltf_geometry(&GltfSnapshot::default())
    }
}
//#endregion 🧮️Kernel

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
    use crate::artifacts::gltf::schema::snapshot::{GltfAccessor, GltfBuffer, GltfBufferView, GltfDocument, GltfMesh, GltfNode, GltfPrimitive, GltfScene, GltfSourceForm};
    use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;

    fn snapshot(points: &[[f32; 3]], indices: &[u32]) -> GltfSnapshot {
        let mut bytes = Vec::new();
        for p in points {
            for x in p {
                bytes.extend_from_slice(&x.to_le_bytes())
            }
        }
        let positions_len = bytes.len();
        for i in indices {
            bytes.extend_from_slice(&i.to_le_bytes())
        }
        let mut document = GltfDocument::default();
        document.buffers = vec![GltfBuffer { byte_length: bytes.len(), uri: None, name: None, extensions: None, extras: None }];
        document.buffer_views = vec![
            GltfBufferView { buffer: 0, byte_offset: 0, byte_length: positions_len, byte_stride: None, target: Some(34962), name: None, extensions: None, extras: None },
            GltfBufferView { buffer: 0, byte_offset: positions_len, byte_length: indices.len() * 4, byte_stride: None, target: Some(34963), name: None, extensions: None, extras: None },
        ];
        document.accessors = vec![
            GltfAccessor {
                buffer_view: Some(0),
                byte_offset: 0,
                component_type: GltfComponentType::Float,
                normalized: false,
                count: points.len(),
                kind: GltfAccessorType::Vec3,
                max: None,
                min: None,
                sparse: None,
                name: None,
                extensions: None,
                extras: None,
            },
            GltfAccessor {
                buffer_view: Some(1),
                byte_offset: 0,
                component_type: GltfComponentType::UnsignedInt,
                normalized: false,
                count: indices.len(),
                kind: GltfAccessorType::Scalar,
                max: None,
                min: None,
                sparse: None,
                name: None,
                extensions: None,
                extras: None,
            },
        ];
        document.meshes = vec![GltfMesh { primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), 0)], indices: Some(1), ..Default::default() }], ..Default::default() }];
        GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![bytes], source_form: GltfSourceForm::Json }
    }
    fn cuboid() -> GltfSnapshot {
        snapshot(&[[0., 0., 0.], [2., 0., 0.], [2., 3., 0.], [0., 3., 0.], [0., 0., 4.], [2., 0., 4.], [2., 3., 4.], [0., 3., 4.]], &[0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5])
    }
    fn append_cube(points: &mut Vec<[f32; 3]>, indices: &mut Vec<u32>, lo: f32, hi: f32) {
        let offset = points.len() as u32;
        points.extend([[lo, lo, lo], [hi, lo, lo], [hi, hi, lo], [lo, hi, lo], [lo, lo, hi], [hi, lo, hi], [hi, hi, hi], [lo, hi, hi]]);
        indices.extend([0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5].map(|index| index + offset));
    }
    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{a} != {b}")
    }

    #[test]
    fn cuboid_exact_core_metrics() {
        let g = compute_gltf_geometry(&cuboid());
        let b = g.overall.size.axis_aligned_bounds.value.unwrap();
        assert_eq!(b.dimensions, GltfVec3::new([2.0, 3.0, 4.0]));
        close(g.overall.area_volume.surface_area.value.unwrap(), 52.0);
        close(g.overall.area_volume.volume.value.unwrap(), 24.0);
        close(g.overall.compactness.hull_fill_ratio.value.unwrap(), 1.0);
        close(g.overall.concavity.convex_hull_gap.value.unwrap(), 0.0);
        assert_eq!(g.overall.curvature.gaussian_curvature.availability, GltfAvailability::Approximate);
        assert_eq!(g.overall.topology.boundary_loops.value, Some(0));
        assert_eq!(g.overall.topology.euler_characteristic.value, Some(2));
        assert_eq!(g.overall.topology.genus.value, Some(0));
    }
    #[test]
    fn open_sheet_reports_open_surface() {
        let g = compute_gltf_geometry(&snapshot(&[[0., 0., 0.], [2., 0., 0.], [2., 3., 0.], [0., 3., 0.]], &[0, 1, 2, 0, 2, 3]));
        close(g.overall.area_volume.surface_area.value.unwrap(), 6.0);
        assert_eq!(g.overall.area_volume.volume.availability, GltfAvailability::OpenSurface);
        assert_eq!(g.overall.thickness.mean_thickness.availability, GltfAvailability::OpenSurface);
        assert_eq!(g.overall.topology.boundary_loops.value, Some(1));
    }
    #[test]
    fn non_manifold_edge_blocks_solid_metrics() {
        let g = compute_gltf_geometry(&snapshot(&[[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., -1., 0.], [0., 0., 1.]], &[0, 1, 2, 1, 0, 3, 0, 1, 4]));
        assert_eq!(g.overall.area_volume.volume.availability, GltfAvailability::NonManifold);
        assert_eq!(g.overall.topology.genus.availability, GltfAvailability::NonManifold);
        assert_eq!(g.overall.thickness.mean_thickness.availability, GltfAvailability::NonManifold);
    }
    #[test]
    fn scene_transform_and_instancing_are_preserved() {
        let mut s = snapshot(&[[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]], &[0, 1, 2]);
        s.document.nodes = vec![GltfNode { mesh: Some(0), translation: Some([2.0, 0.0, 0.0]), ..Default::default() }, GltfNode { mesh: Some(0), translation: Some([5.0, 0.0, 0.0]), ..Default::default() }];
        s.document.scenes = vec![GltfScene { nodes: vec![0, 1], ..Default::default() }];
        let g = compute_gltf_geometry(&s);
        assert_eq!(g.parts.len(), 2);
        assert_eq!(g.counts.primitive_count, 1);
        assert_eq!(g.counts.valid_part_count, 2);
        assert_eq!(g.counts.node_instance_count, 2);
        close(g.overall.symmetry.repetition_ratio.value.unwrap(), 0.5);
        close(g.overall.symmetry.modularity_ratio.value.unwrap(), 1.0);
        close(g.pairs[0].minimum_distance.value.unwrap(), 2.0);
        assert_eq!(g.pairs[0].overlap_volume.availability, GltfAvailability::Unavailable);
        assert_eq!(g.overall.clearance.overlap_volume.availability, GltfAvailability::Unavailable);
        let b = g.overall.size.axis_aligned_bounds.value.unwrap();
        assert_eq!(b.min.x, 2.0);
        assert_eq!(b.max.x, 6.0);
        close(g.overall.area_volume.surface_area.value.unwrap(), 1.0);
    }
    #[test]
    fn rigid_transform_preserves_intrinsic_metrics() {
        let base = compute_gltf_geometry(&cuboid());
        let mut moved = cuboid();
        moved.document.nodes = vec![GltfNode { mesh: Some(0), translation: Some([7.0, -2.0, 9.0]), rotation: Some([0.0, 0.0, (0.5f64).sqrt(), (0.5f64).sqrt()]), ..Default::default() }];
        moved.document.scenes = vec![GltfScene { nodes: vec![0], ..Default::default() }];
        let transformed = compute_gltf_geometry(&moved);
        close(base.overall.area_volume.surface_area.value.unwrap(), transformed.overall.area_volume.surface_area.value.unwrap());
        close(base.overall.area_volume.volume.value.unwrap(), transformed.overall.area_volume.volume.value.unwrap());
    }
    #[test]
    fn convex_and_concave_solids_separate_hull_metrics() {
        let convex = compute_gltf_geometry(&snapshot(
            &[[0., 0., 0.], [2., 0., 0.], [2., 2., 0.], [0., 2., 0.], [0., 0., 2.], [2., 0., 2.], [2., 2., 2.], [0., 2., 2.]],
            &[0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5],
        ));
        let concave = compute_gltf_geometry(&snapshot(
            &[[0., 0., 0.], [2., 0., 0.], [2., 2., 0.], [0., 2., 0.], [0., 0., 2.], [2., 0., 2.], [2., 2., 2.], [0., 2., 2.], [1., 1., 1.]],
            &[0, 2, 1, 0, 3, 2, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5, 4, 5, 8, 5, 6, 8, 6, 7, 8, 7, 4, 8],
        ));
        close(convex.overall.compactness.hull_fill_ratio.value.unwrap(), 1.0);
        assert!(concave.overall.compactness.hull_fill_ratio.value.unwrap() < 1.0);
        assert!(concave.overall.concavity.convex_hull_gap.value.unwrap() > 0.0);
        assert!(concave.overall.concavity.reentrant_area.value.unwrap() > 0.0);
        assert!(concave.overall.concavity.concavity_index.value.unwrap() > 0.0);
    }
    #[test]
    fn gaussian_curvature_and_smoothing_distinguish_plane_and_closed_mesh() {
        let plane_points = [[0., 0., 0.], [1., 0., 0.], [2., 0., 0.], [0., 1., 0.], [1., 1., 0.], [2., 1., 0.], [0., 2., 0.], [1., 2., 0.], [2., 2., 0.]];
        let indices = [0, 1, 4, 0, 4, 3, 1, 2, 5, 1, 5, 4, 3, 4, 7, 3, 7, 6, 4, 5, 8, 4, 8, 7];
        let plane = compute_gltf_geometry(&snapshot(&plane_points, &indices));
        assert!(plane.overall.curvature.gaussian_curvature.value.as_ref().unwrap().minimum.unwrap().abs() < 1e-9);
        let mut corrugated_points = plane_points;
        corrugated_points[4][2] = 1.0;
        let corrugated = compute_gltf_geometry(&snapshot(&corrugated_points, &indices));
        assert!(corrugated.overall.roughness.deviation_from_smoothed_geometry.value.as_ref().unwrap().mean.unwrap() > plane.overall.roughness.deviation_from_smoothed_geometry.value.as_ref().unwrap().mean.unwrap());
        assert!(compute_gltf_geometry(&cuboid()).overall.curvature.gaussian_curvature.value.as_ref().unwrap().mean.unwrap() > 0.0);
    }
    #[test]
    fn parallel_shell_thickness_is_analytic() {
        let points = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0], [2.0, 0.0, 2.0], [0.0, 2.0, 2.0]];
        let samples = thickness_samples(&points, &[[0, 1, 2], [3, 4, 5]], 64, 1e-9);
        assert!(!samples.is_empty());
        assert!(samples.iter().all(|value| (*value - 2.0).abs() < 1e-9));
    }
    #[test]
    fn repeated_part_modularity_uses_intrinsic_signatures() {
        let mut s = snapshot(&[[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]], &[0, 1, 2]);
        s.document.nodes = (0..3).map(|index| GltfNode { mesh: Some(0), translation: Some([index as f64 * 2.0, 0.0, 0.0]), ..Default::default() }).collect();
        s.document.scenes = vec![GltfScene { nodes: vec![0, 1, 2], ..Default::default() }];
        let g = compute_gltf_geometry(&s);
        close(g.overall.symmetry.repetition_ratio.value.unwrap(), 2.0 / 3.0);
        close(g.overall.symmetry.modularity_ratio.value.unwrap(), 1.0);
    }
    #[test]
    fn pair_distance_distinguishes_separation_and_contact() {
        let make = |translation| {
            let mut s = snapshot(&[[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]], &[0, 1, 2]);
            s.document.nodes = vec![GltfNode { mesh: Some(0), ..Default::default() }, GltfNode { mesh: Some(0), translation: Some([translation, 0.0, 0.0]), ..Default::default() }];
            s.document.scenes = vec![GltfScene { nodes: vec![0, 1], ..Default::default() }];
            compute_gltf_geometry(&s)
        };
        let separated = make(2.0);
        close(separated.pairs[0].minimum_distance.value.unwrap(), 1.0);
        assert_eq!(separated.pairs[0].adjacent.value, Some(false));
        close(separated.pairs[0].contact_area.value.unwrap(), 0.0);
        let contacting = make(1.0);
        close(contacting.pairs[0].minimum_distance.value.unwrap(), 0.0);
        assert_eq!(contacting.pairs[0].adjacent.value, Some(true));
        assert_eq!(contacting.pairs[0].contact_area.availability, GltfAvailability::Unavailable);
    }
    #[test]
    fn hull_sampling_is_deterministic_and_hard_capped() {
        let points = (0..100).map(|index| [index as f64, (index as f64).sin(), (index as f64).cos()]).collect::<Vec<_>>();
        let first = hull_sample(&points, usize::MAX);
        let second = hull_sample(&points, usize::MAX);
        assert_eq!(first, second);
        assert_eq!(first.len(), 32);
        assert!(first.contains(&points[0]));
        assert!(first.contains(&points[99]));
    }
    #[test]
    fn nested_shells_split_enclosed_material_and_void_volume() {
        let mut points = Vec::new();
        let mut indices = Vec::new();
        append_cube(&mut points, &mut indices, 0.0, 4.0);
        append_cube(&mut points, &mut indices, 1.0, 3.0);
        let g = compute_gltf_geometry(&snapshot(&points, &indices));
        close(g.overall.area_volume.enclosed_volume.value.unwrap(), 64.0);
        close(g.overall.area_volume.material_volume.value.unwrap(), 56.0);
        close(g.overall.area_volume.volume.value.unwrap(), 56.0);
        close(g.overall.area_volume.void_volume.value.unwrap(), 8.0);
        assert_eq!(g.overall.adjacency.connected_components.value, Some(2));
    }
    #[test]
    fn overlap_volume_estimate_has_analytic_box_bounds() {
        let make = |translation| {
            let mut s = cuboid();
            s.document.nodes = vec![GltfNode { mesh: Some(0), ..Default::default() }, GltfNode { mesh: Some(0), translation: Some([translation, 0.0, 0.0]), ..Default::default() }];
            s.document.scenes = vec![GltfScene { nodes: vec![0, 1], ..Default::default() }];
            compute_gltf_geometry(&s)
        };
        let overlapping = make(1.0);
        close(overlapping.pairs[0].overlap_volume.value.unwrap(), 12.0);
        close(overlapping.overall.clearance.overlap_volume.value.unwrap(), 12.0);
        assert_eq!(overlapping.pairs[0].overlap_volume.availability, GltfAvailability::Approximate);
        let separated = make(3.0);
        close(separated.pairs[0].overlap_volume.value.unwrap(), 0.0);
        close(separated.overall.clearance.interference_volume.value.unwrap(), 0.0);
    }
}
//#endregion 🧪️Tests
