use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_framework_plugin::{StandardId, SubsetId};
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
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "stl→remodeling: expected a text ascii-stl payload".to_string(), diagnostics: Vec::new() });
        };
        let stl = decode_stl_ascii(text).map_err(|error| IoError { message: format!("stl→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromStl::deserialize(&stl)).map_err(|error| IoError { message: format!("stl→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError { message: format!("stl→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
