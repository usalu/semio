//! 📥️ `dwg` (ac1024) → `s.stdio.semio/v1/mesh` — reads the shared logical `DwgSnapshot` drawing model.
//! One `SemioMesh` per real DWG layer that carries at least one `PolyfaceMesh`/`Face3d` entity,
//! named after that layer's own name — the exact inverse grouping the sibling export leaf uses
//! (`DwgDrawing::ensure_layer(&mesh.id)`).
//!
//! Honest lossy points (documented, never fabricated):
//! - Every OTHER `DwgGeometry` variant (`Line`/`Point`/`Circle`/`Arc`/`Ellipse`/`LwPolyline`/
//!   `Spline`/`Text`/`Polyline3d`) has no mesh-shaped equivalent and is dropped — this bridge is
//!   mesh<->mesh only (curves/annotations are the `🖊️drawing` bridge's job, not this one's).
//! - `normals`/`uvs`/`colors`/`material_id` have no DWG polyface-mesh field to recover from and
//!   are left empty/`None`, mirroring the sibling export leaf's own drop list.
//! - Malformed logical geometry is a hard `Err`, not a fabricated empty mesh.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use {semio_plugin::ArtifactDeserializer,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::{DwgDrawing, DwgGeometry, dwg_face_triangles, dwg_mesh_rows};
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

//#region 🔖️MeshBuild
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn append_geometry(geometry: &DwgGeometry, positions: &mut Vec<SemioPoint3>, indices: &mut Vec<u32>) -> Result<(), String> {
    let (vertices, faces) = dwg_mesh_rows(geometry);
    let base = u32::try_from(positions.len()).map_err(|_| "DWG mesh has too many vertices")?;
    let count = u32::try_from(vertices.len()).map_err(|_| "DWG mesh has too many vertices")?;
    base.checked_add(count).ok_or("DWG mesh has too many vertices")?;
    positions.try_reserve(vertices.len()).map_err(|_| "DWG mesh position allocation failed")?;
    for &[x, y, z] in vertices {
        if ![x, y, z].into_iter().all(f64::is_finite) { return Err("DWG mesh position is not finite".into()); }
        positions.push(SemioPoint3 { x, y, z });
    }
    for &face in faces {
        let (first, second) = dwg_face_triangles(base, count, face)?;
        indices.try_reserve(if second.is_some() { 6 } else { 3 }).map_err(|_| "DWG mesh index allocation failed")?;
        indices.extend_from_slice(&first);
        if let Some(second) = second { indices.extend_from_slice(&second); }
    }
    Ok(())
}

fn semio_meshes_from_drawing(drawing: &DwgDrawing) -> Result<Vec<SemioMesh>, String> {
    let mut geometry: Vec<_> = drawing.layers.iter().map(|_| (Vec::new(), Vec::new())).collect();
    for entity in &drawing.entities {
        if let Some((positions, indices)) = geometry.get_mut(entity.layer) {
            append_geometry(&entity.geometry, positions, indices)?;
        }
    }
    Ok(drawing.layers.iter().zip(geometry).filter_map(|(layer, (positions, indices))| {
        if indices.is_empty() { return None; }
        Some(SemioMesh { id: layer.name.clone(), primitives: vec![SemioPrimitive {
            id: format!("{}-mesh", layer.name), topology: SemioTopology::Triangles, positions, indices, ..SemioPrimitive::default()
        }] })
    }).collect())
}
//#endregion 🔖️MeshBuild

//#region 🔖️Deserializer
pub struct SemioMeshFromDwg;

impl ArtifactDeserializer for SemioMeshFromDwg {
    type From = DwgSnapshot;
    type Into = SemioMeshSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let meshes = from.drawing.to_native().and_then(|drawing| semio_meshes_from_drawing(&drawing))
            .map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))?;
        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes, materials: Vec::new(), textures: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
