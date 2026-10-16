//! 🎟️ Original built nodes keep an inline root and fund exact native traversal and backing retirement.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};

const ORIGINAL_BUILT_DEPTH:usize=typed::UI_TYPED_RETIREMENT_DEPTH*2;
enum OriginalBuiltNode{Root(BuiltNode),Child(Box<BuiltNode>)}
impl OriginalBuiltNode{
 fn node(&self)->&BuiltNode{match self{Self::Root(node)=>node,Self::Child(node)=>node}}
 fn node_mut(&mut self)->&mut BuiltNode{match self{Self::Root(node)=>node,Self::Child(node)=>node}}
 fn release_bytes(&self)->usize{match self{Self::Root(_)=>0,Self::Child(_)=>std::mem::size_of::<BuiltNode>()}}
}

pub struct OriginalBuiltTreeRetirement{
 node:Option<OriginalBuiltNode>,
 pages:UiFixedList<BuiltChildrenIntoIter,UI_BUILT_CHILD_RETIRE_SLOTS>,
 pending:Option<BuiltChildrenIntoIter>,
 field:u8,
 cursor:UiTypedRetirementCursor,
}
impl OriginalBuiltTreeRetirement{
 pub fn new(root:BuiltNode)->Self{Self{node:Some(OriginalBuiltNode::Root(root)),pages:Default::default(),pending:None,field:0,cursor:UiTypedRetirementCursor::empty()}}
 pub fn terminal_is_empty(&self)->bool{self.node.is_none()&&self.pending.is_none()&&self.pages.terminal_is_empty()}
 pub fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{
  let mut demand=RetirementDemand{depth:ORIGINAL_BUILT_DEPTH,..Default::default()};
  if self.terminal_is_empty(){return Ok(Default::default())}
  if self.pending.is_some(){if !self.pages.has_reserved_slot(){demand.capacity_bytes=self.pages.next_allocation_bytes().map_err(|error|invalid(error.reason))?;}return Ok(demand)}
  if let Some(owned)=self.node.as_ref(){let node=owned.node();demand.release_bytes=match self.field{
   0=>self.cursor.original_release_demand(&node.key),1=>self.cursor.original_release_demand(&node.component),2=>self.cursor.original_release_demand(&node.layout),3=>self.cursor.original_release_demand(&node.style),4=>self.cursor.original_release_demand(&node.activity),5=>self.cursor.original_release_demand(&node.disabled),6=>self.cursor.original_release_demand(&node.accessibility),7=>self.cursor.original_release_demand(&node.bindings),8=>self.cursor.original_release_demand(&node.menu),9|10=>Ok(0),_=>Ok(owned.release_bytes()),
  }.map_err(invalid)?;return Ok(demand)}
  demand.release_bytes=if let Some(page)=self.pages.len().checked_sub(1).and_then(|index|self.pages.get(index)){page.original_release_demand()}else{self.pages.next_release_allocation_bytes().map_err(|error|invalid(error.reason))?};
  Ok(demand)
 }
 pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.next_demand(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.pending.is_some(){
   if !self.pages.has_reserved_slot(){let step=self.pages.try_reserve_one(grant.maximum_capacity_bytes).map_err(|error|ValueError::literal(ValueRefusalKind::AllocationFailed,error.reason).with_retained_progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()}))}
   if let Err(page)=self.pages.try_push_reserved(self.pending.take().unwrap()){self.pending=Some(page);return Err(invalid("original built traversal lost its reserved page slot"))}
   return Ok(metadata())
  }
  if let Some(owned)=self.node.as_mut(){
   let node=owned.node_mut();
   if self.field<9{
    let body=grant.maximum_copy_bytes.max(1);
    let step=match self.field{0=>self.cursor.advance(&mut node.key,1,body),1=>self.cursor.advance(&mut node.component,1,body),2=>self.cursor.advance(&mut node.layout,1,body),3=>self.cursor.advance(&mut node.style,1,body),4=>self.cursor.advance(&mut node.activity,1,body),5=>self.cursor.advance(&mut node.disabled,1,body),6=>self.cursor.advance(&mut node.accessibility,1,body),7=>self.cursor.advance(&mut node.bindings,1,body),8=>self.cursor.advance(&mut node.menu,1,body),_=>unreachable!()}.map_err(invalid)?;
    if step.complete{self.cursor=UiTypedRetirementCursor::empty();self.field+=1;}
    return Ok(RetainedCloneStep::Progress(if step.progressed{RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}}else{Default::default()}))
   }
   if self.field<11{let children=if self.field==9{&mut node.rejected_children}else{&mut node.children};if children.capacity()!=0{self.pending=Some(std::mem::take(children).into_iter());}self.field+=1;return Ok(metadata())}
   self.node=None;self.field=0;self.cursor=UiTypedRetirementCursor::empty();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))
  }
  if let Some(page)=self.pages.last_mut(){
   if page.original_terminal_is_empty(){self.pages.pop();return Ok(metadata())}
   return match page.try_next_or_release().map_err(invalid)?{BuiltChildRetirementNext::Pending=>Ok(RetainedCloneStep::Progress(Default::default())),BuiltChildRetirementNext::Node(node)=>{self.node=Some(OriginalBuiltNode::Child(node));Ok(metadata())},BuiltChildRetirementNext::Complete=>Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))}
  }
  let step=self.pages.release_empty_page(grant.maximum_release_bytes).map_err(|error|invalid(error.reason))?;
  let progress=RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()};
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
}
impl Drop for OriginalBuiltTreeRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original built tree dropped before full paid closure");}}
fn metadata()->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
