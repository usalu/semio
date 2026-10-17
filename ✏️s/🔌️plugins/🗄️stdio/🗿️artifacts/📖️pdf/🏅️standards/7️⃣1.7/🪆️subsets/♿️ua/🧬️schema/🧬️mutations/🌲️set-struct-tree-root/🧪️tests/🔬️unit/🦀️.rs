use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::ua::schema::mutations::remove_struct_tree_root::RemoveStructTreeRoot;
use crate::standards::v1_7::subsets::ua::schema::mutations::PdfUaMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::ua::schema::mutations::set_struct_tree_root::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
    let base = testkit::document_of(vec![testkit::catalog_object()]);
    let mutation = SetStructTreeRoot { placements: Vec::new(), entry_index: None };
    let next = applied(&base, &PdfUaMutation::SetStructTreeRoot(mutation.clone()));
    let Some(PdfObject::Ref(id)) = support::catalog_entry(&next, "StructTreeRoot") else { panic!("structure tree root reference") };
    assert_eq!(support::object(&next, *id), Some(&support::struct_tree_root_object()));
    assert_eq!(<SetStructTreeRoot as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![PdfUaMutation::RemoveStructTreeRoot(RemoveStructTreeRoot {})]);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document();
    assert_mutation_inverse_sum_law(&PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot { placements: Vec::new(), entry_index: None }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = testkit::with_tail(&testkit::document());
    let mutation = PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot { placements: vec![testkit::placed(98, 1), testkit::placed(99, 2)], entry_index: None });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn replacing_an_installed_entry_inverts_position_exactly() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot { placements: Vec::new(), entry_index: None })));
    assert_mutation_inverse_sum_law(&PdfUaMutation::SetStructTreeRoot(SetStructTreeRoot { placements: Vec::new(), entry_index: None }), &base).await;
}
