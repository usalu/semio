use super::*;
use protocol::DiffAlgebra;

#[test]
fn an_empty_config_has_only_the_empty_diff() {
    let base = Block2dConfig::default();
    let diff = Block2dConfigDiff::between(&base, &base);
    assert!(diff.is_empty());
    assert_eq!(diff.inverse(&base), Block2dConfigDiff::default());
}
