//! 🔬️ `move-used-extension` implementation case `🔬️t058`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📣️used/🚚️move/🔬️t058/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveUsedExtensionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-used-extension");
    super::super::component::fixture_corpus_tests::assert_case("📣️used/🚚️move/🔬️t058");
}
