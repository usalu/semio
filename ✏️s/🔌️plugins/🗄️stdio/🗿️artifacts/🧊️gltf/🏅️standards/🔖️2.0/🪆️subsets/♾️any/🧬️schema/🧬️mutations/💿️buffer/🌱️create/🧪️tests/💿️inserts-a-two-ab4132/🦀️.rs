//! 💿️ `create-buffer` implementation case `💿️inserts-a-two-ab4132`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💿️buffer/🌱️create/💿️inserts-a-two-ab4132/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateBufferMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-buffer");
    super::super::component::fixture_corpus_tests::assert_case("💿️buffer/🌱️create/💿️inserts-a-two-ab4132");
}
