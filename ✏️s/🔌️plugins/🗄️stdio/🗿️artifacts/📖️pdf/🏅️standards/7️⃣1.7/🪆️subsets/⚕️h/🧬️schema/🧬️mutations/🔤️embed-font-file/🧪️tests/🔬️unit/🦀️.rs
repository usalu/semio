
use super::*;
use protocol::MutationDiff;

#[test]
fn attaches_the_program_to_the_selected_descriptor() {
    let mut base = PdfSnapshot::default();
    let program = support::insert_object(&mut base, PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() });
    support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string()))]));
    let mutation = EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program };
    let outcome = <EmbedFontFile as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    let descriptor = support::font_descriptors(&next)[0];
    assert_eq!(support::font_program(&next, descriptor), Some(("FontFile2".to_string(), program)));
}
