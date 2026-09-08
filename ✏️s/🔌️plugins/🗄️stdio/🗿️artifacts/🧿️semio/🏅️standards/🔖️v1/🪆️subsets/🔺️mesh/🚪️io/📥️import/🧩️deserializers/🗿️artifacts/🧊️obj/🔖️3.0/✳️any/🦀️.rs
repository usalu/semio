//! 📥️ Deserialize `s.stdio.semio/v1/mesh` from `s.stdio.obj/3.0/*` — geometry-only. OBJ's `f`
//! lines are MULTI-indexed (each face corner independently references `v`/`vt`/`vn` by its own
//! index — the three can diverge per corner), while `SemioPrimitive` is SINGLE-indexed (one
//! `indices` array walks all of `positions`/`normals`/`uvs` together). Rather than silently
//! re-welding corners into a deduplicated shared-index space (a real algorithm with its own
//! ordering choices this recipe won't invent unasked), every face corner becomes its OWN
//! independent point in a flat, non-indexed triangle-soup layout — geometry VALUES are exact,
//! only the OBJ source's index-sharing structure is flattened. `indices` stays empty (same
//! "empty means sequential" convention the gltf/stl sibling leaves use).
//!
//! 🔖 Documented lossiness:
//! - N-gon `f` faces (`vertices.len() > 3`) are fan-triangulated (`v0,vi,vi+1` for `i in 1..n-1`)
//!   — real triangle count differs from face count for n>3, but every vertex position/uv/normal
//!   value used is copied verbatim, never fabricated.
//! - `groups`/`usemtl`/`smoothing_groups`/`mtllib`/`unknown_statements` (comments, material
//!   library refs, smoothing state) have no `SemioMesh`/`SemioPrimitive` counterpart — dropped.
//!   `objects` (`o` blocks), when present, DO map — one `SemioMesh` per `ObjObject`, using its
//!   `faces` index membership to select which triangulated faces belong to it; when `objects` is
//!   empty, every face lands in one `SemioMesh` named `"mesh-0"`.
//! - OBJ has no material/texture/color model beyond the name-only `usemtl`/`mtllib` strings (no
//!   embedded PBR values or texture bytes to map) — `materials`/`textures` stay empty,
//!   `material_id` stays `None`.

use semio_s_artifact_stdio_obj::schema::snapshot::ObjFace;
use semio_s_artifact_stdio_obj::ObjSnapshot;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioUv};
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

//#region 🔖️FaceMapping
/// 🔺️ Fan-triangulates one (possibly n-gon) `ObjFace` against `from`, appending each triangle
/// corner's resolved position/uv/normal to the accumulators. Negative/relative OBJ indices are
/// already resolved to absolute 0-based indices by the OBJ codec upstream (per `ObjFaceVertex`'s
/// own doc comment) -- this only does the corner lookup + fan expansion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn append_triangulated_face(from: &ObjSnapshot, face: &ObjFace, positions: &mut Vec<SemioPoint3>, normals: &mut Vec<SemioPoint3>, uvs: &mut Vec<SemioUv>) -> Result<(), String> {
    if face.vertices.len() < 3 {
        return Err(format!("obj face has {} corners, need at least 3", face.vertices.len()));
    }
    let corner = |idx: usize| -> Result<(SemioPoint3, Option<SemioPoint3>, Option<SemioUv>), String> {
        let fv = &face.vertices[idx];
        let v = from.vertices.get(fv.vertex as usize).ok_or_else(|| format!("obj face references out-of-range vertex {}", fv.vertex))?;
        let position = SemioPoint3 { x: v.x, y: v.y, z: v.z };
        let normal = match fv.normal {
            Some(ni) => Some(from.normals.get(ni as usize).map(|n| SemioPoint3 { x: n.x, y: n.y, z: n.z }).ok_or_else(|| format!("obj face references out-of-range normal {ni}"))?),
            None => None,
        };
        let uv = match fv.texcoord {
            Some(ti) => Some(from.texcoords.get(ti as usize).map(|t| SemioUv { u: t.u, v: t.v }).ok_or_else(|| format!("obj face references out-of-range texcoord {ti}"))?),
            None => None,
        };
        Ok((position, normal, uv))
    };

    let has_normals = face.vertices.iter().all(|fv| fv.normal.is_some());
    let has_uvs = face.vertices.iter().all(|fv| fv.texcoord.is_some());

    for i in 1..face.vertices.len() - 1 {
        for &corner_idx in &[0usize, i, i + 1] {
            let (position, normal, uv) = corner(corner_idx)?;
            positions.push(position);
            if has_normals {
                normals.push(normal.expect("checked has_normals"));
            }
            if has_uvs {
                uvs.push(uv.expect("checked has_uvs"));
            }
        }
    }
    Ok(())
}
//#endregion 🔖️FaceMapping

pub struct SemioMeshFromObj;

impl ArtifactDeserializer for SemioMeshFromObj {
    type From = ObjSnapshot;
    type Into = SemioMeshSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let build_primitive = |id: String, face_indices: Vec<usize>| -> Result<SemioPrimitive, store::PackError> {
            let mut positions = Vec::new();
            let mut normals = Vec::new();
            let mut uvs = Vec::new();
            for &fi in &face_indices {
                let face = from.faces.get(fi).ok_or_else(|| store::PackError::Schema(format!("SemioMeshFromObj: face index {fi} out of range")))?;
                append_triangulated_face(from, face, &mut positions, &mut normals, &mut uvs).map_err(|e| store::PackError::Schema(format!("SemioMeshFromObj: {e}")))?;
            }
            Ok(SemioPrimitive { id, topology: SemioTopology::Triangles, positions, normals, uvs, colors: Vec::new(), indices: Vec::new(), material_id: None })
        };

        let meshes = if from.objects.is_empty() {
            let all_faces: Vec<usize> = (0..from.faces.len()).collect();
            vec![SemioMesh { id: "mesh-0".to_string(), primitives: vec![build_primitive("mesh-0-prim-0".to_string(), all_faces)?] }]
        } else {
            let mut meshes = Vec::with_capacity(from.objects.len());
            for obj in &from.objects {
                let prim = build_primitive(format!("{}-prim-0", obj.name), obj.faces.clone())?;
                meshes.push(SemioMesh { id: obj.name.clone(), primitives: vec![prim] });
            }
            meshes
        };

        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes, materials: Vec::new(), textures: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
