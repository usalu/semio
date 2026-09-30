//! 🔬️ `change-asset-extension-data` implementation case `🔬️t069`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪪️asset/🧩️change-extensions/🔬️t069/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeAssetExtensionDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-asset-extension-data");
    super::super::component::fixture_corpus_tests::assert_case("🪪️asset/🧩️change-extensions/🔬️t069");
}
