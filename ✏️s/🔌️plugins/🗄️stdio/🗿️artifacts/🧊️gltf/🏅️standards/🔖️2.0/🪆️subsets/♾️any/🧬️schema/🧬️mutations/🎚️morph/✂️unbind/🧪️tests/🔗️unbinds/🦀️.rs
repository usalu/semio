//! 🔗️ `unbind-morph-target-attribute` implementation case `🔗️unbinds`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎚️morph/✂️unbind/🔗️unbinds/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindMorphTargetAttributeMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "unbind-morph-target-attribute");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🎚️morph/✂️unbind/🔗️unbinds");
}
