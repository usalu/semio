#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../🖼️assets/🌲️hexagonal-cut-concrete-forest/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_block_3d_model::Block3dSnapshot;
    use protocol::Inference;

    let text = include_str!("../../🖼️assets/🌲️hexagonal-cut-concrete-forest/🗣️.dsl.semio");
    let snapshot = <Block3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("example dsl parses");
    let inference = semio_s_artifact_block_3d_inferences::Block3dInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, semio_s_artifact_block_3d_inferences::Block3dInference::infer(&snapshot).expect("valid materialized inference fixture"));
    assert!(inference.bounds.bounding_box.is_some(), "example has real vortices, so it should have a bounding box");
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_block_3d_model::Block3dSnapshot;
    use protocol::Inference;

    assert_eq!(semio_s_artifact_block_3d_inferences::Block3dInference::infer(&Block3dSnapshot::default()).expect("valid materialized inference fixture"), semio_s_artifact_block_3d_inferences::Block3dInference::default());
}
//#endregion 🧪️InferenceLaws
