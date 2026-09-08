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

use pack::json;
// 🔬️ `serde`/`serde_json` survive ONLY as a `#[cfg(test)]` differential oracle (see
// `mesh_data_json_oracle_tests`/`mesh_data_from_value_oracle_tests` below) now that `MeshData` has
// its own first-party `ToValue`/`FromValue` codec — never a production dependency of this crate.
// Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region MeshData
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct MeshData {
    #[cfg_attr(test, serde(default))]
    pub positions: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    pub normals: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    pub colors: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    pub indices: Vec<u32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub uvs: Vec<f32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub face_ids: Vec<u32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub vertex_ids: Vec<u32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub edge_positions: Vec<f32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub edge_ids: Vec<u32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub edge_uvs: Vec<f32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub edge_is_seam: Vec<u8>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub paint_texture_base64: Option<String>,
}

/// 🌉️ `pack::json!` leaf conversion, mirroring this type's own serde attributes exactly: keys are
/// camelCase, `positions`/`normals`/`colors`/`indices` are always emitted, and the remaining fields
/// carry `skip_serializing_if` so they appear only when non-empty/`Some`. Widening `f32` to `f64`
/// matches what `serde_json` itself does for `f32`, so the emitted numbers are unchanged.
/// Needed because the viewer/editor main-window template moved from `serde_json::json!` (which
/// reached this type through `Serialize`) to `pack::json!` (which reaches leaves through `From`).
impl From<MeshData> for json::Value {
    fn from(mesh: MeshData) -> Self {
        fn floats(values: Vec<f32>) -> json::Value {
            json::Value::Array(values.into_iter().map(|v| json::Value::from(v as f64)).collect())
        }
        fn ints(values: Vec<u32>) -> json::Value {
            json::Value::Array(values.into_iter().map(json::Value::from).collect())
        }
        fn bytes(values: Vec<u8>) -> json::Value {
            json::Value::Array(values.into_iter().map(|v| json::Value::from(u32::from(v))).collect())
        }
        let mut object = json::Object::new();
        object.insert("positions", floats(mesh.positions));
        object.insert("normals", floats(mesh.normals));
        object.insert("colors", floats(mesh.colors));
        object.insert("indices", ints(mesh.indices));
        if !mesh.uvs.is_empty() { object.insert("uvs", floats(mesh.uvs)); }
        if !mesh.face_ids.is_empty() { object.insert("faceIds", ints(mesh.face_ids)); }
        if !mesh.vertex_ids.is_empty() { object.insert("vertexIds", ints(mesh.vertex_ids)); }
        if !mesh.edge_positions.is_empty() { object.insert("edgePositions", floats(mesh.edge_positions)); }
        if !mesh.edge_ids.is_empty() { object.insert("edgeIds", ints(mesh.edge_ids)); }
        if !mesh.edge_uvs.is_empty() { object.insert("edgeUvs", floats(mesh.edge_uvs)); }
        if !mesh.edge_is_seam.is_empty() { object.insert("edgeIsSeam", bytes(mesh.edge_is_seam)); }
        if let Some(texture) = mesh.paint_texture_base64 { object.insert("paintTextureBase64", json::Value::from(texture)); }
        json::Value::Object(object)
    }
}

/// 🌱️ First-party value encoding, delegating to the `From<MeshData> for pack::json::Value` impl
/// directly above so the two paths cannot drift — the viewer/editor window templates reach this type
/// through both `pack::json!` (which uses `From`) and `ToValue::to_value`, and they must agree.
impl pack::value::ToValue for MeshData {
    fn to_value(&self) -> pack::value::DslValue {
        json::to_dsl_value(&json::Value::from(self.clone()))
    }
}

/// 🔀️ First-party value decoding — the exact inverse of `ToValue` above, hand-written rather than
/// `#[derive(FromValue)]` because that derive's expansion hardcodes `::semio_framework_os_kernel::…`
/// paths (`🌱️value/✨️derive/🦀️.rs`), and this crate sits BELOW `os-kernel` in the dependency graph
/// (its own doc comment: "consumed only from artifact facet code... or engine-to-engine callers"),
/// so taking that dependency here would invert the layering for a "pure mesh data" leaf crate. Reads
/// the same camelCase object shape `ToValue`/`From<MeshData> for pack::json::Value` emit —
/// `positions`/`normals`/`colors`/`indices` default to empty when absent (mirroring `#[serde(default)]`),
/// every other field defaults to empty/`None` when its key is missing (mirroring
/// `skip_serializing_if`). Indices/counts decode through `u32`'s `FromValue` (accepts `UInt`/`Int`/
/// `Float` DslValue numbers but the encoder only ever emits `UInt` for them — see the sibling
/// `ToValue` impl's `ints` helper), positions/normals/colors/uvs through `f32`'s — never crossed, so
/// a mesh index can never silently decode as a float.
impl pack::value::FromValue for MeshData {
    fn from_value(value: pack::value::DslValue) -> Result<Self, pack::value::ValueError> {
        use pack::value::FromValue;
        let entries = value.into_object()?;
        let field = |key: &str| entries.iter().find(|(k, _)| *k == key).map(|(_, v)| v.clone());
        fn decode_vec<T: FromValue>(field: Option<pack::value::DslValue>, key: &str) -> Result<Vec<T>, pack::value::ValueError> {
            match field {
                Some(value) => Vec::<T>::from_value(value).map_err(|error| error.under(key)),
                None => Ok(Vec::new()),
            }
        }
        Ok(MeshData {
            positions: decode_vec(field("positions"), "positions")?,
            normals: decode_vec(field("normals"), "normals")?,
            colors: decode_vec(field("colors"), "colors")?,
            indices: decode_vec(field("indices"), "indices")?,
            uvs: decode_vec(field("uvs"), "uvs")?,
            face_ids: decode_vec(field("faceIds"), "faceIds")?,
            vertex_ids: decode_vec(field("vertexIds"), "vertexIds")?,
            edge_positions: decode_vec(field("edgePositions"), "edgePositions")?,
            edge_ids: decode_vec(field("edgeIds"), "edgeIds")?,
            edge_uvs: decode_vec(field("edgeUvs"), "edgeUvs")?,
            edge_is_seam: decode_vec(field("edgeIsSeam"), "edgeIsSeam")?,
            paint_texture_base64: match field("paintTextureBase64") {
                None | Some(pack::value::DslValue::Null) => None,
                Some(value) => Some(String::from_value(value).map_err(|error| error.under("paintTextureBase64"))?),
            },
        })
    }
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
        for tri in self.indices.as_chunks::<3>().0 {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p0 = [self.positions[i0 * 3], self.positions[i0 * 3 + 1], self.positions[i0 * 3 + 2]];
            let p1 = [self.positions[i1 * 3], self.positions[i1 * 3 + 1], self.positions[i1 * 3 + 2]];
            let p2 = [self.positions[i2 * 3], self.positions[i2 * 3 + 1], self.positions[i2 * 3 + 2]];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            for &idx in tri {
                let i = idx as usize * 3;
                self.normals[i] += n[0];
                self.normals[i + 1] += n[1];
                self.normals[i + 2] += n[2];
            }
        }
        for chunk in self.normals.as_chunks_mut::<3>().0 {
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
        for chunk in self.positions.as_chunks::<3>().0 {
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
        self.indices.extend(other.indices.iter().map(|index| index + base));
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
    [radius * sin_phi * theta.cos(), radius * phi.cos(), radius * sin_phi * theta.sin()]
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
    let mut faces =
        vec![[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = std::collections::HashMap::new();
        for face in &faces {
            let a = midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[[face[0], a, c], [face[1], b, a], [face[2], c, b], [a, b, c]]);
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

fn midpoint(verts: &mut Vec<[f32; 3]>, cache: &mut std::collections::HashMap<(u32, u32), u32>, a: u32, b: u32) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = normalize3([(verts[a as usize][0] + verts[b as usize][0]) * 0.5, (verts[a as usize][1] + verts[b as usize][1]) * 0.5, (verts[a as usize][2] + verts[b as usize][2]) * 0.5]);
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
    let mut mesh = MeshData { positions: positions.to_vec(), normals: normals.to_vec(), indices: indices.to_vec(), ..MeshData::default() };
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
            for slot in face_ids.iter_mut().take((start_tri + count_tri).min(triangle_count)).skip(start_tri) {
                *slot = face_id;
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
    for chunk in mesh.positions.as_chunks::<3>().0 {
        out.push_str(&format!("v {} {} {}\n", chunk[0], chunk[1], chunk[2]));
    }
    if mesh.normals.len() == mesh.positions.len() {
        for chunk in mesh.normals.as_chunks::<3>().0 {
            out.push_str(&format!("vn {} {} {}\n", chunk[0], chunk[1], chunk[2]));
        }
    }
    for tri in mesh.indices.as_chunks::<3>().0 {
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
    let mut mesh = MeshData { positions, indices, ..MeshData::default() };
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

//#region 🔖️GltfCodec
/// 🧊️ First-party glTF 2.0 container split: `.glb` binary (magic/version/chunk walk) or bare
/// `.gltf` JSON text (detected by a leading `{`) with no binary chunk. Replaces the `gltf` crate
/// per ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS — mirrors the
/// byte-for-byte semantics of the stdio gltf artifact's own `decode_glb`
/// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🦀️.rs`),
/// re-expressed against `pack::json` instead of `serde_json` since this is the framework, not
/// that artifact's own mutation-schema codec.
fn gltf_split_container(bytes: &[u8]) -> Result<(Vec<u8>, Option<Vec<u8>>), String> {
    if bytes.first() == Some(&b'{') {
        return Ok((bytes.to_vec(), None));
    }
    if bytes.len() < 12 || &bytes[0..4] != b"glTF" {
        return Err("glb: bad magic, expected 'glTF' or '{'".into());
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    if version != 2 {
        return Err(format!("glb: unsupported version {version}, only 2 is supported"));
    }
    let total_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let mut pos = 12usize;
    let mut json_chunk: Option<Vec<u8>> = None;
    let mut bin_chunk: Option<Vec<u8>> = None;
    while pos + 8 <= bytes.len() && pos < total_len {
        let chunk_len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        let chunk_type = &bytes[pos + 4..pos + 8];
        let data_start = pos + 8;
        let data_end = data_start + chunk_len;
        if data_end > bytes.len() {
            return Err("glb: chunk length exceeds buffer".into());
        }
        if chunk_type == b"JSON" && json_chunk.is_none() {
            json_chunk = Some(bytes[data_start..data_end].to_vec());
        } else if chunk_type == b"BIN\0" && bin_chunk.is_none() {
            bin_chunk = Some(bytes[data_start..data_end].to_vec());
        }
        pos = data_end;
    }
    Ok((json_chunk.ok_or_else(|| "glb: missing JSON chunk".to_string())?, bin_chunk))
}

/// 🔓️ Decodes a `data:...;base64,...` uri through the framework's own base64 codec — external
/// (file-path) buffer uris are left unresolved (empty bytes), same contract as the stdio gltf
/// artifact's `resolve_document_buffers`: this engine has no filesystem/network access.
fn gltf_decode_data_uri(uri: &str) -> Result<Vec<u8>, String> {
    if !uri.starts_with("data:") {
        return Err("gltf: unsupported external buffer uri (no filesystem access)".into());
    }
    let marker = ";base64,";
    let idx = uri.find(marker).ok_or_else(|| "gltf: unsupported non-base64 data uri".to_string())?;
    semio_framework_io_base64::base64_standard_decode(&uri[idx + marker.len()..]).map_err(|error| error.to_string())
}

/// 📦️ Resolves `document.buffers[i]` to raw bytes, index-aligned with the JSON array. Only
/// `buffers[0]` may omit `uri` and be sourced from the `.glb` BIN chunk, per spec.
fn gltf_resolve_buffers(document: &json::Value, embedded_bin: Option<&[u8]>) -> Vec<Vec<u8>> {
    document
        .get("buffers")
        .and_then(json::Value::as_array)
        .map_or(&[][..], Vec::as_slice)
        .iter()
        .enumerate()
        .map(|(i, buffer)| match buffer.get("uri").and_then(json::Value::as_str) {
            Some(uri) => gltf_decode_data_uri(uri).unwrap_or_default(),
            None if i == 0 => embedded_bin.map(<[u8]>::to_vec).unwrap_or_default(),
            None => Vec::new(),
        })
        .collect()
}

fn gltf_component_byte_size(component_type: u64) -> Result<usize, String> {
    Ok(match component_type {
        5120 | 5121 => 1,
        5122 | 5123 => 2,
        5125 | 5126 => 4,
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    })
}

fn gltf_read_component(component_type: u64, bytes: &[u8], offset: usize) -> Result<f64, String> {
    let size = gltf_component_byte_size(component_type)?;
    if offset + size > bytes.len() {
        return Err("gltf: accessor component read out of buffer bounds".into());
    }
    Ok(match component_type {
        5120 => bytes[offset] as i8 as f64,
        5121 => bytes[offset] as f64,
        5122 => i16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as f64,
        5123 => u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as f64,
        5125 => u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as f64,
        5126 => f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as f64,
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    })
}

fn gltf_accessor_type_components(kind: &str) -> Result<usize, String> {
    Ok(match kind {
        "SCALAR" => 1,
        "VEC2" => 2,
        "VEC3" => 3,
        "VEC4" | "MAT2" => 4,
        "MAT3" => 9,
        "MAT4" => 16,
        other => return Err(format!("gltf: unsupported accessor.type {other:?}")),
    })
}

fn gltf_read_elements(bytes: &[u8], base_offset: usize, component_type: u64, accessor_type: &str, count: usize, byte_stride: Option<usize>) -> Result<Vec<f64>, String> {
    let nc = gltf_accessor_type_components(accessor_type)?;
    let component_size = gltf_component_byte_size(component_type)?;
    let stride = byte_stride.unwrap_or(component_size * nc);
    let mut out = Vec::with_capacity(count * nc);
    for i in 0..count {
        let elem_off = base_offset + i * stride;
        for c in 0..nc {
            out.push(gltf_read_component(component_type, bytes, elem_off + c * component_size)?);
        }
    }
    Ok(out)
}

/// 🎚️ Applies glTF 2.0 accessor normalization (§3.6.2.2) after dense/sparse values assembled.
fn gltf_normalize_components(component_type: u64, components: &mut [f64]) -> Result<(), String> {
    let (scale, signed) = match component_type {
        5120 => (127.0, true),
        5121 => (255.0, false),
        5122 => (32_767.0, true),
        5123 => (65_535.0, false),
        5125 => (4_294_967_295.0, false),
        5126 => return Err("gltf: normalized FLOAT accessor is invalid glTF 2.0".into()),
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    };
    for value in components {
        *value = if signed { (*value / scale).max(-1.0) } else { *value / scale };
    }
    Ok(())
}

fn gltf_read_bufferview_elements(document: &json::Value, buffers: &[Vec<u8>], bv_idx: usize, extra_offset: usize, component_type: u64, accessor_type: &str, count: usize) -> Result<Vec<f64>, String> {
    let bv = document.get("bufferViews").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(bv_idx).ok_or_else(|| format!("gltf: bufferView index {bv_idx} out of range"))?;
    let buffer_index = bv.get("buffer").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let byte_offset = bv.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let byte_stride = bv.get("byteStride").and_then(json::Value::as_u64).map(|value| value as usize);
    let bytes = buffers.get(buffer_index).ok_or_else(|| format!("gltf: buffer index {buffer_index} out of range"))?;
    if bytes.is_empty() {
        return Err(format!("gltf: buffer {buffer_index} bytes unavailable (external uri not resolvable, or empty embedded buffer)"));
    }
    gltf_read_elements(bytes, byte_offset + extra_offset, component_type, accessor_type, count, byte_stride)
}

/// 🧩️ Decodes `document.accessors[accessor_index]` against `buffers` — dense `bufferView` read,
/// then `accessor.sparse` substitution (base is zero-filled when there's no `bufferView`).
fn gltf_decode_accessor(document: &json::Value, buffers: &[Vec<u8>], accessor_index: usize) -> Result<Vec<f64>, String> {
    let accessor = document.get("accessors").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(accessor_index).ok_or_else(|| format!("gltf: accessor index {accessor_index} out of range"))?;
    let component_type = accessor.get("componentType").and_then(json::Value::as_u64).ok_or_else(|| "gltf: accessor missing componentType".to_string())?;
    let accessor_type = accessor.get("type").and_then(json::Value::as_str).ok_or_else(|| "gltf: accessor missing type".to_string())?;
    let count = accessor.get("count").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let normalized = accessor.get("normalized").and_then(json::Value::as_bool).unwrap_or(false);
    let nc = gltf_accessor_type_components(accessor_type)?;

    let mut components = vec![0.0f64; count * nc];
    if let Some(bv_idx) = accessor.get("bufferView").and_then(json::Value::as_u64) {
        let extra_offset = accessor.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        components = gltf_read_bufferview_elements(document, buffers, bv_idx as usize, extra_offset, component_type, accessor_type, count)?;
    }

    if let Some(sparse) = accessor.get("sparse") {
        let sparse_count = sparse.get("count").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        let indices = sparse.get("indices").ok_or_else(|| "gltf: sparse accessor missing indices".to_string())?;
        let values = sparse.get("values").ok_or_else(|| "gltf: sparse accessor missing values".to_string())?;
        let indices_bv = indices.get("bufferView").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse indices missing bufferView".to_string())? as usize;
        let indices_offset = indices.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        let indices_component = indices.get("componentType").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse indices missing componentType".to_string())?;
        let values_bv = values.get("bufferView").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse values missing bufferView".to_string())? as usize;
        let values_offset = values.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;

        let idx_values = gltf_read_bufferview_elements(document, buffers, indices_bv, indices_offset, indices_component, "SCALAR", sparse_count)?;
        let val_values = gltf_read_bufferview_elements(document, buffers, values_bv, values_offset, component_type, accessor_type, sparse_count)?;
        for i in 0..sparse_count {
            let idx = idx_values[i] as usize;
            let dst = idx * nc;
            if dst + nc > components.len() {
                return Err(format!("gltf: sparse accessor index {idx} out of range for count {count}"));
            }
            components[dst..dst + nc].copy_from_slice(&val_values[i * nc..i * nc + nc]);
        }
    }

    if normalized {
        gltf_normalize_components(component_type, &mut components)?;
    }
    Ok(components)
}

fn gltf_node_vec3(node: &json::Value, key: &str, default: [f32; 3]) -> [f32; 3] {
    node.get(key)
        .and_then(json::Value::as_array)
        .filter(|values| values.len() == 3)
        .map_or(default, |values| [values[0].as_f64().unwrap_or(default[0] as f64) as f32, values[1].as_f64().unwrap_or(default[1] as f64) as f32, values[2].as_f64().unwrap_or(default[2] as f64) as f32])
}

fn gltf_node_quat(node: &json::Value) -> [f32; 4] {
    node.get("rotation")
        .and_then(json::Value::as_array)
        .filter(|values| values.len() == 4)
        .map_or([0.0, 0.0, 0.0, 1.0], |values| [values[0].as_f64().unwrap_or(0.0) as f32, values[1].as_f64().unwrap_or(0.0) as f32, values[2].as_f64().unwrap_or(0.0) as f32, values[3].as_f64().unwrap_or(1.0) as f32])
}

/// 🧮️ `T * R * S` node transform per glTF 2.0 §5.25 — quaternion-to-rotation composed with
/// translation/scale, laid out column-major to match this file's `GlbMatrix` convention.
fn gltf_trs_matrix(t: [f32; 3], r: [f32; 4], s: [f32; 3]) -> GlbMatrix {
    let (x, y, z, w) = (r[0], r[1], r[2], r[3]);
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    [
        [(1.0 - (yy + zz)) * s[0], (xy + wz) * s[0], (xz - wy) * s[0], 0.0],
        [(xy - wz) * s[1], (1.0 - (xx + zz)) * s[1], (yz + wx) * s[1], 0.0],
        [(xz + wy) * s[2], (yz - wx) * s[2], (1.0 - (xx + yy)) * s[2], 0.0],
        [t[0], t[1], t[2], 1.0],
    ]
}

/// 🧮️ A node's own local matrix: explicit `matrix` (already column-major, 16 floats) takes
/// precedence over `translation`/`rotation`/`scale` per spec.
fn gltf_node_local_matrix(node: &json::Value) -> GlbMatrix {
    if let Some(m) = node.get("matrix").and_then(json::Value::as_array).filter(|values| values.len() == 16) {
        let f: Vec<f32> = m.iter().map(|value| value.as_f64().unwrap_or(0.0) as f32).collect();
        return [[f[0], f[1], f[2], f[3]], [f[4], f[5], f[6], f[7]], [f[8], f[9], f[10], f[11]], [f[12], f[13], f[14], f[15]]];
    }
    gltf_trs_matrix(gltf_node_vec3(node, "translation", [0.0, 0.0, 0.0]), gltf_node_quat(node), gltf_node_vec3(node, "scale", [1.0, 1.0, 1.0]))
}

fn gltf_triangle_indices(mode: u64, source: Vec<u32>) -> Vec<u32> {
    match mode {
        4 => source,
        5 => source.windows(3).enumerate().flat_map(|(index, tri)| if index % 2 == 0 { [tri[0], tri[1], tri[2]] } else { [tri[1], tri[0], tri[2]] }).collect(),
        6 => source.first().map(|first| source[1..].windows(2).flat_map(|pair| [*first, pair[0], pair[1]]).collect()).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn gltf_append_primitive(mesh: &mut MeshData, document: &json::Value, primitive: &json::Value, buffers: &[Vec<u8>], matrix: GlbMatrix) -> Result<(), String> {
    let mode = primitive.get("mode").and_then(json::Value::as_u64).unwrap_or(4);
    if !matches!(mode, 4..=6) {
        return Ok(());
    }
    let attributes = primitive.get("attributes").ok_or_else(|| "gltf: primitive missing attributes".to_string())?;
    let position_accessor = attributes.get("POSITION").and_then(json::Value::as_u64).ok_or_else(|| "glb triangle primitive missing POSITION".to_string())? as usize;
    let positions: Vec<[f32; 3]> = gltf_decode_accessor(document, buffers, position_accessor)?.as_chunks::<3>().0.iter().map(|c| [c[0] as f32, c[1] as f32, c[2] as f32]).collect();

    let source_indices: Vec<u32> = if let Some(indices_accessor) = primitive.get("indices").and_then(json::Value::as_u64) {
        gltf_decode_accessor(document, buffers, indices_accessor as usize)?.into_iter().map(|value| value as u32).collect()
    } else {
        (0..positions.len() as u32).collect()
    };
    let indices = gltf_triangle_indices(mode, source_indices);
    if indices.iter().any(|index| *index as usize >= positions.len()) {
        return Err("glb triangle index outside POSITION accessor".into());
    }
    let normals: Vec<[f32; 3]> = if let Some(normal_accessor) = attributes.get("NORMAL").and_then(json::Value::as_u64) {
        gltf_decode_accessor(document, buffers, normal_accessor as usize)?.as_chunks::<3>().0.iter().map(|c| [c[0] as f32, c[1] as f32, c[2] as f32]).collect()
    } else {
        let mut local = MeshData { positions: positions.iter().flatten().copied().collect(), indices: indices.clone(), ..Default::default() };
        local.compute_normals();
        local.normals.as_chunks::<3>().0.to_vec()
    };
    if normals.len() != positions.len() {
        return Err("glb NORMAL and POSITION accessor counts differ".into());
    }
    let vertex_offset = mesh.vertex_count() as u32;
    for position in positions {
        mesh.positions.extend(glb_transform_point(matrix, position));
    }
    for normal in normals {
        mesh.normals.extend(glb_transform_normal(matrix, normal));
    }
    mesh.indices.extend(indices.into_iter().map(|index| vertex_offset + index));
    Ok(())
}

fn gltf_append_mesh(mesh: &mut MeshData, document: &json::Value, mesh_index: usize, buffers: &[Vec<u8>], matrix: GlbMatrix) -> Result<(), String> {
    let source = document.get("meshes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(mesh_index).ok_or_else(|| format!("gltf: mesh index {mesh_index} out of range"))?;
    for primitive in source.get("primitives").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
        gltf_append_primitive(mesh, document, primitive, buffers, matrix)?;
    }
    Ok(())
}

fn gltf_append_node(mesh: &mut MeshData, document: &json::Value, node_index: usize, parent: GlbMatrix, buffers: &[Vec<u8>]) -> Result<(), String> {
    let node = document.get("nodes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(node_index).ok_or_else(|| format!("gltf: node index {node_index} out of range"))?;
    let matrix = glb_matrix_mul(parent, gltf_node_local_matrix(node));
    if let Some(mesh_index) = node.get("mesh").and_then(json::Value::as_u64) {
        gltf_append_mesh(mesh, document, mesh_index as usize, buffers, matrix)?;
    }
    for child in node.get("children").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
        if let Some(child_index) = child.as_u64() {
            gltf_append_node(mesh, document, child_index as usize, matrix, buffers)?;
        }
    }
    Ok(())
}

/// 🧊️ Decodes every triangle primitive in the active GLB/glTF scene into one renderer-neutral
/// mesh, via this crate's own glTF 2.0 codec (`pack::json` + `semio_framework_io_base64`) —
/// never the `gltf` crate, which survives only as a `[dev-dependencies]` differential-test oracle.
pub fn mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    let (json_bytes, bin) = gltf_split_container(bytes)?;
    let text = std::str::from_utf8(&json_bytes).map_err(|error| format!("gltf json is not valid utf-8: {error}"))?;
    let document = json::parse(text).map_err(|error| format!("gltf json parse error: {error}"))?;
    let buffers = gltf_resolve_buffers(&document, bin.as_deref());
    let mut mesh = MeshData::default();

    let scene_index = document.get("scene").and_then(json::Value::as_u64).map(|value| value as usize);
    let scenes = document.get("scenes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice);
    let scene = scene_index.and_then(|index| scenes.get(index)).or_else(|| scenes.first());

    if let Some(scene) = scene {
        for node in scene.get("nodes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
            if let Some(node_index) = node.as_u64() {
                gltf_append_node(&mut mesh, &document, node_index as usize, glb_identity(), &buffers)?;
            }
        }
    } else {
        let mesh_count = document.get("meshes").and_then(json::Value::as_array).map_or(0, Vec::len);
        for mesh_index in 0..mesh_count {
            gltf_append_mesh(&mut mesh, &document, mesh_index, &buffers, glb_identity())?;
        }
    }

    if mesh.indices.is_empty() {
        return Err("glb contains no triangle primitives".into());
    }
    Ok(mesh)
}
//#endregion 🔖️GltfCodec

fn f32_slice_to_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn u32_slice_to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while !data.len().is_multiple_of(4) {
        data.push(0);
    }
    data
}

fn json_vec3_min(positions: &[f32]) -> String {
    let (min, _) = MeshData { positions: positions.to_vec(), ..Default::default() }.aabb();
    format!("[{}, {}, {}]", min[0], min[1], min[2])
}

fn json_vec3_max(positions: &[f32]) -> String {
    let (_, max) = MeshData { positions: positions.to_vec(), ..Default::default() }.aabb();
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
    for tri in mesh.indices.as_chunks::<3>().0 {
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
    let n = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
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
    fn format_kind(&self) -> &'static str;
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String>;
}

/// 🔌️ Format-keyed mesh import codec; see `MeshExporter`.
pub trait MeshImporter: Send + Sync {
    fn format_kind(&self) -> &'static str;
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String>;
}

pub struct ObjExporter;
impl MeshExporter for ObjExporter {
    fn format_kind(&self) -> &'static str {
        "obj"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_obj(mesh, "mesh").into_bytes())
    }
}

pub struct ObjImporter;
impl MeshImporter for ObjImporter {
    fn format_kind(&self) -> &'static str {
        "obj"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        mesh_from_obj(text)
    }
}

pub struct GlbExporter;
impl MeshExporter for GlbExporter {
    fn format_kind(&self) -> &'static str {
        "glb"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_glb(mesh))
    }
}

pub struct GlbImporter;
impl MeshImporter for GlbImporter {
    fn format_kind(&self) -> &'static str {
        "glb"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_glb(bytes)
    }
}

pub struct StlExporter;
impl MeshExporter for StlExporter {
    fn format_kind(&self) -> &'static str {
        "stl"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_stl(mesh))
    }
}

pub struct StlImporter;
impl MeshImporter for StlImporter {
    fn format_kind(&self) -> &'static str {
        "stl"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_stl(bytes)
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
// G2) from `🧰️framework/🔨️modules/🔺️mesh/🦀️.rs`'s own `#[cfg(test)] mod tests` — that
// file's DOC COMMENT already said its mesh content "now dissolved into semio-framework-mesh-
// engine" (i.e. HERE), but its 20 tests exercising exactly this crate's own public functions
// (`mesh_box`/`mesh_from_obj`/`ObjExporter`/etc.) were left behind, orphaned, testing a module
// they no longer lived in. This region is that overdue move, landing them with the functions they
// actually exercise. The other 9 tests in that same old `mod tests` block exercised the unrelated
// DWG codec that file also held; those moved to `semio-s-plugin-stdio`'s `ac1024`/`🚪️io` instead.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️GltfOracleDifferential
/// 🔬️ Differential test oracle: decodes the SAME `.glb`/`.gltf` bytes through the third-party
/// `gltf` crate (kept ONLY as a `[dev-dependencies]` reference here — never linked into any
/// production target, per ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
/// ARTIFACTS) and asserts the DECODED STRUCTURE matches this crate's own first-party
/// `mesh_from_glb`. Compares structure, not bytes — glTF exporters are not byte-deterministic.
#[cfg(test)]
#[path = "🧪️tests/🔬️gltf-oracle-differential/🦀️.rs"]
mod gltf_oracle_differential;
//#endregion 🧪️GltfOracleDifferential

//#region 🧪️MeshDataJsonOracleDifferential
/// 🧪️ Validates `From<MeshData> for pack::json::Value` against `serde_json`, the third-party
/// oracle, rather than against a hand-written expectation: both must produce the same JSON for the
/// same mesh. This is what pins the camelCase renaming and the `skip_serializing_if` sparseness —
/// a first-party impl that silently emitted `face_ids`, or emitted `uvs: []` where serde omits the
/// key, would still compile and would still round-trip through our own reader, but would change the
/// bytes the viewer/editor windows put on the wire.
#[cfg(test)]
#[path = "🧪️tests/🔬️mesh-data-json-oracle/🦀️.rs"]
mod mesh_data_json_oracle_tests;
//#endregion 🧪️MeshDataJsonOracleDifferential

//#region 🧪️MeshDataFromValueRoundTrip
/// 🔄️ `FromValue` is the literal inverse of `ToValue`/`From<MeshData> for pack::json::Value` above:
/// `FromValue::from_value(ToValue::to_value(&mesh)) == mesh` for a dense mesh (every always-emitted
/// field only) and for one with every sparse field also populated. A differential oracle test proves
/// the SAME JSON this type's `ToValue` produces decodes through serde_json's own `Deserialize` to
/// the identical `MeshData` our first-party `FromValue` decodes from the equivalent `DslValue` tree
/// — not just that our own encode/decode pair agrees with itself.
#[cfg(test)]
#[path = "🧪️tests/🔬️mesh-data-from-value-round-trip/🦀️.rs"]
mod mesh_data_from_value_round_trip;
//#endregion 🧪️MeshDataFromValueRoundTrip
