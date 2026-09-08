
use super::*;

#[semio_framework_async_macros::async_test]
#[ignore = "manual fixture export"]
async fn export_demo_stock_fixture_text() {
    let document = crate::curation_snapshot_from_stock(&crate::schema::demo_stock(), Vec::new());
    println!("{}", store::ArtifactDsl::print_dsl(&document));
}

#[semio_framework_async_macros::async_test]
async fn demo_stock_example_dsl_round_trips() {
    let document = parse_dsl(DEMO_STOCK_TEXT).expect("parse demo-stock example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn demo_stock_example_preserves_authored_content_against_json_oracle() {
    let expected: Vec<crate::ObjectKind> = dsl::json::from_json_str(include_str!("../../../../../📚️examples/🎬️demo/📦️expected-stock.json")).unwrap();
    let document = parse_dsl(DEMO_STOCK_TEXT).expect("authored stock must parse without an empty fallback");
    assert_eq!(crate::stock_of(&document), expected);
    assert_eq!(crate::stock_of(&crate::schema::default_document()), expected);
    assert_eq!(crate::schema::demo_stock(), expected);
    assert_eq!(document.catalog, crate::catalog_child_handle(&expected));
}

#[semio_framework_async_macros::async_test]
async fn empty_curation_example_dsl_round_trips() {
    let document = parse_dsl(EMPTY_CURATION_TEXT).expect("parse empty-curation example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn curation_document_dsl_round_trips_a_mesh_kind_and_a_curated_selection() {
    use crate::{GeometryRecipe, ObjectKind};

    let stock = vec![ObjectKind {
        id: "beam-mesh-custom".into(),
        name: "Custom \"Beam\" \\ Mesh".into(),
        module_id: "beams".into(),
        typology_path: vec!["beams".into(), "steel".into()],
        availability: 5,
        geometry: Box::new(GeometryRecipe::Mesh { positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], normals: vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0], indices: vec![0, 1, 2] }),
    }];
    let document = crate::curation_snapshot_from_stock(&stock, vec![crate::CuratedItem { object_id: "beam-mesh-custom".into(), count: 2 }]);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
