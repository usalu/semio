//! 📜️ Sourcing curate artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::curate::CurateSnapshot;

/// 📄️ The demo-stock example, handcrafted in the `.curate` DSL.
pub const DEMO_STOCK_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📄️ The empty-curation example — empty stock and curated table. `catalog`'s handle is
/// content-addressed from an empty stock (`catalog_child_handle(&[])`, same value
/// `CurateSnapshot::default()` mints), regenerated via the hand-rolled codec, not hand-transcribed.
pub const EMPTY_CURATION_TEXT: &str = r#"semio curate.curate.dsl v1
catalog=child_id=catalog-7904dd65836c8ff4 target="catalog-7904dd65836c8ff4!s.stdio.semio@v1/kit" stock-extra=[ ]
curated [object-id:REF count:UINT] {
}
"#;

/// 📖️ Parses `.curate` DSL text into a `CurateSnapshot`.
pub fn parse_dsl(text: &str) -> Result<CurateSnapshot, store::TextError> {
    <CurateSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CurateSnapshot` back to `.curate` DSL text.
pub fn print_dsl(document: &CurateSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    #[ignore = "manual fixture export"]
    async fn export_demo_stock_fixture_text() {
        let document = crate::artifacts::curate::curate_snapshot_from_stock(crate::artifacts::curate::schema::demo_stock(), Vec::new());
        println!("{}", store::ArtifactDsl::print_dsl(&document));
    }

    #[semio_framework_async_macros::async_test]
    async fn demo_stock_example_dsl_round_trips() {
        let document = parse_dsl(DEMO_STOCK_TEXT).expect("parse demo-stock example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_curation_example_dsl_round_trips() {
        let document = parse_dsl(EMPTY_CURATION_TEXT).expect("parse empty-curation example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn curate_document_dsl_round_trips_a_mesh_kind_and_a_curated_selection() {
        use crate::artifacts::curate::{GeometryRecipe, ObjectKind};

        let stock = vec![ObjectKind {
            id: "beam-mesh-custom".into(),
            name: "Custom \"Beam\" \\ Mesh".into(),
            module_id: "beams".into(),
            typology_path: vec!["beams".into(), "steel".into()],
            availability: 5,
            geometry: Box::new(GeometryRecipe::Mesh { positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], normals: vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0], indices: vec![0, 1, 2] }),
        }];
        let document = crate::artifacts::curate::curate_snapshot_from_stock(stock, vec![crate::artifacts::curate::CuratedItem { object_id: "beam-mesh-custom".into(), count: 2 }]);
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
