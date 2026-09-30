//! 🎞️ `unbind-default-scene` implementation case `🎞️clears-the-b236b0`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🏠️default-scene/✂️unbind/🎞️clears-the-b236b0/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindDefaultSceneMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-default-scene");
    super::super::component::fixture_corpus_tests::assert_case("🏠️default-scene/✂️unbind/🎞️clears-the-b236b0");
}
