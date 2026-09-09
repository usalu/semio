use super::*;
use protocol::MutationDiff;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
    support::set_dpart_root(&mut base, "before");
    let mutation = SetDpartMetadata { job: "after".to_string() };
    let outcome = <SetDpartMetadata as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert_eq!(support::dpart_job(&next).as_deref(), Some("after"));
    assert_eq!(<SetDpartMetadata as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base), vec![PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job: "before".to_string() })]);
}
