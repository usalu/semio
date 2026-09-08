
use super::*;
use protocol::MutationDiff;

#[test]
fn removes_the_matching_script_action() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::action_object("JavaScript", "JS", "audit"));
    let mutation = RemoveJavascriptAction { script: "audit".to_string() };
    let outcome = <RemoveJavascriptAction as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::action_with(&next, "JavaScript", "JS", "audit").is_none());
}
