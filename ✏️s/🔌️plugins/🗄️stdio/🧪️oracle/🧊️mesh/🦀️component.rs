//! 🧊️ Mesh oracles: Wavefront OBJ and STL creation, mutation and projection.
//!
//! The `semantic-mesh-v1` profile compares the vertex set, the face topology and the counts.
//! Generator strings, comments, coordinate precision beyond the declared tolerance and solid naming
//! are writer freedom.
//!
//! @see ../🔣️component.json — the contribution manifest that registers these oracles.

use semio_repo_test_host::Json;

//#region 🔖️MeshSpec
/// 🧊️ Owned description of a triangle mesh — the one input both producers are given.
#[derive(Debug, Clone)]
pub struct MeshSpec {
    pub vertices: Vec<[f32; 3]>,
    /// 🔺️ Zero-based vertex indices, three per triangle.
    pub triangles: Vec<[usize; 3]>,
}

impl MeshSpec {
    /// 🔺️ A deterministic unit tetrahedron — small, closed, and exercises every face.
    pub fn tetrahedron() -> MeshSpec {
        MeshSpec {
            vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            triangles: vec![[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]],
        }
    }

    /// 🔺️ A deterministic axis-aligned quad split into two triangles.
    pub fn quad() -> MeshSpec {
        MeshSpec { vertices: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 3.0, 0.0], [0.0, 3.0, 0.0]], triangles: vec![[0, 1, 2], [0, 2, 3]] }
    }

    /// 🧊️ Reads a spec out of a scenario's owned JSON payload.
    pub fn from_json(value: &Json) -> MeshSpec {
        match value.str("shape").as_str() {
            "quad" => MeshSpec::quad(),
            _ => MeshSpec::tetrahedron(),
        }
    }

    /// 🔁️ The projection every mesh producer is compared through.
    pub fn projection(&self, format: &str) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String(format.to_string())),
            ("vertexCount".to_string(), Json::Number(self.vertices.len() as f64)),
            ("triangleCount".to_string(), Json::Number(self.triangles.len() as f64)),
            ("vertices".to_string(), Json::Array(self.vertices.iter().map(|vertex| Json::Array(vertex.iter().map(|value| Json::Number(*value as f64)).collect())).collect())),
            ("triangles".to_string(), Json::Array(self.triangles.iter().map(|face| Json::Array(face.iter().map(|index| Json::Number(*index as f64)).collect())).collect())),
        ])
    }

    /// 🔁️ The projection an STL producer is compared through. STL carries no vertex INDEX — every
    /// triangle repeats its three corners — so the shared shape would be a lie; this one reports the
    /// resolved corner positions per triangle instead.
    pub fn triangle_soup_projection(&self, format: &str) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String(format.to_string())),
            ("triangleCount".to_string(), Json::Number(self.triangles.len() as f64)),
            (
                "triangles".to_string(),
                Json::Array(
                    self.triangles
                        .iter()
                        .map(|face| Json::Array(face.iter().map(|index| Json::Array(self.vertices[*index].iter().map(|value| Json::Number(*value as f64)).collect())).collect()))
                        .collect(),
                ),
            ),
        ])
    }
}
//#endregion 🔖️MeshSpec

//#region 🔖️Obj
/// 🔮️ Writes a Wavefront OBJ. The format is a plain text grammar with no reference WRITER in the
/// Rust ecosystem — `tobj` is the reference READER — so the oracle writes the grammar directly and
/// the independent reader is what makes the comparison meaningful.
/// @see https://github.com/Twinklebear/tobj
#[cfg(feature = "oracles")]
pub fn oracle_create_obj(spec: &MeshSpec) -> Result<Vec<u8>, String> {
    let mut out = String::from("# semio test oracle\n");
    for vertex in &spec.vertices {
        out.push_str(&format!("v {} {} {}\n", vertex[0], vertex[1], vertex[2]));
    }
    for face in &spec.triangles {
        out.push_str(&format!("f {} {} {}\n", face[0] + 1, face[1] + 1, face[2] + 1));
    }
    Ok(out.into_bytes())
}

/// 👁️ Projects OBJ bytes with the INDEPENDENT `tobj` reader onto the owned `semantic-mesh-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_obj(input: &[u8]) -> Result<Json, String> {
    let text = String::from_utf8(input.to_vec()).map_err(|error| format!("OBJ is not UTF-8: {}", error))?;
    let mut cursor = std::io::Cursor::new(text.into_bytes());
    let (models, _) = tobj::load_obj_buf(&mut cursor, &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default() }, |_| Ok(Default::default())).map_err(|error| format!("independent reader could not parse the OBJ: {}", error))?;
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut triangles: Vec<[usize; 3]> = Vec::new();
    for model in &models {
        let base = vertices.len();
        for chunk in model.mesh.positions.chunks_exact(3) {
            vertices.push([chunk[0], chunk[1], chunk[2]]);
        }
        for face in model.mesh.indices.chunks_exact(3) {
            triangles.push([base + face[0] as usize, base + face[1] as usize, base + face[2] as usize]);
        }
    }
    Ok(MeshSpec { vertices, triangles }.projection("obj"))
}
//#endregion 🔖️Obj

//#region 🔖️Stl
/// 🔮️ Writes a binary STL with the registered `stl_io` reference implementation.
/// @see https://github.com/hmeyer/stl_io
#[cfg(feature = "oracles")]
pub fn oracle_create_stl(spec: &MeshSpec) -> Result<Vec<u8>, String> {
    let faces: Vec<stl_io::Triangle> = spec
        .triangles
        .iter()
        .map(|face| {
            let corners = [spec.vertices[face[0]], spec.vertices[face[1]], spec.vertices[face[2]]];
            stl_io::Triangle { normal: stl_io::Normal::new(face_normal(&corners)), vertices: corners.map(stl_io::Vertex::new) }
        })
        .collect();
    let mut out = std::io::Cursor::new(Vec::new());
    stl_io::write_stl(&mut out, faces.iter()).map_err(|error| format!("stl write: {}", error))?;
    Ok(out.into_inner())
}

#[cfg(feature = "oracles")]
fn face_normal(corners: &[[f32; 3]; 3]) -> [f32; 3] {
    let u = [corners[1][0] - corners[0][0], corners[1][1] - corners[0][1], corners[1][2] - corners[0][2]];
    let v = [corners[2][0] - corners[0][0], corners[2][1] - corners[0][1], corners[2][2] - corners[0][2]];
    let normal = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
    if length == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        [normal[0] / length, normal[1] / length, normal[2] / length]
    }
}

/// 👁️ Projects STL bytes with the INDEPENDENT `stl_io` reader. STL is a triangle soup, so the
/// projection reports resolved corner positions rather than pretending an index buffer survived.
#[cfg(feature = "oracles")]
pub fn project_stl(input: &[u8]) -> Result<Json, String> {
    let mut cursor = std::io::Cursor::new(input.to_vec());
    let mesh = stl_io::read_stl(&mut cursor).map_err(|error| format!("independent reader could not parse the STL: {}", error))?;
    let triangles: Vec<Json> = mesh
        .faces
        .iter()
        .map(|face| Json::Array(face.vertices.iter().map(|index| Json::Array((0..3).map(|axis| Json::Number(mesh.vertices[*index][axis] as f64)).collect())).collect()))
        .collect();
    Ok(Json::Object(vec![("format".to_string(), Json::String("stl".to_string())), ("triangleCount".to_string(), Json::Number(triangles.len() as f64)), ("triangles".to_string(), Json::Array(triangles))]))
}
//#endregion 🔖️Stl

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature nothing here is linked, and every entry point fails loudly.
#[cfg(not(feature = "oracles"))]
mod unavailable {
    use super::{Json, MeshSpec};
    const MESSAGE: &str = "the `oracles` feature is disabled — this host was not built with the registered reference implementations";

    pub fn create_obj(_spec: &MeshSpec) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_obj(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
    pub fn create_stl(_spec: &MeshSpec) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_stl(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
}

#[cfg(not(feature = "oracles"))]
pub use unavailable::{create_obj as oracle_create_obj, create_stl as oracle_create_stl, project_obj, project_stl};
//#endregion 🔖️Unavailable
