use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::a::schema::mutations::PdfAMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::a::schema::mutations::remove_output_intent::RemoveOutputIntent;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::a::schema::mutations::set_output_intent::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn installs_the_conformance_output_intent() {
    let base = testkit::document_of(vec![testkit::catalog_object()]);
    let mutation = SetOutputIntent { identifier: "sRGB IEC61966-2.1".to_string(), placements: Vec::new(), entry_index: None };
    let next = applied(&base, &PdfAMutation::SetOutputIntent(mutation.clone()));
    assert_eq!(support::output_intent_identifier(&next).as_deref(), Some("sRGB IEC61966-2.1"));
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document();
    assert_mutation_inverse_sum_law(&PdfAMutation::SetOutputIntent(SetOutputIntent { identifier: "sRGB IEC61966-2.1".to_string(), placements: Vec::new(), entry_index: None }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = testkit::with_tail(&testkit::document());
    let mutation = PdfAMutation::SetOutputIntent(SetOutputIntent { identifier: "sRGB IEC61966-2.1".to_string(), placements: vec![testkit::placed(98, 1), testkit::placed(99, 2)], entry_index: None });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn replacing_an_installed_entry_inverts_position_exactly() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfAMutation::SetOutputIntent(SetOutputIntent { identifier: "first".to_string(), placements: Vec::new(), entry_index: None })));
    assert_mutation_inverse_sum_law(&PdfAMutation::SetOutputIntent(SetOutputIntent { identifier: "second".to_string(), placements: Vec::new(), entry_index: None }), &base).await;
}
