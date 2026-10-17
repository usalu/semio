//! 🔬️ `move-morph-target-attribute` implementation case `🔬️t047`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎚️morph/🚚️move/🔬️t047/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveMorphTargetAttributeMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "move-morph-target-attribute");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🎚️morph/🚚️move/🔬️t047");
}
