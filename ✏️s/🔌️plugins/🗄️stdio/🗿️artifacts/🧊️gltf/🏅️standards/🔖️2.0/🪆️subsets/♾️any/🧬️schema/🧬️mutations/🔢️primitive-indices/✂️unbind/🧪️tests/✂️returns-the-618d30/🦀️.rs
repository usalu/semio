//! ✂️ `unbind-primitive-indices` implementation case `✂️returns-the-618d30`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔢️primitive-indices/✂️unbind/✂️returns-the-618d30/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindPrimitiveIndicesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-primitive-indices");
    super::super::component::fixture_corpus_tests::assert_case("🔢️primitive-indices/✂️unbind/✂️returns-the-618d30");
}
