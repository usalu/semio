//! 📜️ Remodeling artifact — the textual document grammar surface (`dsl`) and its laws.
//!
//! 🔀️ Deviation from every sibling plugin: remodeling ships no handcrafted `.remodeling` example fixture
//! file, so there is no `REMODELING_EXAMPLE_TEXT` constant here — its single "default" example is
//! generated at runtime from `default_remodeling_scene().print_dsl()` (see `create_remodeling_app`).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

/// 📖️ Parses `.remodeling` DSL text into a `RemodelingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RemodelingSnapshot, semio_framework_diagnostic::TextError> {
    <RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RemodelingSnapshot` back to `.remodeling` DSL text.
pub fn print_dsl(scene: &RemodelingSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(scene)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RemodelingSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::standards::v1::subsets::any::schema::snapshot::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionParams, ReconstructionResults, RemodelingAssetChild, RemodelingDurableArtifactStore, REMODELING_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl pack::snapshot::ArtifactDsl for RemodelingSnapshot {
    const EXTENSION: &'static str = "remodeling";
    fn envelope_id() -> &'static str {
        "remodeling.remodeling"
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
#[cfg(test)] pub use snapshot_codec::*;

mod mutations_codec {
use crate::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_remodeling_dsl(text: &str) -> Result<String, String> {
    use pack::snapshot::ArtifactDsl;
    let parsed = <RemodelingSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed remodeling example does not parse: {error:?}"))?;
    let printed = <RemodelingSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <RemodelingSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted remodeling document does not parse: {error:?}"))?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("printed".to_string(), semio_framework_pack_json::Value::String(printed)),
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&parsed))),
        ("reparsed".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&reparsed))),
    ])))
}
}
pub use mutations_codec::*;
