use semio_s_artifact_stdio_xlsx_base_inferences::standards::v_ecma_376::subsets::base::schema::inferences::*;
use protocol::Inference;
#[cfg(test)] use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = XlsxSnapshot::default();
    assert_eq!(XlsxInference::infer(&snapshot).expect("valid materialized inference fixture"), XlsxInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(XlsxInference::infer(&XlsxSnapshot::default()).expect("valid materialized inference fixture"), XlsxInference::default());
}
