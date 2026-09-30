//! 🔗️ `bind-morph-target-attribute` implementation case `🔗️binds-position-98e14f`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎚️morph-attribute/🔗️bind/🔗️binds-position-98e14f/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindMorphTargetAttributeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-morph-target-attribute");
    super::super::component::fixture_corpus_tests::assert_case("🎚️morph-attribute/🔗️bind/🔗️binds-position-98e14f");
}
