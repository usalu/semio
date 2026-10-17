//! 🧪️ Shared export admission, failure isolation and cancellation laws.
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/📤️export-batch/🔣️.json")).unwrap()
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
        let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
        let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        let groups = scenario["groups"].as_array().unwrap();
        for index in 0..=groups.len() {
            if scenario["cancelBeforeGroup"].as_u64() == Some(index as u64) { shell.opening.cancel_icon_export(); }
            if index == groups.len() { break; }
            let items = (0..groups[index].as_u64().unwrap()).map(|item| semio_framework::kernel::IconRenderExportItem {
                filename: format!("{index}-{item}.png"), request: semio_framework::DslValue::from(&fixture["invalid"][0]["request"]),
            }).collect();
            WindowActionsCtx::of(&mut shell).queue_host_effects("export-queue-law", vec![semio_framework::kernel::Effect::IconRenderExport { items }]);
        }
        let expected = &scenario["expected"];
        let batch = shell.opening.icon_export.as_mut().expect("every export effect is admitted");
        assert_eq!(batch.counts().1, expected["admitted"].as_u64().unwrap() as usize, "{}", scenario["name"]);
        for _ in 0..2048 {
            let before = batch.items.iter().map(|group| group.items.len()).sum::<usize>();
            caller.turn(|cx|batch.advance(cx)).unwrap();
            let after = batch.items.iter().map(|group| group.items.len()).sum::<usize>();
            assert!(before - after <= 1, "a pump releases at most one queued item");
            if !batch.running() { break; }
        }
        assert!(batch.terminal_is_empty(), "{}", scenario["name"]);
        assert_eq!(batch.failed, expected["failed"].as_u64().unwrap() as usize);
        assert_eq!(batch.discarded, expected["discarded"].as_u64().unwrap() as usize);
        assert_eq!(batch.phase_key(), expected["phase"].as_str().unwrap());
        assert_eq!(batch.completed, 0);
        assert!(shell.chrome.error.is_none());
        caller.finish();
    }
}

#[test]
fn icon_export_batch_invalid_items_settle_without_fetching_or_saving() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let fixture = fixture();
    let mut batch = IconExportBatch::new(requests(&fixture));
    for _ in 0..64 {
        caller.turn(|cx|batch.advance(cx)).unwrap();
        if !batch.running() { break; }
    }
    assert!(!batch.running());
    assert_eq!(batch.phase_key(), fixture["expected"]["phase"].as_str().unwrap());
    assert_eq!(batch.completed, fixture["expected"]["completed"].as_u64().unwrap() as usize);
    assert_eq!(batch.failed, fixture["expected"]["failed"].as_u64().unwrap() as usize);
    assert!(batch.asset.is_none() && batch.saving.is_none() && batch.initializing.is_none());
    caller.finish();
}

#[test]
fn icon_export_batch_cancel_discards_queued_items_incrementally() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let fixture = fixture();
    let mut batch = IconExportBatch::new(requests(&fixture));
    batch.cancel();
    for _ in 0..64 {
        caller.turn(|cx|batch.advance(cx)).unwrap();
        if !batch.running() { break; }
    }
    assert_eq!(batch.phase_key(), fixture["expected"]["cancelPhase"].as_str().unwrap());
    assert_eq!(batch.items.len(), fixture["expected"]["remainingAfterCancel"].as_u64().unwrap() as usize);
    assert_eq!(batch.completed, 0);
    assert_eq!(batch.failed, 0);
    assert!(batch.terminal_is_empty());
    for (locale, is_de) in [("en", false), ("de", true)] {
        assert_eq!(super::shell_chrome_string("icon.export.cancel", is_de), fixture["labels"][locale]["cancel"].as_str().unwrap());
        assert_eq!(super::shell_chrome_string("common.close", is_de), fixture["labels"][locale]["close"].as_str().unwrap());
    }
    caller.finish();
}
 

#[test]
fn icon_export_effect_publishes_an_accessible_cancel_control_and_drains_on_activation() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let fixture = fixture();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    WindowActionsCtx::of(&mut shell).queue_host_effects("export-law", vec![semio_framework::kernel::Effect::IconRenderExport { items: requests(&fixture) }]);
    assert!(PointerCtx::of(&mut shell).settle_pump_pending());
    let mut cursor = super::ShellChromeChildCursor::default();
    let mut draw = semio_framework_ui_draw_contract::DrawList::default();
    let mut atlas = semio_framework_ui_text::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::default();
    for _ in 0..1024 {
        if PanelsViewCtx::of(&mut shell).render_icon_export_step(&mut cursor, &mut draw, &mut atlas, &mut input, &semio_framework_ui_theme::Theme::default(), 1280.0) { break; }
    }
    input.publish_hits();
    let nodes = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits());
    let cancel = nodes.iter().find(|node| node.key == CONTROL_ID).expect("export cancel button");
    assert_eq!(cancel.role, "button");
    assert!(cancel.focusable && cancel.tabbable && cancel.actionable);
    assert_eq!(cancel.label.as_deref(), Some("Cancel"));
    let status = nodes.iter().find(|node| node.key == "shell.icon-export.progress").expect("export progress");
    assert_eq!(status.role, "progressbar");
    assert_eq!(status.value_max, Some(2.0));
    assert!(status.busy);
    let target = ui_render::AccessibilityTarget {
        window_id: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.into(),
        window_generation: 1,
        node_id: cancel.node_id,
        node_key: cancel.key.clone(),
    };
    shell.chrome.presented_chrome_accessibility = nodes;
    shell.chrome.presented_chrome_accessibility_generation = 1;
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).unwrap());
    for _ in 0..64 { caller.turn(|cx|shell.opening.advance_icon_export(cx)).unwrap(); }
    assert!(shell.opening.icon_export.as_ref().unwrap().terminal_is_empty());
    assert_eq!(shell.opening.icon_export.as_ref().unwrap().phase_key(), "cancelled");
    assert!(caller.turn(|cx|shell.opening.close_icon_export_step(cx)).unwrap());
    caller.finish();
}

#[test]
fn icon_export_prepared_close_borrows_original_work_and_physical_receipt(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observed;
    use semio_framework_job::{InteractiveJobCloseStep as Close,RetainedCloneGrant,RetainedCloneProgress};
    let law=fixture();let law=&law["originalClose"];let expected=&law["expected"];
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let(mut text,birth)=observed(||law["text"].as_str().unwrap().to_owned());
    let bytes=text.capacity();assert_eq!(birth.requested_bytes,expected["backingBytes"].as_u64().unwrap()as usize);
    let demand=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()};
    let(denied,heap)=observed(||caller.turn(|cx|{cx.consume_fuel(1);semio_framework_os_renderer_wgpu_icon_host::icon_export::close_prepared(cx,Ok(demand),|_|panic!("zero work must preserve original backing"))}));
    assert!(!denied.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,expected["deniedBytes"].as_u64().unwrap()as usize));assert_eq!(text.capacity(),bytes);
    let(done,heap)=observed(||caller.turn(|cx|{let done=semio_framework_os_renderer_wgpu_icon_host::icon_export::close_prepared(cx,Ok(demand),|_|{drop(std::mem::take(&mut text));Ok(Close::Complete{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}})});assert_eq!(cx.retained_progress().released_bytes,bytes);done}));
    assert!(done.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,bytes));assert_eq!(observed(||drop(text)).1.released_bytes,expected["terminalDropBytes"].as_u64().unwrap()as usize);
    let failure=caller.turn(|cx|semio_framework_os_renderer_wgpu_icon_host::icon_export::close_prepared(cx,Ok(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}),|_|Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original icon producer refusal")))).unwrap_err();
    let IconExportFault::Original(failure)=failure else{panic!("complete original producer failure retained")};assert_eq!(failure.primary.message(),"original icon producer refusal");assert!(failure.receipt_refusal.is_none());assert!(failure.context_refusal.is_none());
    caller.finish();
}
