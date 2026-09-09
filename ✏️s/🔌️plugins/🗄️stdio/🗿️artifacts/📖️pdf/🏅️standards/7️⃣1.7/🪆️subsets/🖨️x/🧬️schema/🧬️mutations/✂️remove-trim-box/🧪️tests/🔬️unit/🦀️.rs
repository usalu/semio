use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    let page = support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Page".to_string())), ("TrimBox", support::box_object([1.0, 2.0, 300.0, 400.0]))]));
    let mutation = RemoveTrimBox { page_index: 0 };
    let outcome = <RemoveTrimBox as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::page_box(&next, page, "TrimBox").is_none());
    assert_eq!(<RemoveTrimBox as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base), vec![PdfXMutation::SetTrimBox(SetTrimBox { page_index: 0, trim_box: [1.0, 2.0, 300.0, 400.0] })]);
}
