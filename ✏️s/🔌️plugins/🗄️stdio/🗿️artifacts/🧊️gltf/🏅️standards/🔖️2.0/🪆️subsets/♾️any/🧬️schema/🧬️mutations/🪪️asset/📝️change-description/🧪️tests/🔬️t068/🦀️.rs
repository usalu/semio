//! 🔬️ `change-asset-descriptive-metadata` implementation case `🔬️t068`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪪️asset/📝️change-description/🔬️t068/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeAssetDescriptiveMetadataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-asset-descriptive-metadata");
    super::super::component::fixture_corpus_tests::assert_case("🪪️asset/📝️change-description/🔬️t068");
}
