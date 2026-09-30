//! 🔬️ `change-document-extra-data` implementation case `🔬️t052`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📃️document/📝️change-extras/🔬️t052/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeDocumentExtraDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-document-extra-data");
    super::super::component::fixture_corpus_tests::assert_case("📃️document/📝️change-extras/🔬️t052");
}
