use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::h::schema::mutations::PdfHMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::h::schema::mutations::remove_font_file::RemoveFontFile;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::h::schema::mutations::embed_font_file::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn attaches_the_program_to_the_selected_descriptor() {
    let base = testkit::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string()))])]);
    let program = ObjRef { num: 1, gen: 0 };
    let mutation = EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program, entry_index: None };
    let next = applied(&base, &PdfHMutation::EmbedFontFile(mutation.clone()));
    let descriptor = support::font_descriptors(&next)[0];
    assert_eq!(support::font_program(&next, descriptor), Some(("FontFile2".to_string(), program)));
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string()))])]);
    assert_mutation_inverse_sum_law(&PdfHMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: ObjRef { num: 1, gen: 0 }, entry_index: None }), &base).await;
}
