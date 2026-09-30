//! 🎞️ `create-scene` implementation case `🎞️inserts-an-empty-3bc9ce`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎬️scene/🌱️create/🎞️inserts-an-empty-3bc9ce/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateSceneMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-scene");
    super::super::component::fixture_corpus_tests::assert_case("🎬️scene/🌱️create/🎞️inserts-an-empty-3bc9ce");
}
