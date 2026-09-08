
use super::*;

#[test]
fn kind_weight_group_normalizes_to_sum_one() {
    let ids = vec!["a".into(), "b".into(), "c".into()];
    let initial = puzzle2d_uniform_kind_weights(&ids);
    let next = puzzle2d_normalize_kind_weight_group(&initial, &ids, "a", 0.5);
    let sum: f64 = ids.iter().map(|id| next.get(id).copied().unwrap_or(0.0)).sum();
    assert!((sum - 1.0).abs() < 0.001, "expected normalized weights to sum to 1, got {sum}");
    assert!((next.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 0.001);
}
