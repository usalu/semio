//! 🔬️ `delete-accessor` implementation case `🔬️t055`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📐️accessor/🗑️delete/🔬️t055/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteAccessorMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-accessor");
    super::super::component::fixture_corpus_tests::assert_case("📐️accessor/🗑️delete/🔬️t055");
}
