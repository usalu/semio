use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework::io_schema::Confidence,semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use semio_framework_plugin::{ ArtifactDeserializer};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl;
use semio_s_artifact_stdio_stl::standards::v_ascii::engine::decode_stl_ascii;

/// 🎯️ The foreign dialect this leaf reads.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.stl@ascii/*` → `s.remodel.remodeling@1/*` — the decoded triangle soup becomes a real
/// durable mesh asset. Vertices stay unshared (STL has no index list to recover): `IoFidelity::Lossy`.
pub struct StlIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for StlIntoRemodeling {
    const FROM: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.trim_start().starts_with("solid") => Confidence::Medium,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "stl→remodeling: expected a text ascii-stl payload".to_string())));
        };
        let stl = decode_stl_ascii(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("stl→remodeling: decode failed: {error}"))))?;
        let semio = ::semio_framework_async::poll::resolve_ready(SemioMeshFromStl::deserialize(&stl)).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("stl→remodeling: {error}"))))?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("stl→remodeling: {reason}"))))?;
        Ok(IoOutcome::clean(scene))
    }
}
