//! 🚪️ dag -> png — foreign `Serializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling
//! `Deserializer`'s best-effort `serde_json` structural reinterpretation — a raster image has no
//! node/edge/graph concept, so this hop is `IoFidelity::Lossy`.

use crate::artifacts::dag::DagSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub fn serialize(from: &DagSnapshot) -> Result<PngSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}

pub struct DagIntoPng;

impl Serializer<DagSnapshot> for DagIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &DagSnapshot) -> IoResult<IoPayload> {
        let png = serialize(from).map_err(|error| IoError { message: format!("DagIntoPng: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<PngSnapshot as store::ArtifactPack>::encode_pack(&png))))
    }
}
