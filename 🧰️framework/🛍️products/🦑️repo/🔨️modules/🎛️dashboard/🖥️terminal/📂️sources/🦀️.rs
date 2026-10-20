//! 📂️ Native workspace browsing and document requests kept outside the terminal event loop.

use super::windows::{Body, Dashboard, Kind, Place};
use semio_framework_filesystem_workspace::{Document, Entry, EntryRevision, Error, Progress, Workspace, Stage, ErrorKind};
#[path = "🎮️management/🦀️.rs"]
pub(super) mod management;
#[path="🕰️history/🦀️.rs"]
pub(super) mod history;
use semio_framework_filesystem_workspace::commands::{self, DocumentBuffer, EntryForm, HistoryForm, HistoryAction, Response, Message};
pub(super) use semio_framework_filesystem_workspace::commands::Request;
use management::EntryAction;
use semio_framework_ui_tui_core::event::{mods, Key, KeyEvent, Event};
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_frame::scene::NodeContent;
use semio_framework_ui_tui_widgets::widget::{EditorState, WidgetState};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver}};

const SOURCE_MESSAGE_LIMIT: usize = commands::MESSAGE_CAPACITY;

pub(super) struct Job { window: String, operation: u64, generation: u64, cancelled: Arc<AtomicBool>, receive: Receiver<Message>, protected: Vec<String> }
impl Drop for Job { fn drop(&mut self) { self.cancelled.store(true, Ordering::Release); } }

fn parent(path: &str) -> String { path.rsplit_once('/').map_or_else(String::new, |(parent, _)| parent.into()) }

impl Dashboard {
    pub fn open_files(&mut self, tui: &mut Tui) {
        let index = self.windows.iter().position(|window| matches!(window.body, Body::Files(_))).unwrap_or_else(|| self.create_window(tui, Kind::Files, Place::Preference));
        self.focus_window(tui, index);
        let request=match &self.windows[index].body{Body::Files(files)=>files.browse(files.directory.clone()),_=>Request::List(String::new())};
        self.request_source(index,request);
    }

    fn request_source(&mut self, index: usize, request: Request) {
        if let Some(dirty)=self.windows.iter().position(|window|matches!(&window.body,Body::Document(Some(document))if document.dirty&&request.management_paths().into_iter().flatten().any(|path|commands::path_contains(path,&document.document.path)))){self.dirty_close(dirty);return;}
        if let Some(job) = &self.source_job {
            job.cancelled.store(true, Ordering::Release);
            self.source_pending = Some((self.windows[index].id.clone(), request));
            self.notice = Some(self.text().source_busy.as_str().into());
            return;
        }
        let Some(operation) = self.source_operation.checked_add(1) else { return }; self.source_operation = operation;
        self.notice = Some(self.text().source_starting.fill(&[("cancel", &self.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "cancel-source").unwrap_or_default())]).into_string());
        let protected=request.management_paths().into_iter().flatten().map(str::to_owned).collect();
        let root = self.root.clone();
        let generation = match &self.windows[index].body { Body::Document(Some(document)) => document.generation, _ => 0 };
        let cancelled = Arc::new(AtomicBool::new(false)); let signal = cancelled.clone(); let waker = self.waker.clone();
        let wake=waker.map(|waker|Arc::new(move||{waker.wake();}) as Arc<dyn Fn()+Send+Sync>);
        let spawn=commands::spawn(root,request,signal,wake);
        self.adopt_source_worker(index,operation,generation,cancelled,protected,spawn);
    }

    fn adopt_source_worker(&mut self,index:usize,operation:u64,generation:u64,cancelled:Arc<AtomicBool>,protected:Vec<String>,result:std::io::Result<Receiver<Message>>){match result{Ok(receive)=>self.source_job=Some(Job{window:self.windows[index].id.clone(),operation,generation,cancelled,receive,protected}),Err(error)=>{self.unlock_management(index);self.notice=Some(self.text().source_failed.fill(&[("problem",&error.to_string())]).into_string());}}}

    pub fn cancel_source(&mut self) { self.source_pending = None; if let Some(job) = &self.source_job { job.cancelled.store(true, Ordering::Release); } self.notice = Some(self.text().source_document_cancelled.as_str().into()); }

    pub fn poll_documents(&mut self, tui: &mut Tui) -> bool {
        let mut changed = false;
        let mut remaining = SOURCE_MESSAGE_LIMIT;
        while remaining > 0 {
            let Some(job) = &self.source_job else { break };
            let message = match job.receive.try_recv() {
                Ok(message) => message,
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    let window=job.window.clone();self.source_job.take();if let Some(index)=self.window_index(&window){self.unlock_management(index);} self.notice = Some(self.text().source_worker_failed.as_str().into());
                    if let Some((window, request)) = self.source_pending.take() { if let Some(index) = self.window_index(&window) { self.request_source(index, request); } }
                    changed = true; break;
                }
            };
            remaining -= 1;
            let (window, operation, generation) = (job.window.clone(), job.operation, job.generation);
            let cancelled = job.cancelled.load(Ordering::Acquire);
            let protected=job.protected.clone();
            let mutation = !protected.is_empty();
            if cancelled && matches!(message, Message::Progress(_)) { continue; }
            let committed_save = message.authoritative();
            if matches!(message, Message::Finished(_)) && ((cancelled && !committed_save) || operation != self.source_operation || self.window_index(&window).is_none()) {
                self.source_job.take();if let Some(index)=self.window_index(&window){self.unlock_management(index);}
                if let Some((window, request)) = self.source_pending.take() { if let Some(index) = self.window_index(&window) { self.request_source(index, request); } }
                changed = true;
                break;
            }
            let Some(index) = self.window_index(&window) else { continue };
            changed = true;
            match message {
                Message::Progress(progress)=>self.notice=Some(self.text().source_progress(&progress,&self.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix,"cancel-source").unwrap_or_default())),
                Message::Finished(result) => {
                    self.source_job.take();
                    match result {
                        Ok(Response::Inspected(path,kind,revision)) => self.show_management_form(tui,index,kind,path,Some(revision)),
                        Ok(Response::CreatedFile(document)) => { self.finish_management_form(tui,index); self.adopt_created_document(tui,index,document); }
                        Ok(Response::CreatedDirectory(entry)) => self.management_completed(tui,index,&entry.path,None),
                        Ok(Response::Renamed(previous,entry,document)) => { self.reconcile_renamed_documents(tui,&previous,&entry.path,document); self.management_completed(tui,index,&entry.path,None); }
                        Ok(Response::Retained(retained)) => { self.reconcile_retained_documents(tui,&retained.path); self.management_completed(tui,index,&retained.path,Some(retained.recovery_path)); }
                        Ok(response @ (Response::HistoryPage(_) | Response::HistoryInspected(..) | Response::HistoryChanged(..)))=>self.adopt_history_result(tui,index,response),
                        Ok(Response::RecoveryList(directory,entries))=>{if self.dirty_close(index){continue;}if !matches!(self.windows[index].body,Body::Files(_)){self.set_body(tui,index,Kind::Files,None);}if let Body::Files(files)=&mut self.windows[index].body{files.adopt_recovery_list(directory,entries);}self.paint_files(tui,index);self.notice=Some(self.text().source_read_only.as_str().into());}
                        Ok(Response::List(directory,entries))=>{if self.dirty_close(index){continue;}if !matches!(self.windows[index].body,Body::Files(_)){self.set_body(tui,index,Kind::Files,None);}if let Body::Files(files)=&mut self.windows[index].body{files.adopt_list(directory,entries);}self.paint_files(tui,index);self.notice=match &self.windows[index].body{Body::Files(files)=>files.recovery.as_ref().map(|path|self.text().source_retained.fill(&[("path",path),("inspect",&self.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix,"inspect-recovery").unwrap_or_default())]).into_string()),_=>None};}
                        Ok(response @ (Response::Load(_) | Response::Recovery(_))) => {
                            let (document, read_only) = match response { Response::Load(document) => (document, false), Response::Recovery(document) => (document, true), _ => unreachable!() };
                            if matches!(&self.windows[index].body, Body::Document(Some(current)) if current.generation != generation || current.dirty) { continue; }
                            self.set_body(tui, index, Kind::Document, None);
                            let mut editor = EditorState::new(document.text.clone(), 1_048_576); editor.cursor = 0; editor.read_only = read_only;
                            if let Some(widget) = tui.scene.node_mut(self.windows[index].list).widget() { *widget = WidgetState::Editor(editor); }
                            self.windows[index].body = Body::Document(Some(Box::new(DocumentBuffer::new(document,read_only))));
                            self.notice = Some(self.text().source_reloaded.as_str().into());
                        }
                        Ok(Response::Save(document)) => {
                            let node = self.windows[index].list;
                            if let (Body::Document(Some(current)),NodeContent::Widget(WidgetState::Editor(editor)))=(&mut self.windows[index].body,&tui.scene.node(node).content){current.saved(document,&editor.value);}
                            self.notice = Some(self.text().source_saved.as_str().into());
                        }
                        Err(error) => {
                            if mutation{self.mark_uncertain_documents(tui,&protected,error.recovery_path.as_deref());}
                            self.unlock_management(index);if let Body::Files(files)=&mut self.windows[index].body { if error.recovery_path.is_some(){files.recovery=error.recovery_path.clone();} }
                            if let Body::Document(Some(current)) = &mut self.windows[index].body { current.record_failure(&error); }
                            self.notice = Some(self.text().source_failure(&error,mutation));
                        }
                    }
                    self.remount(tui);
                    if let Some((window, request)) = self.source_pending.take() { if let Some(index) = self.window_index(&window) { self.request_source(index, request); } }
                    break;
                }
            }
        }
        if remaining == 0 && self.source_job.is_some() { if let Some(waker) = &self.waker { waker.wake(); } }
        changed
    }

    pub fn paint_files(&mut self, tui: &mut Tui, index: usize) {
        let node = self.windows[index].list;
        let Body::Files(files) = &mut self.windows[index].body else { return };
        if files.form.is_some() || files.history_form.is_some() { return; }
        if let Some(widget)=tui.scene.node_mut(node).widget() { if !matches!(widget,WidgetState::List(_)){*widget=WidgetState::List(semio_framework_ui_tui_widgets::widget::ListState::new(Vec::new()));} }
        let items=files.rows();
        if let Some(WidgetState::List(list)) = tui.scene.node_mut(node).widget() { list.selected = list.selected.min(items.len().saturating_sub(1)); list.marks = vec![false; items.len()]; list.items = items; }
    }

    pub fn files_key(&mut self, tui: &mut Tui, index: usize, key: &KeyEvent) {
        if self.history_key(tui,index,key)||self.management_key(tui,index,key) { return; }
        let mut request = None;
        if let Body::Files(files) = &mut self.windows[index].body {
            match key.key {
                Key::Char(c) if key.mods & (mods::CTRL | mods::ALT) == 0 && !c.is_control() && files.query.len() + c.len_utf8() <= 256 => files.query.push(c),
                Key::Backspace if !files.query.is_empty() => { files.query.truncate(semio_framework_ui_tui_core::text::previous_boundary(&files.query, files.query.len())); }
                Key::Backspace | Key::Esc => request = Some(files.browse_parent()),
                _ => { let signals = tui.dispatch(&Event::Key(*key)); self.on_signals(tui, signals); return; }
            }
        }
        if let Some(request) = request { self.request_source(index,request); } else { self.paint_files(tui, index); }
    }

    pub fn activate_file(&mut self,_tui:&Tui,index:usize,selected:usize){let request=match &self.windows[index].body{Body::Files(files)=>files.activate(selected),_=>None};if let Some(request)=request{self.request_source(index,request);}}

    pub fn save_document(&mut self, tui: &Tui, index: usize) {
        if self.management_blocks_edit(index){self.notice=Some(self.text().source_busy.as_str().into());return;}
        let Body::Document(Some(current)) = &self.windows[index].body else { return };
        if current.read_only { self.notice = Some(self.text().source_read_only.as_str().into()); return; }
        let NodeContent::Widget(WidgetState::Editor(editor)) = &tui.scene.node(self.windows[index].list).content else { return };
        let request = current.save_request(editor.value.clone());
        self.request_source(index, request);
    }

    pub fn inspect_recovery(&mut self,tui:&mut Tui,index:usize){let path=match &self.windows[index].body{Body::Document(Some(document))=>document.recovery.clone(),Body::Files(files)=>Some(files.recovery.clone().unwrap_or_default()),_=>None};let Some(path)=path else{return;};let request=if path.is_empty(){Request::HistoryPage(None)}else{Request::Recovery(path)};let target=self.create_window(tui,Kind::Files,Place::Preference);self.focus_window(tui,target);self.request_source(target,request);}

    pub fn dirty_close(&mut self, index: usize) -> bool {
        if matches!(&self.windows[index].body,Body::Files(files)if files.history_form.as_ref().is_some_and(|form|form.busy))||self.management_blocks_edit(index)||self.source_job.as_ref().is_some_and(|job|job.window==self.windows[index].id&&!job.protected.is_empty())||self.source_pending.as_ref().is_some_and(|(window,request)|window==&self.windows[index].id&&request.management_paths().into_iter().any(|path|path.is_some())){self.notice=Some(self.text().source_busy.as_str().into());return true;}
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
        if let Body::Document(Some(current)) = &self.windows[index].body { self.request_source(index, current.reload_request()); }
    }

    pub fn discard_document(&mut self, tui: &mut Tui, index: usize) {
        let Body::Document(Some(current)) = &self.windows[index].body else { return };
        if current.read_only { self.close_window(tui, index); return; }
        if current.dirty && !current.discard_confirmation { self.dirty_close(index); return; }
        let directory = parent(&current.document.path);
        self.cancel_source(); self.set_body(tui, index, Kind::Files, None);
        self.request_source(index, Request::List(directory));
    }

    pub fn refresh_documents(&mut self, tui: &mut Tui) {
        for index in 0..self.windows.len() {
            let caption = match &self.windows[index].body {
                Body::Files(files) => self.text().source_browser_caption(files),
                Body::Document(Some(current)) => {
                    let position = match &tui.scene.node(self.windows[index].list).content { NodeContent::Widget(WidgetState::Editor(editor)) => { let (line, column) = editor.position(); self.text().source_position.fill(&[("line", &line.to_string()), ("column", &column.to_string())]).into_string() }, _ => String::new() };
                    self.text().source_document_caption(current,&position)
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
        let dashboard = Dashboard::new(&mut tui, root.clone(), dashboard_preferences::Preferences { language: Some("en".into()), ..Default::default() }, root.join("prefs.jsonl"), false);
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
        dashboard.source_job = Some(Job { window: dashboard.windows[0].id.clone(), operation: 1, generation: 0, cancelled: Arc::new(AtomicBool::new(false)), receive, protected:Vec::new() });
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
        dashboard.windows[0].body = Body::Document(Some(Box::new(DocumentBuffer { document: before, read_only: false, recovery: None, dirty: true, generation: 1, discard_confirmation: false })));
        if let Some(widget) = tui.scene.node_mut(dashboard.windows[0].list).widget() { *widget = WidgetState::Editor(EditorState::new("new", 1_048_576)); }
        let (send, receive) = mpsc::sync_channel(1); send.send(Message::Finished(Ok(Response::Save(committed)))).unwrap();
        dashboard.source_operation = 1;
        dashboard.source_job = Some(Job { window: dashboard.windows[0].id.clone(), operation: 1, generation: 1, cancelled: Arc::new(AtomicBool::new(true)), receive, protected:Vec::new() });
        assert!(dashboard.poll_documents(&mut tui));
        assert!(matches!(&dashboard.windows[0].body, Body::Document(Some(current)) if !current.dirty && current.document.text == "new"));
        assert!(matches!(&tui.scene.node(dashboard.windows[0].list).content, NodeContent::Widget(WidgetState::Editor(editor)) if editor.value == "new"));
        assert_eq!(dashboard.notice.as_deref(), Some("saved")); assert_eq!(std::fs::read_to_string(root.join("a.txt")).unwrap(), "new");
        drop(workspace);
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn source_worker_yields_at_each_shared_message_quantum_without_losing_settlement() {
        let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../📂️sources/🧪️tests/🔣️.json")).unwrap();
        assert_eq!(SOURCE_MESSAGE_LIMIT, fixture["limit"].as_u64().unwrap() as usize);
        for row in fixture["cases"].as_array().unwrap() {
            let (mut dashboard, mut tui, root) = view();
            let index = dashboard.focused_index(&tui);
            dashboard.set_body(&mut tui, index, Kind::Files, None);
            let progress = row["progress"].as_u64().unwrap();
            let cancelled = Arc::new(AtomicBool::new(row["cancelled"].as_bool().unwrap()));
            let (send, receive) = mpsc::channel();
            for completed in 0..progress { send.send(Message::Progress(Progress { stage: Stage::Reading, completed, total: Some(progress) })).unwrap(); }
            send.send(Message::Finished(Ok(Response::List(String::new(), Vec::new())))).unwrap();
            dashboard.source_operation = 1;
            dashboard.source_job = Some(Job { window: dashboard.windows[index].id.clone(), operation: 1, generation: 0, cancelled, receive, protected:Vec::new() });
            let expected = row["polls"].as_u64().unwrap();
            let mut polls = 0;
            while dashboard.source_job.is_some() {
                polls += 1;
                assert!(polls <= expected, "{} exceeded finite poll plan", row["name"]);
                dashboard.poll_documents(&mut tui);
                dashboard.frame(&mut tui);
            }
            assert_eq!(polls, expected, "{} yielded at the wrong message boundary", row["name"]);
            std::fs::remove_dir_all(root).unwrap();
        }
        println!("[DEBUG] source worker preserves all ten shared terminal and cancellation cuts within a finite message quantum");
    }

    fn submit(dashboard:&mut Dashboard,tui:&mut Tui,value:&str){for c in value.chars(){type_key(dashboard,tui,Key::Char(c),0);}type_key(dashboard,tui,Key::Enter,0);settle(dashboard,tui);}

    #[test]
    fn source_management_creates_renames_and_retains_with_guided_confirmation() {
        let (mut dashboard,mut tui,root)=view();dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let index=dashboard.focused_index(&tui);
        dashboard.begin_management(&mut tui,index,EntryAction::CreateFile);submit(&mut dashboard,&mut tui,"new 🧪.txt");assert_eq!(std::fs::read(root.join("new 🧪.txt")).unwrap(),b"");
        dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let index=dashboard.focused_index(&tui);dashboard.begin_management(&mut tui,index,EntryAction::Rename);settle(&mut dashboard,&mut tui);type_key(&mut dashboard,&mut tui,Key::Char('u'),mods::CTRL);submit(&mut dashboard,&mut tui,"renamed 🦑.txt");assert!(!root.join("new 🧪.txt").exists());assert!(root.join("renamed 🦑.txt").exists());
        dashboard.begin_management(&mut tui,index,EntryAction::Retain);settle(&mut dashboard,&mut tui);type_key(&mut dashboard,&mut tui,Key::Enter,0);assert!(root.join("renamed 🦑.txt").exists());submit(&mut dashboard,&mut tui,"renamed 🦑.txt");assert!(!root.join("renamed 🦑.txt").exists());assert!(dashboard.notice.as_ref().unwrap().contains(".workspace-events"));
        println!("[DEBUG] guided file commands settled");
    }

    #[test]
    fn source_management_blocks_dirty_documents_and_supports_nonempty_directories() {
        let (mut dashboard,mut tui,root)=view();std::fs::write(root.join("a.txt"),"original").unwrap();std::fs::create_dir(root.join("folder")).unwrap();std::fs::write(root.join("folder/child"),"foreign").unwrap();dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let files=dashboard.focused_index(&tui);
        dashboard.request_source(files,Request::Load("a.txt".into()));settle(&mut dashboard,&mut tui);type_key(&mut dashboard,&mut tui,Key::Char('x'),0);dashboard.begin_management(&mut tui,files,EntryAction::Rename);assert!(dashboard.source_job.is_none());assert_eq!(std::fs::read_to_string(root.join("a.txt")).unwrap(),"original");
        dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let index=dashboard.focused_index(&tui);dashboard.begin_management(&mut tui,index,EntryAction::Retain);settle(&mut dashboard,&mut tui);submit(&mut dashboard,&mut tui,"folder");assert!(!root.join("folder").exists());let recovery=match &dashboard.windows[index].body{Body::Files(files)=>files.recovery.clone().unwrap(),_=>panic!("retained tree outcome")};assert_eq!(std::fs::read_to_string(std::path::Path::new(&recovery).join("child")).unwrap(),"foreign");
    }

    #[test]
    fn source_management_adopts_committed_rename_after_cancellation_and_blocks_edits_until_settled() {
        let(mut dashboard,mut tui,root)=view();std::fs::write(root.join("a.txt"),"original").unwrap();dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let document_window=dashboard.focused_index(&tui);dashboard.request_source(document_window,Request::Load("a.txt".into()));settle(&mut dashboard,&mut tui);dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let files_window=dashboard.focused_index(&tui);
        let(send,receive)=mpsc::sync_channel(2);dashboard.source_operation=10;dashboard.source_job=Some(Job{window:dashboard.windows[files_window].id.clone(),operation:10,generation:0,cancelled:Arc::new(AtomicBool::new(true)),receive,protected:vec!["a.txt".into(),"b.txt".into()]});assert!(dashboard.management_blocks_edit(document_window));assert!(dashboard.dirty_close(files_window));std::fs::rename(root.join("a.txt"),root.join("b.txt")).unwrap();let document=Workspace::new(&root).unwrap().load("b.txt",&AtomicBool::new(false),&mut |_|{}).unwrap();send.send(Message::Finished(Ok(Response::Renamed("a.txt".into(),Entry{path:"b.txt".into(),name:"b.txt".into(),directory:false},Some(document))))).unwrap();dashboard.poll_documents(&mut tui);settle(&mut dashboard,&mut tui);
        assert!(matches!(&dashboard.windows[document_window].body,Body::Document(Some(current))if current.document.path=="b.txt"&&!current.read_only&&!current.dirty));assert!(!dashboard.management_blocks_edit(document_window));
        let cases:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../../../🔨️modules/📁️filesystem/📝️workspace/📬️commands/📝️buffer/🧫️fixtures/🔣️.json")).unwrap();for row in cases["cases"].as_array().unwrap().iter().filter(|row|row["path"]=="b.txt"){if let Body::Document(Some(current))=&mut dashboard.windows[document_window].body{current.read_only=false;current.recovery=None;}let recovery=row["recovery"].as_str().map(str::to_owned);let(send,receive)=mpsc::sync_channel(1);dashboard.source_operation=11;dashboard.source_job=Some(Job{window:dashboard.windows[files_window].id.clone(),operation:11,generation:0,cancelled:Arc::new(AtomicBool::new(recovery.is_some())),receive,protected:vec!["b.txt".into()]});send.send(Message::Finished(Err(Error{kind:ErrorKind::Io,path:"b.txt".into(),detail:"uncertain".into(),recovery_path:recovery.clone()}))).unwrap();dashboard.poll_documents(&mut tui);assert!(matches!(&dashboard.windows[document_window].body,Body::Document(Some(current))if current.document.path=="b.txt"&&current.document.text=="original"&&current.read_only==row["readOnly"].as_bool().unwrap()&&current.recovery==recovery));assert!(matches!(&tui.scene.node(dashboard.windows[document_window].list).content,NodeContent::Widget(WidgetState::Editor(editor))if editor.read_only==row["readOnly"].as_bool().unwrap()));if row["reloadRefused"].as_bool().unwrap(){std::fs::remove_file(root.join("b.txt")).unwrap();dashboard.reload_document(document_window);settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[document_window].body,Body::Document(Some(current))if current.recovery==recovery&&current.read_only));}}
        println!("[DEBUG] late rename and uncertain outcomes preserve protected buffers");
    }

    #[test]
    fn source_directory_commands_protect_descendants_and_browse_retained_history(){
        let(mut dashboard,mut tui,root)=view();std::fs::create_dir(root.join("folder")).unwrap();std::fs::write(root.join("folder/a.txt"),"kept").unwrap();dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let files=dashboard.focused_index(&tui);let document=dashboard.create_window(&mut tui,Kind::Document,Place::Preference);dashboard.request_source(document,Request::Load("folder/a.txt".into()));settle(&mut dashboard,&mut tui);
        if let Body::Document(Some(current))=&mut dashboard.windows[document].body{current.dirty=true;}let revision=Workspace::new(&root).unwrap().inspect_entry("folder",&AtomicBool::new(false),&mut |_|{}).unwrap();dashboard.request_source(files,Request::Rename{path:"folder".into(),revision:revision.clone(),destination:"moved".into()});assert!(dashboard.source_job.is_none());if let Body::Document(Some(current))=&mut dashboard.windows[document].body{current.dirty=false;}
        dashboard.source_pending=Some((dashboard.windows[files].id.clone(),Request::Rename{path:"folder".into(),revision:revision.clone(),destination:"moved".into()}));assert!(dashboard.management_blocks_edit(document));dashboard.source_pending=None;dashboard.request_source(files,Request::Rename{path:"folder".into(),revision,destination:"moved".into()});assert!(dashboard.management_blocks_edit(document));settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[document].body,Body::Document(Some(current))if current.document.path=="moved/a.txt"&&current.read_only));dashboard.reload_document(document);settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[document].body,Body::Document(Some(current))if !current.read_only&&current.document.text=="kept"));
        let revision=Workspace::new(&root).unwrap().inspect_entry("moved",&AtomicBool::new(false),&mut |_|{}).unwrap();dashboard.request_source(files,Request::Retain{path:"moved".into(),revision});settle(&mut dashboard,&mut tui);assert!(matches!(dashboard.windows[document].body,Body::Files(_)));dashboard.inspect_recovery(&mut tui,files);settle(&mut dashboard,&mut tui);let history=dashboard.focused_index(&tui);assert!(matches!(&dashboard.windows[history].body,Body::Files(files)if files.history&&files.entries.len()==1));let retained=match &dashboard.windows[history].body{Body::Files(files)=>files.entries[0].path.clone(),_=>unreachable!()};dashboard.begin_history(&mut tui,history,Some(HistoryAction::Archive));settle(&mut dashboard,&mut tui);dashboard.history_key(&mut tui,history,&KeyEvent{key:Key::Enter,mods:0});assert!(dashboard.source_job.is_none());let selection=match &dashboard.windows[history].body{Body::Files(files)=>files.history_form.as_ref().unwrap().confirmation().unwrap().to_owned(),_=>unreachable!()};dashboard.history_paste(&mut tui,history,&selection);dashboard.history_key(&mut tui,history,&KeyEvent{key:Key::Esc,mods:0});assert!(matches!(&dashboard.windows[history].body,Body::Files(files)if files.history_form.is_none()));dashboard.begin_history(&mut tui,history,Some(HistoryAction::Archive));settle(&mut dashboard,&mut tui);dashboard.history_paste(&mut tui,history,&selection);dashboard.history_key(&mut tui,history,&KeyEvent{key:Key::Enter,mods:0});settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[history].body,Body::Files(files)if files.history&&files.entries[0].path==retained&&files.inspection.as_ref().is_some_and(|value|value.active!=selection)));dashboard.next_history_page(history);assert!(dashboard.source_job.is_none());dashboard.activate_file(&tui,history,0);settle(&mut dashboard,&mut tui);dashboard.activate_file(&tui,history,1);settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[history].body,Body::Document(Some(current))if current.read_only&&current.document.text=="kept"));std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn source_worker_spawn_refusal_unlocks_and_retains_the_guided_form(){let(mut dashboard,mut tui,root)=view();dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let index=dashboard.focused_index(&tui);dashboard.show_management_form(&mut tui,index,EntryAction::CreateFile,String::new(),None);dashboard.management_paste(&mut tui,index,"keep.txt");if let Body::Files(files)=&mut dashboard.windows[index].body{files.form.as_mut().unwrap().busy=true;}dashboard.management_key(&mut tui,index,&KeyEvent{key:Key::Esc,mods:0});assert!(matches!(&dashboard.windows[index].body,Body::Files(files)if files.form.as_ref().unwrap().busy));dashboard.adopt_source_worker(index,10,0,Arc::new(AtomicBool::new(false)),vec!["keep.txt".into()],Err(std::io::Error::from(std::io::ErrorKind::WouldBlock)));assert!(dashboard.source_job.is_none());assert!(matches!(&dashboard.windows[index].body,Body::Files(files)if !files.form.as_ref().unwrap().busy));assert!(matches!(&tui.scene.node(dashboard.windows[index].list).content,NodeContent::Widget(WidgetState::Input(input))if input.value=="keep.txt"));std::fs::remove_dir_all(root).unwrap();}

}
