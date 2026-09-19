use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfIndirectObject, PdfObject};
use protocol::MutationDiff;

/// 📕️ The smallest document an `/AcroForm` can hang off: one `/Type /Catalog` root.
fn catalog_only() -> PdfSnapshot {
    PdfSnapshot { objects: vec![PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: support::dict(vec![("Type", PdfObject::Name("Catalog".into()))]) }], ..PdfSnapshot::default() }
}

#[test]
fn removes_and_can_restore_the_named_signature_field() {
    let mut base = catalog_only();
    support::insert_signature_field(&mut base, "Signature1");
    let mutation = RemoveSignatureField { name: "Signature1".to_string() };
    let outcome = <RemoveSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::signature_field_named(&next, &mutation.name).is_none());
    assert_eq!(<RemoveSignatureField as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base).len(), 1);
}
