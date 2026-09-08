
use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    support::set_catalog_entry(&mut base, "ViewerPreferences", support::single_entry_dict("DisplayDocTitle", PdfObject::Bool(true)));
    let mutation = RemoveDisplayDocTitle {};
    let outcome = <RemoveDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::catalog_entry(&next, "ViewerPreferences").is_none());
    assert_eq!(<RemoveDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display: true })]);
}
