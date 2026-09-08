
use super::*;
use protocol::MutationDiff;

#[test]
fn inserts_the_named_file_specification() {
    let base = PdfSnapshot::default();
    let mutation = InsertEmbeddedFile { file_name: "measurements.csv".to_string() };
    let outcome = <InsertEmbeddedFile as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::file_spec_named(&next, &mutation.file_name).is_some());
}
