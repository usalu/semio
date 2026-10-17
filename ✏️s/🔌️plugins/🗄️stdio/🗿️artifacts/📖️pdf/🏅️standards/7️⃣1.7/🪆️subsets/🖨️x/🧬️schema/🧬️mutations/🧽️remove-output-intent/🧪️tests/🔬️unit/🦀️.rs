use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::x::schema::mutations::PdfXMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::x::schema::mutations::set_output_intent::SetOutputIntent;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::x::schema::mutations::remove_output_intent::*;
use crate::standards::v1_7::subsets::x::schema::mutations::set_output_intent::{OUTPUT_INTENT_DEST_PROFILE, OUTPUT_INTENT_SUBTYPE};
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_the_catalog_output_intent() {
    let catalog = testkit::document_of(vec![testkit::catalog_object()]);
    let base = crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&catalog, support::output_intent_rows(&catalog, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE, &[], None));
    let mutation = RemoveOutputIntent {};
    let next = applied(&base, &PdfXMutation::RemoveOutputIntent(mutation.clone()));
    assert!(support::output_intent_identifier(&next).is_none());
    assert_eq!(next.objects.len(), 1, "the intent and its profile leave with the entry");
    assert_eq!(<RemoveOutputIntent as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = { let catalog = testkit::document_of(vec![testkit::catalog_object()]); crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&catalog, support::output_intent_rows(&catalog, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE, &[], None)) };
    assert_mutation_inverse_sum_law(&PdfXMutation::RemoveOutputIntent(RemoveOutputIntent {}), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::with_tail(&{ let catalog = testkit::document_of(vec![testkit::catalog_object()]); crate::standards::v1_7::subsets::base::schema::testkit::after_rows(&catalog, support::output_intent_rows(&catalog, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE, &[], None)) });
    assert_mutation_inverse_sum_law(&PdfXMutation::RemoveOutputIntent(RemoveOutputIntent {}), &base).await;
}
