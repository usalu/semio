use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use semio_framework_plugin::{ ArtifactSerializer};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_gltf::standards::v2_0::engine::encode_glb;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;

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
    async fn serialize(from: &RemodelingSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→gltf: nothing to export: {reason}"))))?;
        let gltf = ::semio_framework_async::poll::resolve_ready(SemioMeshToGltf::serialize(&semio)).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→gltf: {error}"))))?;
        let bytes = encode_glb(&gltf).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→gltf: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
