//! 🔀️ `change-node-transform` implementation case `🔀️replaces-a-f09fa4`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/📐️transform/🔀️replaces-a-f09fa4/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeNodeTransformMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-node-transform");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/📐️transform/🔀️replaces-a-f09fa4");
}
