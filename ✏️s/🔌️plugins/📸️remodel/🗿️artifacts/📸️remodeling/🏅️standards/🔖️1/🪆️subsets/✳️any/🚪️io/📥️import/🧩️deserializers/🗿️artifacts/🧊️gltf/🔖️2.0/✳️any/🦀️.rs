use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework::io_schema::Confidence,semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use semio_framework_plugin::{ ArtifactDeserializer};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
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
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "gltf→remodeling: expected a binary glb payload".to_string())));
        };
        let gltf = decode_glb(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("gltf→remodeling: decode failed: {error}"))))?;
        let semio = ::semio_framework_async::poll::resolve_ready(SemioMeshFromGltf::deserialize(&gltf)).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("gltf→remodeling: {error}"))))?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("gltf→remodeling: {reason}"))))?;
        Ok(IoOutcome::clean(scene))
    }
}
