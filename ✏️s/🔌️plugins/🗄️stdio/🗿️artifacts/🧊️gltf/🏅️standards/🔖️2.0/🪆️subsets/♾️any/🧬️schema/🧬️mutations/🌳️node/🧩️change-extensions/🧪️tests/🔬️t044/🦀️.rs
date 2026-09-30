//! 🔬️ `change-node-extension-data` implementation case `🔬️t044`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/🧩️change-extensions/🔬️t044/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeNodeExtensionDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-node-extension-data");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/🧩️change-extensions/🔬️t044");
}
