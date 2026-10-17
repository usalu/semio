use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::x::schema::mutations::PdfXMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::x::schema::mutations::embed_font_file::EmbedFontFile;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::x::schema::mutations::remove_font_file::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn detaches_and_can_restore_the_font_program() {
    let base = testkit::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string())), ("FontFile2", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))])]);
    let mutation = RemoveFontFile { descriptor_ordinal: 0 };
    let next = applied(&base, &PdfXMutation::RemoveFontFile(mutation.clone()));
    let descriptor = support::font_descriptors(&next)[0];
    assert!(support::font_program(&next, descriptor).is_none());
    assert_eq!(<RemoveFontFile as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string())), ("FontFile2", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))])]);
    assert_mutation_inverse_sum_law(&PdfXMutation::RemoveFontFile(RemoveFontFile { descriptor_ordinal: 0 }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::document_of(vec![PdfObject::Stream { dict: Vec::new(), data: b"font".to_vec(), filters: Vec::new() }, support::dict(vec![("Type", PdfObject::Name("FontDescriptor".to_string())), ("FontFile2", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))])]);
    let base = testkit::with_trailing_entry(&base, support::font_descriptors(&base)[0]);
    assert_mutation_inverse_sum_law(&PdfXMutation::RemoveFontFile(RemoveFontFile { descriptor_ordinal: 0 }), &base).await;
}
