//! 🧮️ Vector and matrix math, node transforms, fingerprints, triangle decoding, part collection and bounds.

use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfComponentType;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfDecodedSnapshot;
use std::collections::BTreeSet;
use crate::standards::v2_0::subsets::any::schema::modules::mesh_topology::Topology;
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::*;

//#region 🧮️Kernel
pub type V3 = [f64; 3];
type M4 = [f64; 16];

#[derive(Clone)]
pub struct RawPart {
    pub address: GltfEntityAddress,
    pub name: Option<String>,
    pub points: Vec<V3>,
    pub triangles: Vec<[usize; 3]>,
    pub diagnostic_ids: Vec<String>,
}
pub fn policy() -> GltfAnalysisPolicy {
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
pub fn provenance(space: GltfCoordinateSpace) -> GltfProvenance {
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
pub fn quality(method: GltfComputationMethod, n: usize, topology: Option<Topology>) -> GltfQuality {
    let t = topology.unwrap_or(Topology { components: 0, boundary_loops: 0, chi: 0, genus: None, manifold: true, watertight: false, oriented: true });
    GltfQuality { method, coverage: if n == 0 { 0.0 } else { 1.0 }, absolute_error: None, relative_error: None, sample_count: n as u64, watertight: t.watertight, manifold: t.manifold, consistently_oriented: t.oriented, warnings: Vec::new() }
}

pub fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub fn mul(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}
pub fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
pub fn norm(a: V3) -> f64 {
    dot(a, a).sqrt()
}
pub fn normalize(a: V3) -> V3 {
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
fn node_matrix(node: &semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfNode) -> M4 {
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
pub fn fingerprint(points: &[V3], triangles: &[[usize; 3]]) -> String {
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
pub fn byte_fingerprint(bytes: &[u8]) -> String {
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
            for c in indices.as_chunks::<3>().0 {
                out.push([c[0], c[1], c[2]]);
            }
        }
        5 => {
            for i in 2..indices.len() {
                if i % 2 == 0 {
                    out.push([indices[i - 2], indices[i - 1], indices[i]]);
                } else {
                    out.push([indices[i - 1], indices[i - 2], indices[i]]);
                }
            }
        }
        6 => {
            for i in 2..indices.len() {
                out.push([indices[0], indices[i - 1], indices[i]]);
            }
        }
        _ => return Err(GltfAvailability::UnsupportedPrimitive),
    }
    Ok(out)
}

fn decode_part(snapshot: &GltfDecodedSnapshot<'_>, (mesh_index, primitive_index): (usize, usize), matrix: M4, scene: Option<usize>, path: &[usize], weights: &[f64], diagnostics: &mut Vec<GltfDiagnostic>) -> Option<RawPart> {
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
    let decoded = match snapshot.accessor(position_accessor) {
        Ok(x) if x.components.len() % 3 == 0 => x,
        Ok(_) => return None,
        Err(message) => {
            let id = format!("gltf-geometry-{}", diagnostics.len());
            diagnostics.push(GltfDiagnostic { id, severity: GltfSeverity::Error, code: "unresolved-position-accessor".into(), message, paths: vec![format!("meshes/{mesh_index}/primitives/{primitive_index}/attributes/POSITION")] });
            return None;
        }
    };
    let mut local: Vec<V3> = decoded.components.as_chunks::<3>().0.iter().map(|v| [v[0], v[1], v[2]]).collect();
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
        match snapshot.accessor(accessor) {
            Ok(delta) if delta.components.len() == local.len() * 3 => {
                for (p, d) in local.iter_mut().zip(delta.components.as_chunks::<3>().0) {
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
        match snapshot.accessor(accessor) {
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

/// 🌳️ Accumulated primitive instances, diagnostics, and instance count during traversal.
type TraversalOutput<'a> = (&'a mut Vec<RawPart>, &'a mut Vec<GltfDiagnostic>, &'a mut u64);

pub fn collect_parts(snapshot: &GltfDecodedSnapshot<'_>, diagnostics: &mut Vec<GltfDiagnostic>) -> (Vec<RawPart>, u64) {
    fn visit(snapshot: &GltfDecodedSnapshot<'_>, scene: usize, node_index: usize, parent: M4, path: &mut Vec<usize>, stack: &mut BTreeSet<usize>, (parts, diagnostics, instances): TraversalOutput<'_>) {
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
                    if let Some(p) = decode_part(snapshot, (mesh_index, primitive_index), world, Some(scene), path, weights, diagnostics) {
                        parts.push(p);
                    }
                }
            }
        }
        for child in &node.children {
            visit(snapshot, scene, *child, world, path, stack, (parts, diagnostics, instances));
        }
        path.pop();
        stack.remove(&node_index);
    }
    let mut parts = Vec::new();
    let mut instances = 0;
    if snapshot.document.scenes.is_empty() {
        for (mi, mesh) in snapshot.document.meshes.iter().enumerate() {
            for pi in 0..mesh.primitives.len() {
                if let Some(p) = decode_part(snapshot, (mi, pi), identity(), None, &[], &mesh.weights, diagnostics) {
                    parts.push(p);
                }
            }
        }
    } else {
        for (si, scene) in snapshot.document.scenes.iter().enumerate() {
            for root in &scene.nodes {
                visit(snapshot, si, *root, identity(), &mut Vec::new(), &mut BTreeSet::new(), (&mut parts, diagnostics, &mut instances));
            }
        }
    }
    (parts, instances)
}

pub(crate) fn bounds(points: &[V3]) -> Option<(V3, V3, V3)> {
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
//#endregion 🧮️Kernel
