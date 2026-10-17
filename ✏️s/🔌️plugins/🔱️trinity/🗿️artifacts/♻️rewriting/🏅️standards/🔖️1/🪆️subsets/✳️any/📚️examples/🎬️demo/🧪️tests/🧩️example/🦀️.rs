#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = runtime_fixture_text("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::RewritingInference;
    use protocol::Inference;
    assert_eq!(RewritingInference::infer(&crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot::default()).expect("valid materialized inference fixture"), RewritingInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::RewritingInference;
    use protocol::Inference;
    let text = runtime_fixture_text("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let projection = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).expect("example dsl parses");
    assert_eq!(RewritingInference::infer(&projection).expect("valid materialized inference fixture"), RewritingInference::infer(&projection).expect("valid materialized inference fixture"));
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
