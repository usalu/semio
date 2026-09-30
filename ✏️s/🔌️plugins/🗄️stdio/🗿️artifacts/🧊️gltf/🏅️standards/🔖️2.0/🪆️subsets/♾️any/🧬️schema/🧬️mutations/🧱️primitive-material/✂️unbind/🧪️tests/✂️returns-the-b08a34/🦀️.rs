//! ✂️ `unbind-primitive-material` implementation case `✂️returns-the-b08a34`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🧱️primitive-material/✂️unbind/✂️returns-the-b08a34/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindPrimitiveMaterialMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-primitive-material");
    super::super::component::fixture_corpus_tests::assert_case("🧱️primitive-material/✂️unbind/✂️returns-the-b08a34");
}
