
use super::*;

/// 🧬️ `diff_set_snapshot`/`CurationDiff.artifact` are a generic whole-artifact-replacement escape
/// hatch retained for `apply_to_artifact`'s own callers — no `SourcingMutation` variant reaches
/// it any more (the former whole-snapshot-replace variant is banned outright, see `📓️taxonomy.md`), so this exercises the
/// function directly rather than through a mutation's `diff()`.
#[semio_framework_async_macros::async_test]
async fn diff_set_snapshot_carries_whole_replacement() {
    let base = CurationSnapshot::default();
    let next = CurationSnapshot::default();
    let diff = diff_set_snapshot(&next);
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), next);
}

#[semio_framework_async_macros::async_test]
async fn absorb_keeps_later_artifact_replacement() {
    let mut first = CurationDiff { artifact: Some(Box::new(CurationArtifact::default())), ..Default::default() };
    let second = CurationDiff::default();
    first.absorb(second);
    assert!(first.artifact.is_some());
}
