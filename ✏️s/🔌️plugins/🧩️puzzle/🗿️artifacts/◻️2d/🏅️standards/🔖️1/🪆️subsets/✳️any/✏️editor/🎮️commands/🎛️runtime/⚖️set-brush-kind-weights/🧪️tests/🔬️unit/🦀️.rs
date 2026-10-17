#[allow(unused_imports)]
use crate::editor::puzzle2d::{snapshot::puzzle2d_kind_ids, snapshot::puzzle2d_window_and_measures_scope, interaction::Puzzle2dActionCtx};
#[allow(unused_imports)]
use semio_framework_pack_json::{json, Value};
#[allow(unused_imports)]
use std::collections::BTreeMap;
use crate::editor::puzzle2d::commands::set_brush_kind_weights::*;

#[test]
fn kind_weight_group_normalizes_to_sum_one() {
    let ids = vec!["a".into(), "b".into(), "c".into()];
    let initial = puzzle2d_uniform_kind_weights(&ids);
    let next = puzzle2d_normalize_kind_weight_group(&initial, &ids, "a", 0.5);
    let sum: f64 = ids.iter().map(|id| next.get(id).copied().unwrap_or(0.0)).sum();
    assert!((sum - 1.0).abs() < 0.001, "expected normalized weights to sum to 1, got {sum}");
    assert!((next.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 0.001);
}
