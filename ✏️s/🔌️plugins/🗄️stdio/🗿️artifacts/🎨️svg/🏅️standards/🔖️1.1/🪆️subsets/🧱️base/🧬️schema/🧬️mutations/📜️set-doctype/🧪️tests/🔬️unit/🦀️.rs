use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::SvgDiff;
#[allow(unused_imports)]
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype;
use crate::standards::v1_1::subsets::base::schema::mutations::set_doctype::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetDoctypePayload as protocol::MutationKind<SvgSnapshot, crate::standards::v1_1::subsets::base::schema::mutations::SvgMutation>>::SEMANTICS.kind, "set-doctype");
}
