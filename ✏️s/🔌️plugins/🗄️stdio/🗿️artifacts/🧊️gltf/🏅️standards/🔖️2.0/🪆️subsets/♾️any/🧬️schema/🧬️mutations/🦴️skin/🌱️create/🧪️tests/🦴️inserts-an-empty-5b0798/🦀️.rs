//! 🦴️ `create-skin` implementation case `🦴️inserts-an-empty-5b0798`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🦴️skin/🌱️create/🦴️inserts-an-empty-5b0798/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateSkinMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-skin");
    super::super::component::fixture_corpus_tests::assert_case("🦴️skin/🌱️create/🦴️inserts-an-empty-5b0798");
}
