//! 🕸️ DAG document text carries its marker and five literal Graph child address fields.

use crate::DagSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ Canonical plugin document; its marker belongs to this artifact owner.

/// 📖️ Parses `.dag` DSL text into a `DagSnapshot`.
pub fn parse_dsl(text: &str) -> Result<DagSnapshot, semio_framework_diagnostic::TextError> {
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DagSnapshot` back to `.dag` DSL text.
pub fn print_dsl(document: &DagSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🔖️HandcraftedArtifactDsl
impl store::ArtifactDsl for DagSnapshot {
    const EXTENSION: &'static str = "dag";
    fn envelope_id() -> &'static str {
        "dag.dag"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if let Ok((envelope, _)) = store::semio_format::split_text_preamble(text) {
            if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) {
                return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DAG text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1)));
            }
        }
        let body = store::semio_format::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        let snapshot = Self::__dsl_from_record(&record)?;
        snapshot.validate().map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid DAG envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;
