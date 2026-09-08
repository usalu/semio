use super::*;
#[test]
fn direct_fixture_leaf_contract() {
    super::super::assert_fixture_descriptor::<AddN>(include_str!("../../🔣️.json"));
}
