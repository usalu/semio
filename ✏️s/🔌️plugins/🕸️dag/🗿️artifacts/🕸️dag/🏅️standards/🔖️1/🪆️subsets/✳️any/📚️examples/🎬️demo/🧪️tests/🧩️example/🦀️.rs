#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = crate::examples::demo::PRIMARY_TEXT;
    assert!(text.len() > 8);
}

/// 🗂️ Every bundled example parses under the current DSL grammar (braced `List<Record>` items) and prints back to its
/// committed bytes, so a grammar change that strands an asset fails here instead of in every test that loads the demo.
#[semio_framework_async_macros::async_test]
async fn every_bundled_example_parses_and_prints_canonically() {
    let examples = crate::standards::v1::subsets::any::examples();
    assert!(!examples.is_empty(), "the dag subset bundles at least the demo example");
    for example in examples {
        let text = example.document();
        let parsed = crate::standards::v1::subsets::any::schema::snapshot::parse_dag_dsl(&text).unwrap_or_else(|error| panic!("bundled example {} must parse: {error}", example.id()));
        assert_eq!(crate::standards::v1::subsets::any::schema::snapshot::print_dag_dsl(&parsed), text, "bundled example {} must be the codec's own canonical output", example.id());
    }
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = crate::examples::demo::PRIMARY_TEXT;
    let snapshot = <crate::DagSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::DagInference::infer(&snapshot);
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::DagInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::DagInference::infer(&crate::DagSnapshot::default()), crate::standards::v1::subsets::any::schema::inferences::DagInference::default(),);
}
//#endregion 🧪️InferenceLaws
