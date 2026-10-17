//! 🔬️ `remove-required-extension` implementation case `🔬️t042`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/✅️required/➖️remove/🔬️t042/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::remove_required_extension::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<RemoveRequiredExtensionMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "remove-required-extension");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("✅️required/➖️remove/🔬️t042");
}
