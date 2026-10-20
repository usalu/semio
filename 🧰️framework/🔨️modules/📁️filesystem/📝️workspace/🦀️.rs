//! 📝️ Bounded source commands with optimistic revisions and retained publication predecessors.

use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::time::{SystemTime, UNIX_EPOCH};

pub const MAXIMUM_FILE_BYTES: usize = 1_048_576;
pub const MAXIMUM_MANAGED_FILE_BYTES: u64 = 67_108_864;
pub const MAXIMUM_DIRECTORY_ENTRIES: usize = 4096;
pub const MAXIMUM_PATH_UNITS: usize = 256;
const MAXIMUM_JOURNAL_BYTES: u64 = 67_108_864;
const STATE: &str = ".workspace-events";
static SERIAL: AtomicU64 = AtomicU64::new(0);

/// 🚦️ Stable source-command refusal categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind { InvalidPath, Boundary, NotFile, TooLarge, TooManyEntries, NotText, Conflict, Cancelled, Busy, Io, Unsupported }
/// ⚠️ An owned error with bounded physical context.
#[derive(Clone, Debug)]
pub struct Error { pub kind: ErrorKind, pub path: String, pub detail: String, pub recovery_path: Option<String> }
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}: {}: {}", self.kind, self.path, self.detail) }
}
impl std::error::Error for Error {}
/// ⏳️ An expensive operation's current phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage { Listing, Reading, Staging, Checking, Publishing, Complete }
/// 📈️ Bounded work, without an invented total.
#[derive(Clone, Debug)]
pub struct Progress { pub stage: Stage, pub completed: u64, pub total: Option<u64> }
/// 🌳️ A physical child inside the workspace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry { pub path: String, pub name: String, pub directory: bool }
/// 🪪️ An opaque exact predecessor, retaining bytes and native identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Revision { root: PathBuf, path: String, stamp: Stamp, bytes: Arc<[u8]> }
/// 📄️ A loaded UTF-8 document and the authority needed to save it.
#[derive(Clone, Debug)]
pub struct Document { pub path: String, pub text: String, pub revision: Revision }
/// 🗝️ An opaque observed physical entry bound to one workspace and spelling.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryRevision { root: PathBuf, path: String, stamp: Stamp, directory: bool, nodes: Arc<[TreeNode]> }
/// 🧺️ A retained entry with its durable command identity and actual recovery path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetainedEntry { pub command_id: String, pub path: String, pub recovery_path: String, pub directory: bool }
/// 🗂️ A physical root whose source commands own their local event directory.
#[derive(Clone, Debug)]
pub struct Workspace { root: PathBuf, anchor: Arc<File> }
#[derive(Clone, Debug, PartialEq, Eq)]
struct TreeNode { suffix:String, stamp:Stamp, directory:bool }
#[derive(Clone, Debug, PartialEq, Eq)]
struct RetainedProof { identity:(u64,u64), fingerprint:Option<String>, directory:bool, source:String }
#[derive(Clone, Debug, PartialEq, Eq)]
struct Stamp { identity: (u64, u64), length: u64, modified: Option<SystemTime>, change: (i64, i64) }

fn error(kind: ErrorKind, path: &str, detail: impl ToString) -> Error { Error { kind, path: path.chars().take(256).collect(), detail: detail.to_string().chars().take(1024).collect(), recovery_path:None } }
fn io(path: &str, failure: std::io::Error) -> Error { error(match failure.kind(){std::io::ErrorKind::Interrupted=>ErrorKind::Cancelled,std::io::ErrorKind::Unsupported=>ErrorKind::Unsupported,_=>ErrorKind::Io}, path, failure) }
fn checkpoint(cancel: &AtomicBool, path: &str) -> Result<(), Error> { if cancel.load(Ordering::Acquire) { Err(error(ErrorKind::Cancelled, path, "Source command cancelled before publication")) } else { Ok(()) } }
fn tick(progress: &mut dyn FnMut(Progress), stage: Stage, completed: u64, total: Option<u64>) { progress(Progress { stage, completed, total }); }
fn bounded(path: &Path) -> Result<(), Error> { if path.to_string_lossy().encode_utf16().count() > MAXIMUM_PATH_UNITS { Err(error(ErrorKind::InvalidPath, &path.to_string_lossy(), "Physical path exceeds 256 UTF-16 units")) } else { Ok(()) } }

/// 🧭️ Checks the same portable relative spelling on every host.
pub fn validate_relative(path: &str) -> Result<(), Error> {
    if path.is_empty() || path.encode_utf16().count() > MAXIMUM_PATH_UNITS || path.chars().any(|c| c.is_control() || matches!(c, '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')) { return Err(error(ErrorKind::InvalidPath, path, "Portable relative source path required")); }
    for part in path.split('/') {
        let stem = part.split('.').next().unwrap_or("").to_ascii_uppercase();
        if part.is_empty() || matches!(part, "." | ".." | STATE) || part.ends_with(['.', ' ']) || matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL") || (stem.len() == 4 && (stem.starts_with("COM") || stem.starts_with("LPT")) && matches!(stem.as_bytes()[3], b'1'..=b'9')) { return Err(error(ErrorKind::InvalidPath, path, "Portable physical source component required")); }
    }
    Ok(())
}
/// 🔤️ Checks source encoding and extent without normalizing line endings.
pub fn validate_text(text: &str) -> Result<(), Error> {
    if text.len() > MAXIMUM_FILE_BYTES { return Err(error(ErrorKind::TooLarge, "", "Source exceeds 1 MiB")); }
    if text.contains('\0') { return Err(error(ErrorKind::NotText, "", "Binary source refused")); }
    Ok(())
}

impl Workspace {
    /// 🌱️ Binds an existing physical directory without following a root link.
    pub fn new(root: impl AsRef<Path>) -> Result<Self, Error> {
        let root = root.as_ref();
        if root.to_str().is_none_or(|s|s.chars().any(char::is_control)){return Err(error(ErrorKind::InvalidPath,"","Unicode physical workspace path without control characters required"));}
        let facts = fs::symlink_metadata(root).map_err(|e| io("", e))?;
        if !facts.is_dir() || facts.file_type().is_symlink() || native::reparse(&facts) { return Err(error(ErrorKind::Boundary, "", "Physical workspace root required")); }
        let root = fs::canonicalize(root).map_err(|e| io("", e))?;
        if root.to_str().is_none_or(|s|s.chars().any(char::is_control)){return Err(error(ErrorKind::InvalidPath,"","Unicode canonical workspace path required"));}
        bounded(&root)?;
        let anchor=Arc::new(native::directory(&root).map_err(|e|io("",e))?);
        Ok(Self { root, anchor })
    }

    fn physical(&self, path: &str, root_allowed: bool) -> Result<PathBuf, Error> {
        if path.is_empty() && root_allowed { return Ok(self.root.clone()); }
        validate_relative(path)?;
        let mut physical = self.root.clone();let mut relative=String::new();
        for part in path.split('/') {
            physical.push(part); bounded(&physical)?;
            if !relative.is_empty(){relative.push('/');}relative.push_str(part);
            let facts = native::open_in(&self.root,&self.anchor,&relative).and_then(|f|f.metadata()).map_err(|e|error(ErrorKind::Boundary,path,e))?;
            if facts.file_type().is_symlink() || native::reparse(&facts) { return Err(error(ErrorKind::Boundary, path, "Source links are not traversed")); }
        }
        Ok(physical)
    }

    /// 📋️ Lists one bounded physical directory with cancellable progress.
    pub fn list(&self, path: &str, cancel: &AtomicBool, progress: &mut dyn FnMut(Progress)) -> Result<Vec<Entry>, Error> {
        checkpoint(cancel, path)?;
        self.physical(path, true)?;
        let mut rows = Vec::new();
        let names=native::names(&self.root,&self.anchor,path,cancel).map_err(|e|io(path,e))?;
        if names.len()>MAXIMUM_DIRECTORY_ENTRIES{return Err(error(ErrorKind::TooManyEntries,path,"Directory exceeds 4096 entries"));}
        for name in names {
            checkpoint(cancel, path)?;
            if name == STATE { continue; }
            if rows.len() == MAXIMUM_DIRECTORY_ENTRIES { return Err(error(ErrorKind::TooManyEntries, path, "Directory exceeds 4096 entries")); }
            let child = if path.is_empty() { name.clone() } else { format!("{path}/{name}") };
            let facts=match native::open_in(&self.root,&self.anchor,&child).and_then(|f|f.metadata()){Ok(facts)=>facts,Err(_)=>continue};
            if facts.file_type().is_symlink() || native::reparse(&facts) { continue; }
            if !facts.is_file() && !facts.is_dir() { continue; }
            validate_relative(&child)?; bounded(&self.root.join(&child))?;
            rows.push(Entry { path: child, name, directory: facts.is_dir() });
            tick(progress, Stage::Listing, rows.len() as u64, None);
        }
        rows.sort_by(|a,b| b.directory.cmp(&a.directory).then(a.name.as_bytes().cmp(b.name.as_bytes())));
        checkpoint(cancel, path)?; tick(progress, Stage::Complete, rows.len() as u64, Some(rows.len() as u64));
        Ok(rows)
    }

    /// 📖️ Loads exact bytes and native identity from one no-follow descriptor.
    pub fn load(&self, path: &str, cancel: &AtomicBool, progress: &mut dyn FnMut(Progress)) -> Result<Document, Error> {
        checkpoint(cancel, path)?;
        let physical = self.physical(path, false)?;
        let (bytes, stamp) = self.read_physical(&physical, path, cancel, progress)?;
        let text = String::from_utf8(bytes.to_vec()).map_err(|_| error(ErrorKind::NotText, path, "Strict UTF-8 source required"))?;
        validate_text(&text)?;
        checkpoint(cancel, path)?;
        Ok(Document { path: path.into(), text, revision: Revision { root: self.root.clone(), path: path.into(), stamp, bytes } })
    }

    /// 🛟️ Loads an authenticated segment predecessor without granting source save authority.
    pub fn load_recovery(&self,path:&str,cancel:&AtomicBool,progress:&mut dyn FnMut(Progress))->Result<Document,Error>{checkpoint(cancel,path)?;self.load_retained(Path::new(path),cancel,progress)}

    fn read_physical(&self, physical: &Path, path: &str, cancel: &AtomicBool, progress: &mut dyn FnMut(Progress)) -> Result<(Arc<[u8]>, Stamp), Error> {
        let relative=physical.strip_prefix(&self.root).map_err(|_|error(ErrorKind::Boundary,path,"Source is outside its root"))?.to_string_lossy().replace('\\',"/");
        let mut file = native::open_in(&self.root,&self.anchor,&relative).map_err(|e| io(path, e))?;
        let before = native::stamp(&file).map_err(|e| io(path, e))?;
        if !file.metadata().map_err(|e| io(path, e))?.is_file() { return Err(error(ErrorKind::NotFile, path, "Regular source file required")); }
        if before.length > MAXIMUM_FILE_BYTES as u64 { return Err(error(ErrorKind::TooLarge, path, "Source exceeds 1 MiB")); }
        let mut bytes = Vec::with_capacity(before.length as usize);
        let mut chunk = [0u8; 16384];
        loop {
            checkpoint(cancel, path)?;
            let count = file.read(&mut chunk).map_err(|e| io(path, e))?;
            if count == 0 { break; }
            if bytes.len() + count > MAXIMUM_FILE_BYTES { return Err(error(ErrorKind::TooLarge, path, "Source grew beyond 1 MiB")); }
            bytes.extend_from_slice(&chunk[..count]); tick(progress, Stage::Reading, bytes.len() as u64, Some(before.length));
        }
        let after = native::stamp(&file).map_err(|e| io(path, e))?;
        let current = native::stamp(&native::open_in(&self.root,&self.anchor,&relative).map_err(|e| io(path, e))?).map_err(|e| io(path, e))?;
        if before != after || before != current || bytes.len() as u64 != before.length { return Err(error(ErrorKind::Conflict, path, "Source changed while reading")); }
        Ok((bytes.into(), before))
    }

    fn store(&self,cancel:&AtomicBool,progress:&mut dyn FnMut(Progress))->Result<Journal,Error>{self.history_store(cancel,progress)}

    /// 💾️ Publishes exact source bytes while retaining the actual exchanged predecessor.
    pub fn save(&self,path:&str,expected:&Revision,text:&str,cancel:&AtomicBool,progress:&mut dyn FnMut(Progress))->Result<Document,Error>{
        checkpoint(cancel,path)?;validate_text(text)?;if expected.root!=self.root||expected.path!=path{return Err(error(ErrorKind::Conflict,path,"Revision belongs to another document"));}self.physical(path,false)?;let journal=self.store(cancel,progress)?;let store=&journal.path;let anchor=&journal.anchor;if self.load(path,cancel,progress)?.revision!=*expected{return Err(error(ErrorKind::Conflict,path,"Source changed since load; preserve the draft or reload"));}
        let id=format!("{:x}-{:x}-{:x}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos(),SERIAL.fetch_add(1,Ordering::Relaxed));let stage_name=format!("{id}.new");let prior_name=format!("{id}.before");for name in [&stage_name,&prior_name,&format!("{id}.intent"),&format!("{id}.outcome")]{bounded(&store.join(name))?;}let relative=store.strip_prefix(&self.root).map_err(|_|error(ErrorKind::Boundary,path,"Rooted event segment required"))?.to_string_lossy().replace('\\',"/");let expected_retained=native::recovery_order(&stage_name,&prior_name)[0];let retained_relative=format!("{relative}/{expected_retained}");let mut staged=native::create_in(&store,&anchor,&stage_name,true).map_err(|e|io(path,e))?;let mut attempted=false;
        let result=(||{staged.set_permissions(native::open_in(&self.root,&self.anchor,path).and_then(|file|file.metadata()).map_err(|e|io(path,e))?.permissions()).map_err(|e|io(path,e))?;for chunk in text.as_bytes().chunks(16384){checkpoint(cancel,path)?;staged.write_all(chunk).map_err(|e|io(path,e))?;tick(progress,Stage::Staging,staged.metadata().map_err(|e|io(path,e))?.len(),Some(text.len()as u64));}staged.sync_all().map_err(|e|io(path,e))?;self.entry_event(&journal,&id,"intent","save",path,&retained_relative,false,Some(&expected.stamp),Some(&management::bytes_fingerprint(&expected.bytes)),"prepared")?;
            tick(progress,Stage::Checking,0,Some(1));checkpoint(cancel,path)?;if self.load(path,cancel,progress)?.revision!=*expected{return Err(error(ErrorKind::Conflict,path,"Source changed at final save checkpoint"));}tick(progress,Stage::Publishing,0,Some(1));checkpoint(cancel,path)?;if self.load(path,cancel,progress)?.revision!=*expected{return Err(error(ErrorKind::Conflict,path,"Source changed before publication"));}self.physical(path,false)?;checkpoint(cancel,path)?;attempted=true;
            let retained=native::publish(&self.root,&self.anchor,path,&store,&anchor,&stage_name,&prior_name).map_err(|e|io(path,e))?;let settled=AtomicBool::new(false);let prior=store.join(&retained);let(bytes,stamp)=self.read_physical(&prior,path,&settled,progress)?;let matches=bytes.as_ref()==expected.bytes.as_ref()&&stamp.identity==expected.stamp.identity&&stamp.modified==expected.stamp.modified;let destination=format!("{relative}/{retained}");self.entry_event(&journal,&id,"outcome","save",path,&destination,false,Some(&stamp),Some(&management::bytes_fingerprint(&bytes)),if matches{"committed"}else{"conflict"})?;if !matches{return Err(error(ErrorKind::Conflict,path,"Publication raced; inspect the actual retained predecessor"));}let document=self.load(path,&settled,progress)?;if document.text!=text{return Err(error(ErrorKind::Conflict,path,"A subsequent source edit is present; the predecessor remains retained"));}tick(progress,Stage::Complete,1,Some(1));Ok(document)})();
        if !attempted{drop(staged);let _=native::remove_in(&store,&anchor,&stage_name);}result.map_err(|mut failure|{if attempted&&failure.recovery_path.is_none(){for name in native::recovery_order(&stage_name,&prior_name){if native::open_in(&store,&anchor,name).and_then(|file|file.metadata()).is_ok_and(|facts|facts.is_file()){failure.recovery_path=Some(store.join(name).to_string_lossy().into_owned());break;}}}failure})
    }

}

#[path="📬️commands/🦀️.rs"]
pub mod commands;

#[path="🎮️management/🦀️.rs"]
mod management;

pub use management::history::{HistoryInspection,HistoryCursor,HistoryPage};
use management::history::Journal;

#[cfg(unix)]
mod native {
    use super::*;
    use std::ffi::{CString,c_void,c_char};
    use std::os::fd::{AsRawFd,FromRawFd,IntoRawFd};
    use std::os::unix::fs::{MetadataExt,OpenOptionsExt};
    #[cfg(target_os="macos")] const NOFOLLOW:i32=0x100;
    #[cfg(target_os="macos")] const DIRECTORY:i32=0x100000;
    #[cfg(target_os="macos")] const NONBLOCK:i32=4;
    #[cfg(target_os="macos")] const CREATE:i32=0x200;
    #[cfg(target_os="macos")] const EXCLUSIVE:i32=0x800;
    #[cfg(not(target_os="macos"))] const NOFOLLOW:i32=0x20000;
    #[cfg(not(target_os="macos"))] const DIRECTORY:i32=0x10000;
    #[cfg(not(target_os="macos"))] const NONBLOCK:i32=0x800;
    #[cfg(not(target_os="macos"))] const CREATE:i32=0x40;
    #[cfg(not(target_os="macos"))] const EXCLUSIVE:i32=0x80;
    unsafe extern "C" {
        fn openat(fd:i32,path:*const c_char,flags:i32,...)->i32;
        fn mkdirat(fd:i32,path:*const c_char,mode:u32)->i32;
        fn unlinkat(fd:i32,path:*const c_char,flags:i32)->i32;
        fn fcntl(fd:i32,command:i32,...)->i32;
        fn fdopendir(fd:i32)->*mut c_void;
        #[cfg_attr(all(target_os="macos",target_arch="x86_64"),link_name="readdir$INODE64")]
        fn readdir(directory:*mut c_void)->*mut u8;
        fn closedir(directory:*mut c_void)->i32;
    }
    #[cfg(target_os="macos")] unsafe extern "C" {fn renameatx_np(a:i32,p:*const c_char,b:i32,q:*const c_char,flags:u32)->i32;fn __error()->*mut i32;}
    #[cfg(not(target_os="macos"))] unsafe extern "C" {fn __errno_location()->*mut i32;}
    #[cfg(target_os="linux")] unsafe extern "C" {fn renameat2(a:i32,p:*const c_char,b:i32,q:*const c_char,flags:u32)->i32;}
    pub fn reparse(_:&Metadata)->bool{false}
    pub fn directory(path:&Path)->std::io::Result<File>{OpenOptions::new().read(true).custom_flags(DIRECTORY|NOFOLLOW).open(path)}
    fn opened(parent:&File,name:&str,flags:i32)->std::io::Result<File>{
        let name=CString::new(name)?;let fd=unsafe{openat(parent.as_raw_fd(),name.as_ptr(),flags,0o600u32)};
        if fd<0{return Err(std::io::Error::last_os_error());}let file=unsafe{File::from_raw_fd(fd)};
        if unsafe{fcntl(fd,2,1)}<0{return Err(std::io::Error::last_os_error());}Ok(file)
    }
    fn parent(anchor:&File,path:&str)->std::io::Result<(File,String)>{
        let mut pieces=path.split('/').collect::<Vec<_>>();let name=pieces.pop().unwrap_or("");
        if name.is_empty()||pieces.iter().chain(std::iter::once(&name)).any(|p|p.is_empty()||*p=="."||*p==".."){return Err(std::io::Error::other("Canonical anchored path required"));}
        let mut directory=anchor.try_clone()?;for part in pieces{directory=opened(&directory,part,DIRECTORY|NOFOLLOW)?;}Ok((directory,name.into()))
    }
    pub fn open_in(_root:&Path,anchor:&File,path:&str)->std::io::Result<File>{if path.is_empty(){return anchor.try_clone();}let(parent,name)=parent(anchor,path)?;opened(&parent,&name,NOFOLLOW|NONBLOCK)}
    pub fn create_in(_root:&Path,anchor:&File,name:&str,exclusive:bool)->std::io::Result<File>{let(parent,name)=parent(anchor,name)?;opened(&parent,&name,2|CREATE|NOFOLLOW|if exclusive{EXCLUSIVE}else{0})}
    pub fn state_directory(root:&Path,anchor:&File,name:&str)->std::io::Result<File>{let c=CString::new(name)?;if unsafe{mkdirat(anchor.as_raw_fd(),c.as_ptr(),0o700)}!=0{let e=std::io::Error::last_os_error();if e.kind()!=std::io::ErrorKind::AlreadyExists{return Err(e);}}let file=open_in(root,anchor,name)?;if !file.metadata()?.is_dir(){return Err(std::io::Error::other("Physical source event directory required"));}Ok(file)}
    pub fn remove_in(_root:&Path,anchor:&File,name:&str)->std::io::Result<()>{let(parent,name)=parent(anchor,name)?;let name=CString::new(name)?;if unsafe{unlinkat(parent.as_raw_fd(),name.as_ptr(),0)}==0{Ok(())}else{Err(std::io::Error::last_os_error())}}
    struct Directory(*mut c_void);
    impl Drop for Directory{fn drop(&mut self){unsafe{closedir(self.0);}}}
    unsafe fn errno()->*mut i32{#[cfg(target_os="macos")] {unsafe{__error()}} #[cfg(not(target_os="macos"))] {unsafe{__errno_location()}}}
    pub fn names(root:&Path,anchor:&File,path:&str,cancel:&AtomicBool)->std::io::Result<Vec<String>>{
        let file=if path.is_empty(){opened(anchor,".",DIRECTORY|NOFOLLOW)?}else{open_in(root,anchor,path)?};let descriptor=file.into_raw_fd();let handle=unsafe{fdopendir(descriptor)};
        if handle.is_null(){let failure=std::io::Error::last_os_error();let _file=unsafe{File::from_raw_fd(descriptor)};return Err(failure);}let directory=Directory(handle);let mut names=Vec::new();
        loop{
            if cancel.load(Ordering::Acquire){return Err(std::io::Error::new(std::io::ErrorKind::Interrupted,"Source listing cancelled"));}
            unsafe{*errno()=0;}let entry=unsafe{readdir(directory.0)};if entry.is_null(){if unsafe{*errno()}!=0{return Err(std::io::Error::last_os_error());}break;}
            let length=unsafe{std::ptr::read_unaligned(entry.add(16).cast::<u16>())}as usize;
            #[cfg(target_os="macos")] let offset=21;
            #[cfg(not(target_os="macos"))] let offset=19;
            if length<=offset||length>4096{return Err(std::io::Error::other("Bounded native directory entry required"));}
            let bytes=unsafe{std::slice::from_raw_parts(entry.add(offset),length-offset)};let end=bytes.iter().position(|b|*b==0).ok_or_else(||std::io::Error::other("Terminated native filename required"))?;
            let name=std::str::from_utf8(&bytes[..end]).map_err(|_|std::io::Error::other("Unicode filename required"))?;if name=="."||name==".."{continue;}names.push(name.into());if names.len()>MAXIMUM_DIRECTORY_ENTRIES{break;}
        }Ok(names)
    }
    pub fn stamp(file:&File)->std::io::Result<Stamp>{let m=file.metadata()?;Ok(Stamp{identity:(m.dev(),m.ino()),length:m.len(),modified:m.modified().ok(),change:(m.ctime(),m.ctime_nsec())})}
    pub fn recovery_order<'a>(stage:&'a str,prior:&'a str)->[&'a str;2]{[stage,prior]}
    pub fn publish(_root:&Path,anchor:&File,path:&str,_store:&Path,store:&File,stage:&str,_prior:&str)->std::io::Result<String>{
        let(parent,target)=parent(anchor,path)?;let(a,b)=(CString::new(stage)?,CString::new(target)?);
        #[cfg(target_os="macos")] let swapped=unsafe{renameatx_np(store.as_raw_fd(),a.as_ptr(),parent.as_raw_fd(),b.as_ptr(),2)};
        #[cfg(target_os="linux")] let swapped=unsafe{renameat2(store.as_raw_fd(),a.as_ptr(),parent.as_raw_fd(),b.as_ptr(),2)};
        #[cfg(not(any(target_os="macos",target_os="linux")))] let swapped=-1;
        if swapped!=0{return Err(std::io::Error::last_os_error());}store.sync_all()?;parent.sync_all()?;Ok(stage.into())
    }

    pub fn create_directory_in(_root:&Path,anchor:&File,path:&str)->std::io::Result<()>{let(parent,name)=parent(anchor,path)?;let c=CString::new(name)?;if unsafe{mkdirat(parent.as_raw_fd(),c.as_ptr(),0o700)}!=0{Err(std::io::Error::last_os_error())}else{Ok(())}}
    pub fn remove_empty_in(_root:&Path,anchor:&File,path:&str)->std::io::Result<()>{let(parent,name)=parent(anchor,path)?;let c=CString::new(name)?;#[cfg(target_os="macos")]let flags=0x80;#[cfg(not(target_os="macos"))]let flags=0x200;if unsafe{unlinkat(parent.as_raw_fd(),c.as_ptr(),flags)}==0{Ok(())}else{Err(std::io::Error::last_os_error())}}
    pub fn move_no_replace(_root:&Path,anchor:&File,path:&str,_target:&Path,target:&File,destination:&str)->std::io::Result<()>{
        let(source,name)=parent(anchor,path)?;let(target,to)=parent(target,destination)?;let(a,b)=(CString::new(name)?,CString::new(to)?);
        #[cfg(target_os="macos")]let result=unsafe{renameatx_np(source.as_raw_fd(),a.as_ptr(),target.as_raw_fd(),b.as_ptr(),4)};
        #[cfg(target_os="linux")]let result=unsafe{renameat2(source.as_raw_fd(),a.as_ptr(),target.as_raw_fd(),b.as_ptr(),1)};
        #[cfg(not(any(target_os="macos",target_os="linux")))]return Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"Qualified exclusive rename required"));
        #[cfg(any(target_os="macos",target_os="linux"))]if result==0{Ok(())}else{Err(std::io::Error::last_os_error())}
    }
    pub fn sync_parents(_root:&Path,anchor:&File,path:&str,_target:&Path,target:&File,destination:&str)->std::io::Result<()>{let(source,_)=parent(anchor,path)?;let(target,_)=parent(target,destination)?;source.sync_all()?;target.sync_all()}
}

#[cfg(windows)]
mod native {
    use super::*;
    use std::os::windows::{ffi::OsStrExt,fs::{MetadataExt,OpenOptionsExt},io::AsRawHandle};
    #[repr(C)] struct Info{attributes:u32,creation:[u32;2],access:[u32;2],write:[u32;2],volume:u32,size_high:u32,size_low:u32,links:u32,index_high:u32,index_low:u32}
    #[link(name="Kernel32")] unsafe extern "system"{
        fn SetFileInformationByHandle(file:*mut std::ffi::c_void,kind:u32,information:*const std::ffi::c_void,length:u32)->i32;
        fn GetFileInformationByHandle(file:*mut std::ffi::c_void,info:*mut Info)->i32;
        fn GetFinalPathNameByHandleW(file:*mut std::ffi::c_void,path:*mut u16,capacity:u32,flags:u32)->u32;
        fn ReplaceFileW(target:*const u16,stage:*const u16,prior:*const u16,flags:u32,exclude:*mut std::ffi::c_void,reserved:*mut std::ffi::c_void)->i32;
    }
    pub fn reparse(m:&Metadata)->bool{m.file_attributes()&0x400!=0}
    pub fn directory(path:&Path)->std::io::Result<File>{let file=OpenOptions::new().read(true).share_mode(3).custom_flags(0x02200000).open(path)?;let facts=file.metadata()?;if !facts.is_dir()||reparse(&facts){return Err(std::io::Error::other("Physical directory handle required"));}Ok(file)}
    fn actual(file:&File)->std::io::Result<PathBuf>{let mut wide=[0u16;257];let size=unsafe{GetFinalPathNameByHandleW(file.as_raw_handle(),wide.as_mut_ptr(),wide.len()as u32,0)};if size==0{return Err(std::io::Error::last_os_error());}if size as usize>=wide.len(){return Err(std::io::Error::other("Bounded native path required"));}use std::os::windows::ffi::OsStringExt;Ok(PathBuf::from(std::ffi::OsString::from_wide(&wide[..size as usize])))}
    struct Pin{path:PathBuf,_parents:Vec<File>}
    fn pin(anchor:&File,path:&str)->std::io::Result<Pin>{
        let mut physical=actual(anchor)?;let mut parents=vec![anchor.try_clone()?];let mut pieces=path.split('/').collect::<Vec<_>>();let name=pieces.pop().unwrap_or("");
        if name.is_empty()||pieces.iter().chain(std::iter::once(&name)).any(|p|p.is_empty()||*p=="."||*p==".."){return Err(std::io::Error::other("Canonical anchored path required"));}
        for part in pieces{physical.push(part);parents.push(directory(&physical)?);}physical.push(name);if physical.as_os_str().encode_wide().count()>MAXIMUM_PATH_UNITS{return Err(std::io::Error::other("Bounded physical path required"));}Ok(Pin{path:physical,_parents:parents})
    }
    pub fn open_in(_root:&Path,anchor:&File,path:&str)->std::io::Result<File>{if path.is_empty(){return anchor.try_clone();}let pinned=pin(anchor,path)?;let file=OpenOptions::new().read(true).share_mode(3).custom_flags(0x02200000).open(&pinned.path)?;if reparse(&file.metadata()?){return Err(std::io::Error::other("Physical source file required"));}Ok(file)}
    pub fn create_in(_root:&Path,anchor:&File,path:&str,exclusive:bool)->std::io::Result<File>{let pinned=pin(anchor,path)?;let mut options=OpenOptions::new();options.read(true).write(true).share_mode(7).custom_flags(0x00200000);if exclusive{options.create_new(true);}else{options.create(true).truncate(false);}let file=options.open(&pinned.path)?;if reparse(&file.metadata()?){return Err(std::io::Error::other("Physical event file required"));}Ok(file)}
    pub fn state_directory(_root:&Path,anchor:&File,name:&str)->std::io::Result<File>{let pinned=pin(anchor,name)?;match fs::create_dir(&pinned.path){Ok(())=>{},Err(e)if e.kind()==std::io::ErrorKind::AlreadyExists=>{},Err(e)=>return Err(e)}directory(&pinned.path)}
    pub fn remove_in(_root:&Path,anchor:&File,name:&str)->std::io::Result<()>{let pinned=pin(anchor,name)?;fs::remove_file(&pinned.path)}
    pub fn names(root:&Path,anchor:&File,path:&str,cancel:&AtomicBool)->std::io::Result<Vec<String>>{let file=open_in(root,anchor,path)?;let physical=actual(&file)?;let mut names=Vec::new();for entry in fs::read_dir(physical)?{if cancel.load(Ordering::Acquire){return Err(std::io::Error::new(std::io::ErrorKind::Interrupted,"Source listing cancelled"));}names.push(entry?.file_name().into_string().map_err(|_|std::io::Error::other("Unicode source filename required"))?);if names.len()>MAXIMUM_DIRECTORY_ENTRIES{break;}}Ok(names)}
    pub fn stamp(file:&File)->std::io::Result<Stamp>{let mut info=std::mem::MaybeUninit::<Info>::uninit();if unsafe{GetFileInformationByHandle(file.as_raw_handle(),info.as_mut_ptr())}==0{return Err(std::io::Error::last_os_error());}let info=unsafe{info.assume_init()};let m=file.metadata()?;if reparse(&m){return Err(std::io::Error::other("Physical file required"));}Ok(Stamp{identity:(info.volume as u64,((info.index_high as u64)<<32)|info.index_low as u64),length:m.len(),modified:m.modified().ok(),change:(0,0)})}
    pub fn recovery_order<'a>(stage:&'a str,prior:&'a str)->[&'a str;2]{[prior,stage]}
    pub fn publish(_root:&Path,anchor:&File,path:&str,_store:&Path,store:&File,stage:&str,prior:&str)->std::io::Result<String>{let target=pin(anchor,path)?;let staged=pin(store,stage)?;let backup=pin(store,prior)?;if fs::symlink_metadata(&backup.path).is_ok(){return Err(std::io::Error::other("New predecessor path required"));}let wide=|p:&Path|p.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<u16>>();let(a,b,c)=(wide(&target.path),wide(&staged.path),wide(&backup.path));if unsafe{ReplaceFileW(a.as_ptr(),b.as_ptr(),c.as_ptr(),0,std::ptr::null_mut(),std::ptr::null_mut())}==0{Err(std::io::Error::last_os_error())}else{Ok(prior.into())}}

    pub fn create_directory_in(_root:&Path,anchor:&File,path:&str)->std::io::Result<()>{let pinned=pin(anchor,path)?;fs::create_dir(&pinned.path)}
    pub fn remove_empty_in(_root:&Path,anchor:&File,path:&str)->std::io::Result<()>{let pinned=pin(anchor,path)?;fs::remove_dir(&pinned.path)}
    pub fn move_no_replace(_root:&Path,anchor:&File,path:&str,_target:&Path,target:&File,destination:&str)->std::io::Result<()>{
        #[repr(C)]struct Rename{replace:u32,root:*mut std::ffi::c_void,length:u32,name:[u16;1]}
        let source=pin(anchor,path)?;let destination=pin(target,destination)?;
        let file=OpenOptions::new().access_mode(0x10000|0x80).share_mode(3).custom_flags(0x82200000).open(&source.path)?;
        if reparse(&file.metadata()?){return Err(std::io::Error::other("Physical rename source required"));}
        let name=destination.path.as_os_str().encode_wide().collect::<Vec<_>>();let offset=std::mem::offset_of!(Rename,name);let length=offset+name.len()*2;
        let mut storage=vec![0usize;length.div_ceil(std::mem::size_of::<usize>())];let pointer=storage.as_mut_ptr().cast::<u8>();
        unsafe{std::ptr::write(pointer.cast::<Rename>(),Rename{replace:0,root:std::ptr::null_mut(),length:(name.len()*2)as u32,name:[0]});std::ptr::copy_nonoverlapping(name.as_ptr(),pointer.add(offset).cast::<u16>(),name.len());}
        if unsafe{SetFileInformationByHandle(file.as_raw_handle(),3,pointer.cast(),length as u32)}==0{Err(std::io::Error::last_os_error())}else{Ok(())}
    }
    pub fn sync_parents(_root:&Path,_anchor:&File,_path:&str,_target:&Path,_target_anchor:&File,_destination:&str)->std::io::Result<()>{Ok(())}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(not(any(unix,windows)))]
mod native {
    use super::*;
    fn unavailable<T>()->std::io::Result<T>{Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"Native source workspace requires a qualified host filesystem"))}
    pub fn reparse(_:&Metadata)->bool{false}
    pub fn directory(_:&Path)->std::io::Result<File>{unavailable()}
    pub fn open_in(_:&Path,_:&File,_:&str)->std::io::Result<File>{unavailable()}
    pub fn create_in(_:&Path,_:&File,_:&str,_:bool)->std::io::Result<File>{unavailable()}
    pub fn state_directory(_:&Path,_:&File,_:&str)->std::io::Result<File>{unavailable()}
    pub fn remove_in(_:&Path,_:&File,_:&str)->std::io::Result<()>{unavailable()}
    pub fn names(_:&Path,_:&File,_:&str,_:&AtomicBool)->std::io::Result<Vec<String>>{unavailable()}
    pub fn stamp(_:&File)->std::io::Result<Stamp>{unavailable()}
    pub fn recovery_order<'a>(stage:&'a str,prior:&'a str)->[&'a str;2]{[stage,prior]}
    pub fn publish(_:&Path,_:&File,_:&str,_:&Path,_:&File,_:&str,_:&str)->std::io::Result<String>{unavailable()}
    pub fn create_directory_in(_:&Path,_:&File,_:&str)->std::io::Result<()>{unavailable()}
    pub fn remove_empty_in(_:&Path,_:&File,_:&str)->std::io::Result<()>{unavailable()}
    pub fn move_no_replace(_:&Path,_:&File,_:&str,_:&Path,_:&File,_:&str)->std::io::Result<()>{unavailable()}
    pub fn sync_parents(_:&Path,_:&File,_:&str,_:&Path,_:&File,_:&str)->std::io::Result<()>{unavailable()}
}

#[cfg(test)]
#[path="🎮️management/🧪️tests/🦀️.rs"]
mod management_tests;

#[cfg(test)]
#[path="🎮️management/🕰️history/🧪️tests/🦀️.rs"]
mod history_tests;
