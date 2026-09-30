//! 🔀️ `reorder-skins` implementation case `🔀️flips-the-two-1fc9d9`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🦴️skin/🔀️reorder/🔀️flips-the-two-1fc9d9/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderSkinsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-skins");
    super::super::component::fixture_corpus_tests::assert_case("🦴️skin/🔀️reorder/🔀️flips-the-two-1fc9d9");
}
