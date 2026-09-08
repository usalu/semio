
use super::*;
use protocol::MutationDiff;

#[test]
fn sets_the_relationship_on_the_named_file() {
    let mut base = PdfSnapshot::default();
    support::insert_file_spec(&mut base, "measurements.csv");
    let mutation = SetAfRelationship { file_name: "measurements.csv".to_string(), relationship: "Data".to_string() };
    let outcome = <SetAfRelationship as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    let id = support::file_spec_named(&next, &mutation.file_name).unwrap();
    assert_eq!(support::object(&next, id).and_then(|value| support::dict_name(value, "AFRelationship")), Some("Data"));
}
