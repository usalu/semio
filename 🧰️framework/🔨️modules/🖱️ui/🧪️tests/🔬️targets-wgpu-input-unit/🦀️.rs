use super::*;

#[test]
fn closing_input_reports_each_admitted_action_receipt_once() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️correlated-action-receipts/🔣️.json")).unwrap();
    let rows = fixture["receipts"].as_array().unwrap();
    let mut input = InputState::<()>::default();
    let mut batch = input.reserve_actions(rows.len(), rows.len() * 128).unwrap();
    for row in rows {
        batch
            .action(fixture["controller"].as_str().unwrap(), row["action"].as_str().unwrap(), 128, |builder| {
                builder.set_receipt(crate::wgpu::ActionQueueReceipt {
                    token: std::num::NonZeroU64::new(row["token"].as_u64().unwrap()).unwrap(),
                    member: row["member"].as_u64().unwrap() as u8,
                    abort_correlation_on_error: row["abort"].as_bool().unwrap(),
                })
            })
            .unwrap();
    }
    batch.publish().unwrap();
    let mut cancelled = Vec::new();
    for _ in 0..64 {
        if input.close_step_with_receipt(|receipt| cancelled.push(format!("{}:{}", receipt.token, receipt.member))).unwrap() {
            break;
        }
    }
    assert!(input.terminal_is_empty());
    assert_eq!(serde_json::json!(cancelled), fixture["closed"]);
    assert!(input.close_step_with_receipt(|_| panic!("receipt settled twice")).unwrap());
}

#[test]
fn a_reopened_text_owner_drains_the_prior_projection_before_projecting_its_current_value() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⌨️text-owner-lifecycle/🔣️.json")).expect("text owner lifecycle fixture");
    assert_eq!(fixture["sequence"], serde_json::json!(["focus", "advance-to-projection", "blur", "refocus", "drain"]));
    let owner = fixture["owner"].as_str().expect("owner").to_string();
    let value = fixture["value"].as_str().expect("value").to_string();
    let mut input = InputState::<()>::default();
    input.focus_input_owned(owner.clone(), value.clone());
    for _ in 0..256 {
        input.drive_text_step().expect("initial text step");
        if input.text_projection_pending {
            break;
        }
    }
    assert!(input.text_projection_pending, "the first owner has checked out a real bounded projection");
    input.blur_input();
    input.focus_input_owned(owner.clone(), value.clone());
    for turn in 0..2048 {
        let pending = input.drive_text_step().expect("close/reopen must not manufacture a protocol fault");
        if !pending && input.text_buffer.reserved_bytes() == 0 && !input.text_projection_pending {
            assert!(turn > 0, "the checked-out projection is drained incrementally");
            break;
        }
        assert!(turn < 2047, "the current owner settles under a fixed bound");
    }
    assert_eq!(input.focused_id.as_deref(), fixture["expected"]["owner"].as_str());
    assert_eq!(input.text_view(), fixture["expected"]["value"].as_str().expect("expected value"));

    let mut authority = ui_contract::TextEditAuthority::default();
    authority.start_projection(0, 1).expect("first projection");
    assert_eq!(authority.start_projection(0, 1), Err(ui_contract::TextEditFault::Protocol), "a genuine concurrent projection remains a precise protocol refusal");
}





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

#[test]
fn batch_length_step_observes_fault_then_preserves_the_complete_source_slice() {
    let mut input = InputState::<ActionDescriptor>::default();
    let mut batch = input.reserve_actions(2, 64).unwrap();
    batch.action("controller", "first", 32, |_| Ok(())).unwrap();
    batch.action("controller", "second", 32, |_| Ok(())).unwrap();
    batch.publish().unwrap();
    input.record_action_fault(BoundedActionFault::ByteCredits);
    assert_eq!(input.take_action_batch_len_step(), Err(BoundedActionFault::ByteCredits));
    assert_eq!(input.take_action_batch_len_step(), Ok(Some(2)));
    assert_eq!(input.take_action_step().unwrap().unwrap().into_descriptor().unwrap().action, "first");
    assert_eq!(input.take_action_batch_len_step(), Ok(Some(1)));
}



//#region 🔢️InputConstraints
// 🔢️ LAW: `InputProps`' `min`/`max`/`step` are enforced, not decoration. React hands them to a real
// `<input type="number" min max step>` (`🗣️Interpreter/🟦️.tsx`'s `InputView`) and the browser refuses
// an out-of-range commit for it; an immediate-mode canvas has no browser, so this target enforces
// them itself — at the one commit authority, and in the `InputMeta` the chrome's own commit reads.




//#endregion 🔢️InputConstraints

#[cfg(feature = "wgpu-engine")]
#[path = "../🔬️targets-wgpu-input-engine-unit/🦀️.rs"]
mod engine_tests;
