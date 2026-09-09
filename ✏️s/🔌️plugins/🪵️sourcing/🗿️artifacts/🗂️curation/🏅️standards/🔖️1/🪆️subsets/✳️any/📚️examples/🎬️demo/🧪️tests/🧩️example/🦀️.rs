#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::CurationSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::CurationInference::infer(&snapshot);
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::CurationInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::standards::v1::subsets::any::schema::inferences::CurationInference::infer(&crate::CurationSnapshot::default()),
        crate::standards::v1::subsets::any::schema::inferences::CurationInference::default(),
    );
}

#[semio_framework_async_macros::async_test]
async fn entries_census_the_demo_fixtures_stock_catalog() {
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::CurationSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::CurationInference::infer(&snapshot);
    assert_eq!(inference.entries.stock_count, snapshot.stock_extra.len() as u32);
    assert_eq!(inference.entries.entry_count, snapshot.curated.len() as u32);
}
//#endregion 🧪️InferenceLaws
