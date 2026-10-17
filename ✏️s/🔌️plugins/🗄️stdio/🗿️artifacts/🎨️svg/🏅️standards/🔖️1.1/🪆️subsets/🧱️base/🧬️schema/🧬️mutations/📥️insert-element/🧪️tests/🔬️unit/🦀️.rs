use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::{schema::diff::diff_at_path, schema::diff::SvgChildAdded, schema::diff::SvgChildrenDiff, schema::diff::SvgDiff, schema::diff::SvgElementDiff, schema::diff::SvgNodeDiff};
#[allow(unused_imports)]
use crate::{schema::snapshot::NodePath, schema::snapshot::SvgNode};
use crate::standards::v1_1::subsets::base::schema::mutations::insert_element::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<InsertElementPayload as protocol::MutationKind<SvgSnapshot, crate::standards::v1_1::subsets::base::schema::mutations::SvgMutation>>::SEMANTICS.kind, "insert-element");
}
