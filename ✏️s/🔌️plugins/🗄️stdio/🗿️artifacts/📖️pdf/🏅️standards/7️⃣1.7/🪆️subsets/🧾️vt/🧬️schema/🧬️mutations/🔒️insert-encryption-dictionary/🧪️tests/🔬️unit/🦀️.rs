use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_requested_security_handler() {
    let base = PdfSnapshot::default();
    let mutation = InsertEncryptionDictionary { version: 2, revision: 3 };
    let next = applied(&base, &PdfVtMutation::InsertEncryptionDictionary(mutation.clone()));
    assert!(support::encryption_dictionary_with(&next, 2, 3).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfVtMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3 }), &base).await;
}
