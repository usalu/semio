//! 📝️ `change-node-extra-data` implementation case `📝️attaches-a-unit-5159f9`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/📝️change-extras/📝️attaches-a-unit-5159f9/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeNodeExtraDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-node-extra-data");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/📝️change-extras/📝️attaches-a-unit-5159f9");
}
