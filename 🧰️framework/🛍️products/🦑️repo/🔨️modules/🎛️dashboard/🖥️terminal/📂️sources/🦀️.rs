//! 📂️ Binds app windows to the canonical workspace interaction owner.
use super::windows::{Body,Dashboard,DashboardWindow,Kind,Place};
use dashboard_source_view::{Context,SourceWindowPort,WindowRef,WindowMut,Body as ViewBody,OwnedBody,Effect as SourceEffect,Request,EntryAction};
use semio_framework_filesystem_workspace::{Entry,Error,Progress,Workspace,Stage,ErrorKind};
use semio_framework_filesystem_workspace::commands::{self,DocumentBuffer,HistoryAction,Response,Message};
use semio_framework_ui_tui_core::event::{KeyEvent,Key,Event};
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_frame::scene::{Node,NodeContent};
use semio_framework_ui_tui_widgets::widget::{WidgetState,EditorState};
use std::sync::{Arc,atomic::{AtomicBool,Ordering},mpsc};

struct Windows<'a>{windows:&'a mut [DashboardWindow],input_mode:&'a mut bool}
fn read(window:&DashboardWindow)->WindowRef<'_>{WindowRef{id:&window.id,list:window.list,caption:window.caption,body:match &window.body{Body::Files(value)=>ViewBody::Files(value),Body::Document(value)=>ViewBody::Document(value.as_deref()),_=>ViewBody::Other}}}
fn write(window:&mut DashboardWindow)->WindowMut<'_>{WindowMut{id:&window.id,list:window.list,body:match &mut window.body{Body::Files(value)=>ViewBody::Files(value),Body::Document(value)=>ViewBody::Document(value.as_deref_mut()),_=>ViewBody::Other}}}
impl SourceWindowPort for Windows<'_>{
    type Windows<'a>=std::iter::Map<std::slice::Iter<'a,DashboardWindow>,fn(&'a DashboardWindow)->WindowRef<'a>> where Self:'a;
    type WindowsMut<'a>=std::iter::Map<std::slice::IterMut<'a,DashboardWindow>,fn(&'a mut DashboardWindow)->WindowMut<'a>> where Self:'a;
    fn windows(&self)->Self::Windows<'_>{self.windows.iter().map(read)}
    fn windows_mut(&mut self)->Self::WindowsMut<'_>{self.windows.iter_mut().map(write)}
    fn window(&self,index:usize)->WindowRef<'_>{read(&self.windows[index])}
    fn window_mut(&mut self,index:usize)->WindowMut<'_>{write(&mut self.windows[index])}
    fn len(&self)->usize{self.windows.len()}
    fn set_model(&mut self,index:usize,body:OwnedBody){self.windows[index].body=match body{OwnedBody::Files(value)=>Body::Files(value),OwnedBody::Document(value)=>Body::Document(value)};}
    fn replace(&mut self,tui:&mut Tui,index:usize,kind:dashboard_source_view::Kind){
        let window=&mut self.windows[index];
        let mut focus=tui.focus();let mut focused=false;
        while let Some(node)=focus{if node==window.chrome{focused=true;break;}focus=tui.scene.try_node(node).and_then(|node|node.parent());}
        tui.scene.remove(window.root);
        let root=tui.scene.add(window.chrome,Node::new(NodeContent::Box));
        tui.scene.node_mut(root).set_constraint(super::windows::node_constraint());
        let(list,caption)=dashboard_source_view::mount(tui,root,kind);
        window.root=root;window.list=list;window.caption=Some(caption);window.input=None;window.tree=None;window.editor_slot=None;window.pane=Default::default();
        window.body=match kind{dashboard_source_view::Kind::Files=>Body::Files(Default::default()),dashboard_source_view::Kind::Document=>Body::Document(None)};
        *self.input_mode=false;
        if focused||tui.focus().is_none_or(|node|!tui.scene.contains(node)){tui.set_focus(Some(list));}
    }
}
impl Dashboard{
    fn with_sources<R>(&mut self,action:impl FnOnce(&mut dashboard_source_view::View<'_,Windows<'_>>)->R)->R{
        let context=Context{root:&self.root,keymap:&self.keymap,locale:self.locale,waker:self.waker.clone()};
        let mut port=Windows{windows:&mut self.windows,input_mode:&mut self.input_mode};
        let(result,effects)=self.source.interact(&mut port,context,action);
        self.source_effects.extend(effects);self.apply_local_source_effects();result
    }
    fn apply_local_source_effects(&mut self){
        while matches!(self.source_effects.first(),Some(SourceEffect::Notice(_)|SourceEffect::Failure(_)|SourceEffect::Request(..))){
            match self.source_effects.remove(0){
                SourceEffect::Notice(value)=>self.notice=value,
                SourceEffect::Failure(value)=>self.record_failure(value),
                SourceEffect::Request(window,request)=>if let Some(index)=self.window_index(&window){self.request_source(index,request);},
                _=>unreachable!()
            }
        }
    }
    pub fn flush_source_effects(&mut self,tui:&mut Tui){
        while !self.source_effects.is_empty(){
            match self.source_effects.remove(0){
                SourceEffect::Notice(value)=>self.notice=value,
                SourceEffect::Failure(value)=>self.record_failure(value),
                SourceEffect::Focus(window)=>if let Some(index)=self.window_index(&window){self.focus_window(tui,index);},
                SourceEffect::Close(window)=>if let Some(index)=self.window_index(&window){self.close_window(tui,index);},
                SourceEffect::Remount=>self.remount(tui),
                SourceEffect::Signals(signals)=>self.on_signals(tui,signals),
                SourceEffect::Request(window,request)=>if let Some(index)=self.window_index(&window){self.request_source(index,request);}
            }
        }
    }
    pub fn open_files(&mut self,tui:&mut Tui){
        let index=self.windows.iter().position(|window|matches!(window.body,Body::Files(_))).unwrap_or_else(||self.create_window(tui,Kind::Files,Place::Preference));
        self.focus_window(tui,index);
        let request=match &self.windows[index].body{Body::Files(files)=>files.browse(files.directory.clone()),_=>Request::List(String::new())};
        self.request_source(index,request);
    }
    fn request_source(&mut self,index:usize,request:Request){self.with_sources(|view|view.request_source(index,request));}
    pub fn cancel_source(&mut self){self.with_sources(|view|view.cancel_source());}
    pub fn poll_documents(&mut self,tui:&mut Tui)->bool{let value=self.with_sources(|view|view.poll_documents(tui));self.flush_source_effects(tui);value}
    pub fn paint_files(&mut self,tui:&mut Tui,index:usize){self.with_sources(|view|view.paint_files(tui,index));self.flush_source_effects(tui);}
    pub fn files_key(&mut self,tui:&mut Tui,index:usize,key:&KeyEvent){self.with_sources(|view|view.files_key(tui,index,key));self.flush_source_effects(tui);}
    pub fn activate_file(&mut self,tui:&Tui,index:usize,selected:usize){self.with_sources(|view|view.activate_file(tui,index,selected));}
    pub fn save_document(&mut self,tui:&Tui,index:usize){self.with_sources(|view|view.save_document(tui,index));}
    pub fn dirty_close(&mut self,index:usize)->bool{self.with_sources(|view|view.dirty_close(index))}
    pub fn reload_document(&mut self,index:usize){self.with_sources(|view|view.reload_document(index));}
    pub fn discard_document(&mut self,tui:&mut Tui,index:usize){self.with_sources(|view|view.discard_document(tui,index));self.flush_source_effects(tui);}
    pub fn refresh_documents(&mut self,tui:&mut Tui){self.with_sources(|view|view.refresh_documents(tui));self.flush_source_effects(tui);}
    pub fn begin_management(&mut self,tui:&mut Tui,index:usize,kind:EntryAction){self.with_sources(|view|view.begin_management(tui,index,kind));self.flush_source_effects(tui);}
    pub fn management_paste(&mut self,tui:&mut Tui,index:usize,text:&str)->bool{let value=self.with_sources(|view|view.management_paste(tui,index,text));self.flush_source_effects(tui);value}
    pub fn management_blocks_edit(&self,index:usize)->bool{match &self.windows[index].body{Body::Document(Some(document))=>self.source.blocks_path(&document.document.path),_=>false}}
    pub fn begin_history(&mut self,tui:&mut Tui,index:usize,action:Option<HistoryAction>){
        if self.source.active(){self.notice=Some(self.text().source_busy.as_str().into());return;}
        let index=if matches!(self.windows[index].body,Body::Files(_)){index}else{self.create_window(tui,Kind::Files,Place::Preference)};
        self.focus_window(tui,index);self.request_source(index,Request::InspectHistory(action));
    }
    pub fn next_history_page(&mut self,index:usize){self.with_sources(|view|view.next_history_page(index));}
    pub fn inspect_recovery(&mut self,tui:&mut Tui,index:usize){
        let path=match &self.windows[index].body{Body::Document(Some(document))=>document.recovery.clone(),Body::Files(files)=>Some(files.recovery.clone().unwrap_or_default()),_=>None};
        let Some(path)=path else{return;};let request=if path.is_empty(){Request::HistoryPage(None)}else{Request::Recovery(path)};
        let target=self.create_window(tui,Kind::Files,Place::Preference);self.focus_window(tui,target);self.request_source(target,request);
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
        while dashboard.source.active() {
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
        
        dashboard_source_view::testing::observe(&mut dashboard.source,dashboard_source_view::testing::Worker{ window: dashboard.windows[0].id.clone(), operation: 1, generation: 0, cancelled: Arc::new(AtomicBool::new(false)), receive, protected:Vec::new() });
        assert!(dashboard.poll_documents(&mut tui)); assert!(!dashboard.source.active());
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
        
        dashboard_source_view::testing::observe(&mut dashboard.source,dashboard_source_view::testing::Worker{ window: dashboard.windows[0].id.clone(), operation: 1, generation: 1, cancelled: Arc::new(AtomicBool::new(true)), receive, protected:Vec::new() });
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
        assert_eq!(commands::MESSAGE_CAPACITY, fixture["limit"].as_u64().unwrap() as usize);
        for row in fixture["cases"].as_array().unwrap() {
            let (mut dashboard, mut tui, root) = view();
            let index = dashboard.focused_index(&tui);
            dashboard.set_body(&mut tui, index, Kind::Files, None);
            let progress = row["progress"].as_u64().unwrap();
            let cancelled = Arc::new(AtomicBool::new(row["cancelled"].as_bool().unwrap()));
            let (send, receive) = mpsc::channel();
            for completed in 0..progress { send.send(Message::Progress(Progress { stage: Stage::Reading, completed, total: Some(progress) })).unwrap(); }
            send.send(Message::Finished(Ok(Response::List(String::new(), Vec::new())))).unwrap();
            
            dashboard_source_view::testing::observe(&mut dashboard.source,dashboard_source_view::testing::Worker{ window: dashboard.windows[index].id.clone(), operation: 1, generation: 0, cancelled, receive, protected:Vec::new() });
            let expected = row["polls"].as_u64().unwrap();
            let mut polls = 0;
            while dashboard.source.active() {
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
        dashboard.request_source(files,Request::Load("a.txt".into()));settle(&mut dashboard,&mut tui);type_key(&mut dashboard,&mut tui,Key::Char('x'),0);dashboard.begin_management(&mut tui,files,EntryAction::Rename);assert!(!dashboard.source.active());assert_eq!(std::fs::read_to_string(root.join("a.txt")).unwrap(),"original");
        dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let index=dashboard.focused_index(&tui);dashboard.begin_management(&mut tui,index,EntryAction::Retain);settle(&mut dashboard,&mut tui);submit(&mut dashboard,&mut tui,"folder");assert!(!root.join("folder").exists());let recovery=match &dashboard.windows[index].body{Body::Files(files)=>files.recovery.clone().unwrap(),_=>panic!("retained tree outcome")};assert_eq!(std::fs::read_to_string(std::path::Path::new(&recovery).join("child")).unwrap(),"foreign");
    }

    #[test]
    fn source_management_adopts_committed_rename_after_cancellation_and_blocks_edits_until_settled() {
        let(mut dashboard,mut tui,root)=view();std::fs::write(root.join("a.txt"),"original").unwrap();dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let document_window=dashboard.focused_index(&tui);dashboard.request_source(document_window,Request::Load("a.txt".into()));settle(&mut dashboard,&mut tui);dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let files_window=dashboard.focused_index(&tui);
        let(send,receive)=mpsc::sync_channel(2);dashboard_source_view::testing::observe(&mut dashboard.source,dashboard_source_view::testing::Worker{window:dashboard.windows[files_window].id.clone(),operation:10,generation:0,cancelled:Arc::new(AtomicBool::new(true)),receive,protected:vec!["a.txt".into(),"b.txt".into()]});assert!(dashboard.management_blocks_edit(document_window));assert!(dashboard.dirty_close(files_window));std::fs::rename(root.join("a.txt"),root.join("b.txt")).unwrap();let document=Workspace::new(&root).unwrap().load("b.txt",&AtomicBool::new(false),&mut |_|{}).unwrap();send.send(Message::Finished(Ok(Response::Renamed("a.txt".into(),Entry{path:"b.txt".into(),name:"b.txt".into(),directory:false},Some(document))))).unwrap();dashboard.poll_documents(&mut tui);settle(&mut dashboard,&mut tui);
        assert!(matches!(&dashboard.windows[document_window].body,Body::Document(Some(current))if current.document.path=="b.txt"&&!current.read_only&&!current.dirty));assert!(!dashboard.management_blocks_edit(document_window));
        let cases:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../../../🔨️modules/📁️filesystem/📝️workspace/📬️commands/📝️buffer/🧫️fixtures/🔣️.json")).unwrap();for row in cases["cases"].as_array().unwrap().iter().filter(|row|row["path"]=="b.txt"){if !root.join("b.txt").exists(){std::fs::write(root.join("b.txt"),"original").unwrap();}dashboard.request_source(document_window,Request::Load("b.txt".into()));settle(&mut dashboard,&mut tui);let recovery=row["recovery"].as_str().map(str::to_owned);let(send,receive)=mpsc::sync_channel(1);dashboard_source_view::testing::observe(&mut dashboard.source,dashboard_source_view::testing::Worker{window:dashboard.windows[files_window].id.clone(),operation:11,generation:0,cancelled:Arc::new(AtomicBool::new(recovery.is_some())),receive,protected:vec!["b.txt".into()]});send.send(Message::Finished(Err(Error{kind:ErrorKind::Io,path:"b.txt".into(),detail:"uncertain".into(),recovery_path:recovery.clone()}))).unwrap();dashboard.poll_documents(&mut tui);assert!(matches!(&dashboard.windows[document_window].body,Body::Document(Some(current))if current.document.path=="b.txt"&&current.document.text=="original"&&current.read_only==row["readOnly"].as_bool().unwrap()&&current.recovery==recovery));assert!(matches!(&tui.scene.node(dashboard.windows[document_window].list).content,NodeContent::Widget(WidgetState::Editor(editor))if editor.read_only==row["readOnly"].as_bool().unwrap()));if row["reloadRefused"].as_bool().unwrap(){std::fs::remove_file(root.join("b.txt")).unwrap();dashboard.reload_document(document_window);settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[document_window].body,Body::Document(Some(current))if current.recovery==recovery&&current.read_only));}}
        println!("[DEBUG] late rename and uncertain outcomes preserve protected buffers");
    }

    #[test]
    fn source_directory_commands_protect_descendants_and_browse_retained_history(){
        let(mut dashboard,mut tui,root)=view();std::fs::create_dir(root.join("folder")).unwrap();std::fs::write(root.join("folder/a.txt"),"kept").unwrap();dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let files=dashboard.focused_index(&tui);let document=dashboard.create_window(&mut tui,Kind::Document,Place::Preference);dashboard.request_source(document,Request::Load("folder/a.txt".into()));settle(&mut dashboard,&mut tui);
        if let Body::Document(Some(current))=&mut dashboard.windows[document].body{current.dirty=true;}let revision=Workspace::new(&root).unwrap().inspect_entry("folder",&AtomicBool::new(false),&mut |_|{}).unwrap();dashboard.request_source(files,Request::Rename{path:"folder".into(),revision:revision.clone(),destination:"moved".into()});assert!(!dashboard.source.active());if let Body::Document(Some(current))=&mut dashboard.windows[document].body{current.dirty=false;}
        dashboard_source_view::testing::pending(&mut dashboard.source,Some((dashboard.windows[files].id.clone(),Request::Rename{path:"folder".into(),revision:revision.clone(),destination:"moved".into()})));assert!(dashboard.management_blocks_edit(document));dashboard_source_view::testing::pending(&mut dashboard.source,None);dashboard.request_source(files,Request::Rename{path:"folder".into(),revision,destination:"moved".into()});assert!(dashboard.management_blocks_edit(document));settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[document].body,Body::Document(Some(current))if current.document.path=="moved/a.txt"&&current.read_only));dashboard.reload_document(document);settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[document].body,Body::Document(Some(current))if !current.read_only&&current.document.text=="kept"));
        let revision=Workspace::new(&root).unwrap().inspect_entry("moved",&AtomicBool::new(false),&mut |_|{}).unwrap();dashboard.request_source(files,Request::Retain{path:"moved".into(),revision});settle(&mut dashboard,&mut tui);assert!(matches!(dashboard.windows[document].body,Body::Files(_)));dashboard.inspect_recovery(&mut tui,files);settle(&mut dashboard,&mut tui);let history=dashboard.focused_index(&tui);assert!(matches!(&dashboard.windows[history].body,Body::Files(files)if files.history&&files.entries.len()==1));let (retained,child)=match &dashboard.windows[history].body{Body::Files(files)=>(files.history_root.clone().unwrap(),files.entries[0].path.clone()),_=>unreachable!()};dashboard.begin_history(&mut tui,history,Some(HistoryAction::Archive));settle(&mut dashboard,&mut tui);dashboard.files_key(&mut tui,history,&KeyEvent{key:Key::Enter,mods:0});assert!(!dashboard.source.active());let selection=match &dashboard.windows[history].body{Body::Files(files)=>files.history_form.as_ref().unwrap().confirmation().unwrap().to_owned(),_=>unreachable!()};dashboard.management_paste(&mut tui,history,&selection);dashboard.files_key(&mut tui,history,&KeyEvent{key:Key::Esc,mods:0});assert!(matches!(&dashboard.windows[history].body,Body::Files(files)if files.history_form.is_none()));dashboard.begin_history(&mut tui,history,Some(HistoryAction::Archive));settle(&mut dashboard,&mut tui);dashboard.management_paste(&mut tui,history,&selection);dashboard.files_key(&mut tui,history,&KeyEvent{key:Key::Enter,mods:0});assert!(dashboard.source.active(),"archive not submitted: {:?}",dashboard.notice);settle(&mut dashboard,&mut tui);let archived=Workspace::new(&root).unwrap().inspect_history(&AtomicBool::new(false),&mut |_|{}).unwrap();assert_ne!(archived.active,selection,"archive refused: {:?}",dashboard.notice);let page=Workspace::new(&root).unwrap().history_page(None,&AtomicBool::new(false),&mut |_|{}).unwrap();let retained_row=match &dashboard.windows[history].body{Body::Files(files)=>{assert!(files.history);assert_eq!(files.inspection.as_ref().map(|value|&value.active),Some(&archived.active));assert_eq!(files.entries.iter().map(|entry|&entry.path).collect::<Vec<_>>(),page.entries.iter().map(|entry|&entry.path).collect::<Vec<_>>());assert!(files.entries.windows(2).all(|pair|pair[0].path<pair[1].path));files.entries.iter().position(|entry|entry.path==retained).expect("archive must preserve the authenticated retained path")},_=>panic!("history browser was replaced")};dashboard.next_history_page(history);assert!(!dashboard.source.active());dashboard.activate_file(&tui,history,retained_row);settle(&mut dashboard,&mut tui);let row=match &dashboard.windows[history].body{Body::Files(files)=>1+files.visible.iter().position(|index|files.entries[*index].path==child).unwrap(),_=>unreachable!()};dashboard.activate_file(&tui,history,row);settle(&mut dashboard,&mut tui);assert!(matches!(&dashboard.windows[history].body,Body::Document(Some(current))if current.read_only&&current.document.text=="kept"));std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn source_worker_spawn_refusal_unlocks_and_retains_the_guided_form(){let(mut dashboard,mut tui,root)=view();dashboard.open_files(&mut tui);settle(&mut dashboard,&mut tui);let index=dashboard.focused_index(&tui);dashboard.begin_management(&mut tui,index,EntryAction::CreateFile);dashboard.management_paste(&mut tui,index,"keep.txt");if let Body::Files(files)=&mut dashboard.windows[index].body{files.form.as_mut().unwrap().busy=true;}dashboard.files_key(&mut tui,index,&KeyEvent{key:Key::Esc,mods:0});assert!(matches!(&dashboard.windows[index].body,Body::Files(files)if files.form.as_ref().unwrap().busy));dashboard.with_sources(|view|view.adopt_source_worker(index,10,0,Arc::new(AtomicBool::new(false)),vec!["keep.txt".into()],Err(std::io::Error::from(std::io::ErrorKind::WouldBlock))));assert!(!dashboard.source.active());assert!(matches!(&dashboard.windows[index].body,Body::Files(files)if !files.form.as_ref().unwrap().busy));assert!(matches!(&tui.scene.node(dashboard.windows[index].list).content,NodeContent::Widget(WidgetState::Input(input))if input.value=="keep.txt"));std::fs::remove_dir_all(root).unwrap();}

}
