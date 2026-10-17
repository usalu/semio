use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::SvgDiff;
#[allow(unused_imports)]
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;
use crate::standards::v1_1::subsets::base::schema::mutations::set_declaration::*;
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetDeclarationPayload as protocol::MutationKind<SvgSnapshot, crate::standards::v1_1::subsets::base::schema::mutations::SvgMutation>>::SEMANTICS.kind, "set-declaration");
}
