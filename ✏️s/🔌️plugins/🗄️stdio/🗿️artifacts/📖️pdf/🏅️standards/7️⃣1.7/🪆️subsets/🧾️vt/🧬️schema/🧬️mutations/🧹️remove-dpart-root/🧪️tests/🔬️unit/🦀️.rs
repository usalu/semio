use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root::SetDpartRoot;
use crate::standards::v1_7::subsets::vt::schema::mutations::PdfVtMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_root::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
    let catalog = testkit::document_of(vec![testkit::catalog_object()]);
    let base = crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&catalog, support::dpart_root_rows(&catalog, "run 4711", &[], None));
    let mutation = RemoveDpartRoot {};
    let next = applied(&base, &PdfVtMutation::RemoveDpartRoot(mutation.clone()));
    assert!(support::catalog_entry(&next, "DPartRoot").is_none());
    assert_eq!(next, catalog, "the partition objects leave with the root entry");
    assert_eq!(<RemoveDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string(), placements: Vec::new(), entry_index: None })]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&testkit::document(), &PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string(), placements: Vec::new(), entry_index: None }));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {}), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string(), placements: Vec::new(), entry_index: None })));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {}), &base).await;
}
