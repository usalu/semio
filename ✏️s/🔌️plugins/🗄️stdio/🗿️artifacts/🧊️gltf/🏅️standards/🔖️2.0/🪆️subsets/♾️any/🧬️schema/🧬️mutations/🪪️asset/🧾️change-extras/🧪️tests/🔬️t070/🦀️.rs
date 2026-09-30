//! 🔬️ `change-asset-extra-data` implementation case `🔬️t070`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪪️asset/🧾️change-extras/🔬️t070/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeAssetExtraDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-asset-extra-data");
    super::super::component::fixture_corpus_tests::assert_case("🪪️asset/🧾️change-extras/🔬️t070");
}
