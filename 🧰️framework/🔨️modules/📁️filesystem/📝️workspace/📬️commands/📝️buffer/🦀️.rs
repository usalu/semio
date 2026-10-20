//! 📝️ Document drafts adopt actual publication outcomes without assuming atomic source revisions.
use crate::Document;
use super::path_contains;
/// 📄️ Local document state retains its opaque filesystem revision and unsaved draft status.
pub struct DocumentBuffer { pub document:Document,pub read_only:bool,pub recovery:Option<String>,pub dirty:bool,pub generation:u64,pub discard_confirmation:bool }
impl DocumentBuffer {
 /// 🧭️ A refused retry keeps an earlier actual recovery location until a successful reload replaces the buffer.
 pub fn record_failure(&mut self,error:&crate::Error){if let Some(path)=&error.recovery_path{self.recovery=Some(path.clone());}}
 /// ✏️ Local edits advance the buffer generation and retire a previous discard prompt.
 pub fn edited(&mut self){self.dirty=true;self.generation=self.generation.saturating_add(1);self.discard_confirmation=false;}
 /// 🔄️ Reloading retained content always uses the authenticated readonly recovery route.
 pub fn reload_request(&self)->super::Request{if self.read_only&&std::path::Path::new(&self.document.path).is_absolute(){super::Request::Recovery(self.document.path.clone())}else{super::Request::Load(self.document.path.clone())}}
 /// 📮️ Save requests retain the exact observed revision and current draft.
 pub fn save_request(&self,text:String)->super::Request{super::Request::Save{path:self.document.path.clone(),revision:self.document.revision.clone(),text}}
 /// 🌱️ A loaded document starts with no unsaved changes or retained outcome.
 pub fn new(document:Document,read_only:bool)->Self{Self{document,read_only,recovery:None,dirty:false,generation:0,discard_confirmation:false}}
 /// 💾️ Settled saves keep newer editor text dirty while adopting the actual published revision.
 pub fn saved(&mut self,document:Document,text:&str){if self.document.path==document.path{self.dirty=text!=document.text;self.document=document;self.discard_confirmation=false;}}
 /// 🔀️ A committed move adopts a loaded revision or requires explicit reload at the actual destination.
 pub fn remap(&mut self,previous:&str,path:&str,loaded:Option<&Document>)->bool{if self.dirty||!path_contains(previous,&self.document.path){return false;}let suffix=&self.document.path[previous.len()..];if suffix.is_empty(){if let Some(document)=loaded{self.document=document.clone();self.read_only=false;self.recovery=None;return true;}}self.document.path=format!("{path}{suffix}");self.read_only=true;self.recovery=None;true}
 /// 🛡️ A possibly committed failure freezes clean affected buffers without inferring a move.
 pub fn mark_uncertain(&mut self,paths:&[String],recovery:Option<&str>)->bool{let Some(recovery)=recovery else{return false;};if self.dirty||!paths.iter().any(|path|path_contains(path,&self.document.path)){return false;}self.read_only=true;self.recovery=Some(recovery.into());true}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
