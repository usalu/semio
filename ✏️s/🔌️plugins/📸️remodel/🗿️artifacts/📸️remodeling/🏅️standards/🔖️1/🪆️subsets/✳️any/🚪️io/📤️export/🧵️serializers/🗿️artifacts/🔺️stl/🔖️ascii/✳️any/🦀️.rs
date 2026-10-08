use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use semio_framework_plugin::{ ArtifactSerializer};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
use semio_s_artifact_stdio_stl::standards::v_ascii::engine::encode_stl_ascii;

/// 🎯️ The foreign dialect this leaf writes.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.stl@ascii/*` — `results.mesh` as an ASCII triangle soup
/// through stdio's real `SemioMeshToStl` serializer + `stl::engine::encode_stl_ascii`. STL carries no
/// vertex sharing, colors, uvs or names, so this is `IoFidelity::Lossy` even for the mesh alone.
pub struct RemodelingIntoStl;

impl Serializer<RemodelingSnapshot> for RemodelingIntoStl {
    const INTO: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→stl: nothing to export: {reason}"))))?;
        let stl = ::semio_framework_async::poll::resolve_ready(SemioMeshToStl::serialize(&semio)).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→stl: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Text(encode_stl_ascii(&stl))))
    }
}
