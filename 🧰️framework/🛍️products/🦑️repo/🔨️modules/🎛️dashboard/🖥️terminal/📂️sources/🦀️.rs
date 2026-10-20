//! 📂️ Native workspace browsing and document requests kept outside the terminal event loop.

use super::windows::{Body, Dashboard, Kind, Place};
use semio_framework_filesystem_workspace::{Document, Entry, Error, Progress, Revision, Workspace, Stage, ErrorKind};
use semio_framework_ui_tui_core::event::{mods, Key, KeyEvent, Event};
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_widgets::widget::{EditorState, WidgetState};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver}};

#[derive(Default)]
pub(super) struct Files { pub directory: String, pub query: String, pub entries: Vec<Entry>, pub visible: Vec<usize> }

pub(super) struct OpenDocument { pub document: Document, pub read_only: bool, pub recovery: Option<String>, pub dirty: bool, pub generation: u64, pub discard_confirmation: bool }

pub(super) enum Request { List(String), Load(String), Recovery(String), Save { path: String, revision: Revision, text: String } }
enum Response { List(String, Vec<Entry>), Load(Document), Recovery(Document), Save(Document) }
enum Message { Progress(Progress), Finished(Result<Response, Error>) }

pub(super) struct Job { window: String, operation: u64, generation: u64, cancelled: Arc<AtomicBool>, receive: Receiver<Message> }
impl Drop for Job { fn drop(&mut self) { self.cancelled.store(true, Ordering::Release); } }

fn parent(path: &str) -> String { path.rsplit_once('/').map_or_else(String::new, |(parent, _)| parent.into()) }

impl Dashboard {
    pub fn open_files(&mut self, tui: &mut Tui) {
        let index = self.windows.iter().position(|window| matches!(window.body, Body::Files(_))).unwrap_or_else(|| self.create_window(tui, Kind::Files, Place::Preference));
        self.focus_window(tui, index);
        let directory = match &self.windows[index].body { Body::Files(files) => files.directory.clone(), _ => String::new() };
        self.request_source(index, Request::List(directory));
    }

    fn request_source(&mut self, index: usize, request: Request) {
        if let Some(job) = &self.source_job {
            job.cancelled.store(true, Ordering::Release);
            self.source_pending = Some((self.windows[index].id.clone(), request));
            self.notice = Some(self.text().source_busy.as_str().into());
            return;
        }
        let Some(operation) = self.source_operation.checked_add(1) else { return }; self.source_operation = operation;
        self.notice = Some(self.text().source_starting.fill(&[("cancel", &self.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "cancel-source").unwrap_or_default())]).into_string());
        let root = self.root.clone(); let window = self.windows[index].id.clone();
        let generation = match &self.windows[index].body { Body::Document(Some(document)) => document.generation, _ => 0 };
        let cancelled = Arc::new(AtomicBool::new(false)); let signal = cancelled.clone(); let waker = self.waker.clone();
        let (send, receive) = mpsc::sync_channel(32);
        let spawn = std::thread::Builder::new().name("workspace-document".into()).spawn(move || {
            let result = Workspace::new(&root).and_then(|workspace| {
                let mut progress = |progress| { let _ = send.try_send(Message::Progress(progress)); if let Some(waker) = &waker { waker.wake(); } };
                match request {
                    Request::List(path) => workspace.list(&path, &signal, &mut progress).map(|entries| Response::List(path, entries)),
                    Request::Load(path) => workspace.load(&path, &signal, &mut progress).map(Response::Load),
                    Request::Recovery(path) => workspace.load_recovery(&path, &signal, &mut progress).map(Response::Recovery),
                    Request::Save { path, revision, text } => workspace.save(&path, &revision, &text, &signal, &mut progress).map(Response::Save),
                }
            });
            let _ = send.send(Message::Finished(result)); if let Some(waker) = waker { waker.wake(); }
        });
        match spawn {
            Ok(_) => self.source_job = Some(Job { window, operation, generation, cancelled, receive }),
            Err(error) => self.notice = Some(self.text().source_failed.fill(&[("problem", &error.to_string())]).into_string()),
        }
    }

    pub fn cancel_source(&mut self) { self.source_pending = None; if let Some(job) = &self.source_job { job.cancelled.store(true, Ordering::Release); } self.notice = Some(self.text().source_cancelled.as_str().into()); }

    pub fn poll_documents(&mut self, tui: &mut Tui) -> bool {
        let mut changed = false;
        loop {
            let Some(job) = &self.source_job else { break };
            let message = match job.receive.try_recv() {
                Ok(message) => message,
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.source_job.take(); self.notice = Some(self.text().source_worker_failed.as_str().into());
                    if let Some((window, request)) = self.source_pending.take() { if let Some(index) = self.window_index(&window) { self.request_source(index, request); } }
                    changed = true; break;
                }
            };
            let (window, operation, generation) = (job.window.clone(), job.operation, job.generation);
            let cancelled = job.cancelled.load(Ordering::Acquire);
            if cancelled && matches!(message, Message::Progress(_)) { continue; }
            let committed_save = matches!(&message, Message::Finished(Ok(Response::Save(_)))) || matches!(&message, Message::Finished(Err(error)) if error.recovery_path.is_some());
            if matches!(message, Message::Finished(_)) && ((cancelled && !committed_save) || operation != self.source_operation || self.window_index(&window).is_none()) {
                self.source_job.take();
                if let Some((window, request)) = self.source_pending.take() { if let Some(index) = self.window_index(&window) { self.request_source(index, request); } }
                changed = true;
                break;
            }
            let Some(index) = self.window_index(&window) else { continue };
            changed = true;
            match message {
                Message::Progress(progress) => self.notice = Some(self.text().source_loading.fill(&[("stage", &self.source_stage(&progress.stage)), ("completed", &progress.completed.to_string()), ("total", &progress.total.map_or_else(|| "?".into(), |total| total.to_string())), ("cancel", &self.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "cancel-source").unwrap_or_default())]).into_string()),
                Message::Finished(result) => {
                    self.source_job.take();
                    match result {
                        Ok(Response::List(directory, entries)) => {
                            if let Body::Files(files) = &mut self.windows[index].body { files.directory = directory; files.entries = entries; files.query.clear(); }
                            self.paint_files(tui, index);
                            self.notice = None;
                        }
                        Ok(response @ (Response::Load(_) | Response::Recovery(_))) => {
                            let (document, read_only) = match response { Response::Load(document) => (document, false), Response::Recovery(document) => (document, true), _ => unreachable!() };
                            if matches!(&self.windows[index].body, Body::Document(Some(current)) if current.generation != generation || current.dirty) { continue; }
                            self.set_body(tui, index, Kind::Document, None);
                            let mut editor = EditorState::new(document.text.clone(), 1_048_576); editor.cursor = 0; editor.read_only = read_only;
                            if let Some(widget) = tui.scene.node_mut(self.windows[index].list).widget() { *widget = WidgetState::Editor(editor); }
                            self.windows[index].body = Body::Document(Some(Box::new(OpenDocument { document, read_only, recovery: None, dirty: false, generation: 0, discard_confirmation: false })));
                            self.notice = Some(self.text().source_reloaded.as_str().into());
                        }
                        Ok(Response::Save(document)) => {
                            let node = self.windows[index].list;
                            let dirty = matches!(tui.scene.node(node).content, semio_framework_ui_tui_frame::scene::NodeContent::Widget(WidgetState::Editor(ref editor)) if editor.value != document.text);
                            if let Body::Document(Some(current)) = &mut self.windows[index].body { if current.document.path == document.path { current.document = document; current.dirty = dirty; current.discard_confirmation = false; } }
                            self.notice = Some(self.text().source_saved.as_str().into());
                        }
                        Err(error) => {
                            if let Body::Document(Some(current)) = &mut self.windows[index].body { current.recovery = error.recovery_path.clone(); }
                            self.notice = Some(self.text().source_failed.fill(&[("problem", &self.source_error(&error))]).into_string());
                        }
                    }
                    self.remount(tui);
                    if let Some((window, request)) = self.source_pending.take() { if let Some(index) = self.window_index(&window) { self.request_source(index, request); } }
                    break;
                }
            }
        }
        changed
    }

    pub fn paint_files(&mut self, tui: &mut Tui, index: usize) {
        let Body::Files(files) = &mut self.windows[index].body else { return };
        let query = files.query.to_lowercase();
        files.visible = files.entries.iter().enumerate().filter(|(_, entry)| entry.name.to_lowercase().contains(&query)).map(|(index, _)| index).collect();
        let mut items: Vec<String> = files.visible.iter().map(|index| { let entry = &files.entries[*index]; format!("{}{}", entry.name, if entry.directory { "/" } else { "" }) }).collect();
        if !files.directory.is_empty() && files.query.is_empty() { items.insert(0, "../".into()); }
        let node = self.windows[index].list;
        if let Some(WidgetState::List(list)) = tui.scene.node_mut(node).widget() { list.selected = list.selected.min(items.len().saturating_sub(1)); list.marks = vec![false; items.len()]; list.items = items; }
    }

    pub fn files_key(&mut self, tui: &mut Tui, index: usize, key: &KeyEvent) {
        let mut directory = None;
        if let Body::Files(files) = &mut self.windows[index].body {
            match key.key {
                Key::Char(c) if key.mods & (mods::CTRL | mods::ALT) == 0 && !c.is_control() && files.query.len() + c.len_utf8() <= 256 => files.query.push(c),
                Key::Backspace if !files.query.is_empty() => { files.query.truncate(semio_framework_ui_tui_core::text::previous_boundary(&files.query, files.query.len())); }
                Key::Backspace | Key::Esc => directory = Some(parent(&files.directory)),
                _ => { let signals = tui.dispatch(&Event::Key(*key)); self.on_signals(tui, signals); return; }
            }
        }
        if let Some(directory) = directory { self.request_source(index, Request::List(directory)); } else { self.paint_files(tui, index); }
    }

    pub fn activate_file(&mut self, tui: &Tui, index: usize, selected: usize) {
        let Body::Files(files) = &self.windows[index].body else { return };
        let parent_visible = !files.directory.is_empty() && files.query.is_empty();
        if selected == 0 && parent_visible { self.request_source(index, Request::List(parent(&files.directory))); return; }
        let selected = selected.saturating_sub(usize::from(parent_visible));
        let Some(entry) = files.visible.get(selected).and_then(|selected| files.entries.get(*selected)) else { return };
        let request = if entry.directory { Request::List(entry.path.clone()) } else { Request::Load(entry.path.clone()) };
        let _ = tui; self.request_source(index, request);
    }

    pub fn save_document(&mut self, tui: &Tui, index: usize) {
        let Body::Document(Some(current)) = &self.windows[index].body else { return };
        if current.read_only { self.notice = Some(self.text().source_read_only.as_str().into()); return; }
        let semio_framework_ui_tui_frame::scene::NodeContent::Widget(WidgetState::Editor(editor)) = &tui.scene.node(self.windows[index].list).content else { return };
        let request = Request::Save { path: current.document.path.clone(), revision: current.document.revision.clone(), text: editor.value.clone() };
        self.request_source(index, request);
    }

    pub fn inspect_recovery(&mut self, tui: &mut Tui, index: usize) {
        let Some(path) = (match &self.windows[index].body { Body::Document(Some(document)) => document.recovery.clone(), _ => None }) else { return };
        let target = self.create_window(tui, Kind::Document, Place::Preference); self.focus_window(tui, target);
        self.request_source(target, Request::Recovery(path));
    }

    pub fn dirty_close(&mut self, index: usize) -> bool {
        let dirty = matches!(&self.windows[index].body, Body::Document(Some(document)) if document.dirty);
        if dirty {
            if let Body::Document(Some(document)) = &mut self.windows[index].body { document.discard_confirmation = true; }
            let save = self.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "save-document").unwrap_or_default();
            let discard = self.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "discard-document").unwrap_or_default();
            self.notice = Some(self.text().source_confirm_discard.fill(&[("save", &save), ("discard", &discard)]).into_string());
        }
        dirty
    }

    pub fn reload_document(&mut self, index: usize) {
        if self.dirty_close(index) { return; }
        if let Body::Document(Some(current)) = &self.windows[index].body { self.request_source(index, if current.read_only { Request::Recovery(current.document.path.clone()) } else { Request::Load(current.document.path.clone()) }); }
    }

    pub fn discard_document(&mut self, tui: &mut Tui, index: usize) {
        let Body::Document(Some(current)) = &self.windows[index].body else { return };
        if current.read_only { self.close_window(tui, index); return; }
        if current.dirty && !current.discard_confirmation { self.dirty_close(index); return; }
        let directory = parent(&current.document.path);
        self.cancel_source(); self.set_body(tui, index, Kind::Files, None);
        self.request_source(index, Request::List(directory));
    }

    fn source_stage(&self, stage: &Stage) -> String {
        let text = self.text();
        match stage { Stage::Listing => text.source_listing, Stage::Reading => text.source_reading, Stage::Staging => text.source_staging, Stage::Checking => text.source_checking, Stage::Publishing => text.source_publishing, Stage::Complete => text.source_complete }.as_str().into()
    }

    fn source_error(&self, error: &Error) -> String {
        let text = self.text();
        if let Some(path) = &error.recovery_path { return (if error.kind == ErrorKind::Conflict { text.source_publication_race } else { text.source_partial_publication }).fill(&[("path", path)]).into_string(); }
        if error.kind == ErrorKind::TooLarge && error.path.is_empty() { return text.source_history_full.as_str().into(); }
        let label = match &error.kind { ErrorKind::InvalidPath => text.source_path_error, ErrorKind::Boundary => text.source_boundary_error, ErrorKind::NotFile => text.source_not_file, ErrorKind::TooLarge => text.source_too_large, ErrorKind::TooManyEntries => text.source_too_many, ErrorKind::NotText => text.source_not_text, ErrorKind::Conflict => text.source_conflict, ErrorKind::Cancelled => text.source_cancelled, ErrorKind::Busy => text.source_locked, ErrorKind::Io => text.source_io_error, ErrorKind::Unsupported => text.source_unsupported };
        format!("{}: {}", label.as_str(), error.path)
    }

    pub fn refresh_documents(&mut self, tui: &mut Tui) {
        for index in 0..self.windows.len() {
            let caption = match &self.windows[index].body {
                Body::Files(files) => format!("{} · {}: {}", if files.directory.is_empty() { "." } else { &files.directory }, self.text().source_filter.as_str(), files.query),
                Body::Document(Some(current)) => {
                    let position = match &tui.scene.node(self.windows[index].list).content { semio_framework_ui_tui_frame::scene::NodeContent::Widget(WidgetState::Editor(editor)) => { let (line, column) = editor.position(); self.text().source_position.fill(&[("line", &line.to_string()), ("column", &column.to_string())]).into_string() }, _ => String::new() };
                    format!("{} · {} · {position}", current.document.path, if current.read_only { self.text().source_read_only.as_str() } else if current.dirty { self.text().source_dirty.as_str() } else { self.text().source_saved.as_str() })
                }
                _ => continue,
            };
            if let Some(node) = self.windows[index].caption { if let Some(WidgetState::Label(label)) = tui.scene.node_mut(node).widget() { if label.text != caption { label.text = caption; } } }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_ui_tui_core::{event::mods, geometry::Size, theme::Theme};
    use semio_framework_ui_tui_frame::scene::NodeContent;
    use std::time::{Duration, Instant};
    use ui_styling::appearance::AppearanceName;

    fn view() -> (Dashboard, Tui, std::path::PathBuf) {
        static SERIAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let artifacts = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("caller-owned source editor artifacts required");
        let root = std::path::PathBuf::from(artifacts).join(format!("doc{}.{}", std::process::id(), SERIAL.fetch_add(1, Ordering::Relaxed)));
        assert!(root.to_string_lossy().encode_utf16().count() < 220);
        std::fs::create_dir_all(&root).unwrap();
        let mut tui = Tui::new(Size { width: 120, height: 34 }, Theme::new(AppearanceName::Dark));
        let dashboard = Dashboard::new(&mut tui, root.clone(), dashboard_preferences::Preferences { language: "en".into(), ..Default::default() }, root.join("prefs.jsonl"), false);
        (dashboard, tui, root)
    }

    fn settle(dashboard: &mut Dashboard, tui: &mut Tui) {
        let until = Instant::now() + Duration::from_secs(10);
        while dashboard.source_job.is_some() {
            dashboard.poll_documents(tui); dashboard.frame(tui);
            assert!(Instant::now() < until, "source worker did not settle: {:?}", dashboard.notice);
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    fn type_key(dashboard: &mut Dashboard, tui: &mut Tui, key: Key, mods: u8) { dashboard.handle(tui, &Event::Key(KeyEvent { key, mods })); dashboard.frame(tui); }

    #[test]
    fn source_ui_browses_saves_conflicts_and_requires_explicit_discard() {
        let (mut dashboard, mut tui, root) = view();
        std::fs::create_dir(root.join("src")).unwrap(); std::fs::write(root.join("src/a 🧪.txt"), "base\n").unwrap();
        dashboard.open_files(&mut tui); settle(&mut dashboard, &mut tui);
        let index = dashboard.focused_index(&tui);
        for c in "src".chars() { type_key(&mut dashboard, &mut tui, Key::Char(c), 0); }
        type_key(&mut dashboard, &mut tui, Key::Enter, 0); settle(&mut dashboard, &mut tui);
        for c in "a 🧪".chars() { type_key(&mut dashboard, &mut tui, Key::Char(c), 0); }
        type_key(&mut dashboard, &mut tui, Key::Enter, 0); settle(&mut dashboard, &mut tui);
        assert!(matches!(dashboard.windows[index].body, Body::Document(Some(_))));
        type_key(&mut dashboard, &mut tui, Key::End, mods::CTRL);
        dashboard.handle(&mut tui, &Event::Paste("中é👩‍💻".into()));
        assert!(matches!(&dashboard.windows[index].body, Body::Document(Some(current)) if current.dirty));
        dashboard.save_document(&tui, index); settle(&mut dashboard, &mut tui);
        assert_eq!(std::fs::read_to_string(root.join("src/a 🧪.txt")).unwrap(), "base\n中é👩‍💻");
        std::fs::write(root.join("src/a 🧪.txt"), "foreign\n").unwrap();
        type_key(&mut dashboard, &mut tui, Key::Char('a'), mods::CTRL);
        dashboard.handle(&mut tui, &Event::Paste("local\n".into()));
        dashboard.save_document(&tui, index); settle(&mut dashboard, &mut tui);
        assert_eq!(std::fs::read_to_string(root.join("src/a 🧪.txt")).unwrap(), "foreign\n");
        assert!(dashboard.notice.as_deref().unwrap().contains("changed elsewhere"));
        let count = dashboard.windows.len(); assert!(!dashboard.close_window(&mut tui, index)); assert_eq!(dashboard.windows.len(), count);
        dashboard.discard_document(&mut tui, index); settle(&mut dashboard, &mut tui);
        assert!(matches!(dashboard.windows[index].body, Body::Files(_)));
        std::fs::remove_dir_all(root).unwrap();
        println!("[DEBUG] native source UI browsed actual workspace, saved UTF8 and preserved foreign conflict with explicit discard");
    }

    #[test]
    fn source_worker_disconnect_retires_without_losing_a_document() {
        let (mut dashboard, mut tui, root) = view();
        let (send, receive) = mpsc::sync_channel(1); drop(send);
        dashboard.source_operation = 1;
        dashboard.source_job = Some(Job { window: dashboard.windows[0].id.clone(), operation: 1, generation: 0, cancelled: Arc::new(AtomicBool::new(false)), receive });
        assert!(dashboard.poll_documents(&mut tui)); assert!(dashboard.source_job.is_none());
        assert!(dashboard.notice.as_deref().unwrap().contains("ended unexpectedly"));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn source_committed_save_is_adopted_after_a_late_cancel() {
        let (mut dashboard, mut tui, root) = view(); std::fs::write(root.join("a.txt"), "old").unwrap();
        let workspace = Workspace::new(&root).unwrap(); let signal = AtomicBool::new(false);
        let before = workspace.load("a.txt", &signal, &mut |_| {}).unwrap();
        let committed = workspace.save("a.txt", &before.revision, "new", &signal, &mut |_| {}).unwrap();
        dashboard.set_body(&mut tui, 0, Kind::Document, None);
        dashboard.windows[0].body = Body::Document(Some(Box::new(OpenDocument { document: before, read_only: false, recovery: None, dirty: true, generation: 1, discard_confirmation: false })));
        if let Some(widget) = tui.scene.node_mut(dashboard.windows[0].list).widget() { *widget = WidgetState::Editor(EditorState::new("new", 1_048_576)); }
        let (send, receive) = mpsc::sync_channel(1); send.send(Message::Finished(Ok(Response::Save(committed)))).unwrap();
        dashboard.source_operation = 1;
        dashboard.source_job = Some(Job { window: dashboard.windows[0].id.clone(), operation: 1, generation: 1, cancelled: Arc::new(AtomicBool::new(true)), receive });
        assert!(dashboard.poll_documents(&mut tui));
        assert!(matches!(&dashboard.windows[0].body, Body::Document(Some(current)) if !current.dirty && current.document.text == "new"));
        assert!(matches!(&tui.scene.node(dashboard.windows[0].list).content, NodeContent::Widget(WidgetState::Editor(editor)) if editor.value == "new"));
        assert_eq!(dashboard.notice.as_deref(), Some("saved")); assert_eq!(std::fs::read_to_string(root.join("a.txt")).unwrap(), "new");
        drop(workspace);
        std::fs::remove_dir_all(root).unwrap();
    }
}
