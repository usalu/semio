use super::*;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;

fn child_uri(id: &str) -> String {
    semio_framework_artifact_reference::ArtifactRef { artifact_id: id.into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() } }.to_uri()
}

fn snapshot_declaring(ids: &[&str]) -> TestSnapshot {
    TestSnapshot { count: 3, label: "before".into(), slot: ids.iter().map(|id| crate::test_app_mutation_fixture::document::test_child_handle(&child_uri(id)).expect("a canonical child uri")).collect() }
}

/// ➕️ L3: declaring members into an empty slot, replacing, reordering, shrinking, clearing and re-declaring the same membership sums to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for base in [snapshot_declaring(&[]), snapshot_declaring(&["a", "b", "c"])] {
        for target in [&["a", "b", "c"][..], &["c", "a", "b"], &["a", "z"], &["b"], &[]] {
            let children = target.iter().map(|id| child_uri(id)).collect();
            semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&TestMutation::from(SetSlotChildren { children }), &base).await;
        }
    }
}
