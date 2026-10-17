//! 📜️ `trinity.rewrite.rule` artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use pack::snapshot::ArtifactDsl;

/// 📄️ The bundled Nakagin `label-core` rewrite rule, handcrafted in the `.rewriting` DSL — mirrors the
/// `trinity-rewriting` app's own real default rule over a trimmed two-node/one-edge slice of the
/// bundled `🔱️nakagin-capsule-tower.trinity` working-graph.
pub const NAKAGIN_LABEL_CORE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.rewriting` DSL text into a `RewritingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RewritingSnapshot, semio_framework_diagnostic::TextError> {
    <RewritingSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RewritingSnapshot` back to `.rewriting` DSL text.
pub fn print_dsl(document: &RewritingSnapshot) -> String {
    ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RewritingSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::standards::v1::subsets::any::schema::model::LayoutPoint;
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_graph_manifest::PropertyValue;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl pack::snapshot::ArtifactDsl for RewritingSnapshot {
    const EXTENSION: &'static str = "rewriting";
    fn envelope_id() -> &'static str {
        "trinity.rewriting"
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

/// 📤️ Renders the declared typed rule, Jack child handle, exact words and keyed maps as JSON.
pub fn encode_rewriting_snapshot_json(snapshot: &RewritingSnapshot) -> Result<String, semio_framework_value::ValueError> {
    let value = json::convert(semio_framework_value::ToValue::to_value(snapshot), false)?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))
}

/// 📥️ The inverse of [`encode_rewriting_snapshot_json`] — decodes those committed specification
/// vectors into real [`RewritingSnapshot`] values, so `mutate-rewriting-1`'s adapter reads the committed
/// snapshot rather than re-declaring it as a Rust literal beside it. Reaching a JSON library from that
/// adapter is impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_rewriting_snapshot_json(text: &str) -> Result<RewritingSnapshot, semio_framework_value::ValueError> {
    let value = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
    let value = json::convert(semio_framework_pack_json::to_dsl_value(&value), true)?;
    <RewritingSnapshot as semio_framework_value::FromValue>::from_value(value)
}

/// 📝️ Parses `.rewriting.dsl.semio` text into a [`RewritingSnapshot`] — a named, non-async pass-through
/// of this type's own handcrafted `pack::snapshot::ArtifactDsl` impl above, whose trait and error type are
/// both unnameable outside this crate, so `mutate-rewriting-1`'s `identity-round-trip` scenario
/// reaches the real committed artifact
/// (`../../🖼️assets/🎬️demo/🗣️.dsl.semio`) through this instead.
pub fn parse_rewriting_dsl(text: &str) -> Result<RewritingSnapshot, String> {
    <RewritingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders the declared typed Rewriting document through its owned DSL facet.
pub fn print_rewriting_dsl(snapshot: &RewritingSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;
