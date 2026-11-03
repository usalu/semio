use super::*;
use semio_framework_ui_contract::{accessibility_role, Component, ContainerRole};

struct Stub;

impl PluginShellHooks for Stub {
    fn choose_layout(&self, request: &ShellLayoutRequest) -> Option<ShellLayoutChoice> {
        let mobile = request.signal.width_px < 600;
        Some(ShellLayoutChoice {
            layout_id: if mobile { "stub-mobile" } else { "stub-desktop" }.to_owned(),
            layout: serde_json::from_str("{\"root\":{\"kind\":\"stack\",\"children\":[]}}").unwrap(),
            sheet: (mobile && request.has_selection).then(|| ShellBottomSheet { label: "Properties".to_owned(), state: ShellSheetState::Half, height: 0.5, min_handle_px: 44 }),
        })
    }

    fn prefs_key(&self) -> Option<String> {
        Some("stub.layout".to_owned())
    }

    fn handle_key(&self, event: &ShellKeyEvent) -> ShellKeyOutcome {
        match (event.scope.as_str(), event.key.as_str()) {
            ("viewport", "ArrowLeft") => ShellKeyOutcome::Consumed,
            (_, "z") if event.ctrl => ShellKeyOutcome::Action { action: "undo".to_owned() },
            _ => ShellKeyOutcome::Ignored,
        }
    }

    fn accessible_nodes(&self, _scope: &str, lang: &str) -> Vec<ShellAccessibleNode> {
        vec![ShellAccessibleNode { key: "beam-1".to_owned(), role: "option".to_owned(), name: if lang == "de" { "Stab 1" } else { "Beam 1" }.to_owned(), position: Some(1), set_size: Some(2), selected: Some(true), ..ShellAccessibleNode::default() }]
    }
}

fn driver() -> ShellHookDriver {
    let mut driver = ShellHookDriver::default();
    driver.registry.install("stub", Arc::new(Stub));
    driver
}

#[test]
fn the_tracker_names_start_resize_input_modality_and_selection_once() {
    let mut tracker = ShellHookTracker::default();
    let wide = ShellDeviceSignal { width_px: 1200, touch: false };
    assert_eq!(tracker.observe(wide, false), Some(ShellHookCause::Start));
    assert_eq!(tracker.observe(wide, false), None);
    assert_eq!(tracker.observe(ShellDeviceSignal { width_px: 390, touch: false }, false), Some(ShellHookCause::Resize));
    assert_eq!(tracker.observe(ShellDeviceSignal { width_px: 390, touch: true }, false), Some(ShellHookCause::InputModality));
    assert_eq!(tracker.observe(ShellDeviceSignal { width_px: 390, touch: true }, true), Some(ShellHookCause::Selection));
    assert_eq!(tracker.observe(ShellDeviceSignal { width_px: 1200, touch: false }, true), Some(ShellHookCause::Resize));
    tracker.reset();
    assert_eq!(tracker.observe(wide, true), Some(ShellHookCause::Start));
}

#[test]
fn the_driver_asks_the_plugin_only_when_something_changed_and_keeps_the_sheet() {
    let mut driver = driver();
    let desktop = driver.observe("stub", "app", 1200, None, "en").expect("start");
    assert_eq!(desktop.layout_id, "stub-desktop");
    assert!(driver.sheet.is_none());
    assert!(driver.observe("stub", "app", 1200, None, "en").is_none());
    driver.note_touch(true);
    driver.note_selection(true);
    let mobile = driver.observe("stub", "app", 390, Some("{}"), "de").expect("resize");
    assert_eq!(mobile.layout_id, "stub-mobile");
    assert_eq!(driver.sheet.as_ref().map(|sheet| sheet.state), Some(ShellSheetState::Half));
    assert!(driver.observe("unknown", "app", 700, None, "en").is_none());
    assert!(driver.sheet.is_none());
    assert!(driver.observe("stub", "other-app", 700, None, "en").is_some(), "another app asks again");
}

#[test]
fn the_plugin_sees_the_key_before_the_shell_and_unknown_plugins_ignore_it() {
    let driver = driver();
    let left = ShellKeyEvent { scope: "viewport".to_owned(), key: "ArrowLeft".to_owned(), ..ShellKeyEvent::default() };
    assert_eq!(driver.handle_key("stub", &left), ShellKeyOutcome::Consumed);
    let undo = ShellKeyEvent { scope: "global".to_owned(), key: "z".to_owned(), ctrl: true, ..ShellKeyEvent::default() };
    assert_eq!(driver.handle_key("stub", &undo), ShellKeyOutcome::Action { action: "undo".to_owned() });
    assert_eq!(driver.handle_key("stub", &ShellKeyEvent { key: "q".to_owned(), ..ShellKeyEvent::default() }), ShellKeyOutcome::Ignored);
    assert_eq!(driver.handle_key("other", &left), ShellKeyOutcome::Ignored);
}

#[test]
fn plugin_nodes_become_projection_nodes_with_name_role_and_position() {
    let driver = driver();
    let nodes = driver.registry.accessible_nodes("stub", "viewport", "de");
    let projected = accessible_projection(&nodes, 100);
    assert_eq!(projected.len(), 1);
    assert_eq!((projected[0].node_id, projected[0].role.as_str(), projected[0].label.as_deref()), (100, "option", Some("Stab 1")));
    assert_eq!((projected[0].pos_in_set, projected[0].set_size, projected[0].selected), (Some(1), Some(2), Some(true)));
    assert!(projected[0].focusable);
    assert!(driver.registry.accessible_nodes("other", "viewport", "en").is_empty());
}

#[test]
fn the_bottom_sheet_is_a_complementary_container_node() {
    let sheet = ShellBottomSheet { label: "Properties".to_owned(), state: ShellSheetState::Half, height: 0.5, min_handle_px: 44 };
    let node = bottom_sheet_node(&sheet, "sheet", Vec::new()).expect("sheet node");
    assert!(matches!(&node.component, Component::Container(props) if props.role == ContainerRole::Sheet));
    assert_eq!(accessibility_role(&node.component, false), "complementary");
}

#[test]
fn the_wire_spelling_is_camel_case() {
    let request = ShellLayoutRequest { app_id: "app".to_owned(), signal: ShellDeviceSignal { width_px: 390, touch: true }, cause: ShellHookCause::InputModality, prefs: None, has_selection: true, lang: "en".to_owned() };
    let text = serde_json::to_string(&request).unwrap();
    assert!(text.contains("\"appId\":\"app\"") && text.contains("\"widthPx\":390") && text.contains("\"hasSelection\":true") && text.contains("\"inputModality\""), "{text}");
    assert_eq!(serde_json::to_string(&ShellKeyOutcome::Action { action: "undo".to_owned() }).unwrap(), "{\"type\":\"action\",\"action\":\"undo\"}");
    let select = ShellKeyOutcome::select("norm.en1992", vec![ShellSelectTarget { granularity: "member".to_owned(), id: "b1".to_owned() }]);
    assert_eq!(serde_json::to_string(&select).unwrap(), "{\"type\":\"select\",\"domainId\":\"norm.en1992\",\"targets\":[{\"granularity\":\"member\",\"id\":\"b1\"}],\"merge\":\"replace\"}");
    assert_eq!(serde_json::from_str::<ShellKeyOutcome>(&serde_json::to_string(&select).unwrap()).unwrap(), select);
    let verb = ShellKeyOutcome::verb("selectAll", [("granularities".to_owned(), "member,support".to_owned())]);
    assert_eq!(serde_json::to_string(&verb).unwrap(), "{\"type\":\"verb\",\"action\":\"selectAll\",\"args\":{\"granularities\":\"member,support\"}}");
    assert_eq!(serde_json::from_str::<ShellKeyOutcome>(&serde_json::to_string(&verb).unwrap()).unwrap(), verb);
}

#[test]
fn the_stored_layout_preferences_are_read_once_per_app_and_ride_on_every_request() {
    let mut driver = driver();
    assert_eq!(driver.prefs_to_load("stub", "app").as_deref(), Some("stub.layout"));
    assert_eq!(driver.prefs_to_load("stub", "app"), None);
    driver.supply_prefs(Some("{\"dock\":{}}".to_owned()));
    assert!(driver.observe("stub", "app", 1200, None, "en").is_some());
    assert_eq!(driver.prefs_text.as_deref(), Some("{\"dock\":{}}"));
    assert_eq!(driver.prefs_to_load("stub", "other").as_deref(), Some("stub.layout"));
    assert_eq!(driver.prefs_text, None);
    assert_eq!(driver.prefs_to_load("nobody", "third"), None);
}

struct Remembers(std::sync::Mutex<Vec<ShellPlatformPrefs>>);

impl PluginShellHooks for Remembers {
    fn platform_prefs_changed(&self, prefs: &ShellPlatformPrefs) {
        self.0.lock().unwrap().push(*prefs);
    }
}

#[test]
fn the_platform_preferences_reach_the_plugin_once_per_change() {
    let remembered = Arc::new(Remembers(std::sync::Mutex::new(Vec::new())));
    let mut driver = ShellHookDriver::default();
    driver.registry.install("stub", remembered.clone());
    let calm = ShellPlatformPrefs::default();
    let contrast = ShellPlatformPrefs { forced_colors: true, ..calm };
    assert!(driver.note_platform("stub", calm), "the first report always counts");
    assert!(!driver.note_platform("stub", calm), "an unchanged report is silent");
    assert!(driver.note_platform("stub", contrast));
    assert!(driver.note_platform("stub", ShellPlatformPrefs { prefers_reduced_motion: true, ..contrast }));
    assert_eq!(remembered.0.lock().unwrap().len(), 3);
    assert_eq!(remembered.0.lock().unwrap()[1], contrast);
    assert!(driver.note_platform("nobody", ShellPlatformPrefs { prefers_dark: true, ..calm }), "a change is recorded even for a plugin without hooks");
    assert_eq!(remembered.0.lock().unwrap().len(), 3, "the other plugin heard nothing");
}

#[test]
fn the_platform_preferences_wire_spelling_is_camel_case() {
    let text = serde_json::to_string(&ShellPlatformPrefs { forced_colors: true, prefers_contrast_more: false, prefers_dark: true, prefers_reduced_motion: true }).unwrap();
    assert_eq!(text, "{\"forcedColors\":true,\"prefersContrastMore\":false,\"prefersDark\":true,\"prefersReducedMotion\":true}");
}

struct Touchy;

impl PluginShellHooks for Touchy {
    fn touch_profile(&self, signal: &ShellDeviceSignal) -> ShellTouchProfile {
        ShellTouchProfile { one_finger_orbit: signal.width_px < 600, long_press: true, hit_px: if signal.touch { 44 } else { 0 } }
    }

    fn handle_touch(&self, event: &ShellTouchEvent) -> ShellKeyOutcome {
        match event.gesture {
            ShellTouchGesture::LongPress => ShellKeyOutcome::Action { action: format!("menu@{}", event.scope) },
            ShellTouchGesture::Tap => ShellKeyOutcome::Action { action: format!("close@{}", event.scope) },
            ShellTouchGesture::DoubleTap => ShellKeyOutcome::Consumed,
        }
    }
}

#[test]
fn the_touch_lane_asks_the_plugin_per_device_and_offers_it_each_recognised_gesture() {
    let mut driver = ShellHookDriver::default();
    assert_eq!(driver.touch_profile("nobody"), ShellTouchProfile { one_finger_orbit: false, long_press: true, hit_px: 0 }, "a plugin without hooks gets the neutral default");
    assert_eq!(driver.handle_touch("nobody", "viewport", ShellTouchGesture::LongPress, 1.0, 2.0), ShellKeyOutcome::Ignored);
    driver.registry.install("stub", Arc::new(Touchy));
    driver.observe("stub", "app", 390, None, "en");
    assert_eq!(driver.touch_profile("stub"), ShellTouchProfile { one_finger_orbit: true, long_press: true, hit_px: 44 });
    driver.observe("stub", "app", 1200, None, "en");
    assert!(!driver.touch_profile("stub").one_finger_orbit, "the profile follows the last observed width");
    assert_eq!(driver.handle_touch("stub", "viewport", ShellTouchGesture::LongPress, 10.0, 20.0), ShellKeyOutcome::Action { action: "menu@viewport".to_owned() });
    assert_eq!(driver.handle_touch("stub", "viewport", ShellTouchGesture::Tap, 10.0, 20.0), ShellKeyOutcome::Action { action: "close@viewport".to_owned() }, "a tap on the surface is offered too, so a plugin can dismiss what it opened");
    assert_eq!(driver.handle_touch("stub", "viewport", ShellTouchGesture::DoubleTap, 10.0, 20.0), ShellKeyOutcome::Consumed);
}

#[test]
fn the_touch_wire_spelling_is_camel_case() {
    let text = serde_json::to_string(&ShellTouchEvent { scope: "viewport".to_owned(), gesture: ShellTouchGesture::DoubleTap, x: 1.5, y: 2.0, width_px: 390 }).unwrap();
    assert_eq!(text, "{\"scope\":\"viewport\",\"gesture\":\"doubleTap\",\"x\":1.5,\"y\":2.0,\"widthPx\":390}");
    assert_eq!(serde_json::to_string(&ShellTouchGesture::Tap).unwrap(), "\"tap\"");
    assert_eq!(serde_json::to_string(&ShellTouchProfile::default()).unwrap(), "{\"oneFingerOrbit\":false,\"longPress\":true,\"hitPx\":0}");
}

const PLATFORM_PREFS_FIXTURE: &str = include_str!("../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎛️platform-prefs/🔣️.json");

fn prefs_of(value: &serde_json::Value) -> ShellPlatformPrefs {
    serde_json::from_value(value.clone()).unwrap()
}

#[test]
fn the_platform_preference_reports_answer_the_language_agnostic_fixture_rows() {
    let law: serde_json::Value = serde_json::from_str(PLATFORM_PREFS_FIXTURE).unwrap();
    let remembered = Arc::new(Remembers(std::sync::Mutex::new(Vec::new())));
    let mut driver = ShellHookDriver::default();
    driver.registry.install("stub", remembered.clone());
    let mut told = 0;
    for row in law["reports"].as_array().unwrap() {
        let prefs = prefs_of(&row["prefs"]);
        assert_eq!(driver.note_platform("stub", prefs), row["changed"].as_bool().unwrap(), "{}", row["id"]);
        told += usize::from(row["changed"].as_bool().unwrap());
        assert_eq!(remembered.0.lock().unwrap().len(), told, "{}: the plugin hears exactly the changes", row["id"]);
    }
    assert_eq!(remembered.0.lock().unwrap().last().copied(), Some(prefs_of(&law["reports"].as_array().unwrap().last().unwrap()["prefs"])));
    assert_eq!(prefs_of(&law["calm"]), ShellPlatformPrefs::default());
}

#[test]
fn the_platform_preferences_wire_spelling_is_the_one_of_the_language_agnostic_fixture() {
    let law: serde_json::Value = serde_json::from_str(PLATFORM_PREFS_FIXTURE).unwrap();
    let prefs = ShellPlatformPrefs { forced_colors: true, prefers_contrast_more: false, prefers_dark: true, prefers_reduced_motion: true };
    assert_eq!(serde_json::to_string(&prefs).unwrap(), law["wire"].as_str().unwrap());
}
