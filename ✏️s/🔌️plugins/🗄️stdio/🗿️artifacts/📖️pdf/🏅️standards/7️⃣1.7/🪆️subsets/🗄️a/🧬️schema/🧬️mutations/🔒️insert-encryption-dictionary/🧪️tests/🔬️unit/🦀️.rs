use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_requested_security_handler() {
    let base = PdfSnapshot::default();
    let mutation = InsertEncryptionDictionary { version: 2, revision: 3, placements: Vec::new() };
    let next = applied(&base, &PdfAMutation::InsertEncryptionDictionary(mutation.clone()));
    assert!(support::encryption_dictionary_with(&next, 2, 3).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfAMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3, placements: Vec::new() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = support::with_tail(&support::document());
    let mutation = PdfAMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3, placements: vec![support::placed(98, 1), support::placed(99, 2)] });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}
