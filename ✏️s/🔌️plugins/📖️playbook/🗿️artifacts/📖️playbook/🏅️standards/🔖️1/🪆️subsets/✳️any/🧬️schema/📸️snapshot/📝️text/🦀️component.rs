//! 📜️ Playbook artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::playbook::PlaybookSnapshot;

/// 📄️ The `facade-generator` example spec, handcrafted in the `.playbook` DSL.
pub const FACADE_GENERATOR_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.playbook` DSL text into a `PlaybookSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PlaybookSnapshot, store::TextError> {
    <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PlaybookSnapshot` back to `.playbook` DSL text.
pub fn print_dsl(document: &PlaybookSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::empty_playbook_snapshot;

    #[test]
    fn dsl_round_trips_the_empty_snapshot() {
        let document = empty_playbook_snapshot();
        let text = print_dsl(&document);
        assert_eq!(parse_dsl(&text).expect("parse"), document);
    }

    #[test]
    fn facade_generator_example_dsl_round_trips() {
        let document = parse_dsl(FACADE_GENERATOR_EXAMPLE_TEXT).expect("parse example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn facade_generator_example_matches_the_handcrafted_spec() {
        let document = parse_dsl(FACADE_GENERATOR_EXAMPLE_TEXT).expect("parse example");
        assert!(!document.steps.is_empty());
        assert_eq!(print_dsl(&document).trim_end(), FACADE_GENERATOR_EXAMPLE_TEXT.trim_end());
    }

}
//#endregion 🧪️Tests
