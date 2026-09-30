//! 📸️ `set-snapshot` implementation case `📸️replaces-all`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📸️snapshot/📸️set/📸️replaces-all/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<SetSnapshot as protocol::MutationKind<GltfSnapshot, GltfMutation>>::SEMANTICS.kind, "set-snapshot");
    super::super::component::fixture_corpus_tests::assert_case("📸️snapshot/📸️set/📸️replaces-all");
}
