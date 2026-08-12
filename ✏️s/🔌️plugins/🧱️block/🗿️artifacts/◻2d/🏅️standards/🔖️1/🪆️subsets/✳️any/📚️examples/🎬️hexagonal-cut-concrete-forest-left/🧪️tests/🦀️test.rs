#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::artifacts::block2d::Block2dSnapshot;
    use protocol::Inference;

    let text = include_str!("../🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio");
    let snapshot = <Block2dSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("example dsl parses");
    let inference = crate::artifacts::block2d::standards::v1::subsets::any::schema::inferences::Block2dInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::block2d::standards::v1::subsets::any::schema::inferences::Block2dInference::infer(&snapshot));
    assert!(inference.bounds.bounding_box.is_some(), "example has real handles, so it should have a bounding box");
}

#[test]
fn inference_default_law() {
    use crate::artifacts::block2d::Block2dSnapshot;
    use protocol::Inference;

    assert_eq!(
        crate::artifacts::block2d::standards::v1::subsets::any::schema::inferences::Block2dInference::infer(&Block2dSnapshot::default()),
        crate::artifacts::block2d::standards::v1::subsets::any::schema::inferences::Block2dInference::default()
    );
}
//#endregion 🧪️InferenceLaws
