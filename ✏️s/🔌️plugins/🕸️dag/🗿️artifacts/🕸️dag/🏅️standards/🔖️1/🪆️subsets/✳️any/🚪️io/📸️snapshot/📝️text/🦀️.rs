//! 📜️ DAG document text codec. The graph snapshot owns the canonical node/edge wire grammar;
//! the artifact reconstructs its composed child owner when decoding that graph.

use crate::DagSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ Canonical plugin document; its marker belongs to this artifact owner.

/// 📖️ Parses `.dag` DSL text into a `DagSnapshot`.
pub fn parse_dsl(text: &str) -> Result<DagSnapshot, store::TextError> {
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
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if let Ok((envelope, _)) = store::semio_format::split_text_preamble(text) {
            if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) {
                return Err(store::TextError::new("DAG text envelope mismatch", dsl::TextSpan::at(1, 1)));
            }
        }
        let graph = <semio_framework_artifact_infinite_dag::DagSnapshot as store::ArtifactDsl>::parse_dsl(text)?;
        let snapshot: Self = graph.into();
        snapshot.validate().map_err(|message| store::TextError::new(message, dsl::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let graph = semio_framework_artifact_infinite_dag::DagSnapshot::from(self);
        store::ArtifactDsl::print_dsl(&graph)
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
