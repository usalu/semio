use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_named_file_specification() {
    let base = PdfSnapshot::default();
    let mutation = InsertEmbeddedFile { file_name: "measurements.csv".to_string(), placements: Vec::new() };
    let next = applied(&base, &PdfAMutation::InsertEmbeddedFile(mutation.clone()));
    assert!(support::file_spec_named(&next, &mutation.file_name).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfAMutation::InsertEmbeddedFile(InsertEmbeddedFile { file_name: "measurements.csv".to_string(), placements: Vec::new() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = support::with_tail(&support::document());
    let mutation = PdfAMutation::InsertEmbeddedFile(InsertEmbeddedFile { file_name: "measurements.csv".to_string(), placements: vec![support::placed(98, 1), support::placed(99, 2)] });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}
