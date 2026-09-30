//! 🔬️ `delete-buffer-view` implementation case `🔬️t067`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪟️buffer-view/🗑️delete/🔬️t067/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteBufferViewMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-buffer-view");
    super::super::component::fixture_corpus_tests::assert_case("🪟️buffer-view/🗑️delete/🔬️t067");
}
