use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_ply::standards::v1_0::engine::decode_ply;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;

/// 🎯️ The foreign dialect this leaf reads.
pub const PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.ply@1.0/*` → `s.remodel.remodeling@1/*` — a fresh scene whose `results.sparse` is the
/// decoded point set (plus `results.mesh` when the file also carried faces; dropping them would be
/// silent loss). `results.dense` is never produced: a dense cloud is distinguished in this schema by
/// per-point confidence, which neither PLY's vertex properties as `SemioMeshSnapshot` models them nor
/// any other foreign mesh dialect carries.
pub struct PlyIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for PlyIntoRemodeling {
    const FROM: Dialect = PLY_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(b"ply") => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "ply→remodeling: expected a binary ply payload".to_string(), diagnostics: Vec::new() });
        };
        let ply = decode_ply(bytes).map_err(|error| IoError { message: format!("ply→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromPly::deserialize(&ply)).map_err(|error| IoError { message: format!("ply→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_cloud(&semio).map_err(|reason| IoError { message: format!("ply→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
