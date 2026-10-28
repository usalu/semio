//! 🪟️ Bounded read-only window selection borrows the exact ViewModel roster and caller-owned epoch.
use super::{ViewModel,ViewWindowInstance};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
#[derive(Clone,Copy,PartialEq,Eq)]
struct Source{view:usize,roster:usize,len:usize,target:usize,bytes:usize,epoch:u64}
/// 🧭️ No window data is cloned, moved or published by this selector.
#[derive(Default)]
pub struct ViewModelWindowSelection{source:Option<Source>,row:usize,offset:usize,current:Option<(usize,usize)>,selected:Option<usize>,complete:bool}
impl ViewModelWindowSelection{
 fn source(view:&ViewModel,target:&str,epoch:u64)->Source{Source{view:view as*const _ as usize,roster:view.window_instances.as_ptr()as usize,len:view.window_instances.len(),target:target.as_ptr()as usize,bytes:target.len(),epoch}}
 fn validate(&self,view:&ViewModel,target:&str,epoch:u64)->Result<(),ValueError>{if self.source.is_some_and(|original|original!=Self::source(view,target,epoch)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window selector changed its original ViewModel roster, target or epoch"))}if let Some((pointer,bytes))=self.current{let row=view.window_instances.get(self.row).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"window selector lost its original roster row"))?;if row.id.as_ptr()as usize!=pointer||row.id.len()!=bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window selector changed its original row identity"))}}Ok(())}
 /// 📏️ Selects at most sixty-four original identifier bytes for one exact funded comparison.
 pub fn demands(&self,view:&ViewModel,target:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{self.validate(view,target,epoch)?;if self.complete{return Ok(Default::default())}if self.source.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()})}let scalar=2*size_of::<usize>()+size_of::<Option<(usize,usize)>>();let count=view.window_instances.get(self.row).filter(|row|row.id.len()==target.len()).map_or(0,|_|target.len().saturating_sub(self.offset).min(64).min(body.saturating_sub(scalar)/2));Ok(RetirementDemand{copy_bytes:scalar+2*count,depth:1,..Default::default()})}
 /// 🎟️ A denied comparison leaves every cursor coordinate and original roster byte unchanged.
 pub fn advance(&mut self,view:&ViewModel,target:&str,epoch:u64,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.demands(view,target,epoch,body)?;if self.complete{return Ok(RetainedCloneStep::Complete(Default::default()))}if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.source.is_none(){self.source=Some(Self::source(view,target,epoch));}else if let Some(row)=view.window_instances.get(self.row){if row.id.len()!=target.len(){self.row+=1;self.offset=0;self.current=None;}else{let scalar=2*size_of::<usize>()+size_of::<Option<(usize,usize)>>();let count=(demand.copy_bytes-scalar)/2;if count==0&&self.offset<target.len(){return Ok(RetainedCloneStep::Progress(Default::default()))}self.current=Some((row.id.as_ptr()as usize,row.id.len()));let end=self.offset+count;if row.id.as_bytes()[self.offset..end]!=target.as_bytes()[self.offset..end]{self.row+=1;self.offset=0;self.current=None;}else{self.offset=end;if end==target.len(){self.selected=Some(self.row);self.complete=true;self.current=None;}}}}else{self.complete=true;self.current=None;}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};Ok(if self.complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})}
 /// 👁️ Borrows the matched original row only while the same roster, target and epoch remain current.
 pub fn selected<'a>(&self,view:&'a ViewModel,target:&str,epoch:u64)->Result<Option<&'a ViewWindowInstance>,ValueError>{self.validate(view,target,epoch)?;Ok(self.selected.and_then(|index|view.window_instances.get(index)))}
 pub fn complete(&self)->bool{self.complete}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
