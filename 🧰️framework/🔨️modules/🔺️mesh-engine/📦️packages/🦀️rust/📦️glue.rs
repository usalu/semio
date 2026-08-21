//! 🔺️ Pure mesh data, primitive construction, and Obj/Glb/Stl codecs — engine content dissolved
//! out of the framework-module grab-bag it used to share a file with an unrelated DWG codec.
//! Consumed only from artifact facet code (the mesh artifact's own mutation-diff/inference
//! internals) and from engine-to-engine callers such as brep tessellation/mesh-io — never a
//! standalone public surface a plugin app reaches into to bypass the artifact system.

// 🚫️async: R7 — `MeshExporter`/`MeshImporter` are first-party AFIT traits; Send is obtained
// structurally at the concrete-enum call site per R3, never via a `+ Send` bound on the trait
// method, so rustc's `async_fn_in_trait` lint (which would suggest exactly that bound) is
// silenced here rather than resolved by its own suggestion.
#![allow(async_fn_in_trait)]

use serde::{Deserialize, Serialize};

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
    pub async fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub async fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub async fn compute_normals(&mut self) {
        let count = self.vertex_count().await;
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

    pub async fn aabb(&self) -> ([f32; 3], [f32; 3]) {
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

    pub async fn merge(&mut self, other: &MeshData) {
        let base = self.vertex_count().await as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.colors.extend_from_slice(&other.colors);
        self.indices
            .extend(other.indices.iter().map(|index| index + base));
    }
}
//#endregion MeshData

//#region Primitives
async fn push_triangle(mesh: &mut MeshData, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count().await as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

pub async fn mesh_box(width: f32, height: f32, depth: f32) -> MeshData {
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
        push_triangle(&mut mesh, a, b, c).await;
        push_triangle(&mut mesh, a, c, d).await;
    }
    mesh.compute_normals().await;
    mesh
}

pub async fn mesh_plane(width: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]).await;
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]).await;
    mesh.compute_normals().await;
    mesh
}

pub async fn mesh_uv_sphere(radius: f32, segments: u32, rings: u32) -> MeshData {
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
            let p00 = sphere_point(radius, phi0, theta0).await;
            let p10 = sphere_point(radius, phi0, theta1).await;
            let p01 = sphere_point(radius, phi1, theta0).await;
            let p11 = sphere_point(radius, phi1, theta1).await;
            if ring > 0 {
                push_triangle(&mut mesh, p00, p10, p11).await;
            }
            if ring + 1 < rings {
                push_triangle(&mut mesh, p00, p11, p01).await;
            }
        }
    }
    mesh.compute_normals().await;
    mesh
}

async fn sphere_point(radius: f32, phi: f32, theta: f32) -> [f32; 3] {
    let sin_phi = phi.sin();
    [
        radius * sin_phi * theta.cos(),
        radius * phi.cos(),
        radius * sin_phi * theta.sin(),
    ]
}

pub async fn mesh_ico_sphere(radius: f32, subdivisions: u32) -> MeshData {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        normalize3([-1.0, t, 0.0]).await,
        normalize3([1.0, t, 0.0]).await,
        normalize3([-1.0, -t, 0.0]).await,
        normalize3([1.0, -t, 0.0]).await,
        normalize3([0.0, -1.0, t]).await,
        normalize3([0.0, 1.0, t]).await,
        normalize3([0.0, -1.0, -t]).await,
        normalize3([0.0, 1.0, -t]).await,
        normalize3([t, 0.0, -1.0]).await,
        normalize3([t, 0.0, 1.0]).await,
        normalize3([-t, 0.0, -1.0]).await,
        normalize3([-t, 0.0, 1.0]).await,
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
            let a = midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]).await;
            let b = midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]).await;
            let c = midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]).await;
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
        let a = scale3(verts[face[0] as usize], radius).await;
        let b = scale3(verts[face[1] as usize], radius).await;
        let c = scale3(verts[face[2] as usize], radius).await;
        push_triangle(&mut mesh, a, b, c).await;
    }
    mesh.compute_normals().await;
    mesh
}

async fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

async fn scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

async fn midpoint(
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
    verts.push(mid.await);
    cache.insert(key, index);
    index
}

pub async fn mesh_cylinder(radius: f32, height: f32, segments: u32) -> MeshData {
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
        push_triangle(&mut mesh, p00, p01, p11).await;
        push_triangle(&mut mesh, p00, p11, p10).await;
        push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00).await;
        push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11).await;
    }
    mesh.compute_normals().await;
    mesh
}

pub async fn mesh_cone(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        push_triangle(&mut mesh, apex, p1, p0).await;
        push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1).await;
    }
    mesh.compute_normals().await;
    mesh
}

pub async fn mesh_torus(major_radius: f32, minor_radius: f32, segments: u32, rings: u32) -> MeshData {
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
            let p00 = torus_point(major_radius, minor_radius, phi0, theta0).await;
            let p10 = torus_point(major_radius, minor_radius, phi0, theta1).await;
            let p01 = torus_point(major_radius, minor_radius, phi1, theta0).await;
            let p11 = torus_point(major_radius, minor_radius, phi1, theta1).await;
            push_triangle(&mut mesh, p00, p10, p11).await;
            push_triangle(&mut mesh, p00, p11, p01).await;
        }
    }
    mesh.compute_normals().await;
    mesh
}

async fn torus_point(major: f32, minor: f32, phi: f32, theta: f32) -> [f32; 3] {
    let r = major + minor * theta.cos();
    [r * phi.cos(), minor * theta.sin(), r * phi.sin()]
}

pub async fn mesh_from_kind(kind: &str) -> MeshData {
    match kind {
        "vortex-marker" => mesh_ico_sphere(0.12, 1).await,
        "vertex-marker" => mesh_ico_sphere(1.0, 1).await,
        "sphere" | "uvSphere" => mesh_uv_sphere(0.5, 16, 12).await,
        "icoSphere" => mesh_ico_sphere(0.5, 1).await,
        "plane" => mesh_plane(1.0, 1.0).await,
        "cylinder" => mesh_cylinder(0.5, 1.0, 16).await,
        "cone" => mesh_cone(0.5, 1.0, 16).await,
        "torus" => mesh_torus(0.5, 0.15, 16, 12).await,
        _ => mesh_box(1.0, 1.0, 1.0).await,
    }
}

/** @emoji 🔩️ Builds mesh data from indexed brep tessellation buffers. */
pub async fn mesh_from_indexed(positions: &[f32], normals: &[f32], indices: &[u32]) -> MeshData {
    let mut mesh = MeshData {
        positions: positions.to_vec(),
        normals: normals.to_vec(),
        indices: indices.to_vec(),
        ..MeshData::default()
    };
    if mesh.normals.is_empty() && !mesh.positions.is_empty() {
        mesh.compute_normals().await;
    }
    mesh
}

/** @emoji 🧩️ Like `mesh_from_indexed`, but also stamps `face_ids` per triangle from `(face id, triangle start, triangle count)`
 * groups — lets a picked triangle resolve back to the brep face it came from. Plain tuples (not the kernel's `FaceGroup`)
 * so this crate doesn't need to depend on the kernel engine crate; callers convert their own group type. */
pub async fn mesh_from_indexed_with_face_groups(positions: &[f32], normals: &[f32], indices: &[u32], face_groups: &[(u32, u32, u32)]) -> MeshData {
    let mut mesh = mesh_from_indexed(positions, normals, indices).await;
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
pub async fn mesh_to_obj(mesh: &MeshData, object_name: &str) -> String {
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
pub async fn mesh_from_obj(text: &str) -> Result<MeshData, String> {
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
                    face.push(obj_resolve_index(raw, vertex_count).await?);
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
        mesh.compute_normals().await;
    }
    Ok(mesh)
}

async fn obj_resolve_index(raw: i64, count: usize) -> Result<usize, String> {
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
pub async fn mesh_to_glb(mesh: &MeshData) -> Vec<u8> {
    let positions = f32_slice_to_bytes(&mesh.positions).await;
    let normals = if mesh.normals.len() == mesh.positions.len() {
        f32_slice_to_bytes(&mesh.normals).await
    } else {
        let mut copy = mesh.clone();
        copy.compute_normals().await;
        f32_slice_to_bytes(&copy.normals).await
    };
    let indices = u32_slice_to_bytes(&mesh.indices).await;
    let bin = [positions.as_slice(), normals.as_slice(), indices.as_slice()].concat();
    let padded_bin = pad_to_4(bin).await;
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
        mesh.vertex_count().await,
        json_vec3_min(&mesh.positions).await,
        json_vec3_max(&mesh.positions).await,
        mesh.vertex_count().await,
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

async fn glb_identity() -> GlbMatrix {
    [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

async fn glb_matrix_mul(left: GlbMatrix, right: GlbMatrix) -> GlbMatrix {
    let mut result = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            result[column][row] = (0..4).map(|axis| left[axis][row] * right[column][axis]).sum();
        }
    }
    result
}

async fn glb_transform_point(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * point[0] + matrix[1][0] * point[1] + matrix[2][0] * point[2] + matrix[3][0],
        matrix[0][1] * point[0] + matrix[1][1] * point[1] + matrix[2][1] * point[2] + matrix[3][1],
        matrix[0][2] * point[0] + matrix[1][2] * point[1] + matrix[2][2] * point[2] + matrix[3][2],
    ]
}

async fn glb_transform_normal(matrix: GlbMatrix, normal: [f32; 3]) -> [f32; 3] {
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

async fn glb_triangle_indices(mode: gltf::mesh::Mode, source: Vec<u32>) -> Vec<u32> {
    match mode {
        gltf::mesh::Mode::Triangles => source,
        gltf::mesh::Mode::TriangleStrip => source.windows(3).enumerate().flat_map(|(index, tri)| if index % 2 == 0 { [tri[0], tri[1], tri[2]] } else { [tri[1], tri[0], tri[2]] }).collect(),
        gltf::mesh::Mode::TriangleFan => source.first().map(|first| source[1..].windows(2).flat_map(|pair| [*first, pair[0], pair[1]]).collect()).unwrap_or_default(),
        _ => Vec::new(),
    }
}

async fn append_glb_primitive(mesh: &mut MeshData, primitive: gltf::Primitive<'_>, matrix: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    if !matches!(primitive.mode(), gltf::mesh::Mode::Triangles | gltf::mesh::Mode::TriangleStrip | gltf::mesh::Mode::TriangleFan) {
        return Ok(());
    }
    let reader = primitive.reader(|buffer| (buffer.index() == 0).then_some(bin));
    let positions: Vec<[f32; 3]> = reader.read_positions().ok_or_else(|| "glb triangle primitive missing POSITION".to_string())?.collect();
    let source_indices: Vec<u32> = reader.read_indices().map(|indices| indices.into_u32().collect()).unwrap_or_else(|| (0..positions.len() as u32).collect());
    let indices = glb_triangle_indices(primitive.mode(), source_indices).await;
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
        local.compute_normals().await;
        local.normals.as_chunks::<3>().0.to_vec()
    };
    if normals.len() != positions.len() {
        return Err("glb NORMAL and POSITION accessor counts differ".into());
    }
    let vertex_offset = mesh.vertex_count().await as u32;
    for position in positions {
        mesh.positions.extend(glb_transform_point(matrix, position).await);
    }
    for normal in normals {
        mesh.normals.extend(glb_transform_normal(matrix, normal).await);
    }
    mesh.indices.extend(indices.into_iter().map(|index| vertex_offset + index));
    Ok(())
}

async fn append_glb_mesh(mesh: &mut MeshData, source: gltf::Mesh<'_>, matrix: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    for primitive in source.primitives() {
        append_glb_primitive(mesh, primitive, matrix, bin).await?;
    }
    Ok(())
}

async fn append_glb_node(mesh: &mut MeshData, node: gltf::Node<'_>, parent: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    let matrix = glb_matrix_mul(parent, node.transform().matrix()).await;
    if let Some(source) = node.mesh() {
        append_glb_mesh(mesh, source, matrix, bin).await?;
    }
    for child in node.children() {
        Box::pin(append_glb_node(mesh, child, matrix, bin)).await?;
    }
    Ok(())
}

/// 🧊️ Decodes every triangle primitive in the active GLB scene into one renderer-neutral mesh.
pub async fn mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    let gltf = gltf::Gltf::from_slice(bytes).map_err(|error| error.to_string())?;
    let bin = gltf.blob.as_deref().ok_or_else(|| "glb missing BIN chunk".to_string())?;
    let mut mesh = MeshData::default();
    if let Some(scene) = gltf.default_scene().or_else(|| gltf.scenes().next()) {
        for node in scene.nodes() {
            append_glb_node(&mut mesh, node, glb_identity().await, bin).await?;
        }
    } else {
        for source in gltf.meshes() {
            append_glb_mesh(&mut mesh, source, glb_identity().await, bin).await?;
        }
    }
    if mesh.indices.is_empty() {
        return Err("glb contains no triangle primitives".into());
    }
    Ok(mesh)
}

async fn f32_slice_to_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

async fn u32_slice_to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

async fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while data.len() % 4 != 0 {
        data.push(0);
    }
    data
}

async fn json_vec3_min(positions: &[f32]) -> String {
    let (min, _) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb().await;
    format!("[{}, {}, {}]", min[0], min[1], min[2])
}

async fn json_vec3_max(positions: &[f32]) -> String {
    let (_, max) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb().await;
    format!("[{}, {}, {}]", max[0], max[1], max[2])
}
//#endregion Glb

//#region Stl
/// 🧱️ Hand-rolled binary STL: 80-byte header, `u32` little-endian triangle count, then per triangle a `f32x3` facet normal, three `f32x3` vertices, and a `u16` attribute-byte-count (written as 0). No vertex dedupe, matching the binary STL convention of one independent triangle per record.
pub async fn mesh_to_stl(mesh: &MeshData) -> Vec<u8> {
    let triangle_count = mesh.triangle_count().await as u32;
    let mut out = Vec::with_capacity(80 + 4 + triangle_count as usize * 50);
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&triangle_count.to_le_bytes());
    for tri in mesh.indices.chunks_exact(3) {
        let p0 = stl_vertex(&mesh.positions, tri[0]).await;
        let p1 = stl_vertex(&mesh.positions, tri[1]).await;
        let p2 = stl_vertex(&mesh.positions, tri[2]).await;
        let normal = stl_face_normal(p0, p1, p2).await;
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

pub async fn mesh_from_stl(bytes: &[u8]) -> Result<MeshData, String> {
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

async fn stl_vertex(positions: &[f32], index: u32) -> [f32; 3] {
    let base = index as usize * 3;
    [positions[base], positions[base + 1], positions[base + 2]]
}

async fn stl_face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
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

//#region MeshCodec
/// 🔌️ Format-keyed mesh export codec; concrete implementations below are zero-dependency
/// (hand-rolled OBJ/GLB/STL). B-Rep apps additionally get `SolidExporter` (kernel/3d/brep/rs) which
/// wraps the real kernel's STEP/STL/OBJ writers, and reuse `GlbExporter`/`GlbImporter` here via a
/// tessellation bridge so GLB is the same codec everywhere. `format_kind` is the short stdio format
/// kind id (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
pub trait MeshExporter: Send + Sync {
    async fn format_kind(&self) -> &'static str;
    async fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String>;
}

/// 🔌️ Format-keyed mesh import codec; see `MeshExporter`.
pub trait MeshImporter: Send + Sync {
    async fn format_kind(&self) -> &'static str;
    async fn import(&self, bytes: &[u8]) -> Result<MeshData, String>;
}

pub struct ObjExporter;
impl MeshExporter for ObjExporter {
    async fn format_kind(&self) -> &'static str {
        "obj"
    }
    async fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_obj(mesh, "mesh").await.into_bytes())
    }
}

pub struct ObjImporter;
impl MeshImporter for ObjImporter {
    async fn format_kind(&self) -> &'static str {
        "obj"
    }
    async fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        mesh_from_obj(text).await
    }
}

pub struct GlbExporter;
impl MeshExporter for GlbExporter {
    async fn format_kind(&self) -> &'static str {
        "glb"
    }
    async fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_glb(mesh).await)
    }
}

pub struct GlbImporter;
impl MeshImporter for GlbImporter {
    async fn format_kind(&self) -> &'static str {
        "glb"
    }
    async fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_glb(bytes).await
    }
}

pub struct StlExporter;
impl MeshExporter for StlExporter {
    async fn format_kind(&self) -> &'static str {
        "stl"
    }
    async fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_stl(mesh).await)
    }
}

pub struct StlImporter;
impl MeshImporter for StlImporter {
    async fn format_kind(&self) -> &'static str {
        "stl"
    }
    async fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_stl(bytes).await
    }
}
//#endregion MeshCodec

//#region IoError
/// ⚠️ Media IO error shared by ArtifactImport/Export and framework codecs. `Unsupported` carries the
/// unsupported format's kind id string (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
#[derive(Clone, Debug, PartialEq)]
pub enum IoError {
    Format(String),
    Unsupported(String),
    Payload(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Format(m) => write!(f, "format: {m}"),
            Self::Unsupported(fmt) => write!(f, "unsupported: {fmt}"),
            Self::Payload(m) => write!(f, "payload: {m}"),
        }
    }
}

impl std::error::Error for IoError {}
//#endregion IoError

//#region 🧪️Tests
// 🧪 Relocated verbatim (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
// G2) from `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`'s own `#[cfg(test)] mod tests` — that
// file's DOC COMMENT already said its mesh content "now dissolved into semio-framework-mesh-
// engine" (i.e. HERE), but its 20 tests exercising exactly this crate's own public functions
// (`mesh_box`/`mesh_from_obj`/`ObjExporter`/etc.) were left behind, orphaned, testing a module
// they no longer lived in. This region is that overdue move, landing them with the functions they
// actually exercise. The other 9 tests in that same old `mod tests` block exercised the unrelated
// DWG codec that file also held; those moved to `semio-s-plugin-stdio`'s `ac1024`/`🚪️io` instead.
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn box_has_triangles() {
        let mesh = mesh_box(1.0, 1.0, 1.0).await;
        assert_eq!(mesh.triangle_count().await, 12);
        assert_eq!(mesh.normals.len(), mesh.positions.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn glb_round_trip() {
        let mesh = mesh_uv_sphere(1.0, 8, 6).await;
        let glb = mesh_to_glb(&mesh).await;
        let decoded = mesh_from_glb(&glb).await.expect("decode glb");
        assert_eq!(decoded.vertex_count().await, mesh.vertex_count().await);
        assert_eq!(decoded.indices.len(), mesh.indices.len());
    }

    /// 🏙️ Puzzle GLBs may start with non-triangle guide geometry before their renderable surfaces.
    #[semio_framework_async_macros::async_test]
    async fn glb_import_collects_triangle_primitives_after_guides() {
        let decoded = mesh_from_glb(include_bytes!("../../../🖼️assets/🌱️metabolism/🎨️representation/🧊️capsule_J.glb")).await.expect("decode Puzzle GLB");
        assert_eq!(decoded.vertex_count().await, 1472);
        assert_eq!(decoded.triangle_count().await, 1750);
        assert!(decoded.indices.iter().all(|index| (*index as usize) < 1472));
    }

    #[semio_framework_async_macros::async_test]
    async fn obj_round_trip() {
        let mesh = mesh_uv_sphere(1.0, 8, 6).await;
        let obj = mesh_to_obj(&mesh, "sphere");
        let decoded = mesh_from_obj(&obj).await.expect("decode obj");
        assert_eq!(decoded.vertex_count().await, mesh.vertex_count().await);
        assert_eq!(decoded.indices.len(), mesh.indices.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn stl_round_trip() {
        let mesh = mesh_box(1.0, 1.0, 1.0).await;
        let stl = mesh_to_stl(&mesh).await;
        assert_eq!(stl.len(), 80 + 4 + mesh.triangle_count().await * 50);
        let decoded = mesh_from_stl(&stl).expect("decode stl");
        assert_eq!(decoded.triangle_count().await, mesh.triangle_count().await);
        assert_eq!(decoded.positions.len(), mesh.triangle_count().await * 9);
    }

    /// 🔺️ Small shared-vertex tetrahedron fixture (4 verts, 4 triangles) used by the format round-trip tests below — small enough to assert exact positions/indices, but with enough shared vertices to exercise indexed (not per-face-duplicated) geometry.
    async fn tetra_mesh_fixture() -> MeshData {
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
        mesh.compute_normals().await;
        mesh
    }

    fn assert_positions_close(a: &[f32], b: &[f32]) {
        assert_eq!(a.len(), b.len(), "position array length mismatch");
        for (x, y) in a.iter().zip(b.iter()) {
            assert!((x - y).abs() < 1e-4, "position mismatch: {x} vs {y}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn obj_round_trip_preserves_positions_and_indices() {
        let mesh = tetra_mesh_fixture().await;
        let bytes = ObjExporter.export(&mesh).await.expect("export obj");
        let decoded = ObjImporter.import(&bytes).await.expect("import obj");
        assert_positions_close(&decoded.positions, &mesh.positions);
        assert_eq!(decoded.indices, mesh.indices);
    }

    #[semio_framework_async_macros::async_test]
    async fn glb_round_trip_preserves_positions_and_indices() {
        let mesh = tetra_mesh_fixture().await;
        let bytes = GlbExporter.export(&mesh).await.expect("export glb");
        let decoded = GlbImporter.import(&bytes).await.expect("import glb");
        assert_positions_close(&decoded.positions, &mesh.positions);
        assert_eq!(decoded.indices, mesh.indices);
    }

    #[semio_framework_async_macros::async_test]
    async fn stl_round_trip_preserves_triangle_geometry() {
        let mesh = tetra_mesh_fixture().await;
        let bytes = StlExporter.export(&mesh).await.expect("export stl");
        let decoded = StlImporter.import(&bytes).await.expect("import stl");
        assert_eq!(decoded.triangle_count().await, mesh.triangle_count().await);
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

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_indexed_with_face_groups_stamps_per_triangle_face_ids() {
        let positions: Vec<f32> = (0..6 * 3 * 3).map(|i| i as f32).collect();
        let indices: Vec<u32> = (0..18).collect();
        let face_groups = [(101u32, 0u32, 6u32), (202u32, 6u32, 12u32)];
        let mesh = mesh_from_indexed_with_face_groups(&positions, &[], &indices, &face_groups).await;
        assert_eq!(mesh.face_ids.len(), 6);
        assert_eq!(&mesh.face_ids[0..2], &[101, 101]);
        assert_eq!(&mesh.face_ids[2..6], &[202, 202, 202, 202]);
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_indexed_with_face_groups_empty_groups_leaves_face_ids_empty() {
        let positions: Vec<f32> = (0..9).map(|i| i as f32).collect();
        let indices: Vec<u32> = vec![0, 1, 2];
        let mesh = mesh_from_indexed_with_face_groups(&positions, &[], &indices, &[]).await;
        assert!(mesh.face_ids.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_obj_rejects_malformed_v_and_vn_lines() {
        assert_eq!(mesh_from_obj("v 1.0 2.0\n").await.unwrap_err(), "obj: malformed v line");
        assert_eq!(mesh_from_obj("v 0 0 0\nvn 1.0\n").await.unwrap_err(), "obj: malformed vn line");
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_obj_rejects_malformed_face_index() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf notanumber 2 3\n";
        assert_eq!(mesh_from_obj(text).await.unwrap_err(), "obj: malformed face index");
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_obj_zero_and_out_of_range_negative_indices_error() {
        let text_zero = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 0 1 2\n";
        assert_eq!(mesh_from_obj(text_zero).await.unwrap_err(), "obj: zero vertex index");
        let text_negative = "v 0 0 0\nf -5 1 1\n";
        assert_eq!(mesh_from_obj(text_negative).await.unwrap_err(), "obj: negative vertex index out of range");
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_obj_resolves_negative_relative_face_indices() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n";
        let mesh = mesh_from_obj(text).await.expect("negative indices resolve relative to the current vertex count");
        assert_eq!(mesh.indices, vec![0, 1, 2]);
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_obj_triangulates_ngon_faces_and_skips_degenerate_faces() {
        let text = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\nf 1 2\n";
        let mesh = mesh_from_obj(text).await.expect("decode");
        assert_eq!(mesh.triangle_count().await, 2, "quad fan-triangulates into 2 triangles; the 2-vertex face is skipped");
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_stl_rejects_truncated_header_and_truncated_triangle_data() {
        assert_eq!(mesh_from_stl(&[0u8; 10]).unwrap_err(), "stl: truncated header");
        let mut bytes = vec![0u8; 84];
        bytes[80..84].copy_from_slice(&1u32.to_le_bytes());
        assert_eq!(mesh_from_stl(&bytes).unwrap_err(), "stl: truncated triangle data");
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_glb_rejects_bytes_without_valid_glb_container() {
        assert!(mesh_from_glb(b"not a glb file").await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_from_kind_maps_known_kinds_and_falls_back_to_box() {
        assert_eq!(mesh_from_kind("plane").await.triangle_count().await, mesh_plane(1.0, 1.0).await.triangle_count().await);
        assert_eq!(mesh_from_kind("cylinder").await.triangle_count().await, mesh_cylinder(0.5, 1.0, 16).await.triangle_count().await);
        assert_eq!(mesh_from_kind("cone").await.triangle_count().await, mesh_cone(0.5, 1.0, 16).await.triangle_count().await);
        assert_eq!(mesh_from_kind("torus").await.triangle_count().await, mesh_torus(0.5, 0.15, 16, 12).await.triangle_count().await);
        assert_eq!(mesh_from_kind("vortex-marker").await.triangle_count().await, mesh_ico_sphere(0.12, 1).await.triangle_count().await);
        assert_eq!(mesh_from_kind("totally-unknown-kind").await.triangle_count().await, mesh_box(1.0, 1.0, 1.0).await.triangle_count().await);
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_data_aabb_and_merge() {
        let mut mesh = mesh_box(2.0, 4.0, 6.0).await;
        let (min, max) = mesh.aabb().await;
        assert!((min[0] - -1.0).abs() < 1e-5 && (max[0] - 1.0).abs() < 1e-5);
        assert!((min[1] - -2.0).abs() < 1e-5 && (max[1] - 2.0).abs() < 1e-5);

        let base_vertex_count = mesh.vertex_count().await;
        let extra = mesh_plane(1.0, 1.0).await;
        let extra_vertex_count = extra.vertex_count().await;
        mesh.merge(&extra).await;
        assert_eq!(mesh.vertex_count().await, base_vertex_count + extra_vertex_count);
        assert_eq!(*mesh.indices.last().unwrap(), (base_vertex_count + extra_vertex_count - 1) as u32, "merged indices are offset by the base vertex count");
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_exporter_and_importer_use_short_format_kind_ids_not_media_format() {
        assert_eq!(ObjExporter.format_kind().await, "obj");
        assert_eq!(ObjImporter.format_kind().await, "obj");
        assert_eq!(GlbExporter.format_kind().await, "glb");
        assert_eq!(GlbImporter.format_kind().await, "glb");
        assert_eq!(StlExporter.format_kind().await, "stl");
        assert_eq!(StlImporter.format_kind().await, "stl");
    }
}
//#endregion 🧪️Tests
