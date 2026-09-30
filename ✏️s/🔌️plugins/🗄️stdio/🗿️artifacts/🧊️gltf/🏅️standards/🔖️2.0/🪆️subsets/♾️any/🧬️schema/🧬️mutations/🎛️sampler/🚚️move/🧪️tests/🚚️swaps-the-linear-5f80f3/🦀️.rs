//! 🚚️ `move-sampler` implementation case `🚚️swaps-the-linear-5f80f3`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎛️sampler/🚚️move/🚚️swaps-the-linear-5f80f3/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveSamplerMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-sampler");
    super::super::component::fixture_corpus_tests::assert_case("🎛️sampler/🚚️move/🚚️swaps-the-linear-5f80f3");
}
