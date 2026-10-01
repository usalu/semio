//! ✂️ `unbind-primitive-material` implementation case `✂️returns`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🧱️primitive/✂️unbind/✂️returns/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindPrimitiveMaterialMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-primitive-material");
    super::super::component::fixture_corpus_tests::assert_case("🧱️primitive-material/✂️unbind/✂️returns");
}
