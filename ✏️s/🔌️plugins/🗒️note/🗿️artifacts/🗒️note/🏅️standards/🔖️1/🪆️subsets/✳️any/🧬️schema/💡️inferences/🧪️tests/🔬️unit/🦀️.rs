use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = NoteSnapshot::default();
    assert_eq!(NoteInference::infer(&snapshot).expect("valid materialized inference fixture"), NoteInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(NoteInference::infer(&NoteSnapshot::default()).expect("valid materialized inference fixture"), NoteInference::default());
}
