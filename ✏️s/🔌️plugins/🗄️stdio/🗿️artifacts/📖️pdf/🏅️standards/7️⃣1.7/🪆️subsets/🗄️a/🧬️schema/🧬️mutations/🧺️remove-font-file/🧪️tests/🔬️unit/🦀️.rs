
use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use protocol::MutationDiff;

#[test]
fn detaches_and_can_restore_the_font_program() {
    let mut base = PdfSnapshot::default();
    let program = support::insert_object(&mut base, PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() });
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string())), ("FontFile2", PdfObject::Ref(program))]));
    let mutation = RemoveFontFile { descriptor_ordinal: 0 };
    let outcome = <RemoveFontFile as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    let descriptor = support::font_descriptors(&next)[0];
    assert!(support::font_program(&next, descriptor).is_none());
    assert_eq!(<RemoveFontFile as MutationKind<PdfSnapshot, PdfAMutation>>::inverse(&mutation, &base).len(), 1);
}
