//! 🔬️ `change-document-extension-data` implementation case `🔬️t053`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📃️document/🧩️change-extensions/🔬️t053/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeDocumentExtensionDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-document-extension-data");
    super::super::component::fixture_corpus_tests::assert_case("📃️document/🧩️change-extensions/🔬️t053");
}
