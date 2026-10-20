//! 🎞️ `bind-default-scene` implementation case `🎞️moves`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🏠️default-scene/🔗️bind/🎞️moves/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_default_scene::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindDefaultSceneMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "bind-default-scene");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🏠️default-scene/🔗️bind/🎞️moves");
}
