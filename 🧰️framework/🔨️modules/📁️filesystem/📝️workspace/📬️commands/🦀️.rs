//! 📬️ Bounded workspace command dispatch outside presentation event loops.
use crate::{Document, Entry, EntryRevision, Error, HistoryCursor, HistoryInspection, HistoryPage, Progress, RetainedEntry, Revision, Workspace};
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool, mpsc::{self, Receiver, SyncSender}};

#[path="📝️buffer/🦀️.rs"]
mod buffer;
pub use buffer::DocumentBuffer;
#[path="🗂️browser/🦀️.rs"]
mod browser;
pub use browser::{BrowserState,BrowserError,EntryForm,HistoryForm};

pub const MESSAGE_CAPACITY: usize = 32;
/// 🎮️ A guided entry command intent retained while its observation is issued.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryAction { CreateFile, CreateDirectory, Rename, Retain }
/// 🧰️ Explicit maintenance intents target the exact inspected history selection.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HistoryAction{Reconcile,Archive}
/// 📥️ Workspace-owned commands carry opaque observed revisions.
pub enum Request { HistoryPage(Option<HistoryCursor>), InspectHistory(Option<HistoryAction>), ReconcileHistory(HistoryInspection), ArchiveHistory(HistoryInspection), List(String), Load(String), Recovery(String), RecoveryList(String), Save { path:String, revision:Revision, text:String }, Inspect { path:String, kind:EntryAction }, CreateFile(String), CreateDirectory(String), Rename { path:String, revision:EntryRevision, destination:String }, Retain { path:String, revision:EntryRevision } }
impl Request {
 /// 🛡️ Entry-management footprints stay protected until actual settlement.
 pub fn management_paths(&self)->[Option<&str>;2]{match self{Self::Rename{path,destination,..}=>[Some(path),Some(destination)],Self::Retain{path,..}|Self::CreateFile(path)|Self::CreateDirectory(path)=>[Some(path),None],_=>[None,None]}}
}
/// 📤️ Settled command outcomes preserve committed publication independently of later cancellation.
pub enum Response { HistoryPage(HistoryPage), HistoryInspected(Option<HistoryAction>,HistoryInspection), HistoryChanged(HistoryAction,HistoryInspection), List(String,Vec<Entry>), Load(Document), Recovery(Document), RecoveryList(String,Vec<Entry>), Save(Document), Inspected(String,EntryAction,EntryRevision), CreatedFile(Document), CreatedDirectory(Entry), Renamed(String,Entry,Option<Document>), Retained(RetainedEntry) }
/// 📨️ Progress samples stay bounded while the final result remains lossless.
pub enum Message { Progress(Progress), Finished(Result<Response,Error>) }
impl Message {
 /// ⚖️ Publication outcomes remain authoritative after a caller requests cancellation.
 pub fn authoritative(&self)->bool{matches!(self,Self::Finished(Ok(Response::HistoryChanged(..) | Response::Save(_) | Response::CreatedFile(_) | Response::CreatedDirectory(_) | Response::Renamed(..) | Response::Retained(_))))||matches!(self,Self::Finished(Err(error))if error.recovery_path.is_some())}
}
/// 🌿️ Canonical segment boundaries distinguish descendants from neighboring names.
pub fn path_contains(root:&str,path:&str)->bool { path==root||path.strip_prefix(root).is_some_and(|suffix|suffix.starts_with('/')) }
fn publish_progress(send:&SyncSender<Message>,progress:Progress,wake:&Option<Arc<dyn Fn()+Send+Sync>>){if send.try_send(Message::Progress(progress)).is_ok(){if let Some(wake)=wake{wake();}}}
/// 🧵️ Runs one command with the caller's exact cancellation flag and a bounded progress channel.
pub fn spawn(root:PathBuf,request:Request,cancel:Arc<AtomicBool>,wake:Option<Arc<dyn Fn()+Send+Sync>>)->std::io::Result<Receiver<Message>> {
 let(send,receive)=mpsc::sync_channel(MESSAGE_CAPACITY);
 std::thread::Builder::new().name("workspace-command".into()).spawn(move||{
  let result=Workspace::new(&root).and_then(|workspace|{
   let mut progress=|progress|publish_progress(&send,progress,&wake);
   match request {
    Request::HistoryPage(cursor)=>workspace.history_page(cursor.as_ref(),&cancel,&mut progress).map(Response::HistoryPage),
    Request::InspectHistory(action)=>workspace.inspect_history(&cancel,&mut progress).map(|inspection|Response::HistoryInspected(action,inspection)),
    Request::ReconcileHistory(expected)=>workspace.reconcile_history(&expected,&cancel,&mut progress).map(|inspection|Response::HistoryChanged(HistoryAction::Reconcile,inspection)),
    Request::ArchiveHistory(expected)=>workspace.archive_history(&expected,&cancel,&mut progress).map(|inspection|Response::HistoryChanged(HistoryAction::Archive,inspection)),
    Request::List(path)=>workspace.list(&path,&cancel,&mut progress).map(|entries|Response::List(path,entries)),
    Request::Load(path)=>workspace.load(&path,&cancel,&mut progress).map(Response::Load),
    Request::Recovery(path)=>match workspace.load_recovery(&path,&cancel,&mut progress){Ok(document)=>Ok(Response::Recovery(document)),Err(error)if error.kind==crate::ErrorKind::NotFile=>workspace.list_recovery(&path,&cancel,&mut progress).map(|entries|Response::RecoveryList(path,entries)),Err(error)=>Err(error)},
    Request::RecoveryList(path)=>workspace.list_recovery(&path,&cancel,&mut progress).map(|entries|Response::RecoveryList(path,entries)),
    Request::Save{path,revision,text}=>workspace.save(&path,&revision,&text,&cancel,&mut progress).map(Response::Save),
    Request::Inspect{path,kind}=>workspace.inspect_entry(&path,&cancel,&mut progress).map(|revision|Response::Inspected(path,kind,revision)),
    Request::CreateFile(path)=>workspace.create_file(&path,"",&cancel,&mut progress).map(Response::CreatedFile),
    Request::CreateDirectory(path)=>workspace.create_directory(&path,&cancel,&mut progress).map(Response::CreatedDirectory),
    Request::Rename{path,revision,destination}=>workspace.rename_entry(&path,&revision,&destination,&cancel,&mut progress).map(|entry|{let document=if entry.directory{None}else{workspace.load(&entry.path,&cancel,&mut progress).ok()};Response::Renamed(path,entry,document)}),
    Request::Retain{path,revision}=>workspace.retain_entry(&path,&revision,&cancel,&mut progress).map(Response::Retained),
   }
  });
  let _=send.send(Message::Finished(result));if let Some(wake)=wake{wake();}
 }).map(|_|receive)
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
