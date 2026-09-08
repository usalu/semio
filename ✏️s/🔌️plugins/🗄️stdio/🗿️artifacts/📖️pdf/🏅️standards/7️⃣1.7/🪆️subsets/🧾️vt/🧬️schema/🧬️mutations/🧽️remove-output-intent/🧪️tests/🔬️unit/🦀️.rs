
use super::super::set_output_intent::{OUTPUT_INTENT_DEST_PROFILE, OUTPUT_INTENT_SUBTYPE};
use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use protocol::MutationDiff;

#[test]
fn removes_the_catalog_output_intent() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    support::set_output_intent(&mut base, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE);
    let mutation = RemoveOutputIntent {};
    let outcome = <RemoveOutputIntent as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::output_intent_identifier(&next).is_none());
    assert_eq!(<RemoveOutputIntent as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).len(), 1);
}
