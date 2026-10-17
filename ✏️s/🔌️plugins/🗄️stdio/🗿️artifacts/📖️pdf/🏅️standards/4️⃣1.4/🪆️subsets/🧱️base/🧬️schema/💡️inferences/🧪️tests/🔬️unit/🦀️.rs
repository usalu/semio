use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
use crate::standards::v1_4::subsets::base::schema::inferences::*;
use protocol::Inference;

#[test]
fn inference_determinism_law() {
    let snapshot = PdfSnapshot::default();
    assert_eq!(PdfInference::infer(&snapshot).expect("valid materialized inference fixture"), PdfInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[test]
fn inference_default_law() {
    assert_eq!(PdfInference::infer(&PdfSnapshot::default()).expect("valid materialized inference fixture"), PdfInference::default());
}
