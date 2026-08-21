//! 📤️ `s.stdio.semio/v1/mesh` → `dwg` (ac1024) — mirrors the import leaf. Builds one
//! `DwgGeometry::PolyfaceMesh` entity per `SemioMesh` (one real DWG layer, named after the mesh's
//! own `id`, per `DwgDrawing::ensure_layer`), then encodes the whole `DwgDrawing` through the
//! relocated (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS G2) hand-
//! rolled DWG logical model. Native bytes are materialized only by the DWG serializer.
//!
//! 🔖 Documented lossiness (mirrors the import leaf's list): non-`Triangles` topology is a hard
//! `Err` (`DwgGeometry::PolyfaceMesh` has no fan/strip semantics to preserve, matching the OBJ/
//! gltf mesh bridges' own convention); `normals`/`uvs`/`colors`/`material_id` have no DWG
//! polyface-mesh field to round-trip through and are dropped (DWG entities carry vertex
//! positions + face indices only).

use crate::artifacts::dwg::schema::snapshot::DwgLogicalDrawing;
use crate::artifacts::dwg::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry, DwgSnapshot};
#[cfg(test)]
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTopology};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };

/// 📐 The logical drawing bridge's version identifier.
const DWG_CODEC_VERSION: &str = "AC1015";

//#region 🔖️Serializer
pub struct SemioMeshToDwg;

impl ArtifactSerializer for SemioMeshToDwg {
    type From = SemioMeshSnapshot;
    type Into = DwgSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut drawing = DwgDrawing::default();
        for mesh in &from.meshes {
            let layer = drawing.ensure_layer(&mesh.id);
            let mut vertices: Vec<[f64; 3]> = Vec::new();
            let mut faces: Vec<[i32; 4]> = Vec::new();
            for prim in &mesh.primitives {
                if prim.topology != SemioTopology::Triangles {
                    return Err(store::PackError::Schema(format!("SemioMeshToDwg: primitive {:?} has topology {:?}; this codec only exports Triangles", prim.id, prim.topology)));
                }
                let base = vertices.len() as i32;
                for p in &prim.positions {
                    vertices.push([p.x, p.y, p.z]);
                }
                let corner_indices: Vec<u32> = if !prim.indices.is_empty() {
                    if prim.indices.len() % 3 != 0 {
                        return Err(store::PackError::Schema(format!("SemioMeshToDwg: primitive {:?} indices length {} is not a multiple of 3", prim.id, prim.indices.len())));
                    }
                    prim.indices.clone()
                } else {
                    if prim.positions.len() % 3 != 0 {
                        return Err(store::PackError::Schema(format!("SemioMeshToDwg: non-indexed primitive {:?} has {} positions, not a multiple of 3", prim.id, prim.positions.len())));
                    }
                    (0..prim.positions.len() as u32).collect()
                };
                for tri in corner_indices.chunks(3) {
                    let [a, b, c] = [tri[0] as i32 + base + 1, tri[1] as i32 + base + 1, tri[2] as i32 + base + 1];
                    faces.push([a, b, c, c]);
                }
            }
            if vertices.is_empty() {
                continue;
            }
            drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices, faces } });
        }

        let mut snapshot = DwgSnapshot::default();
        snapshot.version = DWG_CODEC_VERSION.into();
        snapshot.drawing = DwgLogicalDrawing::from_native(&drawing).map_err(store::PackError::Schema)?;
        Ok(snapshot)
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::dwg::v_ac1024::any::SemioMeshFromDwg;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
    use semio_framework_plugin::ArtifactDeserializer;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_semio_mesh() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            schema: "stdio.semio.mesh".into(),
            meshes: vec![SemioMesh {
                id: "box".into(),
                primitives: vec![SemioPrimitive {
                    id: "box-prim-0".into(),
                    topology: SemioTopology::Triangles,
                    positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 1.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                    indices: vec![0, 1, 2, 0, 2, 3],
                    ..SemioPrimitive::default()
                }],
            }],
            ..SemioMeshSnapshot::default()
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn serialize_then_deserialize_round_trips_triangle_and_vertex_counts() {
        let original = sample_semio_mesh();
        let dwg = semio_framework_plugin::resolve_ready(SemioMeshToDwg::serialize(&original)).expect("serialize");
        assert_eq!(dwg.version, DWG_CODEC_VERSION);
        let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromDwg::deserialize(&dwg)).expect("deserialize");
        assert_eq!(round_tripped.meshes.len(), 1);
        assert_eq!(round_tripped.meshes[0].id, "box");
        let prim = &round_tripped.meshes[0].primitives[0];
        assert_eq!(prim.indices.len(), 6, "quad fan-splits into 2 triangles == 6 indices");
        assert_eq!(prim.positions.len(), 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn non_triangle_topology_is_a_hard_error() {
        let mut semio = sample_semio_mesh();
        semio.meshes[0].primitives[0].topology = SemioTopology::TriangleFan;
        let err = semio_framework_plugin::resolve_ready(SemioMeshToDwg::serialize(&semio)).expect_err("TriangleFan must error");
        assert!(format!("{err:?}").contains("Triangles"), "got {err:?}");
    }
}
//#endregion 🔖️Tests
