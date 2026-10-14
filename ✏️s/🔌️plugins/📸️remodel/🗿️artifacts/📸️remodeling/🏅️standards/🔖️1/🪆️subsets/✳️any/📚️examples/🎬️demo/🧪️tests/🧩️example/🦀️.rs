#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = semio_s_artifact_remodel_remodeling_inferences::standards::v1::subsets::any::schema::inferences::RemodelingInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, semio_s_artifact_remodel_remodeling_inferences::standards::v1::subsets::any::schema::inferences::RemodelingInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(semio_s_artifact_remodel_remodeling_inferences::standards::v1::subsets::any::schema::inferences::RemodelingInference::infer(&semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot::default()).expect("valid materialized inference fixture"), semio_s_artifact_remodel_remodeling_inferences::standards::v1::subsets::any::schema::inferences::RemodelingInference::default(),);
}
//#endregion 🧪️InferenceLaws
