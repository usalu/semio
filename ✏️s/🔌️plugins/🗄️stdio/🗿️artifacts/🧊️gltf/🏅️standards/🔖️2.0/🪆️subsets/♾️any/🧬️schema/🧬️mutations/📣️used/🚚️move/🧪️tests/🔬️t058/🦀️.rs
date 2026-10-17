//! 🔬️ `move-used-extension` implementation case `🔬️t058`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📣️used/🚚️move/🔬️t058/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::move_used_extension::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveUsedExtensionMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "move-used-extension");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("📣️used/🚚️move/🔬️t058");
}
