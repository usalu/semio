use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::a::schema::mutations::insert_encryption_dictionary::InsertEncryptionDictionary;
use crate::standards::v1_7::subsets::a::schema::mutations::PdfAMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::a::schema::mutations::remove_encryption_dictionary::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_only_a_present_security_handler() {
    let base = testkit::document_of(vec![support::encryption_dictionary(2, 3)]);
    let mutation = RemoveEncryptionDictionary { version: 2, revision: 3 };
    let next = applied(&base, &PdfAMutation::RemoveEncryptionDictionary(mutation.clone()));
    assert!(support::encryption_dictionary_with(&next, 2, 3).is_none());
    assert_eq!(<RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfAMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&testkit::document(), &PdfAMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3, placements: Vec::new() }));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: 2, revision: 3 }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfAMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3, placements: Vec::new() })));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: 2, revision: 3 }), &base).await;
}
