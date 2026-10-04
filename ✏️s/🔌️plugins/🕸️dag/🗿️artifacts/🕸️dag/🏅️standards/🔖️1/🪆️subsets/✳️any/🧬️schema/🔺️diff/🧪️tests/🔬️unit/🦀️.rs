use super::*;
use crate::default_snapshot;

#[semio_framework_async_macros::async_test]
async fn dag_diff_default_has_no_pending_writes() {
    let diff = DagDiff::default();
    assert!(diff.content.is_none());
}

/// 🔁️ The parent diff only ever swaps the content handle; applying one carries exactly that handle.
#[semio_framework_async_macros::async_test]
async fn a_content_handle_diff_swaps_only_the_handle() {
    let base = default_snapshot();
    let handle = crate::dag_content_child_handle(&crate::DagScene::default());
    let next = DagDiff { content: Some(handle.clone()), ..Default::default() }.apply(&base).expect("valid handle diff");
    assert_eq!(next.content, handle);
    assert_eq!(next.schema, base.schema);
}
