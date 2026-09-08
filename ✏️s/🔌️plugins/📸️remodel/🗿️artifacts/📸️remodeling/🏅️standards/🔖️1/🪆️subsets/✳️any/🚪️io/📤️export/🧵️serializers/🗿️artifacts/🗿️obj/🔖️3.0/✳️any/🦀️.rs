use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{resolve_ready, ArtifactSerializer};
use semio_framework_plugin::{StandardId, SubsetId};
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
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError { message: format!("remodeling→obj: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let obj = resolve_ready(SemioMeshToObj::serialize(&semio)).map_err(|error| IoError { message: format!("remodeling→obj: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Text(encode_obj(&obj))))
    }
}
