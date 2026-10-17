//! 🎞️ `unbind-default-scene` implementation case `🎞️clears`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🏠️default-scene/✂️unbind/🎞️clears/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_default_scene::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindDefaultSceneMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "unbind-default-scene");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🏠️default-scene/✂️unbind/🎞️clears");
}
