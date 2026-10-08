use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_requested_media_annotation() {
    let base = PdfSnapshot::default();
    let mutation = InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string() };
    let next = applied(&base, &PdfVtMutation::InsertMediaAnnotation(mutation.clone()));
    assert!(support::media_annotation(&next, &mutation.subtype, &mutation.title).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfVtMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string() }), &base).await;
}
