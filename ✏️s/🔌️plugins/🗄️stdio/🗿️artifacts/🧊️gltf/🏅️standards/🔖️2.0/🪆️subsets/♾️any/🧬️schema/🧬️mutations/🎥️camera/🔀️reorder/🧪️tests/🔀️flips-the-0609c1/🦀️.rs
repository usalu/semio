//! 🔀️ `reorder-cameras` implementation case `🔀️flips-the-0609c1`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎥️camera/🔀️reorder/🔀️flips-the-0609c1/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderCamerasMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-cameras");
    super::super::component::fixture_corpus_tests::assert_case("🎥️camera/🔀️reorder/🔀️flips-the-0609c1");
}
