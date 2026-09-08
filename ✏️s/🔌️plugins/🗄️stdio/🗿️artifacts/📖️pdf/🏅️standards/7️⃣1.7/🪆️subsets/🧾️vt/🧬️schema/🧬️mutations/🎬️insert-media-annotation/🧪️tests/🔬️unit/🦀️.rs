
use super::*;
use protocol::MutationDiff;

#[test]
fn inserts_the_requested_media_annotation() {
    let base = PdfSnapshot::default();
    let mutation = InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string() };
    let outcome = <InsertMediaAnnotation as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::media_annotation(&next, &mutation.subtype, &mutation.title).is_some());
}
