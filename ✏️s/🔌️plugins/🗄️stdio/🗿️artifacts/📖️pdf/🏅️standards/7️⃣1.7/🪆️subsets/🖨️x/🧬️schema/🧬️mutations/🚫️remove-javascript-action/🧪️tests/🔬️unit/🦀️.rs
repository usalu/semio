use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::MutationKind;
use crate::standards::v1_7::subsets::x::schema::mutations::insert_javascript_action::InsertJavascriptAction;
use crate::standards::v1_7::subsets::x::schema::mutations::PdfXMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::x::schema::mutations::remove_javascript_action::*;
use crate::standards::v1_7::subsets::base::schema::testkit::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_the_matching_script_action() {
    let base = testkit::document_of(vec![support::action_object("JavaScript", "JS", "audit")]);
    let mutation = RemoveJavascriptAction { script: "audit".to_string() };
    let next = applied(&base, &PdfXMutation::RemoveJavascriptAction(mutation.clone()));
    assert!(support::action_with(&next, "JavaScript", "JS", "audit").is_none());
    assert_eq!(<RemoveJavascriptAction as MutationKind<PdfSnapshot, PdfXMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&testkit::document(), &PdfXMutation::InsertJavascriptAction(InsertJavascriptAction { script: "audit".to_string(), placements: Vec::new() }));
    assert_mutation_inverse_sum_law(&PdfXMutation::RemoveJavascriptAction(RemoveJavascriptAction { script: "audit".to_string() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = testkit::with_tail(&applied(&testkit::document(), &PdfXMutation::InsertJavascriptAction(InsertJavascriptAction { script: "audit".to_string(), placements: Vec::new() })));
    assert_mutation_inverse_sum_law(&PdfXMutation::RemoveJavascriptAction(RemoveJavascriptAction { script: "audit".to_string() }), &base).await;
}
