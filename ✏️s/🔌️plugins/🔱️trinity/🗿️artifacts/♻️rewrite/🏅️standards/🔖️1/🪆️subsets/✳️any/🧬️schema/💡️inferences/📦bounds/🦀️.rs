//! 📦 `bounds` — one named inference: the 2d bounding box + node count of `rule_layout`, the only
//! positioned data this rule-editing artifact's snapshot carries a typed shape for
//! (`before_fixture_json`/`lhs_json`/`rhs_json` are opaque JSON blobs, not structured data). A
//! plain whole-snapshot scalar (per the family root's own "simple whole-snapshot scalars"
//! guidance) — no `InferredField`/incremental caching needed for a handful of `{x, y}` points.

use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Bounds
/// 📦 Axis-aligned 2d bounding box over `rule_layout`'s node positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RewriteBoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

/// 📦 Bounding box + node count over `rule_layout`.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RewriteBounds {
    pub bounding_box: RewriteBoundingBox,
    pub node_count: u32,
}

/// 📐️ Computes `bounds` directly from `rule_layout` — an empty layout yields `RewriteBounds::default()`.
pub fn compute_bounds(snapshot: &RewriteSnapshot) -> RewriteBounds {
    let node_count = snapshot.rule_layout.len() as u32;
    if snapshot.rule_layout.is_empty() {
        return RewriteBounds::default();
    }
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for point in snapshot.rule_layout.values() {
        min_x = min_x.min(point.x);
        min_y = min_y.min(point.y);
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
    }
    RewriteBounds { bounding_box: RewriteBoundingBox { min_x, min_y, max_x, max_y }, node_count }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::rewrite::LayoutPoint;
    use std::collections::BTreeMap;

    #[semio_framework_async_macros::async_test]
    async fn empty_rule_layout_yields_default_bounds() {
        assert_eq!(compute_bounds(&RewriteSnapshot::default()), RewriteBounds::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn bounds_matches_rule_layout_extents() {
        let mut rule_layout = BTreeMap::new();
        rule_layout.insert("a".to_string(), LayoutPoint { x: 0.0, y: 0.0 });
        rule_layout.insert("b".to_string(), LayoutPoint { x: -140.0, y: 80.0 });
        let snapshot = RewriteSnapshot { rule_layout, ..RewriteSnapshot::default() };
        let bounds = compute_bounds(&snapshot);
        assert_eq!(bounds.node_count, 2);
        assert_eq!(bounds.bounding_box, RewriteBoundingBox { min_x: -140.0, min_y: 0.0, max_x: 0.0, max_y: 80.0 });
    }
}
//#endregion 🧪️Tests
