use super::*;
use super::super::set_output_intent::{OUTPUT_INTENT_DEST_PROFILE, OUTPUT_INTENT_SUBTYPE};
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_the_catalog_output_intent() {
    let catalog = support::document_of(vec![support::catalog_object()]);
    let base = support::after_rows(&catalog, support::output_intent_rows(&catalog, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE));
    let mutation = RemoveOutputIntent {};
    let next = applied(&base, &PdfVtMutation::RemoveOutputIntent(mutation.clone()));
    assert!(support::output_intent_identifier(&next).is_none());
    assert_eq!(next.objects.len(), 1, "the intent and its profile leave with the entry");
    assert_eq!(<RemoveOutputIntent as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = { let catalog = support::document_of(vec![support::catalog_object()]); support::after_rows(&catalog, support::output_intent_rows(&catalog, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE)) };
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveOutputIntent(RemoveOutputIntent {}), &base).await;
}
