use super::*;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetProperties as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-properties");
}
