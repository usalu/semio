use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_only_a_present_security_handler() {
    let base = support::document_of(vec![support::encryption_dictionary(2, 3)]);
    let mutation = RemoveEncryptionDictionary { version: 2, revision: 3 };
    let next = applied(&base, &PdfXMutation::RemoveEncryptionDictionary(mutation.clone()));
    assert!(support::encryption_dictionary_with(&next, 2, 3).is_none());
    assert_eq!(<RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfXMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3 }));
    assert_mutation_inverse_sum_law(&PdfXMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: 2, revision: 3 }), &base).await;
}
