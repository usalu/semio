//! ✍️ `change-mesh-extra-data` implementation case `✍️attaches`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/📝️change-extras/✍️attaches/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMeshExtraDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-mesh-extra-data");
    super::super::component::fixture_corpus_tests::assert_case("🕸️mesh/📝️change-extras/✍️attaches");
}
