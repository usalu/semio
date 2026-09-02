//! 🚪️ dag -> svg — foreign `Serializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Best-effort `serde_json`
//! structural reinterpretation into `SvgSnapshot`'s own shape — not a real svg-shape export, and
//! does NOT invert the sibling `Deserializer`'s text-content bridge, so this hop is
//! `IoFidelity::Lossy`.

use crate::artifacts::dag::DagSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub async fn serialize(from: &DagSnapshot) -> Result<SvgSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}

pub struct DagIntoSvg;

impl Serializer<DagSnapshot> for DagIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &DagSnapshot) -> IoResult<IoPayload> {
        let svg = serialize(from).map_err(|error| IoError { message: format!("DagIntoSvg: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<SvgSnapshot as store::ArtifactPack>::encode_pack(&svg))))
    }
}
