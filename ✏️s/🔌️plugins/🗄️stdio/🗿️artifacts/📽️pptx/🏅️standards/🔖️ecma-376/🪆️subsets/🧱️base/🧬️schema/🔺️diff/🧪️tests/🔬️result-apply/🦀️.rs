
use super::*;

#[semio_framework_async_macros::async_test]
async fn rejects_missing_slide_target_without_mutating_base() {
    let base = PptxSnapshot::default();
    let diff = PptxDiff { presentation: Some(PptxPresentationDiff { slides: Some(PptxSlidesDiff { modified: vec![IndexModified { index: 0, diff: PptxSlideDiff::default() }], ..Default::default() }), ..Default::default() }), ..Default::default() };
    let result = diff.apply(&base);
    assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(base, PptxSnapshot::default());
}
