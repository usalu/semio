/// 🎯️ Focus window of kind: the one `focusWindow` verb opens a closed window, raises an open one, moves the keyboard focus into it and announces it in the interface language; a kind the app never declared is refused and announced.

fn focus_window_shell() -> semio_renderer_wgpu_shell_state::ShellState {
    let law: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️focus-window-of-kind/🔣️.json")).unwrap();
    let mut shell = host_test_shell();
    let prototype = shell.docs.session.as_ref().unwrap().app.window_kinds.first().clone();
    let kinds = law["windowKinds"].as_array().unwrap().iter().map(|row| {
        let mut kind = prototype.clone();
        kind.id = row["id"].as_str().unwrap().into();
        kind.label = LocalizedLabel::native(row["label"]["en"].as_str().unwrap(), row["label"]["de"].as_str().unwrap());
        kind
    });
    shell.docs.session.as_mut().unwrap().app.window_kinds = WindowKinds::try_from(kinds.collect::<Vec<_>>()).unwrap();
    let open = law["initiallyOpen"][0].as_str().unwrap().to_string();
    let initial = WindowLayout { root: semio_renderer_wgpu_dock::dock_node_to_layout_root(&semio_renderer_wgpu_dock::DockNode::Stack { windows: vec![semio_renderer_wgpu_dock::DockStackTab::new(open.clone())], active: open.clone() }) };
    shell.docs.session.as_mut().unwrap().app.default_layout = Some(initial.clone());
    shell.dock.layout_override = Some(initial);
    shell.windows.active_window_id = Some(open);
    SessionCtx::of(&mut shell).sync_dock();
    shell
}

fn focus_window_action(kind: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: "framework".into(), action: semio_framework::FOCUS_WINDOW_ACTION_ID.into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "windowKindId": kind }) }
}

fn mounted(shell: &semio_renderer_wgpu_shell_state::ShellState) -> Vec<String> {
    shell.dock.dock.window_instances().into_iter().map(|(id, _)| id).collect()
}

#[test]
fn focusing_a_closed_window_kind_opens_it_and_an_open_one_is_only_raised() {
    use semio_renderer_wgpu_shell_dock::WindowKindFocus;
    let mut shell = focus_window_shell();
    assert_eq!(mounted(&shell), ["a"]);
    let opened = shell.dock.focus_window_of_kind(&shell.docs, "b", &mut shell.windows, &mut shell.settle);
    assert_eq!(opened, Ok(WindowKindFocus::Opened { window_id: "b".into() }));
    assert_eq!(mounted(&shell).len(), 2);
    assert_eq!(shell.windows.active_window_id.as_deref(), Some("b"));
    assert_eq!(shell.dock.dock.active_window_id.as_deref(), Some("b"));
    assert!(shell.dock.layout_override.is_some(), "an opened window is remembered in the layout");
    shell.windows.active_window_id = Some("a".into());
    shell.dock.dock.sync_active_window("a");
    let raised = shell.dock.focus_window_of_kind(&shell.docs, "b", &mut shell.windows, &mut shell.settle);
    assert_eq!(raised, Ok(WindowKindFocus::Raised { window_id: "b".into() }));
    assert_eq!(mounted(&shell).len(), 2, "raising never opens a second instance");
    assert_eq!(shell.windows.active_window_id.as_deref(), Some("b"));
    assert_eq!(shell.dock.dock.active_window_id.as_deref(), Some("b"));
    let refused = shell.dock.focus_window_of_kind(&shell.docs, "undeclared", &mut shell.windows, &mut shell.settle);
    assert!(refused.is_err_and(|reason| reason.contains("not declared")));
    assert_eq!(shell.windows.active_window_id.as_deref(), Some("b"), "a refusal leaves the focus where it was");
}

#[test]
fn the_focus_window_action_moves_the_keyboard_ring_into_the_window_and_announces_it_in_both_languages() {
    let law: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️focus-window-of-kind/🔣️.json")).unwrap();
    assert_eq!(law["action"], semio_framework::FOCUS_WINDOW_ACTION_ID);
    assert_eq!(law["argument"], semio_framework::FOCUS_WINDOW_ARG_KIND);
    let dispatch = |shell: &mut semio_renderer_wgpu_shell_state::ShellState, kind: &str| semio_framework_async::block_on(semio_renderer_wgpu_shell_actions::ActionsCtx::of(shell).dispatch_action(focus_window_action(kind)));
    let expected = |locale: &str, outcome: &str| law["announcements"].as_array().unwrap().iter().find(|row| row["locale"] == locale && row["outcome"] == outcome).unwrap().clone();
    for locale in ["en", "de"] {
        let mut shell = focus_window_shell();
        shell.prefs.locale_id = locale.into();
        shell.input.accessibility_focused_control_id = Some("shell.search.toggle".into());
        dispatch(&mut shell, "b").expect("a declared kind is focused");
        assert_eq!(shell.input.accessibility_focused_control_id, None, "{locale}: no chrome control keeps the keyboard focus");
        assert_eq!(shell.windows.active_window_id.as_deref(), Some("b"));
        let row = expected(locale, "opened");
        let notice = shell.chrome.transient_notice().expect("the focus is announced");
        assert_eq!((notice.message.as_str(), notice.code.as_deref()), (row["message"].as_str().unwrap(), row["code"].as_str()), "{locale}");
        assert!(matches!(notice.severity, semio_framework::Severity::Info), "{locale}: a focus is a polite status");
        shell.windows.active_window_id = Some("a".into());
        dispatch(&mut shell, "b").expect("an open window is raised");
        assert_eq!(shell.chrome.transient_notice().map(|notice| notice.message.as_str()), expected(locale, "raised")["message"].as_str(), "{locale}");
        let mut closed = focus_window_shell();
        closed.prefs.locale_id = locale.into();
        closed.docs.session.as_mut().unwrap().app.window_kinds = WindowKinds::try_from(vec![closed.docs.session.as_ref().unwrap().app.window_kinds.first().clone()]).unwrap();
        assert!(dispatch(&mut closed, "b").is_err(), "{locale}: an undeclared kind is refused");
        let row = expected(locale, "refused");
        let notice = closed.chrome.transient_notice().expect("the refusal is announced");
        assert_eq!((notice.message.as_str(), notice.code.as_deref()), (row["message"].as_str().unwrap(), row["code"].as_str()), "{locale}");
        assert!(matches!(notice.severity, semio_framework::Severity::Warning), "{locale}");
    }
    for key in ["window.focus.raised", "window.focus.opened", "window.focus.refused"] {
        for german in [false, true] {
            assert!(shell_chrome_string(key, german).contains("{name}"), "{key} is a template in both languages");
        }
    }
}

fn install_device_layouts(shell: &mut semio_renderer_wgpu_shell_state::ShellState, law: &Value, with_device_layouts: bool) {
    let layouts = law["deviceLayouts"].as_array().unwrap().iter().filter(|_| with_device_layouts).map(|row| {
        let kinds: Vec<String> = row["kinds"].as_array().unwrap().iter().map(|kind| kind.as_str().unwrap().to_string()).collect();
        let id = row["id"].as_str().unwrap();
        semio_ui_declaration::layout::create_named_layout(id, id, semio_ui_declaration::layout::create_stack_layout(&kinds, None), "builtin", None, None)
    });
    shell.docs.session.as_mut().unwrap().app.named_layouts = layouts.collect();
}

fn present_as(shell: &mut semio_renderer_wgpu_shell_state::ShellState, device: &str) {
    shell.chrome.screen_w = if device == "mobile" { 375.0 } else { 1280.0 };
    shell.chrome.chrome_build.preferences.ui_layout = if device == "tablet" { "tablet".to_string() } else { "desktop".to_string() };
}

#[test]
fn a_window_kind_missing_from_the_device_layout_maps_to_its_stand_in_or_is_refused_like_the_language_agnostic_rows() {
    use semio_renderer_wgpu_shell_dock::{FocusDevice, resolve_device_window_kind};
    let law: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️focus-window-of-kind/🔣️.json")).unwrap();
    for row in law["deviceRows"].as_array().unwrap() {
        let id = row["id"].as_str().unwrap();
        let device = FocusDevice::parse(row["device"].as_str().unwrap()).unwrap();
        let mut shell = focus_window_shell();
        install_device_layouts(&mut shell, &law, row["withoutDeviceLayouts"] != true);
        let requested = row["requested"].as_str().unwrap();
        let fallback = row["fallback"].as_str();
        let resolved = shell.dock.resolve_window_kind_for_device(&shell.docs, requested, fallback, device);
        assert_eq!(resolved.as_deref(), row["target"].as_str(), "{id}");
        let layouts = &shell.docs.session.as_ref().unwrap().app.named_layouts;
        let available = semio_renderer_wgpu_shell_dock::device_layout_window_kinds(layouts, device);
        assert_eq!(resolve_device_window_kind(available.as_deref(), requested, fallback).as_deref(), row["target"].as_str(), "{id}: the pure resolver agrees");
    }
}

#[test]
fn the_focus_window_action_lands_on_the_device_stand_in_and_announces_an_unavailable_kind_in_both_languages() {
    let law: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️focus-window-of-kind/🔣️.json")).unwrap();
    let dispatch = |shell: &mut semio_renderer_wgpu_shell_state::ShellState, kind: &str, fallback: Option<&str>| {
        let args = match fallback {
            Some(stand_in) => semio_renderer_wgpu_host_base::action_args_json!({ "windowKindId": kind, "fallbackWindowKindId": stand_in }),
            None => semio_renderer_wgpu_host_base::action_args_json!({ "windowKindId": kind }),
        };
        let action = ActionDescriptor { controller_id: "framework".into(), action: semio_framework::FOCUS_WINDOW_ACTION_ID.into(), args };
        semio_framework_async::block_on(semio_renderer_wgpu_shell_actions::ActionsCtx::of(shell).dispatch_action(action))
    };
    assert_eq!(law["fallbackArgument"], semio_framework::FOCUS_WINDOW_ARG_FALLBACK);
    for row in law["unavailable"].as_array().unwrap() {
        let (locale, device) = (row["locale"].as_str().unwrap(), row["device"].as_str().unwrap());
        let mut shell = focus_window_shell();
        install_device_layouts(&mut shell, &law, true);
        present_as(&mut shell, device);
        shell.prefs.locale_id = locale.into();
        assert!(dispatch(&mut shell, row["kind"].as_str().unwrap(), None).is_err(), "{locale} {device}: no stand-in, no window");
        let notice = shell.chrome.transient_notice().expect("the refusal is announced");
        assert_eq!((notice.message.as_str(), notice.code.as_deref()), (row["message"].as_str().unwrap(), row["code"].as_str()), "{locale} {device}");
        assert!(matches!(notice.severity, semio_framework::Severity::Warning));
        assert_eq!(mounted(&shell), ["a"], "{locale} {device}: a refused focus opens nothing");
    }
    for device in ["mobile", "tablet"] {
        let mut shell = focus_window_shell();
        install_device_layouts(&mut shell, &law, true);
        present_as(&mut shell, device);
        dispatch(&mut shell, "a", Some("c")).expect("the stand-in is focused");
        assert_eq!(shell.windows.active_window_id.as_deref(), Some("c"), "{device}");
        assert!(mounted(&shell).contains(&"c".to_string()) && !mounted(&shell).iter().any(|id| id == "b"), "{device}: only the stand-in opens");
    }
    let mut desktop = focus_window_shell();
    install_device_layouts(&mut desktop, &law, true);
    present_as(&mut desktop, "desktop");
    dispatch(&mut desktop, "a", Some("c")).expect("the desktop layout holds the requested kind");
    assert_eq!(desktop.windows.active_window_id.as_deref(), Some("a"));
}
