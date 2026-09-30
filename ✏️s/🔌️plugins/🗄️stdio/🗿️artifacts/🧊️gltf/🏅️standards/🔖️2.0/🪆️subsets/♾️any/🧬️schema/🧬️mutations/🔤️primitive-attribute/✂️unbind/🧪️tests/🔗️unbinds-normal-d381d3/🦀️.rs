//! 🔗️ `unbind-primitive-attribute` implementation case `🔗️unbinds-normal-d381d3`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔤️primitive-attribute/✂️unbind/🔗️unbinds-normal-d381d3/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindPrimitiveAttributeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-primitive-attribute");
    super::super::component::fixture_corpus_tests::assert_case("🔤️primitive-attribute/✂️unbind/🔗️unbinds-normal-d381d3");
}
