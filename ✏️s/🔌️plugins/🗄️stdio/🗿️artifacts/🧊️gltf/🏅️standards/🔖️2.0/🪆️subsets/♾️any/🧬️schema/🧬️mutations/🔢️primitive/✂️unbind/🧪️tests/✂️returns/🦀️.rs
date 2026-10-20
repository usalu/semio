//! ✂️ `unbind-primitive-indices` implementation case `✂️returns`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔢️primitive/✂️unbind/✂️returns/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_indices::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindPrimitiveIndicesMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "unbind-primitive-indices");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🔢️primitive/✂️unbind/✂️returns");
}
