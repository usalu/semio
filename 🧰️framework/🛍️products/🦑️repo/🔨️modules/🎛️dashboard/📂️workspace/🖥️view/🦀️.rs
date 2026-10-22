//! 📂️ Workspace source interactions over borrowed native view ports and ordered effects.
use semio_framework_filesystem_workspace::{Document,Entry,EntryRevision,Error,Progress,Workspace,Stage,ErrorKind};
pub use semio_framework_filesystem_workspace::commands::{Request,EntryAction,DocumentBuffer,BrowserState};
use semio_framework_filesystem_workspace::commands::{self,EntryForm,HistoryForm,HistoryAction,Response,Message};
pub use semio_framework_ui_tui_core::{event::KeyEvent,signal::WidgetSignal};
use semio_framework_ui_tui_core::event::{mods,Key,Event};
pub use semio_framework_ui_tui_frame::scene::NodeId;
use semio_framework_ui_tui_frame::scene::NodeContent;
pub use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_widgets::widget::{EditorState,WidgetState};
pub use semio_framework_ui_tui_terminal::backend::Waker;
pub use dashboard_preferences::keymap::Keymap;
use dashboard_labels::{labels,DashboardLabels};
pub use ui_locale::Locale;
use std::{path::Path,sync::{Arc,atomic::{AtomicBool,Ordering},mpsc::{self,Receiver}}};

#[path="🎮️management/🦀️.rs"]
mod management;
#[path="🕰️history/🦀️.rs"]
mod history;

pub enum Body<F,D>{Files(F),Document(D),Other}
pub type BodyRef<'a>=Body<&'a BrowserState,Option<&'a DocumentBuffer>>;
pub type BodyMut<'a>=Body<&'a mut BrowserState,Option<&'a mut DocumentBuffer>>;
pub enum OwnedBody{Files(BrowserState),Document(Option<Box<DocumentBuffer>>)}
pub struct WindowRef<'a>{pub id:&'a str,pub list:NodeId,pub caption:Option<NodeId>,pub body:BodyRef<'a>}
pub struct WindowMut<'a>{pub id:&'a str,pub list:NodeId,pub body:BodyMut<'a>}

/// 🪟️ Borrows the actual app-owned source fields; implementations never copy the window model.
pub trait SourceWindowPort{
    type Windows<'a>:Iterator<Item=WindowRef<'a>> where Self:'a;
    type WindowsMut<'a>:Iterator<Item=WindowMut<'a>> where Self:'a;
    fn windows(&self)->Self::Windows<'_>;
    fn windows_mut(&mut self)->Self::WindowsMut<'_>;
    fn window(&self,index:usize)->WindowRef<'_>;
    fn window_mut(&mut self,index:usize)->WindowMut<'_>;
    fn len(&self)->usize;
    fn set_model(&mut self,index:usize,body:OwnedBody);
    fn replace(&mut self,tui:&mut Tui,index:usize,kind:Kind);
    fn index(&self,id:&str)->Option<usize>{self.windows().position(|window|window.id==id)}
}
pub struct Context<'a>{pub root:&'a Path,pub keymap:&'a Keymap,pub locale:Locale,pub waker:Option<Waker>}
impl Context<'_>{fn text(&self)->&'static DashboardLabels{labels(self.locale)}}
pub enum Effect{Notice(Option<String>),Failure(dashboard_feedback::Failure),Focus(String),Close(String),Remount,Request(String,Request),Signals(Vec<(NodeId,WidgetSignal)>)}
#[derive(Default)]
pub struct Controller{job:Option<Job>,operation:u64,pending:Option<(String,Request)>}
pub struct View<'a,H:SourceWindowPort>{source_job:&'a mut Option<Job>,source_operation:&'a mut u64,source_pending:&'a mut Option<(String,Request)>,port:&'a mut H,context:Context<'a>,effects:Vec<Effect>}
struct Job{window:String,operation:u64,generation:u64,cancelled:Arc<AtomicBool>,receive:Receiver<Message>,protected:Vec<String>}
impl Drop for Job{fn drop(&mut self){self.cancelled.store(true,Ordering::Release);}}
impl Controller{
    pub fn blocks_path(&self,path:&str)->bool{blocked_path(self.job.as_ref(),self.pending.as_ref(),path)}
    pub fn active(&self)->bool{self.job.is_some()}
    pub fn pending(&self)->bool{self.pending.is_some()}
    pub fn operation(&self)->u64{self.operation}
    pub fn cancelled(&self)->Option<Arc<AtomicBool>>{self.job.as_ref().map(|job|job.cancelled.clone())}
    pub fn active_window(&self)->Option<&str>{self.job.as_ref().map(|job|job.window.as_str())}
    pub fn interact<'a,H:SourceWindowPort,R>(&'a mut self,port:&'a mut H,context:Context<'a>,action:impl FnOnce(&mut View<'a,H>)->R)->(R,Vec<Effect>){
        let mut view=View{source_job:&mut self.job,source_operation:&mut self.operation,source_pending:&mut self.pending,port,context,effects:Vec::new()};
        let result=action(&mut view);(result,view.effects)
    }
}
#[derive(Clone,Copy)]
pub enum Kind{Files,Document}
const SOURCE_MESSAGE_LIMIT:usize=commands::MESSAGE_CAPACITY;
fn parent(path:&str)->String{path.rsplit_once('/').map_or_else(String::new,|(parent,_)|parent.into())}
impl<H:SourceWindowPort> View<'_,H>{
    fn text(&self)->&'static DashboardLabels{self.context.text()}
    fn remount(&mut self,_:&mut Tui){self.effects.push(Effect::Remount);}
    fn focus_window(&mut self,_:&mut Tui,index:usize){self.effects.push(Effect::Focus(self.port.window(index).id.to_owned()));}
    fn close_window(&mut self,_:&mut Tui,index:usize){self.effects.push(Effect::Close(self.port.window(index).id.to_owned()));}
    fn on_signals(&mut self,_:&mut Tui,signals:Vec<(NodeId,WidgetSignal)>){self.effects.push(Effect::Signals(signals));}
    fn record_failure(&mut self,failure:dashboard_feedback::Failure){self.effects.push(Effect::Failure(failure));}
    fn record_io(&mut self,operation:dashboard_feedback::Operation,error:std::io::Error){self.record_failure(dashboard_feedback::Failure::io(operation,error));}
    fn replace_body(&mut self,tui:&mut Tui,index:usize,kind:Kind){self.port.replace(tui,index,kind);self.effects.push(Effect::Remount);}
}

fn blocked_path(job:Option<&Job>,pending:Option<&(String,Request)>,path:&str)->bool{job.is_some_and(|job|job.protected.iter().any(|protected|commands::path_contains(protected,path)))||pending.is_some_and(|(_,request)|request.management_paths().into_iter().flatten().any(|protected|commands::path_contains(protected,path)))}

/// 🖼️ Mounts the sole source body widgets under an app-owned body root.
pub fn mount(tui:&mut Tui,root:NodeId,kind:Kind)->(NodeId,NodeId){
    use semio_framework_ui_tui_frame::{scene::Node,layout::{Constraint,Dimension}};
    use semio_framework_ui_tui_widgets::widget::{LabelState,Align,ListState};
    let caption=tui.scene.add(root,Node::new(NodeContent::Widget(WidgetState::Label(LabelState{text:String::new(),align:Align::Left,role:semio_framework_ui_tui_core::theme::Role::MutedForeground}))));
    tui.scene.node_mut(caption).set_constraint(Constraint{width:Dimension::Weight(1),height:Dimension::Cells(1),..Default::default()});
    let widget=match kind{Kind::Files=>WidgetState::List(ListState::new(Vec::new())),Kind::Document=>{let mut editor=EditorState::new("",1_048_576);editor.read_only=true;WidgetState::Editor(editor)}};
    let list=tui.scene.add(root,Node::new(NodeContent::Widget(widget)));
    tui.scene.node_mut(list).set_constraint(Constraint{width:Dimension::Weight(1),height:Dimension::Weight(1),..Default::default()});
    (list,caption)
}
#[cfg(feature="testkit")]
pub mod testing{
    use super::*;
    pub struct Worker{pub window:String,pub operation:u64,pub generation:u64,pub cancelled:Arc<AtomicBool>,pub receive:Receiver<Message>,pub protected:Vec<String>}
    pub fn observe(controller:&mut Controller,worker:Worker){controller.operation=worker.operation;controller.job=Some(Job{window:worker.window,operation:worker.operation,generation:worker.generation,cancelled:worker.cancelled,receive:worker.receive,protected:worker.protected});}
    pub fn pending(controller:&mut Controller,value:Option<(String,Request)>){controller.pending=value;}
}

fn accepts_completion(operation:u64,current:u64,cancelled:bool,authoritative:bool,present:bool)->bool{present&&operation==current&&(!cancelled||authoritative)}
fn accepts_document(expected:u64,current:u64,dirty:bool)->bool{expected==current&&!dirty}
impl<H:SourceWindowPort> View<'_,H> {


    pub fn request_source(&mut self, index: usize, request: Request) {
        let dirty=self.port.windows().position(|window|matches!(&window.body,Body::Document(Some(document))if document.dirty&&request.management_paths().into_iter().flatten().any(|path|commands::path_contains(path,&document.document.path))));if let Some(dirty)=dirty{self.dirty_close(dirty);return;}
        if let Some(job) = &self.source_job {
            job.cancelled.store(true, Ordering::Release);
            *self.source_pending = Some((self.port.window(index).id.to_owned(), request));
            self.effects.push(Effect::Notice(Some(self.text().source_busy.as_str().into())));
            return;
        }
        let Some(operation) = self.source_operation.checked_add(1) else { return }; *self.source_operation = operation;
        self.effects.push(Effect::Notice(Some(self.text().source_starting.fill(&[("cancel", &self.context.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "cancel-source").unwrap_or_default())]).into_string())));
        let protected=request.management_paths().into_iter().flatten().map(str::to_owned).collect();
        let root = self.context.root.to_path_buf();
        let generation = match self.port.window(index).body { Body::Document(Some(document)) => document.generation, _ => 0 };
        let cancelled = Arc::new(AtomicBool::new(false)); let signal = cancelled.clone(); let waker = self.context.waker.clone();
        let wake=waker.map(|waker|Arc::new(move||{waker.wake();}) as Arc<dyn Fn()+Send+Sync>);
        let spawn=commands::spawn(root,request,signal,wake);
        self.adopt_source_worker(index,operation,generation,cancelled,protected,spawn);
    }

    pub fn adopt_source_worker(&mut self,index:usize,operation:u64,generation:u64,cancelled:Arc<AtomicBool>,protected:Vec<String>,result:std::io::Result<Receiver<Message>>){match result{Ok(receive)=>*self.source_job =Some(Job{window:self.port.window(index).id.to_owned(),operation,generation,cancelled,receive,protected}),Err(error)=>{self.unlock_management(index);self.record_io(dashboard_feedback::Operation::Source,error);}}}

    pub fn cancel_source(&mut self) { *self.source_pending = None; if let Some(job) = &self.source_job { job.cancelled.store(true, Ordering::Release); } self.effects.push(Effect::Notice(Some(self.text().source_document_cancelled.as_str().into()))); }

    pub fn poll_documents(&mut self, tui: &mut Tui) -> bool {
        let mut changed = false;
        let mut remaining = SOURCE_MESSAGE_LIMIT;
        while remaining > 0 {
            let Some(job) = &self.source_job else { break };
            let message = match job.receive.try_recv() {
                Ok(message) => message,
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    let window=job.window.clone();self.source_job.take();if let Some(index)=self.port.index(&window){self.unlock_management(index);} self.record_failure(dashboard_feedback::Failure::text(dashboard_feedback::Operation::Source,self.text().source_worker_failed.as_str().into()));
                    if let Some((window, request)) = self.source_pending.take() { if self.port.index(&window).is_some() { self.effects.push(Effect::Request(window,request)); } }
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
            if matches!(message, Message::Finished(_)) && !accepts_completion(operation,*self.source_operation,cancelled,committed_save,self.port.index(&window).is_some()) {
                self.source_job.take();if let Some(index)=self.port.index(&window){self.unlock_management(index);}
                if let Some((window, request)) = self.source_pending.take() { if self.port.index(&window).is_some() { self.effects.push(Effect::Request(window,request)); } }
                changed = true;
                break;
            }
            let Some(index) = self.port.index(&window) else { continue };
            changed = true;
            match message {
                Message::Progress(progress)=>self.effects.push(Effect::Notice(Some(self.text().source_progress(&progress,&self.context.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix,"cancel-source").unwrap_or_default())))),
                Message::Finished(result) => {
                    self.source_job.take();
                    match result {
                        Ok(Response::Inspected(path,kind,revision)) => self.show_management_form(tui,index,kind,path,Some(revision)),
                        Ok(Response::CreatedFile(document)) => { self.finish_management_form(tui,index); self.adopt_created_document(tui,index,document); }
                        Ok(Response::CreatedDirectory(entry)) => self.management_completed(tui,index,&entry.path,None),
                        Ok(Response::Renamed(previous,entry,document)) => { self.reconcile_renamed_documents(tui,&previous,&entry.path,document); self.management_completed(tui,index,&entry.path,None); }
                        Ok(Response::Retained(retained)) => { self.reconcile_retained_documents(tui,&retained.path); self.management_completed(tui,index,&retained.path,Some(retained.recovery_path)); }
                        Ok(response @ (Response::HistoryPage(_) | Response::HistoryInspected(..) | Response::HistoryChanged(..)))=>self.adopt_history_result(tui,index,response),
                        Ok(Response::RecoveryList(directory,entries))=>{if self.dirty_close(index){continue;}if !matches!(self.port.window(index).body,Body::Files(_)){self.replace_body(tui,index,Kind::Files);}if let Body::Files(files)=self.port.window_mut(index).body{files.adopt_recovery_list(directory,entries);}self.paint_files(tui,index);self.effects.push(Effect::Notice(Some(self.text().source_read_only.as_str().into())));}
                        Ok(Response::List(directory,entries))=>{if self.dirty_close(index){continue;}if !matches!(self.port.window(index).body,Body::Files(_)){self.replace_body(tui,index,Kind::Files);}if let Body::Files(files)=self.port.window_mut(index).body{files.adopt_list(directory,entries);}self.paint_files(tui,index);self.effects.push(Effect::Notice(match self.port.window(index).body{Body::Files(files)=>files.recovery.as_ref().map(|path|self.text().source_retained.fill(&[("path",path),("inspect",&self.context.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix,"inspect-recovery").unwrap_or_default())]).into_string()),_=>None}));}
                        Ok(response @ (Response::Load(_) | Response::Recovery(_))) => {
                            let (document, read_only) = match response { Response::Load(document) => (document, false), Response::Recovery(document) => (document, true), _ => unreachable!() };
                            if matches!(self.port.window(index).body, Body::Document(Some(current)) if !accepts_document(generation,current.generation,current.dirty)) { continue; }
                            self.replace_body(tui,index,Kind::Document);
                            let mut editor = EditorState::new(document.text.clone(), 1_048_576); editor.cursor = 0; editor.read_only = read_only;
                            if let Some(widget) = tui.scene.node_mut(self.port.window(index).list).widget() { *widget = WidgetState::Editor(editor); }
                            self.port.set_model(index,OwnedBody::Document(Some(Box::new(DocumentBuffer::new(document,read_only)))));
                            self.effects.push(Effect::Notice(Some(self.text().source_reloaded.as_str().into())));
                        }
                        Ok(Response::Save(document)) => {
                            let node = self.port.window(index).list;
                            if let (Body::Document(Some(current)),NodeContent::Widget(WidgetState::Editor(editor)))=(self.port.window_mut(index).body,&tui.scene.node(node).content){current.saved(document,&editor.value);}
                            self.effects.push(Effect::Notice(Some(self.text().source_saved.as_str().into())));
                        }
                        Err(error) => {
                            if mutation{self.mark_uncertain_documents(tui,&protected,error.recovery_path.as_deref());}
                            self.unlock_management(index);if let Body::Files(files)=self.port.window_mut(index).body { if error.recovery_path.is_some(){files.recovery=error.recovery_path.clone();} }
                            if let Body::Document(Some(current)) = self.port.window_mut(index).body { current.record_failure(&error); }
                            self.record_failure(dashboard_feedback::Failure::text(dashboard_feedback::Operation::Source,self.text().source_failure(&error,mutation)));
                        }
                    }
                    self.remount(tui);
                    if let Some((window, request)) = self.source_pending.take() { if self.port.index(&window).is_some() { self.effects.push(Effect::Request(window,request)); } }
                    break;
                }
            }
        }
        if remaining == 0 && self.source_job.is_some() { if let Some(waker) = &self.context.waker { waker.wake(); } }
        changed
    }

    pub fn paint_files(&mut self, tui: &mut Tui, index: usize) {
        let node = self.port.window(index).list;
        let Body::Files(files) = self.port.window_mut(index).body else { return };
        if files.form.is_some() || files.history_form.is_some() { return; }
        if let Some(widget)=tui.scene.node_mut(node).widget() { if !matches!(widget,WidgetState::List(_)){*widget=WidgetState::List(semio_framework_ui_tui_widgets::widget::ListState::new(Vec::new()));} }
        let items=files.rows();
        if let Some(WidgetState::List(list)) = tui.scene.node_mut(node).widget() { list.selected = list.selected.min(items.len().saturating_sub(1)); list.marks = vec![false; items.len()]; list.items = items; }
    }

    pub fn files_key(&mut self, tui: &mut Tui, index: usize, key: &KeyEvent) {
        if self.history_key(tui,index,key)||self.management_key(tui,index,key) { return; }
        let mut request = None;
        if let Body::Files(files) = self.port.window_mut(index).body {
            match key.key {
                Key::Char(c) if key.mods & (mods::CTRL | mods::ALT) == 0 && !c.is_control() && files.query.len() + c.len_utf8() <= 256 => files.query.push(c),
                Key::Backspace if !files.query.is_empty() => { files.query.truncate(semio_framework_ui_tui_core::text::previous_boundary(&files.query, files.query.len())); }
                Key::Backspace | Key::Esc => request = Some(files.browse_parent()),
                _ => { let signals = tui.dispatch(&Event::Key(*key)); self.on_signals(tui, signals); return; }
            }
        }
        if let Some(request) = request { self.request_source(index,request); } else { self.paint_files(tui, index); }
    }

    pub fn activate_file(&mut self,_tui:&Tui,index:usize,selected:usize){let request=match self.port.window(index).body{Body::Files(files)=>files.activate(selected),_=>None};if let Some(request)=request{self.request_source(index,request);}}

    pub fn save_document(&mut self, tui: &Tui, index: usize) {
        if self.management_blocks_edit(index){self.effects.push(Effect::Notice(Some(self.text().source_busy.as_str().into())));return;}
        let Body::Document(Some(current)) = self.port.window(index).body else { return };
        if current.read_only { self.effects.push(Effect::Notice(Some(self.text().source_read_only.as_str().into()))); return; }
        let NodeContent::Widget(WidgetState::Editor(editor)) = &tui.scene.node(self.port.window(index).list).content else { return };
        let request = current.save_request(editor.value.clone());
        self.request_source(index, request);
    }



    pub fn dirty_close(&mut self, index: usize) -> bool {
        if matches!(self.port.window(index).body,Body::Files(files)if files.history_form.as_ref().is_some_and(|form|form.busy))||self.management_blocks_edit(index)||self.source_job.as_ref().is_some_and(|job|job.window==self.port.window(index).id&&!job.protected.is_empty())||self.source_pending.as_ref().is_some_and(|(window,request)|window==self.port.window(index).id&&request.management_paths().into_iter().any(|path|path.is_some())){self.effects.push(Effect::Notice(Some(self.text().source_busy.as_str().into())));return true;}
        let dirty = matches!(self.port.window(index).body, Body::Document(Some(document)) if document.dirty);
        if dirty {
            if let Body::Document(Some(document)) = self.port.window_mut(index).body { document.discard_confirmation = true; }
            let save = self.context.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "save-document").unwrap_or_default();
            let discard = self.context.keymap.keys_label(dashboard_preferences::keymap::Scope::Prefix, "discard-document").unwrap_or_default();
            self.effects.push(Effect::Notice(Some(self.text().source_confirm_discard.fill(&[("save", &save), ("discard", &discard)]).into_string())));
        }
        dirty
    }

    pub fn reload_document(&mut self, index: usize) {
        if self.dirty_close(index) { return; }
        if let Body::Document(Some(current)) = self.port.window(index).body { self.request_source(index, current.reload_request()); }
    }

    pub fn discard_document(&mut self, tui: &mut Tui, index: usize) {
        let Body::Document(Some(current)) = self.port.window(index).body else { return };
        if current.read_only { self.close_window(tui, index); return; }
        if current.dirty && !current.discard_confirmation { self.dirty_close(index); return; }
        let directory = parent(&current.document.path);
        self.cancel_source(); self.replace_body(tui,index,Kind::Files);
        self.request_source(index, Request::List(directory));
    }

    pub fn refresh_documents(&mut self, tui: &mut Tui) {
        for index in 0..self.port.len() {
            let caption = match self.port.window(index).body {
                Body::Files(files) => self.text().source_browser_caption(files),
                Body::Document(Some(current)) => {
                    let position = match &tui.scene.node(self.port.window(index).list).content { NodeContent::Widget(WidgetState::Editor(editor)) => { let (line, column) = editor.position(); self.text().source_position.fill(&[("line", &line.to_string()), ("column", &column.to_string())]).into_string() }, _ => String::new() };
                    self.text().source_document_caption(current,&position)
                }
                _ => continue,
            };
            if let Some(node) = self.port.window(index).caption { if let Some(WidgetState::Label(label)) = tui.scene.node_mut(node).widget() { if label.text != caption { label.text = caption; } } }
        }
    }
}


#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn source_settlement_matches_portable_owner_generation_and_cancellation_cases(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
  for row in fixture["cases"].as_array().unwrap(){
   let n=|name:&str|row[name].as_u64().unwrap();let b=|name:&str|row[name].as_bool().unwrap();
   let kind=row["kind"].as_str().unwrap();
   let actual=if kind=="disconnect"{"disconnect"}else if !accepts_completion(n("operation"),n("currentOperation"),b("cancelled"),b("authoritative"),b("windowPresent"))||kind=="load"&&!accepts_document(n("expectedGeneration"),n("currentGeneration"),b("dirty")){"ignore"}else{"adopt"};
   assert_eq!(actual,row["expected"].as_str().unwrap(),"{}",row["name"]);
  }
 }
}
