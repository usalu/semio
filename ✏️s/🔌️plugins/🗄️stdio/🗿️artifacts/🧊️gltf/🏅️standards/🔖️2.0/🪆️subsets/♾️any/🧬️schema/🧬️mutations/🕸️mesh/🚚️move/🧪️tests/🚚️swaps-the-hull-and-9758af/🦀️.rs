//! 🚚️ `move-mesh` implementation case `🚚️swaps-the-hull-and-9758af`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/🚚️move/🚚️swaps-the-hull-and-9758af/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveMeshMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-mesh");
    super::super::component::fixture_corpus_tests::assert_case("🕸️mesh/🚚️move/🚚️swaps-the-hull-and-9758af");
}
