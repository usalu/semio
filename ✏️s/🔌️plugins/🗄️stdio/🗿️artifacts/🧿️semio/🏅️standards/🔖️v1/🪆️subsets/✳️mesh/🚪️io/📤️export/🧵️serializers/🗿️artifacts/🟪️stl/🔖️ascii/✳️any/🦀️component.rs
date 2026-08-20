//! 📤️ Serialize `s.stdio.semio/v1/mesh` into `s.stdio.stl/ascii/*` — mirror of the sibling
//! deserializer leaf. STL is Triangles-only, non-indexed, one-normal-per-facet, no material/UV/
//! color/multi-mesh concept — every mismatch below is a REAL, documented impedance gap, not a
//! silent drop:
//!
//! - A primitive whose `topology != Triangles` cannot be represented at all -> hard `Err`.
//! - `indices`, if present, must group into complete triangles (len % 3 == 0); non-indexed
//!   positions must themselves already be a flat triangle soup (len % 3 == 0).
//! - Per-vertex `normals` (if present) collapse to ONE facet normal per triangle (averaged, then
//!   renormalized) — STL structurally cannot carry 3 independent per-vertex normals; when the 3
//!   corner normals already agree (the common case, and what the sibling deserializer produces),
//!   this average is exact, not lossy.
//! - `uvs`/`colors`/`material_id` are dropped entirely (STL has no field for any of them).
//! - Multiple `SemioMesh`es flatten into ONE `solid` (STL is single-solid); `solid_name` is the
//!   FIRST mesh's id — later meshes' id boundaries are not preserved in the STL output.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTopology};
use crate::artifacts::stl::schema::snapshot::StlTriangle;
use crate::artifacts::stl::StlSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

//#region 🔖️NormalMath
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_normal(v0: SemioPoint3, v1: SemioPoint3, v2: SemioPoint3) -> [f64; 3] {
    let e1 = [v1.x - v0.x, v1.y - v0.y, v1.z - v0.z];
    let e2 = [v2.x - v0.x, v2.y - v0.y, v2.z - v0.z];
    let n = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]];
    normalize(n)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn average_normal(n0: SemioPoint3, n1: SemioPoint3, n2: SemioPoint3) -> [f64; 3] {
    normalize([(n0.x + n1.x + n2.x) / 3.0, (n0.y + n1.y + n2.y) / 3.0, (n0.z + n1.z + n2.z) / 3.0])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-12 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        [0.0, 0.0, 0.0]
    }
}
//#endregion 🔖️NormalMath

pub struct SemioMeshToStl;

impl ArtifactSerializer for SemioMeshToStl {
    type From = SemioMeshSnapshot;
    type Into = StlSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let solid_name = from.meshes.first().map(|m| m.id.clone()).unwrap_or_default();
        let mut triangles = Vec::new();

        for mesh in &from.meshes {
            for prim in &mesh.primitives {
                if prim.topology != SemioTopology::Triangles {
                    return Err(store::PackError::Schema(format!("SemioMeshToStl: primitive {:?} has topology {:?}; STL can only represent Triangles", prim.id, prim.topology)));
                }
                let corner_indices: Vec<u32> = if !prim.indices.is_empty() {
                    if prim.indices.len() % 3 != 0 {
                        return Err(store::PackError::Schema(format!("SemioMeshToStl: primitive {:?} indices length {} is not a multiple of 3", prim.id, prim.indices.len())));
                    }
                    prim.indices.clone()
                } else {
                    if prim.positions.len() % 3 != 0 {
                        return Err(store::PackError::Schema(format!("SemioMeshToStl: non-indexed primitive {:?} has {} positions, not a multiple of 3", prim.id, prim.positions.len())));
                    }
                    (0..prim.positions.len() as u32).collect()
                };

                for face in corner_indices.chunks(3) {
                    let get = |i: u32| -> Result<SemioPoint3, store::PackError> { prim.positions.get(i as usize).copied().ok_or_else(|| store::PackError::Schema(format!("SemioMeshToStl: primitive {:?} index {i} out of bounds", prim.id))) };
                    let (i0, i1, i2) = (face[0], face[1], face[2]);
                    let (v0, v1, v2) = (get(i0)?, get(i1)?, get(i2)?);
                    let normal = if !prim.normals.is_empty() {
                        let (n0, n1, n2) = (
                            *prim.normals.get(i0 as usize).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToStl: primitive {:?} normal index {i0} out of bounds", prim.id)))?,
                            *prim.normals.get(i1 as usize).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToStl: primitive {:?} normal index {i1} out of bounds", prim.id)))?,
                            *prim.normals.get(i2 as usize).ok_or_else(|| store::PackError::Schema(format!("SemioMeshToStl: primitive {:?} normal index {i2} out of bounds", prim.id)))?,
                        );
                        average_normal(n0, n1, n2)
                    } else {
                        face_normal(v0, v1, v2)
                    };
                    triangles.push(StlTriangle { normal, vertices: [[v0.x, v0.y, v0.z], [v1.x, v1.y, v1.z], [v2.x, v2.y, v2.z]] });
                }
            }
        }

        Ok(StlSnapshot { schema: "stdio.stl".into(), solid_name, triangles })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
    use semio_framework_plugin::ArtifactDeserializer;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_semio_mesh() -> SemioMeshSnapshot {
        SemioMeshSnapshot {
            schema: "stdio.semio.mesh".into(),
            meshes: vec![SemioMesh {
                id: "pyramid".into(),
                primitives: vec![SemioPrimitive {
                    id: "pyramid-prim-0".into(),
                    topology: SemioTopology::Triangles,
                    positions: vec![
                        SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 },
                        SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 },
                        SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 },
                        SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 },
                        SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
                        SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 },
                    ],
                    normals: vec![
                        SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
                        SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
                        SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
                        SemioPoint3 { x: 0.0, y: -1.0, z: 0.0 },
                        SemioPoint3 { x: 0.0, y: -1.0, z: 0.0 },
                        SemioPoint3 { x: 0.0, y: -1.0, z: 0.0 },
                    ],
                    uvs: Vec::new(),
                    colors: Vec::new(),
                    indices: Vec::new(),
                    material_id: None,
                }],
            }],
            materials: Vec::new(),
            textures: Vec::new(),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn serialize_then_deserialize_round_trips_at_the_semio_level() {
        let original = sample_semio_mesh();
        let stl = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&original)).expect("serialize");
        assert_eq!(stl.solid_name, "pyramid");
        assert_eq!(stl.triangles.len(), 2);
        assert_eq!(stl.triangles[0].normal, [0.0, 0.0, 1.0]);
        let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromStl::deserialize(&stl)).expect("deserialize");
        assert_eq!(original, round_tripped, "uniform per-triangle normals must average back exactly");
    }

    #[semio_framework_async_macros::async_test]
    async fn non_triangle_topology_is_a_hard_error() {
        let mut semio = sample_semio_mesh();
        semio.meshes[0].primitives[0].topology = SemioTopology::Lines;
        let err = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&semio)).expect_err("Lines topology must error");
        assert!(format!("{err:?}").contains("Triangles"), "got {err:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn indexed_triangles_export_correctly() {
        let mut semio = sample_semio_mesh();
        semio.meshes[0].primitives[0].positions = vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }];
        semio.meshes[0].primitives[0].normals = vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 3];
        semio.meshes[0].primitives[0].indices = vec![0, 1, 2];
        let stl = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&semio)).expect("serialize");
        assert_eq!(stl.triangles.len(), 1);
        assert_eq!(stl.triangles[0].vertices, [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
    }
}
//#endregion 🔖️Tests
