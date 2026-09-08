
use super::*;
use protocol::MutationDiff;

#[test]
fn removes_the_named_file_specification() {
    let mut base = PdfSnapshot::default();
    support::insert_file_spec(&mut base, "measurements.csv");
    let mutation = RemoveEmbeddedFile { file_name: "measurements.csv".to_string() };
    let outcome = <RemoveEmbeddedFile as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::file_spec_named(&next, &mutation.file_name).is_none());
}
