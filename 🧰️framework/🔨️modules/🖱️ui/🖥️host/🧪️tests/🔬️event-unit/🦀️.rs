
use super::*;

//#region 🎡️Wheel tests

#[test]
fn browser_schema_and_canonical_trace_decode_without_dependencies() {
    assert!(BROWSER_HOST_CONTRACT_JSON.contains("\"semanticUnitsPerGrant\": 1"));
    assert!(BROWSER_HOST_CONTRACT_JSON.contains("\"encodedEventBytes\": 1051"));
    assert!(BROWSER_HOST_LIMITS_FIXTURE.contains("event-bytes\t1024\t1025"));
    assert!(BROWSER_HOST_LIMITS_FIXTURE.contains("encoded-event-bytes\t1051\t1052\tlimit-exceeded-before-copy"));
    assert_eq!(BROWSER_HOST_FRAMING_FIXTURE.lines().skip(1).count(), 8);
    assert!(BROWSER_HOST_FRAMING_FIXTURE.contains("event-exact\t1024\t27\t1051\t1024\t1051\tretained-exact-retry"));
    let canvas = CanvasId::try_new(1).unwrap();
    let listener = ListenerId::try_new(1, 1).unwrap();
    let mut decoded = 0;
    for row in BROWSER_HOST_TRACE_FIXTURE.lines().skip(1) {
        let columns: Vec<_> = row.split('\t').collect();
        assert_eq!(columns.len(), 5);
        let code: u16 = columns[2].parse().unwrap();
        let payload = decode_fixture_hex(columns[3]);
        let event = AbiEvent {
            request_id: crate::abi::AbiRequestId(decoded + 1),
            generation: 1,
            sequence: decoded as u32,
            event: crate::abi::AbiEventCode::try_new(code).unwrap(),
            status: crate::abi::AbiStatus::OK,
            bytes: crate::abi::AbiBytes::try_new(payload).unwrap(),
        };
        decode_browser_host_event(&event, canvas, listener).unwrap();
        decoded += 1;
    }
    assert_eq!(decoded, 10);
}

fn decode_fixture_hex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).unwrap();
            u8::from_str_radix(text, 16).unwrap()
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_line_delta_scales_by_line_height() {
    let (dx, dy) = normalize_wheel_delta_native(winit::event::MouseScrollDelta::LineDelta(1.0, -2.0));
    assert_eq!(dx, WHEEL_LINE_HEIGHT_PX);
    assert_eq!(dy, -2.0 * WHEEL_LINE_HEIGHT_PX);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_pixel_delta_passes_through() {
    let (dx, dy) = normalize_wheel_delta_native(winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition::new(3.5, -7.25)));
    assert_eq!(dx, 3.5);
    assert_eq!(dy, -7.25);
}

#[test]
fn web_pixel_mode_passes_through() {
    assert_eq!(normalize_wheel_delta_web(10.0, 20.0, DOM_DELTA_PIXEL, (800.0, 600.0)), (10.0, 20.0));
}

#[test]
fn web_line_mode_scales_by_line_height() {
    assert_eq!(normalize_wheel_delta_web(1.0, 2.0, DOM_DELTA_LINE, (800.0, 600.0)), (WHEEL_LINE_HEIGHT_PX, 2.0 * WHEEL_LINE_HEIGHT_PX));
}

#[test]
fn web_page_mode_scales_by_viewport() {
    assert_eq!(normalize_wheel_delta_web(1.0, 1.0, DOM_DELTA_PAGE, (800.0, 600.0)), (800.0, 600.0));
}

//#endregion 🎡️Wheel tests

//#region ⌨️Modifier tests

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_modifiers_map_every_flag() {
    let state = winit::keyboard::ModifiersState::SHIFT | winit::keyboard::ModifiersState::ALT;
    let modifiers = modifiers_from_winit(state);
    assert!(modifiers.shift);
    assert!(!modifiers.ctrl);
    assert!(modifiers.alt);
    assert!(!modifiers.meta);
}

#[test]
fn web_modifiers_are_a_plain_copy() {
    let modifiers = modifiers_from_web(false, true, false, true);
    assert!(!modifiers.shift);
    assert!(modifiers.ctrl);
    assert!(!modifiers.alt);
    assert!(modifiers.meta);
}

//#endregion ⌨️Modifier tests

//#region ⌨️Key tests

/// 🇫🇷️ On an AZERTY layout the physical `KeyQ` position produces the logical character "a" — the
/// whole point of the physical/logical split (see `KeyCode::KeyW`'s own docstring example).
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn physical_vs_logical_key_mapping_stays_distinct_across_layouts() {
    let physical = physical_key_from_winit(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ));
    let logical = logical_key_to_dispatch_string(&winit::keyboard::Key::Character("a".into()));
    assert_eq!(physical, PhysicalKeyCode::KeyQ);
    assert_eq!(logical, "a");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn named_keys_map_to_their_dom_key_string() {
    assert_eq!(logical_key_to_dispatch_string(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::Enter)), "Enter");
    assert_eq!(logical_key_to_dispatch_string(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space)), " ");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn unidentified_physical_key_degrades_cleanly() {
    assert_eq!(physical_key_from_winit(winit::keyboard::PhysicalKey::Unidentified(winit::keyboard::NativeKeyCode::Unidentified)), PhysicalKeyCode::Unidentified);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_and_web_agree_on_the_same_physical_key() {
    assert_eq!(physical_key_from_winit(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft)), physical_key_from_web_code("ArrowLeft"));
}

//#endregion ⌨️Key tests

//#region 🈶️IME tests

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ime_preedit_with_no_cursor_range_falls_back_to_text_end() {
    let event = ime_event_from_winit(winit::event::Ime::Preedit("ab".into(), None));
    assert_eq!(event, ImeEvent::Update { text: "ab".into(), cursor: 2 });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ime_disabled_maps_to_cancel() {
    assert_eq!(ime_event_from_winit(winit::event::Ime::Disabled), ImeEvent::Cancel);
}

//#endregion 🈶️IME tests

//#region 🆔️Multi-pointer tests

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn two_simultaneous_touches_on_the_same_device_stay_distinct() {
    let mut registry = PointerRegistry::new();
    let touch_a = winit::event::Touch { device_id: winit::event::DeviceId::dummy(), phase: winit::event::TouchPhase::Started, location: winit::dpi::PhysicalPosition::new(10.0, 10.0), force: None, id: 1 };
    let touch_b = winit::event::Touch { device_id: winit::event::DeviceId::dummy(), phase: winit::event::TouchPhase::Started, location: winit::dpi::PhysicalPosition::new(20.0, 20.0), force: None, id: 2 };
    let a = pointer_info_for_touch(&mut registry, &touch_a);
    let b = pointer_info_for_touch(&mut registry, &touch_b);
    assert_ne!(a.id, b.id, "two simultaneous pointers must normalize to distinct ids");
    assert_eq!(a.kind, PointerKind::Touch);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_mouse_and_a_touch_on_the_same_device_stay_distinct() {
    let mut registry = PointerRegistry::new();
    let mouse = pointer_info_for_mouse(&mut registry, winit::event::DeviceId::dummy());
    let touch = winit::event::Touch { device_id: winit::event::DeviceId::dummy(), phase: winit::event::TouchPhase::Started, location: winit::dpi::PhysicalPosition::new(0.0, 0.0), force: None, id: 0 };
    let touch_info = pointer_info_for_touch(&mut registry, &touch);
    assert_ne!(mouse.id, touch_info.id);
    assert_eq!(mouse.kind, PointerKind::Mouse);
}

#[test]
fn web_pointer_ids_are_used_verbatim_and_stay_distinct() {
    let a = pointer_info_from_web(1, "touch", 0.5, 0.0, 0.0);
    let b = pointer_info_from_web(2, "touch", 0.5, 0.0, 0.0);
    assert_ne!(a.id, b.id);
    assert_eq!(a.id, pointer_id_from_web(1));
}

//#endregion 🆔️Multi-pointer tests

//#region 🖱️Button tests

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_and_web_button_mappings_agree_where_both_define_a_button() {
    assert_eq!(pointer_button_from_winit(winit::event::MouseButton::Left), Some(PointerButton::Primary));
    assert_eq!(pointer_button_from_web(0), Some(PointerButton::Primary));
    assert_eq!(pointer_button_from_winit(winit::event::MouseButton::Right), Some(PointerButton::Secondary));
    assert_eq!(pointer_button_from_web(2), Some(PointerButton::Secondary));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn buttons_outside_the_closed_set_map_to_none() {
    assert_eq!(pointer_button_from_winit(winit::event::MouseButton::Back), None);
    assert_eq!(pointer_button_from_web(4), None);
}

//#endregion 🖱️Button tests
