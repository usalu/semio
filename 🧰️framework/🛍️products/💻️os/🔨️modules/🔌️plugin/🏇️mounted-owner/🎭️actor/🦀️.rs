//! 🎭️ Resumable actor capture retains exact partial text across denial and fallible shared birth.
use semio_framework_value::{SharedUtf8,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::controlled::ControlledRetirement};
/// 📏️ The shared Store protocol admits at most this original identity extent.
pub const MOUNTED_ACTOR_TEXT_BYTES:usize=256;
/// 🎟️ A cold owner preserves its bound source and every paid partial allocation until transfer or close.
pub struct ActorCapture{source:Option<(usize,usize)>,text:String,stage:u8,closing:bool}
impl Default for ActorCapture{fn default()->Self{Self::new()}}
impl ActorCapture{
 pub const fn new()->Self{Self{source:None,text:String::new(),stage:0,closing:false}}
 pub fn phase(&self)->&'static str{match self.stage{0=>"source",1=>"backing",2=>"text",3=>"shared",_=>"settlement"}}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.text.capacity()==0&&self.stage==0}
 pub fn next_demand(&self,source:&str)->Result<RetirementDemand,ValueError>{
  Ok(match self.stage{1=>RetirementDemand{capacity_bytes:source.len(),depth:1,..Default::default()},2=>RetirementDemand{copy_bytes:source.len(),depth:1,..Default::default()},3=>SharedUtf8::admission_demand(),_=>RetirementDemand{depth:1,..Default::default()}})
 }
 pub fn advance(&mut self,source:&str,output:&mut Option<ControlledRetirement<SharedUtf8>>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  if self.closing{return outcome(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"closing actor capture cannot resume publication")),0)}
  if output.is_some()&&self.terminal_is_empty(){return outcome(Ok(RetainedCloneStep::Complete(Default::default())),0)}
  let demand=match self.next_demand(source){Ok(d)=>d,Err(error)=>return outcome(Err(error),0)};
  if !funds(grant,demand){return outcome(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  let identity=(source.as_ptr()as usize,source.len());
  if self.source.is_some_and(|held|held!=identity){return outcome(Err(invalid("mounted actor changed its retained original source")),1)}
  let mut p=RetainedCloneProgress{copied_items:1,..Default::default()};
  match self.stage{
   0=>{if source.is_empty()||source.len()>MOUNTED_ACTOR_TEXT_BYTES{return outcome(Err(invalid("mounted actor exceeds its fixed original identity capacity")),1)}self.source=Some(identity);self.stage=1;}
   1=>{if self.text.try_reserve_exact(source.len()).is_err(){return outcome(Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"mounted actor text backing allocation refused")),1)}p.retained_capacity_bytes=self.text.capacity();self.stage=2;if !p.fits(grant){return outcome(Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mounted actor text backing exceeded its actual quote").with_retained_progress(p)),1)}}
   2=>{self.text.push_str(source);p.copied_bytes=source.len();self.stage=3;}
   3=>{let turn=SharedUtf8::admit(std::mem::take(&mut self.text),grant);return outcome(match turn.result{Ok((owner,p))=>{*output=Some(ControlledRetirement::retain(owner));self.stage=4;Ok(RetainedCloneStep::Progress(p))},Err((error,text))=>{self.text=text;Err(error)}},turn.work)}
   _=>{self.source=None;self.stage=0;return outcome(Ok(RetainedCloneStep::Complete(p)),1)}
  }
  outcome(Ok(RetainedCloneStep::Progress(p)),1)
 }
 pub fn close_demand(&self)->RetirementDemand{if self.terminal_is_empty(){Default::default()}else{RetirementDemand{release_bytes:self.text.capacity(),depth:1,..Default::default()}}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  if self.terminal_is_empty(){return outcome(Ok(RetainedCloneStep::Complete(Default::default())),0)}
  if !funds(grant,self.close_demand()){return outcome(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  self.closing=true;let mut p=RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.text.capacity()>0{p.released_bytes=self.text.capacity();drop(std::mem::take(&mut self.text));return outcome(Ok(RetainedCloneStep::Progress(p)),1)}
  self.source=None;self.stage=0;outcome(Ok(RetainedCloneStep::Complete(p)),1)
 }
}
impl Drop for ActorCapture{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"partial actor capture requires paid close");}}
fn funds(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn outcome(result:Result<RetainedCloneStep,ValueError>,work:u64)->RetainedWorkOutcome<RetainedCloneStep>{RetainedWorkOutcome{result,work}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod work_tests;
