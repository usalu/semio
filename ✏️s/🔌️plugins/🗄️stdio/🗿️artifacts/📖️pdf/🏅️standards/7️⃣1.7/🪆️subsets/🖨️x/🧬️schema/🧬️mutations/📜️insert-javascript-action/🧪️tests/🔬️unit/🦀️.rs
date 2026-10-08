use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn inserts_the_script_action() {
    let base = PdfSnapshot::default();
    let mutation = InsertJavascriptAction { script: "app.alert('audit');".to_string() };
    let next = applied(&base, &PdfXMutation::InsertJavascriptAction(mutation.clone()));
    assert!(support::action_with(&next, "JavaScript", "JS", &mutation.script).is_some());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = support::document();
    assert_mutation_inverse_sum_law(&PdfXMutation::InsertJavascriptAction(InsertJavascriptAction { script: "app.alert('audit');".to_string() }), &base).await;
}
