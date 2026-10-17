//! 🔬️ `create-morph-target` implementation case `🔬️t065`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🧬️morph/🌱️create/🔬️t065/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::create_morph_target::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateMorphTargetMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "create-morph-target");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🧬️morph/🌱️create/🔬️t065");
}
