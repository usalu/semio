//! ✏️ `change-scene-name` implementation case `✏️renames`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎬️scene/🏷️rename/✏️renames/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeSceneNameMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-scene-name");
    super::super::component::fixture_corpus_tests::assert_case("🎬️scene/🏷️rename/✏️renames");
}
