use super::*;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveExtGState as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-ext-g-state");
}
