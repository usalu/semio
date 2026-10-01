//! ✂️ `unbind-node-child` implementation case `✂️releases`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌿️node-child/✂️unbind/✂️releases/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindNodeChildMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-node-child");
    super::super::component::fixture_corpus_tests::assert_case("🌿️node-child/✂️unbind/✂️releases");
}
