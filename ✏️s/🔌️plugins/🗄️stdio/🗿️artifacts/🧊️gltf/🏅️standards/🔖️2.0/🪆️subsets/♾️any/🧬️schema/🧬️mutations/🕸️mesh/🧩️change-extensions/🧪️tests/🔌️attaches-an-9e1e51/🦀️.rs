//! 🔌️ `change-mesh-extension-data` implementation case `🔌️attaches-an-9e1e51`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/🧩️change-extensions/🔌️attaches-an-9e1e51/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMeshExtensionDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-mesh-extension-data");
    super::super::component::fixture_corpus_tests::assert_case("🕸️mesh/🧩️change-extensions/🔌️attaches-an-9e1e51");
}
