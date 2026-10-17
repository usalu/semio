use crate::SvgSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::base::schema::inferences::dimensions::compute_svg_dimensions;
use crate::standards::v1_1::subsets::base::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SvgSnapshot::default();
    assert_eq!(SvgInference::infer(&snapshot).expect("valid materialized inference fixture"), SvgInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SvgInference::infer(&SvgSnapshot::default()).expect("valid materialized inference fixture"), SvgInference::default());
}
