//! 📐️ `change-primitive-topology-mode` implementation case `📐️triangle`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔺️primitive/📐️change-topology/📐️triangle/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_primitive_topology_mode::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangePrimitiveTopologyModeMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-primitive-topology-mode");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🔺️primitive/📐️change/📐️triangle");
}
