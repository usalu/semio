use dashboard_terminal::testing::{Action, Body, Dashboard, Effect, Flow, Link, LinkState};
use dashboard_launch::TaskLabel;
use dashboard_preferences::{Change, Preferences};
use dashboard_registry::{Facts, Registry};
use dashboard_wire::client::Message;
use dashboard_wire::ipc::{ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus};
use std::sync::Arc;
use ui_styling::appearance::AppearanceName;
use semio_framework_ui_tui_frame::{chrome::ChromeState,scene::NodeContent};
use semio_framework_ui_tui_widgets::widget::{TreeState,WidgetState};
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_core::event::{mods, Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use semio_framework_ui_tui_core::geometry::{Pos, Size};
use semio_framework_ui_tui_core::theme::{Status, Theme};

fn fixture() -> serde_json::Value { serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../🧪️tests/🧫️fixtures/🔎️launcher/🔣️.json")).unwrap() }

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
    let mut dashboard = Dashboard::new(&mut tui, "workspace".into(), Preferences { language: Some(language.into()), ..Default::default() }, std::env::temp_dir().join("unused-dashboard-preferences"), false);
    dashboard.link = Link::offline();
    dashboard.state = LinkState::Connected;
    let loaded = registry();
    dashboard.set_registry(&mut tui, loaded);
    dashboard.frame(&mut tui);
    (dashboard, tui)
}

fn view() -> (Dashboard, Tui) { dashboard_with(Preferences {language:Some("en".into()),..Default::default()}) }

fn key(character: char) -> Event { Event::Key(KeyEvent { key: Key::Char(character), mods: 0 }) }


fn pointer(kind: MouseKind, pos: Pos) -> Event { Event::Mouse(MouseEvent { kind, pos, mods: 0, clicks: 1 }) }

fn ctrl(character: char) -> Event { Event::Key(KeyEvent { key: Key::Char(character), mods: mods::CTRL }) }
fn named(key: Key) -> Event { Event::Key(KeyEvent { key, mods: 0 }) }

fn press(dashboard: &mut Dashboard, tui: &mut Tui, events: &[Event]) -> Flow {
    let mut flow = Flow::Continue;
    for event in events { flow = dashboard.handle(tui, event); dashboard.frame(tui); }
    flow
}

fn tree_of<R>(dashboard: &Dashboard, tui: &mut Tui, index: usize, read: impl FnOnce(&TreeState) -> R) -> R {
    match &tui.scene.node(dashboard.windows[index].tree.expect("a launcher has a tree")).content { NodeContent::Widget(WidgetState::Tree(tree)) => read(tree), _ => panic!("tree expected") }
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

#[path = "📂️sources/🦀️.rs"]
mod sources;

include!("🔗️open-ready/🦀️.rs");

include!("🧲️docking/🦀️.rs");

include!("🎹️keymap-and-launcher/🦀️.rs");

include!("🖼️window-focus-and-sessions/🦀️.rs");

include!("🩺️runtime-audit/🦀️.rs");

include!("🧭️scenario-proofs-and-batching/🦀️.rs");


struct FeedbackBackend {
    failure:Option<&'static str>,
    cleanup:bool,
    copy_fails:bool,
    entered:usize,
    left:usize,
    waits:usize,
    copied:Vec<String>,
}
impl FeedbackBackend {
    fn new(failure:Option<&'static str>,cleanup:bool)->Self{Self{failure,cleanup,copy_fails:false,entered:0,left:0,waits:0,copied:Vec::new()}}
    fn check(&self,operation:&str)->std::io::Result<()>{if self.failure==Some(operation){Err(std::io::Error::other(format!("fixture {operation}")))}else{Ok(())}}
}
impl semio_framework_ui_tui_terminal::backend::TerminalBackend for FeedbackBackend {
    fn enter(&mut self)->std::io::Result<()>{self.entered+=1;self.check("enter")}
    fn leave(&mut self)->std::io::Result<()>{self.left+=1;if self.cleanup{Err(std::io::Error::other("fixture leave"))}else{Ok(())}}
    fn size(&self)->std::io::Result<Size>{Ok(Size{width:80,height:24})}
    fn capabilities(&self)->semio_framework_ui_tui_terminal::backend::Capabilities{semio_framework_ui_tui_terminal::backend::detect_capabilities(&|_|None,false)}
    fn present(&mut self,_:&semio_framework_ui_tui_terminal::ansi::AnsiPatch,_:Option<semio_framework_ui_tui_core::signal::CursorSpec>)->std::io::Result<()>{self.check("present")}
    fn wait(&mut self,_:Option<std::time::Instant>)->std::io::Result<Vec<Event>>{self.check("wait")?;self.waits+=1;assert!(self.waits<=2);Ok(if self.waits==1{vec![Event::Wake]}else{vec![ctrl('b'),key('d')]})}
    fn waker(&self)->semio_framework_ui_tui_terminal::backend::Waker{semio_framework_ui_tui_terminal::backend::Waker::new(||{})}
    fn copy(&mut self,text:&str)->std::io::Result<()>{self.copied.push(text.into());if self.copy_fails{Err(std::io::Error::other("fixture clipboard"))}else{Ok(())}}
}

#[test]
fn terminal_backend_failures_preserve_primary_and_cleanup_from_portable_cases(){
    let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../📣️feedback/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["terminalCases"].as_array().unwrap(){
        let failure=match row["fail"].as_str(){Some("enter")=>Some("enter"),Some("present")=>Some("present"),Some("wait")=>Some("wait"),_=>None};
        let mut backend=FeedbackBackend::new(failure,row["leaveFails"].as_bool().unwrap());
        let (mut dashboard,mut tui)=view();
        dashboard.link.online_for_test=true;
        dashboard.link.fail_sends=row["detachFails"].as_bool().unwrap();
        let outcome=dashboard_feedback::terminal_session(&mut backend,|backend|dashboard_terminal::testing::run_view(backend,&mut dashboard,&mut tui));
        assert_eq!(backend.entered,1);
        assert_eq!(backend.left,1);
        assert_eq!(outcome.is_err(),row["expected"]["exit"]==1);
        let primary=outcome.as_ref().err().map(|error|error.primary.operation.key());
        let cleanup=outcome.as_ref().err().map(|error|error.cleanup.iter().map(|error|error.operation.key()).collect::<Vec<_>>()).unwrap_or_default();
        assert_eq!(serde_json::to_value(primary).unwrap(),row["expected"]["primary"]);
        assert_eq!(serde_json::to_value(cleanup).unwrap(),row["expected"]["cleanup"]);
        if failure!=Some("enter")&&!row["detachFails"].as_bool().unwrap(){assert!(dashboard.link.outbox.iter().any(|message|matches!(message,ClientMsg::Detach{})));}
    }
    let (mut dashboard,mut tui)=view();dashboard.link.fail_sends=true;
    let mut backend=FeedbackBackend::new(None,false);
    assert!(dashboard_feedback::terminal_session(&mut backend,|backend|dashboard_terminal::testing::run_view(backend,&mut dashboard,&mut tui)).is_ok());
    assert!(!dashboard.link.outbox.iter().any(|message|matches!(message,ClientMsg::Detach{})));
}

#[test]
fn unresolved_terminal_failure_reports_both_declared_languages_without_a_default(){
    let failure=dashboard_feedback::TerminalFailure{primary:dashboard_feedback::Failure::io(dashboard_feedback::Operation::Enter,std::io::Error::other("fixture")),cleanup:vec![dashboard_feedback::Failure::io(dashboard_feedback::Operation::Leave,std::io::Error::other("cleanup"))]};
    let output=failure.display(None);
    for locale in [ui_locale::Locale::En,ui_locale::Locale::De]{
        let text=dashboard_labels::labels(locale);
        assert!(output.contains(text.local_enter.as_str()));
        assert!(output.contains(text.local_leave.as_str()));
    }
}

#[test]
fn clipboard_failure_is_recoverable_and_diagnostics_remain_inspectable_and_copyable(){
    let (mut dashboard,mut tui)=view();
    dashboard.effects.push(Effect::Copy("fixture".into()));
    let mut backend=FeedbackBackend::new(None,false);backend.copy_fails=true;
    assert!(dashboard_feedback::terminal_session(&mut backend,|backend|dashboard_terminal::testing::run_view(backend,&mut dashboard,&mut tui)).is_ok());
    assert_eq!(backend.left,1);assert_eq!(dashboard.diagnostics.len(),1);
    assert_eq!(dashboard.diagnostics.get(0).unwrap().operation,dashboard_feedback::Operation::Copy);
    press(&mut dashboard,&mut tui,&[ctrl('b'),key('!')]);
    let index=dashboard.focused_index(&tui);
    assert!(matches!(dashboard.windows[index].body,Body::Diagnostics));
    dashboard.refresh_pane(&mut tui,index);
    dashboard.windows[index].pane.selected=dashboard.windows[index].pane.rows.iter().position(|(_,action)|matches!(action,dashboard_terminal::testing::RowAction::Diagnostic(0))).unwrap();
    dashboard.activate_row(&mut tui,index);
    assert!(dashboard.effects.iter().any(|effect|matches!(effect,Effect::Copy(text) if text.contains("fixture clipboard"))));
}

#[test]
fn local_control_failures_are_typed_translated_and_retained_under_a_bounded_history(){
    let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../📣️feedback/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for locale in fixture["locales"].as_array().unwrap(){
        let (mut dashboard,mut tui)=small_view(locale["language"].as_str().unwrap());
        let info=session("failed-control",label("run","fixture",""),SessionStatus::Running,None);
        dashboard.update_session(&mut tui,info);
        let index=dashboard.windows.iter().position(|window|window.session().is_some_and(|session|session.session_id=="failed-control")).unwrap();
        dashboard.link.fail_sends=true;
        dashboard.control_session(&mut tui,index,"stop-task");
        assert!(dashboard.notice.as_ref().unwrap().contains(locale["stop"].as_str().unwrap()));
        assert!(dashboard.notice.as_ref().unwrap().contains(dashboard.text().local_disconnected.as_str()));
        assert_eq!(dashboard.diagnostics.get(0).unwrap().kind,std::io::ErrorKind::NotConnected);
    }
    let mut history=dashboard_feedback::History::default();
    for index in 0..fixture["history"]["count"].as_u64().unwrap(){history.push(dashboard_feedback::Failure::text(dashboard_feedback::Operation::Input,index.to_string()));}
    assert_eq!(history.len(),fixture["history"]["retained"].as_u64().unwrap() as usize);
    assert_eq!(history.get(0).unwrap().detail,fixture["history"]["first"].to_string());
    let detail=dashboard_feedback::Failure::text(dashboard_feedback::Operation::Input,"é".repeat(8192));
    assert_eq!(detail.detail.len(),fixture["maximumDetailBytes"].as_u64().unwrap() as usize);
}

#[test]
fn settings_keymap_drafts_match_portable_save_cancel_and_refusal_cases(){
    let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../⚙️settings/⌨️keymap/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let original=Preferences{language:Some("en".into()),..Default::default()};
        let mut editor=dashboard_keymap_editor::Editor::new(&original);
        if let Some(prefix)=row["prefix"].as_str(){editor.prefix=prefix.into();}
        if let Some(binding)=row["binding"].as_object(){editor.bindings.insert(binding["id"].as_str().unwrap().into(),binding["keys"].as_array().unwrap().iter().map(|key|key.as_str().unwrap().into()).collect());}
        let outcome=if row["cancel"].as_bool().unwrap(){"cancelled"}else{match editor.change(){Err(_)=>"refused",Ok(None)=>"unchanged",Ok(Some(change))=>{let mut next=original.clone();next.apply(&change);assert!(next.keymap().1.is_empty());"saved"}}};
        assert_eq!(outcome,row["expected"].as_str().unwrap(),"{row}");
        assert_eq!(original.prefix,Preferences::default().prefix);
    }
}

#[test]
fn settings_prefix_is_editable_by_real_keys_and_cancel_never_queues_an_event(){
    let (mut dashboard,mut tui)=view();
    press(&mut dashboard,&mut tui,&[ctrl('b'),key('p')]);
    let index=dashboard.focused_index(&tui);
    dashboard.windows[index].pane.selected=dashboard.windows[index].pane.rows.iter().position(|(_,action)|*action==dashboard_terminal::testing::RowAction::EditPrefix).unwrap();
    dashboard.activate_row(&mut tui,index);
    press(&mut dashboard,&mut tui,&[ctrl('u')]);
    press(&mut dashboard,&mut tui,&type_text("ctrl+e"));
    press(&mut dashboard,&mut tui,&[named(Key::Enter)]);
    let Body::KeymapEditor(editor)=&dashboard.windows[index].body else{panic!("editor")};
    assert_eq!(editor.change().unwrap().unwrap().prefix.as_deref(),Some("ctrl+e"));
    dashboard.editor_action(&mut tui,index,dashboard_keymap_editor::Action::Cancel);
    assert!(dashboard.saving_preferences.is_none());
    assert_eq!(dashboard.keymap.prefix().to_string(),"ctrl+b");
    assert!(matches!(dashboard.windows[index].body,Body::Settings));
}

#[test]
fn keymap_alias_reset_and_durable_save_results_preserve_other_preferences(){
    let original=Preferences{language:Some("de".into()),appearance:"light".into(),..Default::default()};
    let mut editor=dashboard_keymap_editor::Editor::new(&original);
    editor.begin(dashboard_keymap_editor::Target::Alias("prefix.show-help".into(),0));editor.input="f12".into();editor.commit().unwrap();
    editor.reset(Some("prefix.show-help"));assert!(editor.change().unwrap().is_none());
    editor.prefix="ctrl+e".into();
    let change=editor.change().unwrap().unwrap();
    let (mut dashboard,mut tui)=dashboard_with(original);
    let (send,receiver)=std::sync::mpsc::channel();
    let cancel=Arc::new(std::sync::atomic::AtomicBool::new(false));
    dashboard.saving_preferences=Some(dashboard_terminal::testing::PreferenceJob{receiver,cancel:cancel.clone()});
    dashboard.set_body(&mut tui,0,dashboard_terminal::testing::Kind::Settings,None);
    dashboard.refresh_pane(&mut tui,0);
    dashboard.windows[0].pane.selected=dashboard.windows[0].pane.rows.iter().position(|(_,action)|*action==dashboard_terminal::testing::RowAction::CancelPreference).unwrap();
    dashboard.activate_row(&mut tui,0);
    assert!(cancel.load(std::sync::atomic::Ordering::Acquire));
    send.send((change.clone(),Ok(1))).unwrap();
    assert!(dashboard.poll_background(&mut tui));
    assert_eq!(dashboard.keymap.prefix().to_string(),"ctrl+e");
    assert_eq!(dashboard.preferences.language.as_deref(),Some("de"));
    assert_eq!(dashboard.preferences.appearance,"light");
    let (send,receiver)=std::sync::mpsc::channel();
    let cancel=Arc::new(std::sync::atomic::AtomicBool::new(false));
    dashboard.saving_preferences=Some(dashboard_terminal::testing::PreferenceJob{receiver,cancel});
    send.send((Change{prefix:Some("ctrl+a".into()),..Default::default()},Err(std::io::Error::from(std::io::ErrorKind::Interrupted)))).unwrap();
    dashboard.poll_background(&mut tui);
    assert_eq!(dashboard.keymap.prefix().to_string(),"ctrl+e");
    assert!(dashboard.notice.as_ref().unwrap().contains(dashboard.text().prefs_cancelled.as_str()));
}
