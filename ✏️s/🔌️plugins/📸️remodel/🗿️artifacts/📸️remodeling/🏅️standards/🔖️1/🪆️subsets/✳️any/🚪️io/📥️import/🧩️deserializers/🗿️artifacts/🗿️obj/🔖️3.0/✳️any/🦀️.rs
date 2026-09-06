use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::engine::decode_obj;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;

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
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "obj→remodeling: expected a text obj payload".to_string(), diagnostics: Vec::new() });
        };
        let obj = decode_obj(text).map_err(|error| IoError { message: format!("obj→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromObj::deserialize(&obj)).map_err(|error| IoError { message: format!("obj→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError { message: format!("obj→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
