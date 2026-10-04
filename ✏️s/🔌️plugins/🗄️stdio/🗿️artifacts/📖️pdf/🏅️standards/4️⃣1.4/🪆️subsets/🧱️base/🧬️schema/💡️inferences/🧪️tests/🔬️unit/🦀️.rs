use super::*;
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
