//! 🏪️ Original catalog sources remain loaned until one funded constructor or installation turn.
use super::*;
use std::mem::ManuallyDrop;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ArtifactStoreConstructorKind { Recipient, Document, Config, Draft, Interaction }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ArtifactStoreConstructorPhase { Preparation, Close }

pub(super) struct OriginalCatalogConstructor<P,Mu>
where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue,
Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>{
 original:ManuallyDrop<Option<Result<store::DocumentStoreOwners<P,Mu>,store::DocumentStoreOwnersAdmissionError<P,Mu>>>>,
 refusal:ManuallyDrop<Option<ValueError>>,
 refusal_close:ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
 done:bool,
}
impl<P,Mu> OriginalCatalogConstructor<P,Mu>
where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+Send+Sync+'static,
Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>+protocol::OpBinary+protocol::OpText+Send+'static{
 pub(super) fn new()->Self{Self{original:ManuallyDrop::new(None),refusal:ManuallyDrop::new(None),refusal_close:ManuallyDrop::new(None),done:false}}
 fn refusal_demands(&self,body:usize)->Option<Result<RetirementDemand,ValueError>>{
  if let Some(owner)=self.refusal_close.as_ref(){return Some(store::artifact_retirement_box_demands(owner,body))}
  if self.refusal.as_ref().is_some_and(|error|matches!(error.message,std::borrow::Cow::Owned(_))){return Some(store::artifact_retirement_owned_birth_demands(&self.refusal))}
  if self.original.as_ref().is_some_and(|original|matches!(original,Err(error)if matches!(error.error.message,std::borrow::Cow::Owned(_)))){return Some(Ok(RetirementDemand{depth:1,..Default::default()}))}
  None
 }
 fn close_refusal(&mut self,grant:RetainedCloneGrant)->Option<Result<RetainedCloneProgress,ValueError>>{
  let demand=match self.refusal_demands(grant.maximum_copy_bytes)?{Ok(demand)=>demand,Err(error)=>return Some(Err(error))};
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Some(Ok(Default::default()))}
  if self.refusal_close.is_some(){return Some(store::artifact_retirement_box_close_step(&mut self.refusal_close,grant).map(|step|step.progress()))}
  if self.refusal.as_ref().is_some_and(|error|matches!(error.message,std::borrow::Cow::Owned(_))){return Some(store::artifact_retirement_admit_owned(&mut self.refusal,&mut self.refusal_close,grant).map(|step|step.progress()))}
  let Some(Err(original))=self.original.as_mut()else{unreachable!()};
  *self.refusal=Some(std::mem::replace(&mut original.error,ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original constructor refusal is retiring")));
  Some(Ok(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 pub(super) fn demands(&self,body:usize,closing:bool,source:impl FnOnce()->Result<RetirementDemand,ValueError>)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  let parent=0usize;
  if let Some(demand)=self.refusal_demands(body){let mut demand=demand?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"catalog refusal parent depth overflow"))?;return Ok(demand)}
  let child=match self.original.as_ref(){Some(Ok(owner))=>if closing{owner.uninstalled_owners_demands(body.saturating_sub(parent))?}else if owner.constructor_is_complete(){RetirementDemand::default()}else{owner.constructor_demands(body.saturating_sub(parent))?},Some(Err(error))=>match error.owners.as_ref(){Some(owner)=>if closing{owner.uninstalled_owners_demands(body.saturating_sub(parent))?}else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"denied original catalog remains retained"))},None=>RetirementDemand::default()},None=>if closing{RetirementDemand::default()}else{source()?}};
  Ok(RetirementDemand{copy_bytes:parent.checked_add(child.copy_bytes).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"catalog source quote copy overflow"))?,capacity_bytes:child.capacity_bytes,release_bytes:child.release_bytes,depth:child.depth.max(1).checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"catalog quote depth overflow"))?})
 }
 pub(super) fn advance(&mut self,store:&mut ArtifactStore<P,Mu>,grant:RetainedCloneGrant,build:impl FnOnce(RetainedCloneGrant)->Option<Result<(store::DocumentStoreOwners<P,Mu>,RetainedCloneProgress),store::DocumentStoreOwnersAdmissionError<P,Mu>>>)->Result<RetainedCloneProgress,ValueError>{
  if self.done{return Ok(Default::default())}
  let parent=0usize;
  if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth<2{return Ok(Default::default())}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};
  if let Some(result)=self.close_refusal(child){return result}
  if self.refusal.as_ref().is_some_and(|error|matches!(error.message,std::borrow::Cow::Borrowed(_))){*self.refusal=None;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:parent,..Default::default()})}
  if self.original.as_ref().is_some_and(|original|matches!(original,Err(error)if error.owners.is_none()&&matches!(error.error.message,std::borrow::Cow::Borrowed(_)))){*self.original=None;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:parent,..Default::default()})}
  let mut receipt=if self.original.is_none(){
   match build(child){None=>{self.done=true;RetainedCloneProgress{copied_items:1,..Default::default()}},Some(Ok((owners,receipt)))=>{if !receipt.fits(child){*self.original=Some(Ok(owners));return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog source exceeded caller authority"))}*self.original=Some(Ok(owners));receipt},Some(Err(original))=>{let mut receipt=original.progress;if !receipt.fits(child){*self.original=Some(Err(original));return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original refused catalog exceeded caller authority"))}*self.original=Some(Err(original));receipt.copied_items=1;receipt}}
  }else{
   let Some(Ok(owners))=self.original.as_mut()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original denied catalog sources remain retained"))};
   if owners.constructor_is_complete(){
    let Some(Ok(owners))=self.original.take()else{unreachable!()};
    match store.install_document_store_owners_exact(owners){Ok(())=>self.done=true,Err((error,owners))=>*self.original=Some(Err(store::DocumentStoreOwnersAdmissionError{error,owners:Some(owners),progress:Default::default()}))}RetainedCloneProgress{copied_items:1,..Default::default()}
   }else{let demand=owners.constructor_demands(child.maximum_copy_bytes)?;if child.maximum_copy_bytes<demand.copy_bytes||child.maximum_capacity_bytes<demand.capacity_bytes||child.maximum_release_bytes<demand.release_bytes||child.maximum_depth<demand.depth{return Ok(Default::default())}match owners.admit_constructor(child){Ok(receipt)=>receipt,Err((error,mut receipt))=>{*self.refusal=Some(error);receipt.copied_items=1;receipt}}}
  };
  if receipt==Default::default(){return Ok(receipt)}
  if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog ticket exceeded caller authority"))}
  receipt.copied_bytes=receipt.copied_bytes.checked_add(parent).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"catalog parent copy extent overflow"))?;
  Ok(receipt)
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.done&&self.original.is_none()&&self.refusal.is_none()&&self.refusal_close.is_none()}
 pub(super) fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  let parent=0usize;
  if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth<2{return Ok(Default::default())}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};
  if let Some(result)=self.close_refusal(child){return result}
  let owner=self.original.as_mut().and_then(|original|match original{Ok(owner)=>Some(owner),Err(error)=>error.owners.as_mut()});
  if let Some(owner)=owner{
   if !owner.uninstalled_owners_terminal_is_empty(){let demand=owner.uninstalled_owners_demands(child.maximum_copy_bytes)?;if child.maximum_copy_bytes<demand.copy_bytes||child.maximum_capacity_bytes<demand.capacity_bytes||child.maximum_release_bytes<demand.release_bytes||child.maximum_depth<demand.depth{return Ok(Default::default())}let step=owner.close_uninstalled_owners_step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.uninstalled_owners_terminal_is_empty(),"original staged catalog")?;let mut receipt=step.progress();if receipt==Default::default(){return Ok(receipt)}receipt.copied_bytes+=parent;return Ok(receipt)}
  }
  *self.original=None;*self.refusal=None;self.done=true;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:parent,..Default::default()})
 }
}

impl<P,Mu> Drop for OriginalCatalogConstructor<P,Mu>
where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue,
Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>{
 fn drop(&mut self){let empty=self.done&&self.original.is_none()&&self.refusal.is_none()&&self.refusal_close.is_none();assert!(std::thread::panicking()||empty,"original catalog cursor requires controlled source/error/ticket closure before shallow Drop");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.refusal);ManuallyDrop::drop(&mut self.refusal_close)}}}
}
