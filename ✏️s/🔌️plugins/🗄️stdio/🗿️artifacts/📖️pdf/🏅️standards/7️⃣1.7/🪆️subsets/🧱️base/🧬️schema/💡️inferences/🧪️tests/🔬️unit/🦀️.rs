
use super::*;
use protocol::Inference;

#[test]
fn inference_determinism_law() {
    let snapshot = PdfSnapshot::default();
    assert_eq!(Pdf17Inference::infer(&snapshot), Pdf17Inference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(Pdf17Inference::infer(&PdfSnapshot::default()), Pdf17Inference::default());
}
