//! 🔢️ `bind-primitive-indices` implementation case `🔢️binds`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔢️primitive/🔗️bind/🔢️binds/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindPrimitiveIndicesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-primitive-indices");
    super::super::component::fixture_corpus_tests::assert_case("🔢️primitive-indices/🔗️bind/🔢️binds");
}
