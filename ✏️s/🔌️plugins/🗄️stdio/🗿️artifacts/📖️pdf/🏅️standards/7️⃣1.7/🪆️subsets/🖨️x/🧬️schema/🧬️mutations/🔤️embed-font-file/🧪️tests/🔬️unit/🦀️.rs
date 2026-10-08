use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn attaches_the_program_to_the_selected_descriptor() {
    let base = support::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string()))])]);
    let program = ObjRef { num: 1, gen: 0 };
    let mutation = EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program, entry_index: None };
    let next = applied(&base, &PdfXMutation::EmbedFontFile(mutation.clone()));
    let descriptor = support::font_descriptors(&next)[0];
    assert_eq!(support::font_program(&next, descriptor), Some(("FontFile2".to_string(), program)));
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string()))])]);
    assert_mutation_inverse_sum_law(&PdfXMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: ObjRef { num: 1, gen: 0 }, entry_index: None }), &base).await;
}
