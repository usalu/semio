
use super::*;
use protocol::MutationDiff;

#[test]
fn sets_and_can_restore_the_document_author() {
    let mut base = PdfSnapshot::default();
    base.info.author = Some("before".to_string());
    let mutation = SetInfoAuthor { author: "after".to_string() };
    let outcome = <SetInfoAuthor as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert_eq!(next.info.author.as_deref(), Some("after"));
    assert_eq!(<SetInfoAuthor as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base), vec![PdfHMutation::SetInfoAuthor(SetInfoAuthor { author: "before".to_string() })]);
}
