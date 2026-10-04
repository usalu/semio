use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = HtmlSnapshot::default();
    assert_eq!(HtmlInference::infer(&snapshot).expect("valid materialized inference fixture"), HtmlInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(HtmlInference::infer(&HtmlSnapshot::default()).expect("valid materialized inference fixture"), HtmlInference::default());
}
