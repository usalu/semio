use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf writes.
pub const PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.ply@1.0/*`. PLY is the one target that carries BOTH a
/// surface and a bare point set, so it exports `results.mesh` when a bounded durable mesh resolves
/// and falls back to `results.dense`/`results.sparse` otherwise. Real codec throughout: stdio's
/// `SemioMeshToPly` serializer + `ply::engine::encode_ply`. `IoFidelity::Lossy` because everything
/// else in the scene — streams, calibration, GCPs, params, job state, QC, geo products — has no PLY
/// representation at all.
pub struct RemodelingIntoPly;

impl Serializer<RemodelingSnapshot> for RemodelingIntoPly {
    const INTO: Dialect = PLY_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_or_cloud_semio(from).map_err(|reason| IoError { message: format!("remodeling→ply: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let bytes = io_root::semio_mesh_to_ply_bytes(&semio).map_err(|error| IoError { message: format!("remodeling→ply: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
