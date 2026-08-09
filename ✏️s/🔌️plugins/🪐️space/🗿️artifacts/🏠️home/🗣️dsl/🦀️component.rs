//! 📜️ S Home launcher artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::home::SHomeSnapshot;

/// 📦️ The `home` app's "default" example, embedded at compile time as handcrafted `.shome` DSL text —
/// exercised by the round-trip test below. Not yet wired into a `.example(...)` manifest registration
/// (the `home` UI manifest has none today).
pub const HOME_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.shome` DSL text into an `SHomeSnapshot`.
pub fn parse_dsl(text: &str) -> Result<SHomeSnapshot, store::TextError> {
    <SHomeSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `SHomeSnapshot` back to `.shome` DSL text.
pub fn print_dsl(document: &SHomeSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_dsl_round_trips_default_and_populated_documents() {
        store::os_store::test_support::assert_dsl_round_trip(&SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 });
        store::os_store::test_support::assert_dsl_round_trip(&SHomeSnapshot { schema: "s.home".into(), catalog_generation: 42 });
    }

    #[test]
    fn home_dsl_round_trips_bundled_default_example() {
        let document = parse_dsl(HOME_EXAMPLE_TEXT).expect("parse default example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
