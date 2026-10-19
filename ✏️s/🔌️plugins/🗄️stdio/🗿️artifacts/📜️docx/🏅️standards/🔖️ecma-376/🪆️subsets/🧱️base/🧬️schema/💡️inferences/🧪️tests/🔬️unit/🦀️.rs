use semio_s_artifact_stdio_docx_base_inferences::standards::v_ecma_376::subsets::base::schema::inferences::*;
use protocol::Inference;
#[cfg(test)] use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DocxSnapshot::default();
    assert_eq!(DocxInference::infer(&snapshot).expect("valid materialized inference fixture"), DocxInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DocxInference::infer(&DocxSnapshot::default()).expect("valid materialized inference fixture"), DocxInference::default());
}
