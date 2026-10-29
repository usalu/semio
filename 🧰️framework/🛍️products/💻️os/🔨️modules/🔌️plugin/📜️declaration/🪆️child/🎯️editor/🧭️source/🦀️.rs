//! 🧭️ Exact immutable command, parent and member reads qualify one registered editor source without cloning content.
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
#[derive(Clone,Copy)]
pub struct ChildEditorFields<'a>{pub parent_artifact:&'a str,pub owner:&'a str,pub slot:&'a str,pub child_id:&'a str,pub artifact_id:&'a str,pub artifact_kind:&'a str,pub standard:&'a str,pub subset:&'a str,pub editor_id:&'a str,pub command_id:&'a str,pub parent_revision:&'a[u8;32],pub child_revision:&'a[u8;32]}
impl ChildEditorFields<'_>{fn text(&self,field:usize)->&str{match field{0=>self.parent_artifact,1=>self.owner,2=>self.slot,3=>self.child_id,4=>self.artifact_id,5=>self.artifact_kind,6=>self.standard,7=>self.subset,8=>self.editor_id,9=>self.command_id,_=>unreachable!("child editor field")}}}
pub struct ChildEditorCapability{pub artifact_kind:&'static str,pub standard:&'static str,pub subset:&'static str,pub editor_id:&'static str,pub command_id:&'static str}
impl ChildEditorCapability{fn text(&self,field:usize)->&str{match field{0=>self.artifact_kind,1=>self.standard,2=>self.subset,3=>self.editor_id,4=>self.command_id,_=>unreachable!("child editor capability field")}}}
pub struct ChildEditorRequest<'a>{pub identity:usize,pub expected_reference_identity:usize,pub fields:ChildEditorFields<'a>}
pub struct ChildEditorCurrent<'a>{pub parent_identity:usize,pub child_identity:usize,pub fields:ChildEditorFields<'a>,pub capability:&'static ChildEditorCapability}
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct ChildEditorStamp{command:usize,expected:usize,parent:usize,child:usize,capability:usize,parent_revision:[u8;32],child_revision:[u8;32]}
impl ChildEditorStamp{pub fn command_identity(&self)->usize{self.command}pub fn expected_reference_identity(&self)->usize{self.expected}pub fn parent_identity(&self)->usize{self.parent}pub fn child_identity(&self)->usize{self.child}pub fn capability_identity(&self)->usize{self.capability}pub fn parent_revision(&self)->&[u8;32]{&self.parent_revision}pub fn child_revision(&self)->&[u8;32]{&self.child_revision}}
pub enum ChildEditorSourceStep{Pending,Ready(ChildEditorStamp)}
#[derive(Clone,Copy,Default)]
struct State{phase:u8,field:usize,offset:usize,command:usize,expected:usize,parent:usize,child:usize,capability:usize,bound:bool,progress:usize}
#[derive(Clone,Copy)]
pub struct ChildEditorSourceCursor{state:State,last:RetainedCloneProgress}
fn refusal(code:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,code)}
fn compare(a:&str,b:&str,offset:&mut usize)->Result<bool,ValueError>{if a.len()!=b.len(){return Err(refusal("child editor identity differs"));}let left=a[*offset..].chars().next();if left!=b[*offset..].chars().next(){return Err(refusal("child editor identity differs"));}if let Some(point)=left{*offset+=point.len_utf8();Ok(false)}else{*offset=0;Ok(true)}}
impl ChildEditorSourceCursor{
 pub fn new()->Self{Self{state:Default::default(),last:Default::default()}}
 pub fn progress(&self)->usize{self.state.progress}
 pub fn last_receipt(&self)->RetainedCloneProgress{self.last}
 pub fn next_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<State>()+if self.state.phase==5{std::mem::size_of::<ChildEditorStamp>()}else{0},depth:1,..Default::default()}}
 pub fn advance(&mut self,request:&ChildEditorRequest<'_>,current:&ChildEditorCurrent<'_>,grant:RetainedCloneGrant)->Result<ChildEditorSourceStep,ValueError>{
  self.last=Default::default();if self.state.phase>=6{return Err(refusal("child editor source cancelled"));}let demand=self.next_demand();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(ChildEditorSourceStep::Pending);}let capability=current.capability as*const ChildEditorCapability as usize;
  if self.state.bound&&(self.state.command!=request.identity||self.state.expected!=request.expected_reference_identity||self.state.parent!=current.parent_identity||self.state.child!=current.child_identity||self.state.capability!=capability){return Err(refusal("child editor original source changed"));}
  let mut stamp=None;match self.state.phase{
   0=>{if request.identity==0||request.expected_reference_identity==0||current.parent_identity==0||current.child_identity==0{return Err(refusal("child editor original source missing"));}self.state.command=request.identity;self.state.expected=request.expected_reference_identity;self.state.parent=current.parent_identity;self.state.child=current.child_identity;self.state.capability=capability;self.state.bound=true;self.state.phase=1;},
   1=>{if compare(request.fields.text(self.state.field),current.fields.text(self.state.field),&mut self.state.offset)?{self.state.field+=1;if self.state.field==10{self.state.phase=2;self.state.field=0;}}},
   2|3=>{let(a,b)=if self.state.phase==2{(request.fields.parent_revision,current.fields.parent_revision)}else{(request.fields.child_revision,current.fields.child_revision)};if a[self.state.offset]!=b[self.state.offset]{return Err(refusal("child editor revision differs"));}self.state.offset+=1;if self.state.offset==32{self.state.offset=0;self.state.phase+=1;}},
   4=>{if compare(current.fields.text(self.state.field+5),current.capability.text(self.state.field),&mut self.state.offset)?{self.state.field+=1;if self.state.field==5{self.state.phase=5;}}},
   5=>stamp=Some(ChildEditorStamp{command:self.state.command,expected:self.state.expected,parent:self.state.parent,child:self.state.child,capability:self.state.capability,parent_revision:*current.fields.parent_revision,child_revision:*current.fields.child_revision}),_=>unreachable!("child editor source phase")}
  self.state.progress+=1;self.last=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};Ok(stamp.map_or(ChildEditorSourceStep::Pending,ChildEditorSourceStep::Ready))
 }
 pub fn cancel(&mut self){self.state.phase=6;}
 pub fn close_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<State>(),depth:1,..Default::default()}}
 pub fn close_one(&mut self,grant:RetainedCloneGrant)->bool{self.last=Default::default();if self.state.phase==7{return true;}if grant.maximum_items==0||grant.maximum_copy_bytes<std::mem::size_of::<State>()||grant.maximum_depth<1{return false;}self.state=State{phase:7,..Default::default()};self.last=RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<State>(),..Default::default()};true}
 pub fn terminal_is_empty(&self)->bool{self.state.phase==7}
}
impl Default for ChildEditorSourceCursor{fn default()->Self{Self::new()}}
semio_framework_value::artifact_retire_leaf!(ChildEditorStamp,State,ChildEditorSourceCursor);
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
