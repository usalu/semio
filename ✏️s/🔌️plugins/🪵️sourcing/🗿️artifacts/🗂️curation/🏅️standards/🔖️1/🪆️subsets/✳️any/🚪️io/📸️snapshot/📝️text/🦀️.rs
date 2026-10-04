//! 📜️ Sourcing curation artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::CurationSnapshot;

/// 📄️ The `demo` example, handcrafted in the `.curation` DSL.
pub const DEMO_STOCK_TEXT: &str = crate::examples::demo::PRIMARY_TEXT;

/// 📄️ The empty curation the shell's "no example" loads — empty stock and curated table. `catalog`'s
/// handle is content-addressed from an empty stock (`catalog_child_handle(&[])`, same value
/// `CurationSnapshot::default()` mints).
pub const EMPTY_CURATION_TEXT: &str = r#"semio curation.curation.dsl v1
catalog=child_id=catalog-4f53cda18c2baa0c target=artifact-id=catalog-4f53cda18c2baa0c artifact-kind=s.stdio.semio standard=v1 subset=kit stock-extra=[ ]
curated [object-id:REF count:UINT] {
}
"#;

/// 📖️ Parses `.curation` DSL text into a `CurationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<CurationSnapshot, semio_framework_diagnostic::TextError> {
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
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = if text.trim_start().starts_with("semio ") {
            let (envelope, body) = store::semio_format::split_text_preamble(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Curation text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1))); }
            body
        } else { text };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        let result = Self::__dsl_from_record(&record)?;
        result.validate().map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(result)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), store::semio_format::Component::Dsl, 1).expect("Curation envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
/// 📥 Parses the document's native text representation.
pub fn parse_curation_dsl(text: &str) -> Result<CurationSnapshot, String> { parse_dsl(text).map_err(|error| format!("{error:?}")) }
/// 📤 Emits the document's native text representation.
pub fn print_curation_dsl(snapshot: &CurationSnapshot) -> String { print_dsl(snapshot) }
