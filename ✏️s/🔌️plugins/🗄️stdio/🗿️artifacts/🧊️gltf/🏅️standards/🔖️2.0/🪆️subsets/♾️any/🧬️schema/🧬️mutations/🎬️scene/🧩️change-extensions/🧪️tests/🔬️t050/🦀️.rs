//! 🔬️ `change-scene-extension-data` implementation case `🔬️t050`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎬️scene/🧩️change-extensions/🔬️t050/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeSceneExtensionDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-scene-extension-data");
    super::super::component::fixture_corpus_tests::assert_case("🎬️scene/🧩️change/🔬️t050");
}
