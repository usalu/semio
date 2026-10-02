//! 🔬️ `remove-used-extension` implementation case `🔬️t057`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📣️used/➖️remove/🔬️t057/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<RemoveUsedExtensionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "remove-used-extension");
    super::super::component::fixture_corpus_tests::assert_case("📣️used/➖️remove/🔬️t057");
}
