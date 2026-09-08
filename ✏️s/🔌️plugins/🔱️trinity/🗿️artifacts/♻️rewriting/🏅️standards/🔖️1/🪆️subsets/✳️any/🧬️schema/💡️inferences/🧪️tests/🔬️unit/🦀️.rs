
use super::*;
use crate::LayoutPoint;
use protocol::Inference;
use std::collections::BTreeMap;

//#region 🧸️Fixtures
fn two_point_snapshot() -> RewritingSnapshot {
    let mut rule_layout = BTreeMap::new();
    rule_layout.insert("a".to_string(), LayoutPoint { x: 0.0, y: 0.0 });
    rule_layout.insert("b".to_string(), LayoutPoint { x: -140.0, y: 80.0 });
    RewritingSnapshot { rule_layout, ..RewritingSnapshot::default() }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = two_point_snapshot();
    assert_eq!(RewritingInference::infer(&snapshot), RewritingInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(RewritingInference::infer(&RewritingSnapshot::default()), RewritingInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_bounds_matches_rule_layout_extents() {
    let snapshot = two_point_snapshot();
    let inferred = RewritingInference::infer(&snapshot);
    assert_eq!(inferred.bounds.node_count, 2);
    assert_eq!(inferred.bounds.bounding_box.min_x, -140.0);
    assert_eq!(inferred.bounds.bounding_box.min_y, 0.0);
    assert_eq!(inferred.bounds.bounding_box.max_x, 0.0);
    assert_eq!(inferred.bounds.bounding_box.max_y, 80.0);
}
//#endregion 🧪️InferenceLaws
