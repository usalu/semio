use crate::standards::v1::subsets::any::io::text::mutations::*;
use crate::empty_playbook_snapshot;

#[semio_framework_async_macros::async_test]
async fn change_title_op_sets_title() {
    let spec = empty_playbook_snapshot();
    let next = apply_playbook_mutation(&spec, &change_title_operation(Some("Renamed".into()))).expect("valid mutation diff");
    assert_eq!(next.title.as_deref(), Some("Renamed"));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_for_every_kind() {
    use protocol::OpText;
    for op in [change_title_operation(Some("Recipe".into())), change_title_operation(None)] {
        let line = op.print_op();
        assert!(!line.contains('\n'));
        assert_eq!(PlaybookMutation::parse_op(&line).expect("parse"), op);
    }
}
