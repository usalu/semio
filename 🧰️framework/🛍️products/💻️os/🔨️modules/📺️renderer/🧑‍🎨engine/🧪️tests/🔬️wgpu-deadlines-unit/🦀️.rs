use super::*;

#[test]
fn presenter_checkouts_park_accepted_animation_and_expired_control_clocks_until_handback() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap();
    let mut animation = AcceptedAnimationClock::default();
    let mut scheduler = ui_render::FrameScheduler::new();
    animation.accept(true);
    for row in fixture["presenterWait"]["deadlineTurns"].as_array().unwrap() {
        let now = row["atMs"].as_f64().unwrap() / 1000.0;
        let _ = scheduler.should_render(now);
        let control = Some(ui_render::Deadline { due: now + row["controlDueMs"].as_f64().unwrap() / 1000.0, reason: InvalidationReason::INPUT_STATE });
        sync_presented_deadlines(&mut scheduler, &mut animation, control, now, row["waiting"].as_bool().unwrap());
        match row["nextMs"].as_f64() {
            Some(expected) => assert!((scheduler.next_deadline().unwrap().due * 1000.0 - expected).abs() < 0.000001, "{row}"),
            None => assert!(scheduler.next_deadline().is_none(), "{row}"),
        }
    }
    scheduler.request_deadline(5.0, InvalidationReason::RESOURCE_READY);
    sync_presented_deadlines(&mut scheduler, &mut animation, None, 4.0, true);
    assert_eq!(scheduler.next_deadline().unwrap().due, 5.0);
}

#[test]
fn accepted_animation_clock_survives_discard_and_stops_after_static_presentation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🖌️render/⏱️schedule/🧫️fixtures/🎞️animation/🔣️.json")).unwrap();
    let mut clock = AcceptedAnimationClock::default();
    let mut scheduler = ui_render::FrameScheduler::new();
    for step in fixture["activity"].as_array().unwrap() {
        let now = step["atMs"].as_f64().unwrap() / 1000.0;
        assert_eq!(scheduler.should_render(now).is_some(), step["wake"].as_bool().unwrap(), "{step}");
        if step["kind"] == "present" {
            clock.accept(step["active"].as_bool().unwrap());
        }
        clock.sync(&mut scheduler, now);
        match step["nextMs"].as_f64() {
            Some(expected) => {
                assert!((scheduler.next_deadline().unwrap().due * 1000.0 - expected).abs() < 0.000001, "{step}");
                assert_eq!(scheduler.next_deadline().unwrap().reason, InvalidationReason::ANIMATION);
            }
            None => assert!(scheduler.next_deadline().is_none(), "{step}"),
        }
    }
    clock.accept(false);
    scheduler.replace_deadline(RETAINED_CONTROL_CLOCK, Some(ui_render::Deadline { due: 2.0, reason: InvalidationReason::INPUT_STATE }));
    clock.sync(&mut scheduler, 1.0);
    assert_eq!(scheduler.next_deadline().unwrap().due, 2.0);
    clock.accept(true);
    clock.sync(&mut scheduler, f64::NAN);
    assert_eq!(scheduler.next_deadline().unwrap().due, 2.0);
}

#[test]
fn gpu_animation_clock_preserves_frame_precision_and_shader_periods_after_long_uptime() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🖌️render/⏱️schedule/🧫️fixtures/🎞️animation/🔣️.json")).unwrap();
    for row in fixture["samples"].as_array().unwrap() {
        let expected = row["phaseUs"].as_u64().unwrap() as f32 / 1_000_000.0;
        assert_eq!(ui_animation_seconds(row["nowUs"].as_u64()), expected, "{}", row["id"]);
    }
    let now = 1_790_000_000_000_000;
    assert_ne!(ui_animation_seconds(Some(now)), ui_animation_seconds(Some(now + 16_000)));
    assert_eq!(ui_animation_seconds(Some(u64::MAX)), (u64::MAX % 3_200_000) as f32 / 1_000_000.0);
}

#[test]
fn retained_control_deadlines_translate_monotonic_origins_without_epoch_loss() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🖌️render/🧫️fixtures/⏱️deadline/🔣️.json")).unwrap();
    for row in fixture["bridges"].as_array().unwrap() {
        let deadline = retained_control_deadline(row["dueUs"].as_u64(), row["nowUs"].as_u64(), row["hostSeconds"].as_f64().unwrap());
        assert_eq!(deadline.map(|deadline| deadline.reason), row["expectedSeconds"].as_f64().map(|_| InvalidationReason::INPUT_STATE));
        assert_eq!(deadline.map(|deadline| deadline.due), row["expectedSeconds"].as_f64(), "{}", row["id"]);
    }
    assert!(retained_control_deadline(Some(5), None, 0.0).is_none());
    assert!(retained_control_deadline(Some(5), Some(0), f64::NAN).is_none());
}

//#region 🧩️NativeHotSwapPoll

#[test]
fn hot_swap_poll_is_due_immediately_on_first_call() {
    let mut poll = HotSwapPoll::new();
    assert!(poll.is_due(0.0));
}

#[test]
fn hot_swap_poll_is_not_due_again_inside_the_window() {
    let mut poll = HotSwapPoll::new();
    assert!(poll.is_due(0.0));
    assert!(!poll.is_due(NATIVE_HOT_SWAP_POLL_SECONDS - 0.001));
}

#[test]
fn hot_swap_poll_is_due_again_once_the_window_elapses() {
    let mut poll = HotSwapPoll::new();
    assert!(poll.is_due(0.0));
    assert!(poll.is_due(NATIVE_HOT_SWAP_POLL_SECONDS));
}

//#endregion 🧩️NativeHotSwapPoll
