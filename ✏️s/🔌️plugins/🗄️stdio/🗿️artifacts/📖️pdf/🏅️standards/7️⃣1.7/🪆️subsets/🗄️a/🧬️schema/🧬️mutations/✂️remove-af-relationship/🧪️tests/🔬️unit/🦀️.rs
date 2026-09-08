
use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use protocol::MutationDiff;

#[test]
fn removes_and_can_restore_the_relationship() {
    let mut base = PdfSnapshot::default();
    let id = support::insert_file_spec(&mut base, "measurements.csv");
    support::set_entry(&mut base, id, "AFRelationship", PdfObject::Name("Data".to_string()));
    let mutation = RemoveAfRelationship { file_name: "measurements.csv".to_string() };
    let outcome = <RemoveAfRelationship as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    let next_id = support::file_spec_named(&next, &mutation.file_name).unwrap();
    assert!(support::object(&next, next_id).and_then(|value| support::dict_name(value, "AFRelationship")).is_none());
    assert_eq!(<RemoveAfRelationship as MutationKind<PdfSnapshot, PdfAMutation>>::inverse(&mutation, &base).len(), 1);
}
