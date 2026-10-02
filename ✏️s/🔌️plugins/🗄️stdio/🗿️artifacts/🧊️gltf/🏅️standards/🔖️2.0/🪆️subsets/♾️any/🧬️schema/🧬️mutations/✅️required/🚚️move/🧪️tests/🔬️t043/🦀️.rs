//! 🔬️ `move-required-extension` implementation case `🔬️t043`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/✅️required/🚚️move/🔬️t043/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveRequiredExtensionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-required-extension");
    super::super::component::fixture_corpus_tests::assert_case("✅️required/🚚️move/🔬️t043");
}
