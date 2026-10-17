use crate::schema::snapshot::SemioObjectSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use crate::schema::inferences::composition::compute_semio_object_composition;
use crate::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioObjectSnapshot::default();
    assert_eq!(SemioObjectInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioObjectInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioObjectInference::infer(&SemioObjectSnapshot::default()).expect("valid materialized inference fixture"), SemioObjectInference::default());
}
