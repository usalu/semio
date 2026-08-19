#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::dag::DagSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::dag::standards::v1::subsets::any::schema::inferences::DagInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::dag::standards::v1::subsets::any::schema::inferences::DagInference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::dag::standards::v1::subsets::any::schema::inferences::DagInference::infer(&crate::artifacts::dag::DagSnapshot::default()),
        crate::artifacts::dag::standards::v1::subsets::any::schema::inferences::DagInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
