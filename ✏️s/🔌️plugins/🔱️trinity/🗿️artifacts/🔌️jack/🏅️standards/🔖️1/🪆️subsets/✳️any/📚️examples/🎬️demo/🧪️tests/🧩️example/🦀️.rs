#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::JackInference;
    use protocol::Inference;
    assert_eq!(JackInference::infer(&crate::JackSnapshot::default()), JackInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::JackInference;
    use protocol::Inference;
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    let projection = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example dsl parses");
    assert_eq!(JackInference::infer(&projection), JackInference::infer(&projection));
}
