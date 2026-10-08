use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn installs_the_conformance_output_intent() {
    let base = support::document_of(vec![support::catalog_object()]);
    let mutation = SetOutputIntent { identifier: "sRGB IEC61966-2.1".to_string() };
    let next = applied(&base, &PdfVtMutation::SetOutputIntent(mutation.clone()));
    assert_eq!(support::output_intent_identifier(&next).as_deref(), Some("sRGB IEC61966-2.1"));
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfVtMutation::SetOutputIntent(SetOutputIntent { identifier: "sRGB IEC61966-2.1".to_string() }), &base).await;
}
