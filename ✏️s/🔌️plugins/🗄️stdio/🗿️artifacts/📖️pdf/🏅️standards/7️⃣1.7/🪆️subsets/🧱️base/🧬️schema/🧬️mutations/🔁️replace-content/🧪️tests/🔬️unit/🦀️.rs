use super::*;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<ReplaceContent as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "replace-content");
}
