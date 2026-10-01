//! 🔗️ `bind-primitive-material` implementation case `🔗️binds`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🧱️primitive/🔗️bind/🔗️binds/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindPrimitiveMaterialMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-primitive-material");
    super::super::component::fixture_corpus_tests::assert_case("🧱️primitive-material/🔗️bind/🔗️binds");
}
