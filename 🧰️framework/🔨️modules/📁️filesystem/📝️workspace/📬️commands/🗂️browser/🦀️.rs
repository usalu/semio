//! 🗂️ Domain browser state turns local presentation intent into bounded Workspace commands.
use crate::{Entry,EntryRevision,HistoryCursor,HistoryInspection};
use super::{EntryAction,HistoryAction,Request};
/// 🧾️ A guided entry form holds its actual issued revision until command settlement.
pub struct EntryForm{pub kind:EntryAction,pub path:String,pub revision:Option<EntryRevision>,pub busy:bool}
/// 🕰️ Maintenance confirmation retains the exact opaque inspected state.
pub struct HistoryForm{pub action:HistoryAction,pub expected:HistoryInspection,pub busy:bool}
/// 🧭️ Local browser state is independent of the device widget and key binding implementation.
#[derive(Default)]
pub struct BrowserState{pub directory:String,pub query:String,pub entries:Vec<Entry>,pub visible:Vec<usize>,pub form:Option<EntryForm>,pub recovery:Option<String>,pub history:bool,pub history_root:Option<String>,pub next:Option<HistoryCursor>,pub inspection:Option<HistoryInspection>,pub history_form:Option<HistoryForm>}
/// 🚧️ Closed presentation refusals carry no filesystem authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BrowserError{ReadOnly,Busy,InvalidPath,MissingObservation,ConfirmRetain,EmptyHistory,PendingHistory,ConfirmHistory,NoPage}
impl BrowserError{pub fn id(self)->&'static str{match self{Self::ReadOnly=>"ReadOnly",Self::Busy=>"Busy",Self::InvalidPath=>"InvalidPath",Self::MissingObservation=>"MissingObservation",Self::ConfirmRetain=>"ConfirmRetain",Self::EmptyHistory=>"EmptyHistory",Self::PendingHistory=>"PendingHistory",Self::ConfirmHistory=>"ConfirmHistory",Self::NoPage=>"NoPage"}}}
/// 🔐️ An active segment is explicitly confirmed without a preselected empty submission.
pub fn history_selection<'a>(active:&'a str,pending:Option<&'a str>,action:HistoryAction)->Result<&'a str,BrowserError>{if let Some(pending)=pending{if action==HistoryAction::Archive{return Err(BrowserError::PendingHistory);}return Ok(pending);}if active.is_empty(){return Err(BrowserError::EmptyHistory);}Ok(active)}
pub fn history_confirmation(active:&str,pending:Option<&str>,action:HistoryAction,value:&str,busy:bool)->Result<(),BrowserError>{if busy{return Err(BrowserError::Busy);}if value!=history_selection(active,pending,action)?{return Err(BrowserError::ConfirmHistory);}Ok(())}
impl HistoryForm{pub fn confirmation(&self)->Result<&str,BrowserError>{history_selection(&self.expected.active,self.expected.pending.as_deref(),self.action)}}
impl BrowserState{
 /// 🗂️ Listing projection retains canonical query matches independently of device widget state.
 pub fn rows(&mut self)->Vec<String>{let query=self.query.to_lowercase();self.visible=self.entries.iter().enumerate().filter(|(_,entry)|entry.name.to_lowercase().contains(&query)).map(|(index,_)|index).collect();let mut rows=self.visible.iter().map(|index|{let entry=&self.entries[*index];format!("{}{}",entry.name,if entry.directory{"/"}else{""})}).collect::<Vec<_>>();if !self.directory.is_empty()&&self.query.is_empty(){rows.insert(0,"../".into());}rows}
 /// 📃️ Paged history replaces only local query state while retaining the real backend cursor.
 pub fn adopt_page(&mut self,page:crate::HistoryPage){self.history=true;self.directory.clear();self.query.clear();self.entries=page.entries;self.next=page.next;self.form=None;self.history_form=None;self.history_root=None;}
 /// 📜️ Authenticated descendants remain bounded to their selected retained root.
 pub fn adopt_recovery_list(&mut self,directory:String,entries:Vec<Entry>){self.history_root=if directory.is_empty(){None}else{self.history_root.take().filter(|root|std::path::Path::new(&directory).starts_with(root)).or_else(||Some(directory.clone()))};self.history=true;self.directory=directory;self.entries=entries;self.query.clear();self.form=None;self.history_form=None;self.next=None;}
 /// 🌳️ Ordinary browsing retires history-only view state without discarding known recovery guidance.
 pub fn adopt_list(&mut self,directory:String,entries:Vec<Entry>){self.directory=directory;self.entries=entries;self.query.clear();self.history=false;self.history_root=None;self.next=None;self.inspection=None;self.form=None;self.history_form=None;}
 /// 🔓️ A settled or refused worker unlocks forms while preserving their inputs and issued observations.
 pub fn unlock_forms(&mut self){if let Some(form)=&mut self.form{form.busy=false;}if let Some(form)=&mut self.history_form{form.busy=false;}}

 /// 📂️ Empty history selects its paged index; descendants retain authenticated browsing.
 pub fn browse(&self,path:String)->Request{if self.history{if path.is_empty(){Request::HistoryPage(None)}else{Request::RecoveryList(path)}}else{Request::List(path)}}
 /// ⬆️ History navigation never escapes the selected authenticated root.
 pub fn browse_parent(&self)->Request{let parent=if self.history{self.history_root.as_ref().filter(|root|*root!=&self.directory).and_then(|root|std::path::Path::new(&self.directory).parent().filter(|path|path.starts_with(root))).map_or_else(String::new,|path|path.to_string_lossy().into_owned())}else{self.directory.rsplit_once('/').map_or_else(String::new,|(parent,_)|parent.into())};self.browse(parent)}
 /// 🎯️ A visible row maps to a domain entry without trusting a display caption.
 pub fn activate(&self,selected:usize)->Option<Request>{let parent=!self.directory.is_empty()&&self.query.is_empty();if parent&&selected==0{return Some(self.browse_parent());}let entry=self.visible.get(selected.saturating_sub(usize::from(parent))).and_then(|index|self.entries.get(*index))?;Some(if self.history{if entry.directory{Request::RecoveryList(entry.path.clone())}else{Request::Recovery(entry.path.clone())}}else if entry.directory{Request::List(entry.path.clone())}else{Request::Load(entry.path.clone())})}
 /// ✉️ Forms produce only fixed entry commands with issued observations and bounded relative destinations.
 pub fn prepare_entry(&self,value:&str)->Result<Request,BrowserError>{if self.history{return Err(BrowserError::ReadOnly);}let form=self.form.as_ref().ok_or(BrowserError::MissingObservation)?;if form.busy{return Err(BrowserError::Busy);}if value.is_empty()||value.contains('/')||crate::validate_relative(value).is_err(){return Err(BrowserError::InvalidPath);}let destination=if self.directory.is_empty(){value.into()}else{format!("{}/{value}",self.directory)};if crate::validate_relative(&destination).is_err(){return Err(BrowserError::InvalidPath);}Ok(match form.kind{EntryAction::CreateFile=>Request::CreateFile(destination),EntryAction::CreateDirectory=>Request::CreateDirectory(destination),EntryAction::Rename=>Request::Rename{path:form.path.clone(),revision:form.revision.clone().ok_or(BrowserError::MissingObservation)?,destination},EntryAction::Retain=>{if value!=form.path.rsplit('/').next().unwrap_or(&form.path){return Err(BrowserError::ConfirmRetain);}Request::Retain{path:form.path.clone(),revision:form.revision.clone().ok_or(BrowserError::MissingObservation)?}}})}
 /// 📑️ A next page can only carry an actual backend-issued cursor.
 pub fn next_page(&self)->Result<Request,BrowserError>{self.next.clone().map(|cursor|Request::HistoryPage(Some(cursor))).ok_or(BrowserError::NoPage)}
 /// 🗳️ Confirmed maintenance carries the exact inspected token, never a reconstructed path.
 pub fn prepare_history(&self,value:&str)->Result<Request,BrowserError>{let form=self.history_form.as_ref().ok_or(BrowserError::MissingObservation)?;history_confirmation(&form.expected.active,form.expected.pending.as_deref(),form.action,value,form.busy)?;Ok(match form.action{HistoryAction::Reconcile=>Request::ReconcileHistory(form.expected.clone()),HistoryAction::Archive=>Request::ArchiveHistory(form.expected.clone())})}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
