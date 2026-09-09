use super::*;
use protocol::MutationDiff;

#[test]
fn removes_only_the_matching_media_annotation() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::media_annotation_object("Sound", "narration"));
    let mutation = RemoveMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string() };
    let outcome = <RemoveMediaAnnotation as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::media_annotation(&next, &mutation.subtype, &mutation.title).is_none());
    assert_eq!(<RemoveMediaAnnotation as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).len(), 1);
}
