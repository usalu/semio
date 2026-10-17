use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use crate::standards::v1_7::subsets::a::schema::mutations::PdfAMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::a::schema::mutations::remove_af_relationship::RemoveAfRelationship;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::a::schema::mutations::set_af_relationship::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn sets_the_relationship_on_the_named_file() {
    let empty = PdfSnapshot::default();
    let base = crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&empty, support::insert_file_spec_rows(&empty, "measurements.csv", &[]));
    let mutation = SetAfRelationship { file_name: "measurements.csv".to_string(), relationship: "Data".to_string(), entry_index: None };
    let next = applied(&base, &PdfAMutation::SetAfRelationship(mutation.clone()));
    let id = support::file_spec_named(&next, &mutation.file_name).unwrap();
    assert_eq!(support::object(&next, id).and_then(|value| support::dict_name(value, "AFRelationship")), Some("Data"));
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = { let empty = PdfSnapshot::default(); crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&empty, support::insert_file_spec_rows(&empty, "measurements.csv", &[])) };
    assert_mutation_inverse_sum_law(&PdfAMutation::SetAfRelationship(SetAfRelationship { file_name: "measurements.csv".to_string(), relationship: "Data".to_string(), entry_index: None }), &base).await;
}
