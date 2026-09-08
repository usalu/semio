
use super::*;
use protocol::MutationDiff;

#[test]
fn inserts_the_script_action() {
    let base = PdfSnapshot::default();
    let mutation = InsertJavascriptAction { script: "app.alert('audit');".to_string() };
    let outcome = <InsertJavascriptAction as MutationKind<PdfSnapshot, PdfHMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::action_with(&next, "JavaScript", "JS", &mutation.script).is_some());
}
