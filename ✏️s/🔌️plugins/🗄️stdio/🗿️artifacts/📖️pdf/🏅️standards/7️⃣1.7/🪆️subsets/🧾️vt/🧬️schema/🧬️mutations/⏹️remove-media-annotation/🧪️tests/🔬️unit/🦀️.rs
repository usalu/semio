use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::vt::schema::mutations::insert_media_annotation::InsertMediaAnnotation;
use crate::standards::v1_7::subsets::vt::schema::mutations::PdfVtMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::vt::schema::mutations::remove_media_annotation::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_only_the_matching_media_annotation() {
    let base = testkit::document_of(vec![support::media_annotation_object("Sound", "narration")]);
    let mutation = RemoveMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string() };
    let next = applied(&base, &PdfVtMutation::RemoveMediaAnnotation(mutation.clone()));
    assert!(support::media_annotation(&next, &mutation.subtype, &mutation.title).is_none());
    assert_eq!(<RemoveMediaAnnotation as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&testkit::document(), &PdfVtMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string(), placements: Vec::new() }));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveMediaAnnotation(RemoveMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfVtMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string(), placements: Vec::new() })));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveMediaAnnotation(RemoveMediaAnnotation { subtype: "Sound".to_string(), title: "narration".to_string() }), &base).await;
}
