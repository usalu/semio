//! 🔬️ `move-primitive-attribute` implementation case `🔬️t060`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔤️primitive/🚚️move/🔬️t060/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::move_primitive_attribute::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MovePrimitiveAttributeMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "move-primitive-attribute");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🔤️primitive/🚚️move/🔬️t060");
}
