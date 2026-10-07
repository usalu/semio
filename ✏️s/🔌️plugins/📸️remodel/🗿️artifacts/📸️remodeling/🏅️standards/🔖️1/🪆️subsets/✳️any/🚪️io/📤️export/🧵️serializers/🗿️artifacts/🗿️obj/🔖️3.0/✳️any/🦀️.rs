use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use semio_framework_plugin::{ ArtifactSerializer};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_obj::standards::v3_0::engine::encode_obj;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;

/// 🎯️ The foreign dialect this leaf writes.
pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.obj@3.0/*` — the reconstructed surface `results.mesh`
/// only, through stdio's real `SemioMeshToObj` serializer + `obj::engine::encode_obj`. A scene with
/// only a point cloud gets a typed `Err` naming that, never an empty solid.
pub struct RemodelingIntoObj;

impl Serializer<RemodelingSnapshot> for RemodelingIntoObj {
    const INTO: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→obj: nothing to export: {reason}"))))?;
        let obj = ::semio_framework_async::poll::resolve_ready(SemioMeshToObj::serialize(&semio)).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→obj: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Text(encode_obj(&obj))))
    }
}
