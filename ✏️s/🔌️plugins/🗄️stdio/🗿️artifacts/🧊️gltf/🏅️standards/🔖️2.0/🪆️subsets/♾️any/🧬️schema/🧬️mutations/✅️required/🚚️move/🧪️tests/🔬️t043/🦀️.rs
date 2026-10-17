//! 🔬️ `move-required-extension` implementation case `🔬️t043`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/✅️required/🚚️move/🔬️t043/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::move_required_extension::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveRequiredExtensionMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "move-required-extension");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("✅️required/🚚️move/🔬️t043");
}
