use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::vt::schema::mutations::set_dpart_metadata::SetDpartMetadata;
use crate::standards::v1_7::subsets::vt::schema::mutations::PdfVtMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_metadata::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let catalog = testkit::document_of(vec![testkit::catalog_object()]);
    let base = crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&catalog, support::dpart_root_rows(&catalog, "run 4711", &[], None));
    let mutation = RemoveDpartMetadata {};
    let next = applied(&base, &PdfVtMutation::RemoveDpartMetadata(mutation.clone()));
    assert!(support::dpart_job(&next).is_none());
    assert_eq!(<RemoveDpartMetadata as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job: "run 4711".to_string(), entry_index: None })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = { let catalog = testkit::document_of(vec![testkit::catalog_object()]); crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&catalog, support::dpart_root_rows(&catalog, "run 4711", &[], None)) };
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveDpartMetadata(RemoveDpartMetadata {}), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = { let catalog = testkit::document_of(vec![testkit::catalog_object()]); crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&catalog, support::dpart_root_rows(&catalog, "run 4711", &[], None)) };
    let base = testkit::with_trailing_entry(&base, support::dpart_root_node(&base).expect("the fixture holds the root node"));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveDpartMetadata(RemoveDpartMetadata {}), &base).await;
}
