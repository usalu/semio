//! 📜️ EnergyModel artifact — textual document grammar surface + laws.

use crate::EnergyModelSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ The bundled demo document.
pub const SEMIO_ENERGY_MODEL_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.energy` DSL text into an `EnergyModelSnapshot`.
pub fn parse_dsl(text: &str) -> Result<EnergyModelSnapshot, semio_framework_diagnostic::TextError> {
    <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `EnergyModelSnapshot` back to `.energy` DSL text.
pub async fn print_dsl(document: &EnergyModelSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type EnergyModelSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{energy_snapshot_with_state, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use crate::standards::v1::subsets::any::io::binary::snapshot::{EnergyModelPackRecord};
/// 🖨️ The derived text body: the same `EnergyModelPackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &EnergyModelSnapshot) -> String {
    semio_framework_dsl_record::print(&EnergyModelPackRecord::from_snapshot(snapshot).__dsl_to_record(), &EnergyModelPackRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `EnergyModelPackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<EnergyModelSnapshot, semio_framework_diagnostic::TextError> {
    let record = semio_framework_dsl_record::parse(body, &EnergyModelPackRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
    EnergyModelPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))
}

impl store::ArtifactDsl for EnergyModelSnapshot {
    const EXTENSION: &'static str = "energy";
    fn envelope_id() -> &'static str {
        "energy.model"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{energy_snapshot_with_state, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;

/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `energy_model_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn energy_model_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <EnergyModelSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = semio_framework_pack_json::object([
        ("parsed".to_string(), semio_framework_pack_json::from_dsl_value(&parsed.to_value())),
        ("reparsed".to_string(), semio_framework_pack_json::from_dsl_value(&reparsed.to_value())),
        ("packDecoded".to_string(), semio_framework_pack_json::from_dsl_value(&unpacked.to_value())),
        ("canonicalText".to_string(), semio_framework_pack_json::Value::String(canonical)),
        ("canonicalTextAgain".to_string(), semio_framework_pack_json::Value::String(canonical_again)),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use snapshot_wire_codec::*;
