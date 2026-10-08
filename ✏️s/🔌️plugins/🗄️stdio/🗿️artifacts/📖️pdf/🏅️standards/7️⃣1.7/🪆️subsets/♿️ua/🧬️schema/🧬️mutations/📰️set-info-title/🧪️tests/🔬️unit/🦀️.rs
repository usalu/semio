use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfInfo;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn sets_and_can_restore_the_document_title() {
    let base = PdfSnapshot { info: PdfInfo { title: Some("before".to_string()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
    let mutation = SetInfoTitle { title: "after".to_string() };
    let next = applied(&base, &PdfUaMutation::SetInfoTitle(mutation.clone()));
    assert_eq!(next.info.title.as_deref(), Some("after"));
    assert_eq!(<SetInfoTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::SetInfoTitle(SetInfoTitle { title: "before".to_string() })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = PdfSnapshot { info: PdfInfo { title: Some("before".to_string()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
    assert_mutation_inverse_sum_law(&PdfUaMutation::SetInfoTitle(SetInfoTitle { title: "after".to_string() }), &base).await;
}
