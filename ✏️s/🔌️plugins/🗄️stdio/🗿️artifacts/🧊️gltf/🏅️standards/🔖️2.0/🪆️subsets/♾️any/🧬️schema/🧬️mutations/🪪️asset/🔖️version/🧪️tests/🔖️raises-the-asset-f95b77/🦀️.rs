//! 🔖️ `change-asset-version` implementation case `🔖️raises-the-asset-f95b77`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪪️asset/🔖️version/🔖️raises-the-asset-f95b77/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeAssetVersionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-asset-version");
    super::super::component::fixture_corpus_tests::assert_case("🪪️asset/🔖️version/🔖️raises-the-asset-f95b77");
}
