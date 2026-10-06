//! 📜️ S Home launcher artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::SHomeSnapshot;

/// 📦️ The `home` app's "default" example, embedded at compile time as handcrafted `.shome` DSL text —
/// exercised by the round-trip test below. Not yet wired into a `.example(...)` manifest registration
/// (the `home` UI manifest has none today).
pub const HOME_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.shome` DSL text into an `SHomeSnapshot`.
pub fn parse_dsl(text: &str) -> Result<SHomeSnapshot, semio_framework_diagnostic::TextError> {
    <SHomeSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `SHomeSnapshot` back to `.shome` DSL text.
pub fn print_dsl(document: &SHomeSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SHomeSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::S_HOME_DOCUMENT_SCHEMA;
use ::semio_framework_schema::ArtifactSchema;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for SHomeSnapshot {
    const EXTENSION: &'static str = "shome";
    fn envelope_id() -> &'static str {
        "s.home"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
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
use crate::S_HOME_DOCUMENT_SCHEMA;
use ::semio_framework_schema::ArtifactSchema;

/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `s_home_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn s_home_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <SHomeSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <SHomeSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <SHomeSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <SHomeSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <SHomeSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <SHomeSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = semio_framework_pack_json::json!({
        "parsed": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&parsed)),
        "reparsed": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&reparsed)),
        "packDecoded": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&unpacked)),
        "canonicalText": canonical,
        "canonicalTextAgain": canonical_again,
    });
    Ok(report.to_string())
}
}
pub use snapshot_wire_codec::*;
