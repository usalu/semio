//! 🧪️ The shell tutorial slot replays the shared fixture the TS descriptor parser replays.
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧊️wgpu/🎬️media-slots/🎞️shell-tutorial-video/🔣️.json")).unwrap()
}

#[test]
fn the_shell_slot_replays_the_shared_fixture() {
    for case in fixture()["cases"].as_array().unwrap() {
        let input = &case["input"];
        let rect = input["rect"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect::<Vec<_>>();
        let src = input.get("src").and_then(|value| value.as_str()).map(str::to_owned).unwrap_or_else(|| input["srcRepeat"]["unit"].as_str().unwrap().repeat(input["srcRepeat"]["times"].as_u64().unwrap() as usize));
        let slot = shell_tutorial_video_slot(Rect::new(rect[0], rect[1], rect[2], rect[3]), &src, input["muted"].as_bool().unwrap(), input["playing"].as_bool().unwrap(), input["rate"].as_f64().unwrap() as f32, input["localTimeMs"].as_f64().unwrap());
        let expected = (!case["slot"].is_null()).then(|| serde_json::to_value(serde_json::from_value::<PresentedMediaSlot>(case["slot"].clone()).unwrap()).unwrap());
        assert_eq!(slot.map(|slot| serde_json::to_value(slot).unwrap()), expected, "{}", case["name"]);
    }
}

#[test]
fn the_shell_slot_joins_last_and_never_past_the_slot_ceiling() {
    let slot = || shell_tutorial_video_slot(Rect::new(0.0, 0.0, 10.0, 10.0), "https://example.test/a.mp4", false, true, 1.0, 0.0);
    let mut slots = vec![slot().unwrap()];
    push_shell_tutorial_video_slot(&mut slots, slot());
    assert_eq!(slots[1].paint_order, 1);
    let mut full: Vec<_> = (0..PRESENTED_MEDIA_SLOT_CAPACITY).map(|_| slot().unwrap()).collect();
    push_shell_tutorial_video_slot(&mut full, slot());
    assert_eq!(full.len(), PRESENTED_MEDIA_SLOT_CAPACITY);
}
