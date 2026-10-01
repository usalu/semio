//! ⬇️ `move-scene` implementation case `⬇️demotes`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎬️scene/🚚️move/⬇️demotes/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveSceneMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-scene");
    super::super::component::fixture_corpus_tests::assert_case("🎬️scene/🚚️move/⬇️demotes");
}
