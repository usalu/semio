use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::{schema::diff::diff_at_path, schema::diff::SvgChildrenDiff, schema::diff::SvgDiff, schema::diff::SvgElementDiff, schema::diff::SvgNodeDiff};
#[allow(unused_imports)]
use crate::{schema::snapshot::node_at, schema::snapshot::NodePath, schema::snapshot::SvgNode};
use crate::standards::v1_1::subsets::base::schema::mutations::remove_element::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<RemoveElementPayload as protocol::MutationKind<SvgSnapshot, crate::standards::v1_1::subsets::base::schema::mutations::SvgMutation>>::SEMANTICS.kind, "remove-element");
}
