use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfObject};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = support::document_of(vec![support::dict(vec![("Type", PdfObject::Name("Catalog".to_string())), ("Lang", support::literal("de-DE"))])]);
    let mutation = RemoveLang {};
    let next = applied(&base, &PdfUaMutation::RemoveLang(mutation.clone()));
    assert!(support::catalog_entry(&next, "Lang").is_none());
    assert_eq!(<RemoveLang as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string(), entry_index: None })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string(), entry_index: None }));
    assert_mutation_inverse_sum_law(&PdfUaMutation::RemoveLang(RemoveLang {}), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = support::with_tail(&applied(&support::document(), &PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string(), entry_index: None })));
    assert_mutation_inverse_sum_law(&PdfUaMutation::RemoveLang(RemoveLang {}), &base).await;
}
