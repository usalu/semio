//! 📦 STL/OBJ/GLB/DWG mesh import/export bridged to native B-Rep.
//!
//! Triangle soups interchange through `semio_framework_mesh_engine`/`crate::artifacts::dwg` codecs
//! where available; solids tessellate via [`crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::tessellation`] and import as
//! one planar face per triangle (shell assembly until [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::sew`] can weld
//! shared edges).
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave DEDUP: this file's sole
//! production consumer was already `✳️brep/🧬️schema/⚙️engine`'s own `use semio_framework_3d::
//! brep::mesh_io::{…}` (the "forward edge" this whole file is a leaf of), and its DWG calls were
//! the last framework-tier caller of the old `semio_framework::mesh_to_dwg_drawing`/`dwg_from_bytes`/
//! `dwg_to_bytes`/`dwg_drawing_to_mesh` re-exports (`🔺️mesh`, deleted this same wave). Moving this
//! file here — rather than repointing it at stdio's real `dwg` artifact from across a framework→
//! plugin edge, which would be a real crate cycle since `stdio → semio-framework-3d` already exists
//! for the algorithm forward-edge above — dissolves that edge instead: the DWG calls become
//! same-crate `crate::artifacts::dwg::{…}`, and the framework-3d algorithm imports become the same
//! external `semio_framework_3d::engine::*` forward-edge pattern the parent `engine/component.rs`
//! used at the time. `MeshTransfer` moved again in ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME
//! wave 1 (W1-A): the parent's own `engine::contract` module now owns it same-crate.

use crate::artifacts::dwg::{dwg_drawing_to_mesh, dwg_from_bytes, dwg_to_bytes, mesh_to_dwg_drawing};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_shell, add_solid};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_planar_face_from_points;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::SolidId;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::MeshTransfer;
use semio_framework_mesh_engine::{mesh_from_obj, mesh_from_stl, mesh_to_obj, mesh_to_stl, GlbExporter, GlbImporter, MeshData, MeshExporter, MeshImporter};

// #region 🔖️Types

/// 📦 Indexed triangle soup in kernel world units.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriangleMesh {
    pub positions: Vec<Pnt3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
}

// #endregion 🔖️Types

// #region 🔖️Convert

/// 📦 Converts a tessellation [`MeshTransfer`] into a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn triangle_mesh_from_transfer(transfer: &MeshTransfer) -> TriangleMesh {
    let mut positions = Vec::with_capacity(transfer.position.len() / 3);
    for chunk in transfer.position.chunks_exact(3) {
        positions.push(Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
    }
    let mut normals = Vec::with_capacity(transfer.normal.len() / 3);
    for chunk in transfer.normal.chunks_exact(3) {
        normals.push(Vec3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
    }
    TriangleMesh { positions, normals, indices: transfer.index.clone() }
}

/// 📦 Converts a [`TriangleMesh`] into framework-core [`MeshData`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mesh_to_mesh_data(mesh: &TriangleMesh) -> MeshData {
    let mut positions = Vec::with_capacity(mesh.positions.len() * 3);
    for p in &mesh.positions {
        positions.extend_from_slice(&[p.x as f32, p.y as f32, p.z as f32]);
    }
    let mut normals = Vec::with_capacity(mesh.normals.len() * 3);
    for n in &mesh.normals {
        normals.extend_from_slice(&[n.x as f32, n.y as f32, n.z as f32]);
    }
    MeshData { positions, normals, indices: mesh.indices.clone(), ..MeshData::default() }
}

/// 📦 Converts [`MeshData`] into a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mesh_from_mesh_data(data: &MeshData) -> TriangleMesh {
    let mut positions = Vec::with_capacity(data.vertex_count());
    for chunk in data.positions.chunks_exact(3) {
        positions.push(Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
    }
    let mut normals = Vec::with_capacity(positions.len());
    if data.normals.len() == data.positions.len() {
        for chunk in data.normals.chunks_exact(3) {
            normals.push(Vec3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
        }
    }
    TriangleMesh { positions, normals, indices: data.indices.clone() }
}

// #endregion 🔖️Convert

// #region 🔖️Api

/// 📦 Tessellates `solid` and encodes binary STL. ASCII STL export lives in the `s.stdio.stl/ascii`
/// artifact dialect (`SemioMeshToStl` + `encode_stl_ascii`), not here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_solid_stl(body: &Body, solid: SolidId, deflection: f64) -> Result<Vec<u8>, KernelError> {
    let transfer = tessellate_solid(body, solid, deflection)?;
    export_stl(&triangle_mesh_from_transfer(&transfer))
}

/// 📦 Decodes STL bytes into `body` as a single solid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_stl_to_body(body: &mut Body, data: &[u8], tolerance: f64) -> Result<SolidId, KernelError> {
    import_triangle_mesh_to_body(body, &import_stl(data)?, tolerance)
}

/// 📦 Tessellates `solid` and encodes OBJ text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_solid_obj(body: &Body, solid: SolidId, deflection: f64) -> Result<String, KernelError> {
    let transfer = tessellate_solid(body, solid, deflection)?;
    export_obj(&triangle_mesh_from_transfer(&transfer))
}

/// 📦 Decodes OBJ text into `body` as a single solid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_obj_to_body(body: &mut Body, text: &str, tolerance: f64) -> Result<SolidId, KernelError> {
    import_triangle_mesh_to_body(body, &import_obj(text)?, tolerance)
}

/// 📦 Tessellates `solid` and encodes GLB bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_solid_glb(body: &Body, solid: SolidId, deflection: f64) -> Result<Vec<u8>, KernelError> {
    let transfer = tessellate_solid(body, solid, deflection)?;
    export_glb(&triangle_mesh_from_transfer(&transfer))
}

/// 📦 Decodes GLB bytes into `body` as a single solid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_glb_to_body(body: &mut Body, data: &[u8], tolerance: f64) -> Result<SolidId, KernelError> {
    import_triangle_mesh_to_body(body, &import_glb(data)?, tolerance)
}

/// 📦 Tessellates `solid` and encodes DWG mesh bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_solid_dwg(body: &Body, solid: SolidId, deflection: f64) -> Result<Vec<u8>, KernelError> {
    let transfer = tessellate_solid(body, solid, deflection)?;
    export_dwg(&triangle_mesh_from_transfer(&transfer))
}

/// 📦 Decodes DWG mesh bytes into `body` as a single solid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_dwg_to_body(body: &mut Body, data: &[u8], tolerance: f64) -> Result<SolidId, KernelError> {
    import_triangle_mesh_to_body(body, &import_dwg(data)?, tolerance)
}

/// 📦 Encodes a [`TriangleMesh`] as binary STL.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_stl(mesh: &TriangleMesh) -> Result<Vec<u8>, KernelError> {
    if mesh.indices.len() < 3 {
        return Err(KernelError::InvalidInput("mesh has no triangles".into()));
    }
    Ok(mesh_to_stl(&mesh_to_mesh_data(mesh)))
}

/// 📦 Decodes STL bytes (auto-detects binary vs ASCII).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_stl(data: &[u8]) -> Result<TriangleMesh, KernelError> {
    if is_ascii_stl(data) {
        read_ascii_stl(data)
    } else {
        mesh_from_stl(data).map(|data| mesh_from_mesh_data(&data)).map_err(KernelError::Operation)
    }
}

/// 📦 Encodes OBJ text from a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_obj(mesh: &TriangleMesh) -> Result<String, KernelError> {
    Ok(mesh_to_obj(&mesh_to_mesh_data(mesh), "mesh"))
}

/// 📦 Parses OBJ text into a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_obj(text: &str) -> Result<TriangleMesh, KernelError> {
    mesh_from_obj(text).map(|data| mesh_from_mesh_data(&data)).map_err(KernelError::Operation)
}

/// 📦 Encodes GLB from a [`TriangleMesh`] using [`GlbExporter`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_glb(mesh: &TriangleMesh) -> Result<Vec<u8>, KernelError> {
    let data = mesh_to_mesh_data(mesh);
    GlbExporter.export(&data).map_err(KernelError::Operation)
}

/// 📦 Decodes GLB into a [`TriangleMesh`] using [`GlbImporter`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_glb(data: &[u8]) -> Result<TriangleMesh, KernelError> {
    GlbImporter.import(data).map(|data| mesh_from_mesh_data(&data)).map_err(KernelError::Operation)
}

/// 📦 Encodes DWG mesh bytes from a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_dwg(mesh: &TriangleMesh) -> Result<Vec<u8>, KernelError> {
    let data = mesh_to_mesh_data(mesh);
    let drawing = mesh_to_dwg_drawing(&data);
    dwg_to_bytes(&drawing).map_err(KernelError::Operation)
}

/// 📦 Decodes DWG bytes into a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_dwg(data: &[u8]) -> Result<TriangleMesh, KernelError> {
    let drawing = dwg_from_bytes(data).map_err(KernelError::Operation)?;
    Ok(mesh_from_mesh_data(&dwg_drawing_to_mesh(&drawing)))
}

/// 📦 Imports a triangle soup as a single solid shell (one planar face per triangle).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_triangle_mesh_to_body(body: &mut Body, mesh: &TriangleMesh, tolerance: f64) -> Result<SolidId, KernelError> {
    if mesh.indices.len() < 3 {
        return Err(KernelError::InvalidInput("mesh has no triangles".into()));
    }
    let _tol = tolerance.max(Tol::DEFAULT.value());
    let flip_all = should_flip_winding(mesh);
    let has_normals = mesh.normals.len() >= mesh.positions.len();
    let mut face_ids = Vec::new();
    let mut rec = OpRecorder::new();
    for tri in mesh.indices.chunks_exact(3) {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        if i0 >= mesh.positions.len() || i1 >= mesh.positions.len() || i2 >= mesh.positions.len() {
            return Err(KernelError::InvalidInput("triangle index out of range".into()));
        }
        let p0 = mesh.positions[i0];
        let mut p1 = mesh.positions[i1];
        let mut p2 = mesh.positions[i2];
        if p0 == p1 || p1 == p2 || p0 == p2 {
            continue;
        }
        if has_normals {
            let geo = (p1 - p0).cross(p2 - p0);
            if geo.dot(mesh.normals[i0]) < 0.0 {
                std::mem::swap(&mut p1, &mut p2);
            }
        } else if flip_all {
            std::mem::swap(&mut p1, &mut p2);
        }
        let face = make_planar_face_from_points(body, &[p0, p1, p2], &mut rec)?;
        face_ids.push(face);
    }
    if face_ids.is_empty() {
        return Err(KernelError::Operation("no valid triangles in mesh".into()));
    }
    let shell = add_shell(body, face_ids, &mut rec);
    Ok(add_solid(body, shell, vec![], &mut rec))
}

// #endregion 🔖️Api

// #region 🔖️StlAscii

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_ascii_stl(data: &[u8]) -> bool {
    if data.len() < 84 {
        return data.len() >= 5 && data[..5].eq_ignore_ascii_case(b"solid");
    }
    if !data[..5].eq_ignore_ascii_case(b"solid") {
        return false;
    }
    let tri_count = u32::from_le_bytes([data[80], data[81], data[82], data[83]]);
    let expected = 84u64 + u64::from(tri_count) * 50;
    expected != data.len() as u64
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_ascii_stl(data: &[u8]) -> Result<TriangleMesh, KernelError> {
    let text = std::str::from_utf8(data).map_err(|e| KernelError::Operation(format!("ascii stl utf-8: {e}")))?;
    let mut mesh = TriangleMesh::default();
    let mut current_normal = Vec3::Z;
    let mut vertex_count = 0u32;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("facet normal") {
            current_normal = parse_vec3_token(rest)?;
        } else if let Some(rest) = trimmed.strip_prefix("vertex") {
            mesh.positions.push(parse_point3_token(rest)?);
            mesh.normals.push(current_normal);
            mesh.indices.push(vertex_count);
            vertex_count += 1;
        }
    }
    if mesh.positions.is_empty() {
        return Err(KernelError::Operation("ascii stl has no vertices".into()));
    }
    Ok(mesh)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_vec3_token(s: &str) -> Result<Vec3, KernelError> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(KernelError::Operation(format!("expected 3 floats, got '{s}'")));
    }
    Ok(Vec3::new(parse_f64_token(parts[0])?, parse_f64_token(parts[1])?, parse_f64_token(parts[2])?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_point3_token(s: &str) -> Result<Pnt3, KernelError> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(KernelError::Operation(format!("expected 3 floats, got '{s}'")));
    }
    Ok(Pnt3::new(parse_f64_token(parts[0])?, parse_f64_token(parts[1])?, parse_f64_token(parts[2])?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64_token(s: &str) -> Result<f64, KernelError> {
    s.parse::<f64>().map_err(|e| KernelError::Operation(format!("invalid float '{s}': {e}")))
}

// #endregion 🔖️StlAscii

// #region 🔖️MeshToBody

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn should_flip_winding(mesh: &TriangleMesh) -> bool {
    if mesh.normals.len() >= mesh.positions.len() {
        return false;
    }
    let mut total = 0.0;
    for tri in mesh.indices.chunks_exact(3) {
        let p0 = mesh.positions[tri[0] as usize];
        let p1 = mesh.positions[tri[1] as usize];
        let p2 = mesh.positions[tri[2] as usize];
        let a = Vec3::new(p0.x, p0.y, p0.z);
        let b = Vec3::new(p1.x, p1.y, p1.z);
        let c = Vec3::new(p2.x, p2.y, p2.z);
        total += a.dot(b.cross(c));
    }
    total < 0.0
}

// #endregion 🔖️MeshToBody

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;

    #[semio_framework_async_macros::async_test]
    async fn export_box_mesh_stl_nonempty() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let bytes = export_solid_stl(&body, solid, 0.1).unwrap();
        assert!(bytes.len() > 84, "binary STL must include header and triangles");
        let tri_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
        assert!(tri_count > 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn import_stl_produces_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let bytes = export_solid_stl(&body, solid, 0.1).unwrap();
        let mut imported_body = Body::new();
        let imported = import_stl_to_body(&mut imported_body, &bytes, 1e-4).unwrap();
        assert!(!imported_body.solid_faces(imported).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn obj_export_import_round_trip() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let text = export_solid_obj(&body, solid, 0.1).unwrap();
        assert!(text.contains("v "));
        let mesh = import_obj(&text).unwrap();
        assert!(mesh.indices.len() >= 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn glb_export_import_round_trip() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let bytes = export_solid_glb(&body, solid, 0.1).unwrap();
        assert!(!bytes.is_empty());
        let mesh = import_glb(&bytes).unwrap();
        assert!(mesh.triangle_count() >= 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn import_glb_to_body_has_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let bytes = export_solid_glb(&body, solid, 0.1).unwrap();
        let mut imported = Body::new();
        let imported_solid = import_glb_to_body(&mut imported, &bytes, 1e-4).unwrap();
        assert!(!imported.solid_faces(imported_solid).is_empty());
    }
}

impl TriangleMesh {
    #[cfg(test)]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

// #endregion 🔖️Tests
