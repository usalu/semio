//! 🔗️ `bind-node-mesh` implementation case `🔗️binds-the-hull-fe46fd`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🏗️node-mesh/🔗️bind/🔗️binds-the-hull-fe46fd/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindNodeMeshMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-node-mesh");
    super::super::component::fixture_corpus_tests::assert_case("🏗️node-mesh/🔗️bind/🔗️binds-the-hull-fe46fd");
}
