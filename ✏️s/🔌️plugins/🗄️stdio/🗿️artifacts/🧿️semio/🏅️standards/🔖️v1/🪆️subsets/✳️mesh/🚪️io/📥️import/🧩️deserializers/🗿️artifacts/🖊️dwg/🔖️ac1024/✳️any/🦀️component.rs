//! 📥️ `dwg` (ac1024) → `s.stdio.semio/v1/mesh` — reads the shared logical `DwgSnapshot` drawing model.
//! One `SemioMesh` per real DWG layer that carries at least one `PolyfaceMesh`/`Face3d` entity,
//! named after that layer's own name — the exact inverse grouping the sibling export leaf uses
//! (`DwgDrawing::ensure_layer(&mesh.id)`).
//!
//! Honest lossy points (documented, never fabricated):
//! - Every OTHER `DwgGeometry` variant (`Line`/`Point`/`Circle`/`Arc`/`Ellipse`/`LwPolyline`/
//!   `Spline`/`Text`/`Polyline3d`) has no mesh-shaped equivalent and is dropped — this bridge is
//!   mesh<->mesh only (curves/annotations are the `✳️drawing` bridge's job, not this one's).
//! - `normals`/`uvs`/`colors`/`material_id` have no DWG polyface-mesh field to recover from and
//!   are left empty/`None`, mirroring the sibling export leaf's own drop list.
//! - Malformed logical geometry is a hard `Err`, not a fabricated empty mesh.

use crate::artifacts::dwg::{DwgDrawing, DwgGeometry, DwgSnapshot};
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

//#region 🔖️MeshBuild
fn append_geometry(geometry: &DwgGeometry, positions: &mut Vec<SemioPoint3>, indices: &mut Vec<u32>) {
    match geometry {
        DwgGeometry::PolyfaceMesh { vertices, faces } => {
            let base = positions.len() as u32;
            for v in vertices {
                positions.push(SemioPoint3 { x: v[0], y: v[1], z: v[2] });
            }
            for face in faces {
                let idx: Vec<u32> = face.iter().map(|i| (i.unsigned_abs().saturating_sub(1)) + base).collect();
                if face[2] == face[3] {
                    indices.extend_from_slice(&[idx[0], idx[1], idx[2]]);
                } else {
                    indices.extend_from_slice(&[idx[0], idx[1], idx[2]]);
                    indices.extend_from_slice(&[idx[0], idx[2], idx[3]]);
                }
            }
        }
        DwgGeometry::Face3d { corners } => {
            let base = positions.len() as u32;
            for c in corners {
                positions.push(SemioPoint3 { x: c[0], y: c[1], z: c[2] });
            }
            indices.extend_from_slice(&[base, base + 1, base + 2]);
            if corners[3] != corners[2] {
                indices.extend_from_slice(&[base, base + 2, base + 3]);
            }
        }
        _ => {}
    }
}

fn semio_meshes_from_drawing(drawing: &DwgDrawing) -> Vec<SemioMesh> {
    let mut meshes = Vec::new();
    for (layer_index, layer) in drawing.layers.iter().enumerate() {
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        for entity in drawing.entities.iter().filter(|e| e.layer == layer_index) {
            append_geometry(&entity.geometry, &mut positions, &mut indices);
        }
        if positions.is_empty() {
            continue;
        }
        meshes.push(SemioMesh { id: layer.name.clone(), primitives: vec![SemioPrimitive { id: format!("{}-mesh", layer.name), topology: SemioTopology::Triangles, positions, indices, ..SemioPrimitive::default() }] });
    }
    meshes
}
//#endregion 🔖️MeshBuild

//#region 🔖️Deserializer
pub struct SemioMeshFromDwg;

impl ArtifactDeserializer for SemioMeshFromDwg {
    type From = DwgSnapshot;
    type Into = SemioMeshSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let drawing = from.drawing.to_native().map_err(store::PackError::Schema)?;
        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes: semio_meshes_from_drawing(&drawing), materials: Vec::new(), textures: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::dwg::schema::snapshot::DwgLogicalDrawing;
    use crate::artifacts::dwg::{DwgColor, DwgEntity};

    fn sample_dwg() -> DwgSnapshot {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("walls");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] } });
        DwgSnapshot { version: "AC1015".into(), drawing: DwgLogicalDrawing::from_native(&drawing), ..DwgSnapshot::default() }
    }

    #[test]
    fn groups_polyface_mesh_by_layer_name() {
        let semio = SemioMeshFromDwg::deserialize(&sample_dwg()).expect("deserialize");
        assert_eq!(semio.meshes.len(), 1);
        assert_eq!(semio.meshes[0].id, "walls");
        let prim = &semio.meshes[0].primitives[0];
        assert_eq!(prim.positions.len(), 4);
        assert_eq!(prim.indices.len(), 6, "quad face splits into 2 triangles");
        assert_eq!(prim.topology, SemioTopology::Triangles);
    }

    #[test]
    fn rejects_malformed_payload() {
        let bad = DwgSnapshot { drawing: crate::artifacts::dwg::schema::snapshot::DwgLogicalDrawing { extmax: vec![0.0], ..Default::default() }, ..DwgSnapshot::default() };
        assert!(SemioMeshFromDwg::deserialize(&bad).is_err());
    }
}
//#endregion 🔖️Tests
