
use super::*;

#[test]
fn hit_at_prefers_content_registered_after_scroll_region() {
    let mut input = InputState::<()>::default();
    let scroll = Rect::new(0.0, 0.0, 200.0, 200.0);
    let row = Rect::new(0.0, 24.0, 200.0, 24.0);
    input.register_hit(HitTarget { rect: scroll, event: None, control_id: Some("scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    input.register_hit(HitTarget { rect: row, event: None, control_id: Some("tree.label.item-1".into()), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
    assert!(input.hit_at(10.0, 36.0).is_none(), "a registry the frame build has not published yet resolves nothing");
    input.publish_hits();
    let hit = input.hit_at(10.0, 36.0).expect("row point should hit");
    assert_eq!(hit.control_id.as_deref(), Some("tree.label.item-1"));
    assert_eq!(hit.kind, HitKind::TreeItem);
}

#[test]
fn event_queue_has_fixed_credits_and_transfers_one_fifo_item() {
    let mut input = InputState::<ActionDescriptor>::default();
    for index in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        let action = format!("event-{index}");
        input.reserve_action("controller", &action, 128).expect("reservation").publish().expect("queue credit");
    }
    assert_eq!(input.pending_actions.len(), crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY);
    for expected in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        assert_eq!(input.take_action_step().expect("authority").expect("event").into_descriptor().expect("materialized").action, format!("event-{expected}"));
    }
    assert!(input.take_action_step().expect("authority").is_none());
}

#[test]
fn event_queue_close_retires_one_owned_slot_per_step() {
    let mut input = InputState::<ActionDescriptor>::default();
    input.reserve_action("controller", "first", 128).expect("first").publish().expect("first queue");
    input.reserve_action("controller", "second", 128).expect("second").publish().expect("second queue");
    assert!(!input.close_step().expect("first retirement step"));
    assert_eq!(input.pending_actions.len(), 1);
    assert!(!input.close_step().expect("second retirement step"));
    assert!(input.pending_actions.is_empty());
}

#[test]
fn saturated_reservation_does_not_consume_semantic_source_and_retry_keeps_fifo() {
    let mut input = InputState::<ActionDescriptor>::default();
    for index in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        let action = format!("event-{index}");
        input.reserve_action("controller", &action, 128).expect("reservation").publish().expect("publish");
    }
    let semantic_source = String::from("retry-owned");
    let result = input.publish_action("controller", "retry", 128, |builder, _| {
        builder.begin_object(None)?;
        builder.string(Some("value"), &semantic_source)?;
        builder.end_container()
    });
    assert_eq!(result, Err(BoundedActionFault::ItemCredits));
    assert_eq!(semantic_source, "retry-owned");
    assert_eq!(input.take_action_step().expect("authority").expect("first").into_descriptor().expect("descriptor").action, "event-0");
    input
        .publish_action("controller", "retry", 128, |builder, _| {
            builder.begin_object(None)?;
            builder.string(Some("value"), &semantic_source)?;
            builder.end_container()
        })
        .expect("retry publication");
    let mut last = None;
    while let Some(action) = input.take_action_step().expect("authority") {
        last = Some(action.into_descriptor().expect("descriptor"));
    }
    assert_eq!(last.expect("last").action, "retry");
}

#[test]
fn action_fault_is_observed_before_another_queued_owner() {
    let mut input = InputState::<ActionDescriptor>::default();
    input.reserve_action("controller", "queued", 128).expect("reservation").publish().expect("publish");
    input.record_action_fault(BoundedActionFault::ByteCredits);
    assert!(matches!(input.take_action_step(), Err(BoundedActionFault::ByteCredits)));
    assert_eq!(input.take_action_step().expect("authority").expect("queued").into_descriptor().expect("descriptor").action, "queued");
}

//#region 🔢️InputConstraints
// 🔢️ LAW: `InputProps`' `min`/`max`/`step` are enforced, not decoration. React hands them to a real
// `<input type="number" min max step>` (`🗣️Interpreter/🟦️.tsx`'s `InputView`) and the browser refuses
// an out-of-range commit for it; an immediate-mode canvas has no browser, so this target enforces
// them itself — at the one commit authority, and in the `InputMeta` the chrome's own commit reads.

#[test]
fn number_constraints_clamp_and_snap_exactly_once_each() {
    use crate::wgpu::events::constrain_number_input;
    assert_eq!(constrain_number_input(999.0, Some(0.0), Some(10.0), None), 10.0);
    assert_eq!(constrain_number_input(-4.0, Some(2.0), Some(10.0), None), 2.0);
    assert_eq!(constrain_number_input(3.3, None, None, Some(0.5)), 3.5, "snapping is to the nearest step, not a floor");
    assert_eq!(constrain_number_input(3.4, Some(1.0), None, Some(2.0)), 3.0, "the step ladder starts at `min`, not at zero");
    assert_eq!(constrain_number_input(7.0, None, None, Some(0.0)), 7.0, "a zero step is no step, never a division");
    assert_eq!(constrain_number_input(7.25, None, None, None), 7.25);
    assert!(constrain_number_input(f64::NAN, Some(0.0), Some(10.0), Some(1.0)).is_nan(), "an unparseable buffer stays a refusal — never an invented in-range number");
}

#[test]
fn an_input_metas_commit_value_carries_its_own_constraints() {
    let text = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "text".into(), min: None, max: None, step: None, accept: None };
    assert_eq!(text.commit_value("12abc"), Some(dsl::DslValue::String("12abc".into())), "a text field commits its text verbatim");

    let number = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "number".into(), min: Some(0.0), max: Some(10.0), step: Some(0.5), accept: None };
    assert_eq!(number.commit_value("99"), Some(dsl::DslValue::float(10.0)));
    assert_eq!(number.commit_value("3.3"), Some(dsl::DslValue::float(3.5)));
    assert_eq!(number.commit_value(""), None, "an empty number buffer commits nothing at all");

    let file = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "file".into(), min: None, max: None, step: None, accept: Some("image/*".into()) };
    assert_eq!(file.accept.as_deref(), Some("image/*"), "a file field's `accept` reaches the host picker instead of being dropped at the render call site");
}
//#endregion 🔢️InputConstraints
