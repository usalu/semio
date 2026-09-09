use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use protocol::MutationDiff;

#[test]
fn installs_the_pdf_x_output_intent() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    let mutation = SetOutputIntent { identifier: "sRGB IEC61966-2.1".to_string() };
    let outcome = <SetOutputIntent as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert_eq!(support::output_intent_identifier(&next).as_deref(), Some("sRGB IEC61966-2.1"));
}
