//! 🧾️ Original publication fields assemble into actual native paged identity owners.
use super::authority::ArtifactCanonicalEditAuthority;
use crate::Edit;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,paged::{PagedUtf8,PagedUtf8AppendCursor,Utf8Text},retirement::controlled::ControlledRetirement,retained_clone::{RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;
type Text=PagedUtf8<{usize::MAX}>;
type Identities=[Text;3];
type RetiringIdentities=(Text,Text,Text);
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn copy(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||refusal("canonical identity child depth overflow"))?;Ok(demand)}
fn child(mut grant:RetainedCloneGrant)->RetainedCloneGrant{grant.maximum_items=1;grant.maximum_depth-=1;grant}
fn permits(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn alias_demand<T:?Sized+Sync>(owner:&RetainedOwnedProjection<T>)->Result<RetirementDemand,ValueError>{let work=owner.next_close_copy_byte_demand()?;nested(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_close_capacity_byte_demand(work)?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?})}

/// 🪪️ Owns real immutable input projections and the actual three paged output fields until paid transfer or closure.
pub struct ArtifactCanonicalEditIdentityCursor<M:Send+Sync+'static>{authority:Option<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>,edit:Option<RetainedOwnedProjection<Edit<M>>>,output:Option<Identities>,append:Option<PagedUtf8AppendCursor>,retirement:Option<ControlledRetirement<RetiringIdentities>>,field:usize,chunk:usize,append_closing:bool,closing:bool}
impl<M:Send+Sync+'static> ArtifactCanonicalEditIdentityCursor<M>{
 pub fn constructor_demand()->RetirementDemand{copy(size_of::<Self>())}
 pub fn admit(authority:RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>,edit:RetainedOwnedProjection<Edit<M>>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>,RetainedOwnedProjection<Edit<M>>)>{let demand=Self::constructor_demand();if !permits(demand,grant){return Err((refusal("canonical identity constructor lacks original alias transfer grant"),authority,edit));}Ok((Self{authority:Some(authority),edit:Some(edit),output:Some(std::array::from_fn(|_|PagedUtf8::new())),append:None,retirement:None,field:0,chunk:0,append_closing:false,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
 fn text(&self)->Result<&(dyn Utf8Text+Sync),ValueError>{if self.field==0{Ok(self.authority.as_ref().ok_or_else(||refusal("canonical identity authority missing"))?.borrow()?.get().actor())}else{Ok(&self.edit.as_ref().ok_or_else(||refusal("canonical identity edit missing"))?.borrow()?.get().id)}}
 pub fn is_ready(&self)->bool{!self.closing&&self.field==3&&self.authority.is_none()&&self.edit.is_none()&&self.append.is_none()&&self.output.is_some()}
 pub fn terminal_is_empty(&self)->bool{self.authority.is_none()&&self.edit.is_none()&&self.append.is_none()&&self.output.is_none()&&self.retirement.is_none()}
 pub fn next_take_demand(&self)->RetirementDemand{copy(size_of::<Option<Identities>>())}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Identities,RetainedCloneProgress)>,ValueError>{if !self.is_ready(){return Ok(None);}let demand=self.next_take_demand();if !permits(demand,grant){return Ok(None);}Ok(self.output.take().map(|output|(output,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(owner)=self.append.as_mut(){owner.begin_close();self.append_closing=true;}}
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.append.as_ref(){
   if owner.terminal_is_empty(){return Ok(copy(size_of::<Option<PagedUtf8AppendCursor>>()));}
   if self.append_closing{let work=owner.next_close_copy_byte_demand()?;return nested(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_close_capacity_byte_demand(work)?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_depth_demand()?});}
   let text=self.text()?;let chunk=if text.text_bytes()==0{""}else{text.text_chunk(self.chunk).ok_or_else(||refusal("canonical identity native input chunk missing"))?};return nested(owner.next_advance_demand(chunk,&self.output.as_ref().unwrap()[self.field])?);
  }
  if self.closing{
   if self.output.is_some(){return Ok(copy(size_of::<Identities>()+size_of::<ControlledRetirement<RetiringIdentities>>()));}
   if let Some(owner)=self.retirement.as_ref(){if owner.terminal_is_empty(){return Ok(copy(size_of::<Option<ControlledRetirement<RetiringIdentities>>>()));}let work=owner.next_copy_byte_demand()?;return nested(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_capacity_byte_demand(work)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
  }else if self.field<3{return Ok(copy(size_of::<PagedUtf8AppendCursor>()));}
  if let Some(owner)=self.edit.as_ref(){return if owner.terminal_is_empty(){Ok(copy(size_of::<Option<RetainedOwnedProjection<Edit<M>>>>()))}else{alias_demand(owner)};}
  if let Some(owner)=self.authority.as_ref(){return if owner.terminal_is_empty(){Ok(copy(size_of::<Option<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>>()))}else{alias_demand(owner)};}
  Ok(Default::default())
 }
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty()||self.is_ready(){return Ok(RetainedCloneStep::Complete(Default::default()));}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let demand=self.next_demand()?;if !permits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let progress=if let Some(owner)=self.append.as_ref(){
   if owner.terminal_is_empty(){self.append=None;self.append_closing=false;if !self.closing{let count=self.text()?.text_chunk_count().max(1);self.chunk+=1;if self.chunk==count{self.chunk=0;self.field+=1;}}RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
   else if self.append_closing{self.append.as_mut().unwrap().close_step(child(grant))?.progress()}
   else{let authority=self.authority.as_ref().unwrap().borrow()?;let edit=self.edit.as_ref().unwrap().borrow()?;let text:&(dyn Utf8Text+Sync)=if self.field==0{authority.get().actor()}else{&edit.get().id};let chunk=if text.text_bytes()==0{""}else{text.text_chunk(self.chunk).ok_or_else(||refusal("canonical identity native input chunk missing"))?};let owner=self.append.as_mut().unwrap();let step=owner.advance(chunk,&mut self.output.as_mut().unwrap()[self.field],child(grant))?;if matches!(step,RetainedCloneStep::Complete(_)){owner.begin_close();self.append_closing=true;}step.progress()}
  }else if self.closing&&self.output.is_some(){let [actor,applied,tail]=self.output.take().unwrap();self.retirement=Some(ControlledRetirement::new((actor,applied,tail)).map_err(|(error,(actor,applied,tail))|{self.output=Some([actor,applied,tail]);error})?);RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
  else if let Some(owner)=self.retirement.as_mut(){if owner.terminal_is_empty(){self.retirement=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}else{owner.step(child(grant))?.progress()}}
  else if !self.closing&&self.field<3{self.append=Some(PagedUtf8AppendCursor::default());RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
  else if let Some(owner)=self.edit.as_mut(){if owner.terminal_is_empty(){self.edit=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}else{owner.close_step(child(grant))?.progress()}}
  else if let Some(owner)=self.authority.as_mut(){if owner.terminal_is_empty(){self.authority=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}else{owner.close_step(child(grant))?.progress()}}
  else{Default::default()};
  Ok(if self.terminal_is_empty()||self.is_ready(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.begin_close();self.advance(grant)}
}
impl<M:Send+Sync+'static> Drop for ArtifactCanonicalEditIdentityCursor<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"canonical identity assembly abandoned original inputs or output pages");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
