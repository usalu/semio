//! 📜️ Sourcing curate artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::curate::CurateDocument;

/// 📄️ The demo-stock example, handcrafted in the `.curate` DSL.
pub const DEMO_STOCK_TEXT: &str = include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.sourcing.curate.demo-stock-curate.dsl.semio");

/// 📄️ The empty-curation example, handcrafted in the `.curate` DSL.
pub const EMPTY_CURATION_TEXT: &str = include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.sourcing.curate.empty-curation-curate.dsl.semio");

/// 📖️ Parses `.curate` DSL text into a `CurateDocument`.
pub fn parse_dsl(text: &str) -> Result<CurateDocument, store::TextError> {
    <CurateDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CurateDocument` back to `.curate` DSL text.
pub fn print_dsl(document: &CurateDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_stock_example_dsl_round_trips() {
        let document = parse_dsl(DEMO_STOCK_TEXT).expect("parse demo-stock example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn empty_curation_example_dsl_round_trips() {
        let document = parse_dsl(EMPTY_CURATION_TEXT).expect("parse empty-curation example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn curate_document_dsl_round_trips_a_mesh_kind_and_a_curated_selection() {
        use crate::artifacts::curate::{GeometryRecipe, ObjectKind};

        let mut document = CurateDocument {
            stock: vec![ObjectKind {
                id: "beam-mesh-custom".into(),
                name: "Custom \"Beam\" \\ Mesh".into(),
                module_id: "beams".into(),
                typology_path: vec!["beams".into(), "steel".into()],
                availability: 5,
                geometry: Box::new(GeometryRecipe::Mesh { positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], normals: vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0], indices: vec![0, 1, 2] }),
            }],
            ..Default::default()
        };
        document.curated = vec![crate::artifacts::curate::CuratedItem { object_id: "beam-mesh-custom".into(), count: 2 }];
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
