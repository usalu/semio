//! 📜️ EnergyModel artifact — textual document grammar surface + laws.

use crate::artifacts::model::EnergyModelSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ The bundled demo document.
pub const SEMIO_ENERGY_MODEL_EXAMPLE_TEXT: &str =
    include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.energy` DSL text into an `EnergyModelSnapshot`.
pub fn parse_dsl(text: &str) -> Result<EnergyModelSnapshot, store::TextError> {
    <EnergyModelSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `EnergyModelSnapshot` back to `.energy` DSL text.
pub fn print_dsl(document: &EnergyModelSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semio_example_dsl_round_trips() {
        let document = parse_dsl(SEMIO_ENERGY_MODEL_EXAMPLE_TEXT).expect("parse semio example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
