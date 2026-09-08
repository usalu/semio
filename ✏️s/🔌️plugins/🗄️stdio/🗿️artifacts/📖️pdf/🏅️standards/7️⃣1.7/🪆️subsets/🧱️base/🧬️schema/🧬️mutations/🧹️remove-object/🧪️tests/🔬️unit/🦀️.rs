
use super::*;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<RemoveObject as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-object");
}
