//! 🚫️ `delete-sampler` implementation case `🚫️removes-the-6e2007`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎛️sampler/🗑️delete/🚫️removes-the-6e2007/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteSamplerMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-sampler");
    super::super::component::fixture_corpus_tests::assert_case("🎛️sampler/🗑️delete/🚫️removes-the-6e2007");
}
