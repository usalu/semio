#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::rewriting::standards::v1::subsets::any::schema::inferences::RewritingInference;
    use protocol::Inference;
    assert_eq!(RewritingInference::infer(&crate::artifacts::rewriting::RewritingSnapshot::default()), RewritingInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::rewriting::standards::v1::subsets::any::schema::inferences::RewritingInference;
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    let projection = crate::artifacts::rewriting::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(RewritingInference::infer(&projection), RewritingInference::infer(&projection));
}
