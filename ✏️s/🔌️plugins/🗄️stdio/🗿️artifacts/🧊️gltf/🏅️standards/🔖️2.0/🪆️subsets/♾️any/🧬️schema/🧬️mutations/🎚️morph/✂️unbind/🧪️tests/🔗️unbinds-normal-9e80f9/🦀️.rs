//! 🔗️ `unbind-morph-target-attribute` implementation case `🔗️unbinds`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎚️morph/✂️unbind/🔗️unbinds/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindMorphTargetAttributeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-morph-target-attribute");
    super::super::component::fixture_corpus_tests::assert_case("🎚️morph/✂️unbind/🔗️unbinds");
}
