//! 🪞️ `change-material-double-sided` implementation case `🪞️makes`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💎️material/🪞️change-sides/🪞️makes/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMaterialDoubleSidedMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-material-double-sided");
    super::super::component::fixture_corpus_tests::assert_case("💎️material/🪞️change-sides/🪞️makes");
}

#[semio_framework_async_macros::async_test]
async fn applies_and_rejects_identity() {
    let mut snapshot = GltfSnapshot::default();
    snapshot.document.materials.push(Default::default());
    let payload = GltfChangeMaterialDoubleSidedPayload { material: 0, double_sided: true };
    apply(&mut snapshot, &payload).unwrap();
    assert!(snapshot.document.materials[0].double_sided);
    assert!(apply(&mut snapshot, &payload).is_err());
}
