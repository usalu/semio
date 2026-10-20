//! 📜️ Drawing artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::snapshot::DrawingSnapshot;

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ P6 handcrafted `ArtifactDsl` (derive no longer emits this trait) — relocated from
/// `🧬️schema/📸️snapshot/🦀️.rs` (design.md §1 CORRECTION: the native codec is one
/// bidirectional thing and sits unsplit at `🚪️io/<facet>/<representation>/`; `🧬️schema` keeps only
/// the `DrawingSnapshot` struct and its `Default` impl).
impl pack::snapshot::ArtifactDsl for DrawingSnapshot {
    const EXTENSION: &'static str = "drawing";
    fn envelope_id() -> &'static str {
        "drawing.drawing"
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
//#endregion 🔖️HandcraftedArtifactDsl

/// 📖️ Parses `.drawing` DSL text into a `DrawingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<DrawingSnapshot, semio_framework_diagnostic::TextError> {
    <DrawingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DrawingSnapshot` back to `.drawing` DSL text.
pub fn print_dsl(document: &DrawingSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}


#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::snapshot::DrawingSnapshot;


/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_drawing_dsl(text: &str) -> Result<String, String> {
    use pack::snapshot::ArtifactDsl;
    let parsed = <DrawingSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed drawing example does not parse: {error:?}"))?;
    let printed = <DrawingSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <DrawingSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted drawing document does not parse: {error:?}"))?;
    let report = semio_framework_value::DslValue::object([("printed".to_string(), semio_framework_value::ToValue::to_value(&printed)), ("snapshot".to_string(), semio_framework_value::ToValue::to_value(&parsed)), ("reparsed".to_string(), semio_framework_value::ToValue::to_value(&reparsed))]);
    Ok(semio_framework_pack_json::to_json_string(&report))
}
}
pub use mutations_codec::*;

