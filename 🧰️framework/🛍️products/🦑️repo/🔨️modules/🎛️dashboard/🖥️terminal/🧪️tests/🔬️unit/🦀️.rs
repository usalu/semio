use super::controls::{Action, Flow};
use super::sessions::Link;
use super::windows::{Body, Dashboard, Effect, LinkState};
use dashboard_launch::TaskLabel;
use dashboard_preferences::{Change, Preferences};
use dashboard_registry::{Facts, Registry};
use dashboard_wire::client::Message;
use dashboard_wire::ipc::{ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus};
use std::sync::Arc;
use ui_styling::appearance::AppearanceName;
use semio_framework_ui_tui_frame::chrome::ChromeState;
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_core::event::{mods, Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use semio_framework_ui_tui_core::geometry::{Pos, Size};
use semio_framework_ui_tui_core::theme::{Status, Theme};

fn fixture() -> serde_json::Value { serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/🧫️fixtures/🔎️launcher/🔣️.json")).unwrap() }

fn registry() -> Arc<Registry> {
    let facts: Facts = serde_json::from_value(fixture()["workspace"].clone()).unwrap();
    Arc::new(Registry::build(std::path::Path::new("workspace"), &facts))
}

fn dashboard_with(preferences: Preferences) -> (Dashboard, Tui) {
    let mut tui = Tui::new(Size { width: 160, height: 48 }, Theme::new(AppearanceName::Dark));
    let mut dashboard = Dashboard::new(&mut tui, "workspace".into(), preferences, std::env::temp_dir().join("unused-dashboard-preferences"), false);
    dashboard.link = Link::offline();
    dashboard.state = LinkState::Connected;
    let loaded = registry();
    dashboard.set_registry(&mut tui, loaded);
    dashboard.frame(&mut tui);
    (dashboard, tui)
}

fn small_view(language: &str) -> (Dashboard, Tui) {
    let mut tui = Tui::new(Size { width: 80, height: 24 }, Theme::new(AppearanceName::Dark));
    let mut dashboard = Dashboard::new(&mut tui, "workspace".into(), Preferences { language: language.into(), ..Default::default() }, std::env::temp_dir().join("unused-dashboard-preferences"), false);
    dashboard.link = Link::offline();
    dashboard.state = LinkState::Connected;
    let loaded = registry();
    dashboard.set_registry(&mut tui, loaded);
    dashboard.frame(&mut tui);
    (dashboard, tui)
}

fn view() -> (Dashboard, Tui) { dashboard_with(Preferences::default()) }

fn key(character: char) -> Event { Event::Key(KeyEvent { key: Key::Char(character), mods: 0 }) }


fn pointer(kind: MouseKind, pos: Pos) -> Event { Event::Mouse(MouseEvent { kind, pos, mods: 0, clicks: 1 }) }

fn ctrl(character: char) -> Event { Event::Key(KeyEvent { key: Key::Char(character), mods: mods::CTRL }) }
fn named(key: Key) -> Event { Event::Key(KeyEvent { key, mods: 0 }) }

fn press(dashboard: &mut Dashboard, tui: &mut Tui, events: &[Event]) -> Flow {
    let mut flow = Flow::Continue;
    for event in events { flow = dashboard.handle(tui, event); dashboard.frame(tui); }
    flow
}

fn tree_of<R>(dashboard: &Dashboard, tui: &mut Tui, index: usize, read: impl FnOnce(&semio_framework_ui_tui_widgets::widget::TreeState) -> R) -> R {
    match &tui.scene.node(dashboard.windows[index].tree.expect("a launcher has a tree")).content { semio_framework_ui_tui_frame::scene::NodeContent::Widget(semio_framework_ui_tui_widgets::widget::WidgetState::Tree(tree)) => read(tree), _ => panic!("tree expected") }
}

fn visible(dashboard: &Dashboard, tui: &mut Tui, index: usize) -> Vec<String> {
    let Body::Launcher(launcher) = &dashboard.windows[index].body else { panic!("launcher expected") };
    tree_of(dashboard, tui, index, |tree| launcher.visible_ids(tree))
}

fn query(dashboard: &Dashboard, tui: &mut Tui, index: usize) -> String { tree_of(dashboard, tui, index, |tree| tree.query().to_string()) }

fn type_text(text: &str) -> Vec<Event> { text.chars().map(key).collect() }

fn tabs(dashboard: &Dashboard, tui: &mut Tui, index: usize) -> Vec<String> {
    match tui.scene.node_mut(dashboard.windows[index].chrome).chrome() { Some(ChromeState::Window(window)) => window.stack_tabs.iter().map(|tab| tab.label.clone()).collect(), _ => Vec::new() }
}

fn statuses(dashboard: &Dashboard, tui: &mut Tui, index: usize) -> Vec<Option<Status>> {
    match tui.scene.node_mut(dashboard.windows[index].chrome).chrome() { Some(ChromeState::Window(window)) => window.stack_tabs.iter().map(|tab| tab.status).collect(), _ => Vec::new() }
}

fn titles(dashboard: &Dashboard, tui: &mut Tui) -> Vec<String> {
    (0..dashboard.windows.len()).map(|index| match tui.scene.node_mut(dashboard.windows[index].chrome).chrome() { Some(ChromeState::Window(window)) => window.title.clone(), _ => String::new() }).collect()
}

fn session(id: &str, label: TaskLabel, status: SessionStatus, code: Option<i32>) -> SessionInfo {
    SessionInfo { session_id: id.into(), command: SessionCommand { cmd: "bun".into(), args: vec!["repo".into(), "run".into(), "x".into()], cwd: "workspace".into(), cols: 80, rows: 24, label, ..Default::default() }, status, pid: Some(42), code, ..Default::default() }
}

fn label(verb: &str, subject: &str, qualifier: &str) -> TaskLabel { TaskLabel { verb: verb.into(), owner: vec!["quiz".into()], subject: subject.into(), qualifier: qualifier.into(), parameters: Vec::new(), members: 0 } }

fn spawned(dashboard: &Dashboard) -> Vec<(String, SessionCommand)> {
    dashboard.link.pending.iter().flat_map(|message| match message {
        ClientMsg::SpawnGroup { members, requires, .. } => requires.iter().chain(members).map(|member| (member.session_id.clone(), member.command.clone())).collect::<Vec<_>>(),
        _ => Vec::new(),
    }).collect()
}

include!("🔗️open-ready/🦀️.rs");

include!("🧲️docking/🦀️.rs");

include!("🎹️keymap-and-launcher/🦀️.rs");

include!("🖼️window-focus-and-sessions/🦀️.rs");

include!("🩺️runtime-audit/🦀️.rs");

include!("🧭️scenario-proofs-and-batching/🦀️.rs");
