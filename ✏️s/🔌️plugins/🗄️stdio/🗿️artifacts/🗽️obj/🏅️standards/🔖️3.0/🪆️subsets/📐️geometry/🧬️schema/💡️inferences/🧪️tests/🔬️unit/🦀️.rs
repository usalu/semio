
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = ObjSnapshot::default();
    assert_eq!(ObjInference::infer(&snapshot), ObjInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(ObjInference::infer(&ObjSnapshot::default()), ObjInference::default());
}
