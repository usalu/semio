use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{resolve_ready, ArtifactSerializer};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::engine::encode_glb;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;

/// 🎯️ The foreign dialect this leaf writes.
pub const GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.gltf@2.0/*` — `results.mesh` as a self-contained GLB
/// container through stdio's real `SemioMeshToGltf` serializer + `gltf::engine::encode_glb`. The mesh's
/// baked texture (`results.mesh.texture_asset_id`) is NOT embedded: it lives in `scene.assets` as a
/// separate composed image child, and inventing a glTF material/texture pair for it would claim a
/// binding the scene does not record. `IoFidelity::Lossy`.
pub struct RemodelingIntoGltf;

impl Serializer<RemodelingSnapshot> for RemodelingIntoGltf {
    const INTO: Dialect = GLTF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError { message: format!("remodeling→gltf: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let gltf = resolve_ready(SemioMeshToGltf::serialize(&semio)).map_err(|error| IoError { message: format!("remodeling→gltf: {error}"), diagnostics: Vec::new() })?;
        let bytes = encode_glb(&gltf).map_err(|error| IoError { message: format!("remodeling→gltf: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
