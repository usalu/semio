use super::*;
use protocol::DiffAlgebra;

#[test]
fn an_empty_config_has_only_the_empty_diff() {
    let base = Block5dConfig::default();
    let diff = Block5dConfigDiff::default();
    assert!(diff.is_empty());
    assert_eq!(diff.inverse(&base), Block5dConfigDiff::default());
}
