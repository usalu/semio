use semio_s_artifact_stdio_pptx_base_inferences::standards::v_ecma_376::subsets::base::schema::inferences::*;
use protocol::Inference;
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law(){
 let snapshot=semio_s_artifact_stdio_pptx_base_inferences::standards::v_ecma_376::subsets::base::schema::inferences::outline::tests::fixture();
 assert_eq!(PptxInference::infer(&snapshot).unwrap(),PptxInference::infer(&snapshot).unwrap());
}
#[semio_framework_async_macros::async_test]
async fn inference_refuses_unmaterialized_default(){
 assert_eq!(PptxInference::infer(&PptxSnapshot::default()).unwrap_err().kind,semio_framework_value::ValueRefusalKind::InvalidValue);
}
