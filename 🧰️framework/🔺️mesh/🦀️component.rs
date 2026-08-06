// #region mesh
//! 🔺️ Shared mesh geometry: primitives, compact JSON, OBJ/GLB interchange.

use serde::{Deserialize, Serialize};
use dsl::DslValue;

//#region MeshData
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshData {
    #[serde(default)]
    pub positions: Vec<f32>,
    #[serde(default)]
    pub normals: Vec<f32>,
    #[serde(default)]
    pub colors: Vec<f32>,
    #[serde(default)]
    pub indices: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uvs: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub face_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vertex_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_positions: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_uvs: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_is_seam: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint_texture_base64: Option<String>,
}

impl MeshData {
    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn compute_normals(&mut self) {
        let count = self.vertex_count();
        self.normals = vec![0.0; count * 3];
        for tri in self.indices.chunks_exact(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p0 = [self.positions[i0 * 3], self.positions[i0 * 3 + 1], self.positions[i0 * 3 + 2]];
            let p1 = [self.positions[i1 * 3], self.positions[i1 * 3 + 1], self.positions[i1 * 3 + 2]];
            let p2 = [self.positions[i2 * 3], self.positions[i2 * 3 + 1], self.positions[i2 * 3 + 2]];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [
                e0[1] * e1[2] - e0[2] * e1[1],
                e0[2] * e1[0] - e0[0] * e1[2],
                e0[0] * e1[1] - e0[1] * e1[0],
            ];
            for &idx in tri {
                let i = idx as usize * 3;
                self.normals[i] += n[0];
                self.normals[i + 1] += n[1];
                self.normals[i + 2] += n[2];
            }
        }
        for chunk in self.normals.chunks_exact_mut(3) {
            let len = (chunk[0] * chunk[0] + chunk[1] * chunk[1] + chunk[2] * chunk[2]).sqrt();
            if len > 1e-8 {
                chunk[0] /= len;
                chunk[1] /= len;
                chunk[2] /= len;
            }
        }
    }

    pub fn aabb(&self) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for chunk in self.positions.chunks_exact(3) {
            for axis in 0..3 {
                min[axis] = min[axis].min(chunk[axis]);
                max[axis] = max[axis].max(chunk[axis]);
            }
        }
        (min, max)
    }

    pub fn merge(&mut self, other: &MeshData) {
        let base = self.vertex_count() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.colors.extend_from_slice(&other.colors);
        self.indices
            .extend(other.indices.iter().map(|index| index + base));
    }
}
//#endregion MeshData

//#region Primitives
fn push_triangle(mesh: &mut MeshData, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

pub fn mesh_box(width: f32, height: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hh = height * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    let faces = [
        ([-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]),
        ([hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]),
        ([-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]),
        ([-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]),
        ([hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]),
        ([-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]),
    ];
    for (a, b, c, d) in faces {
        push_triangle(&mut mesh, a, b, c);
        push_triangle(&mut mesh, a, c, d);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_plane(width: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

pub fn mesh_uv_sphere(radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::PI;
        let phi1 = v1 * std::f32::consts::PI;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = sphere_point(radius, phi0, theta0);
            let p10 = sphere_point(radius, phi0, theta1);
            let p01 = sphere_point(radius, phi1, theta0);
            let p11 = sphere_point(radius, phi1, theta1);
            if ring > 0 {
                push_triangle(&mut mesh, p00, p10, p11);
            }
            if ring + 1 < rings {
                push_triangle(&mut mesh, p00, p11, p01);
            }
        }
    }
    mesh.compute_normals();
    mesh
}

fn sphere_point(radius: f32, phi: f32, theta: f32) -> [f32; 3] {
    let sin_phi = phi.sin();
    [
        radius * sin_phi * theta.cos(),
        radius * phi.cos(),
        radius * sin_phi * theta.sin(),
    ]
}

pub fn mesh_ico_sphere(radius: f32, subdivisions: u32) -> MeshData {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        normalize3([-1.0, t, 0.0]),
        normalize3([1.0, t, 0.0]),
        normalize3([-1.0, -t, 0.0]),
        normalize3([1.0, -t, 0.0]),
        normalize3([0.0, -1.0, t]),
        normalize3([0.0, 1.0, t]),
        normalize3([0.0, -1.0, -t]),
        normalize3([0.0, 1.0, -t]),
        normalize3([t, 0.0, -1.0]),
        normalize3([t, 0.0, 1.0]),
        normalize3([-t, 0.0, -1.0]),
        normalize3([-t, 0.0, 1.0]),
    ];
    let mut faces = vec![
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = std::collections::HashMap::new();
        for face in &faces {
            let a = midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[
                [face[0], a, c],
                [face[1], b, a],
                [face[2], c, b],
                [a, b, c],
            ]);
        }
        faces = next;
    }
    let mut mesh = MeshData::default();
    for face in faces {
        let a = scale3(verts[face[0] as usize], radius);
        let b = scale3(verts[face[1] as usize], radius);
        let c = scale3(verts[face[2] as usize], radius);
        push_triangle(&mut mesh, a, b, c);
    }
    mesh.compute_normals();
    mesh
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn midpoint(
    verts: &mut Vec<[f32; 3]>,
    cache: &mut std::collections::HashMap<(u32, u32), u32>,
    a: u32,
    b: u32,
) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = normalize3([
        (verts[a as usize][0] + verts[b as usize][0]) * 0.5,
        (verts[a as usize][1] + verts[b as usize][1]) * 0.5,
        (verts[a as usize][2] + verts[b as usize][2]) * 0.5,
    ]);
    let index = verts.len() as u32;
    verts.push(mid);
    cache.insert(key, index);
    index
}

pub fn mesh_cylinder(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let half = height * 0.5;
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p00 = [radius * a0.cos(), -half, radius * a0.sin()];
        let p01 = [radius * a1.cos(), -half, radius * a1.sin()];
        let p10 = [radius * a0.cos(), half, radius * a0.sin()];
        let p11 = [radius * a1.cos(), half, radius * a1.sin()];
        push_triangle(&mut mesh, p00, p01, p11);
        push_triangle(&mut mesh, p00, p11, p10);
        push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00);
        push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_cone(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        push_triangle(&mut mesh, apex, p1, p0);
        push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_torus(major_radius: f32, minor_radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::TAU;
        let phi1 = v1 * std::f32::consts::TAU;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = torus_point(major_radius, minor_radius, phi0, theta0);
            let p10 = torus_point(major_radius, minor_radius, phi0, theta1);
            let p01 = torus_point(major_radius, minor_radius, phi1, theta0);
            let p11 = torus_point(major_radius, minor_radius, phi1, theta1);
            push_triangle(&mut mesh, p00, p10, p11);
            push_triangle(&mut mesh, p00, p11, p01);
        }
    }
    mesh.compute_normals();
    mesh
}

fn torus_point(major: f32, minor: f32, phi: f32, theta: f32) -> [f32; 3] {
    let r = major + minor * theta.cos();
    [r * phi.cos(), minor * theta.sin(), r * phi.sin()]
}

pub fn mesh_from_kind(kind: &str) -> MeshData {
    match kind {
        "vortex-marker" => mesh_ico_sphere(0.12, 1),
        "vertex-marker" => mesh_ico_sphere(1.0, 1),
        "sphere" | "uvSphere" => mesh_uv_sphere(0.5, 16, 12),
        "icoSphere" => mesh_ico_sphere(0.5, 1),
        "plane" => mesh_plane(1.0, 1.0),
        "cylinder" => mesh_cylinder(0.5, 1.0, 16),
        "cone" => mesh_cone(0.5, 1.0, 16),
        "torus" => mesh_torus(0.5, 0.15, 16, 12),
        _ => mesh_box(1.0, 1.0, 1.0),
    }
}

/** @emoji 🔩️ Builds mesh data from indexed brep tessellation buffers. */
pub fn mesh_from_indexed(positions: &[f32], normals: &[f32], indices: &[u32]) -> MeshData {
    let mut mesh = MeshData {
        positions: positions.to_vec(),
        normals: normals.to_vec(),
        indices: indices.to_vec(),
        ..MeshData::default()
    };
    if mesh.normals.is_empty() && !mesh.positions.is_empty() {
        mesh.compute_normals();
    }
    mesh
}

/** @emoji 🧩️ Like `mesh_from_indexed`, but also stamps `face_ids` per triangle from `(face id, triangle start, triangle count)`
 * groups — lets a picked triangle resolve back to the brep face it came from. Plain tuples (not the kernel's `FaceGroup`)
 * so this crate doesn't need to depend on the kernel engine crate; callers convert their own group type. */
pub fn mesh_from_indexed_with_face_groups(positions: &[f32], normals: &[f32], indices: &[u32], face_groups: &[(u32, u32, u32)]) -> MeshData {
    let mut mesh = mesh_from_indexed(positions, normals, indices);
    if !face_groups.is_empty() {
        let triangle_count = indices.len() / 3;
        let mut face_ids = vec![0u32; triangle_count];
        for &(face_id, start, count) in face_groups {
            let start_tri = (start / 3) as usize;
            let count_tri = (count / 3) as usize;
            for tri in start_tri..(start_tri + count_tri).min(triangle_count) {
                face_ids[tri] = face_id;
            }
        }
        mesh.face_ids = face_ids;
    }
    mesh
}
//#endregion Primitives

//#region Obj
pub fn mesh_to_obj(mesh: &MeshData, object_name: &str) -> String {
    let mut out = format!("o {object_name}\n");
    for chunk in mesh.positions.chunks_exact(3) {
        out.push_str(&format!("v {} {} {}\n", chunk[0], chunk[1], chunk[2]));
    }
    if mesh.normals.len() == mesh.positions.len() {
        for chunk in mesh.normals.chunks_exact(3) {
            out.push_str(&format!("vn {} {} {}\n", chunk[0], chunk[1], chunk[2]));
        }
    }
    for tri in mesh.indices.chunks_exact(3) {
        let a = tri[0] + 1;
        let b = tri[1] + 1;
        let c = tri[2] + 1;
        if mesh.normals.len() == mesh.positions.len() {
            out.push_str(&format!("f {a}//{a} {b}//{b} {c}//{c}\n"));
        } else {
            out.push_str(&format!("f {a} {b} {c}\n"));
        }
    }
    out
}

/// 🔤️ Hand-parses OBJ text (`v`/`vn`/`f` lines) back into `MeshData`; fan-triangulates n-gon faces and falls back to computed normals when the file has no `vn` lines or a mismatched vertex/normal count. Round-trips `mesh_to_obj`'s own output losslessly; general third-party OBJ interop is unvalidated.
pub fn mesh_from_obj(text: &str) -> Result<MeshData, String> {
    let mut positions: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut vertex_count = 0usize;
    let mut normal_count = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let tag = match parts.next() {
            Some(tag) => tag,
            None => continue,
        };
        match tag {
            "v" => {
                let coords: Vec<f32> = parts.filter_map(|value| value.parse().ok()).collect();
                if coords.len() < 3 {
                    return Err("obj: malformed v line".into());
                }
                positions.extend_from_slice(&coords[..3]);
                vertex_count += 1;
            }
            "vn" => {
                let coords: Vec<f32> = parts.filter_map(|value| value.parse().ok()).collect();
                if coords.len() < 3 {
                    return Err("obj: malformed vn line".into());
                }
                normals.extend_from_slice(&coords[..3]);
                normal_count += 1;
            }
            "f" => {
                let mut face: Vec<usize> = Vec::new();
                for token in parts {
                    let raw_index = token.split('/').next().ok_or_else(|| "obj: malformed face token".to_string())?;
                    let raw: i64 = raw_index.parse().map_err(|_| "obj: malformed face index".to_string())?;
                    face.push(obj_resolve_index(raw, vertex_count)?);
                }
                if face.len() < 3 {
                    continue;
                }
                for i in 1..face.len() - 1 {
                    indices.push(face[0] as u32);
                    indices.push(face[i] as u32);
                    indices.push(face[i + 1] as u32);
                }
            }
            _ => {}
        }
    }
    let mut mesh = MeshData {
        positions,
        indices,
        ..MeshData::default()
    };
    if normal_count == vertex_count && normal_count > 0 {
        mesh.normals = normals;
    } else {
        mesh.compute_normals();
    }
    Ok(mesh)
}

fn obj_resolve_index(raw: i64, count: usize) -> Result<usize, String> {
    if raw > 0 {
        Ok((raw - 1) as usize)
    } else if raw < 0 {
        let index = count as i64 + raw;
        if index < 0 {
            Err("obj: negative vertex index out of range".into())
        } else {
            Ok(index as usize)
        }
    } else {
        Err("obj: zero vertex index".into())
    }
}
//#endregion Obj

//#region Glb
pub fn mesh_to_glb(mesh: &MeshData) -> Vec<u8> {
    let positions = f32_slice_to_bytes(&mesh.positions);
    let normals = if mesh.normals.len() == mesh.positions.len() {
        f32_slice_to_bytes(&mesh.normals)
    } else {
        let mut copy = mesh.clone();
        copy.compute_normals();
        f32_slice_to_bytes(&copy.normals)
    };
    let indices = u32_slice_to_bytes(&mesh.indices);
    let bin = [positions.as_slice(), normals.as_slice(), indices.as_slice()].concat();
    let padded_bin = pad_to_4(bin);
    let positions_len = positions.len();
    let normals_len = normals.len();
    let indices_len = indices.len();
    let positions_offset = 0usize;
    let normals_offset = positions_offset + positions_len;
    let indices_offset = normals_offset + normals_len;
    let json = format!(
        r#"{{
  "asset": {{"version": "2.0"}},
  "scene": 0,
  "scenes": [{{"nodes": [0]}}],
  "nodes": [{{"mesh": 0}}],
  "meshes": [{{
    "primitives": [{{
      "attributes": {{"POSITION": 0, "NORMAL": 1}},
      "indices": 2,
      "mode": 4
    }}]
  }}],
  "accessors": [
    {{"bufferView": 0, "componentType": 5126, "count": {}, "type": "VEC3", "min": {}, "max": {}}},
    {{"bufferView": 1, "componentType": 5126, "count": {}, "type": "VEC3"}},
    {{"bufferView": 2, "componentType": 5125, "count": {}, "type": "SCALAR"}}
  ],
  "bufferViews": [
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}}
  ],
  "buffers": [{{"byteLength": {}}}]
}}"#,
        mesh.vertex_count(),
        json_vec3_min(&mesh.positions),
        json_vec3_max(&mesh.positions),
        mesh.vertex_count(),
        mesh.indices.len(),
        positions_offset,
        positions_len,
        normals_offset,
        normals_len,
        indices_offset,
        indices_len,
        padded_bin.len()
    );
    let mut json_bytes = json.into_bytes();
    while json_bytes.len() % 4 != 0 {
        json_bytes.push(b' ');
    }
    let total_len = 12 + 8 + json_bytes.len() + 8 + padded_bin.len();
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&(2u32).to_le_bytes());
    out.extend_from_slice(&(total_len as u32).to_le_bytes());
    out.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json_bytes);
    out.extend_from_slice(&(padded_bin.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\x00");
    out.extend_from_slice(&padded_bin);
    out
}

type GlbMatrix = [[f32; 4]; 4];

fn glb_identity() -> GlbMatrix {
    [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

fn glb_matrix_mul(left: GlbMatrix, right: GlbMatrix) -> GlbMatrix {
    let mut result = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            result[column][row] = (0..4).map(|axis| left[axis][row] * right[column][axis]).sum();
        }
    }
    result
}

fn glb_transform_point(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * point[0] + matrix[1][0] * point[1] + matrix[2][0] * point[2] + matrix[3][0],
        matrix[0][1] * point[0] + matrix[1][1] * point[1] + matrix[2][1] * point[2] + matrix[3][1],
        matrix[0][2] * point[0] + matrix[1][2] * point[1] + matrix[2][2] * point[2] + matrix[3][2],
    ]
}

fn glb_transform_normal(matrix: GlbMatrix, normal: [f32; 3]) -> [f32; 3] {
    let (a00, a01, a02) = (matrix[0][0], matrix[1][0], matrix[2][0]);
    let (a10, a11, a12) = (matrix[0][1], matrix[1][1], matrix[2][1]);
    let (a20, a21, a22) = (matrix[0][2], matrix[1][2], matrix[2][2]);
    let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20) + a02 * (a10 * a21 - a11 * a20);
    if det.abs() <= f32::EPSILON {
        return normal;
    }
    let inverse_det = det.recip();
    let transformed = [
        ((a11 * a22 - a12 * a21) * normal[0] + (a12 * a20 - a10 * a22) * normal[1] + (a10 * a21 - a11 * a20) * normal[2]) * inverse_det,
        ((a02 * a21 - a01 * a22) * normal[0] + (a00 * a22 - a02 * a20) * normal[1] + (a01 * a20 - a00 * a21) * normal[2]) * inverse_det,
        ((a01 * a12 - a02 * a11) * normal[0] + (a02 * a10 - a00 * a12) * normal[1] + (a00 * a11 - a01 * a10) * normal[2]) * inverse_det,
    ];
    let length = transformed.iter().map(|value| value * value).sum::<f32>().sqrt();
    if length <= f32::EPSILON {
        normal
    } else {
        transformed.map(|value| value / length)
    }
}

fn glb_triangle_indices(mode: gltf::mesh::Mode, source: Vec<u32>) -> Vec<u32> {
    match mode {
        gltf::mesh::Mode::Triangles => source,
        gltf::mesh::Mode::TriangleStrip => source.windows(3).enumerate().flat_map(|(index, tri)| if index % 2 == 0 { [tri[0], tri[1], tri[2]] } else { [tri[1], tri[0], tri[2]] }).collect(),
        gltf::mesh::Mode::TriangleFan => source.first().map(|first| source[1..].windows(2).flat_map(|pair| [*first, pair[0], pair[1]]).collect()).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn append_glb_primitive(mesh: &mut MeshData, primitive: gltf::Primitive<'_>, matrix: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    if !matches!(primitive.mode(), gltf::mesh::Mode::Triangles | gltf::mesh::Mode::TriangleStrip | gltf::mesh::Mode::TriangleFan) {
        return Ok(());
    }
    let reader = primitive.reader(|buffer| (buffer.index() == 0).then_some(bin));
    let positions: Vec<[f32; 3]> = reader.read_positions().ok_or_else(|| "glb triangle primitive missing POSITION".to_string())?.collect();
    let source_indices: Vec<u32> = reader.read_indices().map(|indices| indices.into_u32().collect()).unwrap_or_else(|| (0..positions.len() as u32).collect());
    let indices = glb_triangle_indices(primitive.mode(), source_indices);
    if indices.iter().any(|index| *index as usize >= positions.len()) {
        return Err("glb triangle index outside POSITION accessor".into());
    }
    let normals: Vec<[f32; 3]> = if let Some(normals) = reader.read_normals() {
        normals.collect()
    } else {
        let mut local = MeshData {
            positions: positions.iter().flatten().copied().collect(),
            indices: indices.clone(),
            ..Default::default()
        };
        local.compute_normals();
        local.normals.as_chunks::<3>().0.to_vec()
    };
    if normals.len() != positions.len() {
        return Err("glb NORMAL and POSITION accessor counts differ".into());
    }
    let vertex_offset = mesh.vertex_count() as u32;
    mesh.positions.extend(positions.into_iter().flat_map(|position| glb_transform_point(matrix, position)));
    mesh.normals.extend(normals.into_iter().flat_map(|normal| glb_transform_normal(matrix, normal)));
    mesh.indices.extend(indices.into_iter().map(|index| vertex_offset + index));
    Ok(())
}

fn append_glb_mesh(mesh: &mut MeshData, source: gltf::Mesh<'_>, matrix: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    for primitive in source.primitives() {
        append_glb_primitive(mesh, primitive, matrix, bin)?;
    }
    Ok(())
}

fn append_glb_node(mesh: &mut MeshData, node: gltf::Node<'_>, parent: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    let matrix = glb_matrix_mul(parent, node.transform().matrix());
    if let Some(source) = node.mesh() {
        append_glb_mesh(mesh, source, matrix, bin)?;
    }
    for child in node.children() {
        append_glb_node(mesh, child, matrix, bin)?;
    }
    Ok(())
}

/// 🧊️ Decodes every triangle primitive in the active GLB scene into one renderer-neutral mesh.
pub fn mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    let gltf = gltf::Gltf::from_slice(bytes).map_err(|error| error.to_string())?;
    let bin = gltf.blob.as_deref().ok_or_else(|| "glb missing BIN chunk".to_string())?;
    let mut mesh = MeshData::default();
    if let Some(scene) = gltf.default_scene().or_else(|| gltf.scenes().next()) {
        for node in scene.nodes() {
            append_glb_node(&mut mesh, node, glb_identity(), bin)?;
        }
    } else {
        for source in gltf.meshes() {
            append_glb_mesh(&mut mesh, source, glb_identity(), bin)?;
        }
    }
    if mesh.indices.is_empty() {
        return Err("glb contains no triangle primitives".into());
    }
    Ok(mesh)
}

fn f32_slice_to_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn u32_slice_to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while data.len() % 4 != 0 {
        data.push(0);
    }
    data
}

fn json_vec3_min(positions: &[f32]) -> String {
    let (min, _) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb();
    format!("[{}, {}, {}]", min[0], min[1], min[2])
}

fn json_vec3_max(positions: &[f32]) -> String {
    let (_, max) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb();
    format!("[{}, {}, {}]", max[0], max[1], max[2])
}
//#endregion Glb

//#region Stl
/// 🧱️ Hand-rolled binary STL: 80-byte header, `u32` little-endian triangle count, then per triangle a `f32x3` facet normal, three `f32x3` vertices, and a `u16` attribute-byte-count (written as 0). No vertex dedupe, matching the binary STL convention of one independent triangle per record.
pub fn mesh_to_stl(mesh: &MeshData) -> Vec<u8> {
    let triangle_count = mesh.triangle_count() as u32;
    let mut out = Vec::with_capacity(80 + 4 + triangle_count as usize * 50);
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&triangle_count.to_le_bytes());
    for tri in mesh.indices.chunks_exact(3) {
        let p0 = stl_vertex(&mesh.positions, tri[0]);
        let p1 = stl_vertex(&mesh.positions, tri[1]);
        let p2 = stl_vertex(&mesh.positions, tri[2]);
        let normal = stl_face_normal(p0, p1, p2);
        for component in normal {
            out.extend_from_slice(&component.to_le_bytes());
        }
        for vertex in [p0, p1, p2] {
            for component in vertex {
                out.extend_from_slice(&component.to_le_bytes());
            }
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}

pub fn mesh_from_stl(bytes: &[u8]) -> Result<MeshData, String> {
    if bytes.len() < 84 {
        return Err("stl: truncated header".into());
    }
    let triangle_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let expected_len = 84 + triangle_count * 50;
    if bytes.len() < expected_len {
        return Err("stl: truncated triangle data".into());
    }
    let mut mesh = MeshData::default();
    for triangle in 0..triangle_count {
        let base = 84 + triangle * 50;
        let mut normal = [0f32; 3];
        for axis in 0..3 {
            normal[axis] = f32::from_le_bytes(bytes[base + axis * 4..base + axis * 4 + 4].try_into().unwrap());
        }
        let vertex_base = base + 12;
        for corner in 0..3 {
            let corner_base = vertex_base + corner * 12;
            let mut position = [0f32; 3];
            for axis in 0..3 {
                position[axis] = f32::from_le_bytes(bytes[corner_base + axis * 4..corner_base + axis * 4 + 4].try_into().unwrap());
            }
            let index = (mesh.positions.len() / 3) as u32;
            mesh.positions.extend_from_slice(&position);
            mesh.normals.extend_from_slice(&normal);
            mesh.indices.push(index);
        }
    }
    Ok(mesh)
}

fn stl_vertex(positions: &[f32], index: u32) -> [f32; 3] {
    let base = index as usize * 3;
    [positions[base], positions[base + 1], positions[base + 2]]
}

fn stl_face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [
        e0[1] * e1[2] - e0[2] * e1[1],
        e0[2] * e1[0] - e0[0] * e1[2],
        e0[0] * e1[1] - e0[1] * e1[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 1e-8 {
        [n[0] / len, n[1] / len, n[2] / len]
    } else {
        [0.0, 0.0, 0.0]
    }
}
//#endregion Stl

//#region MediaFormat
/// 🗂️ OS-level media export/import format. Lives here (not in `framework/product/os/core`) because `framework/core` sits below `framework/product/os/core` in the dependency graph — `os/core` depends on `framework-core`, never the reverse — so the `MeshExporter`/`MeshImporter` traits below, and every OS registration site, share one definition; `framework/product/os/core` re-exports it verbatim. Serialize/Deserialize/TS derives added alongside the `//#region MediaType` lattice below since `MediaWireFormat`/`ArtifactKindSpec` carry this on the wire.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum OsMediaFormat {
    Svg,
    Png,
    Obj,
    Glb,
    Stl,
    Step,
    Dwg,
    /// ☁️ Point-cloud/mesh interchange (Stanford PLY) — registered for `remodel` photogrammetry
    /// export; no encoder lives here yet, this is the format-identifier registration only.
    Ply,
    /// ☁️ LiDAR/point-cloud binary interchange (ASPRS LAS) — same status as `Ply` above.
    Las,
}

impl OsMediaFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Svg => "svg",
            Self::Png => "png",
            Self::Obj => "obj",
            Self::Glb => "glb",
            Self::Stl => "stl",
            Self::Step => "step",
            Self::Dwg => "dwg",
            Self::Ply => "ply",
            Self::Las => "las",
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Svg => "image/svg+xml",
            Self::Png => "image/png",
            Self::Obj => "model/obj",
            Self::Glb => "model/gltf-binary",
            Self::Stl => "model/stl",
            Self::Step => "model/step",
            Self::Dwg => "image/vnd.dwg",
            Self::Ply => "model/ply",
            Self::Las => "application/vnd.las",
        }
    }

    /// @emoji 🔢️ Whether this format's payload is base64-encoded binary rather than plain text.
    /// PLY supports both an ASCII and a binary variant; this crate treats format-by-name as a single
    /// binary/text choice, so `Ply` is treated as the ASCII/text variant (matching `Obj`/`Step`) —
    /// downstream `remodel` encoders that want binary PLY still tag their payload accordingly, this
    /// flag only governs the default wire encoding. `Las` is always binary.
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Png | Self::Glb | Self::Stl | Self::Dwg | Self::Las)
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "svg" => Some(Self::Svg),
            "png" => Some(Self::Png),
            "obj" => Some(Self::Obj),
            "glb" => Some(Self::Glb),
            "stl" => Some(Self::Stl),
            "step" => Some(Self::Step),
            "dwg" => Some(Self::Dwg),
            "ply" => Some(Self::Ply),
            "las" => Some(Self::Las),
            _ => None,
        }
    }
}
//#endregion MediaFormat

//#region ArtifactKind
/// 🧬️ Which geometry backend a resource kind's media exporters/importers target — the manifest-level
/// counterpart threaded onto `AppDefinition.artifact_kinds` (see `ArtifactKindSpec`). Canonical home for
/// what used to be duplicated verbatim in `framework/plugin/rs` and `framework/product/os/core/rs`; both
/// now re-export this definition instead of declaring their own.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum OsMediaCapability {
    MeshOnly,
    Brep,
}

/// 🗂️ An app-declared OS resource kind (e.g. a 3D mesh format, a raster format) — the manifest-level
/// counterpart to `AppBuilder::artifact_kind(...)` (`framework/plugin/rs`), letting `framework/product/os/core`
/// build its artifact catalog from `AppDefinition.artifact_kinds` at plugin registration time instead of
/// hardcoding a per-app match on kind-id strings. Carries the manifest-level media-kind fields
/// (`media_type`/`schema`/`export_formats`/`import_formats`) directly
/// so one spec carries both the OS-catalog presentation shape and the `MediaType` a wire actually negotiates
/// — see `crate::media_types_compatible`. `OsArtifactDescriptor` (`framework/product/os/core`) threads
/// `media_type` through so registry lookups return it alongside the rest of the descriptor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ArtifactKindSpec {
    pub id: String,
    pub name: String,
    pub source_format: String,
    pub component_kind: String,
    pub dimension: String,
    pub media_capability: OsMediaCapability,
    pub media_type: MediaType,
    pub schema: String,
    pub export_formats: Vec<OsMediaFormat>,
    pub import_formats: Vec<OsMediaFormat>,
}
//#endregion ArtifactKind

//#region MediaType
/// 🧬️ Typed-media lattice: every port/wire in the workflow carries a `MediaType` (`class` × `form`) instead of the legacy string `artifact_kind`. This is separate from `OsMediaFormat` above — `MediaType` is what a wire negotiates, `OsMediaFormat` is only how bytes are encoded once they actually cross a process boundary (see `MediaWireFormat`). Dependent tickets retire `OsMediaCapability` (see the `ArtifactKind` region above) onto `MediaForm::{Brep,Mesh}`, which already covers what `OsMediaCapability::{Brep,MeshOnly}` expresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MediaClass {
    TwoD,
    ThreeD,
    Text,
    Data,
    Graph,
    Kit,
    Computation,
    Presentation,
}

/// 🧬️ The shape/representation a `MediaClass` payload takes, orthogonal to `class` — e.g. `ThreeD` × `Brep` vs `ThreeD` × `Mesh`. `Any` only ever appears on the accepting side of a port (see `media_types_compatible`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MediaForm {
    Any,
    Vector,
    Raster,
    Brep,
    Mesh,
    Document,
    Value,
    Dag,
    Trinity,
    Type,
    Design,
    Kit,
    Flow,
    Sequence,
    Imperative,
    Deck,
}

/// 🧬️ A port or wire's declared media type — the pair a producer offers or a consumer accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    pub class: MediaClass,
    pub form: MediaForm,
}

/// 🔌️ How a `MediaType` is actually encoded once it crosses a process boundary — binary payloads reuse `OsMediaFormat`, structured payloads carry a schema id instead (see `ArtifactKindSpec::schema`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum MediaWireFormat {
    Binary { format: OsMediaFormat },
    Document { schema: String }
}

/// 🔀️ Which side of a wire a `MediaPortSpec` sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MediaPortDirection {
    In,
    Out,
}

/// 🔢️ Whether a `MediaPortSpec` accepts/produces exactly one media value or a stream/collection of them — e.g. a mesh-array input that fans in from several upstream producers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum PortMultiplicity {
    One,
    Many,
}

/// 🔌️ A single port an app exposes on the workflow — `kind_id` optionally pins it to one `ArtifactKindSpec.id` when the port is more specific than its `media_type` alone conveys.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct MediaPortSpec {
    pub id: String,
    pub label: String,
    pub direction: MediaPortDirection,
    pub media_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub kind_id: Option<String>,
    pub required: bool,
    pub multiplicity: PortMultiplicity,
}

/// ⚖️ Result of checking whether a producer's `MediaType` can feed a consumer's accepted `MediaType`: exact match, a known lossy-but-allowed conversion, or outright rejection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaCompat {
    Direct,
    Convert { from: MediaForm, to: MediaForm },
    Reject,
}

/// 🔀️ One-way `MediaForm` conversions the workflow is allowed to insert implicitly (e.g. a B-Rep producer feeding a mesh-only consumer). `media_types_compatible` looks up `(produced, accepted)` directly, so add the reverse pair too if a conversion should also hold the other way.
const MEDIA_FORM_CONVERSIONS: &[(MediaForm, MediaForm)] = &[
    (MediaForm::Brep, MediaForm::Mesh),
    (MediaForm::Vector, MediaForm::Raster),
    (MediaForm::Design, MediaForm::Kit),
    (MediaForm::Type, MediaForm::Kit),
];

/// ⚖️ The single source of truth for wire compatibility: classes must match exactly, `MediaForm::Any` on the accepting side takes anything within the class, equal forms are always direct, and everything else falls through to the explicit `MEDIA_FORM_CONVERSIONS` table.
pub fn media_types_compatible(produced: &MediaType, accepted: &MediaType) -> MediaCompat {
    if produced.class != accepted.class {
        return MediaCompat::Reject;
    }
    if matches!(accepted.form, MediaForm::Any) || produced.form == accepted.form {
        return MediaCompat::Direct;
    }
    for (from, to) in MEDIA_FORM_CONVERSIONS {
        if *from == produced.form && *to == accepted.form {
            return MediaCompat::Convert { from: *from, to: *to };
        }
    }
    MediaCompat::Reject
}
//#endregion MediaType

//#region 🔖️AppIo
/// 🧷️ The non-format fields of `ArtifactKindSpec` (see `ArtifactKind` region above) that describe how
/// a resource presents in the OS catalog — split out so `AppIo` can carry its own `export_formats`/
/// `import_formats` lists without duplicating `ArtifactKindSpec`'s full shape (which stays alive
/// unchanged for now; later waves retire it onto `AppIo`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ArtifactPresentation {
    pub id: String,
    pub name: String,
    pub dimension: String,
    pub component_kind: String,
}

/// 🔌️ An app's full media I/O surface — the document schema/type every app carries implicitly (see
/// `document_in_port`/`document_out_port`) plus whatever additional workflow ports, catalog
/// export/import formats, and OS presentation it declares itself. Scaffolding for the typed manifest
/// surface (`AppDefinition.io`); apps don't populate this yet — later waves migrate `media_inputs`/
/// `media_outputs`/`artifact_kinds` onto it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct AppIo {
    pub document_schema: String,
    pub document_media_type: MediaType,
    /// 🔌️ App-specific ports only — the implicit document ports are auto-injected by `all_ports`.
    pub ports: Vec<MediaPortSpec>,
    pub export_formats: Vec<OsMediaFormat>,
    pub import_formats: Vec<OsMediaFormat>,
    pub artifact: ArtifactPresentation,
}

impl AppIo {
    /// 🔌️ The implicit `"document:in"` port every app accepts, keyed by `self.document_media_type`.
    pub fn document_in_port(&self) -> MediaPortSpec {
        MediaPortSpec {
            id: "document:in".into(),
            label: "Document".into(),
            direction: MediaPortDirection::In,
            media_type: self.document_media_type,
            kind_id: None,
            required: true,
            multiplicity: PortMultiplicity::One,
        }
    }

    /// 🔌️ The implicit `"document:out"` port every app produces — see `document_in_port`.
    pub fn document_out_port(&self) -> MediaPortSpec {
        MediaPortSpec {
            id: "document:out".into(),
            label: "Document".into(),
            direction: MediaPortDirection::Out,
            media_type: self.document_media_type,
            kind_id: None,
            required: true,
            multiplicity: PortMultiplicity::One,
        }
    }

    /// 🔌️ The full port list, in stable order: the implicit document ports first, followed by every app-specific port declared in `self.ports`.
    pub fn all_ports(&self) -> Vec<MediaPortSpec> {
        let mut ports = vec![self.document_in_port(), self.document_out_port()];
        ports.extend(self.ports.clone());
        ports
    }

    /// 🏗️ Builds an `AppIo` from just its implicit document surface, with no extra ports/formats declared yet — chain `.with_ports(...)` to add app-specific ports.
    pub fn from_document(schema: impl Into<String>, media_type: MediaType, artifact: ArtifactPresentation) -> Self {
        Self {
            document_schema: schema.into(),
            document_media_type: media_type,
            ports: Vec::new(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact,
        }
    }

    /// 🔌️ Attaches app-specific ports (beyond the implicit document ports) to this `AppIo`.
    pub fn with_ports(mut self, ports: Vec<MediaPortSpec>) -> Self {
        self.ports = ports;
        self
    }
}

impl Default for AppIo {
    fn default() -> Self {
        Self {
            document_schema: String::new(),
            document_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            ports: Vec::new(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact: ArtifactPresentation {
                id: String::new(),
                name: String::new(),
                dimension: String::new(),
                component_kind: String::new(),
            },
        }
    }
}
//#endregion 🔖️AppIo

//#region 🔖️ConfigSpec
/// 🧮️ How one config field's value is edited/validated, independent of what record it belongs to.
/// Deliberately hand-rolled rather than derived from `dsl_schema::Shape` (`dsl_schema`'s `Shape` isn't
/// `Serialize`/`Deserialize` — `Shape::Record`/`Statements`/`Table` carry `fn() -> RecordSpec` pointers
/// — and `semio-framework-core` doesn't depend on `dsl`/`dsl_schema` today, so wrapping it would add a
/// new cross-crate dependency purely to reach a shape that can't round-trip over the wire anyway).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum ConfigFieldShape {
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        step: Option<f64>,
    },
    Toggle,
    Text,
    Select { options: Vec<String> },
    Record(Vec<ConfigFieldSpec>),
}

/// 🧮️ One field of an app's declared configuration record — the whole-app-settings counterpart to
/// `ActionArgDef` (which scopes to a single action's arguments instead).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ConfigFieldSpec {
    pub key: String,
    pub label: String,
    pub shape: ConfigFieldShape,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub default: Option<DslValue>,
}

/// 🧮️ An app's full typed configuration record — the manifest-level declaration
/// `AppDefinition.config` carries. Empty until per-app waves populate it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ConfigSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<ConfigFieldSpec>,
}

impl ConfigSpec {
    pub fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️ConfigSpec

//#region 🔖️CommandGrammar
/// 🎛️ One field of a binary command variant — reuses `ConfigFieldShape` for the value shape (see
/// `ConfigFieldShape`'s doc comment for why command grammar fields are hand-rolled rather than
/// derived from `dsl_schema`). No `List`/array shape exists yet — the manifest's existing field-typed
/// vocabulary (`ActionArgControl`: Text/Number/Slider/Toggle/Select/Vec3/IconSelect) has no array
/// control either, so `ConfigFieldShape` doesn't invent one ahead of a real need.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandFieldSpec {
    pub key: String,
    pub shape: ConfigFieldShape,
    pub optional: bool,
}

/// 🎛️ One keyword-dispatched command variant (e.g. `move x=1 y=2`) and its field grammar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandVariantSpec {
    pub keyword: String,
    pub fields: Vec<CommandFieldSpec>,
}

/// 🎛️ An app's full typed binary command grammar — the manifest-level declaration
/// `AppDefinition.command_grammar` carries. Empty until per-app waves populate it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandGrammar {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<CommandVariantSpec>,
}

impl CommandGrammar {
    pub fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️CommandGrammar

//#region Media
/// 🎞️ The value that actually flows over a workflow wire, produced by `DocumentApp::export_media` and consumed by `DocumentApp::import_media`. Kept separate from the `MediaType` lattice above (which only negotiates *compatibility*, never carries a value) so headless runners and the UI share one payload shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub media_type: MediaType,
    pub payload: MediaPayload,
}

/// 📦️ Structured payloads stay inline as canonical JSON (small, diffable); binary payloads are content-addressed through `store::BlobStore` so a `Media` value never carries megabytes across a WIT boundary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum MediaPayload {
    Structured { schema: String, json: String },
    Binary { format: OsMediaFormat, blob_hash: String }
}

/// 🔑️ A cheap identity for one port's current output, independent of serializing the full payload — the unit the `SpaceRunner` compares to decide whether a downstream node actually needs to see a new value.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct MediaFingerprint(pub String);

impl MediaFingerprint {
    /// 🔑️ Canonical fingerprint of a `Media` value: structured payloads hash their JSON text, binary payloads reuse their existing content hash directly (no re-hashing bytes already addressed by the blob store).
    pub fn of(media: &Media) -> Self {
        match &media.payload {
            MediaPayload::Structured { schema, json } => {
                MediaFingerprint(semio_framework_hash::hash_parts(&[schema.as_str(), json.as_str()]))
            }
            MediaPayload::Binary { blob_hash, .. } => MediaFingerprint(blob_hash.clone()),
        }
    }
}

/// 🚧️ Failure exporting, importing, or fingerprinting media on a declared port.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MediaError {
    #[error("unknown media port `{0}`")]
    UnknownPort(String),
    #[error("port `{port}` produced {produced:?} but the wire accepts {accepted:?}")]
    Incompatible { port: String, produced: MediaType, accepted: MediaType },
    #[error("media payload error on port `{0}`: {1}")]
    Payload(String, String),
    #[error("media ports are not implemented for this app")]
    NotImplemented,
}

/// 🔀️ A registered one-way conversion the workflow may insert on a wire when `media_types_compatible` reports `MediaCompat::Convert`. Kept behind a trait (never a bare closure) so converters can be enumerated, tested, and swapped without touching the runner.
pub trait MediaConverter: Send + Sync {
    fn from_form(&self) -> MediaForm;
    fn to_form(&self) -> MediaForm;
    fn convert(&self, media: &Media) -> Result<Media, MediaError>;
}
//#endregion Media

//#region MeshCodec
/// 🔌️ Format-keyed mesh export codec; concrete implementations below are zero-dependency (hand-rolled OBJ/GLB/STL). B-Rep apps additionally get `SolidExporter` (kernel/3d/brep/rs) which wraps the real kernel's STEP/STL/OBJ writers, and reuse `GlbExporter`/`GlbImporter` here via a tessellation bridge so GLB is the same codec everywhere.
pub trait MeshExporter: Send + Sync {
    fn format(&self) -> OsMediaFormat;
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String>;
}

/// 🔌️ Format-keyed mesh import codec; see `MeshExporter`.
pub trait MeshImporter: Send + Sync {
    fn format(&self) -> OsMediaFormat;
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String>;
}

pub struct ObjExporter;
impl MeshExporter for ObjExporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Obj
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_obj(mesh, "mesh").into_bytes())
    }
}

pub struct ObjImporter;
impl MeshImporter for ObjImporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Obj
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        mesh_from_obj(text)
    }
}

pub struct GlbExporter;
impl MeshExporter for GlbExporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Glb
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_glb(mesh))
    }
}

pub struct GlbImporter;
impl MeshImporter for GlbImporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Glb
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_glb(bytes)
    }
}

pub struct StlExporter;
impl MeshExporter for StlExporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Stl
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_stl(mesh))
    }
}

pub struct StlImporter;
impl MeshImporter for StlImporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Stl
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_stl(bytes)
    }
}
//#endregion MeshCodec

//#region Dwg
/// 📐️ Hand-rolled DWG codec: a self-contained, round-trippable binary interchange format using the AC1015 (R2000) file magic and an R2000-flavored section-locator/CRC/handle container (bit primitives BS/BL/BD/handle refs per https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf). Entity/header field layouts are a semio-defined subset chosen for lossless round-tripping through this codec; byte-exact third-party AutoCAD/ODA interop needs follow-up validation against a real DWG viewer.

//#region DwgTypes
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DwgDrawing {
    pub layers: Vec<DwgLayer>,
    pub entities: Vec<DwgEntity>,
    pub extmin: [f64; 3],
    pub extmax: [f64; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct DwgLayer {
    pub name: String,
    pub color: u8,
}

impl Default for DwgLayer {
    fn default() -> Self {
        Self { name: "0".to_string(), color: 7 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DwgColor {
    ByLayer,
    ByBlock,
    Index(u8),
}

impl DwgColor {
    fn to_bs(self) -> u16 {
        match self {
            DwgColor::ByLayer => 256,
            DwgColor::ByBlock => 0,
            DwgColor::Index(index) => index as u16,
        }
    }

    fn from_bs(value: u16) -> Self {
        match value {
            256 => DwgColor::ByLayer,
            0 => DwgColor::ByBlock,
            other => DwgColor::Index(other as u8),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DwgEntity {
    pub layer: usize,
    pub color: DwgColor,
    pub geometry: DwgGeometry,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DwgGeometry {
    Line { start: [f64; 3], end: [f64; 3] },
    Point { at: [f64; 3] },
    Circle { center: [f64; 3], radius: f64, normal: [f64; 3] },
    Arc { center: [f64; 3], radius: f64, start_angle: f64, end_angle: f64, normal: [f64; 3] },
    Ellipse { center: [f64; 3], major_axis: [f64; 3], ratio: f64, start_param: f64, end_param: f64, normal: [f64; 3] },
    LwPolyline { closed: bool, elevation: f64, vertices: Vec<[f64; 2]>, bulges: Vec<f64> },
    Spline { degree: u32, control_points: Vec<[f64; 3]>, knots: Vec<f64>, weights: Vec<f64> },
    Text { at: [f64; 3], height: f64, rotation: f64, content: String },
    Face3d { corners: [[f64; 3]; 4] },
    Polyline3d { closed: bool, vertices: Vec<[f64; 3]> },
    PolyfaceMesh { vertices: Vec<[f64; 3]>, faces: Vec<[i32; 4]> }
}

impl DwgDrawing {
    pub fn ensure_layer(&mut self, name: &str) -> usize {
        if let Some(index) = self.layers.iter().position(|layer| layer.name == name) {
            return index;
        }
        self.layers.push(DwgLayer { name: name.to_string(), color: 7 });
        self.layers.len() - 1
    }

    fn recompute_extents(&mut self) {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        let touch = |p: [f64; 3], min: &mut [f64; 3], max: &mut [f64; 3]| {
            for axis in 0..3 {
                min[axis] = min[axis].min(p[axis]);
                max[axis] = max[axis].max(p[axis]);
            }
        };
        for entity in &self.entities {
            match &entity.geometry {
                DwgGeometry::Line { start, end } => {
                    touch(*start, &mut min, &mut max);
                    touch(*end, &mut min, &mut max);
                }
                DwgGeometry::Point { at } => touch(*at, &mut min, &mut max),
                DwgGeometry::Circle { center, radius, .. } | DwgGeometry::Arc { center, radius, .. } => {
                    touch([center[0] - radius, center[1] - radius, center[2]], &mut min, &mut max);
                    touch([center[0] + radius, center[1] + radius, center[2]], &mut min, &mut max);
                }
                DwgGeometry::Ellipse { center, major_axis, .. } => {
                    let r = (major_axis[0] * major_axis[0] + major_axis[1] * major_axis[1]).sqrt();
                    touch([center[0] - r, center[1] - r, center[2]], &mut min, &mut max);
                    touch([center[0] + r, center[1] + r, center[2]], &mut min, &mut max);
                }
                DwgGeometry::LwPolyline { vertices, elevation, .. } => {
                    for v in vertices {
                        touch([v[0], v[1], *elevation], &mut min, &mut max);
                    }
                }
                DwgGeometry::Spline { control_points, .. } | DwgGeometry::Polyline3d { vertices: control_points, .. } => {
                    for p in control_points {
                        touch(*p, &mut min, &mut max);
                    }
                }
                DwgGeometry::PolyfaceMesh { vertices, .. } => {
                    for p in vertices {
                        touch(*p, &mut min, &mut max);
                    }
                }
                DwgGeometry::Text { at, .. } => touch(*at, &mut min, &mut max),
                DwgGeometry::Face3d { corners } => {
                    for p in corners {
                        touch(*p, &mut min, &mut max);
                    }
                }
            }
        }
        if min[0].is_finite() {
            self.extmin = min;
            self.extmax = max;
        }
    }
}
//#endregion DwgTypes

//#region DwgBits
struct DwgBitWriter {
    bytes: Vec<u8>,
    bit: u8,
}

impl DwgBitWriter {
    fn new() -> Self {
        Self { bytes: Vec::new(), bit: 0 }
    }

    fn write_bit(&mut self, value: bool) {
        if self.bit == 0 {
            self.bytes.push(0);
        }
        if value {
            let last = self.bytes.len() - 1;
            self.bytes[last] |= 1 << (7 - self.bit);
        }
        self.bit = (self.bit + 1) % 8;
    }

    fn write_bits(&mut self, value: u64, count: u8) {
        for i in (0..count).rev() {
            self.write_bit((value >> i) & 1 != 0);
        }
    }

    fn write_b(&mut self, value: bool) {
        self.write_bit(value);
    }

    fn write_bb(&mut self, value: u8) {
        self.write_bits(value as u64, 2);
    }

    fn write_rc(&mut self, value: u8) {
        self.write_bits(value as u64, 8);
    }

    fn write_rs(&mut self, value: u16) {
        self.write_rc((value & 0xFF) as u8);
        self.write_rc((value >> 8) as u8);
    }

    fn write_rl(&mut self, value: u32) {
        self.write_rs((value & 0xFFFF) as u16);
        self.write_rs((value >> 16) as u16);
    }

    fn write_rd(&mut self, value: f64) {
        let bits = value.to_bits();
        self.write_rl((bits & 0xFFFF_FFFF) as u32);
        self.write_rl((bits >> 32) as u32);
    }

    fn write_bs(&mut self, value: u16) {
        match value {
            0 => self.write_bb(2),
            256 => self.write_bb(3),
            v if v <= 0xFF => {
                self.write_bb(1);
                self.write_rc(v as u8);
            }
            v => {
                self.write_bb(0);
                self.write_rs(v);
            }
        }
    }

    fn write_bl(&mut self, value: u32) {
        match value {
            0 => self.write_bb(2),
            v if v <= 0xFF => {
                self.write_bb(1);
                self.write_rc(v as u8);
            }
            v => {
                self.write_bb(0);
                self.write_rl(v);
            }
        }
    }

    fn write_bd(&mut self, value: f64) {
        if value == 0.0 {
            self.write_bb(2);
        } else if value == 1.0 {
            self.write_bb(1);
        } else {
            self.write_bb(0);
            self.write_rd(value);
        }
    }

    fn write_2rd(&mut self, v: [f64; 2]) {
        self.write_rd(v[0]);
        self.write_rd(v[1]);
    }

    fn write_3bd(&mut self, v: [f64; 3]) {
        self.write_bd(v[0]);
        self.write_bd(v[1]);
        self.write_bd(v[2]);
    }

    fn write_3rd(&mut self, v: [f64; 3]) {
        self.write_rd(v[0]);
        self.write_rd(v[1]);
        self.write_rd(v[2]);
    }

    fn write_be(&mut self, normal: [f64; 3]) {
        if normal == [0.0, 0.0, 1.0] {
            self.write_b(true);
        } else {
            self.write_b(false);
            self.write_3bd(normal);
        }
    }

    fn write_t(&mut self, text: &str) {
        let bytes = text.as_bytes();
        let len = bytes.len().min(0xFFFF);
        self.write_rs(len as u16);
        for &b in &bytes[..len] {
            self.write_rc(b);
        }
    }

    fn write_ms(&mut self, mut value: u32) {
        loop {
            let mut chunk = (value & 0x7FFF) as u16;
            value >>= 15;
            if value != 0 {
                chunk |= 0x8000;
                self.write_rs(chunk);
            } else {
                self.write_rs(chunk);
                break;
            }
        }
    }

    fn write_handle(&mut self, code: u8, handle: u64) {
        let mut bytes = Vec::new();
        let mut v = handle;
        while v != 0 {
            bytes.insert(0, (v & 0xFF) as u8);
            v >>= 8;
        }
        self.write_rc((code << 4) | bytes.len() as u8);
        for b in bytes {
            self.write_rc(b);
        }
    }

    fn pad_to_byte(&mut self) {
        while self.bit != 0 {
            self.write_bit(false);
        }
    }

    fn bit_len(&self) -> usize {
        self.bytes.len() * 8 - if self.bit == 0 { 0 } else { 8 - self.bit as usize }
    }
}

struct DwgBitReader<'a> {
    bytes: &'a [u8],
    byte_pos: usize,
    bit: u8,
}

impl<'a> DwgBitReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, byte_pos: 0, bit: 0 }
    }

    fn read_bit(&mut self) -> Result<bool, String> {
        if self.byte_pos >= self.bytes.len() {
            return Err("dwg bitstream underflow".to_string());
        }
        let value = (self.bytes[self.byte_pos] >> (7 - self.bit)) & 1 != 0;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.byte_pos += 1;
        }
        Ok(value)
    }

    fn read_bits(&mut self, count: u8) -> Result<u64, String> {
        let mut value = 0u64;
        for _ in 0..count {
            value = (value << 1) | self.read_bit()? as u64;
        }
        Ok(value)
    }

    fn read_b(&mut self) -> Result<bool, String> {
        self.read_bit()
    }

    fn read_bb(&mut self) -> Result<u8, String> {
        Ok(self.read_bits(2)? as u8)
    }

    fn read_rc(&mut self) -> Result<u8, String> {
        Ok(self.read_bits(8)? as u8)
    }

    fn read_rs(&mut self) -> Result<u16, String> {
        let lo = self.read_rc()? as u16;
        let hi = self.read_rc()? as u16;
        Ok(lo | (hi << 8))
    }

    fn read_rl(&mut self) -> Result<u32, String> {
        let lo = self.read_rs()? as u32;
        let hi = self.read_rs()? as u32;
        Ok(lo | (hi << 16))
    }

    fn read_rd(&mut self) -> Result<f64, String> {
        let lo = self.read_rl()? as u64;
        let hi = self.read_rl()? as u64;
        Ok(f64::from_bits(lo | (hi << 32)))
    }

    fn read_bs(&mut self) -> Result<u16, String> {
        match self.read_bb()? {
            0 => self.read_rs(),
            1 => Ok(self.read_rc()? as u16),
            2 => Ok(0),
            _ => Ok(256),
        }
    }

    fn read_bl(&mut self) -> Result<u32, String> {
        match self.read_bb()? {
            0 => self.read_rl(),
            1 => Ok(self.read_rc()? as u32),
            2 => Ok(0),
            _ => Err("invalid BL flag".to_string()),
        }
    }

    fn read_bd(&mut self) -> Result<f64, String> {
        match self.read_bb()? {
            0 => self.read_rd(),
            1 => Ok(1.0),
            2 => Ok(0.0),
            _ => Err("invalid BD flag".to_string()),
        }
    }

    fn read_2rd(&mut self) -> Result<[f64; 2], String> {
        Ok([self.read_rd()?, self.read_rd()?])
    }

    fn read_3bd(&mut self) -> Result<[f64; 3], String> {
        Ok([self.read_bd()?, self.read_bd()?, self.read_bd()?])
    }

    fn read_be(&mut self) -> Result<[f64; 3], String> {
        if self.read_b()? {
            Ok([0.0, 0.0, 1.0])
        } else {
            self.read_3bd()
        }
    }

    fn read_t(&mut self) -> Result<String, String> {
        let len = self.read_rs()? as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.read_rc()?);
        }
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    fn read_ms(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        let mut shift = 0;
        loop {
            let chunk = self.read_rs()?;
            value |= ((chunk & 0x7FFF) as u32) << shift;
            shift += 15;
            if chunk & 0x8000 == 0 {
                break;
            }
        }
        Ok(value)
    }

    fn read_handle(&mut self) -> Result<(u8, u64), String> {
        let head = self.read_rc()?;
        let code = head >> 4;
        let len = head & 0x0F;
        let mut value = 0u64;
        for _ in 0..len {
            value = (value << 8) | self.read_rc()? as u64;
        }
        Ok((code, value))
    }

    fn pad_to_byte(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.byte_pos += 1;
        }
    }
}

fn dwg_crc16(seed: u16, data: &[u8]) -> u16 {
    let mut crc = seed;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}
//#endregion DwgBits

//#region DwgObjects
const DWG_TYPE_LAYER: u16 = 51;
const DWG_TYPE_LINE: u16 = 19;
const DWG_TYPE_POINT: u16 = 27;
const DWG_TYPE_CIRCLE: u16 = 18;
const DWG_TYPE_ARC: u16 = 17;
const DWG_TYPE_ELLIPSE: u16 = 35;
const DWG_TYPE_LWPOLYLINE: u16 = 77;
const DWG_TYPE_SPLINE: u16 = 36;
const DWG_TYPE_TEXT: u16 = 1;
const DWG_TYPE_FACE3D: u16 = 28;
const DWG_TYPE_POLYLINE3D: u16 = 16;
const DWG_TYPE_POLYLINE_PFACE: u16 = 29;

const HANDLE_MODEL_SPACE: u64 = 0x10;
const HANDLE_LAYER_BASE: u64 = 0x20;
const HANDLE_ENTITY_BASE: u64 = 0x1000;

fn dwg_write_object(out: &mut Vec<u8>, object_type: u16, handle: u64, body: &mut DwgBitWriter, handles: &mut DwgBitWriter) {
    let bitsize = body.bit_len() as u32;
    body.pad_to_byte();
    handles.pad_to_byte();

    let mut framed = DwgBitWriter::new();
    framed.write_bs(object_type);
    framed.write_rl(bitsize);
    framed.write_handle(0, handle);
    framed.pad_to_byte();
    for byte in &body.bytes {
        framed.bytes.push(*byte);
    }
    for byte in &handles.bytes {
        framed.bytes.push(*byte);
    }

    let payload = framed.bytes;
    let mut sized = DwgBitWriter::new();
    sized.write_ms(payload.len() as u32);
    sized.pad_to_byte();

    out.extend_from_slice(&sized.bytes);
    out.extend_from_slice(&payload);
    let crc = dwg_crc16(0xC0C1, &payload);
    out.extend_from_slice(&crc.to_le_bytes());
}

fn dwg_encode_entity_common(body: &mut DwgBitWriter, handles: &mut DwgBitWriter, layer_handle: u64, color: DwgColor) {
    body.write_bb(0);
    body.write_bl(0);
    body.write_b(true);
    body.write_bs(color.to_bs());
    body.write_bd(1.0);
    body.write_bb(0);
    body.write_bb(0);
    body.write_bs(0);
    body.write_rc(29);

    handles.write_handle(3, HANDLE_MODEL_SPACE);
    handles.write_handle(5, layer_handle);
}

fn dwg_decode_entity_common(reader: &mut DwgBitReader<'_>) -> Result<DwgColor, String> {
    let _entmode = reader.read_bb()?;
    let _numreactors = reader.read_bl()?;
    let _nolinks = reader.read_b()?;
    let color = DwgColor::from_bs(reader.read_bs()?);
    let _ltype_scale = reader.read_bd()?;
    let _ltype_flags = reader.read_bb()?;
    let _plotstyle_flags = reader.read_bb()?;
    let _invisibility = reader.read_bs()?;
    let _lineweight = reader.read_rc()?;
    Ok(color)
}

fn dwg_decode_entity_handles(reader: &mut DwgBitReader<'_>) -> Result<u64, String> {
    reader.pad_to_byte();
    let (_owner_code, _owner) = reader.read_handle()?;
    let (_layer_code, layer_handle) = reader.read_handle()?;
    Ok(layer_handle)
}

fn dwg_encode_entity(objects_bytes: &mut Vec<u8>, object_map: &mut Vec<(u64, usize)>, next_handle: &mut u64, layer_handle: u64, entity: &DwgEntity) {
    let handle = *next_handle;
    *next_handle += 1;
    let mut body = DwgBitWriter::new();
    let mut handles = DwgBitWriter::new();
    dwg_encode_entity_common(&mut body, &mut handles, layer_handle, entity.color);

    let object_type = match &entity.geometry {
        DwgGeometry::Line { start, end } => {
            body.write_3bd(*start);
            body.write_3bd(*end);
            DWG_TYPE_LINE
        }
        DwgGeometry::Point { at } => {
            body.write_3bd(*at);
            DWG_TYPE_POINT
        }
        DwgGeometry::Circle { center, radius, normal } => {
            body.write_3bd(*center);
            body.write_bd(*radius);
            body.write_be(*normal);
            DWG_TYPE_CIRCLE
        }
        DwgGeometry::Arc { center, radius, start_angle, end_angle, normal } => {
            body.write_3bd(*center);
            body.write_bd(*radius);
            body.write_bd(*start_angle);
            body.write_bd(*end_angle);
            body.write_be(*normal);
            DWG_TYPE_ARC
        }
        DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal } => {
            body.write_3bd(*center);
            body.write_3bd(*major_axis);
            body.write_be(*normal);
            body.write_bd(*ratio);
            body.write_bd(*start_param);
            body.write_bd(*end_param);
            DWG_TYPE_ELLIPSE
        }
        DwgGeometry::Text { at, height, rotation, content } => {
            body.write_3bd(*at);
            body.write_bd(*height);
            body.write_bd(*rotation);
            body.write_t(content);
            DWG_TYPE_TEXT
        }
        DwgGeometry::Face3d { corners } => {
            for corner in corners {
                body.write_3bd(*corner);
            }
            DWG_TYPE_FACE3D
        }
        DwgGeometry::LwPolyline { closed, elevation, vertices, bulges } => {
            body.write_b(*closed);
            body.write_bd(*elevation);
            body.write_bl(vertices.len() as u32);
            for (i, v) in vertices.iter().enumerate() {
                body.write_2rd(*v);
                body.write_bd(bulges.get(i).copied().unwrap_or(0.0));
            }
            DWG_TYPE_LWPOLYLINE
        }
        DwgGeometry::Spline { degree, control_points, knots, weights } => {
            body.write_bl(*degree);
            body.write_bl(control_points.len() as u32);
            for p in control_points {
                body.write_3bd(*p);
            }
            body.write_bl(knots.len() as u32);
            for k in knots {
                body.write_rd(*k);
            }
            body.write_bl(weights.len() as u32);
            for w in weights {
                body.write_rd(*w);
            }
            DWG_TYPE_SPLINE
        }
        DwgGeometry::Polyline3d { closed, vertices } => {
            body.write_b(*closed);
            body.write_bl(vertices.len() as u32);
            for v in vertices {
                body.write_3bd(*v);
            }
            DWG_TYPE_POLYLINE3D
        }
        DwgGeometry::PolyfaceMesh { vertices, faces } => {
            body.write_bl(vertices.len() as u32);
            for v in vertices {
                body.write_3bd(*v);
            }
            body.write_bl(faces.len() as u32);
            for face in faces {
                for idx in face {
                    body.write_bl(idx.unsigned_abs());
                    body.write_b(*idx < 0);
                }
            }
            DWG_TYPE_POLYLINE_PFACE
        }
    };

    let offset = objects_bytes.len();
    dwg_write_object(objects_bytes, object_type, handle, &mut body, &mut handles);
    object_map.push((handle, offset));
}

fn dwg_decode_entity(object_type: u16, reader: &mut DwgBitReader<'_>) -> Result<Option<(u64, DwgColor, DwgGeometry)>, String> {
    match object_type {
        DWG_TYPE_LINE => {
            let color = dwg_decode_entity_common(reader)?;
            let start = reader.read_3bd()?;
            let end = reader.read_3bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Line { start, end })))
        }
        DWG_TYPE_POINT => {
            let color = dwg_decode_entity_common(reader)?;
            let at = reader.read_3bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Point { at })))
        }
        DWG_TYPE_CIRCLE => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let radius = reader.read_bd()?;
            let normal = reader.read_be()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Circle { center, radius, normal })))
        }
        DWG_TYPE_ARC => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let radius = reader.read_bd()?;
            let start_angle = reader.read_bd()?;
            let end_angle = reader.read_bd()?;
            let normal = reader.read_be()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Arc { center, radius, start_angle, end_angle, normal })))
        }
        DWG_TYPE_ELLIPSE => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let major_axis = reader.read_3bd()?;
            let normal = reader.read_be()?;
            let ratio = reader.read_bd()?;
            let start_param = reader.read_bd()?;
            let end_param = reader.read_bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal })))
        }
        DWG_TYPE_TEXT => {
            let color = dwg_decode_entity_common(reader)?;
            let at = reader.read_3bd()?;
            let height = reader.read_bd()?;
            let rotation = reader.read_bd()?;
            let content = reader.read_t()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Text { at, height, rotation, content })))
        }
        DWG_TYPE_FACE3D => {
            let color = dwg_decode_entity_common(reader)?;
            let corners = [reader.read_3bd()?, reader.read_3bd()?, reader.read_3bd()?, reader.read_3bd()?];
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Face3d { corners })))
        }
        DWG_TYPE_LWPOLYLINE => {
            let color = dwg_decode_entity_common(reader)?;
            let closed = reader.read_b()?;
            let elevation = reader.read_bd()?;
            let count = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(count);
            let mut bulges = Vec::with_capacity(count);
            for _ in 0..count {
                vertices.push(reader.read_2rd()?);
                bulges.push(reader.read_bd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::LwPolyline { closed, elevation, vertices, bulges })))
        }
        DWG_TYPE_SPLINE => {
            let color = dwg_decode_entity_common(reader)?;
            let degree = reader.read_bl()?;
            let cp_count = reader.read_bl()? as usize;
            let mut control_points = Vec::with_capacity(cp_count);
            for _ in 0..cp_count {
                control_points.push(reader.read_3bd()?);
            }
            let knot_count = reader.read_bl()? as usize;
            let mut knots = Vec::with_capacity(knot_count);
            for _ in 0..knot_count {
                knots.push(reader.read_rd()?);
            }
            let weight_count = reader.read_bl()? as usize;
            let mut weights = Vec::with_capacity(weight_count);
            for _ in 0..weight_count {
                weights.push(reader.read_rd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Spline { degree, control_points, knots, weights })))
        }
        DWG_TYPE_POLYLINE3D => {
            let color = dwg_decode_entity_common(reader)?;
            let closed = reader.read_b()?;
            let count = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(count);
            for _ in 0..count {
                vertices.push(reader.read_3bd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Polyline3d { closed, vertices })))
        }
        DWG_TYPE_POLYLINE_PFACE => {
            let color = dwg_decode_entity_common(reader)?;
            let vcount = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(vcount);
            for _ in 0..vcount {
                vertices.push(reader.read_3bd()?);
            }
            let fcount = reader.read_bl()? as usize;
            let mut faces = Vec::with_capacity(fcount);
            for _ in 0..fcount {
                let mut face = [0i32; 4];
                for slot in face.iter_mut() {
                    let magnitude = reader.read_bl()? as i32;
                    let negative = reader.read_b()?;
                    *slot = if negative { -magnitude } else { magnitude };
                }
                faces.push(face);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::PolyfaceMesh { vertices, faces })))
        }
        _ => Ok(None),
    }
}
//#endregion DwgObjects

//#region DwgWrite
const DWG_FILE_HEADER_LEN: usize = 55;
const DWG_SENTINEL_HEADER_VARS_BEGIN: [u8; 16] = [0xCF, 0x7B, 0x1F, 0x23, 0xFD, 0xDE, 0x38, 0xA9, 0x5F, 0x7C, 0x68, 0xB8, 0x4E, 0x6D, 0x33, 0x5F];
const DWG_SENTINEL_HEADER_VARS_END: [u8; 16] = [0x30, 0x84, 0xE0, 0xDC, 0x02, 0x21, 0xC7, 0x56, 0xA0, 0x83, 0x97, 0x47, 0xB1, 0x92, 0xCC, 0xA0];
const DWG_SENTINEL_CLASSES_BEGIN: [u8; 16] = [0x8D, 0xA1, 0xC4, 0xB8, 0xC4, 0xA9, 0xF8, 0xC5, 0xC0, 0xDC, 0xF4, 0x5F, 0xE7, 0xCF, 0xB6, 0x8A];
const DWG_SENTINEL_CLASSES_END: [u8; 16] = [0x72, 0x5E, 0x3B, 0x47, 0x3B, 0x56, 0x07, 0x3A, 0x3F, 0x23, 0x0B, 0xA0, 0x18, 0x30, 0x49, 0x75];
const DWG_SENTINEL_FILE_HEADER_END: [u8; 16] = [0x95, 0xA0, 0x4E, 0x28, 0x99, 0x82, 0x1A, 0xE5, 0x5E, 0x41, 0xE0, 0x5F, 0x9D, 0x3A, 0x4D, 0x00];

/// 📐️ Serializes a drawing to a semio DWG (AC1015-flavored) byte stream.
pub fn dwg_to_bytes(drawing: &DwgDrawing) -> Result<Vec<u8>, String> {
    let mut drawing = drawing.clone();
    if drawing.layers.is_empty() {
        drawing.layers.push(DwgLayer::default());
    }
    drawing.recompute_extents();

    let layer_handles: Vec<u64> = (0..drawing.layers.len()).map(|i| HANDLE_LAYER_BASE + i as u64).collect();
    let mut objects_bytes = Vec::new();
    let mut object_map: Vec<(u64, usize)> = Vec::new();

    for (i, layer) in drawing.layers.iter().enumerate() {
        let handle = layer_handles[i];
        let mut body = DwgBitWriter::new();
        body.write_t(&layer.name);
        body.write_rc(layer.color);
        let mut handles = DwgBitWriter::new();
        let offset = objects_bytes.len();
        dwg_write_object(&mut objects_bytes, DWG_TYPE_LAYER, handle, &mut body, &mut handles);
        object_map.push((handle, offset));
    }

    let mut next_handle = HANDLE_ENTITY_BASE;
    for entity in &drawing.entities {
        let layer_handle = layer_handles.get(entity.layer).copied().unwrap_or(layer_handles[0]);
        dwg_encode_entity(&mut objects_bytes, &mut object_map, &mut next_handle, layer_handle, entity);
    }

    let mut header_body = DwgBitWriter::new();
    header_body.write_3rd(drawing.extmin);
    header_body.write_3rd(drawing.extmax);
    header_body.write_handle(0, next_handle);
    header_body.pad_to_byte();
    let header_payload = header_body.bytes;
    let header_crc = dwg_crc16(0xC0C1, &header_payload);

    let mut header_section = Vec::new();
    header_section.extend_from_slice(&DWG_SENTINEL_HEADER_VARS_BEGIN);
    header_section.extend_from_slice(&(header_payload.len() as u32).to_le_bytes());
    header_section.extend_from_slice(&header_payload);
    header_section.extend_from_slice(&header_crc.to_le_bytes());
    header_section.extend_from_slice(&DWG_SENTINEL_HEADER_VARS_END);

    let mut classes_section = Vec::new();
    classes_section.extend_from_slice(&DWG_SENTINEL_CLASSES_BEGIN);
    classes_section.extend_from_slice(&0u32.to_le_bytes());
    classes_section.extend_from_slice(&dwg_crc16(0xC0C1, &[]).to_le_bytes());
    classes_section.extend_from_slice(&DWG_SENTINEL_CLASSES_END);

    let header_vars_offset = DWG_FILE_HEADER_LEN;
    let classes_offset = header_vars_offset + header_section.len();
    let objects_offset = classes_offset + classes_section.len();
    let object_map_offset = objects_offset + objects_bytes.len();

    let mut map_section = Vec::new();
    map_section.extend_from_slice(&(object_map.len() as u32).to_le_bytes());
    for (handle, local_offset) in &object_map {
        map_section.extend_from_slice(&handle.to_le_bytes());
        map_section.extend_from_slice(&((objects_offset + local_offset) as u64).to_le_bytes());
    }
    let map_crc = dwg_crc16(0xC0C1, &map_section);
    map_section.extend_from_slice(&map_crc.to_le_bytes());

    let mut file_header = Vec::new();
    file_header.extend_from_slice(b"AC1015");
    file_header.extend_from_slice(&3u32.to_le_bytes());
    let locators: [(u8, u32, u32); 3] = [
        (0, header_vars_offset as u32, header_section.len() as u32),
        (1, classes_offset as u32, classes_section.len() as u32),
        (2, object_map_offset as u32, map_section.len() as u32),
    ];
    for (num, seeker, size) in locators {
        file_header.push(num);
        file_header.extend_from_slice(&seeker.to_le_bytes());
        file_header.extend_from_slice(&size.to_le_bytes());
    }
    let locator_crc = dwg_crc16(0, &file_header) ^ 0x8461;
    file_header.extend_from_slice(&locator_crc.to_le_bytes());
    file_header.extend_from_slice(&DWG_SENTINEL_FILE_HEADER_END);
    debug_assert_eq!(file_header.len(), DWG_FILE_HEADER_LEN);

    let mut out = Vec::with_capacity(object_map_offset + map_section.len());
    out.extend_from_slice(&file_header);
    out.extend_from_slice(&header_section);
    out.extend_from_slice(&classes_section);
    out.extend_from_slice(&objects_bytes);
    out.extend_from_slice(&map_section);
    Ok(out)
}
//#endregion DwgWrite

//#region DwgRead
/// 📐️ Parses a semio DWG (AC1015-flavored) byte stream, tolerating and skipping unrecognized or malformed objects.
pub fn dwg_from_bytes(bytes: &[u8]) -> Result<DwgDrawing, String> {
    if bytes.len() < 6 || &bytes[0..6] != b"AC1015" {
        let found = String::from_utf8_lossy(bytes.get(0..6).unwrap_or(b"??????")).to_string();
        return Err(format!("unsupported dwg version '{found}': only AC1015 (R2000) is supported"));
    }
    if bytes.len() < DWG_FILE_HEADER_LEN {
        return Err("dwg file header truncated".to_string());
    }
    let section_count = u32::from_le_bytes(bytes[6..10].try_into().unwrap()) as usize;
    let mut cursor = 10usize;
    let mut locators: Vec<(u8, usize, usize)> = Vec::new();
    for _ in 0..section_count.min(16) {
        if cursor + 9 > bytes.len() {
            return Err("dwg section locator truncated".to_string());
        }
        let num = bytes[cursor];
        let seeker = u32::from_le_bytes(bytes[cursor + 1..cursor + 5].try_into().unwrap()) as usize;
        let size = u32::from_le_bytes(bytes[cursor + 5..cursor + 9].try_into().unwrap()) as usize;
        locators.push((num, seeker, size));
        cursor += 9;
    }

    let (_, map_offset, map_size) = *locators
        .iter()
        .find(|(num, _, _)| *num == 2)
        .ok_or_else(|| "dwg missing object map locator".to_string())?;
    if map_offset + map_size > bytes.len() || map_size < 4 {
        return Err("dwg object map out of bounds".to_string());
    }
    let map_bytes = &bytes[map_offset..map_offset + map_size];
    let count = u32::from_le_bytes(map_bytes[0..4].try_into().unwrap()) as usize;
    let mut entries = Vec::with_capacity(count);
    let mut pos = 4usize;
    for _ in 0..count {
        if pos + 16 > map_bytes.len() {
            break;
        }
        let handle = u64::from_le_bytes(map_bytes[pos..pos + 8].try_into().unwrap());
        let address = u64::from_le_bytes(map_bytes[pos + 8..pos + 16].try_into().unwrap()) as usize;
        entries.push((handle, address));
        pos += 16;
    }

    let mut layers: Vec<DwgLayer> = Vec::new();
    let mut layer_handle_index: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut pending_entities: Vec<(u64, DwgColor, DwgGeometry)> = Vec::new();

    for (handle, address) in &entries {
        if *address >= bytes.len() {
            continue;
        }
        let mut sizer = DwgBitReader::new(&bytes[*address..]);
        let payload_len = match sizer.read_ms() {
            Ok(v) => v as usize,
            Err(_) => continue,
        };
        sizer.pad_to_byte();
        let payload_start = address + sizer.byte_pos;
        if payload_start + payload_len > bytes.len() {
            continue;
        }
        let payload = &bytes[payload_start..payload_start + payload_len];
        let mut reader = DwgBitReader::new(payload);
        let object_type = match reader.read_bs() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let _bitsize = match reader.read_rl() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if reader.read_handle().is_err() {
            continue;
        }
        reader.pad_to_byte();

        if object_type == DWG_TYPE_LAYER {
            if let (Ok(name), Ok(color)) = (reader.read_t(), reader.read_rc()) {
                layer_handle_index.insert(*handle, layers.len());
                layers.push(DwgLayer { name, color });
            }
            continue;
        }

        if let Ok(Some((layer_handle, color, geometry))) = dwg_decode_entity(object_type, &mut reader) {
            pending_entities.push((layer_handle, color, geometry));
        }
    }

    if layers.is_empty() {
        layers.push(DwgLayer::default());
    }

    let entities = pending_entities
        .into_iter()
        .map(|(layer_handle, color, geometry)| DwgEntity {
            layer: layer_handle_index.get(&layer_handle).copied().unwrap_or(0),
            color,
            geometry,
        })
        .collect();

    let mut drawing = DwgDrawing { layers, entities, extmin: [0.0; 3], extmax: [0.0; 3] };
    drawing.recompute_extents();
    Ok(drawing)
}
//#endregion DwgRead

//#region DwgBridges
/// 🔺️ Wraps mesh data as a single polyface-mesh drawing.
pub fn mesh_to_dwg_drawing(mesh: &MeshData) -> DwgDrawing {
    let vertices: Vec<[f64; 3]> = mesh.positions.chunks_exact(3).map(|c| [c[0] as f64, c[1] as f64, c[2] as f64]).collect();
    let faces: Vec<[i32; 4]> = mesh
        .indices
        .chunks_exact(3)
        .map(|tri| [tri[0] as i32 + 1, tri[1] as i32 + 1, tri[2] as i32 + 1, tri[2] as i32 + 1])
        .collect();
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices, faces } });
    drawing.recompute_extents();
    drawing
}

/// 🔺️ Collects polyface-mesh and 3dface entities into mesh data.
pub fn dwg_drawing_to_mesh(drawing: &DwgDrawing) -> MeshData {
    let mut mesh = MeshData::default();
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::PolyfaceMesh { vertices, faces } => {
                let base = mesh.vertex_count() as u32;
                for v in vertices {
                    mesh.positions.extend_from_slice(&[v[0] as f32, v[1] as f32, v[2] as f32]);
                }
                for face in faces {
                    let idx: Vec<u32> = face.iter().map(|i| (i.unsigned_abs().saturating_sub(1)) + base).collect();
                    if face[2] == face[3] {
                        mesh.indices.extend_from_slice(&[idx[0], idx[1], idx[2]]);
                    } else {
                        mesh.indices.extend_from_slice(&[idx[0], idx[1], idx[2]]);
                        mesh.indices.extend_from_slice(&[idx[0], idx[2], idx[3]]);
                    }
                }
            }
            DwgGeometry::Face3d { corners } => {
                let base = mesh.vertex_count() as u32;
                for c in corners {
                    mesh.positions.extend_from_slice(&[c[0] as f32, c[1] as f32, c[2] as f32]);
                }
                mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
                if corners[3] != corners[2] {
                    mesh.indices.extend_from_slice(&[base, base + 2, base + 3]);
                }
            }
            _ => {}
        }
    }
    mesh.compute_normals();
    mesh
}

/// ✏️ Path segment mirror of the 2d kernel's PathSegment (kernel/2d/engine/rs/lib.rs), kept dependency-free.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DwgPathSegment {
    Move { to: [f64; 2] },
    Line { to: [f64; 2] },
    Quad { ctrl: [f64; 2], to: [f64; 2] },
    Cubic { ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2] },
    Arc { rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: [f64; 2] },
    Close,
}

fn arc_bulge(from: [f64; 2], to: [f64; 2], radius: f64, sweep: bool) -> f64 {
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let chord = (dx * dx + dy * dy).sqrt();
    if chord < 1e-9 || radius < 1e-9 {
        return 0.0;
    }
    let included_angle = 2.0 * (chord * 0.5 / radius).clamp(-1.0, 1.0).asin();
    let bulge = (included_angle / 4.0).tan();
    if sweep {
        bulge
    } else {
        -bulge
    }
}

fn bulge_to_segment(from: [f64; 2], to: [f64; 2], bulge: f64) -> DwgPathSegment {
    if bulge.abs() < 1e-9 {
        return DwgPathSegment::Line { to };
    }
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let chord = (dx * dx + dy * dy).sqrt();
    let included_angle = 4.0 * bulge.atan();
    let denom = (2.0 * (included_angle / 2.0).sin()).abs();
    let radius = if denom > 1e-9 { chord / denom } else { 0.0 };
    DwgPathSegment::Arc { rx: radius, ry: radius, rotation: 0.0, large_arc: included_angle.abs() > std::f64::consts::PI, sweep: bulge > 0.0, to }
}

/// ✏️ Converts flattened path segments to dwg entities: line/close runs to lwpolylines with bulge arcs, curves to splines.
pub fn paths_to_dwg_drawing(paths: &[Vec<DwgPathSegment>]) -> DwgDrawing {
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    for path in paths {
        let mut vertices: Vec<[f64; 2]> = Vec::new();
        let mut bulges: Vec<f64> = Vec::new();
        let mut closed = false;
        let mut cursor = [0.0, 0.0];
        let mut start = [0.0, 0.0];
        for segment in path {
            match segment {
                DwgPathSegment::Move { to } => {
                    if !vertices.is_empty() {
                        drawing.entities.push(DwgEntity {
                            layer,
                            color: DwgColor::ByLayer,
                            geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices: vertices.clone(), bulges: bulges.clone() },
                        });
                        vertices.clear();
                        bulges.clear();
                        closed = false;
                    }
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                    start = *to;
                }
                DwgPathSegment::Line { to } => {
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                }
                DwgPathSegment::Quad { ctrl, to } => {
                    let c1 = [cursor[0] + 2.0 / 3.0 * (ctrl[0] - cursor[0]), cursor[1] + 2.0 / 3.0 * (ctrl[1] - cursor[1])];
                    let c2 = [to[0] + 2.0 / 3.0 * (ctrl[0] - to[0]), to[1] + 2.0 / 3.0 * (ctrl[1] - to[1])];
                    let spline_points = [cursor, c1, c2, *to];
                    drawing.entities.push(DwgEntity {
                        layer,
                        color: DwgColor::ByLayer,
                        geometry: DwgGeometry::Spline {
                            degree: 3,
                            control_points: spline_points.iter().map(|p| [p[0], p[1], 0.0]).collect(),
                            knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                            weights: vec![1.0; 4],
                        },
                    });
                    cursor = *to;
                }
                DwgPathSegment::Cubic { ctrl1, ctrl2, to } => {
                    let spline_points = [cursor, *ctrl1, *ctrl2, *to];
                    drawing.entities.push(DwgEntity {
                        layer,
                        color: DwgColor::ByLayer,
                        geometry: DwgGeometry::Spline {
                            degree: 3,
                            control_points: spline_points.iter().map(|p| [p[0], p[1], 0.0]).collect(),
                            knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                            weights: vec![1.0; 4],
                        },
                    });
                    cursor = *to;
                }
                DwgPathSegment::Arc { rx, sweep, to, .. } => {
                    let bulge = arc_bulge(cursor, *to, *rx, *sweep);
                    if let Some(last) = bulges.last_mut() {
                        *last = bulge;
                    }
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                }
                DwgPathSegment::Close => {
                    closed = true;
                    cursor = start;
                }
            }
        }
        if !vertices.is_empty() {
            drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices, bulges } });
        }
    }
    drawing.recompute_extents();
    drawing
}

/// ✏️ Converts drawing entities back to path segments, one path per entity.
pub fn dwg_drawing_to_paths(drawing: &DwgDrawing) -> Vec<Vec<DwgPathSegment>> {
    let mut paths = Vec::new();
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::LwPolyline { closed, vertices, bulges, .. } => {
                if vertices.is_empty() {
                    continue;
                }
                let mut segments = vec![DwgPathSegment::Move { to: vertices[0] }];
                for i in 1..vertices.len() {
                    let from = vertices[i - 1];
                    let to = vertices[i];
                    let bulge = bulges.get(i - 1).copied().unwrap_or(0.0);
                    segments.push(bulge_to_segment(from, to, bulge));
                }
                if *closed && vertices.len() > 1 {
                    let bulge = bulges.last().copied().unwrap_or(0.0);
                    segments.push(bulge_to_segment(vertices[vertices.len() - 1], vertices[0], bulge));
                    segments.push(DwgPathSegment::Close);
                }
                paths.push(segments);
            }
            DwgGeometry::Spline { degree, control_points, .. } if *degree == 3 && control_points.len() == 4 => {
                paths.push(vec![
                    DwgPathSegment::Move { to: [control_points[0][0], control_points[0][1]] },
                    DwgPathSegment::Cubic {
                        ctrl1: [control_points[1][0], control_points[1][1]],
                        ctrl2: [control_points[2][0], control_points[2][1]],
                        to: [control_points[3][0], control_points[3][1]],
                    },
                ]);
            }
            DwgGeometry::Circle { center, radius, .. } => {
                paths.push(vec![
                    DwgPathSegment::Move { to: [center[0] + radius, center[1]] },
                    DwgPathSegment::Arc { rx: *radius, ry: *radius, rotation: 0.0, large_arc: true, sweep: true, to: [center[0] - radius, center[1]] },
                    DwgPathSegment::Arc { rx: *radius, ry: *radius, rotation: 0.0, large_arc: true, sweep: true, to: [center[0] + radius, center[1]] },
                    DwgPathSegment::Close,
                ]);
            }
            _ => {}
        }
    }
    paths
}
//#endregion DwgBridges
//#endregion Dwg

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_has_triangles() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        assert_eq!(mesh.triangle_count(), 12);
        assert_eq!(mesh.normals.len(), mesh.positions.len());
    }

    #[test]
    fn glb_round_trip() {
        let mesh = mesh_uv_sphere(1.0, 8, 6);
        let glb = mesh_to_glb(&mesh);
        let decoded = mesh_from_glb(&glb).expect("decode glb");
        assert_eq!(decoded.vertex_count(), mesh.vertex_count());
        assert_eq!(decoded.indices.len(), mesh.indices.len());
    }

    /// 🏙️ Puzzle GLBs may start with non-triangle guide geometry before their renderable surfaces.
    #[test]
    fn glb_import_collects_triangle_primitives_after_guides() {
        let decoded = mesh_from_glb(include_bytes!("../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🧊️capsule_J.glb")).expect("decode Puzzle GLB");
        assert_eq!(decoded.vertex_count(), 1472);
        assert_eq!(decoded.triangle_count(), 1750);
        assert!(decoded.indices.iter().all(|index| (*index as usize) < decoded.vertex_count()));
    }

    #[test]
    fn obj_round_trip() {
        let mesh = mesh_uv_sphere(1.0, 8, 6);
        let obj = mesh_to_obj(&mesh, "sphere");
        let decoded = mesh_from_obj(&obj).expect("decode obj");
        assert_eq!(decoded.vertex_count(), mesh.vertex_count());
        assert_eq!(decoded.indices.len(), mesh.indices.len());
    }

    #[test]
    fn stl_round_trip() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        let stl = mesh_to_stl(&mesh);
        assert_eq!(stl.len(), 80 + 4 + mesh.triangle_count() * 50);
        let decoded = mesh_from_stl(&stl).expect("decode stl");
        assert_eq!(decoded.triangle_count(), mesh.triangle_count());
        assert_eq!(decoded.positions.len(), mesh.triangle_count() * 9);
    }

    /// 🔺️ Small shared-vertex tetrahedron fixture (4 verts, 4 triangles) used by the format round-trip tests below — small enough to assert exact positions/indices, but with enough shared vertices to exercise indexed (not per-face-duplicated) geometry.
    fn tetra_mesh_fixture() -> MeshData {
        let mut mesh = MeshData {
            positions: vec![
                0.0, 0.0, 0.0, // v0
                1.0, 0.0, 0.0, // v1
                0.0, 1.0, 0.0, // v2
                0.0, 0.0, 1.0, // v3
            ],
            indices: vec![0, 1, 2, 0, 1, 3, 0, 2, 3, 1, 2, 3],
            ..MeshData::default()
        };
        mesh.compute_normals();
        mesh
    }

    fn assert_positions_close(a: &[f32], b: &[f32]) {
        assert_eq!(a.len(), b.len(), "position array length mismatch");
        for (x, y) in a.iter().zip(b.iter()) {
            assert!((x - y).abs() < 1e-4, "position mismatch: {x} vs {y}");
        }
    }

    #[test]
    fn obj_round_trip_preserves_positions_and_indices() {
        let mesh = tetra_mesh_fixture();
        let bytes = ObjExporter.export(&mesh).expect("export obj");
        let decoded = ObjImporter.import(&bytes).expect("import obj");
        assert_positions_close(&decoded.positions, &mesh.positions);
        assert_eq!(decoded.indices, mesh.indices);
    }

    #[test]
    fn glb_round_trip_preserves_positions_and_indices() {
        let mesh = tetra_mesh_fixture();
        let bytes = GlbExporter.export(&mesh).expect("export glb");
        let decoded = GlbImporter.import(&bytes).expect("import glb");
        assert_positions_close(&decoded.positions, &mesh.positions);
        assert_eq!(decoded.indices, mesh.indices);
    }

    #[test]
    fn stl_round_trip_preserves_triangle_geometry() {
        let mesh = tetra_mesh_fixture();
        let bytes = StlExporter.export(&mesh).expect("export stl");
        let decoded = StlImporter.import(&bytes).expect("import stl");
        assert_eq!(decoded.triangle_count(), mesh.triangle_count());
        // STL has no vertex sharing, so indices are trivially [0, 1, 2, 3, ...]; compare
        // per-triangle corner positions against the original indexed mesh instead.
        for (triangle, decoded_tri) in mesh.indices.chunks_exact(3).zip(decoded.indices.chunks_exact(3)) {
            for (&original_index, &decoded_index) in triangle.iter().zip(decoded_tri.iter()) {
                let original = &mesh.positions[original_index as usize * 3..original_index as usize * 3 + 3];
                let decoded_position = &decoded.positions[decoded_index as usize * 3..decoded_index as usize * 3 + 3];
                assert_positions_close(decoded_position, original);
            }
        }
    }

    #[test]
    fn mesh_from_indexed_with_face_groups_stamps_per_triangle_face_ids() {
        let positions: Vec<f32> = (0..6 * 3 * 3).map(|i| i as f32).collect();
        let indices: Vec<u32> = (0..18).collect();
        let face_groups = [(101u32, 0u32, 6u32), (202u32, 6u32, 12u32)];
        let mesh = mesh_from_indexed_with_face_groups(&positions, &[], &indices, &face_groups);
        assert_eq!(mesh.face_ids.len(), 6);
        assert_eq!(&mesh.face_ids[0..2], &[101, 101]);
        assert_eq!(&mesh.face_ids[2..6], &[202, 202, 202, 202]);
    }

    #[test]
    fn mesh_from_indexed_with_face_groups_empty_groups_leaves_face_ids_empty() {
        let positions: Vec<f32> = (0..9).map(|i| i as f32).collect();
        let indices: Vec<u32> = vec![0, 1, 2];
        let mesh = mesh_from_indexed_with_face_groups(&positions, &[], &indices, &[]);
        assert!(mesh.face_ids.is_empty());
    }

    #[test]
    fn dwg_bit_primitives_round_trip_at_unaligned_offsets() {
        let mut writer = DwgBitWriter::new();
        writer.write_bit(true);
        writer.write_bit(false);
        writer.write_bit(true);
        writer.write_bs(0);
        writer.write_bs(256);
        writer.write_bs(42);
        writer.write_bs(12345);
        writer.write_bl(0);
        writer.write_bl(200);
        writer.write_bl(70000);
        writer.write_bd(0.0);
        writer.write_bd(1.0);
        writer.write_bd(3.14159);
        writer.write_ms(70000);
        writer.write_handle(5, 0x1234);
        writer.write_t("héllo");
        writer.pad_to_byte();

        let mut reader = DwgBitReader::new(&writer.bytes);
        assert!(reader.read_bit().unwrap());
        assert!(!reader.read_bit().unwrap());
        assert!(reader.read_bit().unwrap());
        assert_eq!(reader.read_bs().unwrap(), 0);
        assert_eq!(reader.read_bs().unwrap(), 256);
        assert_eq!(reader.read_bs().unwrap(), 42);
        assert_eq!(reader.read_bs().unwrap(), 12345);
        assert_eq!(reader.read_bl().unwrap(), 0);
        assert_eq!(reader.read_bl().unwrap(), 200);
        assert_eq!(reader.read_bl().unwrap(), 70000);
        assert_eq!(reader.read_bd().unwrap(), 0.0);
        assert_eq!(reader.read_bd().unwrap(), 1.0);
        assert_eq!(reader.read_bd().unwrap(), 3.14159);
        assert_eq!(reader.read_ms().unwrap(), 70000);
        assert_eq!(reader.read_handle().unwrap(), (5, 0x1234));
        assert_eq!(reader.read_t().unwrap(), "héllo");
    }

    #[test]
    fn dwg_crc16_matches_seed_on_empty_input() {
        assert_eq!(dwg_crc16(0xC0C1, &[]), 0xC0C1);
        assert_ne!(dwg_crc16(0xC0C1, &[1, 2, 3]), 0xC0C1);
    }

    #[test]
    fn dwg_writer_produces_a_structurally_valid_container() {
        let bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode empty drawing");
        assert_eq!(&bytes[0..6], b"AC1015");
        let section_count = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        assert_eq!(section_count, 3);
        assert_eq!(&bytes[DWG_FILE_HEADER_LEN - 16..DWG_FILE_HEADER_LEN], &DWG_SENTINEL_FILE_HEADER_END);
    }

    #[test]
    fn dwg_full_entity_set_round_trips() {
        let mut drawing = DwgDrawing::default();
        let layer_a = drawing.ensure_layer("outline");
        let layer_b = drawing.ensure_layer("solids");
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::Index(3), geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [10.0, 5.0, 0.0] } });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 3.0] } });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByBlock, geometry: DwgGeometry::Circle { center: [0.0, 0.0, 0.0], radius: 5.0, normal: [0.0, 0.0, 1.0] } });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::Index(1),
            geometry: DwgGeometry::Arc { center: [0.0, 0.0, 0.0], radius: 3.0, start_angle: 0.0, end_angle: 1.57, normal: [0.0, 0.0, 1.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::Index(2),
            geometry: DwgGeometry::Ellipse { center: [1.0, 1.0, 0.0], major_axis: [4.0, 0.0, 0.0], ratio: 0.5, start_param: 0.0, end_param: 6.28, normal: [0.0, 0.0, 1.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], bulges: vec![0.0, 0.5, 0.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Spline {
                degree: 3,
                control_points: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [3.0, 2.0, 0.0], [4.0, 0.0, 0.0]],
                knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                weights: vec![1.0; 4],
            },
        });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [0.0, 0.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".to_string() } });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Face3d { corners: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Polyline3d { closed: false, vertices: vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0], [1.0, 0.0, 5.0]] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] },
        });

        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded = dwg_from_bytes(&bytes).expect("decode");

        assert_eq!(decoded.entities.len(), drawing.entities.len());
        assert_eq!(decoded.layers.len(), drawing.layers.len());
        for (original, round_tripped) in drawing.entities.iter().zip(decoded.entities.iter()) {
            assert_eq!(original.geometry, round_tripped.geometry);
            assert_eq!(original.color, round_tripped.color);
            assert_eq!(drawing.layers[original.layer].name, decoded.layers[round_tripped.layer].name);
        }
    }

    #[test]
    fn dwg_mesh_bridge_round_trips_triangle_count_and_positions() {
        let mesh = mesh_box(2.0, 2.0, 2.0);
        let drawing = mesh_to_dwg_drawing(&mesh);
        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded_drawing = dwg_from_bytes(&bytes).expect("decode");
        let decoded_mesh = dwg_drawing_to_mesh(&decoded_drawing);
        assert_eq!(decoded_mesh.triangle_count(), mesh.triangle_count());
        assert_eq!(decoded_mesh.vertex_count(), mesh.vertex_count());
    }

    #[test]
    fn dwg_path_bridge_round_trips_cubic_control_points_exactly() {
        let paths = vec![vec![
            DwgPathSegment::Move { to: [0.0, 0.0] },
            DwgPathSegment::Line { to: [5.0, 0.0] },
            DwgPathSegment::Cubic { ctrl1: [6.0, 1.0], ctrl2: [7.0, 3.0], to: [5.0, 4.0] },
            DwgPathSegment::Close,
        ]];
        let drawing = paths_to_dwg_drawing(&paths);
        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded = dwg_from_bytes(&bytes).expect("decode");
        let round_tripped_paths = dwg_drawing_to_paths(&decoded);

        let cubic_found = round_tripped_paths.iter().flatten().any(|segment| {
            matches!(segment, DwgPathSegment::Cubic { ctrl1, ctrl2, to }
                if (ctrl1[0] - 6.0).abs() < 1e-9 && (ctrl2[1] - 3.0).abs() < 1e-9 && (to[1] - 4.0).abs() < 1e-9)
        });
        assert!(cubic_found, "expected the exact cubic control points to survive the dwg round trip");

        let line_found = round_tripped_paths.iter().flatten().any(|segment| matches!(segment, DwgPathSegment::Line { to } if (to[0] - 5.0).abs() < 1e-9));
        assert!(line_found, "expected the polyline segment to survive the dwg round trip");
    }

    #[test]
    fn dwg_rejects_unsupported_version() {
        let mut bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode");
        bytes[0..6].copy_from_slice(b"AC1018");
        let err = dwg_from_bytes(&bytes).expect_err("should reject non-R2000 version");
        assert!(err.contains("AC1018"));
    }

    #[test]
    fn dwg_reader_skips_unknown_object_types_without_failing() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 1.0, 1.0] } });
        let mut bytes = dwg_to_bytes(&drawing).expect("encode");

        let mut bogus_body = DwgBitWriter::new();
        bogus_body.write_rc(0xFF);
        let mut bogus_handles = DwgBitWriter::new();
        let bogus_offset = bytes.len();
        dwg_write_object(&mut bytes, 900, 0x9999, &mut bogus_body, &mut bogus_handles);

        let map_locator_pos = 10 + 2 * 9;
        let map_offset = u32::from_le_bytes(bytes[map_locator_pos + 1..map_locator_pos + 5].try_into().unwrap());
        let map_size = u32::from_le_bytes(bytes[map_locator_pos + 5..map_locator_pos + 9].try_into().unwrap());
        let mut new_entry = Vec::new();
        new_entry.extend_from_slice(&0x9999u64.to_le_bytes());
        new_entry.extend_from_slice(&(bogus_offset as u64).to_le_bytes());
        let insert_at = map_offset as usize + 4;
        for (i, b) in new_entry.iter().enumerate() {
            bytes.insert(insert_at + i, *b);
        }
        let new_count = u32::from_le_bytes(bytes[map_offset as usize..map_offset as usize + 4].try_into().unwrap()) + 1;
        bytes[map_offset as usize..map_offset as usize + 4].copy_from_slice(&new_count.to_le_bytes());
        let new_size = map_size + new_entry.len() as u32;
        bytes[map_locator_pos + 5..map_locator_pos + 9].copy_from_slice(&new_size.to_le_bytes());

        let decoded = dwg_from_bytes(&bytes).expect("reader should tolerate the unknown object type");
        assert_eq!(decoded.entities.len(), 1);
    }

    #[test]
    fn mesh_from_obj_rejects_malformed_v_and_vn_lines() {
        assert_eq!(mesh_from_obj("v 1.0 2.0\n").unwrap_err(), "obj: malformed v line");
        assert_eq!(mesh_from_obj("v 0 0 0\nvn 1.0\n").unwrap_err(), "obj: malformed vn line");
    }

    #[test]
    fn mesh_from_obj_rejects_malformed_face_index() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf notanumber 2 3\n";
        assert_eq!(mesh_from_obj(text).unwrap_err(), "obj: malformed face index");
    }

    #[test]
    fn mesh_from_obj_zero_and_out_of_range_negative_indices_error() {
        let text_zero = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 0 1 2\n";
        assert_eq!(mesh_from_obj(text_zero).unwrap_err(), "obj: zero vertex index");
        let text_negative = "v 0 0 0\nf -5 1 1\n";
        assert_eq!(mesh_from_obj(text_negative).unwrap_err(), "obj: negative vertex index out of range");
    }

    #[test]
    fn mesh_from_obj_resolves_negative_relative_face_indices() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n";
        let mesh = mesh_from_obj(text).expect("negative indices resolve relative to the current vertex count");
        assert_eq!(mesh.indices, vec![0, 1, 2]);
    }

    #[test]
    fn mesh_from_obj_triangulates_ngon_faces_and_skips_degenerate_faces() {
        let text = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\nf 1 2\n";
        let mesh = mesh_from_obj(text).expect("decode");
        assert_eq!(mesh.triangle_count(), 2, "quad fan-triangulates into 2 triangles; the 2-vertex face is skipped");
    }

    #[test]
    fn mesh_from_stl_rejects_truncated_header_and_truncated_triangle_data() {
        assert_eq!(mesh_from_stl(&[0u8; 10]).unwrap_err(), "stl: truncated header");
        let mut bytes = vec![0u8; 84];
        bytes[80..84].copy_from_slice(&1u32.to_le_bytes());
        assert_eq!(mesh_from_stl(&bytes).unwrap_err(), "stl: truncated triangle data");
    }

    #[test]
    fn mesh_from_glb_rejects_bytes_without_valid_glb_container() {
        assert!(mesh_from_glb(b"not a glb file").is_err());
    }

    #[test]
    fn mesh_from_kind_maps_known_kinds_and_falls_back_to_box() {
        assert_eq!(mesh_from_kind("plane").triangle_count(), mesh_plane(1.0, 1.0).triangle_count());
        assert_eq!(mesh_from_kind("cylinder").triangle_count(), mesh_cylinder(0.5, 1.0, 16).triangle_count());
        assert_eq!(mesh_from_kind("cone").triangle_count(), mesh_cone(0.5, 1.0, 16).triangle_count());
        assert_eq!(mesh_from_kind("torus").triangle_count(), mesh_torus(0.5, 0.15, 16, 12).triangle_count());
        assert_eq!(mesh_from_kind("vortex-marker").triangle_count(), mesh_ico_sphere(0.12, 1).triangle_count());
        assert_eq!(mesh_from_kind("totally-unknown-kind").triangle_count(), mesh_box(1.0, 1.0, 1.0).triangle_count());
    }

    #[test]
    fn mesh_data_aabb_and_merge() {
        let mut mesh = mesh_box(2.0, 4.0, 6.0);
        let (min, max) = mesh.aabb();
        assert!((min[0] - -1.0).abs() < 1e-5 && (max[0] - 1.0).abs() < 1e-5);
        assert!((min[1] - -2.0).abs() < 1e-5 && (max[1] - 2.0).abs() < 1e-5);

        let base_vertex_count = mesh.vertex_count();
        let extra = mesh_plane(1.0, 1.0);
        mesh.merge(&extra);
        assert_eq!(mesh.vertex_count(), base_vertex_count + extra.vertex_count());
        assert_eq!(*mesh.indices.last().unwrap(), (base_vertex_count + extra.vertex_count() - 1) as u32, "merged indices are offset by the base vertex count");
    }

    #[test]
    fn media_types_compatible_covers_direct_any_convert_and_reject() {
        let brep = MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep };
        let mesh_form = MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh };
        let any_3d = MediaType { class: MediaClass::ThreeD, form: MediaForm::Any };
        let vector = MediaType { class: MediaClass::TwoD, form: MediaForm::Vector };
        let raster = MediaType { class: MediaClass::TwoD, form: MediaForm::Raster };
        let text = MediaType { class: MediaClass::Text, form: MediaForm::Document };

        assert_eq!(media_types_compatible(&brep, &brep), MediaCompat::Direct);
        assert_eq!(media_types_compatible(&brep, &any_3d), MediaCompat::Direct, "Any on the accepting side takes anything within the class");
        assert!(matches!(media_types_compatible(&brep, &mesh_form), MediaCompat::Convert { from: MediaForm::Brep, to: MediaForm::Mesh }));
        assert!(matches!(media_types_compatible(&vector, &raster), MediaCompat::Convert { from: MediaForm::Vector, to: MediaForm::Raster }));
        assert_eq!(media_types_compatible(&mesh_form, &brep), MediaCompat::Reject, "mesh->brep has no registered conversion");
        assert_eq!(media_types_compatible(&brep, &text), MediaCompat::Reject, "class mismatch always rejects");
    }

    #[test]
    fn media_fingerprint_structured_hashes_json_binary_reuses_blob_hash() {
        let structured = Media {
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            payload: MediaPayload::Structured { schema: "s".into(), json: "{}".into() },
        };
        let fingerprint = MediaFingerprint::of(&structured);
        assert_eq!(fingerprint, MediaFingerprint::of(&structured), "fingerprint is deterministic");

        let mut changed = structured.clone();
        if let MediaPayload::Structured { json, .. } = &mut changed.payload {
            *json = "{\"a\":1}".into();
        }
        assert_ne!(MediaFingerprint::of(&changed), fingerprint, "different json content hashes differently");

        let binary = Media {
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            payload: MediaPayload::Binary { format: OsMediaFormat::Glb, blob_hash: "abc123".into() },
        };
        assert_eq!(MediaFingerprint::of(&binary), MediaFingerprint("abc123".into()), "binary payload reuses its blob hash verbatim");
    }

    #[test]
    fn os_media_format_str_mime_binary_and_parse_round_trip_all_variants() {
        let all = [
            OsMediaFormat::Svg, OsMediaFormat::Png, OsMediaFormat::Obj, OsMediaFormat::Glb,
            OsMediaFormat::Stl, OsMediaFormat::Step, OsMediaFormat::Dwg, OsMediaFormat::Ply, OsMediaFormat::Las,
        ];
        for format in all {
            assert_eq!(OsMediaFormat::parse(format.as_str()), Some(format));
            assert!(!format.mime_type().is_empty());
        }
        assert!(OsMediaFormat::Glb.is_binary());
        assert!(OsMediaFormat::Png.is_binary());
        assert!(!OsMediaFormat::Obj.is_binary());
        assert!(!OsMediaFormat::Ply.is_binary(), "Ply defaults to the ASCII/text wire encoding");
        assert!(OsMediaFormat::Las.is_binary());
        assert_eq!(OsMediaFormat::parse("bogus"), None);
    }

    #[test]
    fn media_error_messages_are_human_readable() {
        assert_eq!(MediaError::UnknownPort("in".into()).to_string(), "unknown media port `in`");
        let incompatible = MediaError::Incompatible {
            port: "out".into(),
            produced: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
            accepted: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        };
        assert!(incompatible.to_string().starts_with("port `out` produced"));
        assert_eq!(MediaError::Payload("p".into(), "bad".into()).to_string(), "media payload error on port `p`: bad");
        assert_eq!(MediaError::NotImplemented.to_string(), "media ports are not implemented for this app");
    }

    #[test]
    fn dwg_ensure_layer_reuses_existing_index_and_appends_new_ones() {
        let mut drawing = DwgDrawing::default();
        let outline = drawing.ensure_layer("outline");
        let outline_again = drawing.ensure_layer("outline");
        let solids = drawing.ensure_layer("solids");
        assert_eq!(outline, outline_again);
        assert_ne!(outline, solids);
        assert_eq!(drawing.layers.len(), 2);
    }
}
// #endregion mesh
