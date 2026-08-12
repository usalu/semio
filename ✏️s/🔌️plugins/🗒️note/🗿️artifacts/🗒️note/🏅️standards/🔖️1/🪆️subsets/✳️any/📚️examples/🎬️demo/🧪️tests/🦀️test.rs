#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
fn inference_determinism_law() {
    use crate::artifacts::note::schema::inferences::NoteInference;
    use crate::artifacts::note::NoteSnapshot;
    use protocol::Inference;
    let snapshot = NoteSnapshot::default();
    assert_eq!(NoteInference::infer(&snapshot), NoteInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::artifacts::note::schema::inferences::NoteInference;
    use crate::artifacts::note::NoteSnapshot;
    use protocol::Inference;
    assert_eq!(NoteInference::infer(&NoteSnapshot::default()), NoteInference::default());
}
