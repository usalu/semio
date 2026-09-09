use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    let mutation = SetDpartRoot { job: "run 4711".to_string() };
    let outcome = <SetDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::catalog_entry(&next, "DPartRoot").is_some());
    assert_eq!(support::dpart_job(&next).as_deref(), Some("run 4711"));
    assert_eq!(<SetDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base), vec![PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {})]);
}
