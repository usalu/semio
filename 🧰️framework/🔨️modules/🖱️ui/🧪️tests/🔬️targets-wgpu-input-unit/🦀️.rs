
use super::*;

#[test]
fn hit_at_prefers_content_registered_after_scroll_region() {
    let mut input = InputState::<()>::default();
    let scroll = Rect::new(0.0, 0.0, 200.0, 200.0);
    let row = Rect::new(0.0, 24.0, 200.0, 24.0);
    input.register_hit(HitTarget { rect: scroll, event: None, control_id: Some("scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    input.register_hit(HitTarget { rect: row, event: None, control_id: Some("tree.label.item-1".into()), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
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
