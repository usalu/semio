//! 📜️ Drawing artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::DrawingSnapshot;

/// 🗄️ The Semio emblem example fixture, handcrafted in `drawing`'s DSL (`store::ArtifactDsl`).
pub const SEMIO_DRAW_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ P6 handcrafted `ArtifactDsl` (derive no longer emits this trait) — relocated from
/// `🧬️schema/📸️snapshot/🦀️.rs` (design.md §1 CORRECTION: the native codec is one
/// bidirectional thing and sits unsplit at `🚪️io/<facet>/<representation>/`; `🧬️schema` keeps only
/// the `DrawingSnapshot` struct and its `Default` impl).
impl store::ArtifactDsl for DrawingSnapshot {
    const EXTENSION: &'static str = "drawing";
    fn envelope_id() -> &'static str {
        "drawing.drawing"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📖️ Parses `.drawing` DSL text into a `DrawingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<DrawingSnapshot, store::TextError> {
    <DrawingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DrawingSnapshot` back to `.drawing` DSL text.
pub fn print_dsl(document: &DrawingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
