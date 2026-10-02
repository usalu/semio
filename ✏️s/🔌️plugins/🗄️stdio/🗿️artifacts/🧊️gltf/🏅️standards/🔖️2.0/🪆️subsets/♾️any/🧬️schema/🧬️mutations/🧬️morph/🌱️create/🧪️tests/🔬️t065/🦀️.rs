//! 🔬️ `create-morph-target` implementation case `🔬️t065`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🧬️morph/🌱️create/🔬️t065/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateMorphTargetMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-morph-target");
    super::super::component::fixture_corpus_tests::assert_case("🧬️morph/🌱️create/🔬️t065");
}
