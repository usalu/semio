pub use dashboard_labels::DashboardLabels;
use dashboard_labels::labels;
pub use semio_framework_ui_tui_terminal::backend::TerminalBackend;
pub use ui_locale::Locale;
use std::collections::VecDeque;
use std::io;

pub const MAXIMUM_EVENTS:usize=64;
pub const MAXIMUM_DETAIL_BYTES:usize=8192;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Operation {Enter,Present,Wait,Copy,Leave,Detach,Restart,Stop,Kill,Input,Attach,Shutdown,Start,Preferences,Discovery,Source}

impl Operation {
    pub fn key(self)->&'static str {match self {Self::Enter=>"enter",Self::Present=>"present",Self::Wait=>"wait",Self::Copy=>"copy",Self::Leave=>"leave",Self::Detach=>"detach",Self::Restart=>"restart",Self::Stop=>"stop",Self::Kill=>"kill",Self::Input=>"input",Self::Attach=>"attach",Self::Shutdown=>"shutdown",Self::Start=>"start",Self::Preferences=>"preferences",Self::Discovery=>"discovery",Self::Source=>"source"}}
    pub fn label(self,text:&DashboardLabels)->&str {match self {Self::Enter=>text.local_enter,Self::Present=>text.local_present,Self::Wait=>text.local_wait,Self::Copy=>text.local_copy,Self::Leave=>text.local_leave,Self::Detach=>text.local_detach,Self::Restart=>text.local_restart,Self::Stop=>text.local_stop,Self::Kill=>text.local_kill,Self::Input=>text.local_input,Self::Attach=>text.local_attach,Self::Shutdown=>text.local_shutdown,Self::Start=>text.local_start,Self::Preferences=>text.local_preferences,Self::Discovery=>text.local_discovery,Self::Source=>text.local_source}.as_str()}
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Failure {pub operation:Operation,pub kind:io::ErrorKind,pub os_code:Option<i32>,pub detail:String}

impl Failure {
    pub fn io(operation:Operation,error:io::Error)->Self {let mut value=Self::text(operation,error.to_string());value.kind=error.kind();value.os_code=error.raw_os_error();value}
    pub fn text(operation:Operation,mut detail:String)->Self {let mut end=detail.len().min(MAXIMUM_DETAIL_BYTES);while !detail.is_char_boundary(end){end-=1;}detail.truncate(end);Self{operation,kind:io::ErrorKind::Other,os_code:None,detail}}
    pub fn display(&self,text:&DashboardLabels)->String {let detail=if self.kind==io::ErrorKind::NotConnected{text.local_disconnected.as_str()}else{&self.detail};text.local_failure.fill(&[("operation",self.operation.label(text)),("detail",detail)]).into_string()}
}

#[derive(Default)]
pub struct History {events:VecDeque<Failure>}

impl History {
    pub fn push(&mut self,failure:Failure){if self.events.len()==MAXIMUM_EVENTS{self.events.pop_front();}self.events.push_back(failure);}
    pub fn events(&self)->impl DoubleEndedIterator<Item=&Failure>+ExactSizeIterator{self.events.iter()}
    pub fn get(&self,index:usize)->Option<&Failure>{self.events.get(index)}
    pub fn len(&self)->usize{self.events.len()}
}

#[derive(Debug)]
pub struct TerminalFailure {pub primary:Failure,pub cleanup:Vec<Failure>}

impl TerminalFailure {
    pub fn display(&self,locale:Option<Locale>)->String {
        let render=|locale|{let text=labels(locale);let mut value=self.primary.display(&text);for cleanup in &self.cleanup{value.push('\n');value.push_str(&text.local_cleanup.fill(&[("detail",&cleanup.display(&text))]).into_string());}value};
        locale.map(render).unwrap_or_else(||dashboard_preferences::language::declared().choices().iter().map(|choice|render(Locale::parse(&choice.language).expect("validated language"))).collect::<Vec<_>>().join("\n"))
    }
}

impl From<Failure> for TerminalFailure {fn from(primary:Failure)->Self{Self{primary,cleanup:Vec::new()}}}
/// 🛟️ Settles terminal cleanup once and preserves the first failure and all bounded cleanup failures.
pub fn terminal_session<B:TerminalBackend>(terminal:&mut B,run:impl FnOnce(&mut B)->Result<(),TerminalFailure>)->Result<(),TerminalFailure>{
    let mut result=match terminal.enter(){Ok(())=>run(terminal),Err(error)=>Err(Failure::io(Operation::Enter,error).into())};
    if let Err(error)=terminal.leave(){let cleanup=Failure::io(Operation::Leave,error);match &mut result{Err(failure)=>failure.cleanup.push(cleanup),Ok(())=>result=Err(cleanup.into())}}
    result
}
