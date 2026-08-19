#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::block3d::Block3dSnapshot;
    use protocol::Inference;

    let text = include_str!("../🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio");
    let snapshot = <Block3dSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("example dsl parses");
    let inference = crate::artifacts::block3d::standards::v1::subsets::any::schema::inferences::Block3dInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::block3d::standards::v1::subsets::any::schema::inferences::Block3dInference::infer(&snapshot));
    assert!(inference.bounds.bounding_box.is_some(), "example has real vortices, so it should have a bounding box");
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::block3d::Block3dSnapshot;
    use protocol::Inference;

    assert_eq!(
        crate::artifacts::block3d::standards::v1::subsets::any::schema::inferences::Block3dInference::infer(&Block3dSnapshot::default()),
        crate::artifacts::block3d::standards::v1::subsets::any::schema::inferences::Block3dInference::default()
    );
}
//#endregion 🧪️InferenceLaws
