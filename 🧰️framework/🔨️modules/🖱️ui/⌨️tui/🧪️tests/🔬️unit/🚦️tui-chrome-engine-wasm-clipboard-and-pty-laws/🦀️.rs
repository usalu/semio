// 🚦️ Tui chrome, engine, wasm, clipboard and pty laws: the navbar and footer chrome, the engine focus, dispatch and render laws, the wasm host setup, the osc52 payload and the pty spawn, kill and resize.

//#region 🔖️Chrome
#[test]
fn paint_navbar_places_left_center_and_right_items() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 30, 2);
    let mut buf = CellBuffer::new(Size { width: 30, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let navbar = NavbarState {
        left: vec![NavItem { id: "l".to_string(), label: "L".to_string(), active: false }],
        center: vec![NavItem { id: "c".to_string(), label: "C".to_string(), active: false }],
        right: vec![NavItem { id: "r".to_string(), label: "R".to_string(), active: true }],
    };
    ChromeState::Navbar(navbar).paint(&theme, rect, &mut buf);
    let row = row_text(&buf, 0);
    assert!(row.trim_start().starts_with('L'));
    assert!(row.contains('C'));
    assert!(row.trim_end().ends_with('R'));
    assert_eq!(row_text(&buf, 1), "\u{2500}".repeat(30), "hairline drawn below the navbar");
    let right_x = (0..30).find(|&x| buf.get(x, 0).unwrap().ch == 'R').expect("right item rendered");
    assert_eq!(buf.get(right_x, 0).unwrap().fg, theme.role(Role::Accent), "the active item uses the accent color");
}

#[test]
fn paint_footer_renders_hints_then_status() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 30, 2);
    let mut buf = CellBuffer::new(Size { width: 30, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let footer = FooterState { hints: vec![KeyHint { key: "q".to_string(), label: "quit".to_string() }], status: "OK".to_string() };
    ChromeState::Footer(footer).paint(&theme, rect, &mut buf);
    assert_eq!(row_text(&buf, 0), "\u{2500}".repeat(30), "hairline drawn above the footer");
    let row1 = row_text(&buf, 1);
    assert!(row1.contains('q'));
    assert!(row1.contains("quit"));
    assert!(row1.trim_end().ends_with("OK"));
}
//#endregion 🔖️Chrome

//#region 🔖️Engine

#[test]
fn engine_mouse_move_does_not_steal_focus() {
    use semio_framework_ui_tui_engine::Tui;
    let mut tui = Tui::new(Size { width: 20, height: 10 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    let a = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::List(ListState::new(vec!["a".to_string()])))));
    let b = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::List(ListState::new(vec!["b".to_string()])))));
    tui.scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    tui.scene.node_mut(a).set_constraint(Constraint { width: Dimension::Cells(10), ..Default::default() });
    tui.scene.node_mut(b).set_constraint(Constraint { width: Dimension::Cells(10), ..Default::default() });
    let _ = tui.render_full();
    tui.set_focus(Some(a));
    tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Move, pos: Pos { x: 15, y: 0 }, mods: 0, clicks: 1 }));
    assert_eq!(tui.focus(), Some(a), "move must not change focus");
    tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: 15, y: 0 }, mods: 0, clicks: 1 }));
    assert_eq!(tui.focus(), Some(b), "click focuses the hit widget");
}

#[test]
fn tui_focus_next_and_prev_cycle_through_focusables() {
    let mut tui = semio_framework_ui_tui_engine::Tui::new(Size { width: 20, height: 5 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    let a = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::List(ListState::new(vec!["a".to_string()])))));
    let b = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::List(ListState::new(vec!["b".to_string()])))));
    assert_eq!(tui.focus(), None);
    tui.focus_next();
    assert_eq!(tui.focus(), Some(a));
    tui.focus_next();
    assert_eq!(tui.focus(), Some(b));
    tui.focus_next();
    assert_eq!(tui.focus(), Some(a), "focus wraps back to the first focusable");
    tui.focus_prev();
    assert_eq!(tui.focus(), Some(b), "focus wraps backward past the first focusable");
}

#[test]
fn tui_dispatch_tab_and_backtab_move_focus() {
    let mut tui = semio_framework_ui_tui_engine::Tui::new(Size { width: 20, height: 5 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    let a = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::List(ListState::new(vec!["a".to_string()])))));
    let b = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::List(ListState::new(vec!["b".to_string()])))));
    let tab_ev = Event::Key(KeyEvent { key: Key::Tab, mods: 0 });
    assert_eq!(tui.dispatch(&tab_ev), vec![]);
    assert_eq!(tui.focus(), Some(a));
    let backtab_ev = Event::Key(KeyEvent { key: Key::BackTab, mods: 0 });
    tui.dispatch(&backtab_ev);
    assert_eq!(tui.focus(), Some(b));
}

#[test]
fn tui_render_skips_repaint_when_nothing_dirty() {
    let mut tui = semio_framework_ui_tui_engine::Tui::new(Size { width: 10, height: 3 }, Theme::new(AppearanceName::Dark));
    let first = tui.render_full();
    assert!(!first.0.is_empty());
    let second = tui.render();
    assert!(second.0.is_empty(), "no dirty state and no forced redraw emits nothing");
}

#[test]
fn tui_set_appearance_and_resize_force_full_redraw() {
    let mut tui = semio_framework_ui_tui_engine::Tui::new(Size { width: 10, height: 3 }, Theme::new(AppearanceName::Dark));
    tui.render_full();
    tui.set_appearance(AppearanceName::Light);
    let patch = tui.render();
    assert!(!patch.0.is_empty(), "set_appearance forces a full repaint even with no dirty nodes");
    tui.render();
    tui.resize(Size { width: 12, height: 4 });
    let patch2 = tui.render();
    assert!(!patch2.0.is_empty(), "resize forces a full repaint too");
}
//#endregion 🔖️Engine

//#region 🔖️WasmHost
#[test]
fn wasm_host_setup_and_teardown_match_ansi_sequences() {
    let host = semio_framework_ui_tui_host::WasmHost::new(10, 5, true);
    assert_eq!(host.setup(), setup_sequence());
    assert_eq!(host.teardown(), teardown_sequence());
}

#[test]
fn wasm_host_resize_updates_engine_size() {
    let mut host = semio_framework_ui_tui_host::WasmHost::new(10, 5, true);
    host.render();
    host.resize(20, 8);
    let patch = host.render();
    assert!(!patch.is_empty(), "resizing triggers a full repaint on the next render");
}
//#endregion 🔖️WasmHost

//#region 🔖️Clipboard
#[test]
fn osc52_copy_sequence_is_base64_payload() {
    let seq = semio_framework_ui_tui_terminal::backend::osc52_copy_sequence("hi");
    assert_eq!(seq.as_bytes()[0], 0x1b);
    assert!(seq.contains("]52;c;aGk="), "got {seq:?}");
    assert_eq!(*seq.as_bytes().last().unwrap(), 0x07);
    let mut mem = semio_framework_ui_tui_terminal::backend::MemoryClipboard::default();
    semio_framework_ui_tui_terminal::backend::Clipboard::enqueue_copy(&mut mem, "abc".to_string());
    assert!(matches!(semio_framework_ui_tui_terminal::backend::Clipboard::poll(&mut mem), Some(semio_framework_ui_tui_terminal::backend::ClipboardResult::Copied)));
    semio_framework_ui_tui_terminal::backend::Clipboard::enqueue_paste(&mut mem);
    assert!(matches!(
        semio_framework_ui_tui_terminal::backend::Clipboard::poll(&mut mem),
        Some(semio_framework_ui_tui_terminal::backend::ClipboardResult::Pasted(text)) if text == "abc"
    ));
}
//#endregion 🔖️Clipboard

//#region 🔖️Pty
#[cfg(all(unix, feature = "native"))]
#[test]
fn pty_spawn_echo_hello() {
    use semio_framework_ui_tui_terminal::pty::{Pty, PtySize};
    use std::time::{Duration, Instant};

    let mut pty = Pty::spawn("/bin/echo", &["hello"], &[], &[], None, PtySize { cols: 80, rows: 24 }).expect("spawn echo");
    let mut out = Vec::new();
    let mut buf = [0u8; 1024];
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        match pty.try_read(&mut buf) {
            Ok(0) => {
                let _ = pty.try_wait();
                if Instant::now() >= deadline {
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(n) => {
                out.extend_from_slice(&buf[..n]);
                if String::from_utf8_lossy(&out).contains("hello") {
                    break;
                }
            }
            Err(e) => panic!("try_read failed: {}", e.message),
        }
    }
    let text = String::from_utf8_lossy(&out);
    assert!(text.contains("hello"), "PTY output missing hello: {text:?}");
}

#[cfg(all(unix, feature = "native"))]
#[test]
fn pty_kill_terminates_child_process_group() {
    use semio_framework_ui_tui_terminal::pty::{Pty, PtySize};
    use std::process as system_process;
    use std::time::{Duration, Instant};

    let mut pty = Pty::spawn("bash", &["-c", "sleep 300 & sleep 300; wait"], &[], &[], None, PtySize { cols: 80, rows: 24 }).expect("spawn bash sleep group");
    let pid = pty.pid();
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut saw_child = false;
    while Instant::now() < deadline {
        let out = system_process::Command::new("pgrep").args(["-P", &pid.to_string()]).output().expect("pgrep");
        if !out.stdout.is_empty() {
            saw_child = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    assert!(saw_child, "expected child processes under pty leader pid={pid}");
    pty.kill().expect("kill pty group");
    std::thread::sleep(Duration::from_millis(200));
    let out = system_process::Command::new("pgrep").args(["-P", &pid.to_string()]).output().expect("pgrep after kill");
    assert!(out.stdout.is_empty(), "child processes still alive after kill: {:?}", String::from_utf8_lossy(&out.stdout));
    let leader = system_process::Command::new("ps").args(["-p", &pid.to_string()]).output().expect("ps leader");
    assert!(leader.status.success() == false || leader.stdout.len() < 2, "pty leader still running");
}

#[cfg(all(unix, feature = "native"))]
#[test]
fn pty_resize_ok() {
    use semio_framework_ui_tui_terminal::pty::{Pty, PtySize};

    let mut pty = Pty::spawn("/bin/sleep", &["1"], &[], &[], None, PtySize { cols: 80, rows: 24 }).expect("spawn sleep");
    pty.resize(PtySize { cols: 100, rows: 40 }).expect("resize");
    pty.kill().expect("kill");
}
//#endregion 🔖️Pty
