use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_launch_target() {
    let base = PdfSnapshot::default();
    let mutation = InsertLaunchAction { target: "render.bat".to_string() };
    let next = applied(&base, &PdfAMutation::InsertLaunchAction(mutation.clone()));
    assert!(support::action_with(&next, "Launch", "F", &mutation.target).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfAMutation::InsertLaunchAction(InsertLaunchAction { target: "render.bat".to_string() }), &base).await;
}
