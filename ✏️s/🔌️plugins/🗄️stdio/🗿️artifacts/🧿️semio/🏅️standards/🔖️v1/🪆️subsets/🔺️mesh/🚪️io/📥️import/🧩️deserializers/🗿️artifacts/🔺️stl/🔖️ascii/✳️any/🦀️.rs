//! 📥️ Deserialize `s.stdio.semio/v1/mesh` from `s.stdio.stl/ascii/*` — STL is geometry-only,
//! per-triangle, non-indexed (each `StlTriangle` owns its own 3 vertices, no shared vertex pool).
//! Maps to ONE `SemioPrimitive` per file: `Triangles` topology, positions laid out as a flat
//! triangle soup in file order, `indices` left empty (this artifact's documented convention for
//! "already laid out sequentially, no shared-index draw" — the gltf sibling leaf uses the same
//! convention when a primitive's `indices` accessor is absent).
//!
//! 🔖 Documented lossiness: STL carries ONE normal per facet (`StlTriangle.normal`) — expanded to
//! 3 identical per-vertex `SemioPoint3` normals (`SemioPrimitive.normals` is per-vertex, STL's is
//! per-face; this is a lossless *expansion*, not a fabrication — every expanded value is the real
//! source normal, never invented). STL has no UV/color/material/multi-mesh concept at all — `uvs`/
//! `colors` stay empty, `material_id` stays `None`, and `solid_name` (the one file-level string
//! STL carries) becomes the single `SemioMesh.id`.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_stl::StlSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

pub struct SemioMeshFromStl;

impl ArtifactDeserializer for SemioMeshFromStl {
    type From = StlSnapshot;
    type Into = SemioMeshSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut positions = Vec::with_capacity(from.triangles.len() * 3);
        let mut normals = Vec::with_capacity(from.triangles.len() * 3);
        for tri in &from.triangles {
            let n = SemioPoint3 { x: tri.normal[0], y: tri.normal[1], z: tri.normal[2] };
            for v in &tri.vertices {
                positions.push(SemioPoint3 { x: v[0], y: v[1], z: v[2] });
                normals.push(n);
            }
        }
        let mesh_id = if from.solid_name.is_empty() { "mesh-0".to_string() } else { from.solid_name.clone() };
        let primitive = SemioPrimitive { id: format!("{mesh_id}-prim-0"), topology: SemioTopology::Triangles, positions, normals, uvs: Vec::new(), colors: Vec::new(), indices: Vec::new(), material_id: None };
        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes: vec![SemioMesh { id: mesh_id, primitives: vec![primitive] }], materials: Vec::new(), textures: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
