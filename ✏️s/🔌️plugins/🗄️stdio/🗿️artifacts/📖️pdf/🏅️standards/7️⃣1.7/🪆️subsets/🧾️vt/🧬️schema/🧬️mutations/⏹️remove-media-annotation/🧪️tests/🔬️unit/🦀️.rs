use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_only_the_matching_media_annotation() {
    let base = support::document_of(vec![support::media_annotation_object("Sound", "narration")]);
    let mutation = RemoveMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string() };
    let next = applied(&base, &PdfVtMutation::RemoveMediaAnnotation(mutation.clone()));
    assert!(support::media_annotation(&next, &mutation.subtype, &mutation.title).is_none());
    assert_eq!(<RemoveMediaAnnotation as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfVtMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string() }));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveMediaAnnotation(RemoveMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string() }), &base).await;
}
