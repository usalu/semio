//! 🎞️ `bind-default-scene` implementation case `🎞️moves-the-0e42fe`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🏠️default-scene/🔗️bind/🎞️moves-the-0e42fe/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindDefaultSceneMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-default-scene");
    super::super::component::fixture_corpus_tests::assert_case("🏠️default-scene/🔗️bind/🎞️moves-the-0e42fe");
}
