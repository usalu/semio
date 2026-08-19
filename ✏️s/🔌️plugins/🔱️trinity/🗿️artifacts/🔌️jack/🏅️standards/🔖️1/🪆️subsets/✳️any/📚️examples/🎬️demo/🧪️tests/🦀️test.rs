#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::jack::standards::v1::subsets::any::schema::inferences::JackInference;
    use protocol::Inference;
    assert_eq!(
        JackInference::infer(&crate::artifacts::jack::JackSnapshot::default()),
        JackInference::default()
    );
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::jack::standards::v1::subsets::any::schema::inferences::JackInference;
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let projection = crate::artifacts::jack::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(JackInference::infer(&projection), JackInference::infer(&projection));
}
