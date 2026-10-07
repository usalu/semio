use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use {semio_framework::io_schema::Confidence,semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use semio_framework_plugin::{ ArtifactDeserializer};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_las::standards::v1_0::engine::decode_las;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas;

/// 🎯️ The foreign dialect this leaf reads.
pub const LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.las@1.0/*` → `s.remodel.remodeling@1/*` — the decoded points seed `results.sparse`.
/// LAS classification codes are dropped (`SemioMeshSnapshot` has no per-point class channel), which is
/// exactly why this hop is `IoFidelity::Lossy` rather than semantic.
pub struct LasIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for LasIntoRemodeling {
    const FROM: Dialect = LAS_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(b"LASF") => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "las→remodeling: expected a binary las payload".to_string())));
        };
        let las = decode_las(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("las→remodeling: decode failed: {error}"))))?;
        let semio = ::semio_framework_async::poll::resolve_ready(SemioMeshFromLas::deserialize(&las)).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("las→remodeling: {error}"))))?;
        let scene = io_root::scene_from_semio_cloud(&semio).map_err(|reason| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("las→remodeling: {reason}"))))?;
        Ok(IoOutcome::clean(scene))
    }
}
