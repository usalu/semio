//! 🔬️ `move-morph-target-attribute` implementation case `🔬️t047`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎚️morph/🚚️move/🔬️t047/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveMorphTargetAttributeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-morph-target-attribute");
    super::super::component::fixture_corpus_tests::assert_case("🎚️morph-attribute/🚚️move/🔬️t047");
}
