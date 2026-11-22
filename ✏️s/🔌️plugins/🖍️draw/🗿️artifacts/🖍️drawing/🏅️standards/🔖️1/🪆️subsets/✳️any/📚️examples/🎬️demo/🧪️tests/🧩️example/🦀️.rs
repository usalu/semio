#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_draw_drawing_model::inferences::DrawingInference;
    use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
    use protocol::Inference;
    let snapshot = DrawingSnapshot::default();
    assert_eq!(DrawingInference::infer(&snapshot).expect("valid materialized inference fixture"), DrawingInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_draw_drawing_model::inferences::DrawingInference;
    use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
    use protocol::Inference;
    assert_eq!(DrawingInference::infer(&DrawingSnapshot::default()).expect("valid materialized inference fixture"), DrawingInference::default());
}

#[test]
fn registered_demo_source_parses_the_authored_emblem() {
    use semio_s_artifact_draw_drawing_model::{DrawingLayerNode, layer_id};
    let source = super::source();
    let document = semio_s_artifact_draw_drawing_model::io::text::snapshot::parse_dsl(&source.document_json()).expect("parse registered Draw Demo");
    assert_eq!(document.id, "semio");
    let DrawingLayerNode::Group(emblem) = &document.layers[0] else { panic!("registered Demo must contain the emblem group"); };
    let ids: Vec<String> = emblem.children.iter().map(|layer| layer_id(layer).to_string()).collect();
    assert_eq!(ids, ["emblem-orange", "emblem-red", "emblem-teal"]);
}
