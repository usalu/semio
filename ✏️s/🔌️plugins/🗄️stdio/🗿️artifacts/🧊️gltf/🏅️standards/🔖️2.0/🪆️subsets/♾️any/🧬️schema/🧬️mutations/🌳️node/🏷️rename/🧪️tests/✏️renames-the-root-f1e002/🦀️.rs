//! ✏️ `change-node-name` implementation case `✏️renames-the-root-f1e002`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/🏷️rename/✏️renames-the-root-f1e002/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-node-name");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/🏷️rename/✏️renames-the-root-f1e002");
}
