use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_framework_plugin::{StandardId, SubsetId};
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
            return Err(IoError { message: "las→remodeling: expected a binary las payload".to_string(), diagnostics: Vec::new() });
        };
        let las = decode_las(bytes).map_err(|error| IoError { message: format!("las→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromLas::deserialize(&las)).map_err(|error| IoError { message: format!("las→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_cloud(&semio).map_err(|reason| IoError { message: format!("las→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
