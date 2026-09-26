use super::*;

#[test]
fn secondary_pointer_button_uses_context_menu_code() {
    assert_eq!(pointer_button_to_i16(PointerButton::Secondary), 2);
}

#[test]
fn normalized_host_maps_table_stepper_navigation_keys_without_text_fallback() {
    for (key, expected) in [
        ("Home", ui_wgpu::wgpu::KeyAction::Home),
        ("End", ui_wgpu::wgpu::KeyAction::End),
        ("PageUp", ui_wgpu::wgpu::KeyAction::PageUp),
        ("PageDown", ui_wgpu::wgpu::KeyAction::PageDown),
    ] {
        assert_eq!(key_action_from_dispatch(key, true), Some(expected));
        assert_eq!(key_action_from_dispatch(key, false), None);
    }
}

#[test]
fn normalized_native_ime_commit_preserves_the_exact_unicode_payload() {
    let text = "日本é";
    assert_eq!(ime_event_from_winit(&winit::event::Ime::Commit(text.into())), ui_render::ImeEvent::Commit { text: text.into() });
    assert_eq!(ime_event_from_winit(&winit::event::Ime::Preedit(text.into(), Some((0, text.len())))), ui_render::ImeEvent::Update { text: text.into(), cursor: text.len() });
    assert_eq!(ime_event_from_winit(&winit::event::Ime::Disabled), ui_render::ImeEvent::Cancel);
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct OsShortcutFixture {
    command_id: String,
    bindings: Vec<OsShortcutBinding>,
    collision: OsShortcutCollision,
    retired_chord: String,
}

#[derive(serde::Deserialize)]
struct OsShortcutBinding {
    platform: String,
    chord: String,
}

#[derive(serde::Deserialize)]
struct OsShortcutCollision {
    matches: String,
    refuses: String,
}

fn keyboard_interaction() -> crate::AppInteractionState {
    crate::AppInteractionState {
        shell: crate::shell::ShellState::new(Vec::new(), String::new()),
        input: ui_wgpu::wgpu::InputState::default(),
        theme: ui_wgpu::wgpu::Theme::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: crate::shell::PointerCapture::default(),
        modifiers: ui_wgpu::wgpu::PointerModifiers::default(),
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        caret_blink_at_ms: 0.0,
        caret_blink_visible: true,
        text_streams: std::array::from_fn(|_| None),
        text_fault: None,
        frame_fault: None,
        text_cancel_pending: false,
        #[cfg(not(target_arch = "wasm32"))]
        last_sync_pump_ms: 0.0,
    }
}

#[test]
fn normalized_host_key_routes_the_platform_os_command_to_the_shell() {
    let fixture: OsShortcutFixture = serde_json::from_str(include_str!("../../🧫️fixtures/⌨️os-command-shortcuts/🔣️.json")).expect("OS shortcut fixture");
    assert_eq!(fixture.command_id, "os.toggleFullscreen");
    assert_eq!(fixture.retired_chord, "mod+shift+f");
    assert!(fixture.bindings.iter().any(|binding| binding.platform == "macOs" && binding.chord == "control+meta+f"));

    let restore = crate::boot_descriptor();
    crate::set_host_platform("MacIntel");
    let action = ui_wgpu::wgpu::KeyAction::Char("f".into());
    let modifiers = ui_wgpu::wgpu::PointerModifiers { ctrl: true, meta: true, shift: false, alt: false };
    assert!(crate::shell::key_event_matches_chord(&action, &modifiers, &fixture.collision.matches));
    assert!(!crate::shell::key_event_matches_chord(&action, &modifiers, &fixture.collision.refuses), "control+meta+f must not be reserved by Find's mod+f row");
    let mut interaction = keyboard_interaction();
    semio_framework_async::block_on(dispatch_normalized_event(&mut interaction, DispatchEvent::KeyDown { key: "f".into(), modifiers: EventModifiers { shift: false, ctrl: true, alt: false, meta: true } }));
    assert!(interaction.shell.fullscreen_toggle_requested, "the real host ingress reaches apply_os_command");
    assert!(interaction.shell.deferred_actions.is_empty(), "an OS command never enters the guest action lane");

    crate::set_host_platform_uses_meta(cfg!(target_os = "macos"));
    crate::set_host_platform_kind(if cfg!(target_os = "macos") {
        semio_framework::manifest::Platform::MacOs
    } else if cfg!(target_os = "windows") {
        semio_framework::manifest::Platform::Windows
    } else {
        semio_framework::manifest::Platform::Linux
    });
    crate::apply_boot_descriptor(restore).expect("restore boot descriptor");
}

#[test]
fn normalized_pointer_and_wheel_snapshots_replace_stale_keyboard_modifiers() {
    let mut interaction = keyboard_interaction();
    interaction.modifiers = ui_wgpu::wgpu::PointerModifiers { shift: true, ctrl: true, alt: true, meta: true };
    interaction.input.modifiers = interaction.modifiers.clone();
    let pointer = PointerInfo { id: ui_render::PointerId(1), kind: ui_render::PointerKind::Mouse, pressure: None, tilt: None };
    let ctrl = EventModifiers { shift: false, ctrl: true, alt: false, meta: false };
    semio_framework_async::block_on(dispatch_normalized_event(&mut interaction, DispatchEvent::PointerMove { pointer, x: -10.0, y: -10.0, modifiers: ctrl }));
    assert!(interaction.modifiers.ctrl && !interaction.modifiers.shift && !interaction.modifiers.alt && !interaction.modifiers.meta);
    assert!(interaction.input.modifiers.ctrl && !interaction.input.modifiers.shift, "retained dispatch state uses the pointer event snapshot");

    let shift_meta = EventModifiers { shift: true, ctrl: false, alt: false, meta: true };
    semio_framework_async::block_on(dispatch_normalized_event(&mut interaction, DispatchEvent::PointerDown { pointer, x: -10.0, y: -10.0, button: PointerButton::Primary, modifiers: shift_meta }));
    assert!(interaction.modifiers.shift && interaction.modifiers.meta && !interaction.modifiers.ctrl);
    let alt = EventModifiers { shift: false, ctrl: false, alt: true, meta: false };
    semio_framework_async::block_on(dispatch_normalized_event(&mut interaction, DispatchEvent::PointerUp { pointer, x: -10.0, y: -10.0, button: PointerButton::Primary, modifiers: alt }));
    assert!(interaction.modifiers.alt && !interaction.modifiers.shift && !interaction.modifiers.meta);

    let none = EventModifiers::default();
    semio_framework_async::block_on(dispatch_normalized_event(&mut interaction, DispatchEvent::Scroll { x: -10.0, y: -10.0, delta_x: 0.0, delta_y: 1.0, modifiers: none }));
    assert!(!interaction.modifiers.shift && !interaction.modifiers.ctrl && !interaction.modifiers.alt && !interaction.modifiers.meta, "a pointer snapshot clears stale positive keyboard state");
    assert!(!interaction.input.modifiers.shift && !interaction.input.modifiers.ctrl && !interaction.input.modifiers.alt && !interaction.input.modifiers.meta);
}

#[test]
fn component_close_handoff_keeps_unrelated_window_ingress_routable() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️component-close-frame-turn/🔣️.json")).expect("component-close frame-turn fixture");
    assert_ne!(law["closeHost"], law["liveHost"]);
    assert_eq!(law["maxCloseUnitsPerTurn"], 1);
    let mut events = ui_host::EventQueue::new();
    let mut scheduler = ui_render::FrameScheduler::new();
    let token = ui_host::UiThreadToken::mint_for_host();
    let mut generation = 0;
    let pointer = PointerInfo { id: ui_render::PointerId(77), kind: ui_render::PointerKind::Mouse, pressure: None, tilt: None };
    let mut delivered = Vec::new();
    for turn in law["turns"].as_array().expect("bounded close turns") {
        assert!(matches!(turn["closeOutcome"].as_str(), Some("frameRetirement" | "externalWait" | "terminal")));
        assert_eq!(
            enqueue_host_event(
                &mut events,
                &mut scheduler,
                token,
                &mut generation,
                FrameGenerationHold::Free,
                DispatchEvent::PointerDown {
                    pointer,
                    x: turn["inputSequence"].as_u64().expect("input sequence") as f32,
                    y: 1.0,
                    button: PointerButton::Primary,
                    modifiers: EventModifiers::default(),
                },
            ),
            ui_host::EnqueueOutcome::Accepted
        );
        let input_generation = events.current_generation();
        let page = events.drain_page(ui_host::WorkerContext::new(input_generation));
        assert_eq!(page.discrete.iter().filter(|event| event.is_some()).count(), 1, "the live window keeps its exact discrete event while the unrelated close waits");
        delivered.push(turn["inputSequence"].as_u64().expect("input sequence"));
    }
    assert_eq!(delivered, serde_json::from_value::<Vec<u64>>(law["expected"]["liveInputSequences"].clone()).expect("expected live inputs"));
    assert!(events.is_empty());
}
