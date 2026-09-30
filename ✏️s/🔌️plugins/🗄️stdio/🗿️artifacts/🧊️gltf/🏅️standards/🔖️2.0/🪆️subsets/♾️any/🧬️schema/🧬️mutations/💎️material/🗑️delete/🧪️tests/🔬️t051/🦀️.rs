//! 🔬️ `delete-material` implementation case `🔬️t051`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💎️material/🗑️delete/🔬️t051/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteMaterialMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-material");
    super::super::component::fixture_corpus_tests::assert_case("💎️material/🗑️delete/🔬️t051");
}
