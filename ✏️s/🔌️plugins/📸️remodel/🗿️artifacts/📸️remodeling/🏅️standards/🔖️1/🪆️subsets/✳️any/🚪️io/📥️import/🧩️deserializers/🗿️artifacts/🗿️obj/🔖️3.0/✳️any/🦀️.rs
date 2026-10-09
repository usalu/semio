use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework::io_schema::Confidence,semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use semio_framework_plugin::{ ArtifactDeserializer};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_obj::standards::v3_0::engine::decode_obj;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;

/// 🎯️ The foreign dialect this leaf reads.
pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.obj@3.0/*` → `s.remodel.remodeling@1/*` — the decoded surface becomes a real durable
/// mesh asset (`durable_artifacts` chunks + a replayable `results.mesh.mesh` handle, the same shape a
/// committed reconstruction writes), with `MeshSource::Imported`. Materials, groups and object names
/// are dropped: `IoFidelity::Lossy`.
pub struct ObjIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for ObjIntoRemodeling {
    const FROM: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.lines().any(|line| line.starts_with("v ") || line.starts_with("f ")) => Confidence::Medium,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "obj→remodeling: expected a text obj payload".to_string())));
        };
        let obj = decode_obj(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("obj→remodeling: decode failed: {error}"))))?;
        let semio = ::semio_framework_async::poll::resolve_ready(SemioMeshFromObj::deserialize(&obj)).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("obj→remodeling: {error}"))))?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("obj→remodeling: {reason}"))))?;
        Ok(IoOutcome::clean(scene))
    }
}
