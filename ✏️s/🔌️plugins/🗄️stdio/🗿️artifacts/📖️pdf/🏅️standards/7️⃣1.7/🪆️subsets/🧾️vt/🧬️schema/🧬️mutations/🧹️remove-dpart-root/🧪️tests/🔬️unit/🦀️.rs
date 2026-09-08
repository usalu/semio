
use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    support::set_dpart_root(&mut base, "run 4711");
    let mutation = RemoveDpartRoot {};
    let outcome = <RemoveDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::catalog_entry(&next, "DPartRoot").is_none());
    assert_eq!(<RemoveDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base), vec![PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string() })]);
}
