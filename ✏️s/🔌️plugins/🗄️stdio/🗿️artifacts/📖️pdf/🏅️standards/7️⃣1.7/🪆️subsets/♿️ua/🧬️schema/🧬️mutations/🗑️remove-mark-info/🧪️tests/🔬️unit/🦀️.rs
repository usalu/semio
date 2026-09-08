
use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    support::set_catalog_entry(&mut base, "MarkInfo", support::single_entry_dict("Marked", PdfObject::Bool(true)));
    let mutation = RemoveMarkInfo {};
    let outcome = <RemoveMarkInfo as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::catalog_entry(&next, "MarkInfo").is_none());
    assert_eq!(<RemoveMarkInfo as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::SetMarkInfo(SetMarkInfo { marked: true })]);
}
