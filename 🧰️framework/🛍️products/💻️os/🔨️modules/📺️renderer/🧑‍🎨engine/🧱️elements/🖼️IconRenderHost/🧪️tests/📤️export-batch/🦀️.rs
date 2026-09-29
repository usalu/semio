//! 🧪️ Shared export admission, failure isolation and cancellation laws.
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📤️export-batch/🔣️.json")).unwrap()
}

fn requests(value: &serde_json::Value) -> Vec<semio_framework::kernel::IconRenderExportItem> {
    value["invalid"].as_array().unwrap().iter().map(|item| semio_framework::kernel::IconRenderExportItem {
        filename: item["filename"].as_str().unwrap().into(),
        request: semio_framework::DslValue::from(&item["request"]),
    }).collect()
}

#[test]
fn icon_export_effect_admits_large_batches_and_scopes_cancellation_to_existing_requests() {
    let fixture = fixture();
    for scenario in fixture["queueCases"].as_array().unwrap() {
        let mut shell = ShellState::new(Vec::new(), String::new());
        let groups = scenario["groups"].as_array().unwrap();
        for index in 0..=groups.len() {
            if scenario["cancelBeforeGroup"].as_u64() == Some(index as u64) { shell.cancel_icon_export(); }
            if index == groups.len() { break; }
            let items = (0..groups[index].as_u64().unwrap()).map(|item| semio_framework::kernel::IconRenderExportItem {
                filename: format!("{index}-{item}.png"), request: semio_framework::DslValue::from(&fixture["invalid"][0]["request"]),
            }).collect();
            shell.queue_host_effects("export-queue-law", vec![semio_framework::kernel::Effect::IconRenderExport { items }]);
        }
        let expected = &scenario["expected"];
        let batch = shell.icon_export.as_mut().expect("every export effect is admitted");
        assert_eq!(batch.counts().1, expected["admitted"].as_u64().unwrap() as usize, "{}", scenario["name"]);
        for _ in 0..2048 {
            let before = batch.items.iter().map(|group| group.items.len()).sum::<usize>();
            batch.advance();
            let after = batch.items.iter().map(|group| group.items.len()).sum::<usize>();
            assert!(before - after <= 1, "a pump releases at most one queued item");
            if !batch.running() { break; }
        }
        assert!(batch.terminal_is_empty(), "{}", scenario["name"]);
        assert_eq!(batch.failed, expected["failed"].as_u64().unwrap() as usize);
        assert_eq!(batch.discarded, expected["discarded"].as_u64().unwrap() as usize);
        assert_eq!(batch.phase_key(), expected["phase"].as_str().unwrap());
        assert_eq!(batch.completed, 0);
        assert!(shell.error.is_none());
    }
}

#[test]
fn icon_export_batch_invalid_items_settle_without_fetching_or_saving() {
    let fixture = fixture();
    let mut batch = IconExportBatch::new(requests(&fixture));
    for _ in 0..64 {
        batch.advance();
        if !batch.running() { break; }
    }
    assert!(!batch.running());
    assert_eq!(batch.phase_key(), fixture["expected"]["phase"].as_str().unwrap());
    assert_eq!(batch.completed, fixture["expected"]["completed"].as_u64().unwrap() as usize);
    assert_eq!(batch.failed, fixture["expected"]["failed"].as_u64().unwrap() as usize);
    assert!(batch.asset.is_none() && batch.saving.is_none() && batch.initializing.is_none());
}

#[test]
fn icon_export_batch_cancel_discards_queued_items_incrementally() {
    let fixture = fixture();
    let mut batch = IconExportBatch::new(requests(&fixture));
    batch.cancel();
    for _ in 0..64 {
        batch.advance();
        if !batch.running() { break; }
    }
    assert_eq!(batch.phase_key(), fixture["expected"]["cancelPhase"].as_str().unwrap());
    assert_eq!(batch.items.len(), fixture["expected"]["remainingAfterCancel"].as_u64().unwrap() as usize);
    assert_eq!(batch.completed, 0);
    assert_eq!(batch.failed, 0);
    assert!(batch.terminal_is_empty());
    for (locale, is_de) in [("en", false), ("de", true)] {
        assert_eq!(super::super::shell_chrome_string("icon.export.cancel", is_de), fixture["labels"][locale]["cancel"].as_str().unwrap());
        assert_eq!(super::super::shell_chrome_string("common.close", is_de), fixture["labels"][locale]["close"].as_str().unwrap());
    }
}
 

#[test]
fn icon_export_effect_publishes_an_accessible_cancel_control_and_drains_on_activation() {
    let fixture = fixture();
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.queue_host_effects("export-law", vec![semio_framework::kernel::Effect::IconRenderExport { items: requests(&fixture) }]);
    assert!(shell.settle_pump_pending());
    let mut cursor = super::super::ShellChromeChildCursor::default();
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::default();
    for _ in 0..1024 {
        if shell.render_icon_export_step(&mut cursor, &mut draw, &mut atlas, &mut input, &ui_wgpu::wgpu::Theme::default(), 1280.0) { break; }
    }
    input.publish_hits();
    let nodes = shell.chrome_accessibility_nodes(input.hits());
    let cancel = nodes.iter().find(|node| node.key == CONTROL_ID).expect("export cancel button");
    assert_eq!(cancel.role, "button");
    assert!(cancel.focusable && cancel.tabbable && cancel.actionable);
    assert_eq!(cancel.label.as_deref(), Some("Cancel"));
    let status = nodes.iter().find(|node| node.key == "shell.icon-export.progress").expect("export progress");
    assert_eq!(status.role, "progressbar");
    assert_eq!(status.value_max, Some(2.0));
    assert!(status.busy);
    let target = ui_render::AccessibilityTarget {
        window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.into(),
        window_generation: 1,
        node_id: cancel.node_id,
        node_key: cancel.key.clone(),
    };
    shell.presented_chrome_accessibility = nodes;
    shell.presented_chrome_accessibility_generation = 1;
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).unwrap());
    for _ in 0..64 { shell.advance_icon_export(); }
    assert!(shell.icon_export.as_ref().unwrap().terminal_is_empty());
    assert_eq!(shell.icon_export.as_ref().unwrap().phase_key(), "cancelled");
    assert!(shell.close_icon_export_step());
}
