use crate::DxfSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use crate::standards::v_r12::subsets::any::schema::inferences::bounds::compute_dxf_bounds;
use crate::standards::v_r12::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DxfSnapshot::default();
    assert_eq!(DxfInference::infer(&snapshot).expect("valid materialized inference fixture"), DxfInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DxfInference::infer(&DxfSnapshot::default()).expect("valid materialized inference fixture"), DxfInference::default());
}
