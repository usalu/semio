//! 📜️ Sourcing curation artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::curation::CurationSnapshot;

/// 📄️ The demo-stock example, handcrafted in the `.curation` DSL.
pub const DEMO_STOCK_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    #[ignore = "manual fixture export"]
    async fn export_demo_stock_fixture_text() {
        let document = crate::artifacts::curation::curation_snapshot_from_stock(crate::artifacts::curation::schema::demo_stock(), Vec::new());
        println!("{}", store::ArtifactDsl::print_dsl(&document));
    }

    #[semio_framework_async_macros::async_test]
    async fn demo_stock_example_dsl_round_trips() {
        let document = parse_dsl(DEMO_STOCK_TEXT).expect("parse demo-stock example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn demo_stock_example_preserves_authored_content_against_json_oracle() {
        let expected: Vec<crate::artifacts::curation::ObjectKind> = serde_json::from_str(include_str!("../../../📚️examples/🎬️demo/🧪️expected-stock.json")).unwrap();
        let document = parse_dsl(DEMO_STOCK_TEXT).expect("authored stock must parse without an empty fallback");
        assert_eq!(crate::artifacts::curation::stock_of(&document), expected);
        assert_eq!(crate::artifacts::curation::stock_of(&crate::artifacts::curation::schema::default_document()), expected);
        assert_eq!(crate::artifacts::curation::schema::demo_stock(), expected);
        assert_eq!(document.catalog, crate::artifacts::curation::catalog_child_handle(&expected));
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_curation_example_dsl_round_trips() {
        let document = parse_dsl(EMPTY_CURATION_TEXT).expect("parse empty-curation example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn curation_document_dsl_round_trips_a_mesh_kind_and_a_curated_selection() {
        use crate::artifacts::curation::{GeometryRecipe, ObjectKind};

        let stock = vec![ObjectKind {
            id: "beam-mesh-custom".into(),
            name: "Custom \"Beam\" \\ Mesh".into(),
            module_id: "beams".into(),
            typology_path: vec!["beams".into(), "steel".into()],
            availability: 5,
            geometry: Box::new(GeometryRecipe::Mesh { positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], normals: vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0], indices: vec![0, 1, 2] }),
        }];
        let document = crate::artifacts::curation::curation_snapshot_from_stock(stock, vec![crate::artifacts::curation::CuratedItem { object_id: "beam-mesh-custom".into(), count: 2 }]);
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
