
use super::*;
use protocol::Inference;

#[test]
fn inference_determinism_law() {
    let snapshot = PdfSnapshot::default();
    assert_eq!(PdfInference::infer(&snapshot), PdfInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(PdfInference::infer(&PdfSnapshot::default()), PdfInference::default());
}
