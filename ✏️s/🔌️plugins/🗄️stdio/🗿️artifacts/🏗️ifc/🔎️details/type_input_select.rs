//! 🌿️ Borrowed SELECT traversal preserves released depth-first leaf ordinals.
use super::*;
#[derive(Clone,Copy,Default)]
struct Frame { name:Option<&'static str>,next:usize,count:usize }
#[derive(Clone,Copy)]
struct Walk { frames:[Frame;64],depth:usize,root:Option<&'static str>,next:usize,complete:bool }
impl Default for Walk {fn default()->Self{Self{frames:[Frame::default();64],depth:0,root:None,next:0,complete:false}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(super) enum Step { Pending,Leaf{index:usize,name:&'static str},Complete{count:usize} }
fn invalid()->semio_framework_value::ValueError{semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"IFC SELECT leaf metadata unavailable")}
impl Walk {
 fn advance<C:IfcDetailsProfile>(&mut self,root:&'static str)->Result<Step,semio_framework_value::ValueError>{
  if root.len()>256||self.root.is_some_and(|name|name!=root){return Err(invalid());}
  if self.complete{return Ok(Step::Complete{count:self.next});}
  if self.root.is_none(){let count=C::select_count(root).filter(|count|*count>0).ok_or_else(invalid)?;self.frames[0]=Frame{name:Some(root),next:0,count};self.depth=1;self.root=Some(root);return Ok(Step::Pending);}
  let frame=self.frames.get(self.depth.checked_sub(1).ok_or_else(invalid)?).copied().ok_or_else(invalid)?;
  if frame.next==frame.count{self.depth-=1;self.frames[self.depth]=Frame::default();if self.depth==0{self.complete=true;return Ok(Step::Complete{count:self.next});}return Ok(Step::Pending);}
  let name=C::select_member(frame.name.ok_or_else(invalid)?,frame.next).ok_or_else(invalid)?;
  if name.len()>256{return Err(invalid());}
  if let Some(count)=C::select_count(name){if count==0||self.depth==self.frames.len()||self.frames[..self.depth].iter().any(|frame|frame.name==Some(name)){return Err(invalid());}self.frames[self.depth-1].next+=1;self.frames[self.depth]=Frame{name:Some(name),next:0,count};self.depth+=1;return Ok(Step::Pending);}
  let next=self.next.checked_add(1).ok_or_else(invalid)?;let index=self.next;self.frames[self.depth-1].next+=1;self.next=next;Ok(Step::Leaf{index,name})
 }
}
#[derive(Clone,Copy,Default)]
pub(super) struct Cursor { walk:Walk,previous:Walk,pending:Option<(usize,&'static str)> }
impl Cursor {
 pub(super) fn clear(&mut self){*self=Self::default();}
 pub(super) fn advance<C:IfcDetailsProfile>(&mut self,root:&'static str)->Result<Step,semio_framework_value::ValueError>{
  if self.walk.root.is_some_and(|name|name!=root){return Err(invalid());}
  if let Some((index,name))=self.pending{match self.previous.advance::<C>(root)?{Step::Leaf{index:prior,name:other}=>{if prior<index&&other==name{return Err(invalid());}if prior==index{self.pending=None;self.previous=Walk::default();return Ok(Step::Leaf{index,name});}},Step::Complete{..}=>return Err(invalid()),Step::Pending=>{}}return Ok(Step::Pending);}
  match self.walk.advance::<C>(root)?{Step::Leaf{index:0,name}=>Ok(Step::Leaf{index:0,name}),Step::Leaf{index,name}=>{self.pending=Some((index,name));Ok(Step::Pending)},answer=>Ok(answer)}
 }
}
