//! 🔌️ `reorder-used-extensions` implementation case `🔌️flips`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📣️used/🔀️reorder/🔌️flips/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_used_extensions::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderUsedExtensionsMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "reorder-used-extensions");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("📣️used/🔀️reorder/🔌️flips");
}
