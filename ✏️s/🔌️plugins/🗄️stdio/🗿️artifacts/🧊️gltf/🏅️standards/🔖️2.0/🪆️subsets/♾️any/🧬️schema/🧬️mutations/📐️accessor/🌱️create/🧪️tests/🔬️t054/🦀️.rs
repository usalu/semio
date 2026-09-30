//! 🔬️ `create-accessor` implementation case `🔬️t054`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📐️accessor/🌱️create/🔬️t054/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateAccessorMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-accessor");
    super::super::component::fixture_corpus_tests::assert_case("📐️accessor/🌱️create/🔬️t054");
}
