//! 📝️ `change-primitive-extra-data` implementation case `📝️attaches-a-lod-698164`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔺️primitive/📝️change-extras/📝️attaches-a-lod-698164/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangePrimitiveExtraDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-primitive-extra-data");
    super::super::component::fixture_corpus_tests::assert_case("🔺️primitive/📝️change-extras/📝️attaches-a-lod-698164");
}
