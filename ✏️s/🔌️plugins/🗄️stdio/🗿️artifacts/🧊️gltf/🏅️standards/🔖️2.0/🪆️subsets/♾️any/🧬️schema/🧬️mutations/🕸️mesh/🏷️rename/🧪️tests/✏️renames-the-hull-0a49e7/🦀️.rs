//! ✏️ `change-mesh-name` implementation case `✏️renames`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/🏷️rename/✏️renames/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMeshNameMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-mesh-name");
    super::super::component::fixture_corpus_tests::assert_case("🕸️mesh/🏷️rename/✏️renames");
}
