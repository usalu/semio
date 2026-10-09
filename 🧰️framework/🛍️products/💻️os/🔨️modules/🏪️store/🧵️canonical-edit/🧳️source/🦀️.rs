//! 🧳️ Canonical Store preparation drives the genuine original semantic, identity and hash owners.
use super::ArtifactStoreOneItemLiveAuthority;
use crate::os_spr::{Edit,command::{ArtifactCanonicalEditAuthority,ArtifactCanonicalEditAuthorityCursor,ArtifactCanonicalEditIdentityCursor,ArtifactCanonicalEditSealCursor}};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,ErasedSnapshotRetirement,paged::PagedUtf8,retirement::{RetireOwned,controlled::ControlledRetirement,shared::SharedControlledRetirement},retained_clone::{RetainedCloneSource,RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::size_of,sync::Arc};
type Text=PagedUtf8<{usize::MAX}>;
type Identities=[Text;3];
type Returned<M>=(Box<Edit<M>>,Identities,[u8;32],RetainedCloneProgress);
fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn copy(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||refusal("canonical native source depth overflow"))?;Ok(d)}
fn child(mut g:RetainedCloneGrant)->RetainedCloneGrant{g.maximum_items=1;g.maximum_depth-=1;g}
fn controlled<T:RetireOwned>(o:&ControlledRetirement<T>)->Result<RetirementDemand,ValueError>{let body=o.next_copy_byte_demand()?;nested(RetirementDemand{copy_bytes:body,capacity_bytes:o.next_capacity_byte_demand(body)?,release_bytes:o.next_release_byte_demand()?,depth:o.next_depth_demand()?})}
fn source<T:RetireOwned+Sync>(o:&RetainedCloneSource<T>)->Result<RetirementDemand,ValueError>{let body=o.next_close_copy_byte_demand()?;nested(RetirementDemand{copy_bytes:body,capacity_bytes:o.next_close_capacity_byte_demand(body)?,release_bytes:o.next_close_release_byte_demand()?,depth:o.next_close_depth_demand()?})}

/// 🪪️ Type-erased transport preserves the native typed original owner and every actual receipt.
pub(super) trait ArtifactStoreCanonicalOwner<M>:Send{
 fn next_demand(&self)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn begin_close(&mut self);
 fn terminal_is_empty(&self)->bool;
 fn ready(&self)->bool;
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<Returned<M>>,ValueError>;
}

pub(super) struct ArtifactStoreCanonicalSource<M:RetireOwned+Sync+ArtifactCanonicalJsonTree>{
 pending_edit:Option<Box<Edit<M>>>,
 pending_authority:Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
 edit_source:Option<RetainedCloneSource<Box<Edit<M>>>>,
 authority_source:Option<RetainedCloneSource<ArtifactStoreOneItemLiveAuthority>>,
 semantic:Option<ArtifactCanonicalEditAuthorityCursor<M>>,
 identity:Option<ArtifactCanonicalEditIdentityCursor<M>>,
 hash:Option<ArtifactCanonicalEditSealCursor<M>>,
 edit:Option<Box<Edit<M>>>,
 identities:Option<Identities>,
 digest:Option<[u8;32]>,
 edit_close:Option<ControlledRetirement<Box<Edit<M>>>>,
 identity_close:Option<ControlledRetirement<(Text,Text,Text)>>,
 authority_close:Option<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>,
 phase:u8,
 closing:bool,
}
impl<M:RetireOwned+Sync+ArtifactCanonicalJsonTree> ArtifactStoreCanonicalSource<M>{
 pub(super) fn new(authority:Arc<ArtifactStoreOneItemLiveAuthority>,edit:Box<Edit<M>>)->Self{Self{pending_edit:Some(edit),pending_authority:Some(authority),edit_source:None,authority_source:None,semantic:None,identity:None,hash:None,edit:None,identities:None,digest:None,edit_close:None,identity_close:None,authority_close:None,phase:0,closing:false}}
 pub(super) fn source_constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+size_of::<Box<Edit<M>>>(),capacity_bytes:size_of::<Self>()+size_of::<Edit<M>>(),depth:1,release_bytes:0}}
 fn projections(&self)->Result<(RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>,RetainedOwnedProjection<Edit<M>>),ValueError>{let authority=self.authority_source.as_ref().ok_or_else(||refusal("canonical original authority source is absent"))?;let edit=self.edit_source.as_ref().ok_or_else(||refusal("canonical original edit source is absent"))?;authority.try_borrow()?;edit.try_borrow()?;Ok((authority.project_owned(0,|value|value as&dyn ArtifactCanonicalEditAuthority),edit.project_owned(0,|value|value.as_ref())))}
 fn advance_original(&mut self,g:RetainedCloneGrant,d:RetirementDemand)->Result<RetainedCloneProgress,ValueError>{
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if let Some(o)=self.semantic.as_mut(){if o.terminal_is_empty(){if o.decision()!=Some(true){return Err(refusal("canonical original semantic edit disagrees with Store authority"));}self.semantic=None;self.phase=3;}else{p=o.advance(child(g))?.progress();}}
  else if let Some(o)=self.identity.as_mut(){if o.is_ready(){let(output,receipt)=o.take(child(g))?.ok_or_else(||refusal("canonical preadmitted identity return refused"))?;self.identities=Some(output);p=receipt;}else if o.terminal_is_empty(){self.identity=None;self.phase=4;}else{p=o.advance(child(g))?.progress();}}
  else if let Some(o)=self.hash.as_mut(){if o.is_ready(){let(edit,digest,receipt)=o.take_edit(child(g))?.ok_or_else(||refusal("canonical preadmitted hash return refused"))?;self.edit=Some(edit);self.digest=Some(digest);p=receipt;}else if o.terminal_is_empty(){self.hash=None;self.phase=5;}else{p=o.advance(child(g))?.progress();}}
  else{match self.phase{
   0=>{let original=self.pending_edit.take().ok_or_else(||refusal("canonical pending original edit is absent"))?;match RetainedCloneSource::admit_owned(original,(),child(g)){Ok((owner,receipt))=>{self.edit_source=Some(owner);p=receipt;self.phase=1;},Err((error,original,_))=>{self.pending_edit=Some(original);return Err(error);}}},
   1=>{let original=self.pending_authority.take().ok_or_else(||refusal("canonical pending original authority is absent"))?;match RetainedCloneSource::admit(original,(),child(g)){Ok((owner,receipt))=>{self.authority_source=Some(owner);p=receipt;self.phase=2;},Err((error,original,_))=>{self.pending_authority=Some(original);return Err(error);}}},
   2=>{let(authority,edit)=self.projections()?;let(owner,receipt)=ArtifactCanonicalEditAuthorityCursor::admit(authority,edit,child(g)).unwrap_or_else(|_|unreachable!("canonical paid semantic aliases"));self.semantic=Some(owner);p=RetainedCloneProgress{copied_bytes:receipt.copied_bytes+size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>()+size_of::<RetainedOwnedProjection<Edit<M>>>(),..receipt};},
   3=>{let(authority,edit)=self.projections()?;let(owner,receipt)=ArtifactCanonicalEditIdentityCursor::admit(authority,edit,child(g)).unwrap_or_else(|_|unreachable!("canonical paid identity aliases"));self.identity=Some(owner);p=RetainedCloneProgress{copied_bytes:receipt.copied_bytes+size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>()+size_of::<RetainedOwnedProjection<Edit<M>>>(),..receipt};},
   4=>{let(owner,receipt)=ArtifactCanonicalEditSealCursor::admit_source(&mut self.edit_source,child(g))?.ok_or_else(||refusal("canonical paid original hash source transfer refused"))?;self.hash=Some(owner);p=receipt;},
   5=>{let o=self.authority_source.as_mut().ok_or_else(||refusal("canonical original authority close source is absent"))?;if o.terminal_is_empty(){self.authority_source=None;self.phase=6;}else{p=o.close_step(child(g))?.progress();}},
   _=>return Err(refusal("canonical original preparation phase cannot advance")),
  }}Ok(p)
 }
 fn advance_close(&mut self,g:RetainedCloneGrant,d:RetirementDemand)->Result<RetainedCloneProgress,ValueError>{
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if let Some(o)=self.semantic.as_mut(){if o.terminal_is_empty(){self.semantic=None;}else{p=o.advance(child(g))?.progress();}}
  else if let Some(o)=self.identity.as_mut(){if o.terminal_is_empty(){self.identity=None;}else{p=o.advance(child(g))?.progress();}}
  else if let Some(o)=self.hash.as_mut(){if o.terminal_is_empty(){self.hash=None;}else{p=o.advance(child(g))?.progress();}}
  else if let Some(o)=self.edit_source.as_mut(){if o.terminal_is_empty(){self.edit_source=None;}else{p=o.close_step(child(g))?.progress();}}
  else if let Some(o)=self.authority_source.as_mut(){if o.terminal_is_empty(){self.authority_source=None;}else{p=o.close_step(child(g))?.progress();}}
  else if self.pending_edit.is_some()||self.edit.is_some(){let original=if self.pending_edit.is_some(){self.pending_edit.take().unwrap()}else{self.edit.take().unwrap()};match ControlledRetirement::new(original){Ok(owner)=>self.edit_close=Some(owner),Err((error,original))=>{self.edit=Some(original);return Err(error);}}}
  else if let Some(o)=self.edit_close.as_mut(){if o.terminal_is_empty(){self.edit_close=None;}else{p=o.step(child(g))?.progress();}}
  else if self.identities.is_some(){let[a,b,c]=self.identities.take().unwrap();match ControlledRetirement::new((a,b,c)){Ok(owner)=>self.identity_close=Some(owner),Err((error,(a,b,c)))=>{self.identities=Some([a,b,c]);return Err(error);}}}
  else if let Some(o)=self.identity_close.as_mut(){if o.terminal_is_empty(){self.identity_close=None;}else{p=o.step(child(g))?.progress();}}
  else if let Some(original)=self.pending_authority.take(){self.authority_close=Some(SharedControlledRetirement::lease(original));}
  else if let Some(o)=self.authority_close.as_mut(){if o.terminal_is_empty(){self.authority_close=None;}else{p=o.step(child(g))?.progress();}}
  Ok(p)
 }
}
impl<M:RetireOwned+Sync+ArtifactCanonicalJsonTree> ArtifactStoreCanonicalOwner<M> for ArtifactStoreCanonicalSource<M>{
 fn next_demand(&self)->Result<RetirementDemand,ValueError>{
  if let Some(o)=self.semantic.as_ref(){return if o.terminal_is_empty(){Ok(copy(size_of::<Option<ArtifactCanonicalEditAuthorityCursor<M>>>()))}else{nested(o.next_demand()?)};}
  if let Some(o)=self.identity.as_ref(){return if o.is_ready(){nested(o.next_take_demand())}else if o.terminal_is_empty(){Ok(copy(size_of::<Option<ArtifactCanonicalEditIdentityCursor<M>>>()))}else{nested(o.next_demand()?)};}
  if let Some(o)=self.hash.as_ref(){return if o.is_ready(){nested(o.next_take_demand())}else if o.terminal_is_empty(){Ok(copy(size_of::<Option<ArtifactCanonicalEditSealCursor<M>>>()))}else{nested(o.next_demand()?)};}
  if self.closing{
   if let Some(o)=self.edit_source.as_ref(){return if o.terminal_is_empty(){Ok(copy(size_of::<Option<RetainedCloneSource<Box<Edit<M>>>>>()))}else{source(o)};}
   if let Some(o)=self.authority_source.as_ref(){return if o.terminal_is_empty(){Ok(copy(size_of::<Option<RetainedCloneSource<ArtifactStoreOneItemLiveAuthority>>>()))}else{source(o)};}
   if self.pending_edit.is_some()||self.edit.is_some(){return Ok(copy(size_of::<Box<Edit<M>>>()+size_of::<ControlledRetirement<Box<Edit<M>>>>()));}
   if let Some(o)=self.edit_close.as_ref(){return if o.terminal_is_empty(){Ok(copy(size_of::<Option<ControlledRetirement<Box<Edit<M>>>>>() ))}else{controlled(o)};}
   if self.identities.is_some(){return Ok(copy(size_of::<Identities>()+size_of::<ControlledRetirement<(Text,Text,Text)>>()));}
   if let Some(o)=self.identity_close.as_ref(){return if o.terminal_is_empty(){Ok(copy(size_of::<Option<ControlledRetirement<(Text,Text,Text)>>>() ))}else{controlled(o)};}
   if self.pending_authority.is_some(){return Ok(copy(size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+size_of::<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>()));}
   if let Some(o)=self.authority_close.as_ref(){if o.terminal_is_empty(){return Ok(copy(size_of::<Option<SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>>>()));}let body=o.next_copy_byte_demand()?;return nested(RetirementDemand{copy_bytes:body,capacity_bytes:o.next_capacity_byte_demand(body)?,release_bytes:o.next_release_byte_demand()?,depth:o.next_depth_demand()?});}
   return Ok(Default::default());
  }
  Ok(match self.phase{
   0=>{let birth=RetainedCloneSource::<Box<Edit<M>>>::owned_constructor_demand::<()>();nested(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()})?},
   1=>nested(RetirementDemand{capacity_bytes:RetainedCloneSource::<ArtifactStoreOneItemLiveAuthority>::constructor_capacity_bytes::<()>(),depth:1,..Default::default()})?,
   2=>nested(copy(ArtifactCanonicalEditAuthorityCursor::<M>::constructor_demand().copy_bytes+size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>()+size_of::<RetainedOwnedProjection<Edit<M>>>()))?,
   3=>nested(copy(ArtifactCanonicalEditIdentityCursor::<M>::constructor_demand().copy_bytes+size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>()+size_of::<RetainedOwnedProjection<Edit<M>>>()))?,
   4=>nested(ArtifactCanonicalEditSealCursor::<M>::source_constructor_demand())?,
   5=>{let o=self.authority_source.as_ref().ok_or_else(||refusal("canonical original authority source disappeared"))?;if o.terminal_is_empty(){copy(size_of::<Option<RetainedCloneSource<ArtifactStoreOneItemLiveAuthority>>>())}else{return source(o);}},
   6=>copy(size_of::<Returned<M>>()),
   _=>return Err(refusal("canonical original source phase is invalid")),
  })
 }
 fn advance(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if self.ready()||g.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.next_demand()?;if !permits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let p=if self.closing{self.advance_close(g,d)?}else{self.advance_original(g,d)?};if !p.fits(g){return Err(refusal("canonical original child receipt exceeds caller grant"));}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 fn begin_close(&mut self){self.closing=true;if let Some(o)=self.semantic.as_mut(){o.begin_close();}if let Some(o)=self.identity.as_mut(){o.begin_close();}if let Some(o)=self.hash.as_mut(){o.begin_close();}}
 fn terminal_is_empty(&self)->bool{self.pending_edit.is_none()&&self.pending_authority.is_none()&&self.edit_source.is_none()&&self.authority_source.is_none()&&self.semantic.is_none()&&self.identity.is_none()&&self.hash.is_none()&&self.edit.is_none()&&self.identities.is_none()&&self.edit_close.is_none()&&self.identity_close.is_none()&&self.authority_close.is_none()}
 fn ready(&self)->bool{!self.closing&&self.phase==6&&self.edit.is_some()&&self.identities.is_some()&&self.digest.is_some()}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<Returned<M>>,ValueError>{if !self.ready(){return Ok(None);}let d=self.next_demand()?;if !permits(d,g){return Ok(None);}Ok(Some((self.edit.take().unwrap(),self.identities.take().unwrap(),self.digest.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
}
impl<M:RetireOwned+Sync+ArtifactCanonicalJsonTree> Drop for ArtifactStoreCanonicalSource<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"canonical original source abandoned its native custody");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
