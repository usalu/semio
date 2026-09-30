//! 🔬️ `create-primitive` implementation case `🔬️t061`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔺️primitive/🌱️create/🔬️t061/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreatePrimitiveMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-primitive");
    super::super::component::fixture_corpus_tests::assert_case("🔺️primitive/🌱️create/🔬️t061");
}
