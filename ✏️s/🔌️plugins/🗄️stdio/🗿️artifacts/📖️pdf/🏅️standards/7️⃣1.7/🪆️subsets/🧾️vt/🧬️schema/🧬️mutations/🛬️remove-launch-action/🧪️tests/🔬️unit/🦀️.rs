
use super::*;
use protocol::MutationDiff;

#[test]
fn removes_the_matching_launch_target() {
    let mut base = PdfSnapshot::default();
    support::insert_object(&mut base, support::action_object("Launch", "F", "render.bat"));
    let mutation = RemoveLaunchAction { target: "render.bat".to_string() };
    let outcome = <RemoveLaunchAction as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
    let next = outcome.diff().apply(&base).unwrap();
    assert!(support::action_with(&next, "Launch", "F", "render.bat").is_none());
}
