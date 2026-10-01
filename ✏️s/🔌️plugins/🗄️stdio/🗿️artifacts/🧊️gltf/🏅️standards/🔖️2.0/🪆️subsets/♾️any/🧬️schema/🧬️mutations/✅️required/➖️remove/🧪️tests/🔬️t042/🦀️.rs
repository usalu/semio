//! 🔬️ `remove-required-extension` implementation case `🔬️t042`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/✅️required/➖️remove/🔬️t042/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<RemoveRequiredExtensionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "remove-required-extension");
    super::super::component::fixture_corpus_tests::assert_case("✅️required-extension/➖️remove/🔬️t042");
}
