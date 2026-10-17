use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use crate::standards::v1_7::subsets::x::schema::mutations::PdfXMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::x::schema::mutations::remove_launch_action::RemoveLaunchAction;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::x::schema::mutations::insert_launch_action::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_launch_target() {
    let base = PdfSnapshot::default();
    let mutation = InsertLaunchAction { target: "render.bat".to_string(), placements: Vec::new() };
    let next = applied(&base, &PdfXMutation::InsertLaunchAction(mutation.clone()));
    assert!(support::action_with(&next, "Launch", "F", &mutation.target).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = testkit::document();
    assert_mutation_inverse_sum_law(&PdfXMutation::InsertLaunchAction(InsertLaunchAction { target: "render.bat".to_string(), placements: Vec::new() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserts_at_the_placements_it_is_given() {
    let base = testkit::with_tail(&testkit::document());
    let mutation = PdfXMutation::InsertLaunchAction(InsertLaunchAction { target: "render.bat".to_string(), placements: vec![testkit::placed(98, 1), testkit::placed(99, 2)] });
    let next = applied(&base, &mutation);
    assert_eq!(next.objects[1].id.num, 98);
    assert_mutation_inverse_sum_law(&mutation, &base).await;
}
