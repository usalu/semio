use super::*;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetPageCropBox as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-page-crop-box");
}
