use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf writes.
pub const LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.las@1.0/*` — `results.dense` when a dense run produced
/// one, else `results.sparse`, through stdio's real `SemioMeshToLas` serializer +
/// `las::engine::encode_las`. LAS is a point format with no topology, so `results.mesh` is NOT a
/// candidate here (that is what the `🧱️ply`/`🗿️obj`/`🔺️stl`/`🧊️gltf` leaves are for) and per-point
/// classification/confidence have no `SemioMeshSnapshot` channel to travel in: `IoFidelity::Lossy`.
pub struct RemodelingIntoLas;

impl Serializer<RemodelingSnapshot> for RemodelingIntoLas {
    const INTO: Dialect = LAS_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_cloud_semio(from).map_err(|reason| IoError { message: format!("remodeling→las: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let bytes = io_root::semio_mesh_to_las_bytes(&semio).map_err(|error| IoError { message: format!("remodeling→las: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
