/// 🐕️ The presentation watchdog's arithmetic, over `🧫️fixtures/🐕️present-stall-watch/🔣️.json`: a healthy
/// ladder is never reported however long it runs, and a frozen cursor is named exactly once, on the
/// ceiling step. `serde_json` is the independent oracle for the fixture.
mod present_stall_watch_tests {
    use super::*;

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🐕️present-stall-watch/🔣️.json")).expect("present stall watch fixture parses")
    }

    fn phase(tag: &str) -> AppPresentPhase {
        match tag {
            "Fullscreen" => AppPresentPhase::Fullscreen,
            "Engine" => AppPresentPhase::Engine,
            "BeginGpu" => AppPresentPhase::BeginGpu,
            "Uploads" => AppPresentPhase::Uploads,
            "Stage" => AppPresentPhase::Stage,
            "Render" => AppPresentPhase::Render,
            "CloseGpu" => AppPresentPhase::CloseGpu,
            "Acknowledge" => AppPresentPhase::Acknowledge,
            "ProgressAcknowledge" => AppPresentPhase::ProgressAcknowledge,
            "Directives" => AppPresentPhase::Directives,
            _ => AppPresentPhase::Aborted,
        }
    }

    fn signature(row: &serde_json::Value) -> AppPresentProgress {
        let row = row.as_array().expect("fixture signature");
        let gpu = row[3].as_array().map(|gpu| {
            (
                gpu[0].as_u64().expect("gpu phase") as u8,
                gpu[1].as_u64().expect("gpu command") as usize,
                gpu[2].as_u64().expect("gpu glass command") as usize,
                gpu[3].as_u64().expect("gpu blur mip") as u32,
            )
        });
        (phase(row[0].as_str().expect("phase tag")), row[1].as_u64().expect("engine index") as usize, row[2].as_u64().expect("upload index") as usize, gpu)
    }

    #[test]
    fn the_presentation_watchdog_names_a_frozen_cursor_once_and_a_healthy_one_never() {
        let fixture = fixture();
        assert_eq!(fixture["ceilingSteps"].as_u64(), Some(u64::from(APP_PRESENT_STALL_STEPS)), "the fixture pins the shipped ceiling");
        for case in fixture["cases"].as_array().expect("fixture cases") {
            let signatures: Vec<_> = case["signatures"].as_array().expect("signatures").iter().map(signature).collect();
            let repeat = case["repeatEachFor"].as_u64().expect("repeat count");
            let mut watch = AppPresentStallWatch::default();
            let mut reports = Vec::new();
            for _ in 0..repeat {
                for entry in &signatures {
                    if let Some(shape) = note_present_stall_signature(&mut watch, *entry) {
                        reports.push(shape);
                    }
                }
            }
            let why = case["why"].as_str().unwrap_or_default();
            assert_eq!(reports.len(), case["expectReports"].as_u64().expect("expected reports") as usize, "{why}");
            if let Some(shape) = case["expectShape"].as_str() {
                assert_eq!(reports[0], shape, "{why}");
            }
        }
    }

    #[test]
    fn a_cursor_that_moves_after_the_ceiling_rearms_the_watchdog() {
        let mut watch = AppPresentStallWatch::default();
        let frozen: AppPresentProgress = (AppPresentPhase::Engine, 0usize, 0usize, None);
        let mut reports = 0;
        for _ in 0..(APP_PRESENT_STALL_STEPS * 2) {
            reports += usize::from(note_present_stall_signature(&mut watch, frozen).is_some());
        }
        assert_eq!(reports, 1);
        assert!(note_present_stall_signature(&mut watch, (AppPresentPhase::Engine, 1, 0, None)).is_none(), "one moved index rearms the watchdog");
        assert_eq!(watch.steps, 0);
        for _ in 0..(APP_PRESENT_STALL_STEPS * 2) {
            reports += usize::from(note_present_stall_signature(&mut watch, (AppPresentPhase::Engine, 1, 0, None)).is_some());
        }
        assert_eq!(reports, 2, "a second freeze is reported once on its own ceiling");
    }
}
