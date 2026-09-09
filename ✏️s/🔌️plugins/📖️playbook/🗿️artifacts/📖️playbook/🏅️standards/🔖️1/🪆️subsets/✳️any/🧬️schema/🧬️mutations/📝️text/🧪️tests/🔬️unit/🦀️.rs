use super::*;
use crate::empty_playbook_snapshot;

#[semio_framework_async_macros::async_test]
async fn change_title_op_sets_title() {
    let spec = empty_playbook_snapshot();
    let mutation = change_title_operation(Some("Renamed".into()));
    let next = apply_playbook_mutation(&spec, &mutation).expect("valid mutation diff");
    assert_eq!(next.title.as_deref(), Some("Renamed"));
}

#[semio_framework_async_macros::async_test]
async fn apply_playbook_add_step_roundtrip() {
    let spec = empty_playbook_snapshot();
    let next = apply_playbook_mutation(&spec, &add_step_operation("step-test".into(), "Step test".into())).expect("valid mutation diff");
    assert_eq!(next.steps().len(), 2);
}

fn sample_block() -> crate::PlaybookBlock {
    crate::PlaybookBlock {
        id: "b1".into(),
        label: "Team size".into(),
        kind: "number".into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_for_every_kind() {
    use protocol::OpText;
    let ops = vec![
        add_step_operation("s2".into(), "Step 2".into()),
        remove_step_operation("s1"),
        move_step_operation("s1", 2),
        add_block_operation("s1", sample_block(), None),
        remove_block_operation("s1", "b1"),
        move_block_operation("b1", "s1", "s2", 0),
        replace_block_operation("s1", sample_block()),
        update_step_operation("s1", "Basics".into(), Some("d".into())),
        change_title_operation(Some("Recipe".into())),
    ];
    for op in ops {
        let line = op.print_op();
        assert!(!line.contains('\n'));
        assert_eq!(PlaybookMutation::parse_op(&line).expect("parse"), op);
    }
}
