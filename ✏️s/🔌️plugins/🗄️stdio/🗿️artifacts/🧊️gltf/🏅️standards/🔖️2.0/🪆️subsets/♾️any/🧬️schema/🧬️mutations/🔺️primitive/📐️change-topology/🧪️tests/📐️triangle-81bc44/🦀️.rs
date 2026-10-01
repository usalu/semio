//! 📐️ `change-primitive-topology-mode` implementation case `📐️triangle`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔺️primitive/📐️change-topology/📐️triangle/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangePrimitiveTopologyModeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-primitive-topology-mode");
    super::super::component::fixture_corpus_tests::assert_case("🔺️primitive/📐️change-topology/📐️triangle");
}
