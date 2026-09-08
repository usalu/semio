
use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    let mutation = SetStructTreeRoot {};
    let outcome = <SetStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    let Some(PdfObject::Ref(id)) = support::catalog_entry(&next, "StructTreeRoot") else { panic!("structure tree root reference") };
    assert_eq!(support::object(&next, *id), Some(&support::struct_tree_root_object()));
    assert_eq!(<SetStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::RemoveStructTreeRoot(RemoveStructTreeRoot {})]);
}
