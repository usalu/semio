//! 🛂️ Native semantic authority validates original fields through retained immutable projections.
use crate::{Edit,HybridLogicalTimestamp};
use semio_framework_pack_json::ArtifactCanonicalJsonText;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,paged::Utf8Text,retained_clone::{RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;

type Text<'a>=&'a(dyn Utf8Text+Sync);
/// 🪪️ Exposes only original fixed metadata and native text fields of the retained publication authority.
pub trait ArtifactCanonicalEditAuthority:Sync+'static{
 fn sequence_number(&self)->i32;
 fn clock(&self)->HybridLogicalTimestamp;
 fn actor(&self)->Text<'_>;
 fn line(&self)->Option<Text<'_>>;
 fn group(&self)->Option<Text<'_>>;
}
fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn copy(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn admitted(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items>0&&demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes&&demand.depth<=grant.maximum_depth}
fn projection_demand<T:?Sized+Sync>(owner:&RetainedOwnedProjection<T>)->Result<RetirementDemand,ValueError>{let work=owner.next_close_copy_byte_demand()?;Ok(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_close_capacity_byte_demand(work)?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?.checked_add(1).ok_or_else(||refusal("semantic authority alias depth overflow"))?})}

/// 🔎️ Compares one native byte pair per granted turn and releases both original aliases before completion.
pub struct ArtifactCanonicalEditAuthorityCursor<M:Send+Sync+'static>{
 authority:Option<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>,
 edit:Option<RetainedOwnedProjection<Edit<M>>>,
 scalar:bool,
 field:usize,
 left_chunk:usize,
 left_offset:usize,
 right_chunk:usize,
 right_offset:usize,
 verifying:bool,
 result:Option<bool>,
}
impl<M:Send+Sync+'static> ArtifactCanonicalEditAuthorityCursor<M>{
 pub fn constructor_demand()->RetirementDemand{copy(size_of::<Self>())}
 pub fn admit(authority:RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>,edit:RetainedOwnedProjection<Edit<M>>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>,RetainedOwnedProjection<Edit<M>>)>{let demand=Self::constructor_demand();if !admitted(demand,grant){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"semantic authority requires both original alias transfers"),authority,edit));}Ok((Self{authority:Some(authority),edit:Some(edit),scalar:true,field:0,left_chunk:0,left_offset:0,right_chunk:0,right_offset:0,verifying:true,result:None},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{
  if self.verifying{return Ok(copy(if self.scalar{size_of::<HybridLogicalTimestamp>()*2+size_of::<usize>()*5+size_of::<i32>()*2}else{2}));}
  if let Some(owner)=self.edit.as_ref(){return if owner.terminal_is_empty(){Ok(copy(size_of::<RetainedOwnedProjection<Edit<M>>>() ))}else{projection_demand(owner)};}
  if let Some(owner)=self.authority.as_ref(){return if owner.terminal_is_empty(){Ok(copy(size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>()))}else{projection_demand(owner)};}
  Ok(Default::default())
 }
 pub fn begin_close(&mut self){self.verifying=false;self.result=None;}
 pub fn terminal_is_empty(&self)->bool{self.edit.is_none()&&self.authority.is_none()}
 pub fn decision(&self)->Option<bool>{if self.terminal_is_empty(){self.result}else{None}}
 fn pair<'a>(field:usize,authority:&'a dyn ArtifactCanonicalEditAuthority,edit:&'a Edit<M>)->Result<(Option<Text<'a>>,Option<Text<'a>>),ValueError>{let meta=edit.mutation_meta.first().ok_or_else(||refusal("semantic authority lost original metadata"))?;Ok(match field{0=>(Some(authority.actor()),edit.actor.as_ref().map(|value|value as Text<'_>)),1=>(authority.line(),edit.line.as_ref().map(|value|value as Text<'_>)),2=>(Some(authority.actor()),meta.author_id.as_ref().map(|value|&value.0 as Text<'_>)),3=>(authority.group(),meta.group_id.as_ref().map(|value|value as Text<'_>)),_=>return Err(refusal("semantic authority native field is absent"))})}
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let demand=self.next_demand()?;if !admitted(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.verifying{
   let authority=self.authority.as_ref().unwrap().borrow()?;let edit=self.edit.as_ref().unwrap().borrow()?;let authority=authority.get();let edit=edit.get();
   if self.scalar{self.scalar=false;let valid=!edit.id.is_empty()&&edit.id.len()<=128&&edit.sequence_number==authority.sequence_number()&&edit.forwards.len()==1&&edit.mutation_meta.len()==1&&edit.mutation_meta[0].timestamp==authority.clock();if !valid{self.verifying=false;self.result=Some(false);}}
   else if self.field==4{self.verifying=false;self.result=Some(true);}
   else{let(left,right)=Self::pair(self.field,authority,edit)?;let(done,equal)=match(left,right){(None,None)=>(true,true),(None,Some(_))|(Some(_),None)=>(true,false),(Some(left),Some(right))=>{if left.text_bytes()!=right.text_bytes(){(true,false)}else{let left=ArtifactCanonicalJsonText::Native(left).next_byte(&mut self.left_chunk,&mut self.left_offset).map_err(refusal)?;let right=ArtifactCanonicalJsonText::Native(right).next_byte(&mut self.right_chunk,&mut self.right_offset).map_err(refusal)?;(left.is_none()&&right.is_none(),left==right)}}};if !equal{self.verifying=false;self.result=Some(false);}else if done{self.field+=1;self.left_chunk=0;self.left_offset=0;self.right_chunk=0;self.right_offset=0;}}
   progress.copied_bytes=demand.copy_bytes;
  }else if let Some(owner)=self.edit.as_mut(){if owner.terminal_is_empty(){self.edit=None;progress.copied_bytes=demand.copy_bytes;}else{let mut child=grant;child.maximum_depth-=1;progress=owner.close_step(child)?.progress();}}
  else if let Some(owner)=self.authority.as_mut(){if owner.terminal_is_empty(){self.authority=None;progress.copied_bytes=demand.copy_bytes;}else{let mut child=grant;child.maximum_depth-=1;progress=owner.close_step(child)?.progress();}}
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.begin_close();self.advance(grant)}
}
impl<M:Send+Sync+'static> Drop for ArtifactCanonicalEditAuthorityCursor<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"semantic authority abandoned original native aliases");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
