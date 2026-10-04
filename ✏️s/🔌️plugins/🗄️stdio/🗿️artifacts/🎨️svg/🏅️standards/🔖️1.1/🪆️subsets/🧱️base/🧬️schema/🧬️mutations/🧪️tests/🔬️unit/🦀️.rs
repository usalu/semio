use super::*;
use protocol::SemanticMutation;
#[test]
fn aggregate_roster_is_exact() {
    assert_eq!(SvgMutation::kinds().iter().map(|item|item.kind).collect::<Vec<_>>(),[
        "set-snapshot","patch-snapshot","set-declaration","set-doctype","insert-element","remove-element","set-element-name","set-attribute","set-text","set-view-box","set-transform"
    ]);
}
