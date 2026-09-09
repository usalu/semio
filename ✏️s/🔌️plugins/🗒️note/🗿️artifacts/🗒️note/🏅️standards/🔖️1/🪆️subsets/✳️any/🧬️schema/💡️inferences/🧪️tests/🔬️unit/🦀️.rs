use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = NoteSnapshot::default();
    assert_eq!(NoteInference::infer(&snapshot), NoteInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(NoteInference::infer(&NoteSnapshot::default()), NoteInference::default());
}
