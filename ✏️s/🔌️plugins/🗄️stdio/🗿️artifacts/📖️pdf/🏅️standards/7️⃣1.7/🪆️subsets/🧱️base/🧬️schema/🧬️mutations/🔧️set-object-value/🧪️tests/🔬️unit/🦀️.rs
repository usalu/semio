use super::*;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetObjectValue as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-object-value");
}
