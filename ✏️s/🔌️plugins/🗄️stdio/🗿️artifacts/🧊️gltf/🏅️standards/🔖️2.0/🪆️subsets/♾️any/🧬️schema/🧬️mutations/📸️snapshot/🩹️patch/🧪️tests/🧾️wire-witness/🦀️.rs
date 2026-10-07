//! 🧾️ The neutral snapshot patch witness exercises canonical admission, diff, and inverse laws.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<PatchSnapshot as protocol::MutationKind<GltfSnapshot, GltfMutation>>::SEMANTICS.kind, "patch-snapshot");
    super::super::component::fixture_corpus_tests::assert_case("📸️snapshot/🩹️patch/🧾️wire-witness");
}
