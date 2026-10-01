//! 🎛️ `create-sampler` implementation case `🎛️inserts`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎛️sampler/🌱️create/🎛️inserts/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateSamplerMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-sampler");
    super::super::component::fixture_corpus_tests::assert_case("🎛️sampler/🌱️create/🎛️inserts");
}
