use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use crate::standards::v1_7::subsets::a::schema::mutations::insert_embedded_file::InsertEmbeddedFile;
use crate::standards::v1_7::subsets::a::schema::mutations::PdfAMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::a::schema::mutations::remove_embedded_file::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_the_named_file_specification() {
    let empty = PdfSnapshot::default();
    let base = applied(&empty, &PdfAMutation::InsertEmbeddedFile(InsertEmbeddedFile { file_name: "measurements.csv".to_string(), placements: Vec::new() }));
    let mutation = RemoveEmbeddedFile { file_name: "measurements.csv".to_string() };
    let next = applied(&base, &PdfAMutation::RemoveEmbeddedFile(mutation.clone()));
    assert!(support::file_spec_named(&next, &mutation.file_name).is_none());
    assert_eq!(next, empty, "the attached payload leaves with its specification");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&testkit::document(), &PdfAMutation::InsertEmbeddedFile(InsertEmbeddedFile { file_name: "measurements.csv".to_string(), placements: Vec::new() }));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { file_name: "measurements.csv".to_string() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfAMutation::InsertEmbeddedFile(InsertEmbeddedFile { file_name: "measurements.csv".to_string(), placements: Vec::new() })));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { file_name: "measurements.csv".to_string() }), &base).await;
}
