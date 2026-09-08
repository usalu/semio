use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_gltf::standards::v2_0::engine::decode_glb;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf;

/// 🎯️ The foreign dialect this leaf reads.
pub const GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.gltf@2.0/*` → `s.remodel.remodeling@1/*` — the first mesh primitive of the decoded GLB
/// becomes a real durable mesh asset. Scene graph, nodes, animations, cameras and materials are
/// dropped (this document has no slot for any of them): `IoFidelity::Lossy`.
pub struct GltfIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for GltfIntoRemodeling {
    const FROM: Dialect = GLTF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(b"glTF") => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "gltf→remodeling: expected a binary glb payload".to_string(), diagnostics: Vec::new() });
        };
        let gltf = decode_glb(bytes).map_err(|error| IoError { message: format!("gltf→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromGltf::deserialize(&gltf)).map_err(|error| IoError { message: format!("gltf→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError { message: format!("gltf→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
