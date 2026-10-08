use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_the_matching_launch_target() {
    let base = support::document_of(vec![support::action_object("Launch", "F", "render.bat")]);
    let mutation = RemoveLaunchAction { target: "render.bat".to_string() };
    let next = applied(&base, &PdfAMutation::RemoveLaunchAction(mutation.clone()));
    assert!(support::action_with(&next, "Launch", "F", "render.bat").is_none());
    assert_eq!(<RemoveLaunchAction as MutationKind<PdfSnapshot, PdfAMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfAMutation::InsertLaunchAction(InsertLaunchAction { target: "render.bat".to_string(), placements: Vec::new() }));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveLaunchAction(RemoveLaunchAction { target: "render.bat".to_string() }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = support::with_tail(&applied(&support::document(), &PdfAMutation::InsertLaunchAction(InsertLaunchAction { target: "render.bat".to_string(), placements: Vec::new() })));
    assert_mutation_inverse_sum_law(&PdfAMutation::RemoveLaunchAction(RemoveLaunchAction { target: "render.bat".to_string() }), &base).await;
}
