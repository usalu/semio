use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    let id = support::insert_object(&mut base, support::struct_tree_root_object());
    support::set_catalog_entry(&mut base, "StructTreeRoot", PdfObject::Ref(id));
    let mutation = RemoveStructTreeRoot {};
    let outcome = <RemoveStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::catalog_entry(&next, "StructTreeRoot").is_none());
    assert_eq!(<RemoveStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot {})]);
}
