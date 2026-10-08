use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::catalog_object()]);
    let mutation = SetLang { lang: "de-DE".to_string() };
    let next = applied(&base, &PdfUaMutation::SetLang(mutation.clone()));
    assert_eq!(support::catalog_entry(&next, "Lang"), Some(&support::literal("de-DE")));
    assert_eq!(<SetLang as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::RemoveLang(RemoveLang {})]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string() }), &base).await;
}
