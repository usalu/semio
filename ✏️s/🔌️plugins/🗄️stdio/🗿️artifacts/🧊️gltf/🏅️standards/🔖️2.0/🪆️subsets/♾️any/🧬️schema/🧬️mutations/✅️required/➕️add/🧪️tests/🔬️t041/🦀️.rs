//! 🔬️ `add-required-extension` implementation case `🔬️t041`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/✅️required/➕️add/🔬️t041/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<AddRequiredExtensionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "add-required-extension");
    super::super::component::fixture_corpus_tests::assert_case("✅️required/➕️add/🔬️t041");
}
