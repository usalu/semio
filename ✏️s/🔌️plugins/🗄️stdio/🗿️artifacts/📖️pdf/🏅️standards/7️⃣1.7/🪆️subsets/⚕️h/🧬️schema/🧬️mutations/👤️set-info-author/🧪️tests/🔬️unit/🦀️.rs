use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfInfo;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn sets_and_can_restore_the_document_author() {
    let base = PdfSnapshot { info: PdfInfo { author: Some("before".to_string()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
    let mutation = SetInfoAuthor { author: "after".to_string() };
    let next = applied(&base, &PdfHMutation::SetInfoAuthor(mutation.clone()));
    assert_eq!(next.info.author.as_deref(), Some("after"));
    assert_eq!(<SetInfoAuthor as MutationKind<PdfSnapshot, PdfHMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfHMutation::SetInfoAuthor(SetInfoAuthor { author: "before".to_string() })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = PdfSnapshot { info: PdfInfo { author: Some("before".to_string()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
    assert_mutation_inverse_sum_law(&PdfHMutation::SetInfoAuthor(SetInfoAuthor { author: "after".to_string() }), &base).await;
}
