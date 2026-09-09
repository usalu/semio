use super::*;
use protocol::MutationDiff;

#[test]
fn inserts_the_launch_target() {
    let base = PdfSnapshot::default();
    let mutation = InsertLaunchAction { target: "render.bat".to_string() };
    let outcome = <InsertLaunchAction as MutationKind<PdfSnapshot, PdfXMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::action_with(&next, "Launch", "F", &mutation.target).is_some());
}
