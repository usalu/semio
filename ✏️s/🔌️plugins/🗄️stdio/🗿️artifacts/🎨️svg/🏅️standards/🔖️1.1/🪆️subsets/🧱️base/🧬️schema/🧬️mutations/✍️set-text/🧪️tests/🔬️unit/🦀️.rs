use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::{schema::diff::diff_at_path, schema::diff::SvgDiff, schema::diff::SvgNodeDiff};
#[allow(unused_imports)]
use crate::{schema::snapshot::node_at, schema::snapshot::NodePath, schema::snapshot::SvgNode};
use crate::standards::v1_1::subsets::base::schema::mutations::set_text::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetTextPayload as protocol::MutationKind<SvgSnapshot, crate::standards::v1_1::subsets::base::schema::mutations::SvgMutation>>::SEMANTICS.kind, "set-text");
}
