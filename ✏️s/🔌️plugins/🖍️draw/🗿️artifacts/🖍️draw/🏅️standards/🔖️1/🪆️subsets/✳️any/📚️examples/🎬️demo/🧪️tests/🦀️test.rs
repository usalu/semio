#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
fn inference_determinism_law() {
    use crate::artifacts::draw::schema::inferences::DrawInference;
    use crate::artifacts::draw::DrawSnapshot;
    use protocol::Inference;
    let snapshot = DrawSnapshot::default();
    assert_eq!(DrawInference::infer(&snapshot), DrawInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::artifacts::draw::schema::inferences::DrawInference;
    use crate::artifacts::draw::DrawSnapshot;
    use protocol::Inference;
    assert_eq!(DrawInference::infer(&DrawSnapshot::default()), DrawInference::default());
}
