use super::*;
use crate::LayoutPoint;
use std::collections::BTreeMap;

#[semio_framework_async_macros::async_test]
async fn empty_rule_layout_yields_default_bounds() {
    assert_eq!(compute_bounds(&RewritingSnapshot::default()), RewritingBounds::default());
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_rule_layout_extents() {
    let mut rule_layout = BTreeMap::new();
    rule_layout.insert("a".to_string(), LayoutPoint { x: 0.0, y: 0.0 });
    rule_layout.insert("b".to_string(), LayoutPoint { x: -140.0, y: 80.0 });
    let snapshot = RewritingSnapshot { rule_layout, ..RewritingSnapshot::default() };
    let bounds = compute_bounds(&snapshot);
    assert_eq!(bounds.node_count, 2);
    assert_eq!(bounds.bounding_box, RewritingBoundingBox { min_x: -140.0, min_y: 0.0, max_x: 0.0, max_y: 80.0 });
}
