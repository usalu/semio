//! 📐️ Authoritative glTF 2.0 static-pose universal geometric analysis.

use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{adjacency::*, area_volume::*, clearance::*, compactness::*, concavity::*, curvature::*, mass_distribution::*, measure::*, orientation::*, proportion::*, roughness::*, size::*, symmetry::*, thickness::*, topology::*};

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
type V3 = [f64; 3];
type M4 = [f64; 16];

#[derive(Clone)]
struct RawPart {
    address: GltfEntityAddress,
    name: Option<String>,
    points: Vec<V3>,
    triangles: Vec<[usize; 3]>,
    diagnostic_ids: Vec<String>,
}
#[derive(Clone, Copy)]
struct Topology {
    components: u64,
    boundary_loops: u64,
    chi: i64,
    genus: Option<u64>,
    manifold: bool,
    watertight: bool,
    oriented: bool,
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
fn unavailable<T>(unit: GltfUnit, availability: GltfAvailability, ids: Vec<String>, n: usize, t: Option<Topology>) -> GltfMeasure<T> {
    let validity = if availability == GltfAvailability::InvalidInput { GltfValidity::Invalid } else { GltfValidity::Indeterminate };
    GltfMeasure { value: None, unit, availability, validity, diagnostic_ids: ids, quality: quality(GltfComputationMethod::Exact, n, t), provenance: provenance(GltfCoordinateSpace::SceneWorld) }
}
fn exact<T>(v: T, u: GltfUnit, n: usize, t: Option<Topology>) -> GltfMeasure<T> {
    measure(v, u, GltfComputationMethod::Exact, n, t)
}
fn estimate<T>(v: T, u: GltfUnit, n: usize, t: Option<Topology>) -> GltfMeasure<T> {
    measure(v, u, GltfComputationMethod::DeterministicEstimate, n, t)
}

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn mul(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(a: V3) -> f64 {
    dot(a, a).sqrt()
}
fn normalize(a: V3) -> V3 {
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
fn triangle_area(a: V3, b: V3, c: V3) -> f64 {
    0.5 * norm(cross(sub(b, a), sub(c, a)))
}

fn convex_hull_metrics(points: &[V3], tolerance: f64) -> Option<(f64, f64, Vec<(V3, f64)>)> {
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

fn hull_sample(points: &[V3], budget: usize) -> Vec<V3> {
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

fn thickness_samples(points: &[V3], faces: &[[usize; 3]], budget: usize, tolerance: f64) -> Vec<f64> {
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

fn roughness_samples(points: &[V3], faces: &[[usize; 3]]) -> Vec<f64> {
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

fn statistics(values: &[f64], edges: &[f64]) -> GltfStatistics {
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

fn empty_indicators(ids: Vec<String>) -> GltfEntityIndicators {
    let s = || unavailable::<f64>(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None);
    let l = || unavailable::<f64>(GltfUnit::Metre, GltfAvailability::Unavailable, ids.clone(), 0, None);
    let a = || unavailable::<f64>(GltfUnit::SquareMetre, GltfAvailability::Unavailable, ids.clone(), 0, None);
    let v = || unavailable::<f64>(GltfUnit::CubicMetre, GltfAvailability::Unavailable, ids.clone(), 0, None);
    let c = || unavailable::<u64>(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None);
    let st = || unavailable::<GltfStatistics>(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None);
    GltfEntityIndicators {
        size: GltfSizeIndicators {
            overall_size: l(),
            axis_aligned_bounds: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.clone(), 0, None),
            oriented_bounds: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.clone(), 0, None),
            bounding_box_dimensions: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.clone(), 0, None),
            characteristic_length: l(),
            footprint_area: a(),
            projected_area: st(),
        },
        area_volume: GltfAreaVolumeIndicators { surface_area: a(), total_area: a(), exposed_area: a(), contact_area: a(), volume: v(), enclosed_volume: v(), material_volume: v(), void_volume: v() },
        compactness: GltfCompactnessIndicators { compactness: s(), surface_to_volume_ratio: unavailable(GltfUnit::InverseMetre, GltfAvailability::Unavailable, ids.clone(), 0, None), sphericity: s(), compactness_index: s(), hull_fill_ratio: s() },
        proportion: GltfProportionIndicators { aspect_ratios: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None), slenderness: s(), flatness: s(), elongation: s() },
        mass: GltfMassIndicators {
            centroid: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.clone(), 0, None),
            principal_frame: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None),
            principal_axes: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None),
            moments_of_inertia: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None),
            inertia_tensor: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None),
        },
        curvature: GltfCurvatureIndicators { mean_curvature: st(), gaussian_curvature: st(), curvature_histogram: st(), sharp_feature_proportion: s() },
        thickness: GltfThicknessIndicators { mean_thickness: l(), minimum_thickness: l(), thickness_variability: l(), thickness_distribution: st() },
        concavity: GltfConcavityIndicators { convex_hull_gap: v(), reentrant_area: a(), reentrant_volume: v(), concavity_index: s() },
        clearance: GltfClearanceIndicators { minimum_distance_to_neighbors: l(), clearance_distribution: st(), interference_volume: v(), overlap_volume: v() },
        adjacency: GltfAdjacencyIndicators { number_of_contacts: c(), contact_graph_degree: c(), connected_components: c() },
        orientation: GltfOrientationIndicators { main_axis_direction: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None), face_normal_distribution: st(), orientation_consistency: s() },
        symmetry: GltfSymmetryIndicators {
            reflection_symmetry_score: s(),
            rotational_symmetry_score: s(),
            reflection_symmetries: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None),
            rotational_symmetries: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None),
            repetition_ratio: s(),
            modularity_ratio: s(),
        },
        roughness: GltfRoughnessIndicators { deviation_from_ideal: st(), deviation_from_smoothed_geometry: st(), normal_variation: st(), surface_waviness: st(), irregularity: s() },
        topology: GltfTopologyIndicators { holes: c(), handles: c(), boundary_loops: c(), euler_characteristic: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.clone(), 0, None), genus: c() },
    }
}

fn symmetry_score(points: &[V3], centroid: V3, axis: V3, scale: f64, rotation: bool, budget: usize) -> f64 {
    if points.is_empty() || scale <= 0.0 {
        return 0.0;
    }
    let step = (points.len() * points.len() / budget.max(1)).max(1);
    let mut error = 0.0;
    let mut count = 0;
    for p in points.iter().step_by(step) {
        let d = sub(*p, centroid);
        let q = if rotation { add(centroid, sub(mul(axis, 2.0 * dot(d, axis)), d)) } else { sub(*p, mul(axis, 2.0 * dot(d, axis))) };
        let nearest = points.iter().map(|x| norm(sub(*x, q))).fold(f64::INFINITY, f64::min);
        error += nearest / scale;
        count += 1
    }
    (1.0 - error / count.max(1) as f64).clamp(0.0, 1.0)
}

fn analyze(points: &[V3], triangles: &[[usize; 3]], policy: &GltfAnalysisPolicy) -> (GltfEntityIndicators, Topology) {
    let (t, welded, faces, edge_faces) = topology(points, triangles);
    let n = welded.len();
    let Some((lo, hi, dims)) = bounds(&welded) else { return (empty_indicators(Vec::new()), t) };
    let bbox = GltfBounds3 { min: GltfVec3::new(lo), max: GltfVec3::new(hi), dimensions: GltfVec3::new(dims) };
    let diagonal = norm(dims);
    let surface_area = faces.iter().map(|f| triangle_area(welded[f[0]], welded[f[1]], welded[f[2]])).sum::<f64>();
    let mut surface_centroid = [0.0; 3];
    for f in &faces {
        let (a, b, c) = (welded[f[0]], welded[f[1]], welded[f[2]]);
        let ar = triangle_area(a, b, c);
        surface_centroid = add(surface_centroid, mul(add(add(a, b), c), ar / 3.0));
    }
    if surface_area > 0.0 {
        surface_centroid = mul(surface_centroid, 1.0 / surface_area)
    } else {
        surface_centroid = mul(welded.iter().fold([0.0; 3], |s, p| add(s, *p)), 1.0 / n.max(1) as f64)
    }
    let solid = if t.watertight && t.manifold && t.oriented { shell_material_metrics(&welded, &faces, &edge_faces, (diagonal * policy.relative_tolerance).max(policy.absolute_length_tolerance), policy.sampling_budget as usize) } else { None };
    let volume = solid.map(|metrics| metrics.0).unwrap_or(0.0);
    let centroid = solid.map(|metrics| metrics.3).unwrap_or(surface_centroid);
    let frame = principal_frame(&welded, centroid);
    let axes = frame.axes.iter().enumerate().map(|(i, a)| GltfDirectionScore { direction: *a, score: frame.eigenvalues[i], order: Some((i + 1) as u32) }).collect::<Vec<_>>();
    let mut oriented_extent = [0.0; 3];
    let mut oriented_min = [0.0; 3];
    let mut oriented_max = [0.0; 3];
    for (i, a) in frame.axes.iter().enumerate() {
        let axis = a.array();
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for p in &welded {
            let q = dot(sub(*p, centroid), axis);
            min = min.min(q);
            max = max.max(q)
        }
        oriented_min[i] = min;
        oriented_max[i] = max;
        oriented_extent[i] = max - min
    }
    let obb = GltfBounds3 { min: GltfVec3::new(oriented_min), max: GltfVec3::new(oriented_max), dimensions: GltfVec3::new(oriented_extent) };
    let projected = [
        faces.iter().map(|f| 0.5 * cross(sub(welded[f[1]], welded[f[0]]), sub(welded[f[2]], welded[f[0]]))[0].abs()).sum(),
        faces.iter().map(|f| 0.5 * cross(sub(welded[f[1]], welded[f[0]]), sub(welded[f[2]], welded[f[0]]))[1].abs()).sum(),
        faces.iter().map(|f| 0.5 * cross(sub(welded[f[1]], welded[f[0]]), sub(welded[f[2]], welded[f[0]]))[2].abs()).sum(),
    ];
    let mut angles = Vec::new();
    let mut edge_curvatures = Vec::new();
    let mut sharp_length = 0.0;
    let mut edge_length = 0.0;
    for (&(a, b), fs) in &edge_faces {
        let len = norm(sub(welded[b], welded[a]));
        edge_length += len;
        if fs.len() == 2 {
            let normal = |fi: usize| {
                let f = faces[fi];
                normalize(cross(sub(welded[f[1]], welded[f[0]]), sub(welded[f[2]], welded[f[0]])))
            };
            let angle = dot(normal(fs[0].0), normal(fs[1].0)).clamp(-1.0, 1.0).acos();
            angles.push(angle);
            if len > 0.0 {
                edge_curvatures.push(angle / len)
            }
            if angle > policy.sharp_feature_angle_radians {
                sharp_length += len
            }
        }
    }
    let curvature = statistics(&edge_curvatures, &policy.histogram_edges);
    let face_angles = statistics(
        &faces
            .iter()
            .map(|f| {
                let normal = normalize(cross(sub(welded[f[1]], welded[f[0]]), sub(welded[f[2]], welded[f[0]])));
                dot(normal, frame.axes[0].array()).clamp(-1.0, 1.0).acos()
            })
            .collect::<Vec<_>>(),
        &policy.histogram_edges,
    );
    let mut vertex_areas = vec![0.0; n];
    let mut angle_sums = vec![0.0; n];
    for f in &faces {
        let ar = triangle_area(welded[f[0]], welded[f[1]], welded[f[2]]);
        for corner in 0..3 {
            let i = f[corner];
            let a = sub(welded[f[(corner + 1) % 3]], welded[i]);
            let b = sub(welded[f[(corner + 2) % 3]], welded[i]);
            angle_sums[i] += dot(normalize(a), normalize(b)).clamp(-1.0, 1.0).acos();
            vertex_areas[i] += ar / 3.0;
        }
    }
    let boundary_vertices = edge_faces.iter().filter(|(_, f)| f.len() == 1).flat_map(|((a, b), _)| [*a, *b]).collect::<BTreeSet<_>>();
    let gaussian_values =
        (0..n).filter_map(|i| if vertex_areas[i] > 0.0 { Some(((if boundary_vertices.contains(&i) { std::f64::consts::PI } else { 2.0 * std::f64::consts::PI }) - angle_sums[i]) / vertex_areas[i]) } else { None }).collect::<Vec<_>>();
    let gaussian = statistics(&gaussian_values, &policy.histogram_edges);
    let unavailable_volume = if !t.manifold {
        GltfAvailability::NonManifold
    } else if !t.watertight {
        GltfAvailability::OpenSurface
    } else if !t.oriented {
        GltfAvailability::InvalidInput
    } else {
        GltfAvailability::InvalidInput
    };
    let vol = if solid.is_some() { exact(volume, GltfUnit::CubicMetre, n, Some(t)) } else { unavailable(GltfUnit::CubicMetre, unavailable_volume, Vec::new(), n, Some(t)) };
    let enclosed = if let Some(metrics) = solid { exact(metrics.1, GltfUnit::CubicMetre, n, Some(t)) } else { unavailable(GltfUnit::CubicMetre, unavailable_volume, Vec::new(), n, Some(t)) };
    let void = if let Some(metrics) = solid { exact(metrics.2, GltfUnit::CubicMetre, n, Some(t)) } else { unavailable(GltfUnit::CubicMetre, unavailable_volume, Vec::new(), n, Some(t)) };
    let ratio = if volume > 1e-15 && t.watertight && t.manifold && t.oriented { Some(surface_area / volume) } else { None };
    let sphericity = if volume > 1e-15 && surface_area > 0.0 && t.watertight { Some(std::f64::consts::PI.powf(1.0 / 3.0) * (6.0 * volume).powf(2.0 / 3.0) / surface_area) } else { None };
    let sorted = {
        let mut x = oriented_extent;
        x.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        x
    };
    let thickness_values = if t.watertight && t.manifold { thickness_samples(&welded, &faces, policy.sampling_budget as usize, policy.absolute_length_tolerance) } else { Vec::new() };
    let thickness_stats = statistics(&thickness_values, &policy.histogram_edges);
    let thick_mean = thickness_stats.mean;
    let thick_min = thickness_stats.minimum;
    let thick_variability = thickness_stats.standard_deviation;
    let rough_values = roughness_samples(&welded, &faces);
    let rough_stats = statistics(&rough_values, &policy.histogram_edges);
    let rough_irregularity = match (rough_stats.mean, rough_stats.standard_deviation) {
        (Some(m), Some(s)) if m > 0.0 => Some(s / m),
        _ => None,
    };
    let hull_input = hull_sample(&welded, policy.sampling_budget as usize);
    let hull_tolerance = (diagonal * policy.relative_tolerance).max(policy.absolute_length_tolerance);
    let hull = convex_hull_metrics(&hull_input, hull_tolerance);
    let reentrant_area = hull.as_ref().map(|(_, _, planes)| {
        faces
            .iter()
            .filter(|face| {
                let centroid = mul(add(add(welded[face[0]], welded[face[1]]), welded[face[2]]), 1.0 / 3.0);
                !planes.iter().any(|(normal, offset)| (dot(*normal, centroid) - *offset).abs() <= hull_tolerance * 4.0)
            })
            .map(|face| triangle_area(welded[face[0]], welded[face[1]], welded[face[2]]))
            .sum::<f64>()
    });
    let moments = GltfVec3::new([frame.eigenvalues[1] + frame.eigenvalues[2], frame.eigenvalues[0] + frame.eigenvalues[2], frame.eigenvalues[0] + frame.eigenvalues[1]]);
    let tensor = vec![moments.x, 0.0, 0.0, 0.0, moments.y, 0.0, 0.0, 0.0, moments.z];
    let unavailable_stats = || unavailable::<GltfStatistics>(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), n, Some(t));
    (
        GltfEntityIndicators {
            size: GltfSizeIndicators {
                overall_size: exact(diagonal, GltfUnit::Metre, n, Some(t)),
                axis_aligned_bounds: exact(bbox.clone(), GltfUnit::Metre, n, Some(t)),
                oriented_bounds: exact(obb, GltfUnit::Metre, n, Some(t)),
                bounding_box_dimensions: exact(GltfVec3::new(dims), GltfUnit::Metre, n, Some(t)),
                characteristic_length: exact(if surface_area > 0.0 { surface_area.sqrt() } else { diagonal }, GltfUnit::Metre, n, Some(t)),
                footprint_area: estimate(projected[2], GltfUnit::SquareMetre, n, Some(t)),
                projected_area: estimate(statistics(&projected, &policy.histogram_edges), GltfUnit::SquareMetre, n, Some(t)),
            },
            area_volume: GltfAreaVolumeIndicators {
                surface_area: exact(surface_area, GltfUnit::SquareMetre, n, Some(t)),
                total_area: exact(surface_area, GltfUnit::SquareMetre, n, Some(t)),
                exposed_area: unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
                contact_area: unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
                volume: vol.clone(),
                enclosed_volume: enclosed,
                material_volume: vol.clone(),
                void_volume: void,
            },
            compactness: GltfCompactnessIndicators {
                compactness: if let Some(s) = sphericity { exact(s, GltfUnit::Unitless, n, Some(t)) } else { unavailable(GltfUnit::Unitless, unavailable_volume, Vec::new(), n, Some(t)) },
                surface_to_volume_ratio: if let Some(r) = ratio { exact(r, GltfUnit::InverseMetre, n, Some(t)) } else { unavailable(GltfUnit::InverseMetre, unavailable_volume, Vec::new(), n, Some(t)) },
                sphericity: if let Some(s) = sphericity { exact(s, GltfUnit::Unitless, n, Some(t)) } else { unavailable(GltfUnit::Unitless, unavailable_volume, Vec::new(), n, Some(t)) },
                compactness_index: if let Some(s) = sphericity { exact(s, GltfUnit::Unitless, n, Some(t)) } else { unavailable(GltfUnit::Unitless, unavailable_volume, Vec::new(), n, Some(t)) },
                hull_fill_ratio: if let Some((_, hv, _)) = hull.as_ref().filter(|(_, v, _)| *v > 0.0) {
                    if vol.value.is_some() {
                        estimate((volume / *hv).clamp(0.0, 1.0), GltfUnit::Unitless, n, Some(t))
                    } else {
                        unavailable(GltfUnit::Unitless, unavailable_volume, Vec::new(), n, Some(t))
                    }
                } else {
                    unavailable(GltfUnit::Unitless, GltfAvailability::Degenerate, Vec::new(), n, Some(t))
                },
            },
            proportion: GltfProportionIndicators {
                aspect_ratios: exact(
                    GltfVec3::new([if sorted[1] > 0.0 { sorted[0] / sorted[1] } else { 0.0 }, if sorted[2] > 0.0 { sorted[1] / sorted[2] } else { 0.0 }, if sorted[2] > 0.0 { sorted[0] / sorted[2] } else { 0.0 }]),
                    GltfUnit::Unitless,
                    n,
                    Some(t),
                ),
                slenderness: exact(if sorted[1] > 0.0 { sorted[0] / sorted[1] } else { 0.0 }, GltfUnit::Unitless, n, Some(t)),
                flatness: exact(if sorted[1] > 0.0 { sorted[2] / sorted[1] } else { 0.0 }, GltfUnit::Unitless, n, Some(t)),
                elongation: exact(if sorted[0] > 0.0 { sorted[1] / sorted[0] } else { 0.0 }, GltfUnit::Unitless, n, Some(t)),
            },
            mass: GltfMassIndicators {
                centroid: if t.watertight && volume > 1e-15 { exact(GltfVec3::new(centroid), GltfUnit::Metre, n, Some(t)) } else { estimate(GltfVec3::new(centroid), GltfUnit::Metre, n, Some(t)) },
                principal_frame: estimate(frame.clone(), GltfUnit::Unitless, n, Some(t)),
                principal_axes: estimate(axes.clone(), GltfUnit::Unitless, n, Some(t)),
                moments_of_inertia: estimate(moments, GltfUnit::SquareMetre, n, Some(t)),
                inertia_tensor: estimate(tensor, GltfUnit::SquareMetre, n, Some(t)),
            },
            curvature: GltfCurvatureIndicators {
                mean_curvature: estimate(curvature.clone(), GltfUnit::InverseMetre, edge_curvatures.len(), Some(t)),
                gaussian_curvature: estimate(gaussian, GltfUnit::InverseSquareMetre, gaussian_values.len(), Some(t)),
                curvature_histogram: estimate(curvature.clone(), GltfUnit::InverseMetre, edge_curvatures.len(), Some(t)),
                sharp_feature_proportion: exact(if edge_length > 0.0 { sharp_length / edge_length } else { 0.0 }, GltfUnit::Unitless, n, Some(t)),
            },
            thickness: GltfThicknessIndicators {
                mean_thickness: if let Some(x) = thick_mean { estimate(x, GltfUnit::Metre, n, Some(t)) } else { unavailable(GltfUnit::Metre, unavailable_volume, Vec::new(), n, Some(t)) },
                minimum_thickness: if let Some(x) = thick_min { estimate(x, GltfUnit::Metre, n, Some(t)) } else { unavailable(GltfUnit::Metre, unavailable_volume, Vec::new(), n, Some(t)) },
                thickness_variability: if let Some(x) = thick_variability { estimate(x, GltfUnit::Metre, n, Some(t)) } else { unavailable(GltfUnit::Metre, unavailable_volume, Vec::new(), n, Some(t)) },
                thickness_distribution: if thickness_values.is_empty() { unavailable(GltfUnit::Metre, unavailable_volume, Vec::new(), n, Some(t)) } else { estimate(thickness_stats, GltfUnit::Metre, n, Some(t)) },
            },
            concavity: GltfConcavityIndicators {
                convex_hull_gap: if let Some((_, hv, _)) = hull.as_ref() {
                    if vol.value.is_some() {
                        estimate((*hv - volume).max(0.0), GltfUnit::CubicMetre, n, Some(t))
                    } else {
                        unavailable(GltfUnit::CubicMetre, unavailable_volume, Vec::new(), n, Some(t))
                    }
                } else {
                    unavailable(GltfUnit::CubicMetre, GltfAvailability::Degenerate, Vec::new(), n, Some(t))
                },
                reentrant_area: if let Some(area) = reentrant_area { estimate(area, GltfUnit::SquareMetre, n, Some(t)) } else { unavailable(GltfUnit::SquareMetre, GltfAvailability::Degenerate, Vec::new(), n, Some(t)) },
                reentrant_volume: if let Some((_, hv, _)) = hull.as_ref() {
                    if vol.value.is_some() {
                        estimate((*hv - volume).max(0.0), GltfUnit::CubicMetre, n, Some(t))
                    } else {
                        unavailable(GltfUnit::CubicMetre, unavailable_volume, Vec::new(), n, Some(t))
                    }
                } else {
                    unavailable(GltfUnit::CubicMetre, GltfAvailability::Degenerate, Vec::new(), n, Some(t))
                },
                concavity_index: if let Some((_, hv, _)) = hull.as_ref().filter(|(_, v, _)| *v > 0.0) {
                    if vol.value.is_some() {
                        estimate((1.0 - volume / *hv).clamp(0.0, 1.0), GltfUnit::Unitless, n, Some(t))
                    } else {
                        unavailable(GltfUnit::Unitless, unavailable_volume, Vec::new(), n, Some(t))
                    }
                } else {
                    unavailable(GltfUnit::Unitless, GltfAvailability::Degenerate, Vec::new(), n, Some(t))
                },
            },
            clearance: GltfClearanceIndicators {
                minimum_distance_to_neighbors: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
                clearance_distribution: unavailable_stats(),
                interference_volume: unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
                overlap_volume: unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
            },
            adjacency: GltfAdjacencyIndicators {
                number_of_contacts: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
                contact_graph_degree: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
                connected_components: exact(t.components, GltfUnit::Unitless, n, Some(t)),
            },
            orientation: GltfOrientationIndicators {
                main_axis_direction: estimate(axes[0].direction, GltfUnit::Unitless, n, Some(t)),
                face_normal_distribution: exact(face_angles, GltfUnit::Radian, faces.len(), Some(t)),
                orientation_consistency: exact(if t.oriented { 1.0 } else { 0.0 }, GltfUnit::Unitless, n, Some(t)),
            },
            symmetry: GltfSymmetryIndicators {
                reflection_symmetry_score: estimate(symmetry_score(&welded, centroid, frame.axes[0].array(), diagonal, false, policy.sampling_budget as usize), GltfUnit::Unitless, n, Some(t)),
                rotational_symmetry_score: estimate(symmetry_score(&welded, centroid, frame.axes[0].array(), diagonal, true, policy.sampling_budget as usize), GltfUnit::Unitless, n, Some(t)),
                reflection_symmetries: estimate(
                    frame.axes.iter().map(|a| GltfDirectionScore { direction: *a, score: symmetry_score(&welded, centroid, a.array(), diagonal, false, policy.sampling_budget as usize), order: None }).collect(),
                    GltfUnit::Unitless,
                    n,
                    Some(t),
                ),
                rotational_symmetries: estimate(
                    frame.axes.iter().map(|a| GltfDirectionScore { direction: *a, score: symmetry_score(&welded, centroid, a.array(), diagonal, true, policy.sampling_budget as usize), order: Some(2) }).collect(),
                    GltfUnit::Unitless,
                    n,
                    Some(t),
                ),
                repetition_ratio: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
                modularity_ratio: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), n, Some(t)),
            },
            roughness: GltfRoughnessIndicators {
                deviation_from_ideal: unavailable_stats(),
                deviation_from_smoothed_geometry: estimate(rough_stats.clone(), GltfUnit::Metre, rough_values.len(), Some(t)),
                normal_variation: exact(statistics(&angles, &policy.histogram_edges), GltfUnit::Radian, n, Some(t)),
                surface_waviness: estimate(rough_stats, GltfUnit::Metre, rough_values.len(), Some(t)),
                irregularity: if let Some(x) = rough_irregularity { estimate(x, GltfUnit::Unitless, rough_values.len(), Some(t)) } else { unavailable(GltfUnit::Unitless, GltfAvailability::Degenerate, Vec::new(), n, Some(t)) },
            },
            topology: GltfTopologyIndicators {
                holes: if let Some(g) = t.genus { exact(g, GltfUnit::Unitless, n, Some(t)) } else { unavailable(GltfUnit::Unitless, GltfAvailability::NonManifold, Vec::new(), n, Some(t)) },
                handles: if let Some(g) = t.genus { exact(g, GltfUnit::Unitless, n, Some(t)) } else { unavailable(GltfUnit::Unitless, GltfAvailability::NonManifold, Vec::new(), n, Some(t)) },
                boundary_loops: exact(t.boundary_loops, GltfUnit::Unitless, n, Some(t)),
                euler_characteristic: exact(t.chi, GltfUnit::Unitless, n, Some(t)),
                genus: if let Some(g) = t.genus { exact(g, GltfUnit::Unitless, n, Some(t)) } else { unavailable(GltfUnit::Unitless, GltfAvailability::NonManifold, Vec::new(), n, Some(t)) },
            },
        },
        t,
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

fn analyze_pair(a: &RawPart, b: &RawPart, p: &GltfAnalysisPolicy) -> Option<GltfPairInference> {
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
    Some(GltfPairInference {
        first: a.address.clone(),
        second: b.address.clone(),
        minimum_distance: estimate(distance, GltfUnit::Metre, n, None),
        clearance_distribution: estimate(statistics(&[distance], &p.histogram_edges), GltfUnit::Metre, n, None),
        contact_area: if coincident_contact {
            estimate(contact_area, GltfUnit::SquareMetre, n, None)
        } else if aabb_separation > p.contact_tolerance {
            exact(0.0, GltfUnit::SquareMetre, n, None)
        } else {
            unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), n, None)
        },
        interference_volume: if let Some((volume, samples)) = overlap { estimate(volume, GltfUnit::CubicMetre, samples, None) } else { unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), n, None) },
        overlap_volume: if let Some((volume, samples)) = overlap { estimate(volume, GltfUnit::CubicMetre, samples, None) } else { unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), n, None) },
        adjacent: estimate(adjacent, GltfUnit::Unitless, n, None),
        orientation_consistency: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), n, None),
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
    let signature = |part: &GltfPartInference| {
        let dimensions = part.indicators.size.oriented_bounds.value.as_ref().map(|x| x.dimensions.array()).unwrap_or([0.0; 3]);
        let mut dimensions = dimensions;
        dimensions.sort_by(f64::total_cmp);
        let quantum = p.absolute_length_tolerance.max(1e-9);
        let area_quantum = quantum * quantum;
        let volume_quantum = area_quantum * quantum;
        format!(
            "{},{},{},{},{}",
            (dimensions[0] / quantum).round() as i64,
            (dimensions[1] / quantum).round() as i64,
            (dimensions[2] / quantum).round() as i64,
            (part.indicators.area_volume.surface_area.value.unwrap_or(0.0) / area_quantum).round() as i64,
            (part.indicators.area_volume.volume.value.unwrap_or(0.0) / volume_quantum).round() as i64
        )
    };
    let mut signatures = BTreeMap::<String, usize>::new();
    for part in &parts {
        *signatures.entry(signature(part)).or_default() += 1
    }
    if !parts.is_empty() {
        let repeated_members = signatures.values().filter(|count| **count > 1).sum::<usize>();
        let repeated_excess = signatures.values().map(|count| count.saturating_sub(1)).sum::<usize>();
        overall.symmetry.repetition_ratio = estimate(repeated_excess as f64 / parts.len() as f64, GltfUnit::Unitless, parts.len(), Some(overall_topology));
        overall.symmetry.modularity_ratio = estimate(repeated_members as f64 / parts.len() as f64, GltfUnit::Unitless, parts.len(), Some(overall_topology));
    }
    let mut pairs = Vec::new();
    for i in 0..raw_parts.len() {
        for j in i + 1..raw_parts.len() {
            if let Some(pair) = analyze_pair(&raw_parts[i], &raw_parts[j], &p) {
                pairs.push(pair)
            }
        }
    }
    let distances = pairs.iter().filter_map(|x| x.minimum_distance.value).collect::<Vec<_>>();
    let contact_area = pairs.iter().filter_map(|x| x.contact_area.value).sum::<f64>();
    let contact_area_complete = pairs.iter().all(|pair| pair.contact_area.value.is_some());
    let overlap_volume = pairs.iter().filter_map(|pair| pair.overlap_volume.value).sum::<f64>();
    let overlap_complete = pairs.iter().all(|pair| pair.overlap_volume.value.is_some());
    let contacts = pairs.iter().filter(|x| x.adjacent.value == Some(true)).count() as u64;
    let sample = all_points.len();
    if raw_parts.len() <= 1 {
        overall.area_volume.exposed_area = overall.area_volume.surface_area.clone();
        overall.area_volume.contact_area = exact(0.0, GltfUnit::SquareMetre, sample, Some(overall_topology));
        overall.adjacency.number_of_contacts = exact(0, GltfUnit::Unitless, sample, Some(overall_topology));
        overall.adjacency.contact_graph_degree = exact(0, GltfUnit::Unitless, sample, Some(overall_topology));
    } else {
        if contact_area_complete {
            overall.area_volume.contact_area = estimate(contact_area, GltfUnit::SquareMetre, sample, Some(overall_topology));
            overall.area_volume.exposed_area = estimate((overall.area_volume.surface_area.value.unwrap_or(0.0) - 2.0 * contact_area).max(0.0), GltfUnit::SquareMetre, sample, Some(overall_topology));
        } else {
            overall.area_volume.contact_area = unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), sample, Some(overall_topology));
            overall.area_volume.exposed_area = unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), sample, Some(overall_topology));
        }
        overall.adjacency.number_of_contacts = estimate(contacts, GltfUnit::Unitless, sample, Some(overall_topology));
        overall.adjacency.contact_graph_degree = estimate(if raw_parts.is_empty() { 0 } else { 2 * contacts / raw_parts.len() as u64 }, GltfUnit::Unitless, sample, Some(overall_topology));
        overall.orientation.orientation_consistency = unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), sample, Some(overall_topology));
    }
    if !distances.is_empty() {
        overall.clearance.minimum_distance_to_neighbors = estimate(distances.iter().copied().fold(f64::INFINITY, f64::min), GltfUnit::Metre, sample, Some(overall_topology));
        overall.clearance.clearance_distribution = estimate(statistics(&distances, &p.histogram_edges), GltfUnit::Metre, sample, Some(overall_topology));
    }
    if !pairs.is_empty() && overlap_complete {
        overall.clearance.interference_volume = estimate(overlap_volume, GltfUnit::CubicMetre, sample, Some(overall_topology));
        overall.clearance.overlap_volume = estimate(overlap_volume, GltfUnit::CubicMetre, sample, Some(overall_topology));
    }
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
