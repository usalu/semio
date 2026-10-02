//! 🔬️ `add-used-extension` implementation case `🔬️t056`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📣️used/➕️add/🔬️t056/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<AddUsedExtensionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "add-used-extension");
    super::super::component::fixture_corpus_tests::assert_case("📣️used/➕️add/🔬️t056");
}
