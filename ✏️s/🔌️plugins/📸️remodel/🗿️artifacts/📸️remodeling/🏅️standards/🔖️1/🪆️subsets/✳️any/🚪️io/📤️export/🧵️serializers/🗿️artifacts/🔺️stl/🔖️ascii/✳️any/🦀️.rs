use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{resolve_ready, ArtifactSerializer};
use semio_framework_plugin::{StandardId, SubsetId};
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
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError { message: format!("remodeling→stl: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let stl = resolve_ready(SemioMeshToStl::serialize(&semio)).map_err(|error| IoError { message: format!("remodeling→stl: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Text(encode_stl_ascii(&stl))))
    }
}
