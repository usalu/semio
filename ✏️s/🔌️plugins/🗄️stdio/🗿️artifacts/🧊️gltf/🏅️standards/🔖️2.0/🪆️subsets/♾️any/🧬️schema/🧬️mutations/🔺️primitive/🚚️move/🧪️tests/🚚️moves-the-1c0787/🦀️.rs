//! 🚚️ `move-primitive` implementation case `🚚️moves`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔺️primitive/🚚️move/🚚️moves/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MovePrimitiveMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-primitive");
    super::super::component::fixture_corpus_tests::assert_case("🔺️primitive/🚚️move/🚚️moves");
}
