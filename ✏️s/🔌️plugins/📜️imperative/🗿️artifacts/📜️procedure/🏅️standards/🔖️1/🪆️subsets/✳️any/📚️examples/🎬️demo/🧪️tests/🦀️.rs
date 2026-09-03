#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    let snapshot = <crate::artifacts::procedure::ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::procedure::standards::v1::subsets::any::schema::inferences::ProcedureInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::procedure::standards::v1::subsets::any::schema::inferences::ProcedureInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::procedure::standards::v1::subsets::any::schema::inferences::ProcedureInference::infer(&crate::artifacts::procedure::ProcedureSnapshot::default()),
        crate::artifacts::procedure::standards::v1::subsets::any::schema::inferences::ProcedureInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
