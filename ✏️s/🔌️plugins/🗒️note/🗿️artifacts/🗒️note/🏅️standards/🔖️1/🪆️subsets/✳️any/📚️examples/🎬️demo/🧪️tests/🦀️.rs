#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::note::schema::inferences::NoteInference;
    use crate::artifacts::note::NoteSnapshot;
    use protocol::Inference;
    let snapshot = NoteSnapshot::default();
    assert_eq!(NoteInference::infer(&snapshot), NoteInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::note::schema::inferences::NoteInference;
    use crate::artifacts::note::NoteSnapshot;
    use protocol::Inference;
    assert_eq!(NoteInference::infer(&NoteSnapshot::default()), NoteInference::default());
}
