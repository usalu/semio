//! 🚫️ `delete-buffer` implementation case `🚫️removes-the-two-156f9d`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💿️buffer/🗑️delete/🚫️removes-the-two-156f9d/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteBufferMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-buffer");
    super::super::component::fixture_corpus_tests::assert_case("💿️buffer/🗑️delete/🚫️removes-the-two-156f9d");
}
