use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use crate::standards::v1_7::subsets::e::schema::mutations::PdfEMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::e::schema::mutations::remove_media_annotation::RemoveMediaAnnotation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::e::schema::mutations::insert_media_annotation::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_requested_media_annotation() {
    let base = PdfSnapshot::default();
    let mutation = InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string(), placements: Vec::new() };
    let next = applied(&base, &PdfEMutation::InsertMediaAnnotation(mutation.clone()));
    assert!(support::media_annotation(&next, &mutation.subtype, &mutation.title).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document();
    assert_mutation_inverse_sum_law(&PdfEMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string(), placements: Vec::new() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = testkit::with_tail(&testkit::document());
    let mutation = PdfEMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string(), placements: vec![testkit::placed(98, 1), testkit::placed(99, 2)] });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}
