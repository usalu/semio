
use super::*;
use protocol::MutationDiff;

#[test]
fn sets_and_can_restore_the_document_title() {
    let mut base = PdfSnapshot::default();
    base.info.title = Some("before".to_string());
    let mutation = SetInfoTitle { title: "after".to_string() };
    let outcome = <SetInfoTitle as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert_eq!(next.info.title.as_deref(), Some("after"));
    assert_eq!(<SetInfoTitle as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base), vec![PdfHMutation::SetInfoTitle(SetInfoTitle { title: "before".to_string() })]);
}
