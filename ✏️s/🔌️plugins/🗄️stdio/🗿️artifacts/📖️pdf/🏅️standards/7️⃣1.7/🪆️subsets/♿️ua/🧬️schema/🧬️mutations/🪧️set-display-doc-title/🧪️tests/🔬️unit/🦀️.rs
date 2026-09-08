
use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    let mutation = SetDisplayDocTitle { display: true };
    let outcome = <SetDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert_eq!(support::catalog_flag(&next, "ViewerPreferences", "DisplayDocTitle"), Some(true));
    assert_eq!(<SetDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::RemoveDisplayDocTitle(RemoveDisplayDocTitle {})]);
}
