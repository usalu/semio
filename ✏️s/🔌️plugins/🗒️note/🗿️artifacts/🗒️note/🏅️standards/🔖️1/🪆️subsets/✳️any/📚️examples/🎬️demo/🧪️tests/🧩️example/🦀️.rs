use semio_framework_test_fixtures::fixture_text;

#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = fixture_text!("../../../../../../✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::schema::inferences::NoteInference;
    use semio_s_artifact_note_note_schema::NoteSnapshot;
    use protocol::Inference;
    let snapshot = NoteSnapshot::default();
    assert_eq!(NoteInference::infer(&snapshot).expect("valid materialized inference fixture"), NoteInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::schema::inferences::NoteInference;
    use protocol::Inference;
    assert_eq!(NoteInference::infer(&NoteSnapshot::default()).expect("valid materialized inference fixture"), NoteInference::default());
}
