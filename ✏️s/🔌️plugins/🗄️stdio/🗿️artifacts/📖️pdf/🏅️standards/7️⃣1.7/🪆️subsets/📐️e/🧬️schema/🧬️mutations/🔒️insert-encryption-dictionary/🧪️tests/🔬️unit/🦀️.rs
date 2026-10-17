use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use crate::standards::v1_7::subsets::e::schema::mutations::PdfEMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::e::schema::mutations::remove_encryption_dictionary::RemoveEncryptionDictionary;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::e::schema::mutations::insert_encryption_dictionary::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_requested_security_handler() {
    let base = PdfSnapshot::default();
    let mutation = InsertEncryptionDictionary { version: 2, revision: 3, placements: Vec::new() };
    let next = applied(&base, &PdfEMutation::InsertEncryptionDictionary(mutation.clone()));
    assert!(support::encryption_dictionary_with(&next, 2, 3).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document();
    assert_mutation_inverse_sum_law(&PdfEMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3, placements: Vec::new() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = testkit::with_tail(&testkit::document());
    let mutation = PdfEMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3, placements: vec![testkit::placed(98, 1), testkit::placed(99, 2)] });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}
