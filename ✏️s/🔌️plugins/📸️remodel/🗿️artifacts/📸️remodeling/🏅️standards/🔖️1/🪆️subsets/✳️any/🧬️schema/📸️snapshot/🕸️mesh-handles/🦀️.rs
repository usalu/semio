//! 🕸️ Mesh handles of the remodeling snapshot: the mesh child handle, the chunk assembly, the content handles, the resolution envelope and the bounded mesh chunks with the test mesh workspace. See [`super`].

use super::asset_handles_and_durable_content::{
    remodeling_durable_chunk, RemodelingDurableArtifactStore, RemodelingMeshChild, REMODELING_BOUNDED_MESH_TRIANGLES, REMODELING_BOUNDED_MESH_VERTICES, REMODELING_BOX_MESH_CHILD_ID, REMODELING_EMPTY_MESH_CHILD_ID, REMODELING_MESH_CONTENT_BYTES,
};
#[cfg(test)]
use super::asset_handles_and_durable_content::{RemodelingContentDigest, RemodelingContentKind};
use super::buffers::ByteBuffer;
use semio_framework::MeshData;
#[cfg(test)]
use std::collections::BTreeMap;
use std::sync::Arc;

//#region 🧩️Composition
//#region 🔖️MeshHandle
fn mesh_child_handle(child_id: String, artifact_id: String) -> RemodelingMeshChild {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id, dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

#[cfg(test)]
fn mesh_digest_bytes(chunks: impl IntoIterator<Item = impl AsRef<[u8]>>) -> String {
    let mut digest = RemodelingContentDigest::default();
    for chunk in chunks {
        digest.record(chunk.as_ref());
    }
    digest.content_id(RemodelingContentKind::Mesh)
}

/// 🧱️ Folds one field-tagged mesh leaf (`[field, values…]`, fields in ascending order) into `mesh`.
pub fn apply_mesh_chunk(mesh: &mut MeshData, last_field: Option<u8>, bytes: &[u8]) -> bool {
    let Some((&field, values)) = bytes.split_first() else { return false };
    if field > 11 || last_field.is_some_and(|previous| field < previous) {
        return false;
    }
    let component_count = values.len() / 4;
    match field {
        0 | 1 | 2 | 4 | 7 | 9 if values.len() % 4 == 0 => {
            let target = match field {
                0 => &mut mesh.positions,
                1 => &mut mesh.normals,
                2 => &mut mesh.colors,
                4 => &mut mesh.uvs,
                7 => &mut mesh.edge_positions,
                _ => &mut mesh.edge_uvs,
            };
            let limit = match field {
                0 | 1 => REMODELING_BOUNDED_MESH_VERTICES * 3,
                2 => REMODELING_BOUNDED_MESH_VERTICES * 4,
                4 => REMODELING_BOUNDED_MESH_VERTICES * 2,
                7 => REMODELING_BOUNDED_MESH_TRIANGLES * 6,
                _ => REMODELING_BOUNDED_MESH_TRIANGLES * 4,
            };
            if target.len().checked_add(component_count).is_none_or(|count| count > limit) {
                return false;
            }
            target.extend(values.as_chunks::<4>().0.iter().map(|value| f32::from_le_bytes(*value)));
        }
        3 | 5 | 6 | 8 if values.len() % 4 == 0 => {
            let target = match field {
                3 => &mut mesh.indices,
                5 => &mut mesh.face_ids,
                6 => &mut mesh.vertex_ids,
                _ => &mut mesh.edge_ids,
            };
            let limit = match field {
                3 => REMODELING_BOUNDED_MESH_TRIANGLES * 3,
                5 => REMODELING_BOUNDED_MESH_TRIANGLES,
                6 => REMODELING_BOUNDED_MESH_VERTICES,
                _ => REMODELING_BOUNDED_MESH_TRIANGLES * 3,
            };
            if target.len().checked_add(component_count).is_none_or(|count| count > limit) {
                return false;
            }
            target.extend(values.as_chunks::<4>().0.iter().map(|value| u32::from_le_bytes(*value)));
        }
        10 => {
            if mesh.edge_is_seam.len().checked_add(values.len()).is_none_or(|count| count > REMODELING_BOUNDED_MESH_TRIANGLES * 3) {
                return false;
            }
            mesh.edge_is_seam.extend_from_slice(values);
        }
        11 => {
            let Ok(value) = std::str::from_utf8(values) else { return false };
            if mesh.paint_texture_base64.as_ref().map_or(0, String::len).checked_add(value.len()).is_none_or(|count| count > 24_576) {
                return false;
            }
            mesh.paint_texture_base64.get_or_insert_with(String::new).push_str(value);
        }
        _ => return false,
    }
    true
}

pub(super) fn mesh_from_durable_chunks(chunks: &[ByteBuffer]) -> Option<MeshData> {
    let mut mesh = MeshData::default();
    let mut last_field = None;
    let mut bytes = 0usize;
    for encoded in chunks {
        let chunk = remodeling_durable_chunk(encoded)?;
        bytes = bytes.checked_add(chunk.len()).filter(|total| *total <= REMODELING_MESH_CONTENT_BYTES)?;
        if !apply_mesh_chunk(&mut mesh, last_field, &chunk) {
            return None;
        }
        last_field = chunk.first().copied();
    }
    mesh_is_within_resolution_envelope(&mesh).then_some(mesh)
}

/// 🔗️ The composed mesh child a durable reconstructed or imported mesh publishes.
pub fn remodeling_mesh_content_handle(content_id: &str, chunk_count: u64) -> RemodelingMeshChild {
    mesh_child_handle(content_id.into(), format!("remodeling-mesh-content:{chunk_count}"))
}

pub fn remodeling_mesh_content_handle_parts(handle: &RemodelingMeshChild) -> Option<(&str, u64)> {
    let chunk_count = handle.target.artifact_id.strip_prefix("remodeling-mesh-content:")?;
    Some((&handle.child_id, chunk_count.parse().ok()?))
}

/// 🧊️ Stable empty-mesh handle with no process-owned payload.
pub fn empty_remodeling_mesh_handle() -> RemodelingMeshChild {
    mesh_child_handle(REMODELING_EMPTY_MESH_CHILD_ID.into(), "remodeling-mesh-constant:empty".into())
}

/// 📦️ Stable bounded placeholder handle with no process-owned payload.
pub fn placeholder_remodeling_mesh_handle() -> RemodelingMeshChild {
    mesh_child_handle(REMODELING_BOX_MESH_CHILD_ID.into(), "remodeling-mesh-constant:box".into())
}

pub fn mesh_is_within_resolution_envelope(mesh: &MeshData) -> bool {
    let vertices = mesh.positions.len().checked_div(3);
    let triangles = mesh.indices.len().checked_div(3);
    let Some(vertices) = vertices.filter(|_| mesh.positions.len().is_multiple_of(3)) else { return false };
    let Some(triangles) = triangles.filter(|_| mesh.indices.len().is_multiple_of(3)) else { return false };
    vertices <= REMODELING_BOUNDED_MESH_VERTICES
        && triangles <= REMODELING_BOUNDED_MESH_TRIANGLES
        && mesh.indices.iter().all(|index| usize::try_from(*index).ok().is_some_and(|index| index < vertices))
        && (mesh.normals.is_empty() || vertices.checked_mul(3) == Some(mesh.normals.len()))
        && (mesh.colors.is_empty() || vertices.checked_mul(3) == Some(mesh.colors.len()) || vertices.checked_mul(4) == Some(mesh.colors.len()))
        && (mesh.uvs.is_empty() || vertices.checked_mul(2) == Some(mesh.uvs.len()))
        && (mesh.face_ids.is_empty() || mesh.face_ids.len() == triangles)
        && (mesh.vertex_ids.is_empty() || mesh.vertex_ids.len() == vertices)
        && mesh.normals.len() <= REMODELING_BOUNDED_MESH_VERTICES * 3
        && mesh.colors.len() <= REMODELING_BOUNDED_MESH_VERTICES * 4
        && mesh.uvs.len() <= REMODELING_BOUNDED_MESH_VERTICES * 2
        && mesh.face_ids.len() <= REMODELING_BOUNDED_MESH_TRIANGLES
        && mesh.vertex_ids.len() <= REMODELING_BOUNDED_MESH_VERTICES
        && mesh.edge_positions.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 6
        && mesh.edge_ids.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 3
        && mesh.edge_uvs.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 4
        && mesh.edge_is_seam.len() <= REMODELING_BOUNDED_MESH_TRIANGLES * 3
        && mesh.paint_texture_base64.as_ref().is_none_or(|value| value.len() <= 24_576)
}

/// 🧱️ Returns the durable raw chunk count without reconstructing or cloning mesh fields.
pub fn bounded_remodeling_mesh_chunk_count(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild) -> Option<u64> {
    let (content_id, chunk_count) = remodeling_mesh_content_handle_parts(handle)?;
    let artifact = store.get(content_id).filter(|artifact| artifact.kind == "mesh")?;
    (u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)).then_some(chunk_count)
}

/// 🧱️ Resolves one admitted durable mesh chunk without cloning the reconstructed mesh.
pub fn bounded_remodeling_mesh_chunk(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild, index: u64) -> Option<Arc<[u8]>> {
    let chunk_count = bounded_remodeling_mesh_chunk_count(store, handle)?;
    if index >= chunk_count {
        return None;
    }
    let artifact = store.get(&handle.child_id)?;
    Some(Arc::from(remodeling_durable_chunk(artifact.chunks.get(usize::try_from(index).ok()?)?)?))
}

/// 🧱️ Resolves only fixed constants or durable content admitted by the 512/512 envelope.
pub fn resolve_bounded_remodeling_mesh(store: &RemodelingDurableArtifactStore, handle: &RemodelingMeshChild) -> Option<MeshData> {
    match handle.child_id.as_str() {
        REMODELING_EMPTY_MESH_CHILD_ID => Some(MeshData::default()),
        REMODELING_BOX_MESH_CHILD_ID => Some(semio_framework::mesh_from_kind("box")),
        _ => {
            let chunk_count = bounded_remodeling_mesh_chunk_count(store, handle)?;
            let artifact = store.get(&handle.child_id)?;
            (u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)).then(|| mesh_from_durable_chunks(&artifact.chunks)).flatten()
        }
    }
}

#[cfg(test)]
static REMODELING_TEST_MESHES: std::sync::OnceLock<std::sync::Mutex<BTreeMap<String, MeshData>>> = std::sync::OnceLock::new();

#[cfg(test)]
fn remodeling_test_meshes() -> &'static std::sync::Mutex<BTreeMap<String, MeshData>> {
    REMODELING_TEST_MESHES.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()))
}

#[cfg(any(test, feature = "testkit"))]
pub fn mint_and_stash_mesh(mesh: MeshData) -> RemodelingMeshChild {
    let bytes = semio_framework_pack_json::to_json_string(&mesh).into_bytes();
    let child_id = mesh_digest_bytes([bytes.as_slice()]);
    remodeling_test_meshes().lock().expect("remodeling test mesh lock").insert(child_id.clone(), mesh);
    mesh_child_handle(child_id, "remodeling-mesh".into())
}

#[cfg(any(test, feature = "testkit"))]
pub fn remodeling_mesh_workspace(handle: &RemodelingMeshChild) -> Option<MeshData> {
    if handle.child_id == REMODELING_EMPTY_MESH_CHILD_ID {
        return Some(MeshData::default());
    }
    if handle.child_id == REMODELING_BOX_MESH_CHILD_ID {
        return Some(semio_framework::mesh_from_kind("box"));
    }
    remodeling_test_meshes().lock().expect("remodeling test mesh lock").get(&handle.child_id).cloned()
}
//#endregion 🔖️MeshHandle
//#endregion 🧩️Composition
