use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::SvgDiff;
#[allow(unused_imports)]
use crate::{schema::mutation_support::attribute_diff_at_path, schema::mutation_support::prior_attribute};
#[allow(unused_imports)]
use crate::{schema::snapshot::NodePath, schema::snapshot::SvgAttributeValue, schema::snapshot::ViewBox};
use crate::standards::v1_1::subsets::base::schema::mutations::set_view_box::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetViewBoxPayload as protocol::MutationKind<SvgSnapshot, crate::standards::v1_1::subsets::base::schema::mutations::SvgMutation>>::SEMANTICS.kind, "set-view-box");
}
