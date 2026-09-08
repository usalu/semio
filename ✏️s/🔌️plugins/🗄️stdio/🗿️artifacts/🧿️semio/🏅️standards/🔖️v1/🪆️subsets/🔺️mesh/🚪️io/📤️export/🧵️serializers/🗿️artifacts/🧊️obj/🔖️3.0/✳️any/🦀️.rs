//! 📤️ Serialize `s.stdio.semio/v1/mesh` into `s.stdio.obj/3.0/*` — mirror of the sibling
//! deserializer leaf. Every triangle corner appends a FRESH `v`/`vt`/`vn` entry to the file-global
//! pools rather than deduplicating shared vertices across faces -- `SemioPrimitive`'s flat,
//! non-indexed model has no shared-vertex-pool concept to preserve, so there is nothing honest to
//! deduplicate against (real, valid OBJ; just not vertex-count-minimal — documented, not a
//! fabrication). One `ObjObject` (`o` block) is emitted per `SemioMesh`, its `faces` set to
//! exactly the face range that mesh's primitives produced, so a re-import (the sibling
//! deserializer) recovers the same mesh boundaries.
//!
//! 🔖 Documented lossiness (mirrors the deserializer's list): non-`Triangles` topology is a hard
//! `Err` (OBJ's `f` lines are polygonal, but this codec's fan-triangulation only defines an
//! IMPORT-side transform, not an export-side one — `SemioPrimitive` doesn't retain n-gon
//! structure to re-emit); `material_id`/`colors` are dropped (OBJ's `usemtl` is a bare name with
//! no PBR value/color model to round-trip against `SemioMaterial`).

use semio_s_artifact_stdio_obj::schema::snapshot::{ObjFace, ObjFaceVertex, ObjNormal, ObjObject, ObjTexCoord, ObjVertex};
use semio_s_artifact_stdio_obj::ObjSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTopology};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

pub struct SemioMeshToObj;

impl ArtifactSerializer for SemioMeshToObj {
    type From = SemioMeshSnapshot;
    type Into = ObjSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut vertices: Vec<ObjVertex> = Vec::new();
        let mut texcoords: Vec<ObjTexCoord> = Vec::new();
        let mut normals: Vec<ObjNormal> = Vec::new();
        let mut faces: Vec<ObjFace> = Vec::new();
        let mut objects: Vec<ObjObject> = Vec::new();

        for mesh in &from.meshes {
            let face_range_start = faces.len();
            for prim in &mesh.primitives {
                if prim.topology != SemioTopology::Triangles {
                    return Err(store::PackError::Schema(format!("SemioMeshToObj: primitive {:?} has topology {:?}; this codec only exports Triangles", prim.id, prim.topology)));
                }
                let corner_indices: Vec<u32> = if !prim.indices.is_empty() {
                    if prim.indices.len() % 3 != 0 {
                        return Err(store::PackError::Schema(format!("SemioMeshToObj: primitive {:?} indices length {} is not a multiple of 3", prim.id, prim.indices.len())));
                    }
                    prim.indices.clone()
                } else {
                    if prim.positions.len() % 3 != 0 {
                        return Err(store::PackError::Schema(format!("SemioMeshToObj: non-indexed primitive {:?} has {} positions, not a multiple of 3", prim.id, prim.positions.len())));
                    }
                    (0..prim.positions.len() as u32).collect()
                };

                for tri in corner_indices.chunks(3) {
                    let mut face_vertices = Vec::with_capacity(3);
                    for &idx in tri {
                        let p = prim.positions.get(idx as usize).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToObj: primitive {:?} index {idx} out of bounds", prim.id)))?;
                        vertices.push(ObjVertex { x: p.x, y: p.y, z: p.z, w: None });
                        let vertex_ref = (vertices.len() - 1) as u32;

                        let normal_ref = if !prim.normals.is_empty() {
                            let n = prim.normals.get(idx as usize).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToObj: primitive {:?} normal index {idx} out of bounds", prim.id)))?;
                            normals.push(ObjNormal { x: n.x, y: n.y, z: n.z });
                            Some((normals.len() - 1) as u32)
                        } else {
                            None
                        };

                        let texcoord_ref = if !prim.uvs.is_empty() {
                            let uv = prim.uvs.get(idx as usize).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToObj: primitive {:?} uv index {idx} out of bounds", prim.id)))?;
                            texcoords.push(ObjTexCoord { u: uv.u, v: uv.v, w: None });
                            Some((texcoords.len() - 1) as u32)
                        } else {
                            None
                        };

                        face_vertices.push(ObjFaceVertex { vertex: vertex_ref, texcoord: texcoord_ref, normal: normal_ref });
                    }
                    faces.push(ObjFace { vertices: face_vertices });
                }
            }
            let face_range_end = faces.len();
            objects.push(ObjObject { name: mesh.id.clone(), faces: (face_range_start..face_range_end).collect() });
        }

        Ok(ObjSnapshot { schema: "stdio.obj".into(), vertices, texcoords, normals, faces, groups: Vec::new(), objects, mtllib: None, usemtl: Vec::new(), smoothing_groups: Vec::new(), unknown_statements: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
