//! 🎞️ `create-animation` implementation case `🎞️inserts`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎞️animation/🌱️create/🎞️inserts/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateAnimationMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-animation");
    super::super::component::fixture_corpus_tests::assert_case("🎞️animation/🌱️create/🎞️inserts");
}
