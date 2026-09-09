use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    let page = support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Page".to_string()))]));
    let mutation = SetTrimBox { page_index: 0, trim_box: [1.0, 2.0, 300.0, 400.0] };
    let outcome = <SetTrimBox as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert_eq!(support::page_box(&next, page, "TrimBox"), Some([1.0, 2.0, 300.0, 400.0]));
    assert_eq!(<SetTrimBox as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base), vec![PdfXMutation::RemoveTrimBox(RemoveTrimBox { page_index: 0 })]);
}
