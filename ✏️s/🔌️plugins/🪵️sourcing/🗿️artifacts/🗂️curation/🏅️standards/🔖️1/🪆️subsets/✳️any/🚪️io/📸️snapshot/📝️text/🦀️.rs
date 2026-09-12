//! 📜️ Sourcing curation artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::CurationSnapshot;

/// 📄️ The demo-stock example, handcrafted in the `.curation` DSL.
pub const DEMO_STOCK_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📄️ The empty-curation example — empty stock and curated table. `catalog`'s handle is
/// content-addressed from an empty stock (`catalog_child_handle(&[])`, same value
/// `CurationSnapshot::default()` mints), regenerated via the hand-rolled codec, not hand-transcribed.
pub const EMPTY_CURATION_TEXT: &str = r#"semio curation.curation.dsl v1
catalog=child_id=catalog-7904dd65836c8ff4 target="catalog-7904dd65836c8ff4!s.stdio.semio@v1/kit" stock-extra=[ ]
curated [object-id:REF count:UINT] {
}
"#;

/// 📖️ Parses `.curation` DSL text into a `CurationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<CurationSnapshot, store::TextError> {
    <CurationSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CurationSnapshot` back to `.curation` DSL text.
pub fn print_dsl(document: &CurationSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

impl store::ArtifactDsl for CurationSnapshot {
    const EXTENSION: &'static str = "curation";
    fn envelope_id() -> &'static str { "curation.curation" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = if text.trim_start().starts_with("semio ") {
            let (envelope, body) = store::semio_format::split_text_preamble(text).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))?;
            if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) { return Err(store::TextError::new("Curation text envelope mismatch", dsl::TextSpan::at(1, 1))); }
            body
        } else { text };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        let result = Self::__dsl_from_record(&record)?;
        result.validate().map_err(|message| store::TextError::new(message, dsl::TextSpan::at(1, 1)))?;
        Ok(result)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), store::semio_format::Component::Dsl, 1).expect("Curation envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
/// 📥 Parses the document's native text representation.
pub fn parse_curation_dsl(text: &str) -> Result<CurationSnapshot, String> { parse_dsl(text).map_err(|error| format!("{error:?}")) }
/// 📤 Emits the document's native text representation.
pub fn print_curation_dsl(snapshot: &CurationSnapshot) -> String { print_dsl(snapshot) }
