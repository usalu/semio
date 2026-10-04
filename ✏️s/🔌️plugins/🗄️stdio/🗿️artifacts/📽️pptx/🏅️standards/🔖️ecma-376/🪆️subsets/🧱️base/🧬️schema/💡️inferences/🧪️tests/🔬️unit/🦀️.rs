use super::*;
use protocol::Inference;
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law(){
 let snapshot=crate::schema::inferences::outline::tests::fixture();
 assert_eq!(PptxInference::infer(&snapshot).unwrap(),PptxInference::infer(&snapshot).unwrap());
}
#[semio_framework_async_macros::async_test]
async fn inference_refuses_unmaterialized_default(){
 assert_eq!(PptxInference::infer(&PptxSnapshot::default()).unwrap_err().kind,semio_framework_value::ValueRefusalKind::InvalidValue);
}
