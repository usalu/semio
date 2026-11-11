//! 🖼️ One explicit scope copy cursor owns every vector, string and partial output through paid transfer or close.
use super::*;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_value::RetainedWorkOutcome;
/// 🧵️ Source references live only for a synchronous step; the parent keeps its source immutable.
pub struct MountedUiCopy{output:Option<UiDirtyScope>,phase:u8,index:usize,close:Option<ControlledRetirement<UiDirtyScope>>}
impl MountedUiCopy{
 pub const fn new()->Self{Self{output:None,phase:0,index:0,close:None}}
 pub fn ready(&self)->bool{self.phase==5&&self.output.is_some()}
 pub fn terminal_is_empty(&self)->bool{self.output.is_none()&&self.close.is_none()&&self.phase==0}
 fn metadata()->usize{size_of::<u8>()+size_of::<usize>()}
 fn list<'a>(scope:&'a UiDirtyScope,panel:bool)->&'a[String]{match scope{UiDirtyScope::Partial{window_bodies,panel_bodies,..}=>if panel{panel_bodies}else{window_bodies},_=>&[]}}
 fn list_mut(scope:&mut UiDirtyScope,panel:bool)->&mut Vec<String>{match scope{UiDirtyScope::Partial{window_bodies,panel_bodies,..}=>if panel{panel_bodies}else{window_bodies},_=>unreachable!("partial clone phase")}}
 pub fn demands(&self,source:&UiDirtyScope,body:usize)->Result<RetirementDemand,ValueError>{
  let mut d=RetirementDemand{copy_bytes:Self::metadata(),depth:1,..Default::default()};
  if self.phase==0{d.copy_bytes+=2*size_of::<UiDirtyScope>()+size_of::<Option<UiDirtyScope>>();return Ok(d)}
  if self.ready(){return Ok(Default::default())}
  let panel=self.phase>=3;let source=Self::list(source,panel);let target=Self::list(self.output.as_ref().expect("scope clone owns its output"),panel);
  if self.phase==1||self.phase==3{d.copy_bytes+=size_of::<Vec<String>>();d.capacity_bytes=source.len().checked_mul(size_of::<String>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"scope vector quote overflow"))?;}
  else if self.index<source.len(){
   if target.len()==self.index{d.copy_bytes+=2*size_of::<String>()+size_of::<Vec<String>>();}
   else{let text=&target[self.index];let original=&source[self.index];if text.capacity()==0&&!original.is_empty(){d.copy_bytes+=size_of::<String>();d.capacity_bytes=original.len();}
    else if text.len()<original.len(){let mut count=original.len().saturating_sub(text.len()).min(body.saturating_sub(d.copy_bytes+size_of::<String>())).min(2048);while count>0&&!original.is_char_boundary(text.len()+count){count-=1}if count==0{return Ok(RetirementDemand{copy_bytes:Self::metadata()+size_of::<String>()+4,depth:1,..Default::default()})}d.copy_bytes+=size_of::<String>()+count;}
   }
  }Ok(d)
 }
 pub fn advance(&mut self,source:&UiDirtyScope,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  if self.ready(){return RetainedWorkOutcome{work:0,result:Ok(RetainedCloneStep::Complete(Default::default()))}}
  let d=match self.demands(source,grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error)}};
  if !plugin_grant_funds(d,grant)||grant.maximum_items==0{return RetainedWorkOutcome{work:0,result:Ok(RetainedCloneStep::Progress(Default::default()))}}
  let action=(||{
   if self.phase==0{self.output=Some(match source{UiDirtyScope::Full=>UiDirtyScope::Full,UiDirtyScope::None=>UiDirtyScope::None,UiDirtyScope::Partial{utilities,tools,engagements,measures,labels,..}=>UiDirtyScope::Partial{window_bodies:Vec::new(),panel_bodies:Vec::new(),utilities:*utilities,tools:*tools,engagements:*engagements,measures:*measures,labels:*labels}});self.phase=if matches!(source,UiDirtyScope::Partial{..}){1}else{5};}
   else{let panel=self.phase>=3;let original=Self::list(source,panel);let target=Self::list_mut(self.output.as_mut().unwrap(),panel);
    if self.phase==1||self.phase==3{target.try_reserve_exact(original.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"scope vector allocation refused"))?;self.phase+=1;}
    else if self.index==original.len(){self.index=0;self.phase=if panel{5}else{3};}
    else if target.len()==self.index{target.push(String::new());}
    else{let text=&mut target[self.index];let original=&original[self.index];if text.capacity()==0&&!original.is_empty(){text.try_reserve_exact(original.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"scope text allocation refused"))?;}
     else if text.len()==original.len(){self.index+=1;}
     else{let count=d.copy_bytes-Self::metadata()-size_of::<String>();text.push_str(&original[text.len()..text.len()+count]);}
    }
   }Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))
  })();RetainedWorkOutcome{work:1,result:action}
 }
 pub fn transfer_demands()->RetirementDemand{RetirementDemand{copy_bytes:4*size_of::<UiDirtyScope>()+2*size_of::<Option<UiDirtyScope>>()+Self::metadata(),depth:1,..Default::default()}}
 pub fn take(&mut self)->UiDirtyScope{assert!(self.ready());self.phase=0;self.index=0;self.output.take().unwrap()}
 pub fn restore(&mut self,scope:UiDirtyScope){assert!(self.output.is_none());self.output=Some(scope);self.phase=5;}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<UiDirtyScope>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)}}
  Ok(RetirementDemand{copy_bytes:2*size_of::<Option<UiDirtyScope>>()+size_of::<Option<ControlledRetirement<UiDirtyScope>>>()+Self::metadata(),depth:1,..Default::default()})
 }
 pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{
  let grant=effect.grant();
  if self.terminal_is_empty(){return RetainedWorkOutcome{work:0,result:Ok(RetainedCloneStep::Complete(Default::default()))}}
  let d=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error)}};
  if !plugin_grant_funds(d,grant)||grant.maximum_items==0{return RetainedWorkOutcome{work:0,result:Ok(RetainedCloneStep::Progress(Default::default()))}}
  if let Some(owner)=self.close.as_mut(){if !owner.terminal_is_empty(){return semio_plugin_ownership::bounded_ownership::plugin_poll_outcome(owner.step(grant.maximum_copy_bytes,effect))}self.close=None;}
  else{self.phase=0;self.index=0;if let Some(output)=self.output.take(){self.close=Some(ControlledRetirement::retain(output));}}
  RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 }
}
impl Drop for MountedUiCopy{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"scope copy requires original paid handoff or close")}}
